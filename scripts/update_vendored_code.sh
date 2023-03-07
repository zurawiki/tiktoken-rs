#!/usr/bin/env bash

# Run this script from the git root

# Fail on first error, on undefined variables, and on failures in pipelines.
set -euxo pipefail

rm -rf tiktoken-rs/src/vendor_tiktoken.rs
cp -rfv vendor/tiktoken/src/lib.rs tiktoken-rs/src/vendor_tiktoken.rs

