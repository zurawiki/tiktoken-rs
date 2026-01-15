use super::vendor_tiktoken::*;
use anyhow::anyhow;
use anyhow::Result;
use fancy_regex::Regex;
use lazy_static::lazy_static;
use rustc_hash::FxHashMap as HashMap;
use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::num::NonZeroU64;
use std::thread;

pub struct FakeThreadId(NonZeroU64);

fn thread_slot() -> usize {
    // It's easier to use unsafe than to use nightly. Rust has this nice u64 thread id counter
    // that works great for our use case of avoiding collisions in our array. Unfortunately,
    // it's private. However, there are only so many ways you can layout a u64, so just transmute
    // https://github.com/rust-lang/rust/issues/67939
    const _: [u8; 8] = [0; std::mem::size_of::<std::thread::ThreadId>()];
    const _: [u8; 8] = [0; std::mem::size_of::<FakeThreadId>()];
    let x = unsafe {
        std::mem::transmute::<std::thread::ThreadId, FakeThreadId>(thread::current().id()).0
    };
    (u64::from(x) as usize) % MAX_NUM_THREADS
}

const BPE_HEAP_THRESHOLD: usize = 128;
const NONE: usize = usize::MAX;
const PAT_KIND_UNKNOWN: u8 = 0;
const PAT_KIND_O200K: u8 = 1;

lazy_static! {
    static ref O200K_FAST_REGEX: regex::Regex = {
        // Same as `O200K_BASE_PAT_STR`, but without the lookahead branch `\\s+(?!\\S)`.
        // We reproduce that behavior in code for ASCII-only inputs (see `ascii_ws_split_end`).
        let pat = crate::O200K_BASE_PAT_STR.replace("|\\s+(?!\\S)|", "|");
        regex::Regex::new(&pat).expect("compile o200k_base fast regex")
    };
}

#[derive(Default)]
struct BpeScratch {
    parts: Vec<(usize, Rank)>,
    prev: Vec<usize>,
    next: Vec<usize>,
    end: Vec<usize>,
    ver: Vec<u32>,
    heap: BinaryHeap<Reverse<(Rank, usize, u32)>>,
}

fn byte_pair_encode_len_in_place(
    piece: &[u8],
    ranks: &HashMap<Vec<u8>, Rank>,
    parts: &mut Vec<(usize, Rank)>,
) -> usize {
    if piece.is_empty() {
        return 0;
    }
    if piece.len() == 1 {
        return 1;
    }

    parts.clear();
    // two sentinels at the end
    parts.reserve(piece.len().saturating_add(2));

    let mut min_rank: (Rank, usize) = (Rank::MAX, usize::MAX);
    for i in 0..piece.len() - 1 {
        let rank = *ranks.get(&piece[i..i + 2]).unwrap_or(&Rank::MAX);
        if rank < min_rank.0 {
            min_rank = (rank, i);
        }
        parts.push((i, rank));
    }
    parts.push((piece.len() - 1, Rank::MAX));
    parts.push((piece.len(), Rank::MAX));

    let get_rank = |parts: &Vec<(usize, Rank)>, i: usize| {
        if (i + 3) < parts.len() {
            *ranks
                .get(&piece[parts[i].0..parts[i + 3].0])
                .unwrap_or(&Rank::MAX)
        } else {
            Rank::MAX
        }
    };

    while min_rank.0 != Rank::MAX {
        let i = min_rank.1;
        if i > 0 {
            parts[i - 1].1 = get_rank(parts, i - 1);
        }
        parts[i].1 = get_rank(parts, i);
        parts.remove(i + 1);

        min_rank = (Rank::MAX, usize::MAX);
        for (i, &(_, rank)) in parts[..parts.len() - 1].iter().enumerate() {
            if rank < min_rank.0 {
                min_rank = (rank, i);
            }
        }
    }

    // final token count is the number of intervals
    parts.len() - 1
}

fn byte_pair_encode_len_heap(
    piece: &[u8],
    ranks: &HashMap<Vec<u8>, Rank>,
    scratch: &mut BpeScratch,
) -> usize {
    let n = piece.len();
    debug_assert!(n >= 2);

    scratch.heap.clear();
    scratch.prev.resize(n, NONE);
    scratch.next.resize(n, NONE);
    scratch.end.resize(n, 0);
    scratch.ver.resize(n, 0);

    for i in 0..n {
        scratch.prev[i] = if i == 0 { NONE } else { i - 1 };
        scratch.next[i] = if i + 1 < n { i + 1 } else { NONE };
        scratch.end[i] = i + 1;
        scratch.ver[i] = 0;
    }

    for i in 0..n - 1 {
        let j = i + 1;
        let rank = *ranks.get(&piece[i..scratch.end[j]]).unwrap_or(&Rank::MAX);
        if rank != Rank::MAX {
            scratch.heap.push(Reverse((rank, i, 0)));
        }
    }

    let mut segments = n;
    while let Some(Reverse((rank, i, v))) = scratch.heap.pop() {
        if scratch.end[i] == 0 {
            continue;
        }
        if scratch.ver[i] != v {
            continue;
        }
        let j = scratch.next[i];
        if j == NONE || scratch.end[j] == 0 {
            continue;
        }

        let cur = *ranks.get(&piece[i..scratch.end[j]]).unwrap_or(&Rank::MAX);
        if cur != rank {
            // should be rare, but handle stale heap items defensively
            if cur != Rank::MAX {
                scratch.heap.push(Reverse((cur, i, v)));
            }
            continue;
        }

        // merge segment i with segment j
        let k = scratch.next[j];
        scratch.end[i] = scratch.end[j];
        scratch.next[i] = k;
        if k != NONE {
            scratch.prev[k] = i;
        }
        scratch.end[j] = 0; // mark dead
        segments -= 1;

        // update i (pair i,k) and prev(i) (pair prev,i)
        scratch.ver[i] = scratch.ver[i].wrapping_add(1);
        let vi = scratch.ver[i];
        if k != NONE {
            let r = *ranks.get(&piece[i..scratch.end[k]]).unwrap_or(&Rank::MAX);
            if r != Rank::MAX {
                scratch.heap.push(Reverse((r, i, vi)));
            }
        }

        let p = scratch.prev[i];
        if p != NONE {
            scratch.ver[p] = scratch.ver[p].wrapping_add(1);
            let vp = scratch.ver[p];
            let r = *ranks.get(&piece[p..scratch.end[i]]).unwrap_or(&Rank::MAX);
            if r != Rank::MAX {
                scratch.heap.push(Reverse((r, p, vp)));
            }
        }
    }

    segments
}

fn byte_pair_encode_len(
    piece: &[u8],
    ranks: &HashMap<Vec<u8>, Rank>,
    scratch: &mut BpeScratch,
) -> usize {
    let n = piece.len();
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    if n == 2 {
        return if ranks.get(piece).is_some() { 1 } else { 2 };
    }
    if n >= BPE_HEAP_THRESHOLD {
        return byte_pair_encode_len_heap(piece, ranks, scratch);
    }
    byte_pair_encode_len_in_place(piece, ranks, &mut scratch.parts)
}

thread_local! {
    static BPE_SCRATCH: RefCell<BpeScratch> = RefCell::new(BpeScratch::default());
}

#[inline(always)]
fn ascii_ws_split_end(bytes: &[u8], i: usize) -> Option<usize> {
    // Implements the effect of the regex branch `\\s+(?!\\S)` for ASCII-only inputs:
    // if we have >=2 whitespace bytes before a non-whitespace, consume all but the last,
    // leaving one whitespace byte available for the next token branch to take as a prefix.
    //
    // Important: if the whitespace run contains `\\r` or `\\n`, we must not split here because
    // the earlier regex branch `\\s*[\\r\\n]+` takes precedence and consumes up to the newline(s).
    let n = bytes.len();
    if i + 1 >= n {
        return None;
    }
    let b0 = bytes[i];
    let b1 = bytes[i + 1];
    if !b0.is_ascii_whitespace() || !b1.is_ascii_whitespace() {
        return None;
    }
    if b0 == b'\n' || b0 == b'\r' || b1 == b'\n' || b1 == b'\r' {
        return None;
    }

    let mut j = i + 2;
    while j < n {
        let b = bytes[j];
        if !b.is_ascii_whitespace() {
            break;
        }
        if b == b'\n' || b == b'\r' {
            return None;
        }
        j += 1;
    }
    if j < n {
        Some(j - 1)
    } else {
        None
    }
}

/// Rust API
impl CoreBPE {
    // ====================
    // Encoding
    // ====================

    // This function a copy of the similar function in python API, but it return
    // Rust's results and errors
    pub fn new(
        encoder: HashMap<Vec<u8>, Rank>,
        special_tokens_encoder: HashMap<String, Rank>,
        pattern: &str,
    ) -> Result<Self> {
        let regex = Regex::new(pattern)?;
        let pat_kind = if pattern == crate::O200K_BASE_PAT_STR {
            PAT_KIND_O200K
        } else {
            PAT_KIND_UNKNOWN
        };

        let special_regex = {
            let parts = special_tokens_encoder
                .keys()
                .map(|s| fancy_regex::escape(s))
                .collect::<Vec<_>>();
            Regex::new(&parts.join("|"))?
        };

        let decoder: HashMap<Rank, Vec<u8>> =
            encoder.iter().map(|(k, v)| (*v, k.clone())).collect();

        assert!(
            encoder.len() == decoder.len(),
            "Encoder and decoder must be of equal length; maybe you had duplicate token indices in your encoder?"
        );

        let special_tokens_decoder: HashMap<Rank, Vec<u8>> = special_tokens_encoder
            .iter()
            .map(|(k, v)| (*v, k.as_bytes().to_vec()))
            .collect();

        // Clone because I don't know how to tell Rust I'm not going to change the map
        let mut sorted_token_bytes: Vec<Vec<u8>> = encoder.keys().cloned().collect();
        sorted_token_bytes.sort();

        Ok(Self {
            encoder,
            special_tokens_encoder,
            decoder,
            special_tokens_decoder,
            pat_kind,
            regex_tls: (0..MAX_NUM_THREADS).map(|_| regex.clone()).collect(),
            special_regex_tls: (0..MAX_NUM_THREADS)
                .map(|_| special_regex.clone())
                .collect(),
            sorted_token_bytes,
        })
    }

    fn count_ordinary_with_regex(
        &self,
        regex: &Regex,
        text: &str,
        scratch: &mut BpeScratch,
    ) -> usize {
        let mut count: usize = 0;
        for mat in regex.find_iter(text) {
            let piece = mat.unwrap().as_str().as_bytes();
            if self.encoder.get(piece).is_some() {
                count += 1;
            } else {
                count += byte_pair_encode_len(piece, &self.encoder, scratch);
            }
        }
        count
    }

    fn count_ordinary_slow(&self, text: &str) -> usize {
        let regex = &self.regex_tls[thread_slot()];
        BPE_SCRATCH.with(|scratch| {
            let mut scratch = scratch.borrow_mut();
            self.count_ordinary_with_regex(regex, text, &mut *scratch)
        })
    }

    fn count_ordinary_o200k_ascii(&self, text: &str) -> usize {
        let bytes = text.as_bytes();
        let mut count: usize = 0;

        BPE_SCRATCH.with(|scratch| {
            let mut scratch = scratch.borrow_mut();
            let slow_regex = &self.regex_tls[thread_slot()];
            let mut i = 0;
            while i < bytes.len() {
                if let Some(end) = ascii_ws_split_end(bytes, i) {
                    let piece = &bytes[i..end];
                    if self.encoder.get(piece).is_some() {
                        count += 1;
                    } else {
                        count += byte_pair_encode_len(piece, &self.encoder, &mut *scratch);
                    }
                    i = end;
                    continue;
                }

                let m = O200K_FAST_REGEX.find_at(text, i);
                let Some(m) = m else {
                    count += self.count_ordinary_with_regex(slow_regex, &text[i..], &mut *scratch);
                    break;
                };
                if m.start() != i {
                    count += self.count_ordinary_with_regex(slow_regex, &text[i..], &mut *scratch);
                    break;
                }

                let piece = &bytes[m.start()..m.end()];
                if self.encoder.get(piece).is_some() {
                    count += 1;
                } else {
                    count += byte_pair_encode_len(piece, &self.encoder, &mut *scratch);
                }
                i = m.end();
            }
        });

        count
    }

    pub fn count_ordinary(&self, text: &str) -> usize {
        if self.pat_kind == PAT_KIND_O200K && text.len() >= 1024 && text.is_ascii() {
            return self.count_ordinary_o200k_ascii(text);
        }
        self.count_ordinary_slow(text)
    }

    pub fn count(&self, text: &str, allowed_special: &HashSet<&str>) -> usize {
        let special_regex = &self.special_regex_tls[thread_slot()];
        let regex = &self.regex_tls[thread_slot()];
        let mut count: usize = 0;

        BPE_SCRATCH.with(|scratch| {
            let mut scratch = scratch.borrow_mut();
            let mut start = 0;
            loop {
                let mut next_special;
                let mut start_find = start;
                loop {
                    next_special = special_regex.find_from_pos(text, start_find).unwrap();
                    match next_special {
                        Some(m) => {
                            if allowed_special.contains(&text[m.start()..m.end()]) {
                                break;
                            }
                            start_find = m.start() + 1;
                        }
                        None => break,
                    }
                }
                let end = next_special.map_or(text.len(), |m| m.start());

                for mat in regex.find_iter(&text[start..end]) {
                    let piece = mat.unwrap().as_str().as_bytes();
                    if self.encoder.get(piece).is_some() {
                        count += 1;
                    } else {
                        count += byte_pair_encode_len(piece, &self.encoder, &mut *scratch);
                    }
                }

                match next_special {
                    Some(m) => {
                        count += 1;
                        start = m.end();
                    }
                    None => break,
                }
            }
        });

        count
    }

    pub fn count_with_special_tokens(&self, text: &str) -> usize {
        let allowed_special = self.special_tokens();
        self.count(text, &allowed_special)
    }

    // ====================
    // Decoding
    // ====================

    /// Decode a vector of tokens into a valid UTF-8 String
    ///
    /// If unicode validation is not wanted, see _decode_native.
    pub fn decode(&self, tokens: Vec<Rank>) -> Result<String> {
        match String::from_utf8(self.decode_bytes(&tokens)?) {
            Ok(text) => Ok(text),
            Err(e) => Err(anyhow!("Unable to decode into a valid UTF-8 string: {}", e)),
        }
    }

    pub fn _decode_native_and_split(
        &self,
        tokens: Vec<Rank>,
    ) -> impl Iterator<Item = Vec<u8>> + '_ {
        tokens.into_iter().map(|token| {
            let token_bytes = self
                .decoder
                .get(&token)
                .unwrap_or_else(|| &self.special_tokens_decoder[&token]);
            token_bytes.clone()
        })
    }

    /// Tokenize a string and return the decoded tokens using the correct BPE model.
    ///
    /// This method takes a string, encodes it using the BPE model, and decodes the encoded tokens into
    /// a vector of strings. It can be used to tokenize a string and return the decoded tokens using the
    /// correct BPE model.
    ///
    /// # Examples
    ///
    /// ```
    ///     use tiktoken_rs::cl100k_base;
    ///     let bpe = cl100k_base().unwrap();
    ///     let tokenized: Result<Vec<_>, _> = bpe
    ///         .split_by_token("This is a test         with a lot of spaces", true);
    ///     let tokenized = tokenized.unwrap();
    ///     assert_eq!(
    ///         tokenized,
    ///         vec!["This", " is", " a", " test", "        ", " with", " a", " lot", " of", " spaces"]
    ///     );
    /// ```
    ///
    /// # Arguments
    ///
    /// * text: A string slice containing the text to be tokenized.
    /// * use_special_tokens: A boolean indicating whether to use the special tokens in the BPE model.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<String>>`: A Result containing a vector of decoded tokens as strings, or an error
    ///   if the string cannot be converted into a valid UTF-8 string.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    ///
    /// * The input text cannot be converted into a valid UTF-8 string during the decoding process.
    ///
    pub fn split_by_token<'a>(
        &'a self,
        text: &'a str,
        use_special_tokens: bool,
    ) -> Result<Vec<String>> {
        self.split_by_token_iter(text, use_special_tokens).collect()
    }

    /// Iterator for decoding and splitting a String.
    /// See `split_by_token` for more details.
    pub fn split_by_token_iter<'a>(
        &'a self,
        text: &'a str,
        use_special_tokens: bool,
    ) -> impl Iterator<Item = Result<String>> + 'a {
        // First, encode the text using the BPE model
        let encoded = match use_special_tokens {
            true => self.encode_with_special_tokens(text),
            false => self.encode_ordinary(text),
        };

        self._decode_native_and_split(encoded).map(|token| {
            // Map each token to a Result<String>
            Ok(String::from_utf8_lossy(token.as_slice()).to_string())
        })
    }

    /// Tokenize a string and return the decoded tokens using the correct BPE model.
    /// This method is equivalent to `split_by_token(text, false)`.
    pub fn split_by_token_ordinary<'a>(&'a self, text: &'a str) -> Result<Vec<String>> {
        self.split_by_token(text, false)
    }

    /// Iterator for decoding and splitting a String.
    /// This method is equivalent to `split_by_token_iter(text, false)`.
    pub fn split_by_token_ordinary_iter<'a>(
        &'a self,
        text: &'a str,
    ) -> impl Iterator<Item = Result<String>> + 'a {
        self.split_by_token_iter(text, false)
    }
}
