# Rust Orphans

Central place for issues and tools for the [Rust
Orphans](https://github.com/rust-orphans) project.

The aim is to create a central place to adopt crates that are still in
use but are no longer maintained.

The ideal solution is, of course, to move away from such crates - but
that can take time. While you're trainsitioning, this project aims to
provide compatible crates with fixes for RustSec advisories, minor bug
fixes, etc.

See [EXAMPLE.md](docs/EXAMPLE.md) for an example.


## What we do

- Security updates
- ABI-compatible bug fixes
- Updates so that crates can still be built (i.e. if they use
  now-deprecated language/standard library features)
- Keep crates patch-level compatible
- Provide guidance on using an alternative dependencies


## What we do NOT do

- Updates/breaking ABI changes
  - Unless for some reason required for security fixes


## What you need to do

Use the [`patch`][cargo-patch] feature of Cargo to override the
offending dependency.


## Potential first work

- [`bitmaps`](https://rustsec.org/advisories/RUSTSEC-2025-0167.html)
  - Has an advisory and no obvious fork
  - Lots of dependencies
- [`im`](https://crates.io/crates/im)
  - [Advisories](https://github.com/rustsec/advisory-db/tree/31cf5a8673e10d3a4d7d829ad5095f47ed419049/crates/im)
  - [Issue](https://github.com/bodil/im-rs/issues/207)


[cargo-patch]: https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html#the-patch-section
