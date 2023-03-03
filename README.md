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
let tokens = bpe.encode_with_special_tokens("This is an example");
println!("Token count: {}", tokens.len());
```

`tiktoken` supports three encodings used by OpenAI models:

| Encoding name           | OpenAI models                                       |
|-------------------------|-----------------------------------------------------|
| `cl100k_base`           | ChatGPT models, `text-embedding-ada-002`            |
| `p50k_base`             | Code models, `text-davinci-002`, `text-davinci-003` |
| `r50k_base` (or `gpt2`) | GPT-3 models like `davinci`                         |


See the [examples](./examples/) in the repo for use cases. For more context on the different tokenizers, see the [OpenAI Cookbook](https://github.com/openai/openai-cookbook/blob/66b988407d8d13cad5060a881dc8c892141f2d5c/examples/How_to_count_tokens_with_tiktoken.ipynb)


## Encountered any bugs?

If you encounter any bugs or have any suggestions for improvements, please open an issue on the repository.

## Acknowledgements

- Thanks @spolu for the original code, and `.tiktoken` files.

## License

This project is licensed under the [MIT License](./LICENSE).
