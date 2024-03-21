pub mod wrap;
pub use wrap::*;

use tiktoken_rs::{model, tokenizer, CoreBPE};
use tiktoken_rs::{
    r50k_base as openai_r50k_base,
    p50k_base as openai_p50k_base,
    cl100k_base as openai_cl100k_base,
    p50k_edit as openai_p50k_edit,
};

use std::collections::HashMap;
use std::collections::HashSet;
use hex;
use polywrap_wasm_rs::Map;
use rustc_hash::FxHasher;
use std::hash::BuildHasherDefault;
use rmp_serde;

type FxHashMap<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;

fn transform_encoding(input: Map<String, u32>) -> FxHashMap<Vec<u8>, usize> {
    let mut output = FxHashMap::default();
    
    for (key, value) in input {
        let bytes = hex::decode(&key).expect("Failed to decode hex string");
        output.insert(bytes, value as usize);
    }

    output
}

fn transform_special_tokens_encoder(input: Map<String, u32>) -> FxHashMap<String, usize> {
    let mut output = FxHashMap::default();

    for (key, value) in input {
        output.insert(key, value as usize);
    }
    
    output
}

fn get_bpe(tokenizer: Tokenizer, bpe: Option<Vec<u8>>) -> Result<CoreBPE, String> {
    let bpe = match tokenizer {
        Tokenizer::R50kBase => openai_r50k_base().map_err(|e| e.to_string())?,
        Tokenizer::P50kBase => openai_p50k_base().map_err(|e| e.to_string())?,
        Tokenizer::Cl100kBase => openai_cl100k_base().map_err(|e| e.to_string())?,
        Tokenizer::P50kEdit => openai_p50k_edit().map_err(|e| e.to_string())?,
        Tokenizer::Gpt2 => return Err("GPT2 tokenizer does not have a BPE".to_string()),
        Tokenizer::Custom => {
            let bpe = bpe.ok_or("Custom tokenizer requires a BPE".to_string())?;
            rmp_serde::from_slice(&bpe).map_err(|e| e.to_string())?
        }
        _ => return Err("Tokenizer not supported".to_string()),
    };
    Ok(bpe)
}

impl ModuleTrait for Module {
    fn get_context_size(args: ArgsGetContextSize) -> Result<u32, String> {
        let context_size = model::get_context_size(&args.model);
        u32::try_from(context_size).map_err(|e| e.to_string())
    }

    fn get_tokenizer(args: ArgsGetTokenizer) -> Result<Option<Tokenizer>, String> {
        let tokenizer = tokenizer::get_tokenizer(&args.model);
        match tokenizer {
            Some(tokenizer) => match tokenizer {
                tokenizer::Tokenizer::Cl100kBase => Ok(Some(Tokenizer::Cl100kBase)),
                tokenizer::Tokenizer::Gpt2 => Ok(Some(Tokenizer::Gpt2)),
                tokenizer::Tokenizer::P50kBase => Ok(Some(Tokenizer::P50kBase)),
                tokenizer::Tokenizer::R50kBase => Ok(Some(Tokenizer::R50kBase)),
                tokenizer::Tokenizer::P50kEdit => Ok(Some(Tokenizer::P50kEdit)),
            }
            None => Ok(None),
        }
    }

    fn create_custom_bpe(args: ArgsCreateCustomBpe) -> Result<Vec<u8>, String> {
        let encoder = transform_encoding(args.encoder); 
        let special_tokens_encoder = transform_special_tokens_encoder(args.special_tokens_encoder);
        let bpe = CoreBPE::new(
            encoder,
            special_tokens_encoder,
            &args.pattern
        ).map_err(|e| e.to_string())?;
        rmp_serde::to_vec(&bpe).map_err(|e| e.to_string())
    }

    fn encode_ordinary(args: ArgsEncodeOrdinary) -> Result<Vec<u32>, String> {
        let bpe = get_bpe(args.tokenizer, args.bpe)?;
        let output = bpe.encode_ordinary(&args.text);
        Ok(output.into_iter().map(|i| u32::try_from(i).map_err(|e| e.to_string()).unwrap()).collect())
    }

    fn encode(args: ArgsEncode) -> Result<Vec<u32>, String> {
        let bpe = get_bpe(args.tokenizer, args.bpe)?;
        let allowed_special: HashSet<&str> = args.allowed_special.iter().map(AsRef::as_ref).collect();
        let output = bpe.encode(&args.text, allowed_special);
        Ok(output.into_iter().map(|i| u32::try_from(i).map_err(|e| e.to_string()).unwrap()).collect())
    }

    fn encode_with_special_tokens(args: ArgsEncodeWithSpecialTokens) -> Result<Vec<u32>, String> {
        let bpe = get_bpe(args.tokenizer, args.bpe)?;
        let output = bpe.encode_with_special_tokens(&args.text);
        Ok(output.into_iter().map(|i| u32::try_from(i).map_err(|e| e.to_string()).unwrap()).collect())
    }

    fn decode(args: ArgsDecode) -> Result<String, String> {
        let bpe = get_bpe(args.tokenizer, args.bpe)?;
        let tokens = args.tokens.into_iter().map(|i| i as usize).collect();
        bpe.decode(tokens).map_err(|e| e.to_string())
    }

    fn split_by_token(args: ArgsSplitByToken) -> Result<Vec<String>, String> {
        let bpe = get_bpe(args.tokenizer, args.bpe)?;
        bpe.split_by_token(&args.text, args.use_special_tokens).map_err(|e| e.to_string())
    }

    fn split_by_token_ordinary(args: ArgsSplitByTokenOrdinary) -> Result<Vec<String>, String> {
        let bpe = get_bpe(args.tokenizer, args.bpe)?;
        bpe.split_by_token_ordinary(&args.text).map_err(|e| e.to_string())
    }

    fn r50k_base(_: ArgsR50kBase) -> Result<Vec<u8>, String> {
        let bpe = openai_r50k_base().map_err(|e| e.to_string())?;
        rmp_serde::to_vec(&bpe).map_err(|e| e.to_string())
    }

    fn p50k_base(_: ArgsP50kBase) -> Result<Vec<u8>, String> {
        let bpe = openai_p50k_base().map_err(|e| e.to_string())?;
        rmp_serde::to_vec(&bpe).map_err(|e| e.to_string())
    }

    fn cl100k_base(_: ArgsCl100kBase) -> Result<Vec<u8>, String> {
        let bpe = openai_cl100k_base().map_err(|e| e.to_string())?;
        rmp_serde::to_vec(&bpe).map_err(|e| e.to_string())
    }

    fn p50k_edit(_: ArgsP50kEdit) -> Result<Vec<u8>, String> {
        let bpe = openai_p50k_edit().map_err(|e| e.to_string())?;
        rmp_serde::to_vec(&bpe).map_err(|e| e.to_string())
    }

}
