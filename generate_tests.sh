#!/bin/bash

TEST_RUNNER=../writing-a-c-compiler-tests/test_compiler

set -euxo pipefail
  cargo build --features test_gen
  rm src/lexer/test.rs
  arch -x86_64 $TEST_RUNNER target/debug/writing-a-c-compiler --chapter 4 --stage lex
  rustfmt src/lexer/test.rs
