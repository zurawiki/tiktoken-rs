use tiktoken_rs::{Rank, cl100k_base_singleton};

#[test]
fn split_decode_reports_unknown_tokens_without_losing_valid_neighbors() {
    let bpe = cl100k_base_singleton();
    let valid = bpe.encode_with_special_tokens("hello <|endoftext|>");
    let mut tokens = valid.clone();
    tokens.insert(1, Rank::MAX);

    let mut decoded = bpe._decode_native_and_split(tokens);
    assert_eq!(
        decoded.next().unwrap().unwrap(),
        bpe.decode_bytes(&valid[..1]).unwrap()
    );
    assert_eq!(decoded.next().unwrap().unwrap_err().token, Rank::MAX);
    let remaining: Vec<u8> = decoded.collect::<Result<Vec<_>, _>>().unwrap().concat();
    assert_eq!(remaining, bpe.decode_bytes(&valid[1..]).unwrap());
}

#[test]
fn split_decode_can_collect_errors_and_empty_input() {
    let bpe = cl100k_base_singleton();
    let result = bpe
        ._decode_native_and_split(vec![Rank::MAX])
        .collect::<Result<Vec<_>, _>>();
    assert_eq!(result.unwrap_err().token, Rank::MAX);
    assert!(
        bpe._decode_native_and_split(vec![])
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .is_empty()
    );
}
