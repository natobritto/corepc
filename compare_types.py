#!/usr/bin/env python3
"""
Compare struct definitions between generated.rs (from OpenRPC spec) and 
flattened.rs (from repo source code).

This tool helps identify:
1. Structs that exist in one file but not the other
2. Field differences between structs with the same name
"""
import re
import sys
from pathlib import Path
from dataclasses import dataclass
from typing import Dict, List, Optional, Tuple


@dataclass
class StructField:
    name: str
    type_: str
    serde_rename: Optional[str] = None
    is_optional: bool = False
    
    def __str__(self) -> str:
        opt = "Option<...>" if self.is_optional else ""
        rename = f' (serde: "{self.serde_rename}")' if self.serde_rename else ""
        return f"  {self.name}: {self.type_}{rename}"


@dataclass 
class StructDef:
    name: str
    fields: List[StructField]
    is_tuple_struct: bool = False
    tuple_type: Optional[str] = None
    
    def __str__(self) -> str:
        if self.is_tuple_struct:
            return f"struct {self.name}({self.tuple_type})"
        return f"struct {self.name} {{ {len(self.fields)} fields }}"


def parse_structs(text: str) -> Dict[str, StructDef]:
    """Parse struct definitions from Rust source text."""
    structs = {}
    lines = text.splitlines()
    
    i = 0
    while i < len(lines):
        line = lines[i]
        
        # Match struct declaration
        # Tuple struct: pub struct Name(pub Type);
        tuple_match = re.match(r'^pub struct (\w+)\((.*)\);?\s*$', line)
        if tuple_match:
            name = tuple_match.group(1)
            tuple_type = tuple_match.group(2).strip()
            structs[name] = StructDef(
                name=name,
                fields=[],
                is_tuple_struct=True,
                tuple_type=tuple_type
            )
            i += 1
            continue
        
        # Regular struct: pub struct Name {
        struct_match = re.match(r'^pub struct (\w+)\s*\{', line)
        if struct_match:
            name = struct_match.group(1)
            fields = []
            serde_rename = None
            i += 1
            
            # Parse fields until closing brace
            while i < len(lines) and lines[i].strip() != '}':
                field_line = lines[i].strip()
                
                # Check for serde rename attribute
                rename_match = re.search(r'#\[serde\(rename\s*=\s*"([^"]+)"', field_line)
                if rename_match:
                    serde_rename = rename_match.group(1)
                    i += 1
                    continue
                
                # Parse field: pub field_name: Type,
                field_match = re.match(r'pub\s+(\w+):\s*(.+?),?\s*$', field_line)
                if field_match:
                    field_name = field_match.group(1)
                    field_type = field_match.group(2).rstrip(',')
                    is_optional = field_type.startswith('Option<')
                    
                    fields.append(StructField(
                        name=field_name,
                        type_=field_type,
                        serde_rename=serde_rename,
                        is_optional=is_optional
                    ))
                    serde_rename = None  # Reset for next field
                
                i += 1
            
            structs[name] = StructDef(name=name, fields=fields)
        
        i += 1
    
    return structs


def compare_structs(repo_structs: Dict[str, StructDef], 
                    spec_structs: Dict[str, StructDef]) -> None:
    """Compare struct definitions and print differences."""
    
    repo_names = set(repo_structs.keys())
    spec_names = set(spec_structs.keys())
    
    # Structs only in repo
    only_in_repo = sorted(repo_names - spec_names)
    # Structs only in spec  
    only_in_spec = sorted(spec_names - repo_names)
    # Structs in both
    in_both = sorted(repo_names & spec_names)
    
    print("=" * 60)
    print("STRUCT COMPARISON REPORT")
    print("=" * 60)
    
    print(f"\n📊 Summary:")
    print(f"  Repo structs:     {len(repo_names)}")
    print(f"  Spec structs:     {len(spec_names)}")
    print(f"  Only in repo:     {len(only_in_repo)}")
    print(f"  Only in spec:     {len(only_in_spec)}")
    print(f"  In both:          {len(in_both)}")
    
    if only_in_repo:
        print(f"\n🔵 Structs only in REPO ({len(only_in_repo)}):")
        for name in only_in_repo:  # Limit output
            print(f"  - {name}")
    
    if only_in_spec:
        print(f"\n🟡 Structs only in SPEC ({len(only_in_spec)}) - need to implement:")
        for name in only_in_spec:
            print(f"  + {name}")
    
    # Compare fields for matching structs
    field_diffs = []
    for name in in_both:
        repo_s = repo_structs[name]
        spec_s = spec_structs[name]
        
        # Skip tuple structs for now
        if repo_s.is_tuple_struct or spec_s.is_tuple_struct:
            continue
        
        repo_fields = {f.name: f for f in repo_s.fields}
        spec_fields = {f.name: f for f in spec_s.fields}
        
        repo_field_names = set(repo_fields.keys())
        spec_field_names = set(spec_fields.keys())
        
        missing_in_repo = spec_field_names - repo_field_names
        extra_in_repo = repo_field_names - spec_field_names
        
        # Check for type differences in common fields
        type_diffs = []
        for fname in repo_field_names & spec_field_names:
            rf = repo_fields[fname]
            sf = spec_fields[fname]
            if rf.type_ != sf.type_:
                type_diffs.append((fname, rf.type_, sf.type_))
        
        if missing_in_repo or extra_in_repo or type_diffs:
            field_diffs.append((name, missing_in_repo, extra_in_repo, type_diffs))
    
    if field_diffs:
        print(f"\n🔄 Structs with FIELD DIFFERENCES ({len(field_diffs)}):")
        for name, missing, extra, type_diffs in field_diffs:
            print(f"\n  {name}:")
            if missing:
                print(f"    Missing in repo: {', '.join(sorted(missing))}")
            if extra:
                print(f"    Extra in repo:   {', '.join(sorted(extra))}")
            if type_diffs:
                for fname, rtype, stype in type_diffs:
                    print(f"    Type diff '{fname}': repo={rtype} vs spec={stype}")
    
    print("\n" + "=" * 60)


def main():
    import argparse
    ap = argparse.ArgumentParser(description="Compare struct definitions between files")
    ap.add_argument("--repo", default="flattened.rs", help="File with repo struct definitions")
    ap.add_argument("--spec", default="generated.rs", help="File with spec struct definitions")
    args = ap.parse_args()
    
    repo_text = Path(args.repo).read_text(encoding="utf-8")
    spec_text = Path(args.spec).read_text(encoding="utf-8")
    
    repo_structs = parse_structs(repo_text)
    spec_structs = parse_structs(spec_text)
    
    compare_structs(repo_structs, spec_structs)
    return 0


if __name__ == "__main__":
    sys.exit(main())
