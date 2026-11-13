# Example

Create a crate with a transitive dependency that is unmaintained, and
has a RustSec advisory against it:

```
$ cargo new example
$ cargo add rublk@0.2.15
$ cargo audit
[...eliding 6 other adisories...]
Crate:     bitmaps
Version:   3.2.1
Warning:   unmaintained
Title:     bitmaps is unmaintained
Date:      2026-05-03
ID:        RUSTSEC-2026-0247
URL:       https://rustsec.org/advisories/RUSTSEC-2026-0247
Solution:  No fixed upgrade is available!

Crate:     bitmaps
Version:   3.2.1
Warning:   unsound
Title:     `Bitmap::try_from(&[u8])` can create invalid values
Date:      2025-12-25
ID:        RUSTSEC-2025-0167
URL:       https://rustsec.org/advisories/RUSTSEC-2025-0167
Solution:  No fixed upgrade is available!

warning: 8 allowed warnings found
```

The dependency is several levels down the heirarchy:

```
$ cargo tree
[...eliding other entries...]
example v0.1.0 (/home/john/tmp/example)
└── rublk v0.2.15
    └── libublk v0.4.8
        └── bitmaps v3.2.1
```

We now use [Cargo's patch feature][cargo-patch] feature to override
bitmaps v3.2.1:

```
$ cat >Cargo.toml <<EOF
> [patch.crates-io]
> bitmaps = {
>   git = "https://github.com/rust-orphans/bitmaps.git",
>   branch = "v3.2.1-orphan-1",
> }
> EOF
```

The 2 `bitmaps` advisories are now gone:

```
$ cargo build  # regenerate Cargo.lock
$ cargo audit
[...eliding 6 other adisories...]

warning: 6 allowed warnings found
```


[cargo-patch]: https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html#the-patch-section
