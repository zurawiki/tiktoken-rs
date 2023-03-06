/*!

Rust library for tokenizing text for GPT using tiktoken

## Counting token length

```
use tiktoken_rs::p50k_base;

let bpe = p50k_base().unwrap();
let tokens = bpe.encode_with_special_tokens(
  "This is a sentence   with spaces"
);
println!("Token count: {}", tokens.len());
```

## Examples
For full working examples for all supported features see [examples](https://github.com/zurawiki/tiktoken-rs/tree/main/examples) directory in the repository.

*/

/// Adaptation of the tiktoken crate for use in Rust projects
use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use lazy_static::lazy_static;
use parking_lot::Mutex;
use rustc_hash::FxHashMap as HashMap;

use std::sync::Arc;
use tiktoken::CoreBPE;

/// Use for GPT-3 models like `davinci`
/// Initializes and returns a new instance of the r50k_base tokenizer (also known as `gpt2`)
pub fn r50k_base() -> Result<CoreBPE> {
    let r50k_base = include_str!("../../assets/r50k_base.tiktoken");

    let mut encoder = HashMap::default();
    for line in r50k_base.lines() {
        let mut parts = line.split(' ');
        let token = &general_purpose::STANDARD.decode(parts.next().unwrap())?;
        let rank: usize = parts.next().unwrap().parse().unwrap();
        encoder.insert(token.clone(), rank);
    }

    let mut special_tokens = HashMap::default();
    special_tokens.insert(String::from("<|endoftext|>"), 50256);

    let bpe = CoreBPE::new(
        encoder,
        special_tokens,
        "'s|'t|'re|'ve|'m|'ll|'d| ?\\p{L}+| ?\\p{N}+| ?[^\\s\\p{L}\\p{N}]+|\\s+(?!\\S)|\\s+",
    )?;
    Ok(bpe)
}

/// Use for Code models, `text-davinci-002`, `text-davinci-003`
/// Initializes and returns a new instance of the p50k_base tokenizer.
pub fn p50k_base() -> Result<CoreBPE> {
    let p50k_base = include_str!("../../assets/p50k_base.tiktoken");

    let mut encoder = HashMap::default();
    for line in p50k_base.lines() {
        let mut parts = line.split(' ');
        let raw = parts.next().unwrap();
        let token = &general_purpose::STANDARD.decode(raw)?;
        let rank: usize = parts.next().unwrap().parse().unwrap();
        encoder.insert(token.clone(), rank);
    }

    let mut special_tokens = HashMap::default();
    special_tokens.insert(String::from("<|endoftext|>"), 50256);

    let bpe = CoreBPE::new(
        encoder,
        special_tokens,
        "'s|'t|'re|'ve|'m|'ll|'d| ?\\p{L}+| ?\\p{N}+| ?[^\\s\\p{L}\\p{N}]+|\\s+(?!\\S)|\\s+",
    )?;
    Ok(bpe)
}
/// Use for ChatGPT models, `text-embedding-ada-002`
/// Initializes and returns a new instance of the cl100k_base tokenizer.
pub fn cl100k_base() -> Result<CoreBPE> {
    let cl100k_base = include_str!("../../assets/cl100k_base.tiktoken");

    let mut encoder = HashMap::default();
    for line in cl100k_base.lines() {
        let mut parts = line.split(' ');
        let raw = parts.next().unwrap();
        let token = &general_purpose::STANDARD.decode(raw)?;
        let rank: usize = parts.next().unwrap().parse().unwrap();
        encoder.insert(token.clone(), rank);
    }

    let mut special_tokens = HashMap::default();
    special_tokens.insert(String::from("<|endoftext|>"), 100257);
    special_tokens.insert(String::from("<|fim_prefix|>"), 100258);
    special_tokens.insert(String::from("<|fim_middle|>"), 100259);
    special_tokens.insert(String::from("<|fim_suffix|>"), 100260);
    special_tokens.insert(String::from("<|endofprompt|>"), 100276);

    let bpe = CoreBPE::new(
        encoder,
        special_tokens,
        "(?i:'s|'t|'re|'ve|'m|'ll|'d)|[^\\r\\n\\p{L}\\p{N}]?\\p{L}+|\\p{N}{1,3}| ?[^\\s\\p{L}\\p{N}]+[\\r\\n]*|\\s*[\\r\\n]+|\\s+(?!\\S)|\\s+",
    )?;
    Ok(bpe)
}

/// Returns a singleton instance of the r50k_base tokenizer. (also known as `gpt2`)
/// Use for GPT-3 models like `davinci`
///
/// This function will only initialize the tokenizer once, and then return a reference the tokenizer
pub fn r50k_base_singleton() -> Arc<Mutex<CoreBPE>> {
    lazy_static! {
        static ref R50K_BASE: Arc<Mutex<CoreBPE>> = Arc::new(Mutex::new(r50k_base().unwrap()));
    }
    R50K_BASE.clone()
}

/// Returns a singleton instance of the p50k_base tokenizer.
/// Use for Code models, `text-davinci-002`, `text-davinci-003`
///
/// This function will only initialize the tokenizer once, and then return a reference the tokenizer.
pub fn p50k_base_singleton() -> Arc<Mutex<CoreBPE>> {
    lazy_static! {
        static ref P50K_BASE: Arc<Mutex<CoreBPE>> = Arc::new(Mutex::new(p50k_base().unwrap()));
    }
    P50K_BASE.clone()
}

/// Returns a singleton instance of the cl100k_base tokenizer.
/// Use for ChatGPT models, `text-embedding-ada-002`
///
/// This function will only initialize the tokenizer once, and then return a reference the tokenizer
pub fn cl100k_base_singleton() -> Arc<Mutex<CoreBPE>> {
    lazy_static! {
        static ref CL100K_BASE: Arc<Mutex<CoreBPE>> = Arc::new(Mutex::new(cl100k_base().unwrap()));
    }
    CL100K_BASE.clone()
}
