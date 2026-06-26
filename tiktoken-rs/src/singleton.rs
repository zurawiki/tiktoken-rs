use std::sync::LazyLock;

use crate::vendor_tiktoken::CoreBPE;

use crate::{cl100k_base, o200k_base, o200k_harmony, p50k_base, p50k_edit, r50k_base};

/// Returns a singleton instance of the r50k_base tokenizer. (also known as `gpt2`)
/// Use for GPT-3 models like `davinci`
///
/// This function will only initialize the tokenizer once, and then return a reference the tokenizer
pub fn r50k_base_singleton() -> &'static CoreBPE {
    static R50K_BASE: LazyLock<CoreBPE> = LazyLock::new(|| r50k_base().unwrap());
    &R50K_BASE
}

/// Returns a singleton instance of the p50k_base tokenizer.
/// Use for Code models, `text-davinci-002`, `text-davinci-003`
///
/// This function will only initialize the tokenizer once, and then return a reference the tokenizer.
pub fn p50k_base_singleton() -> &'static CoreBPE {
    static P50K_BASE: LazyLock<CoreBPE> = LazyLock::new(|| p50k_base().unwrap());
    &P50K_BASE
}

/// Returns a singleton instance of the p50k_edit tokenizer.
/// Use for edit models like `text-davinci-edit-001`, `code-davinci-edit-001`
///
/// This function will only initialize the tokenizer once, and then return a reference the tokenizer.
pub fn p50k_edit_singleton() -> &'static CoreBPE {
    static P50K_EDIT: LazyLock<CoreBPE> = LazyLock::new(|| p50k_edit().unwrap());
    &P50K_EDIT
}

/// Returns a singleton instance of the cl100k_base tokenizer.
/// Use for ChatGPT models, `text-embedding-ada-002`
///
/// This function will only initialize the tokenizer once, and then return a reference the tokenizer
pub fn cl100k_base_singleton() -> &'static CoreBPE {
    static CL100K_BASE: LazyLock<CoreBPE> = LazyLock::new(|| cl100k_base().unwrap());
    &CL100K_BASE
}

/// Returns a singleton instance of the o200k_base tokenizer.
/// Use for GPT-5, GPT-4.1, GPT-4o, and other `o` series models like `o1`, `o3`, and `o4`.
///
/// This function will only initialize the tokenizer once, and then return a reference the tokenizer
pub fn o200k_base_singleton() -> &'static CoreBPE {
    static O200K_BASE: LazyLock<CoreBPE> = LazyLock::new(|| o200k_base().unwrap());
    &O200K_BASE
}

/// Returns a singleton instance of the o200k_harmony tokenizer.
/// Use for gpt-oss models like `gpt-oss-20b`, `gpt-oss-120b`.
///
/// This function will only initialize the tokenizer once, and then return a reference the tokenizer
pub fn o200k_harmony_singleton() -> &'static CoreBPE {
    static O200K_HARMONY: LazyLock<CoreBPE> = LazyLock::new(|| o200k_harmony().unwrap());
    &O200K_HARMONY
}
