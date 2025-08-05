use rustc_hash::FxHashMap as HashMap;

use tiktoken_rs::{
    byte_pair_split, cl100k_base, o200k_base, o200k_harmony, p50k_base, p50k_base_singleton,
    r50k_base, CoreBPE, Rank,
};

#[test]
fn very_simple_test() {
    let mut ranks = HashMap::default();
    ranks.insert(b"ab".to_vec(), 1);
    ranks.insert(b"cd".to_vec(), 2);

    let res = byte_pair_split(b"abcd", &ranks);
    assert_eq!(res, vec![b"ab", b"cd"]);
}

fn test_roundtrip(bpe: &CoreBPE, text: &str) {
    let tokens = bpe.encode_with_special_tokens(text);
    let decoded = bpe.decode(tokens).unwrap();
    assert_eq!(decoded, text);
}

fn test_decode(bpe: &CoreBPE, text: &str, exected_tokens: Vec<Rank>) {
    let tokens = bpe.encode_with_special_tokens(text);
    assert_eq!(tokens, exected_tokens,);
}

#[test]
fn p50k_base_test() {
    let bpe = p50k_base().unwrap();
    test_roundtrip(&bpe, "This is a test         with a lot of spaces");
    test_decode(
        &bpe,
        "This is a test         with a lot of spaces",
        vec![1212, 318, 257, 1332, 50263, 351, 257, 1256, 286, 9029],
    );
    test_decode(
        &bpe,
        "This is a test         with a lot of spaces<|endoftext|>",
        vec![
            1212, 318, 257, 1332, 50263, 351, 257, 1256, 286, 9029, 50256,
        ],
    );
}

#[test]
fn r50k_base_test() {
    let bpe = r50k_base().unwrap();
    test_roundtrip(&bpe, "This is a test         with a lot of spaces");
    test_decode(
        &bpe,
        "This is a test         with a lot of spaces",
        vec![
            1212, 318, 257, 1332, 220, 220, 220, 220, 220, 220, 220, 220, 351, 257, 1256, 286, 9029,
        ],
    );
    test_decode(
        &bpe,
        "This is a test         with a lot of spaces<|endoftext|>",
        vec![
            1212, 318, 257, 1332, 220, 220, 220, 220, 220, 220, 220, 220, 351, 257, 1256, 286,
            9029, 50256,
        ],
    );
}

#[test]
fn cl100k_base_test() {
    let bpe = cl100k_base().unwrap();
    test_roundtrip(&bpe, "This is a test         with a lot of spaces");
    test_decode(
        &bpe,
        "This is a test         with a lot of spaces",
        vec![2028, 374, 264, 1296, 260, 449, 264, 2763, 315, 12908],
    );
    test_decode(
        &bpe,
        "This is a test         with a lot of spaces<|endoftext|>",
        vec![
            2028, 374, 264, 1296, 260, 449, 264, 2763, 315, 12908, 100257,
        ],
    );
}

#[test]
fn cl100k_split_test() {
    let bpe = cl100k_base().unwrap();
    let tokenized: Result<Vec<_>, _> = bpe
        .split_by_token_iter("This is a test         with a lot of spaces", true)
        .collect();
    let tokenized = tokenized.unwrap();
    assert_eq!(
        tokenized,
        vec!["This", " is", " a", " test", "        ", " with", " a", " lot", " of", " spaces"]
    );
}

#[test]
fn o200k_base_test() {
    let bpe = o200k_base().unwrap();
    test_roundtrip(&bpe, "This is a test         with a lot of spaces");
    test_decode(
        &bpe,
        "This is a test         with a lot of spaces",
        vec![2500, 382, 261, 1746, 269, 483, 261, 3261, 328, 18608],
    );
    test_decode(
        &bpe,
        "This is a test         with a lot of spaces<|endoftext|>",
        vec![
            2500, 382, 261, 1746, 269, 483, 261, 3261, 328, 18608, 199999,
        ],
    );
}

#[test]
fn o200k_split_test() {
    let bpe = o200k_base().unwrap();
    let tokenized: Result<Vec<_>, _> = bpe
        .split_by_token_iter("This is a test         with a lot of spaces", true)
        .collect();
    let tokenized = tokenized.unwrap();
    assert_eq!(
        tokenized,
        vec!["This", " is", " a", " test", "        ", " with", " a", " lot", " of", " spaces"]
    );
}

#[test]
fn p50k_base_singleton_test() {
    // let now = std::time::Instant::now();
    let bpe1 = p50k_base_singleton();
    // println!("p50k_base_singleton load 1: {:?}", now.elapsed());
    // let now = std::time::Instant::now();
    let tokens = bpe1.encode_with_special_tokens("This is a test         with a lot of spaces");
    bpe1.decode(tokens).unwrap();
    // println!("p50k_base encode/decode 1: {:?}", now.elapsed());

    //let now = std::time::Instant::now();
    let bpe2 = p50k_base_singleton();
    // println!("p50k_base_singleton load 2: {:?}", now.elapsed());
    // let now = std::time::Instant::now();
    let tokens = bpe2.encode_with_special_tokens("This is a test         with a lot of spaces");
    bpe2.decode(tokens).unwrap();
    // println!("p50k_base encode/decode 2: {:?}", now.elapsed());
}

#[test]
fn test_unicode_encode() {
    let bpe = r50k_base().unwrap();

    let input = "🍌This is a sentence";
    let tokenized = bpe.split_by_token(input, true).unwrap();
    assert_eq!(tokenized.len(), 7);

    let input = "你会说中文吗？";
    let tokenized = bpe.split_by_token(input, true).unwrap();
    assert_eq!(tokenized.len(), 14);
}

#[test]
fn test_unicode_roundtrip() {
    test_roundtrip(&cl100k_base().unwrap(), "🍌This is a sentence");
    test_roundtrip(&p50k_base().unwrap(), "我想借几本汉语书");
    test_roundtrip(&r50k_base().unwrap(), "我想借几本汉语书");
    test_roundtrip(&cl100k_base().unwrap(), "你会说中文吗？");
    test_roundtrip(&o200k_base().unwrap(), "ひらがなカタカナ漢字");
    test_roundtrip(&o200k_harmony().unwrap(), "ひらがなカタカナ漢字");
}
