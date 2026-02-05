#!/usr/bin/env python3
"""
Flatten Rust module tree into one file by inlining `mod x;`.

With --extract-structs, it further processes the output to extract only
struct/enum definitions (no impl blocks, no mod error/into) in a flat format
suitable for diffing against generated.rs from codegen.

With --all-versions, it collects structs from all version modules (v17-v30)
that are re-exported by v30, giving a complete picture.
"""
import re
from pathlib import Path
from typing import Set, List, Tuple, Optional

MOD_RE = re.compile(
    r'^(?P<attrs>(?:\s*#\[[^\]]+\]\s*)*)\s*'
    r'(?P<vis>pub(?:\([^)]*\))?\s+)?'
    r'mod\s+(?P<name>[a-zA-Z_]\w*)\s*;\s*$'
)


def find_mod_file(base: Path, name: str) -> Path:
    """Find module file for a `mod name;` declaration."""
    cand1 = base / f"{name}.rs"
    cand2 = base / name / "mod.rs"
    if cand1.exists():
        return cand1
    if cand2.exists():
        return cand2
    raise FileNotFoundError(f"Could not find module file for mod {name}; looked for {cand1} and {cand2}")


def flatten_file(path: Path, seen: Set[Path]) -> str:
    """Recursively inline all `mod x;` declarations."""
    path = path.resolve()
    if path in seen:
        return f"// [flatten] skipped already-inlined file: {path}\n"
    seen.add(path)

    base_dir = path.parent
    out_lines = []
    for line in path.read_text(encoding="utf-8").splitlines(True):
        m = MOD_RE.match(line)
        if not m:
            out_lines.append(line)
            continue

        attrs = m.group("attrs") or ""
        vis = m.group("vis") or ""
        name = m.group("name")
        mod_path = find_mod_file(base_dir, name)

        out_lines.append(attrs)
        out_lines.append(f"{vis}mod {name} {{\n")
        body = flatten_file(mod_path, seen)
        indented = "".join(("    " + l if l.strip() else l) for l in body.splitlines(True))
        out_lines.append(indented)
        out_lines.append("}\n")

    return "".join(out_lines)


def extract_structs_from_source(text: str) -> str:
    """
    Extract struct/enum definitions from flattened Rust source.
    
    This strips:
    - mod into { ... }
    - mod error { ... }  
    - impl blocks
    - pub use statements
    - use statements
    - Module wrappers (keeps content but removes `mod name { }` wrapper)
    
    Keeps:
    - struct definitions with their doc comments and attributes
    - enum definitions with their doc comments and attributes
    """
    lines = text.splitlines(keepends=True)
    result = []
    
    # Track brace depth to know if we're inside a struct/enum
    struct_depth = 0
    in_struct = False
    skip_depth = 0  # When > 0, we're skipping a block
    
    i = 0
    while i < len(lines):
        line = lines[i]
        stripped = line.strip()
        
        # If we're in skip mode, count braces and skip until we exit
        if skip_depth > 0:
            skip_depth += stripped.count('{') - stripped.count('}')
            i += 1
            continue
        
        # Skip mod into { ... } and mod error { ... } blocks entirely
        if re.match(r'^\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+(into|error)\s*\{', stripped):
            skip_depth = stripped.count('{') - stripped.count('}')
            i += 1
            continue
        
        # Skip impl blocks
        if re.match(r'^\s*impl\s+', stripped):
            skip_depth = stripped.count('{') - stripped.count('}')
            i += 1
            continue
        
        # Skip #[cfg(feature = "std")] impl blocks
        if stripped == '#[cfg(feature = "std")]':
            i += 1
            if i < len(lines):
                next_stripped = lines[i].strip()
                if 'impl' in next_stripped:
                    skip_depth = next_stripped.count('{') - next_stripped.count('}')
                    i += 1
            continue
        
        # Skip use statements
        if re.match(r'^\s*(pub\s+)?use\s+', stripped):
            # Handle multi-line use statements
            while i < len(lines) and not lines[i].rstrip().endswith(';'):
                i += 1
            i += 1
            continue
        
        # Skip module declarations wrapper lines: `mod name {`
        if re.match(r'^\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+\w+\s*\{\s*$', stripped):
            i += 1
            continue
        
        # Skip SPDX headers
        if stripped.startswith('// SPDX-License-Identifier'):
            i += 1
            continue
        
        # Skip module-level //! doc comments
        if stripped.startswith('//!'):
            i += 1
            continue
        
        # Track struct/enum depth
        if re.match(r'^(pub\s+)?(struct|enum)\s+\w+', stripped):
            in_struct = True
            struct_depth = 0
        
        if in_struct:
            struct_depth += stripped.count('{') - stripped.count('}')
            if struct_depth <= 0 and '{' in stripped or '}' in stripped:
                # Check if this line closes the struct
                pass
        
        # Closing brace: only skip if it's a module closer (not struct closer)
        if stripped == '}':
            if in_struct and struct_depth <= 0:
                # This closes a struct
                in_struct = False
                # Remove indentation and add
                leading_ws = len(line) - len(line.lstrip())
                result.append(line[leading_ws:] if leading_ws > 0 else line)
                i += 1
                continue
            else:
                # This is a module closer, skip
                i += 1
                continue
        
        # Skip empty lines that aren't between struct definitions
        if not stripped:
            i += 1
            continue
        
        # Remove indentation (de-indent everything to root level)
        leading_ws = len(line) - len(line.lstrip())
        result.append(line[leading_ws:] if leading_ws > 0 else line)
        i += 1
    
    # Clean up result - remove excessive blank lines, add proper spacing
    cleaned = []
    prev_blank = False
    for line in result:
        is_blank = not line.strip()
        if is_blank and prev_blank:
            continue
        cleaned.append(line)
        prev_blank = is_blank
    
    return ''.join(cleaned)


def collect_all_versions(types_src: Path, latest_only: bool = True, show_version: bool = False) -> str:
    """
    Collect struct definitions from all version modules (v17-v30).
    This gives a complete picture of all types available in v30.
    
    If latest_only=True, only keep the latest version of each struct name.
    If show_version=True, add a comment showing which version the struct is from.
    """
    all_structs = {}  # struct_name -> (version, definition)
    
    # Process each version directory in order (so later versions override)
    for version_dir in sorted(types_src.iterdir()):
        if not version_dir.is_dir() or not version_dir.name.startswith('v'):
            continue
        
        mod_file = version_dir / "mod.rs"
        if not mod_file.exists():
            continue
        
        version = version_dir.name
        
        # Flatten this version's module
        text = flatten_file(mod_file, set())
        # Extract just structs
        structs_text = extract_structs_from_source(text)
        
        if latest_only:
            # Parse out individual structs and store by name
            current_struct = []
            current_doc = []  # doc comments before the struct
            struct_name = None
            in_doc = False
            
            for line in structs_text.splitlines(keepends=True):
                stripped = line.strip()
                if 'CreateRawTransaction' in stripped:
                    print("Debug: Found CreateRawTransaction line")
                    pass 
                
                # Check if this starts a new struct
                match = re.match(r'^pub struct (\w+)', line)
                if match:
                    # Save previous struct if any
                    if struct_name and current_struct:
                        all_structs[struct_name] = (version, ''.join(current_struct))
                    struct_name = match.group(1)
                    current_struct = current_doc + [line]
                    current_doc = []
                    in_doc = False
                elif struct_name:
                    current_struct.append(line)
                    # Check for struct closing brace
                    if line.strip() == '}' or (line.strip().endswith(');') and '(' in ''.join(current_struct)):
                        all_structs[struct_name] = (version, ''.join(current_struct))
                        struct_name = None
                        current_struct = []
                elif stripped.startswith('///') or stripped.startswith('#['):
                    # Doc comment or attribute before struct
                    current_doc.append(line)
                else:
                    current_doc = []  # Reset if non-doc line
            
            # Don't forget last struct
            if struct_name and current_struct:
                all_structs[struct_name] = (version, ''.join(current_struct))
        else:
            # Just append all
            if structs_text.strip():
                all_structs[f"{version}_all"] = (version, f"\n// ============ {version} ============\n\n{structs_text}")
    
    if latest_only:
        # Sort by struct name and output
        result = []
        for name in sorted(all_structs.keys()):
            version, definition = all_structs[name]
            if show_version:
                result.append(f"// From {version}\n")
            result.append(definition)
            result.append("\n")
        return ''.join(result)
    else:
        return ''.join(defn for _, defn in sorted(all_structs.values()))


def main():
    import argparse
    ap = argparse.ArgumentParser(
        description="Flatten Rust module tree into one file by inlining `mod x;`."
    )
    ap.add_argument("entry", nargs='?', help="Entry file (e.g. src/lib.rs or src/main.rs)")
    ap.add_argument("-o", "--out", default="flattened.rs", help="Output file")
    ap.add_argument(
        "--extract-structs",
        action="store_true",
        help="Extract only struct/enum definitions, removing impl blocks and helper modules"
    )
    ap.add_argument(
        "--all-versions",
        action="store_true",
        help="Collect structs from all version modules (v17-v30) in types/src/"
    )
    ap.add_argument(
        "--show-version",
        action="store_true",
        help="Add comments showing which version each struct is from (use with --all-versions)"
    )
    ap.add_argument(
        "--types-dir",
        default=None,
        help="Path to types/src directory (required with --all-versions)"
    )
    args = ap.parse_args()

    if args.all_versions:
        types_dir = Path(args.types_dir) if args.types_dir else Path("types/src")
        if not types_dir.exists():
            print(f"Error: types directory not found: {types_dir}")
            return 1
        text = collect_all_versions(types_dir, show_version=args.show_version)
    else:
        if not args.entry:
            print("Error: entry file required unless using --all-versions")
            return 1
        entry = Path(args.entry)
        text = flatten_file(entry, set())
        
        if args.extract_structs:
            text = extract_structs_from_source(text)
    
    Path(args.out).write_text(text, encoding="utf-8")
    print(f"Wrote {args.out}")
    return 0


if __name__ == "__main__":
    exit(main() or 0)
