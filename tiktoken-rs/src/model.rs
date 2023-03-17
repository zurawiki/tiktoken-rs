const MODEL_PREFIX_TO_ENCODING: &[(&str, &str)] = &[
    // chat
    ("gpt-4-turbo-", "cl100k_base"), // e.g, gpt-4-0314, gpt-4-32k-0314,
    ("gpt-3.5-turbo-", "cl100k_base"), // e.g, gpt-3.5-turbo-0301, -0401, etc.
];

const MODEL_TO_ENCODING: &[(&str, &str)] = &[
    // chat
    ("gpt-4-0314", "cl100k_base"),
    ("gpt-4-32k-0314", "cl100k_base"),
    ("gpt-3.5-turbo", "cl100k_base"),
    // text
    ("text-davinci-003", "p50k_base"),
    ("text-davinci-002", "p50k_base"),
    ("text-davinci-001", "r50k_base"),
    ("text-curie-001", "r50k_base"),
    ("text-babbage-001", "r50k_base"),
    ("text-ada-001", "r50k_base"),
    ("davinci", "r50k_base"),
    ("curie", "r50k_base"),
    ("babbage", "r50k_base"),
    ("ada", "r50k_base"),
    // code
    ("code-davinci-002", "p50k_base"),
    ("code-davinci-001", "p50k_base"),
    ("code-cushman-002", "p50k_base"),
    ("code-cushman-001", "p50k_base"),
    ("davinci-codex", "p50k_base"),
    ("cushman-codex", "p50k_base"),
    // edit
    ("text-davinci-edit-001", "p50k_edit"),
    ("code-davinci-edit-001", "p50k_edit"),
    // embeddings
    ("text-embedding-ada-002", "cl100k_base"),
    // old embeddings
    ("text-similarity-davinci-001", "r50k_base"),
    ("text-similarity-curie-001", "r50k_base"),
    ("text-similarity-babbage-001", "r50k_base"),
    ("text-similarity-ada-001", "r50k_base"),
    ("text-search-davinci-doc-001", "r50k_base"),
    ("text-search-curie-doc-001", "r50k_base"),
    ("text-search-babbage-doc-001", "r50k_base"),
    ("text-search-ada-doc-001", "r50k_base"),
    ("code-search-babbage-code-001", "r50k_base"),
    ("code-search-ada-code-001", "r50k_base"),
    // open source
    ("gpt2", "gpt2"),
];

/// Returns the encoding used by a model.
///
/// TODO use hashmap
pub fn encoding_for_model(model_name: &str) -> Option<&str> {
    if let Some(encoding) = MODEL_TO_ENCODING
        .iter()
        .find(|(model, _)| *model == model_name)
    {
        return Some(encoding.1);
    }
    if let Some(encoding) = MODEL_PREFIX_TO_ENCODING
        .iter()
        .find(|(model_prefix, _)| model_name.starts_with(*model_prefix))
    {
        return Some(encoding.1);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoding_for_model() {
        assert_eq!(encoding_for_model("gpt-4-32k-0314"), Some("cl100k_base"));
        assert_eq!(encoding_for_model("gpt-3.5-turbo"), Some("cl100k_base"));
        assert_eq!(
            encoding_for_model("gpt-3.5-turbo-0301"),
            Some("cl100k_base")
        );
        assert_eq!(encoding_for_model("text-davinci-003"), Some("p50k_base"));
        assert_eq!(
            encoding_for_model("code-search-ada-code-001"),
            Some("r50k_base")
        );
        assert_eq!(encoding_for_model("foo"), None);
    }
}
