#!/usr/bin/env python3
"""
Flatten Rust module tree and extract struct definitions.

This script reads the corepc-types source tree (types/src/v17..v30) and
produces a single .rs file containing every `pub struct` definition,
suitable for diffing against codegen output.

Two modes:
  1. Single-file flatten: inline all `mod x;` declarations recursively.
  2. All-versions collect (--all-versions): walk types/src/v17..vN, extract
     struct definitions, and keep only the latest version of each struct.
"""
import argparse
import re
from pathlib import Path
from typing import Optional, Set

# Matches `mod name;` declarations (with optional attributes and visibility).
_MOD_RE = re.compile(
    r'^(?P<attrs>(?:\s*#\[[^\]]+\]\s*)*)\s*'
    r'(?P<vis>pub(?:\([^)]*\))?\s+)?'
    r'mod\s+(?P<name>[a-zA-Z_]\w*)\s*;\s*$'
)


# ── Core helpers ───────────────────────────────────────────────────────────────

def _find_mod_file(base: Path, name: str) -> Path:
    """Resolve `mod name;` to either `name.rs` or `name/mod.rs`."""
    for candidate in (base / f"{name}.rs", base / name / "mod.rs"):
        if candidate.exists():
            return candidate
    raise FileNotFoundError(
        f"Cannot find module file for `mod {name};` under {base}"
    )


def _flatten_file(path: Path, seen: Set[Path]) -> str:
    """Recursively inline `mod x;` declarations into a single string."""
    path = path.resolve()
    if path in seen:
        return f"// [flatten] already inlined: {path}\n"
    seen.add(path)

    base_dir = path.parent
    out = []
    for line in path.read_text(encoding="utf-8").splitlines(True):
        m = _MOD_RE.match(line)
        if not m:
            out.append(line)
            continue

        attrs = m.group("attrs") or ""
        vis = m.group("vis") or ""
        name = m.group("name")
        mod_path = _find_mod_file(base_dir, name)

        out.append(attrs)
        out.append(f"{vis}mod {name} {{\n")
        body = _flatten_file(mod_path, seen)
        out.append("".join(
            ("    " + l if l.strip() else l) for l in body.splitlines(True)
        ))
        out.append("}\n")

    return "".join(out)


# ── Struct extraction ──────────────────────────────────────────────────────────

def _extract_structs(text: str) -> str:
    """
    Strip everything except struct/enum definitions from flattened Rust source.

    Removes: mod into/error blocks, impl blocks, use statements, module
    wrappers, SPDX headers, module-level doc comments.
    Keeps:   pub struct / pub enum definitions with their doc comments and attrs.
    """
    lines = text.splitlines(keepends=True)
    result = []
    skip_depth = 0
    in_struct = False
    struct_depth = 0
    i = 0

    while i < len(lines):
        line = lines[i]
        stripped = line.strip()

        # ── skip mode: count braces until we leave the block ──
        if skip_depth > 0:
            skip_depth += stripped.count('{') - stripped.count('}')
            i += 1
            continue

        # ── skip certain blocks entirely ──
        if re.match(r'^\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+(into|error)\s*\{', stripped):
            skip_depth = stripped.count('{') - stripped.count('}')
            i += 1
            continue

        if re.match(r'^\s*impl\s+', stripped):
            skip_depth = stripped.count('{') - stripped.count('}')
            i += 1
            continue

        if stripped == '#[cfg(feature = "std")]':
            i += 1
            if i < len(lines) and 'impl' in lines[i].strip():
                skip_depth = lines[i].strip().count('{') - lines[i].strip().count('}')
                i += 1
            continue

        # ── skip use statements (including multi-line) ──
        if re.match(r'^\s*(pub\s+)?use\s+', stripped):
            while i < len(lines) and not lines[i].rstrip().endswith(';'):
                i += 1
            i += 1
            continue

        # ── skip module wrapper lines (`mod name {`) ──
        if re.match(r'^\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+\w+\s*\{\s*$', stripped):
            i += 1
            continue

        # ── skip SPDX / module-doc lines ──
        if stripped.startswith('// SPDX-License-Identifier') or stripped.startswith('//!'):
            i += 1
            continue

        # ── track struct/enum depth ──
        if re.match(r'^(pub\s+)?(struct|enum)\s+\w+', stripped):
            in_struct = True
            struct_depth = 0

        if in_struct:
            struct_depth += stripped.count('{') - stripped.count('}')

        # ── closing brace: struct closer vs module closer ──
        if stripped == '}':
            if in_struct and struct_depth <= 0:
                in_struct = False
                result.append(line.lstrip())
                i += 1
                continue
            else:
                i += 1
                continue

        # ── skip blank lines, de-indent everything to root level ──
        if not stripped:
            i += 1
            continue

        result.append(line.lstrip())
        i += 1

    # Collapse consecutive blank lines
    cleaned = []
    prev_blank = False
    for line in result:
        is_blank = not line.strip()
        if is_blank and prev_blank:
            continue
        cleaned.append(line)
        prev_blank = is_blank

    return "".join(cleaned)


# ── All-versions collector ─────────────────────────────────────────────────────

def _collect_all_versions(
    types_src: Path,
    up_to_version: Optional[int] = None,
) -> str:
    """
    Walk types/src/v17..vN, extract struct definitions from each version
    module, and keep only the latest definition of each struct name.
    """
    all_structs: dict[str, tuple[str, str]] = {}  # name -> (version_label, definition)

    for version_dir in sorted(types_src.iterdir()):
        if not version_dir.is_dir() or not version_dir.name.startswith('v'):
            continue

        if up_to_version is not None:
            try:
                if int(version_dir.name[1:]) > up_to_version:
                    continue
            except ValueError:
                continue

        mod_file = version_dir / "mod.rs"
        if not mod_file.exists():
            continue

        version = version_dir.name
        structs_text = _extract_structs(_flatten_file(mod_file, set()))

        # Parse individual structs and store by name (later versions overwrite)
        current_doc: list[str] = []
        current_body: list[str] = []
        struct_name: Optional[str] = None

        for line in structs_text.splitlines(keepends=True):
            match = re.match(r'^pub struct (\w+)', line)
            if match:
                # save previous struct
                if struct_name and current_body:
                    all_structs[struct_name] = (version, "".join(current_body))
                struct_name = match.group(1)
                current_body = current_doc + [line]
                current_doc = []
            elif struct_name:
                current_body.append(line)
                # detect struct end
                s = line.strip()
                if s == '}' or (s.endswith(');') and '(' in "".join(current_body)):
                    all_structs[struct_name] = (version, "".join(current_body))
                    struct_name = None
                    current_body = []
            elif line.strip().startswith('///') or line.strip().startswith('#['):
                current_doc.append(line)
            else:
                current_doc = []

        if struct_name and current_body:
            all_structs[struct_name] = (version, "".join(current_body))

    # Emit sorted by struct name
    parts = []
    for name in sorted(all_structs):
        _, definition = all_structs[name]
        parts.append(definition)
        parts.append("\n")
    return "".join(parts)


# ── CLI ────────────────────────────────────────────────────────────────────────

def main() -> int:
    ap = argparse.ArgumentParser(
        description="Flatten Rust module tree and extract struct definitions."
    )
    ap.add_argument("entry", nargs='?', help="Entry .rs file (e.g. src/lib.rs)")
    ap.add_argument("-o", "--out", default="flattened.rs", help="Output file")
    ap.add_argument("--extract-structs", action="store_true",
                     help="Keep only struct/enum definitions")
    ap.add_argument("--all-versions", action="store_true",
                     help="Collect structs from all version modules in types/src/")
    ap.add_argument("--types-dir", default=None,
                     help="Path to types/src/ (required with --all-versions)")
    ap.add_argument("--up-to-version", type=int, default=None,
                     help="Only collect from versions <= N (e.g. 28)")
    args = ap.parse_args()

    if args.all_versions:
        types_dir = Path(args.types_dir) if args.types_dir else Path("types/src")
        if not types_dir.exists():
            print(f"Error: types directory not found: {types_dir}")
            return 1
        text = _collect_all_versions(types_dir, up_to_version=args.up_to_version)
    else:
        if not args.entry:
            print("Error: entry file required unless using --all-versions")
            return 1
        text = _flatten_file(Path(args.entry), set())
        if args.extract_structs:
            text = _extract_structs(text)

    Path(args.out).write_text(text, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
