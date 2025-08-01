use super::{assert_error, assert_parse};

#[test]
fn test_invalid_parse_double_operation() {
    assert_error(
        r#"
        int main(void) {
            return 1 * / 2;
                     //^ Expected expression, but found '/'
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_extra_credit_bitwise_double_operator() {
    assert_error(
        r#"
        int main(void) {
            return 1 | | 2;
                     //^ Expected expression, but found '|'
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_imbalanced_paren() {
    assert_error(
        r#"
        int main(void) {
            return 1 + (2;
                       //^ Expected ')', but found ';'
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_malformed_paren() {
    assert_error(
        r#"
        int main(void) {
            return 2 (- 3);
                   //^ Expected ';', but found '('
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_misplaced_semicolon() {
    assert_error(
        r#"
        int main(void) {
            return 1 + (2;)
                       //^ Expected ')', but found ';'
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_missing_first_op() {
    assert_error(
        r#"
        int main(void) {
            return /3;
                 //^ Expected expression, but found '/'
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_missing_open_paren() {
    assert_error(
        r#"
        int main(void) {
            return 1 + 2);
                      //^ Expected ';', but found ')'
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_missing_second_op() {
    assert_error(
        r#"
        int main(void) {
            return 1 + ;
                     //^ Expected expression, but found ';'
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_no_semicolon() {
    assert_error(
        r#"
        int main(void) {
            return 2*2
        }
      //^ Expected ';', but found '}'
    "#,
    );
}

#[test]
fn test_valid_add() {
    let src = r#"
        int main(void) {
            return 1 + 2;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <8>  [+]
                            ├── <5> Constant Int [1]
                            ╰── <7> Constant Int [2]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_associativity() {
    let src = r#"
        int main(void) {
            return 1 - 2 - 3;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <11>  [-]
                            ├── <8>  [-]
                            │   ├── <5> Constant Int [1]
                            │   ╰── <7> Constant Int [2]
                            ╰── <10> Constant Int [3]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_associativity_2() {
    let src = r#"
        int main(void) {
            return 6 / 3 / 2;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <11>  [/]
                            ├── <8>  [/]
                            │   ├── <5> Constant Int [6]
                            │   ╰── <7> Constant Int [3]
                            ╰── <10> Constant Int [2]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_associativity_3() {
    let src = r#"
        int main(void) {
            return (3 / 2 * 4) + (5 - 4 + 3);
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <22>  [+]
                            ├── <12>  [*]
                            │   ├── <8>  [/]
                            │   │   ├── <5> Constant Int [3]
                            │   │   ╰── <7> Constant Int [2]
                            │   ╰── <10> Constant Int [4]
                            ╰── <21>  [+]
                                ├── <17>  [-]
                                │   ├── <14> Constant Int [5]
                                │   ╰── <16> Constant Int [4]
                                ╰── <19> Constant Int [3]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_associativity_and_precedence() {
    let src = r#"
        int main(void) {
            return 5 * 4 / 2 -
                3 % (2 + 1);
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <21>  [-]
                            ├── <11>  [/]
                            │   ├── <8>  [*]
                            │   │   ├── <5> Constant Int [5]
                            │   │   ╰── <7> Constant Int [4]
                            │   ╰── <10> Constant Int [2]
                            ╰── <20>  [%]
                                ├── <13> Constant Int [3]
                                ╰── <19>  [+]
                                    ├── <15> Constant Int [2]
                                    ╰── <17> Constant Int [1]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_div() {
    let src = r#"
        int main(void) {
            return 4 / 2;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <8>  [/]
                            ├── <5> Constant Int [4]
                            ╰── <7> Constant Int [2]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_div_neg() {
    let src = r#"
        int main(void) {
            return (-12) / 5;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <11>  [/]
                            ├── <8> Unary [-]
                            │   ╰── <6> Constant Int [12]
                            ╰── <10> Constant Int [5]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_bitwise_and() {
    let src = r#"
        int main(void) {
            return 3 & 5;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <8>  [&]
                            ├── <5> Constant Int [3]
                            ╰── <7> Constant Int [5]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_bitwise_or() {
    let src = r#"
        int main(void) {
            return 1 | 2;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <8>  [|]
                            ├── <5> Constant Int [1]
                            ╰── <7> Constant Int [2]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_bitwise_precedence() {
    let src = r#"
        int main(void) {
            return 80 >> 2 | 1 ^ 5 & 7 << 1;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <20>  [|]
                            ├── <8>  [>>]
                            │   ├── <5> Constant Int [80]
                            │   ╰── <7> Constant Int [2]
                            ╰── <19>  [^]
                                ├── <10> Constant Int [1]
                                ╰── <18>  [&]
                                    ├── <12> Constant Int [5]
                                    ╰── <17>  [<<]
                                        ├── <14> Constant Int [7]
                                        ╰── <16> Constant Int [1]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_bitwise_shift_associativity() {
    let src = r#"
        int main(void) {
            return 33 << 4 >> 2;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <11>  [>>]
                            ├── <8>  [<<]
                            │   ├── <5> Constant Int [33]
                            │   ╰── <7> Constant Int [4]
                            ╰── <10> Constant Int [2]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_bitwise_shift_associativity_2() {
    let src = r#"
        int main(void) {
            return 33 >> 2 << 1;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <11>  [<<]
                            ├── <8>  [>>]
                            │   ├── <5> Constant Int [33]
                            │   ╰── <7> Constant Int [2]
                            ╰── <10> Constant Int [1]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_bitwise_shift_precedence() {
    let src = r#"
        int main(void) {
            return 40 << 4 + 12 >> 1;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <14>  [>>]
                            ├── <11>  [<<]
                            │   ├── <5> Constant Int [40]
                            │   ╰── <10>  [+]
                            │       ├── <7> Constant Int [4]
                            │       ╰── <9> Constant Int [12]
                            ╰── <13> Constant Int [1]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_bitwise_shiftl() {
    let src = r#"
        int main(void) {
            return 35 << 2;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <8>  [<<]
                            ├── <5> Constant Int [35]
                            ╰── <7> Constant Int [2]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_bitwise_shiftr() {
    let src = r#"
        int main(void) {
            return 1000 >> 4;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <8>  [>>]
                            ├── <5> Constant Int [1000]
                            ╰── <7> Constant Int [4]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_bitwise_shiftr_negative() {
    let src = r#"
        int main(void) {
            return -5 >> 30;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <10>  [>>]
                            ├── <7> Unary [-]
                            │   ╰── <6> Constant Int [5]
                            ╰── <9> Constant Int [30]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_bitwise_variable_shift_count() {
    let src = r#"
        int main(void) {
            return (4 << (2 * 2)) + (100 >> (1 + 2));
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <24>  [+]
                            ├── <13>  [<<]
                            │   ├── <5> Constant Int [4]
                            │   ╰── <11>  [*]
                            │       ├── <7> Constant Int [2]
                            │       ╰── <9> Constant Int [2]
                            ╰── <23>  [>>]
                                ├── <15> Constant Int [100]
                                ╰── <21>  [+]
                                    ├── <17> Constant Int [1]
                                    ╰── <19> Constant Int [2]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_bitwise_xor() {
    let src = r#"
        int main(void) {
            return 7 ^ 1;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <8>  [^]
                            ├── <5> Constant Int [7]
                            ╰── <7> Constant Int [1]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_mod() {
    let src = r#"
        int main(void) {
            return 4 % 2;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <8>  [%]
                            ├── <5> Constant Int [4]
                            ╰── <7> Constant Int [2]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_mult() {
    let src = r#"
        int main(void) {
            return 2 * 3;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <8>  [*]
                            ├── <5> Constant Int [2]
                            ╰── <7> Constant Int [3]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_parens() {
    let src = r#"
        int main(void) {
            return 2 * (3 + 4);
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <12>  [*]
                            ├── <5> Constant Int [2]
                            ╰── <11>  [+]
                                ├── <7> Constant Int [3]
                                ╰── <9> Constant Int [4]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_precedence() {
    let src = r#"
        int main(void) {
            return 2 + 3 * 4;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <11>  [+]
                            ├── <5> Constant Int [2]
                            ╰── <10>  [*]
                                ├── <7> Constant Int [3]
                                ╰── <9> Constant Int [4]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_sub() {
    let src = r#"
        int main(void) {
            return 1 - 2;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <8>  [-]
                            ├── <5> Constant Int [1]
                            ╰── <7> Constant Int [2]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_sub_neg() {
    let src = r#"
        int main(void) {
            return 2- -1;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <10>  [-]
                            ├── <5> Constant Int [2]
                            ╰── <9> Unary [-]
                                ╰── <8> Constant Int [1]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_unop_add() {
    let src = r#"
        int main(void) {
            return ~2 + 3;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <10>  [+]
                            ├── <7> Unary [~]
                            │   ╰── <6> Constant Int [2]
                            ╰── <9> Constant Int [3]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_unop_parens() {
    let src = r#"
        int main(void) {
            return ~(1 + 1);
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <11> Unary [~]
                            ╰── <10>  [+]
                                ├── <6> Constant Int [1]
                                ╰── <8> Constant Int [1]
    "#;
    assert_parse(src, expected);
}
