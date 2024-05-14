#!/usr/bin/env bash

# Fail on first error, on undefined variables, and on failures in pipelines.
set -euo pipefail

export ASSETS=$(cat <<EOF
https://openaipublic.blob.core.windows.net/gpt-2/encodings/main/vocab.bpe
https://openaipublic.blob.core.windows.net/gpt-2/encodings/main/encoder.json
https://openaipublic.blob.core.windows.net/encodings/r50k_base.tiktoken
https://openaipublic.blob.core.windows.net/encodings/p50k_base.tiktoken
https://openaipublic.blob.core.windows.net/encodings/cl100k_base.tiktoken
https://openaipublic.blob.core.windows.net/encodings/o200k_base.tiktoken
EOF
)

# Download assets
echo "Downloading assets..."
for asset in $ASSETS; do
    echo "Downloading $asset..."
    curl \
      --proto '=https' --tlsv1.2 -Sf -L \
      -o ./assets/$(basename $asset)\
      $asset
done
