/*!

Rust library for tokenizing text for GPT using tiktoken

## Counting token length

```
use tiktoken_rs::p50k_base;

let bpe = p50k_base().unwrap();
let tokens = bpe.encode_with_special_tokens(
  "This is a sentence   with spaces"
);
println!("Token count: {}", tokens.len());
```

## Examples
For full working examples for all supported features see [examples](https://github.com/zurawiki/tiktoken-rs/tree/main/examples) directory in the repository.

*/
mod model;
mod singleton;
mod tiktoken_ext;
mod vendor_tiktoken;

pub use model::*;
pub use singleton::*;
pub use tiktoken_ext::openai_public::*;
pub use vendor_tiktoken::*;
