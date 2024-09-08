#!/bin/bash

TEST_RUNNER=../writing-a-c-compiler-tests/test_compiler

set -euxo pipefail
  cp /dev/null src/lexer/test.rs
  cp /dev/null src/parser/test.rs
  cargo build --features test_gen
  arch -x86_64 $TEST_RUNNER target/debug/writing-a-c-compiler --chapter 6 --stage lex --bitwise --compound --increment
  arch -x86_64 $TEST_RUNNER target/debug/writing-a-c-compiler --chapter 6 --stage parse --bitwise --compound --increment
  rustfmt src/lexer/test.rs
  rustfmt src/parser/test.rs
