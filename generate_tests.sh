#!/bin/bash

TEST_RUNNER=../writing-a-c-compiler-tests/test_compiler

set -euxo pipefail
  cp /dev/null src/lexer/test.rs
  cp /dev/null src/parser/test.rs
  cp /dev/null src/semantic/test.rs
  cp /dev/null src/tacky/test.rs
  cargo build --features test_gen
  arch -x86_64 $TEST_RUNNER target/debug/writing-a-c-compiler --verbose --chapter 10 --stage lex --bitwise --compound --increment --goto --switch
  arch -x86_64 $TEST_RUNNER target/debug/writing-a-c-compiler --verbose --chapter 10 --stage parse --bitwise --compound --increment --goto --switch
  arch -x86_64 $TEST_RUNNER target/debug/writing-a-c-compiler --verbose --chapter 10 --stage validate --bitwise --compound --increment --goto --switch
  arch -x86_64 $TEST_RUNNER target/debug/writing-a-c-compiler --verbose --chapter 9 --stage tacky --bitwise --compound --increment --goto --switch
  cargo clean
  rustfmt src/lexer/test.rs
  rustfmt src/parser/test.rs
  rustfmt src/semantic/test.rs
  rustfmt src/tacky/test.rs