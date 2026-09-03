//! Regression tests for decoding untrusted / out-of-vocabulary token sequences.
//!
//! `CoreBPE::_decode_native_and_split` (used by `split_by_token*` and the
//! streaming decode helpers) used to index `special_tokens_decoder` by an
//! absent key for any token id that is neither a regular token nor a special
//! token. That turned a single attacker-controlled out-of-range id into a
//! `panic!` (HashMap index out of bounds) — a denial-of-service footgun when
//! decoding token sequences coming from an untrusted source (e.g. model
//! output, a network protocol, or a mismatched vocab version).
//!
//! These tests pin the safe behaviour: unknown ids are skipped instead of
//! panicking, matching the non-panicking `decode()` / `decode_bytes()` path.

use tiktoken_rs::cl100k_base;

#[test]
fn decode_native_and_split_skips_unknown_token_without_panicking() {
    let bpe = cl100k_base().unwrap();

    // 0 and 1 are valid cl100k tokens; 999_999 is not in `decoder` nor in
    // `special_tokens_decoder`. Before the fix this panicked mid-iteration.
    let out: Vec<Vec<u8>> = bpe._decode_native_and_split(vec![0, 999_999, 1]).collect();

    // Only the two in-vocab tokens survive; the unknown id is dropped.
    assert_eq!(out.len(), 2, "unknown token should be skipped, got {out:?}");
}

#[test]
fn decode_native_and_split_all_unknown_yields_empty_without_panicking() {
    let bpe = cl100k_base().unwrap();

    let out: Vec<Vec<u8>> = bpe
        ._decode_native_and_split(vec![1_000_000, 1_000_001, 4_000_000_000])
        .collect();

    assert!(
        out.is_empty(),
        "all-unknown token sequence should yield no segments, got {out:?}"
    );
}

#[test]
fn split_by_token_iter_is_panic_free_for_untrusted_token_ids() {
    let bpe = cl100k_base().unwrap();

    // `split_by_token_iter` takes text and re-encodes internally (always valid
    // tokens), but the lower-level `_decode_native_and_split` it relies on must
    // stay robust if ever fed externally-controlled ids. Exercise it directly.
    let collected: Vec<Vec<u8>> = bpe
        ._decode_native_and_split(vec![42, 100_000_000, 7])
        .collect();
    assert_eq!(collected.len(), 2);
}
