use std::error::Error;
use tiktoken_rs::{get_chat_completion_max_tokens, ChatCompletionRequestMessage};

fn main() -> Result<(), Box<dyn Error>> {
    let model = "gpt-4";
    let messages = vec![
        ChatCompletionRequestMessage {
            content: "You are a helpful assistant that only speaks French.".to_string(),
            role: "system".to_string(),
            name: None,
        },
        ChatCompletionRequestMessage {
            content: "Hello, how are you?".to_string(),
            role: "user".to_string(),
            name: None,
        },
        ChatCompletionRequestMessage {
            content: "Parlez-vous francais?".to_string(),
            role: "system".to_string(),
            name: None,
        },
    ];
    let max_tokens = get_chat_completion_max_tokens(model, &messages).unwrap();

    println!("Max_token parameter value: {}", max_tokens);

    Ok(())
}
