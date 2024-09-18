#!/bin/bash

TEST_RUNNER=../writing-a-c-compiler-tests/test_compiler

set -euxo pipefail
  cp /dev/null src/lexer/test.rs
  cp /dev/null src/parser/test.rs
  cp /dev/null src/resolver/test.rs
  cp /dev/null src/tacky/test.rs
  cp /dev/null src/tacky/interpreter/test.rs
  cargo build --features test_gen
  arch -x86_64 $TEST_RUNNER target/debug/writing-a-c-compiler --verbose --chapter 8 --stage lex --bitwise --compound --increment --goto --switch
  arch -x86_64 $TEST_RUNNER target/debug/writing-a-c-compiler --verbose --chapter 8 --stage parse --bitwise --compound --increment --goto --switch
  arch -x86_64 $TEST_RUNNER target/debug/writing-a-c-compiler --verbose --chapter 8 --stage validate --bitwise --compound --increment --goto --switch
  arch -x86_64 $TEST_RUNNER target/debug/writing-a-c-compiler --verbose --chapter 8 --stage tacky --bitwise --compound --increment --goto --switch
  arch -x86_64 $TEST_RUNNER target/debug/writing-a-c-compiler --verbose --chapter 8 --stage tacky --bitwise --compound --increment --goto --switch -- --interpret
  cargo clean
  rustfmt src/lexer/test.rs
  rustfmt src/parser/test.rs
  rustfmt src/resolver/test.rs
  rustfmt src/tacky/test.rs
  rustfmt src/tacky/interpreter/test.rs
