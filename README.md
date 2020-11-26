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

| Crate | Desc | Dependencies† | Links |
| --- | --- | --- | --- |
| [`pico-args`] | CLI Args Parser | 0 | [📦](https://crates.io/crates/pico-args) • [📚](https://docs.rs/pico-args) • [🏠](https://github.com/RazrFalcon/pico-args) |
| [`seahorse`] | CLI Framework | 0 | [📦](https://crates.io/crates/seahorse) • [📚](https://docs.rs/seahorse) • [🏠](https://github.com/ksk001100/seahorse) |
| [`nanorand`] | Random Numbers | 0 | [📦](https://crates.io/crates/nanorand) • [📚](https://docs.rs/nanorand) • [🏠](https://github.com/aspenluxxxy/nanorand-rs) |
| [`sval`] | De/Serializer | 0 | [📦](https://crates.io/crates/sval) • [📚](https://docs.rs/sval) • [🏠](https://github.com/sval-rs/sval) |
| [`json`] | JSON Parser | 0 | [📦](https://crates.io/crates/json) • [📚](https://docs.rs/json) • [🏠](https://github.com/maciejhirsz/json-rust) |
| [`tinyjson`] | JSON Parser/Generator | 0 | [📦](https://crates.io/crates/tinyjson) • [📚](https://docs.rs/tinyjson) • [🏠](https://github.com/rhysd/tinyjson) |
| [`minreq`] | HTTP Client | 0 | [📦](https://crates.io/crates/minreq) • [📚](https://docs.rs/minreq) • [🏠](https://github.com/neonmoe/minreq) |
| [`httparse`] | HTTP Parser | 0 | [📦](https://crates.io/crates/httparse) • [📚](https://docs.rs/httparse) • [🏠](https://github.com/seanmonstar/httparse) |
| [`percent-encoding`] | Percent Encoding | 0 | [📦](https://crates.io/crates/percent-encoding) • [📚](https://docs.rs/percent-encoding) • [🏠](https://github.com/servo/rust-url/tree/percent_encoding) |
| [`lazy_static`] | Runtime Globals | 0 | [📦](https://crates.io/crates/lazy_static) • [📚](https://docs.rs/lazy_static) • [🏠](https://github.com/rust-lang-nursery/lazy-static.rs) |
| [`anyhow`] | Error Trait | 0 | [📦](https://crates.io/crates/anyhow) • [📚](https://docs.rs/anyhow) • [🏠](https://github.com/dtolnay/anyhow) |
| [`ryu`] | Float to String | 0 | [📦](https://crates.io/crates/ryu) • [📚](https://docs.rs/ryu) • [🏠](https://github.com/dtolnay/ryu) |
| [`bytes`] | Bytes Utility | 0 | [📦](https://crates.io/crates/bytes) • [📚](https://docs.rs/bytes) • [🏠](https://github.com/tokio-rs/bytes) |
| [`memchr`] | Search Bytes | 0 | [📦](https://crates.io/crates/memchr) • [📚](https://docs.rs/memchr) • [🏠](https://github.com/BurntSushi/rust-memchr) |

## One dependency

| Crate | Desc | Dependencies† | Links |
| --- | --- | --- | --- |
| [`csv-core`] | CSV Reader/Writer | 1 | [📦](https://crates.io/crates/csv-core) • [📚](https://docs.rs/csv-core) • [🏠](https://github.com/BurntSushi/rust-csv) |
| [`quick-xml`] | XML Parser/Generator | 1 | [📦](https://crates.io/crates/quick-xml) • [📚](https://docs.rs/quick-xml) • [🏠](https://github.com/tafia/quick-xml) |
| [`nanoserde`] | De/Serializer | 1 | [📦](https://crates.io/crates/nanoserde) • [📚](https://docs.rs/nanoserde) • [🏠](https://github.com/not-fl3/nanoserde) |
| [`rs-complete`] | Tab Completion | 1 | [📦](https://crates.io/crates/rs-complete) • [📚](https://docs.rs/rs-complete) • [🏠](https://github.com/liquidityc/rs-complete) |
| [`libc-strftime`] | Time Formatter | 1 | [📦](https://crates.io/crates/libc-strftime) • [📚](https://docs.rs/libc-strftime) • [🏠](https://github.com/cecton/libc-strftime) |
| [`bstr`] | Byte Strings | 1 | [📦](https://crates.io/crates/bstr) • [📚](https://docs.rs/bstr) • [🏠](https://github.com/BurntSushi/bstr) |
| [`aho-corasick`] | Find Substrings | 1 | [📦](https://crates.io/crates/aho-corasick) • [📚](https://docs.rs/aho-corasick) • [🏠](https://github.com/BurntSushi/aho-corasick) |
| [`ropey`] | Rope Type | 1 | [📦](https://crates.io/crates/ropey) • [📚](https://docs.rs/ropey) • [🏠](https://github.com/cessen/ropey) |

## Two dependencies

| Crate | Desc | Dependencies† | Links |
| --- | --- | --- | --- |
| [`indexmap`] | Insertion-Ordered Map | 2 | [📦](https://crates.io/crates/indexmap) • [📚](https://docs.rs/indexmap) • [🏠](https://github.com/bluss/indexmap) |
| [`fuzzy-matcher`] | Fuzzy Find | 2 | [📦](https://crates.io/crates/fuzzy-matcher) • [📚](https://docs.rs/fuzzy-matcher) • [🏠](https://github.com/lotabout/fuzzy-matchertmai) |

---

† _Dependency count is determined by running `cargo tree` on an empty
project with the crate included with `default-features = false`.
Specifically running `make count CRATE=<name>` from the root of this repo._

[`pico-args`]: https://crates.io/crates/pico-args
[`seahorse`]: https://crates.io/crates/seahorse
[`nanorand`]: https://crates.io/crates/nanorand
[`sval`]: https://crates.io/crates/sval
[`json`]: https://crates.io/crates/json
[`tinyjson`]: https://crates.io/crates/tinyjson
[`minreq`]: https://crates.io/crates/minreq
[`httparse`]: https://crates.io/crates/httparse
[`percent-encoding`]: https://crates.io/crates/percent-encoding
[`lazy_static`]: https://crates.io/crates/lazy_static
[`anyhow`]: https://crates.io/crates/anyhow
[`ryu`]: https://crates.io/crates/ryu
[`bytes`]: https://crates.io/crates/bytes
[`memchr`]: https://crates.io/crates/memchr
[`csv-core`]: https://crates.io/crates/csv-core
[`quick-xml`]: https://crates.io/crates/quick-xml
[`nanoserde`]: https://crates.io/crates/nanoserde
[`rs-complete`]: https://crates.io/crates/rs-complete
[`libc-strftime`]: https://crates.io/crates/libc-strftime
[`bstr`]: https://crates.io/crates/bstr
[`aho-corasick`]: https://crates.io/crates/aho-corasick
[`ropey`]: https://crates.io/crates/ropey
[`indexmap`]: https://crates.io/crates/indexmap
[`fuzzy-matcher`]: https://crates.io/crates/fuzzy-matcher

