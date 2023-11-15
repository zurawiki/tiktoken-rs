use std::error::Error;

use tiktoken_rs::{num_tokens_from_messages, ChatCompletionRequestMessage};

static SIZE_FACTOR: usize = 128;
static CONTENT: &str = include_str!("./example_text.txt");

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    // let content = CONTENT.repeat(SIZE_FACTOR);
    let content = CONTENT.repeat(SIZE_FACTOR).to_string();

    let messages = vec![
        ChatCompletionRequestMessage {
            role: "system".to_string(),
            name: None,
            content: Some(content),
            function_call: None,
        },
        ChatCompletionRequestMessage {
            role: "user".to_string(),
            name: None,
            content: Some("We don't have time to boil the ocean.".to_string()),
            function_call: None,
        },
    ];
    let num_tokens = num_tokens_from_messages("gpt-4-turbo", &messages).unwrap();
    println!("Token count: {}", num_tokens);
    Ok(())
}
