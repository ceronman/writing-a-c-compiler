use super::{assert_error, assert_parse};

#[test]
fn test_invalid_parse_extra_paren() {
    assert_error(
        r#"
        int main(void)
        {
            return (3));
                    //^ Expected ';', but found ')'
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_missing_const() {
    assert_error(
        r#"
        int main(void) {
            return ~;
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
            return -5
        }
      //^ Expected ';', but found '}'
    "#,
    );
}

#[test]
fn test_invalid_parse_nested_missing_const() {
    assert_error(
        r#"
        int main(void)
        {
            return -~;
                   //^ Expected expression, but found ';'
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_parenthesize_operand() {
    assert_error(
        r#"
        int main(void) {
            return (-)3;
                   //^ Expected expression, but found ')'
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_unclosed_paren() {
    assert_error(
        r#"
        int main(void)
        {
            return (1;
                   //^ Expected ')', but found ';'
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_wrong_order() {
    assert_error(
        r#"
        int main(void) {
            return 4-;
                   //^ Expected expression, but found ';'
        }
    "#,
    );
}

#[test]
fn test_valid_bitwise() {
    let src = r#"
        int main(void) {
            return ~12;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <7> Unary [~]
                            ╰── <6> Constant Int [12]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_bitwise_int_min() {
    let src = r#"
        int main(void) {
            return ~-2147483647;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <9> Unary [~]
                            ╰── <8> Unary [-]
                                ╰── <7> Constant Int [2147483647]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_bitwise_zero() {
    let src = r#"
        int main(void) {
            return ~0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <7> Unary [~]
                            ╰── <6> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_neg() {
    let src = r#"
        int main(void) {
            return -5;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <7> Unary [-]
                            ╰── <6> Constant Int [5]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_neg_zero() {
    let src = r#"
        int main(void) {
            return -0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <7> Unary [-]
                            ╰── <6> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_negate_int_max() {
    let src = r#"
        int main(void) {
            return -2147483647;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <7> Unary [-]
                            ╰── <6> Constant Int [2147483647]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_nested_ops() {
    let src = r#"
        int main(void) {
            return ~-3;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <9> Unary [~]
                            ╰── <8> Unary [-]
                                ╰── <7> Constant Int [3]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_nested_ops_2() {
    let src = r#"
        int main(void) {
            return -~0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <9> Unary [-]
                            ╰── <8> Unary [~]
                                ╰── <7> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_parens() {
    let src = r#"
        int main(void) {
            return (-2);
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <8> Unary [-]
                            ╰── <6> Constant Int [2]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_parens_2() {
    let src = r#"
        int main(void) {
            return ~(2);
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <8> Unary [~]
                            ╰── <7> Constant Int [2]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_parens_3() {
    let src = r#"
        int main(void) {
            return -(-4);
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <10> Unary [-]
                            ╰── <9> Unary [-]
                                ╰── <7> Constant Int [4]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_redundant_parens() {
    let src = r#"
        int main(void)
        {
            return -((((10))));
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <11> Unary [-]
                            ╰── <10> Constant Int [10]
    "#;
    assert_parse(src, expected);
}
