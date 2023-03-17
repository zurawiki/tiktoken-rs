/// Rust library for tokenizing text with OpenAI models using tiktoken.
///
/// This library provides a set of utilities for working with OpenAI models, particularly
/// for tokenizing and counting tokens in text inputs. It is built on top of the tiktoken
/// library and includes some additional features and enhancements for ease of use with
/// rust code.
///
/// # Counting token length
///
/// ```rust
/// use tiktoken_rs::p50k_base;
///
/// let bpe = p50k_base().unwrap();
/// let tokens = bpe.encode_with_special_tokens(
///   "This is a sentence   with spaces"
/// );
/// println!("Token count: {}", tokens.len());
/// ```
///
/// # Counting max_tokens for a chat completion request
///
/// ```rust
/// use tiktoken_rs::get_chat_completion_max_tokens;
/// use async_openai::types::{ChatCompletionRequestMessageArgs, Role};
///
/// let messages = vec![
///     ChatCompletionRequestMessageArgs::default()
///         .content("You are a helpful assistant!")
///         .role(Role::System)
///         .build()
///         .unwrap(),
///     ChatCompletionRequestMessageArgs::default()
///         .content("Hello, how are you?")
///         .role(Role::User)
///         .build()
///         .unwrap(),
/// ];
/// let max_tokens = get_chat_completion_max_tokens("gpt-4", &messages).unwrap();
/// println!("max_tokens: {}", max_tokens);
/// ```
///
/// # Examples
///
/// For full working examples for all supported features, see the [examples](https://github.com/zurawiki/tiktoken-rs/tree/main/examples) directory in the repository.
mod api;
mod singleton;
mod tiktoken_ext;
mod vendor_tiktoken;

pub use api::*;
pub mod model;
pub mod tokenizer;
pub use singleton::*;
pub use tiktoken_ext::openai_public::*;
pub use vendor_tiktoken::*;
