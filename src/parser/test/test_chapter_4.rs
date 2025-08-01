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
                        ╰── <23>  [+]
                            ├── <16>  [+]
                            │   ├── <9>  [&&]
                            │   │   ├── <5> Constant Int [10]
                            │   │   ╰── <7> Constant Int [0]
                            │   ╰── <15>  [&&]
                            │       ├── <11> Constant Int [0]
                            │       ╰── <13> Constant Int [4]
                            ╰── <22>  [&&]
                                ├── <18> Constant Int [0]
                                ╰── <20> Constant Int [0]
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
                        ╰── <12>  [&&]
                            ├── <5> Constant Int [0]
                            ╰── <11>  [/]
                                ├── <7> Constant Int [1]
                                ╰── <9> Constant Int [0]
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
                        ╰── <10>  [&&]
                            ├── <5> Constant Int [1]
                            ╰── <9> Unary [-]
                                ╰── <8> Constant Int [1]
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
                        ╰── <14>  [<=]
                            ├── <11>  [>]
                            │   ├── <8>  [>=]
                            │   │   ├── <5> Constant Int [5]
                            │   │   ╰── <7> Constant Int [0]
                            │   ╰── <10> Constant Int [1]
                            ╰── <13> Constant Int [0]
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
                        ╰── <18>  [==]
                            ├── <12>  [*]
                            │   ├── <7> Unary [~]
                            │   │   ╰── <6> Constant Int [2]
                            │   ╰── <11> Unary [-]
                            │       ╰── <10> Constant Int [2]
                            ╰── <17>  [+]
                                ├── <14> Constant Int [1]
                                ╰── <16> Constant Int [5]
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
                        ╰── <8>  [==]
                            ├── <5> Constant Int [1]
                            ╰── <7> Constant Int [2]
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
                        ╰── <11>  [!=]
                            ├── <8>  [==]
                            │   ├── <5> Constant Int [3]
                            │   ╰── <7> Constant Int [1]
                            ╰── <10> Constant Int [2]
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
                        ╰── <8>  [==]
                            ├── <5> Constant Int [1]
                            ╰── <7> Constant Int [1]
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
                        ╰── <11>  [&]
                            ├── <5> Constant Int [5]
                            ╰── <10>  [==]
                                ├── <7> Constant Int [7]
                                ╰── <9> Constant Int [5]
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
                        ╰── <11>  [|]
                            ├── <5> Constant Int [5]
                            ╰── <10>  [!=]
                                ├── <7> Constant Int [7]
                                ╰── <9> Constant Int [5]
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
                        ╰── <14>  [<=]
                            ├── <8>  [>>]
                            │   ├── <5> Constant Int [20]
                            │   ╰── <7> Constant Int [4]
                            ╰── <13>  [<<]
                                ├── <10> Constant Int [3]
                                ╰── <12> Constant Int [1]
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
                        ╰── <11>  [^]
                            ├── <5> Constant Int [5]
                            ╰── <10>  [<]
                                ├── <7> Constant Int [7]
                                ╰── <9> Constant Int [5]
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
                        ╰── <8>  [>=]
                            ├── <5> Constant Int [1]
                            ╰── <7> Constant Int [2]
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
                        ╰── <18>  [+]
                            ├── <9>  [>=]
                            │   ├── <5> Constant Int [1]
                            │   ╰── <7> Constant Int [1]
                            ╰── <17>  [>=]
                                ├── <11> Constant Int [1]
                                ╰── <15> Unary [-]
                                    ╰── <14> Constant Int [4]
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
                        ╰── <16>  [+]
                            ├── <9>  [>]
                            │   ├── <5> Constant Int [1]
                            │   ╰── <7> Constant Int [2]
                            ╰── <15>  [>]
                                ├── <11> Constant Int [1]
                                ╰── <13> Constant Int [1]
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
                        ╰── <8>  [>]
                            ├── <5> Constant Int [15]
                            ╰── <7> Constant Int [10]
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
                        ╰── <10>  [<=]
                            ├── <5> Constant Int [1]
                            ╰── <9> Unary [-]
                                ╰── <8> Constant Int [1]
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
                        ╰── <16>  [+]
                            ├── <9>  [<=]
                            │   ├── <5> Constant Int [0]
                            │   ╰── <7> Constant Int [2]
                            ╰── <15>  [<=]
                                ├── <11> Constant Int [0]
                                ╰── <13> Constant Int [0]
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
                        ╰── <8>  [<]
                            ├── <5> Constant Int [2]
                            ╰── <7> Constant Int [1]
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
                        ╰── <8>  [<]
                            ├── <5> Constant Int [1]
                            ╰── <7> Constant Int [2]
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
                        ╰── <15>  [||]
                            ├── <5> Constant Int [0]
                            ╰── <14>  [&&]
                                ├── <7> Constant Int [0]
                                ╰── <13>  [/]
                                    ├── <9> Constant Int [1]
                                    ╰── <11> Constant Int [0]
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
                        ╰── <8>  [!=]
                            ├── <5> Constant Int [0]
                            ╰── <7> Constant Int [0]
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
                        ╰── <12>  [!=]
                            ├── <7> Unary [-]
                            │   ╰── <6> Constant Int [1]
                            ╰── <11> Unary [-]
                                ╰── <10> Constant Int [2]
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
                        ╰── <9> Unary [!]
                            ╰── <8> Unary [-]
                                ╰── <7> Constant Int [3]
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
                        ╰── <7> Unary [!]
                            ╰── <6> Constant Int [5]
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
                        ╰── <11> Unary [!]
                            ╰── <10>  [-]
                                ├── <6> Constant Int [4]
                                ╰── <8> Constant Int [4]
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
                        ╰── <11> Unary [!]
                            ╰── <10>  [-]
                                ├── <6> Constant Int [3]
                                ╰── <8> Constant Int [44]
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
                        ╰── <7> Unary [!]
                            ╰── <6> Constant Int [0]
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
                        ╰── <20>  [-]
                            ├── <11> Unary [~]
                            │   ╰── <10>  [&&]
                            │       ├── <6> Constant Int [0]
                            │       ╰── <8> Constant Int [1]
                            ╰── <19> Unary [-]
                                ╰── <18>  [||]
                                    ├── <14> Constant Int [4]
                                    ╰── <16> Constant Int [3]
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
                        ╰── <8>  [||]
                            ├── <5> Constant Int [0]
                            ╰── <7> Constant Int [0]
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
                        ╰── <12>  [||]
                            ├── <5> Constant Int [1]
                            ╰── <11>  [/]
                                ├── <7> Constant Int [1]
                                ╰── <9> Constant Int [0]
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
                        ╰── <23>  [+]
                            ├── <16>  [+]
                            │   ├── <9>  [||]
                            │   │   ├── <5> Constant Int [4]
                            │   │   ╰── <7> Constant Int [0]
                            │   ╰── <15>  [||]
                            │       ├── <11> Constant Int [0]
                            │       ╰── <13> Constant Int [3]
                            ╰── <22>  [||]
                                ├── <18> Constant Int [5]
                                ╰── <20> Constant Int [5]
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
                        ╰── <11>  [||]
                            ├── <5> Constant Int [1]
                            ╰── <10>  [&&]
                                ├── <7> Constant Int [0]
                                ╰── <9> Constant Int [2]
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
                        ╰── <12>  [&&]
                            ├── <9>  [||]
                            │   ├── <5> Constant Int [1]
                            │   ╰── <7> Constant Int [0]
                            ╰── <11> Constant Int [0]
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
                        ╰── <11>  [==]
                            ├── <5> Constant Int [2]
                            ╰── <10>  [>=]
                                ├── <7> Constant Int [2]
                                ╰── <9> Constant Int [0]
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
                        ╰── <11>  [||]
                            ├── <8>  [==]
                            │   ├── <5> Constant Int [2]
                            │   ╰── <7> Constant Int [2]
                            ╰── <10> Constant Int [0]
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
                        ╰── <24>  [+]
                            ├── <21>  [&&]
                            │   ├── <8>  [==]
                            │   │   ├── <5> Constant Int [0]
                            │   │   ╰── <7> Constant Int [0]
                            │   ╰── <19>  [==]
                            │       ├── <10> Constant Int [3]
                            │       ╰── <18>  [>]
                            │           ├── <15>  [+]
                            │           │   ├── <12> Constant Int [2]
                            │           │   ╰── <14> Constant Int [1]
                            │           ╰── <17> Constant Int [1]
                            ╰── <23> Constant Int [1]
    "#;
    assert_parse(src, expected);
}
