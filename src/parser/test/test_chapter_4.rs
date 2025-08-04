use super::{assert_error, assert_parse};

#[test]
fn test_invalid_parse_missing_const() {
    assert_error(
        r#"
        int main(void)
        {
            10 <= !;
                 //^ Expected expression, but found ';'
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_missing_first_op() {
    assert_error(
        r#"
        int main(void) {
            return <= 2;
                 //^^ Expected expression, but found '<='
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_missing_operand() {
    assert_error(
        r#"
        int main(void) {
            return 1 < > 3;
                     //^ Expected expression, but found '>'
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_missing_second_op() {
    assert_error(
        r#"
        int main(void) {
            return 2 && ~;
                       //^ Expected expression, but found ';'
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_missing_semicolon() {
    assert_error(
        r#"
        int main(void) {
            return 1 || 2
        }
      //^ Expected ';', but found '}'
    "#,
    );
}

#[test]
fn test_invalid_parse_unary_missing_semicolon() {
    assert_error(
        r#"
        int main(void)
        {
            return !10
        }
      //^ Expected ';', but found '}'
    "#,
    );
}

#[test]
fn test_valid_and_false() {
    let src = r#"
        int main(void) {
            return (10 && 0) + (0 && 4) + (0 && 0);
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <24>  [+]
                            ├── <17>  [+]
                            │   ├── <10>  [&&]
                            │   │   ├── <6> Constant Int [10]
                            │   │   ╰── <8> Constant Int [0]
                            │   ╰── <16>  [&&]
                            │       ├── <12> Constant Int [0]
                            │       ╰── <14> Constant Int [4]
                            ╰── <23>  [&&]
                                ├── <19> Constant Int [0]
                                ╰── <21> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_and_short_circuit() {
    let src = r#"
        int main(void) {
            return 0 && (1 / 0);
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <13>  [&&]
                            ├── <6> Constant Int [0]
                            ╰── <12>  [/]
                                ├── <8> Constant Int [1]
                                ╰── <10> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_and_true() {
    let src = r#"
        int main(void) {
            return 1 && -1;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <11>  [&&]
                            ├── <6> Constant Int [1]
                            ╰── <10> Unary [-]
                                ╰── <9> Constant Int [1]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_associativity() {
    let src = r#"
        int main(void) {
            return 5 >= 0 > 1 <= 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <15>  [<=]
                            ├── <12>  [>]
                            │   ├── <9>  [>=]
                            │   │   ├── <6> Constant Int [5]
                            │   │   ╰── <8> Constant Int [0]
                            │   ╰── <11> Constant Int [1]
                            ╰── <14> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_compare_arithmetic_results() {
    let src = r#"
        int main(void) {
            return ~2 * -2 == 1 + 5;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <19>  [==]
                            ├── <13>  [*]
                            │   ├── <8> Unary [~]
                            │   │   ╰── <7> Constant Int [2]
                            │   ╰── <12> Unary [-]
                            │       ╰── <11> Constant Int [2]
                            ╰── <18>  [+]
                                ├── <15> Constant Int [1]
                                ╰── <17> Constant Int [5]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_eq_false() {
    let src = r#"
        int main(void) {
            return 1 == 2;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <9>  [==]
                            ├── <6> Constant Int [1]
                            ╰── <8> Constant Int [2]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_eq_precedence() {
    let src = r#"
        int main(void) {
            return 3 == 1 != 2;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <12>  [!=]
                            ├── <9>  [==]
                            │   ├── <6> Constant Int [3]
                            │   ╰── <8> Constant Int [1]
                            ╰── <11> Constant Int [2]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_eq_true() {
    let src = r#"
        int main(void) {
            return 1 == 1;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <9>  [==]
                            ├── <6> Constant Int [1]
                            ╰── <8> Constant Int [1]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_bitwise_and_precedence() {
    let src = r#"
        int main(void) {
            return 5 & 7 == 5;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <12>  [&]
                            ├── <6> Constant Int [5]
                            ╰── <11>  [==]
                                ├── <8> Constant Int [7]
                                ╰── <10> Constant Int [5]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_bitwise_or_precedence() {
    let src = r#"
        int main(void) {
            return 5 | 7 != 5;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <12>  [|]
                            ├── <6> Constant Int [5]
                            ╰── <11>  [!=]
                                ├── <8> Constant Int [7]
                                ╰── <10> Constant Int [5]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_bitwise_shift_precedence() {
    let src = r#"
        int main(void) {
            return 20 >> 4 <= 3 << 1;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <15>  [<=]
                            ├── <9>  [>>]
                            │   ├── <6> Constant Int [20]
                            │   ╰── <8> Constant Int [4]
                            ╰── <14>  [<<]
                                ├── <11> Constant Int [3]
                                ╰── <13> Constant Int [1]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_bitwise_xor_precedence() {
    let src = r#"
        int main(void) {
            return 5 ^ 7 < 5;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <12>  [^]
                            ├── <6> Constant Int [5]
                            ╰── <11>  [<]
                                ├── <8> Constant Int [7]
                                ╰── <10> Constant Int [5]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_ge_false() {
    let src = r#"
        int main(void) {
            return 1 >= 2;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <9>  [>=]
                            ├── <6> Constant Int [1]
                            ╰── <8> Constant Int [2]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_ge_true() {
    let src = r#"
        int main(void) {
            return (1 >= 1) + (1 >= -4);
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <19>  [+]
                            ├── <10>  [>=]
                            │   ├── <6> Constant Int [1]
                            │   ╰── <8> Constant Int [1]
                            ╰── <18>  [>=]
                                ├── <12> Constant Int [1]
                                ╰── <16> Unary [-]
                                    ╰── <15> Constant Int [4]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_gt_false() {
    let src = r#"
        int main(void) {
            return (1 > 2) + (1 > 1);
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <17>  [+]
                            ├── <10>  [>]
                            │   ├── <6> Constant Int [1]
                            │   ╰── <8> Constant Int [2]
                            ╰── <16>  [>]
                                ├── <12> Constant Int [1]
                                ╰── <14> Constant Int [1]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_gt_true() {
    let src = r#"
        int main(void) {
            return 15 > 10;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <9>  [>]
                            ├── <6> Constant Int [15]
                            ╰── <8> Constant Int [10]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_le_false() {
    let src = r#"
        int main(void) {
            return 1 <= -1;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <11>  [<=]
                            ├── <6> Constant Int [1]
                            ╰── <10> Unary [-]
                                ╰── <9> Constant Int [1]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_le_true() {
    let src = r#"
        int main(void) {
            return (0 <= 2) + (0 <= 0);
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <17>  [+]
                            ├── <10>  [<=]
                            │   ├── <6> Constant Int [0]
                            │   ╰── <8> Constant Int [2]
                            ╰── <16>  [<=]
                                ├── <12> Constant Int [0]
                                ╰── <14> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_lt_false() {
    let src = r#"
        int main(void) {
            return 2 < 1;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <9>  [<]
                            ├── <6> Constant Int [2]
                            ╰── <8> Constant Int [1]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_lt_true() {
    let src = r#"
        int main(void) {
            return 1 < 2;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <9>  [<]
                            ├── <6> Constant Int [1]
                            ╰── <8> Constant Int [2]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_multi_short_circuit() {
    let src = r#"
        int main(void) {
            return 0 || 0 && (1 / 0);
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <16>  [||]
                            ├── <6> Constant Int [0]
                            ╰── <15>  [&&]
                                ├── <8> Constant Int [0]
                                ╰── <14>  [/]
                                    ├── <10> Constant Int [1]
                                    ╰── <12> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_ne_false() {
    let src = r#"
        int main(void) {
            return 0 != 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <9>  [!=]
                            ├── <6> Constant Int [0]
                            ╰── <8> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_ne_true() {
    let src = r#"
        int main(void) {
            return -1 != -2;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <13>  [!=]
                            ├── <8> Unary [-]
                            │   ╰── <7> Constant Int [1]
                            ╰── <12> Unary [-]
                                ╰── <11> Constant Int [2]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_nested_ops() {
    let src = r#"
        int main(void) {
            return !-3;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <10> Unary [!]
                            ╰── <9> Unary [-]
                                ╰── <8> Constant Int [3]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_not() {
    let src = r#"
        int main(void) {
            return !5;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <8> Unary [!]
                            ╰── <7> Constant Int [5]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_not_sum() {
    let src = r#"
        int main(void) {
            return !(4-4);
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <12> Unary [!]
                            ╰── <11>  [-]
                                ├── <7> Constant Int [4]
                                ╰── <9> Constant Int [4]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_not_sum_2() {
    let src = r#"
        int main(void) {
            return !(3 - 44);
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <12> Unary [!]
                            ╰── <11>  [-]
                                ├── <7> Constant Int [3]
                                ╰── <9> Constant Int [44]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_not_zero() {
    let src = r#"
        int main(void) {
            return !0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <8> Unary [!]
                            ╰── <7> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_operate_on_booleans() {
    let src = r#"
        int main(void) {
            return ~(0 && 1) - -(4 || 3);
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <21>  [-]
                            ├── <12> Unary [~]
                            │   ╰── <11>  [&&]
                            │       ├── <7> Constant Int [0]
                            │       ╰── <9> Constant Int [1]
                            ╰── <20> Unary [-]
                                ╰── <19>  [||]
                                    ├── <15> Constant Int [4]
                                    ╰── <17> Constant Int [3]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_or_false() {
    let src = r#"
        int main(void) {
            return 0 || 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <9>  [||]
                            ├── <6> Constant Int [0]
                            ╰── <8> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_or_short_circuit() {
    let src = r#"
        int main(void) {
            return 1 || (1 / 0);
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <13>  [||]
                            ├── <6> Constant Int [1]
                            ╰── <12>  [/]
                                ├── <8> Constant Int [1]
                                ╰── <10> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_or_true() {
    let src = r#"
        int main(void) {
            return (4 || 0) + (0 || 3) + (5 || 5);
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <24>  [+]
                            ├── <17>  [+]
                            │   ├── <10>  [||]
                            │   │   ├── <6> Constant Int [4]
                            │   │   ╰── <8> Constant Int [0]
                            │   ╰── <16>  [||]
                            │       ├── <12> Constant Int [0]
                            │       ╰── <14> Constant Int [3]
                            ╰── <23>  [||]
                                ├── <19> Constant Int [5]
                                ╰── <21> Constant Int [5]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_precedence() {
    let src = r#"
        int main(void) {
            return 1 || 0 && 2;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <12>  [||]
                            ├── <6> Constant Int [1]
                            ╰── <11>  [&&]
                                ├── <8> Constant Int [0]
                                ╰── <10> Constant Int [2]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_precedence_2() {
    let src = r#"
        int main(void) {
            return (1 || 0) && 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <13>  [&&]
                            ├── <10>  [||]
                            │   ├── <6> Constant Int [1]
                            │   ╰── <8> Constant Int [0]
                            ╰── <12> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_precedence_3() {
    let src = r#"
        int main(void) {
            return 2 == 2 >= 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <12>  [==]
                            ├── <6> Constant Int [2]
                            ╰── <11>  [>=]
                                ├── <8> Constant Int [2]
                                ╰── <10> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_precedence_4() {
    let src = r#"
        int main(void) {
            return 2 == 2 || 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <12>  [||]
                            ├── <9>  [==]
                            │   ├── <6> Constant Int [2]
                            │   ╰── <8> Constant Int [2]
                            ╰── <11> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_precedence_5() {
    let src = r#"
        int main(void) {
            return (0 == 0 && 3 == 2 + 1 > 1) + 1;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <25>  [+]
                            ├── <22>  [&&]
                            │   ├── <9>  [==]
                            │   │   ├── <6> Constant Int [0]
                            │   │   ╰── <8> Constant Int [0]
                            │   ╰── <20>  [==]
                            │       ├── <11> Constant Int [3]
                            │       ╰── <19>  [>]
                            │           ├── <16>  [+]
                            │           │   ├── <13> Constant Int [2]
                            │           │   ╰── <15> Constant Int [1]
                            │           ╰── <18> Constant Int [1]
                            ╰── <24> Constant Int [1]
    "#;
    assert_parse(src, expected);
}
