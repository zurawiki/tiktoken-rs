# `tiktoken-rs`

[![Github Contributors](https://img.shields.io/github/contributors/zurawiki/tiktoken-rs.svg)](https://github.com/zurawiki/tiktoken-rs/graphs/contributors)
[![Github Stars](https://img.shields.io/github/stars/zurawiki/tiktoken-rs.svg)](https://github.com/zurawiki/tiktoken-rs/stargazers)
[![CI](https://github.com/zurawiki/tiktoken-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/zurawiki/tiktoken-rs/actions/workflows/ci.yml)

[![crates.io status](https://img.shields.io/crates/v/tiktoken-rs.svg)](https://crates.io/crates/tiktoken-rs)
[![crates.io downloads](https://img.shields.io/crates/d/tiktoken-rs.svg)](https://crates.io/crates/tiktoken-rs)
[![Rust dependency status](https://deps.rs/repo/github/zurawiki/tiktoken-rs/status.svg)](https://deps.rs/repo/github/zurawiki/tiktoken-rs)

Ready-made tokenizer library for working with GPT and tiktoken

## Usage

1. Install this tool locally with `cargo`

```sh
cargo add tiktoken-rs
```

Then in your rust code, call the API
```rust
use tiktoken_rs::tiktoken::p50k_base;
let bpe = p50k_base().unwrap();
let tokens = bpe.encode_with_special_tokens("This is an example         with a lot of spaces");
println!("Token count: {}", tokens.len());
```

See the examples in the repo for usecases.

## Encountered any bugs?

If you encounter any bugs or have any suggestions for improvements, please open an issue on the repository.

## License

This project is licensed under the [MIT License](./LICENSE).
