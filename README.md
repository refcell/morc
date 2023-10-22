# morc 

[![Build Status]][actions]
[![License]][mit-license]
[![Docs]][Docs-rs]
[![Latest Version]][crates.io]
[![rustc 1.31+]][Rust 1.31]

[Build Status]: https://img.shields.io/github/actions/workflow/status/refcell/morc/ci.yml?branch=main
[actions]: https://github.com/refcell/morc/actions?query=branch%3Amain
[Latest Version]: https://img.shields.io/crates/v/morc.svg
[crates.io]: https://crates.io/crates/morc
[rustc 1.31+]: https://img.shields.io/badge/rustc_1.31+-lightgray.svg
[Rust 1.31]: https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html
[License]: https://img.shields.io/badge/license-MIT-7795AF.svg
[mit-license]: https://github.com/refcell/morc/blob/main/LICENSE.md
[Docs-rs]: https://docs.rs/morc/
[Docs]: https://img.shields.io/docsrs/morc.svg?color=319e8c&label=docs.rs


**Dead simple, minimal markdown generator library written in Rust.** Morc is in https://github.com/refcell/morc/labels/beta.

![](./etc/banner.png)

**[Install](#usage)**
| [User Docs](#what-is-morc)
| [Crate Docs][crates.io]
| [Reference][Docs-rs]
| [Contributing](#contributing)
| [License](#license)
| [Attribution](#attribution)

## What is morc?

`morc` is a dead simple, minimal library for generating
valid markdown files. It was primarily built for `amble`
but ripped out into it's own crate to provide an extensible
library, published on [crates.io][crates.io].

## Usage

Add `morc` as a dependency with cargo.

```bash,ignore
cargo add morc
```

A short example to generate markdown with `morc` is shown below.

```rust
// todo
```

## Contributing

All contributions are welcome! Experimentation is highly encouraged
and new issues are welcome.

## Troubleshooting & Bug Reports

Please check existing issues for similar bugs or
[open an issue](https://github.com/refcell/morc/issues/new)
if no relevant issue already exists.

## Attribution

Morc is built from first principles, designed from scratch with
minimal external references. It is a sandboxed experiment in design.

## License

This project is licensed under the [MIT License](LICENSE.md).
Free and open-source, forever.
*All our rust are belong to you.*
