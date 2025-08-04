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
                        ╰── <9>  [+]
                            ├── <6> Constant Int [1]
                            ╰── <8> Constant Int [2]
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
                        ╰── <12>  [-]
                            ├── <9>  [-]
                            │   ├── <6> Constant Int [1]
                            │   ╰── <8> Constant Int [2]
                            ╰── <11> Constant Int [3]
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
                        ╰── <12>  [/]
                            ├── <9>  [/]
                            │   ├── <6> Constant Int [6]
                            │   ╰── <8> Constant Int [3]
                            ╰── <11> Constant Int [2]
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
                        ╰── <23>  [+]
                            ├── <13>  [*]
                            │   ├── <9>  [/]
                            │   │   ├── <6> Constant Int [3]
                            │   │   ╰── <8> Constant Int [2]
                            │   ╰── <11> Constant Int [4]
                            ╰── <22>  [+]
                                ├── <18>  [-]
                                │   ├── <15> Constant Int [5]
                                │   ╰── <17> Constant Int [4]
                                ╰── <20> Constant Int [3]
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
                        ╰── <22>  [-]
                            ├── <12>  [/]
                            │   ├── <9>  [*]
                            │   │   ├── <6> Constant Int [5]
                            │   │   ╰── <8> Constant Int [4]
                            │   ╰── <11> Constant Int [2]
                            ╰── <21>  [%]
                                ├── <14> Constant Int [3]
                                ╰── <20>  [+]
                                    ├── <16> Constant Int [2]
                                    ╰── <18> Constant Int [1]
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
                        ╰── <9>  [/]
                            ├── <6> Constant Int [4]
                            ╰── <8> Constant Int [2]
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
                        ╰── <12>  [/]
                            ├── <9> Unary [-]
                            │   ╰── <7> Constant Int [12]
                            ╰── <11> Constant Int [5]
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
                        ╰── <9>  [&]
                            ├── <6> Constant Int [3]
                            ╰── <8> Constant Int [5]
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
                        ╰── <9>  [|]
                            ├── <6> Constant Int [1]
                            ╰── <8> Constant Int [2]
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
                        ╰── <21>  [|]
                            ├── <9>  [>>]
                            │   ├── <6> Constant Int [80]
                            │   ╰── <8> Constant Int [2]
                            ╰── <20>  [^]
                                ├── <11> Constant Int [1]
                                ╰── <19>  [&]
                                    ├── <13> Constant Int [5]
                                    ╰── <18>  [<<]
                                        ├── <15> Constant Int [7]
                                        ╰── <17> Constant Int [1]
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
                        ╰── <12>  [>>]
                            ├── <9>  [<<]
                            │   ├── <6> Constant Int [33]
                            │   ╰── <8> Constant Int [4]
                            ╰── <11> Constant Int [2]
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
                        ╰── <12>  [<<]
                            ├── <9>  [>>]
                            │   ├── <6> Constant Int [33]
                            │   ╰── <8> Constant Int [2]
                            ╰── <11> Constant Int [1]
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
                        ╰── <15>  [>>]
                            ├── <12>  [<<]
                            │   ├── <6> Constant Int [40]
                            │   ╰── <11>  [+]
                            │       ├── <8> Constant Int [4]
                            │       ╰── <10> Constant Int [12]
                            ╰── <14> Constant Int [1]
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
                        ╰── <9>  [<<]
                            ├── <6> Constant Int [35]
                            ╰── <8> Constant Int [2]
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
                        ╰── <9>  [>>]
                            ├── <6> Constant Int [1000]
                            ╰── <8> Constant Int [4]
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
                        ╰── <11>  [>>]
                            ├── <8> Unary [-]
                            │   ╰── <7> Constant Int [5]
                            ╰── <10> Constant Int [30]
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
                        ╰── <25>  [+]
                            ├── <14>  [<<]
                            │   ├── <6> Constant Int [4]
                            │   ╰── <12>  [*]
                            │       ├── <8> Constant Int [2]
                            │       ╰── <10> Constant Int [2]
                            ╰── <24>  [>>]
                                ├── <16> Constant Int [100]
                                ╰── <22>  [+]
                                    ├── <18> Constant Int [1]
                                    ╰── <20> Constant Int [2]
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
                        ╰── <9>  [^]
                            ├── <6> Constant Int [7]
                            ╰── <8> Constant Int [1]
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
                        ╰── <9>  [%]
                            ├── <6> Constant Int [4]
                            ╰── <8> Constant Int [2]
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
                        ╰── <9>  [*]
                            ├── <6> Constant Int [2]
                            ╰── <8> Constant Int [3]
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
                        ╰── <13>  [*]
                            ├── <6> Constant Int [2]
                            ╰── <12>  [+]
                                ├── <8> Constant Int [3]
                                ╰── <10> Constant Int [4]
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
                        ╰── <12>  [+]
                            ├── <6> Constant Int [2]
                            ╰── <11>  [*]
                                ├── <8> Constant Int [3]
                                ╰── <10> Constant Int [4]
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
                        ╰── <9>  [-]
                            ├── <6> Constant Int [1]
                            ╰── <8> Constant Int [2]
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
                        ╰── <11>  [-]
                            ├── <6> Constant Int [2]
                            ╰── <10> Unary [-]
                                ╰── <9> Constant Int [1]
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
                        ╰── <11>  [+]
                            ├── <8> Unary [~]
                            │   ╰── <7> Constant Int [2]
                            ╰── <10> Constant Int [3]
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
                        ╰── <12> Unary [~]
                            ╰── <11>  [+]
                                ├── <7> Constant Int [1]
                                ╰── <9> Constant Int [1]
    "#;
    assert_parse(src, expected);
}
