use tiktoken_rs::model::get_context_size;

#[test]
fn test_finetuned_context_size() {
    assert_eq!(
        get_context_size("ft:gpt-3.5-turbo-0125:custom"),
        get_context_size("gpt-3.5-turbo-0125")
    );
    assert_eq!(
        get_context_size("ft:gpt-4o:custom"),
        get_context_size("gpt-4o")
    );
    assert_eq!(get_context_size("ft:gpt-5.4:org:name:id"), Some(1_050_000));
    assert_eq!(get_context_size("ft:gpt-5.4-mini:org"), Some(400_000));
}

#[test]
fn test_o_series_context_size() {
    assert_eq!(get_context_size("o1"), Some(200_000));
    assert_eq!(get_context_size("o1-pro"), Some(200_000));
    assert_eq!(get_context_size("o1-mini"), Some(128_000));
    assert_eq!(get_context_size("o1-preview"), Some(128_000));
    assert_eq!(get_context_size("o3"), Some(200_000));
    assert_eq!(get_context_size("o3-mini"), Some(200_000));
    assert_eq!(get_context_size("o3-pro"), Some(200_000));
    assert_eq!(get_context_size("o4-mini"), Some(200_000));
}

#[test]
fn test_gpt5_context_size() {
    assert_eq!(get_context_size("gpt-5"), Some(400_000));
    assert_eq!(get_context_size("gpt-5-mini"), Some(400_000));
    assert_eq!(get_context_size("gpt-5-nano"), Some(400_000));
    assert_eq!(get_context_size("gpt-5.4"), Some(1_050_000));
    assert_eq!(get_context_size("gpt-5.4-pro"), Some(1_050_000));
    assert_eq!(get_context_size("gpt-5.4-mini"), Some(400_000));
    assert_eq!(get_context_size("gpt-5.4-nano"), Some(400_000));
    // gpt-5.2 / 5.3 / codex variants
    assert_eq!(get_context_size("gpt-5.2"), Some(400_000));
    assert_eq!(get_context_size("gpt-5.2-pro"), Some(400_000));
    assert_eq!(get_context_size("gpt-5.2-codex"), Some(400_000));
    assert_eq!(get_context_size("gpt-5.3-codex"), Some(400_000));
    assert_eq!(get_context_size("gpt-5.3-codex-spark"), Some(128_000));
    assert_eq!(get_context_size("gpt-5.1-codex"), Some(400_000));
    assert_eq!(get_context_size("gpt-5.1-codex-mini"), Some(400_000));
    assert_eq!(get_context_size("gpt-5-codex"), Some(400_000));
    assert_eq!(get_context_size("codex-mini-latest"), Some(200_000));
}

#[test]
fn test_gpt4_context_size() {
    assert_eq!(get_context_size("gpt-4.5-preview"), Some(128_000));
    assert_eq!(get_context_size("gpt-4.1"), Some(1_047_576));
    assert_eq!(get_context_size("gpt-4.1-mini"), Some(1_047_576));
    assert_eq!(get_context_size("gpt-4.1-nano"), Some(1_047_576));
    assert_eq!(get_context_size("chatgpt-4o-latest"), Some(128_000));
    assert_eq!(get_context_size("gpt-4o"), Some(128_000));
    assert_eq!(get_context_size("gpt-4o-mini"), Some(128_000));
}

#[test]
fn test_gpt35_aliases() {
    // Common shorthand and Azure deployment name should resolve like gpt-3.5-turbo
    assert_eq!(get_context_size("gpt-3.5"), Some(16_385));
    assert_eq!(get_context_size("gpt-35-turbo"), Some(16_385));
}

#[test]
fn test_unknown_model_returns_none() {
    assert_eq!(get_context_size("foo"), None);
    assert_eq!(get_context_size("not-a-model"), None);
    assert_eq!(get_context_size(""), None);
}
