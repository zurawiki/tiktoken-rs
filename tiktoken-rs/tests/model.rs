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
    assert_eq!(get_context_size("ft:gpt-5.4:org:name:id"), 1_050_000);
    assert_eq!(get_context_size("ft:gpt-5.4-mini:org"), 400_000);
}

#[test]
fn test_o_series_context_size() {
    assert_eq!(get_context_size("o1"), 200_000);
    assert_eq!(get_context_size("o1-pro"), 200_000);
    assert_eq!(get_context_size("o1-mini"), 128_000);
    assert_eq!(get_context_size("o1-preview"), 128_000);
    assert_eq!(get_context_size("o3"), 200_000);
    assert_eq!(get_context_size("o3-mini"), 200_000);
    assert_eq!(get_context_size("o3-pro"), 200_000);
    assert_eq!(get_context_size("o4-mini"), 200_000);
}

#[test]
fn test_gpt5_context_size() {
    assert_eq!(get_context_size("gpt-5"), 400_000);
    assert_eq!(get_context_size("gpt-5-mini"), 400_000);
    assert_eq!(get_context_size("gpt-5-nano"), 400_000);
    assert_eq!(get_context_size("gpt-5.4"), 1_050_000);
    assert_eq!(get_context_size("gpt-5.4-pro"), 1_050_000);
    assert_eq!(get_context_size("gpt-5.4-mini"), 400_000);
    assert_eq!(get_context_size("gpt-5.4-nano"), 400_000);
    // gpt-5.2 / 5.3 / codex variants
    assert_eq!(get_context_size("gpt-5.2"), 400_000);
    assert_eq!(get_context_size("gpt-5.2-pro"), 400_000);
    assert_eq!(get_context_size("gpt-5.2-codex"), 400_000);
    assert_eq!(get_context_size("gpt-5.3-codex"), 400_000);
    assert_eq!(get_context_size("gpt-5.3-codex-spark"), 128_000);
    assert_eq!(get_context_size("gpt-5.1-codex"), 400_000);
    assert_eq!(get_context_size("gpt-5.1-codex-mini"), 400_000);
    assert_eq!(get_context_size("gpt-5-codex"), 400_000);
    assert_eq!(get_context_size("codex-mini-latest"), 200_000);
}

#[test]
fn test_gpt4_context_size() {
    assert_eq!(get_context_size("gpt-4.5-preview"), 128_000);
    assert_eq!(get_context_size("gpt-4.1"), 1_047_576);
    assert_eq!(get_context_size("gpt-4.1-mini"), 1_047_576);
    assert_eq!(get_context_size("gpt-4.1-nano"), 1_047_576);
    assert_eq!(get_context_size("chatgpt-4o-latest"), 128_000);
    assert_eq!(get_context_size("gpt-4o"), 128_000);
    assert_eq!(get_context_size("gpt-4o-mini"), 128_000);
}
