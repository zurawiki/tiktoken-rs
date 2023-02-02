//! Rust library for tokenizing text for GPT using tiktoken
//!
//! ## Counting token length
//!
//!```
//!  use tiktoken_rs::tiktoken::p50k_base;
//!
//!  let bpe = p50k_base().unwrap();
//!  let tokens = bpe.encode_with_special_tokens("This is a test         with a lot of spaces");
//!  println!("Token count: {}", tokens.len());
//!```
//!
//! ## Examples
//! For full working examples for all supported features see [examples](https://github.com/zurawiki/tiktoken-rs/tree/main/examples) directory in the repository.
//!
/// Adaptation of the tiktoken crate for use in Rust projects
pub mod tiktoken;
