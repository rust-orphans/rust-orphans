# Rust Orphans

Central place for issues and tools for the [Rust
Orphans](https://github.com/rust-orphans) project.

The aim is to create a central place to adopt crates that are still in
use but are no longer maintained.


## TODO

The [bitmaps](https://rustsec.org/advisories/RUSTSEC-2025-0167.html)
crate looks like a good starting place.


# What we do

- Security updates
- ABI-compatible bug fixes
- Updates so that crates can still be built (i.e. if they use
  now-deprecated language/standard library features)
- Keep crates patch-level compatible
- Provide guidance on using an alternative dependencies


# What we do NOT do

- Updates/breaking ABI changes
  - Unless for some reason required for security fixes
