#!/bin/bash

TEST_RUNNER=../writing-a-c-compiler-tests/test_compiler

set -euxo pipefail
  cp /dev/null src/lexer/test.rs
  cp /dev/null src/parser/test.rs
  cargo build --features test_gen
  arch -x86_64 $TEST_RUNNER target/debug/writing-a-c-compiler --chapter 5 --stage lex --bitwise
  arch -x86_64 $TEST_RUNNER target/debug/writing-a-c-compiler --chapter 5 --stage parse --bitwise
  rustfmt src/lexer/test.rs
  rustfmt src/parser/test.rs
