## `tiktoken-rs`

[![Github Contributors](https://img.shields.io/github/contributors/zurawiki/tiktoken-rs.svg)](https://github.com/zurawiki/tiktoken-rs/graphs/contributors)
[![Github Stars](https://img.shields.io/github/stars/zurawiki/tiktoken-rs.svg)](https://github.com/zurawiki/tiktoken-rs/stargazers)
[![CI](https://github.com/zurawiki/tiktoken-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/zurawiki/tiktoken-rs/actions/workflows/ci.yml)

[![crates.io status](https://img.shields.io/crates/v/tiktoken-rs.svg)](https://crates.io/crates/tiktoken-rs)
[![crates.io downloads](https://img.shields.io/crates/d/tiktoken-rs.svg)](https://crates.io/crates/tiktoken-rs)
[![Rust dependency status](https://deps.rs/repo/github/zurawiki/tiktoken-rs/status.svg)](https://deps.rs/repo/github/zurawiki/tiktoken-rs)

Rust library for tokenizing text with OpenAI models using tiktoken.

This library provides a set of ready-made tokenizer libraries for working with GPT, tiktoken and related OpenAI models. Use cases cover tokenizing and counting tokens in text inputs.

This library is built on top of the `tiktoken` library and includes some additional features and enhancements for ease of use with Rust code.

Supports all current OpenAI models including GPT-5, GPT-4.1, GPT-4o, o1, o3, o4-mini, and gpt-oss models.

# Examples

For full working examples for all supported features, see the [examples](https://github.com/zurawiki/tiktoken-rs/tree/main/tiktoken-rs/examples) directory in the repository.

# Usage

1. Install this tool locally with `cargo`

```sh
cargo add tiktoken-rs
```

Then in your rust code, call the API

## Counting token length

```rust
use tiktoken_rs::o200k_base;

let bpe = o200k_base().unwrap();
let tokens = bpe.encode_with_special_tokens(
  "This is a sentence   with spaces"
);
println!("Token count: {}", tokens.len());
```

For repeated calls, use the singleton to avoid re-initializing the tokenizer:

```rust
use tiktoken_rs::o200k_base_singleton;

let bpe = o200k_base_singleton();
let tokens = bpe.encode_with_special_tokens(
  "This is a sentence   with spaces"
);
println!("Token count: {}", tokens.len());
```

## Counting max_tokens parameter for a chat completion request

```rust
use tiktoken_rs::{get_chat_completion_max_tokens, ChatCompletionRequestMessage};

let messages = vec![
    ChatCompletionRequestMessage {
        content: Some("You are a helpful assistant that only speaks French.".to_string()),
        role: "system".to_string(),
        name: None,
        function_call: None,
    },
    ChatCompletionRequestMessage {
        content: Some("Hello, how are you?".to_string()),
        role: "user".to_string(),
        name: None,
        function_call: None,
    },
    ChatCompletionRequestMessage {
        content: Some("Parlez-vous francais?".to_string()),
        role: "system".to_string(),
        name: None,
        function_call: None,
    },
];
let max_tokens = get_chat_completion_max_tokens("o1-mini", &messages).unwrap();
println!("max_tokens: {}", max_tokens);
```

## Counting max_tokens parameter for a chat completion request with [async-openai](https://crates.io/crates/async-openai)

Need to enable the `async-openai` feature in your `Cargo.toml` file.

```rust
use tiktoken_rs::async_openai::get_chat_completion_max_tokens;
use async_openai::types::chat::{
    ChatCompletionRequestMessage, ChatCompletionRequestSystemMessage,
    ChatCompletionRequestSystemMessageContent, ChatCompletionRequestUserMessage,
    ChatCompletionRequestUserMessageContent,
};

let messages = vec![
    ChatCompletionRequestMessage::System(ChatCompletionRequestSystemMessage {
        content: ChatCompletionRequestSystemMessageContent::Text(
            "You are a helpful assistant that only speaks French.".to_string(),
        ),
        name: None,
    }),
    ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage {
        content: ChatCompletionRequestUserMessageContent::Text(
            "Hello, how are you?".to_string(),
        ),
        name: None,
    }),
];
let max_tokens = get_chat_completion_max_tokens("o1-mini", &messages).unwrap();
println!("max_tokens: {}", max_tokens);
```

`tiktoken` supports these encodings used by OpenAI models:

| Encoding name           | OpenAI models                                                             |
| ----------------------- | ------------------------------------------------------------------------- |
| `o200k_harmony`         | `gpt-oss-20b`, `gpt-oss-120b`                                            |
| `o200k_base`            | `gpt-5`, `gpt-4.1`, `gpt-4.5`, `gpt-4o`, `o4-mini`, `o3`, `o1`, `chatgpt-4o-latest` |
| `cl100k_base`           | `gpt-4`, `gpt-3.5-turbo`, `text-embedding-ada-002`, `text-embedding-3-*` |
| `p50k_base`             | Code models, `text-davinci-002`, `text-davinci-003`                       |
| `p50k_edit`             | Edit models like `text-davinci-edit-001`, `code-davinci-edit-001`         |
| `r50k_base` (or `gpt2`) | GPT-3 models like `davinci`                                               |

### Context sizes

| Model           | Context window |
| --------------- | -------------- |
| `gpt-5`         | 400,000        |
| `gpt-4.1`       | 1,047,576      |
| `o1`, `o3`, `o4-mini` | 200,000  |
| `gpt-4o`        | 128,000        |
| `gpt-oss`       | 131,072        |
| `gpt-4`         | 8,192          |
| `gpt-3.5-turbo` | 16,385         |

See the [examples](https://github.com/zurawiki/tiktoken-rs/tree/main/tiktoken-rs/examples) in the repo for use cases. For more context on the different tokenizers, see the [OpenAI Cookbook](https://github.com/openai/openai-cookbook/blob/66b988407d8d13cad5060a881dc8c892141f2d5c/examples/How_to_count_tokens_with_tiktoken.ipynb)

# Encountered any bugs?

If you encounter any bugs or have any suggestions for improvements, please open an issue on the repository.

# Acknowledgements

Thanks @spolu for the original code, and `.tiktoken` files.

# License

This project is licensed under the [MIT License](./LICENSE).
