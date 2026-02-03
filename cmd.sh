cargo expand -p corepc-types --lib v30 > flattened.rs
# OR: cargo expand --manifest-path types/Cargo.toml --lib v30 > flattened.rs
# OR: python flatten_mods.py types/src/v30/mod.rs > flattened.rs

cd codegen; cargo run -- --input ../OpenRPC.json --output .. --core-version 30 --single-file # this creates generated.rs


I need to generate flattened.md file in a format as similar as possible to  generated.rs file.
The codegen module generates generated.rs from OpenRPC.json spec, meanwhile the flattened.rs is generated from the rust source code in this repo.
The reason is I need to make a tool that: receives OpenRPC.json spec to compare to current implemented types to find differences in implementation, what is missing, what is different.
what you did before is not what was needed. I dont need to make one file from the other, i need to have them somehow share the same structure. It would be ideal if the generated.rs had output similar to what this repo has.
The final goal here is to be able to compare them. In an ideal world, that would be a simple `diff` to capture all the changes that need to be done on types of this repo to match the OpenRPC spec.
You are free to modify:
- the codegen in codegen/ folder
- the command in cmd.sh
- the python file in flatten_mods.py file which is an alternative attempt to do the same as  `cargo expand -p corepc-types --lib v30 > flattened.rs`
