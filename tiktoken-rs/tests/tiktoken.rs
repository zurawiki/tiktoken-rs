use rustc_hash::FxHashMap as HashMap;

use tiktoken_rs::{
    byte_pair_split, cl100k_base, o200k_base, o200k_harmony, p50k_base, p50k_base_singleton,
    p50k_edit, r50k_base, CoreBPE, Rank,
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
    let decoded = bpe.decode(&tokens).unwrap();
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
    bpe1.decode(&tokens).unwrap();
    // println!("p50k_base encode/decode 1: {:?}", now.elapsed());

    //let now = std::time::Instant::now();
    let bpe2 = p50k_base_singleton();
    // println!("p50k_base_singleton load 2: {:?}", now.elapsed());
    // let now = std::time::Instant::now();
    let tokens = bpe2.encode_with_special_tokens("This is a test         with a lot of spaces");
    bpe2.decode(&tokens).unwrap();
    // println!("p50k_base encode/decode 2: {:?}", now.elapsed());
}

#[test]
fn test_decode_bytes_non_utf8() {
    let bpe = r50k_base().unwrap();
    // Token 49426 in GPT-2/r50k_base is not valid UTF-8 on its own
    assert!(bpe.decode(&[49426]).is_err());
    // decode_bytes should succeed and return the raw bytes
    let bytes = bpe.decode_bytes(&[49426]).unwrap();
    assert!(!bytes.is_empty());
}

/// Ported from upstream tiktoken test_simple_repeated
#[test]
fn test_simple_repeated() {
    let bpe = r50k_base().unwrap();
    test_decode(&bpe, "0", vec![15]);
    test_decode(&bpe, "00", vec![405]);
    test_decode(&bpe, "000", vec![830]);
    test_decode(&bpe, "0000", vec![2388]);
    test_decode(&bpe, "00000", vec![20483]);
    test_decode(&bpe, "000000", vec![10535]);
    test_decode(&bpe, "0000000", vec![24598]);
    test_decode(&bpe, "00000000", vec![8269]);
    test_decode(&bpe, "000000000", vec![10535, 830]);
    test_decode(&bpe, "0000000000", vec![8269, 405]);
    test_decode(&bpe, "00000000000", vec![8269, 830]);
    test_decode(&bpe, "000000000000", vec![8269, 2388]);
    test_decode(&bpe, "0000000000000", vec![8269, 20483]);
    test_decode(&bpe, "00000000000000", vec![8269, 10535]);
    test_decode(&bpe, "000000000000000", vec![8269, 24598]);
    test_decode(&bpe, "0000000000000000", vec![25645]);
    test_decode(&bpe, "00000000000000000", vec![8269, 10535, 830]);
}

/// Ported from upstream tiktoken test_simple_regex
#[test]
fn test_simple_regex() {
    let bpe = cl100k_base().unwrap();
    test_decode(&bpe, "rer", vec![38149]);
    test_decode(&bpe, "'rer", vec![2351, 81]);
    test_decode(&bpe, "today\n ", vec![31213, 198, 220]);
    test_decode(&bpe, "today\n \n", vec![31213, 27907]);
    test_decode(&bpe, "today\n  \n", vec![31213, 14211]);
}

/// Ported from upstream tiktoken test_encode_empty
#[test]
fn test_encode_empty() {
    let bpe = r50k_base().unwrap();
    assert_eq!(bpe.encode_with_special_tokens(""), vec![] as Vec<Rank>);
}

/// Ported from upstream tiktoken test_catastrophically_repetitive.
/// Validates that possessive quantifiers prevent catastrophic backtracking.
#[test]
fn test_catastrophically_repetitive() {
    let encoders: Vec<(&str, CoreBPE)> = vec![
        ("r50k", r50k_base().unwrap()),
        ("p50k", p50k_base().unwrap()),
        ("p50k_edit", p50k_edit().unwrap()),
        ("cl100k", cl100k_base().unwrap()),
        ("o200k", o200k_base().unwrap()),
        ("o200k_harmony", o200k_harmony().unwrap()),
    ];
    for (name, bpe) in &encoders {
        for c in ["^", "0", "a", "'s", " ", "\n"] {
            let big_value: String = c.repeat(10_000);
            let tokens = bpe.encode_with_special_tokens(&big_value);
            let decoded = bpe.decode(&tokens).unwrap();
            assert_eq!(decoded, big_value, "roundtrip failed for {name} with {c:?}");

            let big_value = format!(" {big_value}");
            let tokens = bpe.encode_with_special_tokens(&big_value);
            let decoded = bpe.decode(&tokens).unwrap();
            assert_eq!(
                decoded, big_value,
                "roundtrip failed for {name} with ' ' + {c:?}"
            );

            let big_value = format!("{big_value}\n");
            let tokens = bpe.encode_with_special_tokens(&big_value);
            let decoded = bpe.decode(&tokens).unwrap();
            assert_eq!(
                decoded, big_value,
                "roundtrip failed for {name} with {c:?} + newline"
            );
        }
    }
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

#[test]
fn test_encode_as_generic() {
    let bpe = cl100k_base().unwrap();
    let text = "hello world<|endoftext|>";

    // encode_ordinary_as: test all FromRank types against encode_ordinary
    let ordinary = bpe.encode_ordinary(text);
    let as_rank: Vec<Rank> = bpe.encode_ordinary_as(text);
    let as_usize: Vec<usize> = bpe.encode_ordinary_as(text);
    let as_u64: Vec<u64> = bpe.encode_ordinary_as(text);
    let as_i64: Vec<i64> = bpe.encode_ordinary_as(text);
    assert_eq!(ordinary, as_rank);
    assert_eq!(
        ordinary.iter().map(|&t| t as usize).collect::<Vec<_>>(),
        as_usize
    );
    assert_eq!(
        ordinary.iter().map(|&t| t as u64).collect::<Vec<_>>(),
        as_u64
    );
    assert_eq!(
        ordinary.iter().map(|&t| t as i64).collect::<Vec<_>>(),
        as_i64
    );

    // encode_with_special_tokens_as: uses special tokens, so output differs from ordinary
    let special = bpe.encode_with_special_tokens(text);
    let special_usize: Vec<usize> = bpe.encode_with_special_tokens_as(text);
    assert_ne!(ordinary.len(), special.len()); // special token is one token, not split
    assert_eq!(
        special.iter().map(|&t| t as usize).collect::<Vec<_>>(),
        special_usize
    );

    // encode_as: test tuple return
    let allowed = bpe.special_tokens();
    let (enc, last_len) = bpe.encode(text, &allowed);
    let (enc_usize, last_len_as): (Vec<usize>, usize) = bpe.encode_as(text, &allowed);
    assert_eq!(
        enc.iter().map(|&t| t as usize).collect::<Vec<_>>(),
        enc_usize
    );
    assert_eq!(last_len, last_len_as);
}

fn assert_count_matches_encode(bpe: &CoreBPE, text: &str) {
    assert_eq!(bpe.count_ordinary(text), bpe.encode_ordinary(text).len());
    assert_eq!(
        bpe.count_with_special_tokens(text),
        bpe.encode_with_special_tokens(text).len()
    );
}

#[test]
fn count_matches_encode() {
    let texts = [
        "hello world",
        "",
        "🍌This is a sentence",
        "我想借几本汉语书",
        &"ab ".repeat(512),
    ];
    let bpes: Vec<CoreBPE> = vec![
        r50k_base().unwrap(),
        p50k_base().unwrap(),
        cl100k_base().unwrap(),
        o200k_base().unwrap(),
        o200k_harmony().unwrap(),
    ];
    for bpe in &bpes {
        for text in &texts {
            assert_count_matches_encode(bpe, text);
        }
    }
}
