# tiktoken-rs — Contribution Game Plan

## Ready to review/merge

### Parallel encoding — PR #136
- rayon-based `encode_ordinary_parallel()` / `encode_with_special_tokens_parallel()`
- 5-6x speedup for texts >50KB
- Consider feature-gating `rayon` to keep default deps lean

### tiktoken CLI — PR #115
- `clap`-based CLI for quick token counting from stdin/files

### Count-only fast path for o200k ASCII — PR #135
- Count-only API was merged (#153), but this PR has an additional ASCII fast-path
- Cherry-pick the fast-path piece if benchmarks confirm the speedup

---

## Needs implementation

### Memory/regex performance — #39
- `fancy_regex` negative lookahead patterns cause memory spikes on large texts
- Long-term: explore regex-crate-compatible patterns to eliminate fancy-regex
