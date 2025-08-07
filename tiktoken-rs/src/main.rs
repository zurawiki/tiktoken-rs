use clap::Parser;
use serde::Serialize;
use std::io::{self, Read};
use tiktoken_rs::{get_bpe_from_model, model::get_context_size};

#[derive(Parser)]
#[command(
    name = "tiktoken",
    about = "Count tokens in text using OpenAI's tiktoken library",
    version
)]
struct Args {
    /// Model to use for tokenization (e.g., gpt-4o, gpt-3.5-turbo, o1)
    #[arg(short, long, default_value = "gpt-4.1")]
    model: String,

    /// Output format for the results
    #[arg(short, long, default_value = "count")]
    output: OutputFormat,

    /// Input text to count tokens for (reads from stdin if not provided)
    #[arg(value_name = "TEXT")]
    text: Option<String>,
}

#[derive(Clone, Copy, clap::ValueEnum)]
enum OutputFormat {
    /// Output just the token count (default)
    Count,
    /// Output detailed information in JSON format
    Json,
}

#[derive(Serialize)]
struct TokenCountResponse {
    /// Number of tokens in the input text
    token_count: usize,
    /// Model used for tokenization
    model: String,
    /// Context size for the model
    context_size: usize,
    /// Remaining tokens available for completion
    remaining_tokens: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Get input text from argument or stdin
    let input_text = if let Some(text) = args.text {
        text
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    };

    // Count tokens using the specified model
    let bpe = get_bpe_from_model(&args.model)?;
    let token_count = bpe.encode_with_special_tokens(&input_text).len();
    let context_size = get_context_size(&args.model);
    let remaining_tokens = context_size.saturating_sub(token_count);

    // Output based on the specified format
    match args.output {
        OutputFormat::Count => {
            println!("{}", token_count);
        }
        OutputFormat::Json => {
            let response = TokenCountResponse {
                token_count,
                model: args.model,
                context_size,
                remaining_tokens,
            };
            println!("{}", serde_json::to_string_pretty(&response)?);
        }
    }

    Ok(())
}
