# tiktoken-rs

[![crates.io](https://img.shields.io/crates/v/tiktoken-rs.svg)](https://crates.io/crates/tiktoken-rs)
[![Documentation](https://docs.rs/tiktoken-rs/badge.svg)](https://docs.rs/tiktoken-rs)
[![CI](https://github.com/zurawiki/tiktoken-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/zurawiki/tiktoken-rs/actions/workflows/ci.yml)

Count tokens, encode text into token IDs, and decode IDs back into text using
OpenAI's tiktoken tokenizers. Tokenization runs locally with vocabularies bundled
in the crate.

[API reference](https://docs.rs/tiktoken-rs) ·
[Examples](https://github.com/zurawiki/tiktoken-rs/tree/main/tiktoken-rs/examples) ·
[Releases and migration notes](https://github.com/zurawiki/tiktoken-rs/releases)

## Count tokens

Add the library to your project:

```sh
cargo add tiktoken-rs
```

Choose the model you are sending text to, then count its tokens:

```rust
use tiktoken_rs::bpe_for_model;

let tokenizer = bpe_for_model("gpt-4o").expect("supported model");
let text = "Hello, world!";
println!("{} tokens", tokenizer.count_ordinary(text));
```

`bpe_for_model` selects the model's encoding and reuses a cached tokenizer.
It returns an error if the model name is not recognized. `count_ordinary` treats
the entire input as plain text, including strings that look like special tokens.

For the minimum supported Rust compiler, see
[`rust-version` in Cargo.toml](https://github.com/zurawiki/tiktoken-rs/blob/main/tiktoken-rs/Cargo.toml).

## Encode and decode text

Encoding returns token IDs. Decoding those IDs reconstructs the original text:

```rust
use tiktoken_rs::bpe_for_model;

let tokenizer = bpe_for_model("gpt-4o").expect("supported model");
let text = "Hello, world!";
let tokens = tokenizer.encode_ordinary(text);
let decoded = tokenizer.decode(&tokens).expect("valid tokens and UTF-8");

assert_eq!(decoded, text);
```

Decode with the same encoding used to create the IDs. `decode` returns an error
for unknown token IDs or invalid UTF-8. Use `decode_bytes` when you need bytes
rather than a UTF-8 string—for example, when decoding individual tokens that may
contain only part of a character.

## Choose an encoding directly

If you know the encoding name, use its constructor or cached singleton, such as
`o200k_base()` or `o200k_base_singleton()`. Prefer the singleton for repeated calls
to avoid loading the vocabulary each time.

| Encoding | Example models or uses |
| --- | --- |
| `o200k_base` | GPT-4o, GPT-4.1, GPT-5, o1, o3 |
| `o200k_harmony` | gpt-oss |
| `cl100k_base` | GPT-4, GPT-3.5 Turbo, text-embedding-3 |
| `p50k_base` | Legacy Codex, text-davinci-002, text-davinci-003 |
| `p50k_edit` | Legacy edit models |
| `r50k_base` | GPT-2, GPT-3 |

These are OpenAI tokenizers. For other model families, such as Llama or Mistral,
use a library that supports their tokenizers, such as
[Hugging Face tokenizers](https://crates.io/crates/tokenizers).

## Handle special tokens

Use `encode_ordinary` for plain text. If your input deliberately includes special
token markers, `encode_with_special_tokens` recognizes the encoding's markers:

```rust
use tiktoken_rs::cl100k_base_singleton;

let tokenizer = cl100k_base_singleton();
let tokens = tokenizer.encode_with_special_tokens("Hello <|endoftext|>");
assert_eq!(tokenizer.decode(&tokens).unwrap(), "Hello <|endoftext|>");
```

For control over which markers are allowed, use `encode(text, &allowed_special)`.
It returns a `Result`; markers outside the allowed set are treated as ordinary
text. The `count` and `count_with_special_tokens` helpers provide the corresponding
token counts.

## Estimate chat token usage

Chat requests include message framing as well as text. Use
`num_tokens_from_messages` to include an estimate of that overhead:

```rust
use tiktoken_rs::{ChatCompletionRequestMessage, num_tokens_from_messages};

let messages = [ChatCompletionRequestMessage {
    role: "user".into(),
    content: Some("Explain Rust's ownership model.".into()),
    ..Default::default()
}];

let estimate = num_tokens_from_messages("gpt-4o", &messages).unwrap();
println!("Estimated prompt tokens: {estimate}");
```

Chat counts are estimates, not a guarantee of the API's reported usage. Message
framing and tool-call overhead can vary; tool definitions and image, audio, or
file token usage are not covered by this text-based estimate.

`get_chat_completion_max_tokens` estimates the **remaining context capacity**
after the messages. A model also has a separate output limit: cap the result at
that limit before using it as `max_tokens` or `max_completion_tokens`. Check the
[model documentation](https://developers.openai.com/api/docs/models) for limits.

### Use async-openai message types

Enable the optional integration:

```sh
cargo add tiktoken-rs --features async-openai
```

`tiktoken_rs::async_openai::num_tokens_from_messages` and
`tiktoken_rs::async_openai::get_chat_completion_max_tokens` accept
`async_openai` chat message types. The integration uses the dependency version
listed in the crate's [Cargo.toml](https://github.com/zurawiki/tiktoken-rs/blob/main/tiktoken-rs/Cargo.toml).

## Contributing

Bug reports and improvements are welcome. See the
[contribution guide](https://github.com/zurawiki/tiktoken-rs/blob/main/CONTRIBUTING.md)
or [open an issue](https://github.com/zurawiki/tiktoken-rs/issues).

## License and acknowledgements

[MIT](https://github.com/zurawiki/tiktoken-rs/blob/main/LICENSE). Based on
[OpenAI's tiktoken](https://github.com/openai/tiktoken). Thanks to @spolu for the
original code and tokenizer files.
