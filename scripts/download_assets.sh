#!/usr/bin/env bash

set -euo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel)"
ASSET_DIR="${ASSET_DIR:-$REPO_ROOT/tiktoken-rs/assets}"

ASSETS=$(cat <<EOF
1ce1664773c50f3e0cc8842619a93edc4624525b728b188a9e0be33b7726adc5 https://openaipublic.blob.core.windows.net/gpt-2/encodings/main/vocab.bpe
196139668be63f3b5d6574427317ae82f612a97c5d1cdaf36ed2256dbf636783 https://openaipublic.blob.core.windows.net/gpt-2/encodings/main/encoder.json
306cd27f03c1a714eca7108e03d66b7dc042abe8c258b44c199a7ed9838dd930 https://openaipublic.blob.core.windows.net/encodings/r50k_base.tiktoken
94b5ca7dff4d00767bc256fdd1b27e5b17361d7b8a5f968547f9f23eb70d2069 https://openaipublic.blob.core.windows.net/encodings/p50k_base.tiktoken
223921b76ee99bde995b7ff738513eef100fb51d18c93597a113bcffe865b2a7 https://openaipublic.blob.core.windows.net/encodings/cl100k_base.tiktoken
446a9538cb6c348e3516120d7c08b09f57c36495e2acfffe59a5bf8b0cfb1a2d https://openaipublic.blob.core.windows.net/encodings/o200k_base.tiktoken
EOF
)

echo "Downloading assets..."
mkdir -p "$ASSET_DIR"
while read -r expected_hash asset; do
    echo "Downloading $asset..."
    output="$ASSET_DIR/$(basename "$asset")"
    curl \
      --proto '=https' --tlsv1.2 -Sf -L \
      -o "$output" \
      "$asset"

    actual_hash="$(shasum -a 256 "$output" | awk '{print $1}')"
    if [[ "$actual_hash" != "$expected_hash" ]]; then
        echo "Hash mismatch for $asset" >&2
        echo "expected: $expected_hash" >&2
        echo "actual:   $actual_hash" >&2
        exit 1
    fi
done <<< "$ASSETS"
