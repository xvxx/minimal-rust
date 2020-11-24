# Minimal Rust

This is a list of Rust libraries which have zero, one, or two total
dependencies with `default-features = false`, as reported by `cargo tree.`

Some of these trade dependencies for completeness, meaning they have a
limited amount of features compared to their more well-known cousins,
while some of these **are** the more well-known cousins and are listed
because they already have minimal dependencies, like `memchr`.

For a great list of major Rust crates without any pedantry around
dependencies:

- https://github.com/brson/stdx/blob/master/README.md

For information on how to reduce your Rust binary's total size:

- https://github.com/johnthagen/min-sized-rust

---

## The List

$$LIST$$
---

† _Dependency count is determined by running `cargo tree` on an empty
project with the crate included with `default-features = false`.
Specifically running `make count CRATE=<name>` from the root of this repo._

$$FOOTNOTE$$
