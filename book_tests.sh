#!/bin/bash

TEST_RUNNER=../writing-a-c-compiler-tests/test_compiler

set -euxo pipefail
  cargo build
  arch -x86_64 $TEST_RUNNER target/debug/writing-a-c-compiler \
    --chapter 20 \
    --bitwise \
    --compound \
    --increment \
    --goto \
    --switch \
    --nan \
    --union \
    --verbose
#    --latest-only \
#    --int-only \
#    --failfast \

