#!/bin/bash

TEST_RUNNER=../writing-a-c-compiler-tests/test_compiler

set -euxo pipefail
  cargo build
  arch -x86_64 $TEST_RUNNER target/debug/writing-a-c-compiler \
    --verbose \
    --failfast \
    --chapter 10 \
    --stage lex \
    --bitwise \
    --compound \
    --increment \
    --goto \
    --switch
