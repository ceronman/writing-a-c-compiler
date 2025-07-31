#!/bin/bash

TEST_RUNNER=../writing-a-c-compiler-tests/test_compiler

set -euxo pipefail
cargo build --features test_gen

#rm -rf src/lexer/test.rs src/lexer/test/*
#arch -x86_64 $TEST_RUNNER target/debug/writing-a-c-compiler --verbose --chapter 17 --stage lex --bitwise --compound --increment --goto --switch --nan
#rustfmt src/lexer/test/*

#rm -rf src/parser/test.rs src/parser/test/*
#arch -x86_64 $TEST_RUNNER target/debug/writing-a-c-compiler --verbose --chapter 17 --stage parse --bitwise --compound --increment --goto --switch --nan
#rustfmt src/parser/test/*

#rm -rf src/semantic/test.rs src/semantic/test/*
#arch -x86_64 $TEST_RUNNER target/debug/writing-a-c-compiler --verbose --chapter 17 --stage validate --bitwise --compound --increment --goto --switch --nan
#rustfmt src/semantic/test/*

rm -rf src/tacky/test.rs src/tacky/test/*
arch -x86_64 $TEST_RUNNER target/debug/writing-a-c-compiler --verbose --chapter 17 --stage tacky --bitwise --compound --increment --goto --switch --nan
rustfmt src/tacky/test.rs

cargo clean
