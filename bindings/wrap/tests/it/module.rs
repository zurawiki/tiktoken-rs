use crate::types::wrap::types::{
    TiktokenModule,
    TiktokenModuleArgsEncodeWithSpecialTokens,
    TiktokenModuleArgsDecode,
    TiktokenTokenizer,
};

#[test]
fn test_mirror() {
    let text = "This is a test         with a lot of spaces".to_string();
    let tiktoken = TiktokenModule::new(None, None, None);

    let encoded = tiktoken.encode_with_special_tokens(&TiktokenModuleArgsEncodeWithSpecialTokens {
        tokenizer: TiktokenTokenizer::P50kBase,
        text: text.clone(),
        bpe: None,
    }, None, None, None).unwrap();

    let decoded = tiktoken.decode(&TiktokenModuleArgsDecode {
        tokenizer: TiktokenTokenizer::P50kBase,
        tokens: encoded,
        bpe: None,
    }, None, None, None).unwrap();

    assert_eq!(decoded, text);
}
