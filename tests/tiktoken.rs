use rustc_hash::FxHashMap as HashMap;

use tiktoken_rs::tiktoken::*;

#[test]
fn very_simple_test() {
    let mut ranks = HashMap::default();
    ranks.insert(b"ab".to_vec(), 1);
    ranks.insert(b"cd".to_vec(), 2);

    let res = byte_pair_split(b"abcd", &ranks);
    assert_eq!(res, vec![b"ab", b"cd"]);
}

#[test]
fn p50k_base_test() {
    let bpe = p50k_base().unwrap();
    let tokens = bpe.encode_with_special_tokens("This is a test         with a lot of spaces");
    let decoded = bpe.decode(tokens.clone()).unwrap();
    assert_eq!(decoded, "This is a test         with a lot of spaces");
    assert_eq!(
        tokens,
        vec![1212, 318, 257, 1332, 50263, 351, 257, 1256, 286, 9029]
    );
}

#[test]
fn r50k_base_test() {
    let bpe = r50k_base().unwrap();
    let tokens = bpe.encode_with_special_tokens("This is a test         with a lot of spaces");
    let decoded = bpe.decode(tokens.clone()).unwrap();
    assert_eq!(decoded, "This is a test         with a lot of spaces");
    println!("{:?}", tokens);
    assert_eq!(
        tokens,
        vec![
            1212, 318, 257, 1332, 220, 220, 220, 220, 220, 220, 220, 220, 351, 257, 1256, 286, 9029
        ]
    );
}

#[test]
fn p50k_base_singleton_test() {
    // let now = std::time::Instant::now();
    let bpe1 = p50k_base_singleton();
    // println!("p50k_base_singleton load 1: {:?}", now.elapsed());
    // let now = std::time::Instant::now();
    {
        let guard = bpe1.lock();
        let tokens =
            guard.encode_with_special_tokens("This is a test         with a lot of spaces");
        guard.decode(tokens.clone()).unwrap();
    }
    // println!("p50k_base encode/decode 1: {:?}", now.elapsed());

    //let now = std::time::Instant::now();
    let bpe2 = p50k_base_singleton();
    // println!("p50k_base_singleton load 2: {:?}", now.elapsed());
    // let now = std::time::Instant::now();
    {
        let guard = bpe2.lock();
        let tokens =
            guard.encode_with_special_tokens("This is a test         with a lot of spaces");
        guard.decode(tokens.clone()).unwrap();
    }
    // println!("p50k_base encode/decode 2: {:?}", now.elapsed());
}
