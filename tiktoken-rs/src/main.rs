use clap::Parser;
use serde::Serialize;
use std::io::{self, Read};
use tiktoken_rs::{get_bpe_from_model, model::get_context_size, tokenizer::list_available_models};

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

    /// List all available models and exit
    #[arg(long)]
    list_models: bool,

    /// Input text to count tokens for (reads from stdin if not provided)
    #[arg(value_name = "TEXT")]
    text: Vec<String>,
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
    /// Percentage of context used (rounded to 3 decimal places)
    usage_percentage: f64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Handle list models command
    if args.list_models {
        println!("Available models:");
        println!();

        // Get all models from the tokenizer module
        let models = list_available_models();

        for model in models.iter() {
            let context_size = get_context_size(model);
            println!("  {:<25} (context: {})", model, context_size);
        }

        println!();
        println!(
            "Note: Many models support version suffixes (e.g., gpt-4-0314, gpt-3.5-turbo-0125)"
        );
        println!("      and fine-tuned models use the ft: prefix (e.g., ft:gpt-3.5-turbo:xxx:2023-11-11)");
        return Ok(());
    }

    // Get input text from argument or stdin
    let input_text = if !args.text.is_empty() {
        args.text.join(" ")
    } else {
        let mut buffer = String::new();
        eprintln!("🔎 Reading from stdin...");
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    };

    // Count tokens using the specified model
    let bpe = get_bpe_from_model(&args.model)?;
    let token_count = bpe.encode_with_special_tokens(&input_text).len();
    let context_size = get_context_size(&args.model);
    let remaining_tokens = context_size.saturating_sub(token_count);

    // Calculate usage percentage rounded to 3 decimal places
    let usage_percentage = if context_size > 0 {
        ((token_count as f64 / context_size as f64) * 100.0 * 1000.0).round() / 1000.0
    } else {
        0.0
    };

    // Output based on the specified format
    match args.output {
        OutputFormat::Count => {
            println!("{token_count}");
        }
        OutputFormat::Json => {
            let response = TokenCountResponse {
                token_count,
                model: args.model,
                context_size,
                remaining_tokens,
                usage_percentage,
            };
            println!("{}", serde_json::to_string_pretty(&response)?);
        }
    }

    Ok(())
}
