#!/bin/bash

TEST_RUNNER=../writing-a-c-compiler-tests/test_compiler

set -euxo pipefail
  cargo build
  arch -x86_64 $TEST_RUNNER target/debug/writing-a-c-compiler \
    --latest-only \
    --verbose \
    --failfast \
    --chapter 12 \
    --stage tacky \
    --bitwise \
    --compound \
    --increment \
    --goto \
    --switch
