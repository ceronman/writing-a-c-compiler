#!/usr/bin/env bash

set -euo pipefail

TEST_RUNNER=../writing-a-c-compiler-tests/test_compiler
MY_COMPILER=target/release/writing-a-c-compiler

ARGS=(
  --chapter 20
  --bitwise
  --compound
  --increment
  --goto
  --switch
  --nan
  --union
  --verbose
#  --latest-only
#  --failfast
)

cargo build --release

if [[ "$(uname -s)" == "Darwin" ]]; then
  # macOS: ensure x86_64 execution (use Rosetta on Apple Silicon)
  arch -x86_64 "$TEST_RUNNER" "$MY_COMPILER" "${ARGS[@]}"
else
  # Linux
  "$TEST_RUNNER" "$MY_COMPILER" "${ARGS[@]}"
fi
