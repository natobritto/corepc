#!/usr/bin/env python3
import re
from pathlib import Path

MOD_RE = re.compile(r'^(?P<attrs>(?:\s*#\[[^\]]+\]\s*)*)\s*mod\s+(?P<name>[a-zA-Z_]\w*)\s*;\s*$')

def find_mod_file(base: Path, name: str) -> Path:
    # base is directory containing the current file
    cand1 = base / f"{name}.rs"
    cand2 = base / name / "mod.rs"
    if cand1.exists():
        return cand1
    if cand2.exists():
        return cand2
    raise FileNotFoundError(f"Could not find module file for mod {name}; looked for {cand1} and {cand2}")

def flatten_file(path: Path, seen: set[Path]) -> str:
    path = path.resolve()
    if path in seen:
        # Prevent infinite recursion on weird module graphs.
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
        name = m.group("name")
        mod_path = find_mod_file(base_dir, name)

        # Inline it as `mod name { ... }` (keeping attrs)
        out_lines.append(attrs)
        out_lines.append(f"mod {name} {{\n")
        body = flatten_file(mod_path, seen)

        # indent body by 4 spaces
        indented = "".join(("    " + l if l.strip() else l) for l in body.splitlines(True))
        out_lines.append(indented)
        out_lines.append("}\n")

    return "".join(out_lines)

def main():
    import argparse
    ap = argparse.ArgumentParser(description="Flatten Rust module tree into one file by inlining `mod x;`.")
    ap.add_argument("entry", help="Entry file (e.g. src/lib.rs or src/main.rs)")
    ap.add_argument("-o", "--out", default="flattened.rs", help="Output file")
    args = ap.parse_args()

    entry = Path(args.entry)
    text = flatten_file(entry, set())
    Path(args.out).write_text(text, encoding="utf-8")
    print(f"Wrote {args.out}")

if __name__ == "__main__":
    main()
