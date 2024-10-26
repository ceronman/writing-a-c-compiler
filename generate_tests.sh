#!/bin/bash

TEST_RUNNER=../writing-a-c-compiler-tests/test_compiler

set -euxo pipefail
cargo build --features test_gen

#cp /dev/null src/lexer/test.rs
#arch -x86_64 $TEST_RUNNER target/debug/writing-a-c-compiler --verbose --chapter 13 --stage lex --bitwise --compound --increment --goto --switch
#rustfmt src/lexer/test.rs

#cp /dev/null src/parser/test.rs
#arch -x86_64 $TEST_RUNNER target/debug/writing-a-c-compiler --verbose --chapter 13 --stage parse --bitwise --compound --increment --goto --switch
#rustfmt src/parser/test.rs

#cp /dev/null src/semantic/test.rs
#arch -x86_64 $TEST_RUNNER target/debug/writing-a-c-compiler --verbose --chapter 13 --stage validate --bitwise --compound --increment --goto --switch
#rustfmt src/semantic/test.rs

cp /dev/null src/tacky/test.rs
arch -x86_64 $TEST_RUNNER target/debug/writing-a-c-compiler --verbose --chapter 13 --stage tacky --bitwise --compound --increment --goto --switch
rustfmt src/tacky/test.rs

cargo clean
