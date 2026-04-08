/*!
 * contains information about OpenAI models.
 */

/// Macro to check if a given str starts with any of the specified prefixes.
macro_rules! starts_with_any {
    ($str:expr, $($prefix:expr),* $(,)?) => {
        false $(|| $str.starts_with($prefix))*
    };
}

/// Returns the context size of a specified model.
///
/// The context size represents the maximum number of tokens a model can process in a single input.
/// This function checks the model name and returns the corresponding context size.
/// See <https://platform.openai.com/docs/models> for up-to-date information.
///
/// # Arguments
///
/// * `model` - A string slice that holds the name of the model.
///
/// # Examples
///
/// ```
/// use tiktoken_rs::model::get_context_size;
/// let model = "gpt-4-32k";
/// let context_size = get_context_size(model);
/// assert_eq!(context_size, Some(32768));
/// ```
///
/// # Returns
///
/// Returns `None` if the model is not recognized. Callers should handle this case
/// explicitly rather than assuming a default context size.
pub fn get_context_size(model: &str) -> Option<usize> {
    if let Some(rest) = model.strip_prefix("ft:") {
        let base = rest.split(':').next().unwrap_or(rest);
        return get_context_size(base);
    }
    if starts_with_any!(model, "gpt-5.4-mini", "gpt-5.4-nano") {
        return Some(400_000);
    }
    if starts_with_any!(model, "gpt-5.4") {
        return Some(1_050_000);
    }
    if starts_with_any!(model, "gpt-5.3-codex-spark") {
        return Some(128_000);
    }
    if starts_with_any!(model, "gpt-5") {
        return Some(400_000);
    }
    if starts_with_any!(model, "codex-mini") {
        return Some(200_000);
    }
    if starts_with_any!(model, "gpt-oss") {
        return Some(131_072);
    }
    if starts_with_any!(model, "o1-mini", "o1-preview") {
        return Some(128_000);
    }
    if starts_with_any!(model, "o1", "o3", "o4") {
        return Some(200_000);
    }
    if starts_with_any!(model, "gpt-4.1") {
        return Some(1_047_576);
    }
    if starts_with_any!(model, "chatgpt-4o", "gpt-4o") {
        return Some(128_000);
    }
    if starts_with_any!(model, "gpt-4.5") {
        return Some(128_000);
    }
    if starts_with_any!(model, "gpt-4-turbo-") {
        return Some(128_000);
    }
    if starts_with_any!(model, "gpt-4-0125") {
        return Some(128_000);
    }
    if starts_with_any!(model, "gpt-4-1106") {
        return Some(128_000);
    }
    if starts_with_any!(model, "gpt-4-32k") {
        return Some(32_768);
    }
    if starts_with_any!(model, "gpt-4") {
        return Some(8192);
    }
    if starts_with_any!(model, "gpt-3.5-turbo-0125") {
        return Some(16_385);
    }
    if starts_with_any!(model, "gpt-3.5-turbo-1106") {
        return Some(16_385);
    }
    if starts_with_any!(model, "gpt-3.5-turbo-16k") {
        return Some(16_385);
    }
    if starts_with_any!(model, "gpt-3.5-turbo") {
        return Some(16_385);
    }
    if starts_with_any!(model, "text-davinci-002", "text-davinci-003") {
        return Some(4097);
    }
    if starts_with_any!(model, "ada", "babbage", "curie") {
        return Some(2049);
    }
    if starts_with_any!(model, "code-cushman-001") {
        return Some(2048);
    }
    if starts_with_any!(model, "code-davinci-002") {
        return Some(8001);
    }
    if starts_with_any!(model, "davinci") {
        return Some(2049);
    }
    if starts_with_any!(model, "text-ada-001", "text-babbage-001", "text-curie-001") {
        return Some(2049);
    }
    if starts_with_any!(model, "text-embedding-ada-002") {
        return Some(8192);
    }
    None
}
