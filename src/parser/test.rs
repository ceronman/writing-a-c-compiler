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
    
// Expected statement, but found ''"#,
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

#[test]
fn test_chapter_6_invalid_parse_declaration_as_statement() {
    assert_error(
        r#"
        int main(void) {
            if (5)
                int i = 0;
              //^^^ Expected statement, but found 'int'
        }
    "#,
    );
}

#[test]
fn test_chapter_6_invalid_parse_empty_if_body() {
    assert_error(
        r#"
        int main(void) {
            if (0) else return 0;
                 //^^^^ Expected statement, but found 'else'
        }
    "#,
    );
}

#[test]
fn test_chapter_6_invalid_parse_extra_credit_goto_without_label() {
    assert_error(
        r#"
        int main(void) {
            goto;
              //^ Expected Identifier, but found ';'
        lbl:
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_6_invalid_parse_extra_credit_kw_label() {
    assert_error(
        r#"
        int main(void) {
            return: return 0;
                //^ Expected expression, but found ':'
        }
    "#,
    );
}

#[test]
fn test_chapter_6_invalid_parse_extra_credit_label_declaration() {
    assert_error(
        r#"
        int main(void) {
        label:
            int a = 0;
          //^^^ Expected statement, but found 'int'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_6_invalid_parse_extra_credit_label_expression_clause() {
    assert_error(
        r#"
        int main(void) {
            1 && label: 2;
                    //^ Expected Semicolon, but found ':'
        }
    "#,
    );
}

#[test]
fn test_chapter_6_invalid_parse_extra_credit_label_outside_function() {
    assert_error(
        r#"
        label:
      //^^^^^ Expected Int, but found 'label'
        int main(void) {
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_6_invalid_parse_extra_credit_label_without_statement() {
    assert_error(
        r#"
        int main(void) {
            foo:
        }
      //^ Expected statement, but found '}'
    "#,
    );
}

#[test]
fn test_chapter_6_invalid_parse_extra_credit_parenthesized_label() {
    assert_error(
        r#"
        int main(void) {
            goto(a);
              //^ Expected Identifier, but found '('
        a:
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_6_invalid_parse_if_assignment() {
    assert_error(
        r#"
        int main(void) {
            int flag = 0;
            int a = if (flag)
                  //^^ Expected expression, but found 'if'
                        2;
                    else
                        3;
            return a;
        }
    "#,
    );
}

#[test]
fn test_chapter_6_invalid_parse_if_no_parens() {
    assert_error(
        r#"
        int main(void) {
            if 0 return 1;
             //^ Expected OpenParen, but found '0'
        }
    "#,
    );
}

#[test]
fn test_chapter_6_invalid_parse_incomplete_ternary() {
    assert_error(
        r#"
        int main(void) {
            return 1 ? 2;
                      //^ Expected Colon, but found ';'
        }
    "#,
    );
}

#[test]
fn test_chapter_6_invalid_parse_malformed_ternary() {
    assert_error(
        r#"
        int main(void) {
            return 1 ? 2 : 3 : 4;
                           //^ Expected Semicolon, but found ':'
        }
    "#,
    );
}

#[test]
fn test_chapter_6_invalid_parse_malformed_ternary_2() {
    assert_error(
        r#"
        int main(void) {
            return 1 ? 2 ? 3 : 4;
                              //^ Expected Colon, but found ';'
        }
    "#,
    );
}

#[test]
fn test_chapter_6_invalid_parse_mismatched_nesting() {
    assert_error(
        r#"
        int main(void) {
            int a = 0;
            if (1)
                return 1;
            else
                return 2;
            else
          //^^^^ Expected statement, but found 'else'
                return 3;
        }
    "#,
    );
}

#[test]
fn test_chapter_6_invalid_parse_wrong_ternary_delimiter() {
    assert_error(
        r#"
        int main(void) {
            int x = 10;
            return x ? 1 = 2;
                          //^ Expected Colon, but found ';'
        }
    "#,
    );
}

#[test]
fn test_chapter_6_invalid_semantics_extra_credit_duplicate_labels() {
    let src = r#"
        
        int main(void) {
            int x = 0;
        label:
            x = 1;
        label:
            return 2;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [x]
            │   ╰── Constant [0]
            ├── Label [label]
            │   ╰── Assign [=]
            │       ├── Var [x]
            │       ╰── Constant [1]
            ╰── Label [label]
                ╰── Return
                    ╰── Constant [2]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_6_invalid_semantics_extra_credit_goto_missing_label() {
    let src = r#"
        int main(void) {
            goto label;
            return 0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Goto [label]
            ╰── Return
                ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_6_invalid_semantics_extra_credit_goto_variable() {
    let src = r#"
        int main(void) {
            int a;
            goto a;
            return 0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            ├── Goto [a]
            ╰── Return
                ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_6_invalid_semantics_extra_credit_undeclared_var_in_labeled_statement() {
    let src = r#"
        int main(void) {
        lbl:
            return a;
            return 0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Label [lbl]
            │   ╰── Return
            │       ╰── Var [a]
            ╰── Return
                ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_6_invalid_semantics_extra_credit_use_label_as_variable() {
    let src = r#"
        int main(void) {
            int x = 0;
            a:
            x = a;
            return 0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [x]
            │   ╰── Constant [0]
            ├── Label [a]
            │   ╰── Assign [=]
            │       ├── Var [x]
            │       ╰── Var [a]
            ╰── Return
                ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_6_invalid_semantics_invalid_var_in_if() {
    let src = r#"
        int main(void) {
            if (1)
                return c;
            int c = 0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── If
            │   ├── Constant [1]
            │   ╰── Return
            │       ╰── Var [c]
            ├── If
            ╰── Declaration [c]
                ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_6_invalid_semantics_ternary_assign() {
    let src = r#"
        int main(void) {
            int a = 2;
            int b = 1;
            a > b ? a = 1 : a = 0;
            return a;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [2]
            ├── Declaration [b]
            │   ╰── Constant [1]
            ├── Assign [=]
            │   ├── Cond [?]
            │   │   ├── Binary [>]
            │   │   │   ├── Var [a]
            │   │   │   ╰── Var [b]
            │   │   ├── Assign [=]
            │   │   │   ├── Var [a]
            │   │   │   ╰── Constant [1]
            │   │   ╰── Var [a]
            │   ╰── Constant [0]
            ╰── Return
                ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_6_invalid_semantics_undeclared_var_in_ternary() {
    let src = r#"
        int main(void) {
            return a > 0 ? 1 : 2;
            int a = 5;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Return
            │   ╰── Cond [?]
            │       ├── Binary [>]
            │       │   ├── Var [a]
            │       │   ╰── Constant [0]
            │       ├── Constant [1]
            │       ╰── Constant [2]
            ╰── Declaration [a]
                ╰── Constant [5]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_assign_ternary() {
    let src = r#"
        int main(void) {
            int a = 0;
            a = 1 ? 2 : 3;
            return a;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [0]
            ├── Assign [=]
            │   ├── Var [a]
            │   ╰── Cond [?]
            │       ├── Constant [1]
            │       ├── Constant [2]
            │       ╰── Constant [3]
            ╰── Return
                ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_binary_condition() {
    let src = r#"
        int main(void) {
            if (1 + 2 == 3)
                return 5;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── If
                ├── Binary [==]
                │   ├── Binary [+]
                │   │   ├── Constant [1]
                │   │   ╰── Constant [2]
                │   ╰── Constant [3]
                ╰── Return
                    ╰── Constant [5]
            ╰── If
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_binary_false_condition() {
    let src = r#"
        int main(void) {
            if (1 + 2 == 4)
                return 5;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── If
                ├── Binary [==]
                │   ├── Binary [+]
                │   │   ├── Constant [1]
                │   │   ╰── Constant [2]
                │   ╰── Constant [4]
                ╰── Return
                    ╰── Constant [5]
            ╰── If
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_else() {
    let src = r#"
        int main(void) {
            int a = 0;
            if (a)
                return 1;
            else
                return 2;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [0]
            ╰── If
                ├── Var [a]
                ├── Return
                │   ╰── Constant [1]
                ╰── Return
                    ╰── Constant [2]
            ╰── If
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_extra_credit_bitwise_ternary() {
    let src = r#"
        int main(void) {
            int result;
            1 ^ 1 ? result = 4 : (result = 5);
            return result;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [result]
            ├── Cond [?]
            │   ├── Binary [^]
            │   │   ├── Constant [1]
            │   │   ╰── Constant [1]
            │   ├── Assign [=]
            │   │   ├── Var [result]
            │   │   ╰── Constant [4]
            │   ╰── Assign [=]
            │       ├── Var [result]
            │       ╰── Constant [5]
            ╰── Return
                ╰── Var [result]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_extra_credit_compound_assign_ternary() {
    let src = r#"
        int main(void) {
            int a = 4;
            a *= 1 ? 2 : 3;
            return a;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [4]
            ├── Assign [*=]
            │   ├── Var [a]
            │   ╰── Cond [?]
            │       ├── Constant [1]
            │       ├── Constant [2]
            │       ╰── Constant [3]
            ╰── Return
                ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_extra_credit_compound_if_expression() {
    let src = r#"
        int main(void) {
            int a = 0;
            if (a += 1)
                return a;
            return 10;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [0]
            ├── If
            │   ├── Assign [+=]
            │   │   ├── Var [a]
            │   │   ╰── Constant [1]
            │   ╰── Return
            │       ╰── Var [a]
            ├── If
            ╰── Return
                ╰── Constant [10]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_extra_credit_goto_after_declaration() {
    let src = r#"
        int main(void) {
            int x = 1;
            goto post_declaration;
            int i = (x = 0);
        post_declaration:
            i = 5;
            return (x == 1 && i == 5);
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [x]
            │   ╰── Constant [1]
            ├── Goto [post_declaration]
            ├── Declaration [i]
            │   ╰── Assign [=]
            │       ├── Var [x]
            │       ╰── Constant [0]
            ├── Label [post_declaration]
            │   ╰── Assign [=]
            │       ├── Var [i]
            │       ╰── Constant [5]
            ╰── Return
                ╰── Binary [&&]
                    ├── Binary [==]
                    │   ├── Var [x]
                    │   ╰── Constant [1]
                    ╰── Binary [==]
                        ├── Var [i]
                        ╰── Constant [5]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_extra_credit_goto_backwards() {
    let src = r#"
        int main(void) {
            if (0)
            label:
                return 5;
            goto label;
            return 0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── If
            │   ├── Constant [0]
            │   ╰── Label [label]
            │       ╰── Return
            │           ╰── Constant [5]
            ├── If
            ├── Goto [label]
            ╰── Return
                ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_extra_credit_goto_label() {
    let src = r#"
        int main(void) {
            goto label;
            return 0;
        label:
            return 1;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Goto [label]
            ├── Return
            │   ╰── Constant [0]
            ╰── Label [label]
                ╰── Return
                    ╰── Constant [1]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_extra_credit_goto_label_and_var() {
    let src = r#"
        int main(void) {
            int ident = 5;
            goto ident;
            return 0;
        ident:
            return ident;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [ident]
            │   ╰── Constant [5]
            ├── Goto [ident]
            ├── Return
            │   ╰── Constant [0]
            ╰── Label [ident]
                ╰── Return
                    ╰── Var [ident]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_extra_credit_goto_label_main() {
    let src = r#"
        int main(void) {
            goto main;
            return 5;
        main:
            return 0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Goto [main]
            ├── Return
            │   ╰── Constant [5]
            ╰── Label [main]
                ╰── Return
                    ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_extra_credit_goto_label_main_2() {
    let src = r#"
        int main(void) {
            goto _main;
            return 0;
            _main:
                return 1;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Goto [_main]
            ├── Return
            │   ╰── Constant [0]
            ╰── Label [_main]
                ╰── Return
                    ╰── Constant [1]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_extra_credit_goto_nested_label() {
    let src = r#"
        int main(void) {
            goto labelB;
            labelA:
                labelB:
                    return 5;
            return 0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Goto [labelB]
            ├── Label [labelA]
            │   ╰── Label [labelB]
            │       ╰── Return
            │           ╰── Constant [5]
            ╰── Return
                ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_extra_credit_label_all_statements() {
    let src = r#"
        int main(void) {
            int a = 1;
        label_if:
            if (a)
                goto label_expression;
            else
                goto label_empty;
        label_goto:
            goto label_return;
            if (0)
            label_expression:
                a = 0;
            goto label_if;
        label_return:
            return a;
        label_empty:;
            a = 100;
            goto label_goto;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [1]
            ├── Label [label_if]
            │   ╰── If
            │       ├── Var [a]
            │       ├── Goto [label_expression]
            │       ╰── Goto [label_empty]
            │   ╰── If
            ├── Label [label_goto]
            │   ╰── Goto [label_return]
            ├── If
            │   ├── Constant [0]
            │   ╰── Label [label_expression]
            │       ╰── Assign [=]
            │           ├── Var [a]
            │           ╰── Constant [0]
            ├── If
            ├── Goto [label_if]
            ├── Label [label_return]
            │   ╰── Return
            │       ╰── Var [a]
            ├── Label [label_empty]
            ├── Assign [=]
            │   ├── Var [a]
            │   ╰── Constant [100]
            ╰── Goto [label_goto]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_extra_credit_label_token() {
    let src = r#"
        int main(void) {
            goto _foo_1_;
            return 0;
        _foo_1_:
            return 1;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Goto [_foo_1_]
            ├── Return
            │   ╰── Constant [0]
            ╰── Label [_foo_1_]
                ╰── Return
                    ╰── Constant [1]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_extra_credit_lh_compound_assignment() {
    let src = r#"
        int main(void) {
            int x = 10;
            (x -= 1) ? (x /= 2) : 0;
            return x == 4;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [x]
            │   ╰── Constant [10]
            ├── Cond [?]
            │   ├── Assign [-=]
            │   │   ├── Var [x]
            │   │   ╰── Constant [1]
            │   ├── Assign [/=]
            │   │   ├── Var [x]
            │   │   ╰── Constant [2]
            │   ╰── Constant [0]
            ╰── Return
                ╰── Binary [==]
                    ├── Var [x]
                    ╰── Constant [4]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_extra_credit_postfix_if() {
    let src = r#"
        int main(void) {
            int a = 0;
            if (a--)
                return 0;
            else if (a--)
                return 1;
            return 0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [0]
            ├── If
            │   ├── Postfix [--]
            │   │   ╰── Var [a]
            │   ├── Return
            │   │   ╰── Constant [0]
            │   ╰── If
            │       ├── Postfix [--]
            │       │   ╰── Var [a]
            │       ╰── Return
            │           ╰── Constant [1]
            │   ╰── If
            ├── If
            ╰── Return
                ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_extra_credit_postfix_in_ternary() {
    let src = r#"
        int main(void) {
            int x = 10;
            x - 10 ? 0 : x--;
            return x;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [x]
            │   ╰── Constant [10]
            ├── Cond [?]
            │   ├── Binary [-]
            │   │   ├── Var [x]
            │   │   ╰── Constant [10]
            │   ├── Constant [0]
            │   ╰── Postfix [--]
            │       ╰── Var [x]
            ╰── Return
                ╰── Var [x]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_extra_credit_prefix_if() {
    let src = r#"
        int main(void) {
            int a = -1;
            if (++a)
                return 0;
            else if (++a)
                return 1;
            return 0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Unary [-]
            │       ╰── Constant [1]
            ├── If
            │   ├── Unary [++]
            │   │   ╰── Var [a]
            │   ├── Return
            │   │   ╰── Constant [0]
            │   ╰── If
            │       ├── Unary [++]
            │       │   ╰── Var [a]
            │       ╰── Return
            │           ╰── Constant [1]
            │   ╰── If
            ├── If
            ╰── Return
                ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_extra_credit_prefix_in_ternary() {
    let src = r#"
        int main(void) {
            int a = 0;
            return (++a ? ++a : 0);
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [0]
            ╰── Return
                ╰── Cond [?]
                    ├── Unary [++]
                    │   ╰── Var [a]
                    ├── Unary [++]
                    │   ╰── Var [a]
                    ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_extra_credit_unused_label() {
    let src = r#"
        int main(void) {
        unused:
            return 0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Label [unused]
                ╰── Return
                    ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_extra_credit_whitespace_after_label() {
    let src = r#"
        int main(void) {
            goto label2;
            return 0;
            label1 :
            label2
            :
            return 1;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Goto [label2]
            ├── Return
            │   ╰── Constant [0]
            ╰── Label [label1]
                ╰── Label [label2]
                    ╰── Return
                        ╰── Constant [1]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_if_nested() {
    let src = r#"
        int main(void) {
            int a = 1;
            int b = 0;
            if (a)
                b = 1;
            else if (b)
                b = 2;
            return b;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [1]
            ├── Declaration [b]
            │   ╰── Constant [0]
            ├── If
            │   ├── Var [a]
            │   ├── Assign [=]
            │   │   ├── Var [b]
            │   │   ╰── Constant [1]
            │   ╰── If
            │       ├── Var [b]
            │       ╰── Assign [=]
            │           ├── Var [b]
            │           ╰── Constant [2]
            │   ╰── If
            ├── If
            ╰── Return
                ╰── Var [b]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_if_nested_2() {
    let src = r#"
        int main(void) {
            int a = 0;
            int b = 1;
            if (a)
                b = 1;
            else if (~b)
                b = 2;
            return b;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [0]
            ├── Declaration [b]
            │   ╰── Constant [1]
            ├── If
            │   ├── Var [a]
            │   ├── Assign [=]
            │   │   ├── Var [b]
            │   │   ╰── Constant [1]
            │   ╰── If
            │       ├── Unary [~]
            │       │   ╰── Var [b]
            │       ╰── Assign [=]
            │           ├── Var [b]
            │           ╰── Constant [2]
            │   ╰── If
            ├── If
            ╰── Return
                ╰── Var [b]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_if_nested_3() {
    let src = r#"
        int main(void) {
            int a = 0;
            if ( (a = 1) )
                if (a == 1)
                    a = 3;
                else
                    a = 4;
            return a;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [0]
            ├── If
            │   ├── Assign [=]
            │   │   ├── Var [a]
            │   │   ╰── Constant [1]
            │   ╰── If
            │       ├── Binary [==]
            │       │   ├── Var [a]
            │       │   ╰── Constant [1]
            │       ├── Assign [=]
            │       │   ├── Var [a]
            │       │   ╰── Constant [3]
            │       ╰── Assign [=]
            │           ├── Var [a]
            │           ╰── Constant [4]
            │   ╰── If
            ├── If
            ╰── Return
                ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_if_nested_4() {
    let src = r#"
        int main(void) {
            int a = 0;
            if (!a)
                if (3 / 4)
                    a = 3;
                else
                    a = 8 / 2;
            return a;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [0]
            ├── If
            │   ├── Unary [!]
            │   │   ╰── Var [a]
            │   ╰── If
            │       ├── Binary [/]
            │       │   ├── Constant [3]
            │       │   ╰── Constant [4]
            │       ├── Assign [=]
            │       │   ├── Var [a]
            │       │   ╰── Constant [3]
            │       ╰── Assign [=]
            │           ├── Var [a]
            │           ╰── Binary [/]
            │               ├── Constant [8]
            │               ╰── Constant [2]
            │   ╰── If
            ├── If
            ╰── Return
                ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_if_nested_5() {
    let src = r#"
        int main(void) {
            int a = 0;
            if (0)
                if (0)
                    a = 3;
                else
                    a = 4;
            else
                a = 1;
            return a;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [0]
            ├── If
            │   ├── Constant [0]
            │   ├── If
            │   │   ├── Constant [0]
            │   │   ├── Assign [=]
            │   │   │   ├── Var [a]
            │   │   │   ╰── Constant [3]
            │   │   ╰── Assign [=]
            │   │       ├── Var [a]
            │   │       ╰── Constant [4]
            │   ├── If
            │   ╰── Assign [=]
            │       ├── Var [a]
            │       ╰── Constant [1]
            ├── If
            ╰── Return
                ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_if_not_taken() {
    let src = r#"
        int main(void) {
            int a = 0;
            int b = 0;
            if (a)
                b = 1;
            return b;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [0]
            ├── Declaration [b]
            │   ╰── Constant [0]
            ├── If
            │   ├── Var [a]
            │   ╰── Assign [=]
            │       ├── Var [b]
            │       ╰── Constant [1]
            ├── If
            ╰── Return
                ╰── Var [b]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_if_null_body() {
    let src = r#"
        int main(void) {
            int x = 0;
            if (0)
                ;
            else
                x = 1;
            return x;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [x]
            │   ╰── Constant [0]
            ├── If
            │   ├── Constant [0]
            │   ╰── Assign [=]
            │       ├── Var [x]
            │       ╰── Constant [1]
            ├── If
            ╰── Return
                ╰── Var [x]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_if_taken() {
    let src = r#"
        int main(void) {
            int a = 1;
            int b = 0;
            if (a)
                b = 1;
            return b;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [1]
            ├── Declaration [b]
            │   ╰── Constant [0]
            ├── If
            │   ├── Var [a]
            │   ╰── Assign [=]
            │       ├── Var [b]
            │       ╰── Constant [1]
            ├── If
            ╰── Return
                ╰── Var [b]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_lh_assignment() {
    let src = r#"
        int main(void) {
            int x = 10;
            int y = 0;
            y = (x = 5) ? x : 2;
            return (x == 5 && y == 5);
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [x]
            │   ╰── Constant [10]
            ├── Declaration [y]
            │   ╰── Constant [0]
            ├── Assign [=]
            │   ├── Var [y]
            │   ╰── Cond [?]
            │       ├── Assign [=]
            │       │   ├── Var [x]
            │       │   ╰── Constant [5]
            │       ├── Var [x]
            │       ╰── Constant [2]
            ╰── Return
                ╰── Binary [&&]
                    ├── Binary [==]
                    │   ├── Var [x]
                    │   ╰── Constant [5]
                    ╰── Binary [==]
                        ├── Var [y]
                        ╰── Constant [5]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_multiple_if() {
    let src = r#"
        int main(void) {
            int a = 0;
            int b = 0;
            if (a)
                a = 2;
            else
                a = 3;
            if (b)
                b = 4;
            else
                b = 5;
            return a + b;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [0]
            ├── Declaration [b]
            │   ╰── Constant [0]
            ├── If
            │   ├── Var [a]
            │   ├── Assign [=]
            │   │   ├── Var [a]
            │   │   ╰── Constant [2]
            │   ╰── Assign [=]
            │       ├── Var [a]
            │       ╰── Constant [3]
            ├── If
            ├── If
            │   ├── Var [b]
            │   ├── Assign [=]
            │   │   ├── Var [b]
            │   │   ╰── Constant [4]
            │   ╰── Assign [=]
            │       ├── Var [b]
            │       ╰── Constant [5]
            ├── If
            ╰── Return
                ╰── Binary [+]
                    ├── Var [a]
                    ╰── Var [b]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_nested_ternary() {
    let src = r#"
        int main(void) {
            int a = 1;
            int b = 2;
            int flag = 0;
            return a > b ? 5 : flag ? 6 : 7;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [1]
            ├── Declaration [b]
            │   ╰── Constant [2]
            ├── Declaration [flag]
            │   ╰── Constant [0]
            ╰── Return
                ╰── Cond [?]
                    ├── Binary [>]
                    │   ├── Var [a]
                    │   ╰── Var [b]
                    ├── Constant [5]
                    ╰── Cond [?]
                        ├── Var [flag]
                        ├── Constant [6]
                        ╰── Constant [7]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_nested_ternary_2() {
    let src = r#"
        int main(void) {
            int a = 1 ? 2 ? 3 : 4 : 5;
            int b = 0 ? 2 ? 3 : 4 : 5;
            return a * b;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Cond [?]
            │       ├── Constant [1]
            │       ├── Cond [?]
            │       │   ├── Constant [2]
            │       │   ├── Constant [3]
            │       │   ╰── Constant [4]
            │       ╰── Constant [5]
            ├── Declaration [b]
            │   ╰── Cond [?]
            │       ├── Constant [0]
            │       ├── Cond [?]
            │       │   ├── Constant [2]
            │       │   ├── Constant [3]
            │       │   ╰── Constant [4]
            │       ╰── Constant [5]
            ╰── Return
                ╰── Binary [*]
                    ├── Var [a]
                    ╰── Var [b]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_rh_assignment() {
    let src = r#"
        int main(void) {
            int flag = 1;
            int a = 0;
            flag ? a = 1 : (a = 0);
            return a;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [flag]
            │   ╰── Constant [1]
            ├── Declaration [a]
            │   ╰── Constant [0]
            ├── Cond [?]
            │   ├── Var [flag]
            │   ├── Assign [=]
            │   │   ├── Var [a]
            │   │   ╰── Constant [1]
            │   ╰── Assign [=]
            │       ├── Var [a]
            │       ╰── Constant [0]
            ╰── Return
                ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_ternary() {
    let src = r#"
        int main(void) {
            int a = 0;
            return a > -1 ? 4 : 5;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [0]
            ╰── Return
                ╰── Cond [?]
                    ├── Binary [>]
                    │   ├── Var [a]
                    │   ╰── Unary [-]
                    │       ╰── Constant [1]
                    ├── Constant [4]
                    ╰── Constant [5]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_ternary_middle_assignment() {
    let src = r#"
        int main(void) {
            int a = 1;
            a != 2 ? a = 2 : 0;
            return a;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [1]
            ├── Cond [?]
            │   ├── Binary [!=]
            │   │   ├── Var [a]
            │   │   ╰── Constant [2]
            │   ├── Assign [=]
            │   │   ├── Var [a]
            │   │   ╰── Constant [2]
            │   ╰── Constant [0]
            ╰── Return
                ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_ternary_middle_binop() {
    let src = r#"
        int main(void) {
            int a = 1 ? 3 % 2 : 4;
            return a;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Cond [?]
            │       ├── Constant [1]
            │       ├── Binary [%]
            │       │   ├── Constant [3]
            │       │   ╰── Constant [2]
            │       ╰── Constant [4]
            ╰── Return
                ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_ternary_precedence() {
    let src = r#"
        int main(void) {
            int a = 10;
            return a || 0 ? 20 : 0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [10]
            ╰── Return
                ╰── Cond [?]
                    ├── Binary [||]
                    │   ├── Var [a]
                    │   ╰── Constant [0]
                    ├── Constant [20]
                    ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_ternary_rh_binop() {
    let src = r#"
        int main(void) {
            return 0 ? 1 : 0 || 2;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Return
                ╰── Cond [?]
                    ├── Constant [0]
                    ├── Constant [1]
                    ╰── Binary [||]
                        ├── Constant [0]
                        ╰── Constant [2]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_ternary_short_circuit() {
    let src = r#"
        int main(void) {
            int a = 1;
            int b = 0;
            a ? (b = 1) : (b = 2);
            return b;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [1]
            ├── Declaration [b]
            │   ╰── Constant [0]
            ├── Cond [?]
            │   ├── Var [a]
            │   ├── Assign [=]
            │   │   ├── Var [b]
            │   │   ╰── Constant [1]
            │   ╰── Assign [=]
            │       ├── Var [b]
            │       ╰── Constant [2]
            ╰── Return
                ╰── Var [b]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_ternary_short_circuit_2() {
    let src = r#"
        int main(void) {
            int a = 0;
            int b = 0;
            a ? (b = 1) : (b = 2);
            return b;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [0]
            ├── Declaration [b]
            │   ╰── Constant [0]
            ├── Cond [?]
            │   ├── Var [a]
            │   ├── Assign [=]
            │   │   ├── Var [b]
            │   │   ╰── Constant [1]
            │   ╰── Assign [=]
            │       ├── Var [b]
            │       ╰── Constant [2]
            ╰── Return
                ╰── Var [b]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}
