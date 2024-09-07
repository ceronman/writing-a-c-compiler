use crate::parser::parse;
use crate::pretty::{annotate, dedent, dump_ast, remove_annotation};

fn assert_error(expected_annotated: &str) {
    let clean_source = remove_annotation(expected_annotated);
    let Err(error) = parse(&clean_source) else {
        panic!("No error produced!")
    };
    let actual_annotated = annotate(&clean_source, &error);
    assert_eq!(actual_annotated, expected_annotated);
}

#[test]
fn test_chapter_1_invalid_parse_end_before_expr() {
    assert_error(
        r#"
        int main(void) {
            return
    
// Expected expression, but found ''"#,
    );
}

#[test]
fn test_chapter_1_invalid_parse_extra_junk() {
    assert_error(
        r#"
        int main(void)
        {
            return 2;
        }
        foo
      //^^^ Expected Eof, but found 'foo'
    "#,
    );
}

#[test]
fn test_chapter_1_invalid_parse_invalid_function_name() {
    assert_error(
        r#"
        
        int 3 (void) {
          //^ Expected Identifier, but found '3'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_1_invalid_parse_keyword_wrong_case() {
    assert_error(
        r#"
        int main(void) {
            RETURN 0;
                 //^ Expected Semicolon, but found '0'
        }
    "#,
    );
}

#[test]
fn test_chapter_1_invalid_parse_missing_type() {
    assert_error(
        r#"
        main(void) {
      //^^^^ Expected Int, but found 'main'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_1_invalid_parse_misspelled_keyword() {
    assert_error(
        r#"
        int main(void) {
            returns 0;
                  //^ Expected Semicolon, but found '0'
        }
    "#,
    );
}

#[test]
fn test_chapter_1_invalid_parse_no_semicolon() {
    assert_error(
        r#"
        int main (void) {
            return 0
        }
      //^ Expected Semicolon, but found '}'
    "#,
    );
}

#[test]
fn test_chapter_1_invalid_parse_not_expression() {
    assert_error(
        r#"
        int main(void) {
            return int;
                 //^^^ Expected expression, but found 'int'
        }
    "#,
    );
}

#[test]
fn test_chapter_1_invalid_parse_space_in_keyword() {
    assert_error(
        r#"
        int main(void){
            retur n 0;
                //^ Expected Semicolon, but found 'n'
        }
    "#,
    );
}

#[test]
fn test_chapter_1_invalid_parse_switched_parens() {
    assert_error(
        r#"
        int main )( {
               //^ Expected OpenParen, but found ')'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_1_invalid_parse_unclosed_brace() {
    assert_error(
        r#"
        int main(void) {
            return 0;
    
// Expected expression, but found ''"#,
    );
}

#[test]
fn test_chapter_1_invalid_parse_unclosed_paren() {
    assert_error(
        r#"
        int main( {
                //^ Expected Void, but found '{'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_1_valid_multi_digit() {
    let src = r#"
        int main(void) {
            return 100;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Constant [100]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_1_valid_newlines() {
    let src = r#"
        int
        main
        (
        void
        )
        {
        return
        0
        ;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_1_valid_no_newlines() {
    let src = r#"
        int main(void){return 0;}
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_1_valid_return_0() {
    let src = r#"
        int main(void) {
            return 0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_1_valid_return_2() {
    let src = r#"
        int main(void) {
            return 2;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Constant [2]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_1_valid_spaces() {
    let src = r#"
           int main ( void) { return 0 ; }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_1_valid_tabs() {
    let src = r#"
        int main ( void) { return 0 ; }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_2_invalid_parse_extra_paren() {
    assert_error(
        r#"
        int main(void)
        {
            return (3));
                    //^ Expected Semicolon, but found ')'
        }
    "#,
    );
}

#[test]
fn test_chapter_2_invalid_parse_missing_const() {
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
fn test_chapter_2_invalid_parse_missing_semicolon() {
    assert_error(
        r#"
        int main(void) {
            return -5
        }
      //^ Expected Semicolon, but found '}'
    "#,
    );
}

#[test]
fn test_chapter_2_invalid_parse_nested_missing_const() {
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
fn test_chapter_2_invalid_parse_parenthesize_operand() {
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
fn test_chapter_2_invalid_parse_unclosed_paren() {
    assert_error(
        r#"
        int main(void)
        {
            return (1;
                   //^ Expected CloseParen, but found ';'
        }
    "#,
    );
}

#[test]
fn test_chapter_2_invalid_parse_wrong_order() {
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
fn test_chapter_2_valid_bitwise() {
    let src = r#"
        int main(void) {
            return ~12;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Unary [~]
                    ╰── Constant [12]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_2_valid_bitwise_int_min() {
    let src = r#"
        int main(void) {
            return ~-2147483647;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Unary [~]
                    ╰── Unary [-]
                        ╰── Constant [2147483647]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_2_valid_bitwise_zero() {
    let src = r#"
        int main(void) {
            return ~0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Unary [~]
                    ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_2_valid_neg() {
    let src = r#"
        int main(void) {
            return -5;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Unary [-]
                    ╰── Constant [5]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_2_valid_neg_zero() {
    let src = r#"
        int main(void) {
            return -0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Unary [-]
                    ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_2_valid_negate_int_max() {
    let src = r#"
        int main(void) {
            return -2147483647;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Unary [-]
                    ╰── Constant [2147483647]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_2_valid_nested_ops() {
    let src = r#"
        int main(void) {
            return ~-3;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Unary [~]
                    ╰── Unary [-]
                        ╰── Constant [3]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_2_valid_nested_ops_2() {
    let src = r#"
        int main(void) {
            return -~0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Unary [-]
                    ╰── Unary [~]
                        ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_2_valid_parens() {
    let src = r#"
        int main(void) {
            return (-2);
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Unary [-]
                    ╰── Constant [2]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_2_valid_parens_2() {
    let src = r#"
        int main(void) {
            return ~(2);
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Unary [~]
                    ╰── Constant [2]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_2_valid_parens_3() {
    let src = r#"
        int main(void) {
            return -(-4);
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Unary [-]
                    ╰── Unary [-]
                        ╰── Constant [4]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_2_valid_redundant_parens() {
    let src = r#"
        int main(void)
        {
            return -((((10))));
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Unary [-]
                    ╰── Constant [10]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_3_invalid_parse_double_operation() {
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
fn test_chapter_3_invalid_parse_extra_credit_bitwise_double_operator() {
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
fn test_chapter_3_invalid_parse_imbalanced_paren() {
    assert_error(
        r#"
        int main(void) {
            return 1 + (2;
                       //^ Expected CloseParen, but found ';'
        }
    "#,
    );
}

#[test]
fn test_chapter_3_invalid_parse_malformed_paren() {
    assert_error(
        r#"
        int main(void) {
            return 2 (- 3);
                   //^ Expected Semicolon, but found '('
        }
    "#,
    );
}

#[test]
fn test_chapter_3_invalid_parse_misplaced_semicolon() {
    assert_error(
        r#"
        int main(void) {
            return 1 + (2;)
                       //^ Expected CloseParen, but found ';'
        }
    "#,
    );
}

#[test]
fn test_chapter_3_invalid_parse_missing_first_op() {
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
fn test_chapter_3_invalid_parse_missing_open_paren() {
    assert_error(
        r#"
        int main(void) {
            return 1 + 2);
                      //^ Expected Semicolon, but found ')'
        }
    "#,
    );
}

#[test]
fn test_chapter_3_invalid_parse_missing_second_op() {
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
fn test_chapter_3_invalid_parse_no_semicolon() {
    assert_error(
        r#"
        int main(void) {
            return 2*2
        }
      //^ Expected Semicolon, but found '}'
    "#,
    );
}

#[test]
fn test_chapter_3_valid_add() {
    let src = r#"
        int main(void) {
            return 1 + 2;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [+]
                    ├── Constant [1]
                    ╰── Constant [2]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_associativity() {
    let src = r#"
        int main(void) {
            return 1 - 2 - 3;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [-]
                    ├── Binary [-]
                    │   ├── Constant [1]
                    │   ╰── Constant [2]
                    ╰── Constant [3]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_associativity_2() {
    let src = r#"
        int main(void) {
            return 6 / 3 / 2;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [/]
                    ├── Binary [/]
                    │   ├── Constant [6]
                    │   ╰── Constant [3]
                    ╰── Constant [2]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_associativity_3() {
    let src = r#"
        int main(void) {
            return (3 / 2 * 4) + (5 - 4 + 3);
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [+]
                    ├── Binary [*]
                    │   ├── Binary [/]
                    │   │   ├── Constant [3]
                    │   │   ╰── Constant [2]
                    │   ╰── Constant [4]
                    ╰── Binary [+]
                        ├── Binary [-]
                        │   ├── Constant [5]
                        │   ╰── Constant [4]
                        ╰── Constant [3]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_associativity_and_precedence() {
    let src = r#"
        int main(void) {
            return 5 * 4 / 2 -
                3 % (2 + 1);
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [-]
                    ├── Binary [/]
                    │   ├── Binary [*]
                    │   │   ├── Constant [5]
                    │   │   ╰── Constant [4]
                    │   ╰── Constant [2]
                    ╰── Binary [%]
                        ├── Constant [3]
                        ╰── Binary [+]
                            ├── Constant [2]
                            ╰── Constant [1]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_div() {
    let src = r#"
        int main(void) {
            return 4 / 2;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [/]
                    ├── Constant [4]
                    ╰── Constant [2]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_div_neg() {
    let src = r#"
        int main(void) {
            return (-12) / 5;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [/]
                    ├── Unary [-]
                    │   ╰── Constant [12]
                    ╰── Constant [5]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_extra_credit_bitwise_and() {
    let src = r#"
        int main(void) {
            return 3 & 5;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [&]
                    ├── Constant [3]
                    ╰── Constant [5]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_extra_credit_bitwise_or() {
    let src = r#"
        int main(void) {
            return 1 | 2;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [|]
                    ├── Constant [1]
                    ╰── Constant [2]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_extra_credit_bitwise_precedence() {
    let src = r#"
        int main(void) {
            return 80 >> 2 | 1 ^ 5 & 7 << 1;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [|]
                    ├── Binary [>>]
                    │   ├── Constant [80]
                    │   ╰── Constant [2]
                    ╰── Binary [^]
                        ├── Constant [1]
                        ╰── Binary [&]
                            ├── Constant [5]
                            ╰── Binary [<<]
                                ├── Constant [7]
                                ╰── Constant [1]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_extra_credit_bitwise_shift_associativity() {
    let src = r#"
        int main(void) {
            return 33 << 4 >> 2;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [>>]
                    ├── Binary [<<]
                    │   ├── Constant [33]
                    │   ╰── Constant [4]
                    ╰── Constant [2]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_extra_credit_bitwise_shift_associativity_2() {
    let src = r#"
        int main(void) {
            return 33 >> 2 << 1;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [<<]
                    ├── Binary [>>]
                    │   ├── Constant [33]
                    │   ╰── Constant [2]
                    ╰── Constant [1]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_extra_credit_bitwise_shift_precedence() {
    let src = r#"
        int main(void) {
            return 40 << 4 + 12 >> 1;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [>>]
                    ├── Binary [<<]
                    │   ├── Constant [40]
                    │   ╰── Binary [+]
                    │       ├── Constant [4]
                    │       ╰── Constant [12]
                    ╰── Constant [1]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_extra_credit_bitwise_shiftl() {
    let src = r#"
        int main(void) {
            return 35 << 2;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [<<]
                    ├── Constant [35]
                    ╰── Constant [2]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_extra_credit_bitwise_shiftr() {
    let src = r#"
        int main(void) {
            return 1000 >> 4;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [>>]
                    ├── Constant [1000]
                    ╰── Constant [4]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_extra_credit_bitwise_shiftr_negative() {
    let src = r#"
        int main(void) {
            return -5 >> 30;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [>>]
                    ├── Unary [-]
                    │   ╰── Constant [5]
                    ╰── Constant [30]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_extra_credit_bitwise_variable_shift_count() {
    let src = r#"
        int main(void) {
            return (4 << (2 * 2)) + (100 >> (1 + 2));
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [+]
                    ├── Binary [<<]
                    │   ├── Constant [4]
                    │   ╰── Binary [*]
                    │       ├── Constant [2]
                    │       ╰── Constant [2]
                    ╰── Binary [>>]
                        ├── Constant [100]
                        ╰── Binary [+]
                            ├── Constant [1]
                            ╰── Constant [2]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_extra_credit_bitwise_xor() {
    let src = r#"
        int main(void) {
            return 7 ^ 1;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [^]
                    ├── Constant [7]
                    ╰── Constant [1]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_mod() {
    let src = r#"
        int main(void) {
            return 4 % 2;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [%]
                    ├── Constant [4]
                    ╰── Constant [2]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_mult() {
    let src = r#"
        int main(void) {
            return 2 * 3;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [*]
                    ├── Constant [2]
                    ╰── Constant [3]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_parens() {
    let src = r#"
        int main(void) {
            return 2 * (3 + 4);
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [*]
                    ├── Constant [2]
                    ╰── Binary [+]
                        ├── Constant [3]
                        ╰── Constant [4]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_precedence() {
    let src = r#"
        int main(void) {
            return 2 + 3 * 4;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [+]
                    ├── Constant [2]
                    ╰── Binary [*]
                        ├── Constant [3]
                        ╰── Constant [4]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_sub() {
    let src = r#"
        int main(void) {
            return 1 - 2;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [-]
                    ├── Constant [1]
                    ╰── Constant [2]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_sub_neg() {
    let src = r#"
        int main(void) {
            return 2- -1;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [-]
                    ├── Constant [2]
                    ╰── Unary [-]
                        ╰── Constant [1]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_unop_add() {
    let src = r#"
        int main(void) {
            return ~2 + 3;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [+]
                    ├── Unary [~]
                    │   ╰── Constant [2]
                    ╰── Constant [3]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_unop_parens() {
    let src = r#"
        int main(void) {
            return ~(1 + 1);
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Unary [~]
                    ╰── Binary [+]
                        ├── Constant [1]
                        ╰── Constant [1]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_4_invalid_parse_missing_const() {
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
fn test_chapter_4_invalid_parse_missing_first_op() {
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
fn test_chapter_4_invalid_parse_missing_operand() {
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
fn test_chapter_4_invalid_parse_missing_second_op() {
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
fn test_chapter_4_invalid_parse_missing_semicolon() {
    assert_error(
        r#"
        int main(void) {
            return 1 || 2
        }
      //^ Expected Semicolon, but found '}'
    "#,
    );
}

#[test]
fn test_chapter_4_invalid_parse_unary_missing_semicolon() {
    assert_error(
        r#"
        int main(void)
        {
            return !10
        }
      //^ Expected Semicolon, but found '}'
    "#,
    );
}

#[test]
fn test_chapter_4_valid_and_false() {
    let src = r#"
        int main(void) {
            return (10 && 0) + (0 && 4) + (0 && 0);
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [+]
                    ├── Binary [+]
                    │   ├── Binary [&&]
                    │   │   ├── Constant [10]
                    │   │   ╰── Constant [0]
                    │   ╰── Binary [&&]
                    │       ├── Constant [0]
                    │       ╰── Constant [4]
                    ╰── Binary [&&]
                        ├── Constant [0]
                        ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_and_short_circuit() {
    let src = r#"
        int main(void) {
            return 0 && (1 / 0);
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [&&]
                    ├── Constant [0]
                    ╰── Binary [/]
                        ├── Constant [1]
                        ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_and_true() {
    let src = r#"
        int main(void) {
            return 1 && -1;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [&&]
                    ├── Constant [1]
                    ╰── Unary [-]
                        ╰── Constant [1]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_associativity() {
    let src = r#"
        int main(void) {
            return 5 >= 0 > 1 <= 0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [<=]
                    ├── Binary [>]
                    │   ├── Binary [>=]
                    │   │   ├── Constant [5]
                    │   │   ╰── Constant [0]
                    │   ╰── Constant [1]
                    ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_compare_arithmetic_results() {
    let src = r#"
        int main(void) {
            return ~2 * -2 == 1 + 5;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [==]
                    ├── Binary [*]
                    │   ├── Unary [~]
                    │   │   ╰── Constant [2]
                    │   ╰── Unary [-]
                    │       ╰── Constant [2]
                    ╰── Binary [+]
                        ├── Constant [1]
                        ╰── Constant [5]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_eq_false() {
    let src = r#"
        int main(void) {
            return 1 == 2;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [==]
                    ├── Constant [1]
                    ╰── Constant [2]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_eq_precedence() {
    let src = r#"
        int main(void) {
            return 3 == 1 != 2;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [!=]
                    ├── Binary [==]
                    │   ├── Constant [3]
                    │   ╰── Constant [1]
                    ╰── Constant [2]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_eq_true() {
    let src = r#"
        int main(void) {
            return 1 == 1;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [==]
                    ├── Constant [1]
                    ╰── Constant [1]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_extra_credit_bitwise_and_precedence() {
    let src = r#"
        int main(void) {
            return 5 & 7 == 5;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [&]
                    ├── Constant [5]
                    ╰── Binary [==]
                        ├── Constant [7]
                        ╰── Constant [5]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_extra_credit_bitwise_or_precedence() {
    let src = r#"
        int main(void) {
            return 5 | 7 != 5;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [|]
                    ├── Constant [5]
                    ╰── Binary [!=]
                        ├── Constant [7]
                        ╰── Constant [5]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_extra_credit_bitwise_shift_precedence() {
    let src = r#"
        int main(void) {
            return 20 >> 4 <= 3 << 1;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [<=]
                    ├── Binary [>>]
                    │   ├── Constant [20]
                    │   ╰── Constant [4]
                    ╰── Binary [<<]
                        ├── Constant [3]
                        ╰── Constant [1]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_extra_credit_bitwise_xor_precedence() {
    let src = r#"
        int main(void) {
            return 5 ^ 7 < 5;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [^]
                    ├── Constant [5]
                    ╰── Binary [<]
                        ├── Constant [7]
                        ╰── Constant [5]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_ge_false() {
    let src = r#"
        int main(void) {
            return 1 >= 2;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [>=]
                    ├── Constant [1]
                    ╰── Constant [2]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_ge_true() {
    let src = r#"
        int main(void) {
            return (1 >= 1) + (1 >= -4);
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [+]
                    ├── Binary [>=]
                    │   ├── Constant [1]
                    │   ╰── Constant [1]
                    ╰── Binary [>=]
                        ├── Constant [1]
                        ╰── Unary [-]
                            ╰── Constant [4]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_gt_false() {
    let src = r#"
        int main(void) {
            return (1 > 2) + (1 > 1);
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [+]
                    ├── Binary [>]
                    │   ├── Constant [1]
                    │   ╰── Constant [2]
                    ╰── Binary [>]
                        ├── Constant [1]
                        ╰── Constant [1]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_gt_true() {
    let src = r#"
        int main(void) {
            return 15 > 10;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [>]
                    ├── Constant [15]
                    ╰── Constant [10]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_le_false() {
    let src = r#"
        int main(void) {
            return 1 <= -1;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [<=]
                    ├── Constant [1]
                    ╰── Unary [-]
                        ╰── Constant [1]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_le_true() {
    let src = r#"
        int main(void) {
            return (0 <= 2) + (0 <= 0);
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [+]
                    ├── Binary [<=]
                    │   ├── Constant [0]
                    │   ╰── Constant [2]
                    ╰── Binary [<=]
                        ├── Constant [0]
                        ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_lt_false() {
    let src = r#"
        int main(void) {
            return 2 < 1;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [<]
                    ├── Constant [2]
                    ╰── Constant [1]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_lt_true() {
    let src = r#"
        int main(void) {
            return 1 < 2;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [<]
                    ├── Constant [1]
                    ╰── Constant [2]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_multi_short_circuit() {
    let src = r#"
        int main(void) {
            return 0 || 0 && (1 / 0);
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [||]
                    ├── Constant [0]
                    ╰── Binary [&&]
                        ├── Constant [0]
                        ╰── Binary [/]
                            ├── Constant [1]
                            ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_ne_false() {
    let src = r#"
        int main(void) {
            return 0 != 0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [!=]
                    ├── Constant [0]
                    ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_ne_true() {
    let src = r#"
        int main(void) {
            return -1 != -2;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [!=]
                    ├── Unary [-]
                    │   ╰── Constant [1]
                    ╰── Unary [-]
                        ╰── Constant [2]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_nested_ops() {
    let src = r#"
        int main(void) {
            return !-3;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Unary [!]
                    ╰── Unary [-]
                        ╰── Constant [3]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_not() {
    let src = r#"
        int main(void) {
            return !5;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Unary [!]
                    ╰── Constant [5]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_not_sum() {
    let src = r#"
        int main(void) {
            return !(4-4);
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Unary [!]
                    ╰── Binary [-]
                        ├── Constant [4]
                        ╰── Constant [4]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_not_sum_2() {
    let src = r#"
        int main(void) {
            return !(3 - 44);
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Unary [!]
                    ╰── Binary [-]
                        ├── Constant [3]
                        ╰── Constant [44]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_not_zero() {
    let src = r#"
        int main(void) {
            return !0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Unary [!]
                    ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_operate_on_booleans() {
    let src = r#"
        int main(void) {
            return ~(0 && 1) - -(4 || 3);
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [-]
                    ├── Unary [~]
                    │   ╰── Binary [&&]
                    │       ├── Constant [0]
                    │       ╰── Constant [1]
                    ╰── Unary [-]
                        ╰── Binary [||]
                            ├── Constant [4]
                            ╰── Constant [3]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_or_false() {
    let src = r#"
        int main(void) {
            return 0 || 0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [||]
                    ├── Constant [0]
                    ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_or_short_circuit() {
    let src = r#"
        int main(void) {
            return 1 || (1 / 0);
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [||]
                    ├── Constant [1]
                    ╰── Binary [/]
                        ├── Constant [1]
                        ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_or_true() {
    let src = r#"
        int main(void) {
            return (4 || 0) + (0 || 3) + (5 || 5);
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [+]
                    ├── Binary [+]
                    │   ├── Binary [||]
                    │   │   ├── Constant [4]
                    │   │   ╰── Constant [0]
                    │   ╰── Binary [||]
                    │       ├── Constant [0]
                    │       ╰── Constant [3]
                    ╰── Binary [||]
                        ├── Constant [5]
                        ╰── Constant [5]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_precedence() {
    let src = r#"
        int main(void) {
            return 1 || 0 && 2;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [||]
                    ├── Constant [1]
                    ╰── Binary [&&]
                        ├── Constant [0]
                        ╰── Constant [2]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_precedence_2() {
    let src = r#"
        int main(void) {
            return (1 || 0) && 0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [&&]
                    ├── Binary [||]
                    │   ├── Constant [1]
                    │   ╰── Constant [0]
                    ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_precedence_3() {
    let src = r#"
        int main(void) {
            return 2 == 2 >= 0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [==]
                    ├── Constant [2]
                    ╰── Binary [>=]
                        ├── Constant [2]
                        ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_precedence_4() {
    let src = r#"
        int main(void) {
            return 2 == 2 || 0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [||]
                    ├── Binary [==]
                    │   ├── Constant [2]
                    │   ╰── Constant [2]
                    ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_precedence_5() {
    let src = r#"
        int main(void) {
            return (0 == 0 && 3 == 2 + 1 > 1) + 1;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [+]
                    ├── Binary [&&]
                    │   ├── Binary [==]
                    │   │   ├── Constant [0]
                    │   │   ╰── Constant [0]
                    │   ╰── Binary [==]
                    │       ├── Constant [3]
                    │       ╰── Binary [>]
                    │           ├── Binary [+]
                    │           │   ├── Constant [2]
                    │           │   ╰── Constant [1]
                    │           ╰── Constant [1]
                    ╰── Constant [1]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_invalid_parse_compound_invalid_operator() {
    assert_error(
        r#"
        int main(void) {
            int a = 0;
            a + = 1;
              //^ Expected expression, but found '='
            return a;
        }
    "#,
    );
}

#[test]
fn test_chapter_5_invalid_parse_declare_keyword_as_var() {
    assert_error(
        r#"
        int main(void) {
            int return = 4;
              //^^^^^^ Expected Identifier, but found 'return'
            return return + 1;
        }
    "#,
    );
}

#[test]
fn test_chapter_5_invalid_parse_extra_credit_binary_decrement() {
    assert_error(
        r#"
        int main(void) {
            int a = 0;
            return a -- 1;
                      //^ Expected Semicolon, but found '1'
        }
    "#,
    );
}

#[test]
fn test_chapter_5_invalid_parse_extra_credit_binary_increment() {
    assert_error(
        r#"
        int main(void) {
            int a = 0;
            return a ++ 1;
                      //^ Expected Semicolon, but found '1'
        }
    "#,
    );
}

#[test]
fn test_chapter_5_invalid_parse_extra_credit_compound_initializer() {
    assert_error(
        r#"
        int main(void) {
            int a += 0;
                //^^ Expected Semicolon, but found '+='
            return a;
        }
    "#,
    );
}

#[test]
fn test_chapter_5_invalid_parse_extra_credit_increment_declaration() {
    assert_error(
        r#"
        int main(void) {
            int a++;
               //^^ Expected Semicolon, but found '++'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_5_invalid_parse_invalid_specifier() {
    assert_error(
        r#"
        int main(void) {
            int foo bar = 3;
                  //^^^ Expected Semicolon, but found 'bar'
            return bar;
        }
    "#,
    );
}

#[test]
fn test_chapter_5_invalid_parse_invalid_type() {
    assert_error(
        r#"
        int main(void) {
            ints a = 1;
               //^ Expected Semicolon, but found 'a'
            return a;
        }
    "#,
    );
}

#[test]
fn test_chapter_5_invalid_parse_invalid_variable_name() {
    assert_error(
        r#"
        int main(void)
        {
            int 10 = 0;
              //^^ Expected Identifier, but found '10'
            return 10;
        }
    "#,
    );
}

#[test]
fn test_chapter_5_invalid_parse_malformed_compound_assignment() {
    assert_error(
        r#"
        int main(void) {
            int a = 10;
            a =/ 1;
             //^ Expected expression, but found '/'
            return a;
        }
    "#,
    );
}

#[test]
fn test_chapter_5_invalid_parse_malformed_decrement() {
    assert_error(
        r#"
        int main(void) {
            int a = 0;
            a - -;
               //^ Expected expression, but found ';'
            return a;
        }
    "#,
    );
}

#[test]
fn test_chapter_5_invalid_parse_malformed_increment() {
    assert_error(
        r#"
        int main(void) {
            int a = 0;
            a + +;
              //^ Expected expression, but found '+'
            return a;
        }
    "#,
    );
}

#[test]
fn test_chapter_5_invalid_parse_malformed_less_equal() {
    assert_error(
        r#"
        int main(void)
        {
            return 1 < = 2;
                     //^ Expected expression, but found '='
        }
    "#,
    );
}

#[test]
fn test_chapter_5_invalid_parse_malformed_not_equal() {
    assert_error(
        r#"
        int main(void)
        {
            return 1 ! = 0;
                   //^ Expected Semicolon, but found '!'
        }
    "#,
    );
}

#[test]
fn test_chapter_5_invalid_parse_missing_semicolon() {
    assert_error(
        r#"
        int main(void) {
            int a = 2
            a = a + 4;
          //^ Expected Semicolon, but found 'a'
            return a;
        }
    "#,
    );
}

#[test]
fn test_chapter_5_invalid_parse_return_in_assignment() {
    assert_error(
        r#"
        int main(void)
        {
            int 10 = return 0;
              //^^ Expected Identifier, but found '10'
        }
    "#,
    );
}

#[test]
fn test_chapter_5_invalid_semantics_declared_after_use() {
    let src = r#"
        int main(void) {
            a = 1 + 2;
            int a;
            return a;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Assign [=]
            │   ├── Var [a]
            │   ╰── Binary [+]
            │       ├── Constant [1]
            │       ╰── Constant [2]
            ├── Declaration [a]
            ╰── Return
                ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_invalid_semantics_extra_credit_compound_invalid_lvalue() {
    let src = r#"
        int main(void) {
            int a = 0;
            -a += 1;
            return a;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [0]
            ├── Assign [+=]
            │   ├── Unary [-]
            │   │   ╰── Var [a]
            │   ╰── Constant [1]
            ╰── Return
                ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_invalid_semantics_extra_credit_compound_invalid_lvalue_2() {
    let src = r#"
        int main(void) {
            int a = 10;
            (a += 1) -= 2;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [10]
            ╰── Assign [-=]
                ├── Assign [+=]
                │   ├── Var [a]
                │   ╰── Constant [1]
                ╰── Constant [2]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_invalid_semantics_extra_credit_postfix_decr_non_lvalue() {
    let src = r#"
        int main(void) {
            int a = 10;
            return a++--;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [10]
            ╰── Return
                ╰── Postfix [--]
                    ╰── Postfix [++]
                        ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_invalid_semantics_extra_credit_postfix_incr_non_lvalue() {
    let src = r#"
        int main(void) {
            int a = 0;
            (a = 4)++;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [0]
            ╰── Postfix [++]
                ╰── Assign [=]
                    ├── Var [a]
                    ╰── Constant [4]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_invalid_semantics_extra_credit_prefix_decr_non_lvalue() {
    let src = r#"
        int main(void) {
            return --3;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Unary [--]
                    ╰── Constant [3]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_invalid_semantics_extra_credit_prefix_incr_non_lvalue() {
    let src = r#"
        int main(void) {
            int a = 1;
            ++(a+1);
            return 0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [1]
            ├── Unary [++]
            │   ╰── Binary [+]
            │       ├── Var [a]
            │       ╰── Constant [1]
            ╰── Return
                ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_invalid_semantics_extra_credit_undeclared_bitwise_op() {
    let src = r#"
        int main(void){
            return a >> 2;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [>>]
                    ├── Var [a]
                    ╰── Constant [2]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_invalid_semantics_extra_credit_undeclared_compound_assignment() {
    let src = r#"
        int main(void) {
            a += 1;
            return 0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Assign [+=]
            │   ├── Var [a]
            │   ╰── Constant [1]
            ╰── Return
                ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_invalid_semantics_extra_credit_undeclared_compound_assignment_use() {
    let src = r#"
        int main(void) {
            int b = 10;
            b *= a;
            return 0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [b]
            │   ╰── Constant [10]
            ├── Assign [*=]
            │   ├── Var [b]
            │   ╰── Var [a]
            ╰── Return
                ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_invalid_semantics_extra_credit_undeclared_postfix_decr() {
    let src = r#"
        int main(void) {
            a--;
            return 0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Postfix [--]
            │   ╰── Var [a]
            ╰── Return
                ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_invalid_semantics_extra_credit_undeclared_prefix_incr() {
    let src = r#"
        int main(void) {
            a++;
            return 0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Postfix [++]
            │   ╰── Var [a]
            ╰── Return
                ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_invalid_semantics_invalid_lvalue() {
    let src = r#"
        int main(void) {
            int a = 2;
            a + 3 = 4;
            return a;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [2]
            ├── Assign [=]
            │   ├── Binary [+]
            │   │   ├── Var [a]
            │   │   ╰── Constant [3]
            │   ╰── Constant [4]
            ╰── Return
                ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_invalid_semantics_invalid_lvalue_2() {
    let src = r#"
        int main(void) {
            int a = 2;
            !a = 3;
            return a;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [2]
            ├── Assign [=]
            │   ├── Unary [!]
            │   │   ╰── Var [a]
            │   ╰── Constant [3]
            ╰── Return
                ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_invalid_semantics_mixed_precedence_assignment() {
    let src = r#"
        int main(void) {
            int a = 1;
            int b = 2;
            a = 3 * b = a;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [1]
            ├── Declaration [b]
            │   ╰── Constant [2]
            ╰── Assign [=]
                ├── Var [a]
                ╰── Assign [=]
                    ├── Binary [*]
                    │   ├── Constant [3]
                    │   ╰── Var [b]
                    ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_invalid_semantics_redefine() {
    let src = r#"
        int main(void) {
            int a = 1;
            int a = 2;
            return a;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [1]
            ├── Declaration [a]
            │   ╰── Constant [2]
            ╰── Return
                ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_invalid_semantics_undeclared_var() {
    let src = r#"
        int main(void) {
            return a;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_invalid_semantics_undeclared_var_and() {
    let src = r#"
        int main(void) {
            return 0 && a;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [&&]
                    ├── Constant [0]
                    ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_invalid_semantics_undeclared_var_compare() {
    let src = r#"
        int main(void) {
            return a < 5;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Binary [<]
                    ├── Var [a]
                    ╰── Constant [5]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_invalid_semantics_undeclared_var_unary() {
    let src = r#"
        int main(void) {
            return -a;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Unary [-]
                    ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_invalid_semantics_use_then_redefine() {
    let src = r#"
        int main(void) {
            int a = 0;
            return a;
            int a = 1;
            return a;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [0]
            ├── Return
            │   ╰── Var [a]
            ├── Declaration [a]
            │   ╰── Constant [1]
            ╰── Return
                ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_add_variables() {
    let src = r#"
        int main(void) {
            int first_variable = 1;
            int second_variable = 2;
            return first_variable + second_variable;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [first_variable]
            │   ╰── Constant [1]
            ├── Declaration [second_variable]
            │   ╰── Constant [2]
            ╰── Return
                ╰── Binary [+]
                    ├── Var [first_variable]
                    ╰── Var [second_variable]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_allocate_temps_and_vars() {
    let src = r#"
        int main(void) {
            int a = 2147483646;
            int b = 0;
            int c = a / 6 + !b;
            return c * 2 == a - 1431655762;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [2147483646]
            ├── Declaration [b]
            │   ╰── Constant [0]
            ├── Declaration [c]
            │   ╰── Binary [+]
            │       ├── Binary [/]
            │       │   ├── Var [a]
            │       │   ╰── Constant [6]
            │       ╰── Unary [!]
            │           ╰── Var [b]
            ╰── Return
                ╰── Binary [==]
                    ├── Binary [*]
                    │   ├── Var [c]
                    │   ╰── Constant [2]
                    ╰── Binary [-]
                        ├── Var [a]
                        ╰── Constant [1431655762]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_assign() {
    let src = r#"
        int main(void) {
            int var0;
            var0 = 2;
            return var0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [var0]
            ├── Assign [=]
            │   ├── Var [var0]
            │   ╰── Constant [2]
            ╰── Return
                ╰── Var [var0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_assign_val_in_initializer() {
    let src = r#"
        int main(void) {
            int a = a = 5;
            return a;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Assign [=]
            │       ├── Var [a]
            │       ╰── Constant [5]
            ╰── Return
                ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_assignment_in_initializer() {
    let src = r#"
        int main(void) {
            int a;
            int b = a = 0;
            return b;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            ├── Declaration [b]
            │   ╰── Assign [=]
            │       ├── Var [a]
            │       ╰── Constant [0]
            ╰── Return
                ╰── Var [b]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_assignment_lowest_precedence() {
    let src = r#"
        int main(void) {
            int a;
            a = 0 || 5;
            return a;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            ├── Assign [=]
            │   ├── Var [a]
            │   ╰── Binary [||]
            │       ├── Constant [0]
            │       ╰── Constant [5]
            ╰── Return
                ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_empty_function_body() {
    let src = r#"
        int main(void) {
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_exp_then_declaration() {
    let src = r#"
        int main(void) {
            int a = -2593;
            a = a % 3;
            int b = -a;
            return b;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Unary [-]
            │       ╰── Constant [2593]
            ├── Assign [=]
            │   ├── Var [a]
            │   ╰── Binary [%]
            │       ├── Var [a]
            │       ╰── Constant [3]
            ├── Declaration [b]
            │   ╰── Unary [-]
            │       ╰── Var [a]
            ╰── Return
                ╰── Var [b]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_extra_credit_bitwise_in_initializer() {
    let src = r#"
        int main(void) {
            int a = 15;
            int b = a ^ 5;
            return 1 | b;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [15]
            ├── Declaration [b]
            │   ╰── Binary [^]
            │       ├── Var [a]
            │       ╰── Constant [5]
            ╰── Return
                ╰── Binary [|]
                    ├── Constant [1]
                    ╰── Var [b]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_extra_credit_bitwise_ops_vars() {
    let src = r#"
        int main(void) {
            int a = 3;
            int b = 5;
            int c = 8;
            return a & b | c;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [3]
            ├── Declaration [b]
            │   ╰── Constant [5]
            ├── Declaration [c]
            │   ╰── Constant [8]
            ╰── Return
                ╰── Binary [|]
                    ├── Binary [&]
                    │   ├── Var [a]
                    │   ╰── Var [b]
                    ╰── Var [c]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_extra_credit_bitwise_shiftl_variable() {
    let src = r#"
        int main(void) {
            int x = 3;
            return x << 3;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [x]
            │   ╰── Constant [3]
            ╰── Return
                ╰── Binary [<<]
                    ├── Var [x]
                    ╰── Constant [3]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_extra_credit_bitwise_shiftr_assign() {
    let src = r#"
        int main(void) {
            int var_to_shift = 1234;
            int x = 0;
            x = var_to_shift >> 4;
            return x;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [var_to_shift]
            │   ╰── Constant [1234]
            ├── Declaration [x]
            │   ╰── Constant [0]
            ├── Assign [=]
            │   ├── Var [x]
            │   ╰── Binary [>>]
            │       ├── Var [var_to_shift]
            │       ╰── Constant [4]
            ╰── Return
                ╰── Var [x]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_extra_credit_compound_assignment_chained() {
    let src = r#"
        int main(void) {
            int a = 250;
            int b = 200;
            int c = 100;
            int d = 75;
            int e = -25;
            int f = 0;
            int x = 0;
            x = a += b -= c *= d /= e %= f = -7;
            return a == 2250 && b == 2000 && c == -1800 && d == -18 && e == -4 &&
                   f == -7 && x == 2250;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [250]
            ├── Declaration [b]
            │   ╰── Constant [200]
            ├── Declaration [c]
            │   ╰── Constant [100]
            ├── Declaration [d]
            │   ╰── Constant [75]
            ├── Declaration [e]
            │   ╰── Unary [-]
            │       ╰── Constant [25]
            ├── Declaration [f]
            │   ╰── Constant [0]
            ├── Declaration [x]
            │   ╰── Constant [0]
            ├── Assign [=]
            │   ├── Var [x]
            │   ╰── Assign [+=]
            │       ├── Var [a]
            │       ╰── Assign [-=]
            │           ├── Var [b]
            │           ╰── Assign [*=]
            │               ├── Var [c]
            │               ╰── Assign [/=]
            │                   ├── Var [d]
            │                   ╰── Assign [&=]
            │                       ├── Var [e]
            │                       ╰── Assign [=]
            │                           ├── Var [f]
            │                           ╰── Unary [-]
            │                               ╰── Constant [7]
            ╰── Return
                ╰── Binary [&&]
                    ├── Binary [&&]
                    │   ├── Binary [&&]
                    │   │   ├── Binary [&&]
                    │   │   │   ├── Binary [&&]
                    │   │   │   │   ├── Binary [&&]
                    │   │   │   │   │   ├── Binary [==]
                    │   │   │   │   │   │   ├── Var [a]
                    │   │   │   │   │   │   ╰── Constant [2250]
                    │   │   │   │   │   ╰── Binary [==]
                    │   │   │   │   │       ├── Var [b]
                    │   │   │   │   │       ╰── Constant [2000]
                    │   │   │   │   ╰── Binary [==]
                    │   │   │   │       ├── Var [c]
                    │   │   │   │       ╰── Unary [-]
                    │   │   │   │           ╰── Constant [1800]
                    │   │   │   ╰── Binary [==]
                    │   │   │       ├── Var [d]
                    │   │   │       ╰── Unary [-]
                    │   │   │           ╰── Constant [18]
                    │   │   ╰── Binary [==]
                    │   │       ├── Var [e]
                    │   │       ╰── Unary [-]
                    │   │           ╰── Constant [4]
                    │   ╰── Binary [==]
                    │       ├── Var [f]
                    │       ╰── Unary [-]
                    │           ╰── Constant [7]
                    ╰── Binary [==]
                        ├── Var [x]
                        ╰── Constant [2250]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_extra_credit_compound_assignment_lowest_precedence() {
    let src = r#"
        int main(void) {
            int a = 10;
            int b = 12;
            a += 0 || b;
            b *= a && 0;
            int c = 14;
            c -= a || b;
            int d = 16;
            d /= c || d;
            return (a == 11 && b == 0 && c == 13 && d == 16);
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [10]
            ├── Declaration [b]
            │   ╰── Constant [12]
            ├── Assign [+=]
            │   ├── Var [a]
            │   ╰── Binary [||]
            │       ├── Constant [0]
            │       ╰── Var [b]
            ├── Assign [*=]
            │   ├── Var [b]
            │   ╰── Binary [&&]
            │       ├── Var [a]
            │       ╰── Constant [0]
            ├── Declaration [c]
            │   ╰── Constant [14]
            ├── Assign [-=]
            │   ├── Var [c]
            │   ╰── Binary [||]
            │       ├── Var [a]
            │       ╰── Var [b]
            ├── Declaration [d]
            │   ╰── Constant [16]
            ├── Assign [/=]
            │   ├── Var [d]
            │   ╰── Binary [||]
            │       ├── Var [c]
            │       ╰── Var [d]
            ╰── Return
                ╰── Binary [&&]
                    ├── Binary [&&]
                    │   ├── Binary [&&]
                    │   │   ├── Binary [==]
                    │   │   │   ├── Var [a]
                    │   │   │   ╰── Constant [11]
                    │   │   ╰── Binary [==]
                    │   │       ├── Var [b]
                    │   │       ╰── Constant [0]
                    │   ╰── Binary [==]
                    │       ├── Var [c]
                    │       ╰── Constant [13]
                    ╰── Binary [==]
                        ├── Var [d]
                        ╰── Constant [16]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_extra_credit_compound_assignment_use_result() {
    let src = r#"
        int main(void) {
            int x = 1;
            int y = x += 3;
            return (x == 4 && y == 4);
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [x]
            │   ╰── Constant [1]
            ├── Declaration [y]
            │   ╰── Assign [+=]
            │       ├── Var [x]
            │       ╰── Constant [3]
            ╰── Return
                ╰── Binary [&&]
                    ├── Binary [==]
                    │   ├── Var [x]
                    │   ╰── Constant [4]
                    ╰── Binary [==]
                        ├── Var [y]
                        ╰── Constant [4]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_extra_credit_compound_bitwise_and() {
    let src = r#"
        int main(void) {
            int to_and = 3;
            to_and &= 6;
            return to_and;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [to_and]
            │   ╰── Constant [3]
            ├── Assign [&=]
            │   ├── Var [to_and]
            │   ╰── Constant [6]
            ╰── Return
                ╰── Var [to_and]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_extra_credit_compound_bitwise_assignment_lowest_precedence() {
    let src = r#"
        int main(void) {
            int a = 11;
            int b = 12;
            a &= 0 || b;
            b ^= a || 1;
            int c = 14;
            c |= a || b;
            int d = 16;
            d >>= c || d;
            int e = 18;
            e <<= c || d;
            return (a == 1 && b == 13 && c == 15 && d == 8 && e == 36);
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [11]
            ├── Declaration [b]
            │   ╰── Constant [12]
            ├── Assign [&=]
            │   ├── Var [a]
            │   ╰── Binary [||]
            │       ├── Constant [0]
            │       ╰── Var [b]
            ├── Assign [^=]
            │   ├── Var [b]
            │   ╰── Binary [||]
            │       ├── Var [a]
            │       ╰── Constant [1]
            ├── Declaration [c]
            │   ╰── Constant [14]
            ├── Assign [|=]
            │   ├── Var [c]
            │   ╰── Binary [||]
            │       ├── Var [a]
            │       ╰── Var [b]
            ├── Declaration [d]
            │   ╰── Constant [16]
            ├── Assign [>>=]
            │   ├── Var [d]
            │   ╰── Binary [||]
            │       ├── Var [c]
            │       ╰── Var [d]
            ├── Declaration [e]
            │   ╰── Constant [18]
            ├── Assign [<<=]
            │   ├── Var [e]
            │   ╰── Binary [||]
            │       ├── Var [c]
            │       ╰── Var [d]
            ╰── Return
                ╰── Binary [&&]
                    ├── Binary [&&]
                    │   ├── Binary [&&]
                    │   │   ├── Binary [&&]
                    │   │   │   ├── Binary [==]
                    │   │   │   │   ├── Var [a]
                    │   │   │   │   ╰── Constant [1]
                    │   │   │   ╰── Binary [==]
                    │   │   │       ├── Var [b]
                    │   │   │       ╰── Constant [13]
                    │   │   ╰── Binary [==]
                    │   │       ├── Var [c]
                    │   │       ╰── Constant [15]
                    │   ╰── Binary [==]
                    │       ├── Var [d]
                    │       ╰── Constant [8]
                    ╰── Binary [==]
                        ├── Var [e]
                        ╰── Constant [36]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_extra_credit_compound_bitwise_chained() {
    let src = r#"
        int main(void) {
            int a = 250;
            int b = 200;
            int c = 100;
            int d = 75;
            int e = 50;
            int f = 25;
            int g = 10;
            int h = 1;
            int j = 0;
            int x = 0;
            x = a &= b *= c |= d = e ^= f += g >>= h <<= j = 1;
            return (a == 40 && b == 21800 && c == 109 && d == 41 && e == 41 &&
                    f == 27 && g == 2 && h == 2 && j == 1 && x == 40);
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [250]
            ├── Declaration [b]
            │   ╰── Constant [200]
            ├── Declaration [c]
            │   ╰── Constant [100]
            ├── Declaration [d]
            │   ╰── Constant [75]
            ├── Declaration [e]
            │   ╰── Constant [50]
            ├── Declaration [f]
            │   ╰── Constant [25]
            ├── Declaration [g]
            │   ╰── Constant [10]
            ├── Declaration [h]
            │   ╰── Constant [1]
            ├── Declaration [j]
            │   ╰── Constant [0]
            ├── Declaration [x]
            │   ╰── Constant [0]
            ├── Assign [=]
            │   ├── Var [x]
            │   ╰── Assign [&=]
            │       ├── Var [a]
            │       ╰── Assign [*=]
            │           ├── Var [b]
            │           ╰── Assign [|=]
            │               ├── Var [c]
            │               ╰── Assign [=]
            │                   ├── Var [d]
            │                   ╰── Assign [^=]
            │                       ├── Var [e]
            │                       ╰── Assign [+=]
            │                           ├── Var [f]
            │                           ╰── Assign [>>=]
            │                               ├── Var [g]
            │                               ╰── Assign [<<=]
            │                                   ├── Var [h]
            │                                   ╰── Assign [=]
            │                                       ├── Var [j]
            │                                       ╰── Constant [1]
            ╰── Return
                ╰── Binary [&&]
                    ├── Binary [&&]
                    │   ├── Binary [&&]
                    │   │   ├── Binary [&&]
                    │   │   │   ├── Binary [&&]
                    │   │   │   │   ├── Binary [&&]
                    │   │   │   │   │   ├── Binary [&&]
                    │   │   │   │   │   │   ├── Binary [&&]
                    │   │   │   │   │   │   │   ├── Binary [&&]
                    │   │   │   │   │   │   │   │   ├── Binary [==]
                    │   │   │   │   │   │   │   │   │   ├── Var [a]
                    │   │   │   │   │   │   │   │   │   ╰── Constant [40]
                    │   │   │   │   │   │   │   │   ╰── Binary [==]
                    │   │   │   │   │   │   │   │       ├── Var [b]
                    │   │   │   │   │   │   │   │       ╰── Constant [21800]
                    │   │   │   │   │   │   │   ╰── Binary [==]
                    │   │   │   │   │   │   │       ├── Var [c]
                    │   │   │   │   │   │   │       ╰── Constant [109]
                    │   │   │   │   │   │   ╰── Binary [==]
                    │   │   │   │   │   │       ├── Var [d]
                    │   │   │   │   │   │       ╰── Constant [41]
                    │   │   │   │   │   ╰── Binary [==]
                    │   │   │   │   │       ├── Var [e]
                    │   │   │   │   │       ╰── Constant [41]
                    │   │   │   │   ╰── Binary [==]
                    │   │   │   │       ├── Var [f]
                    │   │   │   │       ╰── Constant [27]
                    │   │   │   ╰── Binary [==]
                    │   │   │       ├── Var [g]
                    │   │   │       ╰── Constant [2]
                    │   │   ╰── Binary [==]
                    │   │       ├── Var [h]
                    │   │       ╰── Constant [2]
                    │   ╰── Binary [==]
                    │       ├── Var [j]
                    │       ╰── Constant [1]
                    ╰── Binary [==]
                        ├── Var [x]
                        ╰── Constant [40]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_extra_credit_compound_bitwise_or() {
    let src = r#"
        int main(void) {
            int to_or = 1;
            to_or |= 30;
            return to_or;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [to_or]
            │   ╰── Constant [1]
            ├── Assign [|=]
            │   ├── Var [to_or]
            │   ╰── Constant [30]
            ╰── Return
                ╰── Var [to_or]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_extra_credit_compound_bitwise_shiftl() {
    let src = r#"
        int main(void) {
            int to_shiftl = 3;
            to_shiftl <<= 4;
            return to_shiftl;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [to_shiftl]
            │   ╰── Constant [3]
            ├── Assign [<<=]
            │   ├── Var [to_shiftl]
            │   ╰── Constant [4]
            ╰── Return
                ╰── Var [to_shiftl]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_extra_credit_compound_bitwise_shiftr() {
    let src = r#"
        int main(void) {
            int to_shiftr = 382574;
            to_shiftr >>= 4;
            return to_shiftr;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [to_shiftr]
            │   ╰── Constant [382574]
            ├── Assign [>>=]
            │   ├── Var [to_shiftr]
            │   ╰── Constant [4]
            ╰── Return
                ╰── Var [to_shiftr]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_extra_credit_compound_bitwise_xor() {
    let src = r#"
        int main(void) {
            int to_xor = 7;
            to_xor ^= 5;
            return to_xor;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [to_xor]
            │   ╰── Constant [7]
            ├── Assign [^=]
            │   ├── Var [to_xor]
            │   ╰── Constant [5]
            ╰── Return
                ╰── Var [to_xor]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_extra_credit_compound_divide() {
    let src = r#"
        int main(void) {
            int to_divide = 8;
            to_divide /= 4;
            return to_divide;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [to_divide]
            │   ╰── Constant [8]
            ├── Assign [/=]
            │   ├── Var [to_divide]
            │   ╰── Constant [4]
            ╰── Return
                ╰── Var [to_divide]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_extra_credit_compound_minus() {
    let src = r#"
        int main(void) {
            int to_subtract = 10;
            to_subtract -= 8;
            return to_subtract;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [to_subtract]
            │   ╰── Constant [10]
            ├── Assign [-=]
            │   ├── Var [to_subtract]
            │   ╰── Constant [8]
            ╰── Return
                ╰── Var [to_subtract]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_extra_credit_compound_mod() {
    let src = r#"
        int main(void) {
            int to_mod = 5;
            to_mod %= 3;
            return to_mod;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [to_mod]
            │   ╰── Constant [5]
            ├── Assign [&=]
            │   ├── Var [to_mod]
            │   ╰── Constant [3]
            ╰── Return
                ╰── Var [to_mod]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_extra_credit_compound_multiply() {
    let src = r#"
        int main(void) {
            int to_multiply = 4;
            to_multiply *= 3;
            return to_multiply;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [to_multiply]
            │   ╰── Constant [4]
            ├── Assign [*=]
            │   ├── Var [to_multiply]
            │   ╰── Constant [3]
            ╰── Return
                ╰── Var [to_multiply]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_extra_credit_compound_plus() {
    let src = r#"
        int main(void) {
            int to_add = 0;
            to_add += 4;
            return to_add;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [to_add]
            │   ╰── Constant [0]
            ├── Assign [+=]
            │   ├── Var [to_add]
            │   ╰── Constant [4]
            ╰── Return
                ╰── Var [to_add]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_extra_credit_incr_expression_statement() {
    let src = r#"
        int main(void) {
            int a = 0;
            int b = 0;
            a++;
            ++a;
            ++a;
            b--;
            --b;
            return (a == 3 && b == -2);
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [0]
            ├── Declaration [b]
            │   ╰── Constant [0]
            ├── Postfix [++]
            │   ╰── Var [a]
            ├── Unary [++]
            │   ╰── Var [a]
            ├── Unary [++]
            │   ╰── Var [a]
            ├── Postfix [--]
            │   ╰── Var [b]
            ├── Unary [--]
            │   ╰── Var [b]
            ╰── Return
                ╰── Binary [&&]
                    ├── Binary [==]
                    │   ├── Var [a]
                    │   ╰── Constant [3]
                    ╰── Binary [==]
                        ├── Var [b]
                        ╰── Unary [-]
                            ╰── Constant [2]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_extra_credit_incr_in_binary_expr() {
    let src = r#"
        int main(void) {
            int a = 2;
            int b = 3 + a++;
            int c = 4 + ++b;
            return (a == 3 && b == 6 && c == 10);
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [2]
            ├── Declaration [b]
            │   ╰── Binary [+]
            │       ├── Constant [3]
            │       ╰── Postfix [++]
            │           ╰── Var [a]
            ├── Declaration [c]
            │   ╰── Binary [+]
            │       ├── Constant [4]
            │       ╰── Unary [++]
            │           ╰── Var [b]
            ╰── Return
                ╰── Binary [&&]
                    ├── Binary [&&]
                    │   ├── Binary [==]
                    │   │   ├── Var [a]
                    │   │   ╰── Constant [3]
                    │   ╰── Binary [==]
                    │       ├── Var [b]
                    │       ╰── Constant [6]
                    ╰── Binary [==]
                        ├── Var [c]
                        ╰── Constant [10]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_extra_credit_incr_parenthesized() {
    let src = r#"
        
        int main(void) {
            int a = 1;
            int b = 2;
            int c = -++(a);
            int d = !(b)--;
            return (a == 2 && b == 1 && c == -2 && d == 0);
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [1]
            ├── Declaration [b]
            │   ╰── Constant [2]
            ├── Declaration [c]
            │   ╰── Unary [-]
            │       ╰── Unary [++]
            │           ╰── Var [a]
            ├── Declaration [d]
            │   ╰── Unary [!]
            │       ╰── Postfix [--]
            │           ╰── Var [b]
            ╰── Return
                ╰── Binary [&&]
                    ├── Binary [&&]
                    │   ├── Binary [&&]
                    │   │   ├── Binary [==]
                    │   │   │   ├── Var [a]
                    │   │   │   ╰── Constant [2]
                    │   │   ╰── Binary [==]
                    │   │       ├── Var [b]
                    │   │       ╰── Constant [1]
                    │   ╰── Binary [==]
                    │       ├── Var [c]
                    │       ╰── Unary [-]
                    │           ╰── Constant [2]
                    ╰── Binary [==]
                        ├── Var [d]
                        ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_extra_credit_postfix_incr_and_decr() {
    let src = r#"
        int main(void) {
            int a = 1;
            int b = 2;
            int c = a++;
            int d = b--;
            return (a == 2 && b == 1 && c == 1 && d == 2);
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [1]
            ├── Declaration [b]
            │   ╰── Constant [2]
            ├── Declaration [c]
            │   ╰── Postfix [++]
            │       ╰── Var [a]
            ├── Declaration [d]
            │   ╰── Postfix [--]
            │       ╰── Var [b]
            ╰── Return
                ╰── Binary [&&]
                    ├── Binary [&&]
                    │   ├── Binary [&&]
                    │   │   ├── Binary [==]
                    │   │   │   ├── Var [a]
                    │   │   │   ╰── Constant [2]
                    │   │   ╰── Binary [==]
                    │   │       ├── Var [b]
                    │   │       ╰── Constant [1]
                    │   ╰── Binary [==]
                    │       ├── Var [c]
                    │       ╰── Constant [1]
                    ╰── Binary [==]
                        ├── Var [d]
                        ╰── Constant [2]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_extra_credit_postfix_precedence() {
    let src = r#"
        int main(void) {
            int a = 1;
            int b = !a++;
            return (a == 2 && b == 0);
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [1]
            ├── Declaration [b]
            │   ╰── Unary [!]
            │       ╰── Postfix [++]
            │           ╰── Var [a]
            ╰── Return
                ╰── Binary [&&]
                    ├── Binary [==]
                    │   ├── Var [a]
                    │   ╰── Constant [2]
                    ╰── Binary [==]
                        ├── Var [b]
                        ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_extra_credit_prefix_incr_and_decr() {
    let src = r#"
        int main(void) {
            int a = 1;
            int b = 2;
            int c = ++a;
            int d = --b;
            return (a == 2 && b == 1 && c == 2 && d == 1);
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [1]
            ├── Declaration [b]
            │   ╰── Constant [2]
            ├── Declaration [c]
            │   ╰── Unary [++]
            │       ╰── Var [a]
            ├── Declaration [d]
            │   ╰── Unary [--]
            │       ╰── Var [b]
            ╰── Return
                ╰── Binary [&&]
                    ├── Binary [&&]
                    │   ├── Binary [&&]
                    │   │   ├── Binary [==]
                    │   │   │   ├── Var [a]
                    │   │   │   ╰── Constant [2]
                    │   │   ╰── Binary [==]
                    │   │       ├── Var [b]
                    │   │       ╰── Constant [1]
                    │   ╰── Binary [==]
                    │       ├── Var [c]
                    │       ╰── Constant [2]
                    ╰── Binary [==]
                        ├── Var [d]
                        ╰── Constant [1]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_kw_var_names() {
    let src = r#"
        int main(void) {
            int return_val = 3;
            int void2 = 2;
            return return_val + void2;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [return_val]
            │   ╰── Constant [3]
            ├── Declaration [void2]
            │   ╰── Constant [2]
            ╰── Return
                ╰── Binary [+]
                    ├── Var [return_val]
                    ╰── Var [void2]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_local_var_missing_return() {
    let src = r#"
        int main(void) {
            int a = 3;
            a = a + 5;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [3]
            ╰── Assign [=]
                ├── Var [a]
                ╰── Binary [+]
                    ├── Var [a]
                    ╰── Constant [5]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_mixed_precedence_assignment() {
    let src = r#"
        int main(void) {
            int a = 1;
            int b = 0;
            a = 3 * (b = a);
            return a + b;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [1]
            ├── Declaration [b]
            │   ╰── Constant [0]
            ├── Assign [=]
            │   ├── Var [a]
            │   ╰── Binary [*]
            │       ├── Constant [3]
            │       ╰── Assign [=]
            │           ├── Var [b]
            │           ╰── Var [a]
            ╰── Return
                ╰── Binary [+]
                    ├── Var [a]
                    ╰── Var [b]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_non_short_circuit_or() {
    let src = r#"
        int main(void) {
            int a = 0;
            0 || (a = 1);
            return a;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [0]
            ├── Binary [||]
            │   ├── Constant [0]
            │   ╰── Assign [=]
            │       ├── Var [a]
            │       ╰── Constant [1]
            ╰── Return
                ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_null_statement() {
    let src = r#"
        int main(void) {
            ;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_null_then_return() {
    let src = r#"
        int main(void) {
            ;
            return 0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_return_var() {
    let src = r#"
        int main(void) {
            int a = 2;
            return a;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [2]
            ╰── Return
                ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_short_circuit_and_fail() {
    let src = r#"
        int main(void) {
            int a = 0;
            0 && (a = 5);
            return a;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [0]
            ├── Binary [&&]
            │   ├── Constant [0]
            │   ╰── Assign [=]
            │       ├── Var [a]
            │       ╰── Constant [5]
            ╰── Return
                ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_short_circuit_or() {
    let src = r#"
        int main(void) {
            int a = 0;
            1 || (a = 1);
            return a;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [0]
            ├── Binary [||]
            │   ├── Constant [1]
            │   ╰── Assign [=]
            │       ├── Var [a]
            │       ╰── Constant [1]
            ╰── Return
                ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_unused_exp() {
    let src = r#"
        int main(void) {
            2 + 2;
            return 0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Binary [+]
            │   ├── Constant [2]
            │   ╰── Constant [2]
            ╰── Return
                ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_use_assignment_result() {
    let src = r#"
        int main(void) {
            int a = 1;
            int b = 2;
            return a = b = 4;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [1]
            ├── Declaration [b]
            │   ╰── Constant [2]
            ╰── Return
                ╰── Assign [=]
                    ├── Var [a]
                    ╰── Assign [=]
                        ├── Var [b]
                        ╰── Constant [4]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_use_val_in_own_initializer() {
    let src = r#"
        int main(void) {
            int a = 0 && a;
            return a;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Binary [&&]
            │       ├── Constant [0]
            │       ╰── Var [a]
            ╰── Return
                ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}
