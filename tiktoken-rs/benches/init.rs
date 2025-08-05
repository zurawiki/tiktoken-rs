#![feature(test)]
extern crate test;

use tiktoken_rs::{cl100k_base, o200k_base, o200k_harmony, p50k_base, p50k_edit, r50k_base};

#[bench]
fn bench_init_o200k_harmony(b: &mut test::Bencher) {
    b.iter(|| o200k_harmony().unwrap());
}

#[bench]
fn bench_init_o200k_base(b: &mut test::Bencher) {
    b.iter(|| o200k_base().unwrap());
}

#[bench]
fn bench_init_cl100k_base(b: &mut test::Bencher) {
    b.iter(|| cl100k_base().unwrap());
}

#[bench]
fn bench_init_p50k_base(b: &mut test::Bencher) {
    b.iter(|| p50k_base().unwrap());
}

#[bench]
fn bench_init_p50k_edit(b: &mut test::Bencher) {
    b.iter(|| p50k_edit().unwrap());
}

#[bench]
fn bench_init_r50k_base(b: &mut test::Bencher) {
    b.iter(|| r50k_base().unwrap());
}
