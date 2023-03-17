/*!
 * lists out the available tokenizers for different OpenAI models.
 */

use std::collections::HashMap;

use lazy_static::lazy_static;

/// Enum representing the available tokenizers for different OpenAI models.
///
/// This enum lists the possible tokenizer types that can be used for tokenizing text
/// when working with various OpenAI models.
///
/// # Example
///
/// ```
/// use tiktoken_rs::tokenizer::Tokenizer;
///
/// let tokenizer = Tokenizer::Cl100kBase;
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Tokenizer {
    Cl100kBase,
    P50kBase,
    R50kBase,
    P50kEdit,
    Gpt2,
}

const MODEL_PREFIX_TO_TOKENIZER: &[(&str, Tokenizer)] = &[
    // chat
    ("gpt-4-", Tokenizer::Cl100kBase),
    ("gpt-3.5-turbo-", Tokenizer::Cl100kBase),
];

const MODEL_TO_TOKENIZER: &[(&str, Tokenizer)] = &[
    // chat
    ("gpt-4-32k", Tokenizer::Cl100kBase),
    ("gpt-4", Tokenizer::Cl100kBase),
    ("gpt-3.5-turbo", Tokenizer::Cl100kBase),
    // text
    ("text-davinci-003", Tokenizer::P50kBase),
    ("text-davinci-002", Tokenizer::P50kBase),
    ("text-davinci-001", Tokenizer::R50kBase),
    ("text-curie-001", Tokenizer::R50kBase),
    ("text-babbage-001", Tokenizer::R50kBase),
    ("text-ada-001", Tokenizer::R50kBase),
    ("davinci", Tokenizer::R50kBase),
    ("curie", Tokenizer::R50kBase),
    ("babbage", Tokenizer::R50kBase),
    ("ada", Tokenizer::R50kBase),
    // code
    ("code-davinci-002", Tokenizer::P50kBase),
    ("code-davinci-001", Tokenizer::P50kBase),
    ("code-cushman-002", Tokenizer::P50kBase),
    ("code-cushman-001", Tokenizer::P50kBase),
    ("davinci-codex", Tokenizer::P50kBase),
    ("cushman-codex", Tokenizer::P50kBase),
    // edit
    ("text-davinci-edit-001", Tokenizer::P50kEdit),
    ("code-davinci-edit-001", Tokenizer::P50kEdit),
    // embeddings
    ("text-embedding-ada-002", Tokenizer::Cl100kBase),
    // old embeddings
    ("text-similarity-davinci-001", Tokenizer::R50kBase),
    ("text-similarity-curie-001", Tokenizer::R50kBase),
    ("text-similarity-babbage-001", Tokenizer::R50kBase),
    ("text-similarity-ada-001", Tokenizer::R50kBase),
    ("text-search-davinci-doc-001", Tokenizer::R50kBase),
    ("text-search-curie-doc-001", Tokenizer::R50kBase),
    ("text-search-babbage-doc-001", Tokenizer::R50kBase),
    ("text-search-ada-doc-001", Tokenizer::R50kBase),
    ("code-search-babbage-code-001", Tokenizer::R50kBase),
    ("code-search-ada-code-001", Tokenizer::R50kBase),
    // open source
    ("gpt2", Tokenizer::Gpt2),
];

lazy_static! {
    static ref MODEL_TO_TOKENIZER_MAP: HashMap<&'static str, Tokenizer> = {
        let mut map = HashMap::new();
        MODEL_TO_TOKENIZER.iter().for_each(|&(model, tokenizer)| {
            map.insert(model, tokenizer);
        });
        map
    };
}

/// Returns the tokenizer type used by a model.
///
/// This function retrieves the corresponding tokenizer enum variant for the given model name. It first looks
/// for an exact match in the `MODEL_TO_TOKENIZER` mapping. If it doesn't find a match, it checks for
/// model name prefixes in the `MODEL_PREFIX_TO_TOKENIZER` mapping.
///
/// # Arguments
///
/// * `model_name` - A string slice representing the model name for which the tokenizer should be retrieved.
///
/// # Examples
///
/// ```
/// use tiktoken_rs::tokenizer::{get_tokenizer, Tokenizer};
/// let model = "gpt-4-0314";
/// let tokenizer = get_tokenizer(model).unwrap();
/// assert_eq!(tokenizer, Tokenizer::Cl100kBase);
/// ```
///
/// # Returns
///
/// If a tokenizer is found for the given model name, the function returns an `Option` containing the tokenizer
/// enum variant; otherwise, it returns `None`.
pub fn get_tokenizer(model_name: &str) -> Option<Tokenizer> {
    if let Some(tokenizer) = MODEL_TO_TOKENIZER_MAP.get(model_name) {
        return Some(*tokenizer);
    }
    if let Some(tokenizer) = MODEL_PREFIX_TO_TOKENIZER
        .iter()
        .find(|(model_prefix, _)| model_name.starts_with(*model_prefix))
    {
        return Some(tokenizer.1);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_tokenizer() {
        assert_eq!(get_tokenizer("gpt-4-32k-0314"), Some(Tokenizer::Cl100kBase));
        assert_eq!(get_tokenizer("gpt-3.5-turbo"), Some(Tokenizer::Cl100kBase));
        assert_eq!(
            get_tokenizer("gpt-3.5-turbo-0301"),
            Some(Tokenizer::Cl100kBase)
        );
        assert_eq!(get_tokenizer("text-davinci-003"), Some(Tokenizer::P50kBase));
        assert_eq!(
            get_tokenizer("code-search-ada-code-001"),
            Some(Tokenizer::R50kBase)
        );
        assert_eq!(get_tokenizer("foo"), None);
    }
}
