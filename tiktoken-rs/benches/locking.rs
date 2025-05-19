#![feature(test)]
extern crate test;

use tiktoken_rs::{cl100k_base, cl100k_base_singleton};
static CONTENT: &str = "Hello world";

#[bench]
fn bench_cl100k_singleton_roundtrip(b: &mut test::Bencher) {
    let content = CONTENT.repeat(10);
    let bpe = cl100k_base_singleton();
    b.iter(|| {
        let tokens = bpe.encode_with_special_tokens(&content);
        bpe.decode(tokens).unwrap();
    });
}

#[bench]
fn bench_cl100k_roundtrip(b: &mut test::Bencher) {
    let content = CONTENT.repeat(10);

    b.iter(|| {
        let bpe = cl100k_base().unwrap();
        let tokens = bpe.encode_with_special_tokens(&content);
        bpe.decode(tokens).unwrap();
    });
}
