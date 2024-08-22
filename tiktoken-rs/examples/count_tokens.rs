use std::error::Error;

use tiktoken_rs::p50k_base;

fn main() -> Result<(), Box<dyn Error>> {
    let bpe = p50k_base().unwrap();
    let tokens = bpe.encode_with_special_tokens("This is a test         with a lot of spaces")?;
    println!("Token count: {}", tokens.len());

    Ok(())
}
