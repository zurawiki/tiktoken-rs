pub const BEGIN_OF_TEXT: &str = "<|begin_of_text|>";
pub const END_OF_TEXT: &str = "<|end_of_text|>";
pub const RESERVED_SPECIAL_TOKEN_0: &str = "<|reserved_special_token_0|>";
pub const RESERVED_SPECIAL_TOKEN_1: &str = "<|reserved_special_token_1|>";
pub const RESERVED_SPECIAL_TOKEN_2: &str = "<|reserved_special_token_2|>";
pub const RESERVED_SPECIAL_TOKEN_3: &str = "<|reserved_special_token_3|>";
pub const START_HEADER_ID: &str = "<|start_header_id|>";
pub const END_HEADER_ID: &str = "<|end_header_id|>";
pub const RESERVED_SPECIAL_TOKEN_4: &str = "<|reserved_special_token_4|>";
pub const EOT_ID: &str = "<|eot_id|>";

/// Adaptation of the tiktoken crate for use in Rust projects
use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};

use rustc_hash::FxHashMap as HashMap;

use crate::vendor_tiktoken::CoreBPE;

const NUM_RESERVED_SPECIAL_TOKENS: usize = 256;

pub fn llama3_base() -> Result<CoreBPE> {
    let llama3_base = include_str!("../../assets/llama3_base.tiktoken");

    let mut encoder: std::collections::HashMap<
        Vec<u8>,
        usize,
        std::hash::BuildHasherDefault<rustc_hash::FxHasher>,
    > = HashMap::default();
    for line in llama3_base.lines() {
        let mut parts = line.split(' ');
        let raw = parts.next().unwrap();
        let token = &general_purpose::STANDARD.decode(raw)?;
        let rank: usize = parts.next().unwrap().parse().unwrap();
        encoder.insert(token.clone(), rank);
    }

    let mut special_tokens_list = vec![
        BEGIN_OF_TEXT.to_string(),
        END_OF_TEXT.to_string(),
        RESERVED_SPECIAL_TOKEN_0.to_string(),
        RESERVED_SPECIAL_TOKEN_1.to_string(),
        RESERVED_SPECIAL_TOKEN_2.to_string(),
        RESERVED_SPECIAL_TOKEN_3.to_string(),
        START_HEADER_ID.to_string(),
        END_HEADER_ID.to_string(),
        RESERVED_SPECIAL_TOKEN_4.to_string(),
        EOT_ID.to_string(),
    ];

    for i in 5..NUM_RESERVED_SPECIAL_TOKENS - 5 {
        let token = format!("<|reserved_special_token_{}|>", i);
        special_tokens_list.push(token);
    }

    let num_base_tokens = encoder.len();
    let mut special_tokens = HashMap::default();
    for (i, token) in special_tokens_list.iter().enumerate() {
        special_tokens.insert((*token).to_string(), num_base_tokens + i);
    }

    let bpe = CoreBPE::new(
        encoder,
        special_tokens,
        "(?i:'s|'t|'re|'ve|'m|'ll|'d)|[^\\r\\n\\p{L}\\p{N}]?\\p{L}+|\\p{N}{1,3}| ?[^\\s\\p{L}\\p{N}]+[\\r\\n]*|\\s*[\\r\\n]+|\\s+(?!\\S)|\\s+",
    )?;
    Ok(bpe)
}
