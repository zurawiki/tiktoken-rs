from .wrap import Tiktoken, TiktokenTokenizer


if __name__ == "__main__":
    tiktoken = Tiktoken()

    print("encode...")
    encoded = tiktoken.encode_with_special_tokens({
        "tokenizer": TiktokenTokenizer.P50kBase,
        "text": "This is a test         with a lot of spaces"
    })

    print("decode...")
    decoded = tiktoken.decode({
        "tokenizer": TiktokenTokenizer.P50kBase,
        "tokens": encoded
    })

    print("encoded:", encoded)
    print("decoded:", decoded)
    assert decoded == "This is a test         with a lot of spaces"
