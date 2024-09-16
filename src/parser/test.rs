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
      //^^^ Expected end of file, but found 'foo'
    "#,
    );
}

#[test]
fn test_chapter_1_invalid_parse_invalid_function_name() {
    assert_error(
        r#"
        
        int 3 (void) {
          //^ Expected identifier, but found '3'
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
                 //^ Expected ';', but found '0'
        }
    "#,
    );
}

#[test]
fn test_chapter_1_invalid_parse_missing_type() {
    assert_error(
        r#"
        main(void) {
      //^^^^ Expected 'int', but found 'main'
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
                  //^ Expected ';', but found '0'
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
      //^ Expected ';', but found '}'
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
                //^ Expected ';', but found 'n'
        }
    "#,
    );
}

#[test]
fn test_chapter_1_invalid_parse_switched_parens() {
    assert_error(
        r#"
        int main )( {
               //^ Expected '(', but found ')'
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
                //^ Expected 'void', but found '{'
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
                    //^ Expected ';', but found ')'
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
      //^ Expected ';', but found '}'
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
                   //^ Expected ')', but found ';'
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
                       //^ Expected ')', but found ';'
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
                   //^ Expected ';', but found '('
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
                       //^ Expected ')', but found ';'
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
                      //^ Expected ';', but found ')'
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
      //^ Expected ';', but found '}'
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
      //^ Expected ';', but found '}'
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
      //^ Expected ';', but found '}'
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
              //^^^^^^ Expected identifier, but found 'return'
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
                      //^ Expected ';', but found '1'
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
                      //^ Expected ';', but found '1'
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
                //^^ Expected ';', but found '+='
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
               //^^ Expected ';', but found '++'
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
                  //^^^ Expected ';', but found 'bar'
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
               //^ Expected ';', but found 'a'
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
              //^^ Expected identifier, but found '10'
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
                   //^ Expected ';', but found '!'
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
          //^ Expected ';', but found 'a'
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
              //^^ Expected identifier, but found '10'
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
            ╰── Empty
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
            ├── Empty
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
              //^ Expected identifier, but found ';'
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
                    //^ Expected ';', but found ':'
        }
    "#,
    );
}

#[test]
fn test_chapter_6_invalid_parse_extra_credit_label_outside_function() {
    assert_error(
        r#"
        label:
      //^^^^^ Expected 'int', but found 'label'
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
              //^ Expected identifier, but found '('
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
             //^ Expected '(', but found '0'
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
                      //^ Expected ':', but found ';'
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
                           //^ Expected ';', but found ':'
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
                              //^ Expected ':', but found ';'
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
                          //^ Expected ':', but found ';'
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
            ├── Label [label_goto]
            │   ╰── Goto [label_return]
            ├── If
            │   ├── Constant [0]
            │   ╰── Label [label_expression]
            │       ╰── Assign [=]
            │           ├── Var [a]
            │           ╰── Constant [0]
            ├── Goto [label_if]
            ├── Label [label_return]
            │   ╰── Return
            │       ╰── Var [a]
            ├── Label [label_empty]
            │   ╰── Empty
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
            │   ╰── Assign [=]
            │       ├── Var [a]
            │       ╰── Constant [1]
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
            │   ├── Empty
            │   ╰── Assign [=]
            │       ├── Var [x]
            │       ╰── Constant [1]
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
            │   ├── Var [b]
            │   ├── Assign [=]
            │   │   ├── Var [b]
            │   │   ╰── Constant [4]
            │   ╰── Assign [=]
            │       ├── Var [b]
            │       ╰── Constant [5]
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

#[test]
fn test_chapter_7_invalid_parse_extra_brace() {
    assert_error(
        r#"
        int main(void) {
            if(0){
                return 1;
            }}
            return 2;
          //^^^^^^ Expected end of file, but found 'return'
        }
    "#,
    );
}

#[test]
fn test_chapter_7_invalid_parse_missing_brace() {
    assert_error(
        r#"
        int main(void) {
            if(0){
                return 1;
            return 2;
        }
    
// Expected statement, but found ''"#,
    );
}

#[test]
fn test_chapter_7_invalid_parse_missing_semicolon() {
    assert_error(
        r#"
        int main(void) {
            int a = 4;
            {
                a = 5;
                return a
            }
          //^ Expected ';', but found '}'
        }
    "#,
    );
}

#[test]
fn test_chapter_7_invalid_parse_ternary_blocks() {
    assert_error(
        r#"
        int main(void) {
            int a;
            return 1 ? { a = 2 } : a = 4;
                     //^ Expected expression, but found '{'
        }
    "#,
    );
}

#[test]
fn test_chapter_7_invalid_semantics_double_define() {
    let src = r#"
        int main(void) {
            {
                int a;
                int a;
            }
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Block
                ├── Declaration [a]
                ╰── Declaration [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_7_invalid_semantics_double_define_after_scope() {
    let src = r#"
        int main(void) {
            int a = 3;
            {
                a = 5;
            }
            int a = 2;
            return a;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [3]
            ├── Block
            │   ╰── Assign [=]
            │       ├── Var [a]
            │       ╰── Constant [5]
            ├── Declaration [a]
            │   ╰── Constant [2]
            ╰── Return
                ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_7_invalid_semantics_extra_credit_different_labels_same_scope() {
    let src = r#"
        int main(void) {
        label1:;
            int a = 10;
        label2:;
            int a = 11;
            return 1;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Label [label1]
            │   ╰── Empty
            ├── Declaration [a]
            │   ╰── Constant [10]
            ├── Label [label2]
            │   ╰── Empty
            ├── Declaration [a]
            │   ╰── Constant [11]
            ╰── Return
                ╰── Constant [1]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_7_invalid_semantics_extra_credit_duplicate_labels_different_scopes() {
    let src = r#"
        int main(void) {
            int x = 0;
            if (x) {
                x = 5;
                goto l;
                return 0;
                l:
                    return x;
            } else {
                goto l;
                return 0;
                l:
                    return x;
            }
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [x]
            │   ╰── Constant [0]
            ╰── If
                ├── Var [x]
                ├── Block
                │   ├── Assign [=]
                │   │   ├── Var [x]
                │   │   ╰── Constant [5]
                │   ├── Goto [l]
                │   ├── Return
                │   │   ╰── Constant [0]
                │   ╰── Label [l]
                │       ╰── Return
                │           ╰── Var [x]
                ╰── Block
                    ├── Goto [l]
                    ├── Return
                    │   ╰── Constant [0]
                    ╰── Label [l]
                        ╰── Return
                            ╰── Var [x]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_7_invalid_semantics_extra_credit_goto_use_before_declare() {
    let src = r#"
        int main(void) {
            int x = 0;
            if (x != 0) {
                return_y:
                return y;
            }
            int y = 4;
            goto return_y;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [x]
            │   ╰── Constant [0]
            ├── If
            │   ├── Binary [!=]
            │   │   ├── Var [x]
            │   │   ╰── Constant [0]
            │   ╰── Block
            │       ╰── Label [return_y]
            │           ╰── Return
            │               ╰── Var [y]
            ├── Declaration [y]
            │   ╰── Constant [4]
            ╰── Goto [return_y]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_7_invalid_semantics_out_of_scope() {
    let src = r#"
        int main(void) {
            {
                int a = 2;
            }
            return a;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Block
            │   ╰── Declaration [a]
            │       ╰── Constant [2]
            ╰── Return
                ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_7_invalid_semantics_use_before_declare() {
    let src = r#"
        int main(void) {
            int a;
            {
                b = 10;
            }
            int b;
            return b;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            ├── Block
            │   ╰── Assign [=]
            │       ├── Var [b]
            │       ╰── Constant [10]
            ├── Declaration [b]
            ╰── Return
                ╰── Var [b]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_7_valid_assign_to_self() {
    let src = r#"
        int main(void) {
            int a = 3;
            {
                int a = a = 4;
                return a;
            }
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [3]
            ╰── Block
                ├── Declaration [a]
                │   ╰── Assign [=]
                │       ├── Var [a]
                │       ╰── Constant [4]
                ╰── Return
                    ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_7_valid_assign_to_self_2() {
    let src = r#"
        int main(void) {
            int a = 3;
            {
                int a = a = 4;
            }
            return a;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [3]
            ├── Block
            │   ╰── Declaration [a]
            │       ╰── Assign [=]
            │           ├── Var [a]
            │           ╰── Constant [4]
            ╰── Return
                ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_7_valid_declaration_only() {
    let src = r#"
        int main(void) {
            int a;
            {
                int b = a = 1;
            }
            return a;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            ├── Block
            │   ╰── Declaration [b]
            │       ╰── Assign [=]
            │           ├── Var [a]
            │           ╰── Constant [1]
            ╰── Return
                ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_7_valid_empty_blocks() {
    let src = r#"
        int main(void) {
            int ten = 10;
            {}
            int twenty = 10 * 2;
            {{}}
            return ten + twenty;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [ten]
            │   ╰── Constant [10]
            ├── Block
            ├── Declaration [twenty]
            │   ╰── Binary [*]
            │       ├── Constant [10]
            │       ╰── Constant [2]
            ├── Block
            │   ╰── Block
            ╰── Return
                ╰── Binary [+]
                    ├── Var [ten]
                    ╰── Var [twenty]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_7_valid_extra_credit_compound_subtract_in_block() {
    let src = r#"
        int main(void) {
            int a = 5;
            if (a > 4) {
                a -= 4;
                int a = 5;
                if (a > 4) {
                    a -= 4;
                }
            }
            return a;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [5]
            ├── If
            │   ├── Binary [>]
            │   │   ├── Var [a]
            │   │   ╰── Constant [4]
            │   ╰── Block
            │       ├── Assign [-=]
            │       │   ├── Var [a]
            │       │   ╰── Constant [4]
            │       ├── Declaration [a]
            │       │   ╰── Constant [5]
            │       ╰── If
            │           ├── Binary [>]
            │           │   ├── Var [a]
            │           │   ╰── Constant [4]
            │           ╰── Block
            │               ╰── Assign [-=]
            │                   ├── Var [a]
            │                   ╰── Constant [4]
            ╰── Return
                ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_7_valid_extra_credit_goto_before_declaration() {
    let src = r#"
        int main(void) {
            int a = 0;
            {
                if (a != 0)
                    return_a:
                        return a;
                int a = 4;
                goto return_a;
            }
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [0]
            ╰── Block
                ├── If
                │   ├── Binary [!=]
                │   │   ├── Var [a]
                │   │   ╰── Constant [0]
                │   ╰── Label [return_a]
                │       ╰── Return
                │           ╰── Var [a]
                ├── Declaration [a]
                │   ╰── Constant [4]
                ╰── Goto [return_a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_7_valid_extra_credit_goto_inner_scope() {
    let src = r#"
        int main(void) {
            int x = 5;
            goto inner;
            {
                int x = 0;
                inner:
                x = 1;
                return x;
            }
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [x]
            │   ╰── Constant [5]
            ├── Goto [inner]
            ╰── Block
                ├── Declaration [x]
                │   ╰── Constant [0]
                ├── Label [inner]
                │   ╰── Assign [=]
                │       ├── Var [x]
                │       ╰── Constant [1]
                ╰── Return
                    ╰── Var [x]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_7_valid_extra_credit_goto_outer_scope() {
    let src = r#"
        int main(void) {
            int a = 10;
            int b = 0;
            if (a) {
                int a = 1;
                b = a;
                goto end;
            }
            a = 9;
        end:
            return (a == 10 && b == 1);
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [10]
            ├── Declaration [b]
            │   ╰── Constant [0]
            ├── If
            │   ├── Var [a]
            │   ╰── Block
            │       ├── Declaration [a]
            │       │   ╰── Constant [1]
            │       ├── Assign [=]
            │       │   ├── Var [b]
            │       │   ╰── Var [a]
            │       ╰── Goto [end]
            ├── Assign [=]
            │   ├── Var [a]
            │   ╰── Constant [9]
            ╰── Label [end]
                ╰── Return
                    ╰── Binary [&&]
                        ├── Binary [==]
                        │   ├── Var [a]
                        │   ╰── Constant [10]
                        ╰── Binary [==]
                            ├── Var [b]
                            ╰── Constant [1]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_7_valid_extra_credit_goto_sibling_scope() {
    let src = r#"
        int main(void) {
            int sum = 0;
            if (1) {
                int a = 5;
                goto other_if;
                sum = 0;
            first_if:
                a = 5;
                sum = sum + a;
            }
            if (0) {
            other_if:;
                int a = 6;
                sum = sum + a;
                goto first_if;
                sum = 0;
            }
            return sum;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [sum]
            │   ╰── Constant [0]
            ├── If
            │   ├── Constant [1]
            │   ╰── Block
            │       ├── Declaration [a]
            │       │   ╰── Constant [5]
            │       ├── Goto [other_if]
            │       ├── Assign [=]
            │       │   ├── Var [sum]
            │       │   ╰── Constant [0]
            │       ├── Label [first_if]
            │       │   ╰── Assign [=]
            │       │       ├── Var [a]
            │       │       ╰── Constant [5]
            │       ╰── Assign [=]
            │           ├── Var [sum]
            │           ╰── Binary [+]
            │               ├── Var [sum]
            │               ╰── Var [a]
            ├── If
            │   ├── Constant [0]
            │   ╰── Block
            │       ├── Label [other_if]
            │       │   ╰── Empty
            │       ├── Declaration [a]
            │       │   ╰── Constant [6]
            │       ├── Assign [=]
            │       │   ├── Var [sum]
            │       │   ╰── Binary [+]
            │       │       ├── Var [sum]
            │       │       ╰── Var [a]
            │       ├── Goto [first_if]
            │       ╰── Assign [=]
            │           ├── Var [sum]
            │           ╰── Constant [0]
            ╰── Return
                ╰── Var [sum]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_7_valid_hidden_then_visible() {
    let src = r#"
        int main(void) {
            int a = 2;
            int b;
            {
                a = -4;
                int a = 7;
                b = a + 1;
            }
            return b == 8 && a == -4;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [2]
            ├── Declaration [b]
            ├── Block
            │   ├── Assign [=]
            │   │   ├── Var [a]
            │   │   ╰── Unary [-]
            │   │       ╰── Constant [4]
            │   ├── Declaration [a]
            │   │   ╰── Constant [7]
            │   ╰── Assign [=]
            │       ├── Var [b]
            │       ╰── Binary [+]
            │           ├── Var [a]
            │           ╰── Constant [1]
            ╰── Return
                ╰── Binary [&&]
                    ├── Binary [==]
                    │   ├── Var [b]
                    │   ╰── Constant [8]
                    ╰── Binary [==]
                        ├── Var [a]
                        ╰── Unary [-]
                            ╰── Constant [4]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_7_valid_hidden_variable() {
    let src = r#"
        int main(void) {
            int a = 2;
            {
                int a = 1;
                return a;
            }
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [2]
            ╰── Block
                ├── Declaration [a]
                │   ╰── Constant [1]
                ╰── Return
                    ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_7_valid_inner_uninitialized() {
    let src = r#"
        int main(void) {
            int x = 4;
            {
                int x;
            }
            return x;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [x]
            │   ╰── Constant [4]
            ├── Block
            │   ╰── Declaration [x]
            ╰── Return
                ╰── Var [x]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_7_valid_multiple_vars_same_name() {
    let src = r#"
        int main(void) {
            int a = 0;
            {
                int b = 4;
                a = b;
            }
            {
                int b = 2;
                a = a - b;
            }
            return a;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [0]
            ├── Block
            │   ├── Declaration [b]
            │   │   ╰── Constant [4]
            │   ╰── Assign [=]
            │       ├── Var [a]
            │       ╰── Var [b]
            ├── Block
            │   ├── Declaration [b]
            │   │   ╰── Constant [2]
            │   ╰── Assign [=]
            │       ├── Var [a]
            │       ╰── Binary [-]
            │           ├── Var [a]
            │           ╰── Var [b]
            ╰── Return
                ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_7_valid_nested_if() {
    let src = r#"
        int main(void) {
            int a = 0;
            if (a) {
                int b = 2;
                return b;
            } else {
                int c = 3;
                if (a < c) {
                    return !a;
                } else {
                    return 5;
                }
            }
            return a;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [0]
            ├── If
            │   ├── Var [a]
            │   ├── Block
            │   │   ├── Declaration [b]
            │   │   │   ╰── Constant [2]
            │   │   ╰── Return
            │   │       ╰── Var [b]
            │   ╰── Block
            │       ├── Declaration [c]
            │       │   ╰── Constant [3]
            │       ╰── If
            │           ├── Binary [<]
            │           │   ├── Var [a]
            │           │   ╰── Var [c]
            │           ├── Block
            │           │   ╰── Return
            │           │       ╰── Unary [!]
            │           │           ╰── Var [a]
            │           ╰── Block
            │               ╰── Return
            │                   ╰── Constant [5]
            ╰── Return
                ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_7_valid_similar_var_names() {
    let src = r#"
        int main(void) {
            int a;
            int result;
            int a1 = 1;
            {
                int a = 2;
                int a1 = 2;
                {
                    int a;
                    {
                        int a;
                        {
                            int a;
                            {
                                int a;
                                {
                                    int a;
                                    {
                                        int a;
                                        {
                                            int a;
                                            {
                                                int a;
                                                {
                                                    int a = 20;
                                                    result = a;
                                                    {
                                                        int a;
                                                        a = 5;
                                                        result = result + a;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                result = result + a1;
            }
            return result + a1;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            ├── Declaration [result]
            ├── Declaration [a1]
            │   ╰── Constant [1]
            ├── Block
            │   ├── Declaration [a]
            │   │   ╰── Constant [2]
            │   ├── Declaration [a1]
            │   │   ╰── Constant [2]
            │   ├── Block
            │   │   ├── Declaration [a]
            │   │   ╰── Block
            │   │       ├── Declaration [a]
            │   │       ╰── Block
            │   │           ├── Declaration [a]
            │   │           ╰── Block
            │   │               ├── Declaration [a]
            │   │               ╰── Block
            │   │                   ├── Declaration [a]
            │   │                   ╰── Block
            │   │                       ├── Declaration [a]
            │   │                       ╰── Block
            │   │                           ├── Declaration [a]
            │   │                           ╰── Block
            │   │                               ├── Declaration [a]
            │   │                               ╰── Block
            │   │                                   ├── Declaration [a]
            │   │                                   │   ╰── Constant [20]
            │   │                                   ├── Assign [=]
            │   │                                   │   ├── Var [result]
            │   │                                   │   ╰── Var [a]
            │   │                                   ╰── Block
            │   │                                       ├── Declaration [a]
            │   │                                       ├── Assign [=]
            │   │                                       │   ├── Var [a]
            │   │                                       │   ╰── Constant [5]
            │   │                                       ╰── Assign [=]
            │   │                                           ├── Var [result]
            │   │                                           ╰── Binary [+]
            │   │                                               ├── Var [result]
            │   │                                               ╰── Var [a]
            │   ╰── Assign [=]
            │       ├── Var [result]
            │       ╰── Binary [+]
            │           ├── Var [result]
            │           ╰── Var [a1]
            ╰── Return
                ╰── Binary [+]
                    ├── Var [result]
                    ╰── Var [a1]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_7_valid_use_in_inner_scope() {
    let src = r#"
        int main(void)
        {
            int x;
            {
                x = 3;
            }
            {
                return x;
            }
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [x]
            ├── Block
            │   ╰── Assign [=]
            │       ├── Var [x]
            │       ╰── Constant [3]
            ╰── Block
                ╰── Return
                    ╰── Var [x]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_invalid_parse_decl_as_loop_body() {
    assert_error(
        r#"
        int main(void) {
            while (1)
                int i = 0;
              //^^^ Expected statement, but found 'int'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_8_invalid_parse_do_extra_semicolon() {
    assert_error(
        r#"
        int main(void) {
            do {
                int a;
            }; while(1);
           //^ Expected 'while', but found ';'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_8_invalid_parse_do_missing_semicolon() {
    assert_error(
        r#"
        int main(void) {
            do {
                4;
            } while(1)
            return 0;
          //^^^^^^ Expected ';', but found 'return'
        }
    "#,
    );
}

#[test]
fn test_chapter_8_invalid_parse_do_while_empty_parens() {
    assert_error(
        r#"
        int main(void) {
            do
                1;
            while ();
                 //^ Expected expression, but found ')'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_8_invalid_parse_extra_credit_compound_assignment_invalid_decl() {
    assert_error(
        r#"
        int main(void) {
            for (int i += 1; i < 10; i += 1) {
                     //^^ Expected ';', but found '+='
                return 0;
            }
        }
    "#,
    );
}

#[test]
fn test_chapter_8_invalid_parse_extra_credit_label_in_loop_header() {
    assert_error(
        r#"
        int main(void) {
            for (int i = 0; label: i < 10; i = i + 1) {
                               //^ Expected ';', but found ':'
                ;
            }
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_8_invalid_parse_extra_credit_label_is_not_block() {
    assert_error(
        r#"
        int main(void) {
            int a = 0;
            int b = 0;
            do
            do_body:
                a = a + 1;
                b = b - 1;
              //^ Expected 'while', but found 'b'
            while (a < 10)
                ;
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_8_invalid_parse_extra_credit_switch_case_declaration() {
    assert_error(
        r#"
        int main(void) {
            switch(3) {
                case 3:
                    int i = 0;
                  //^^^ Expected statement, but found 'int'
                    return i;
            }
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_8_invalid_parse_extra_credit_switch_goto_case() {
    assert_error(
        r#"
        int main(void) {
            goto 3;
               //^ Expected identifier, but found '3'
            switch (3) {
                case 3: return 0;
            }
        }
    "#,
    );
}

#[test]
fn test_chapter_8_invalid_parse_extra_credit_switch_missing_case_value() {
    assert_error(
        r#"
        int main(void) {
            switch(0) {
                case: return 0;
                  //^ Expected expression, but found ':'
            }
        }
    "#,
    );
}

#[test]
fn test_chapter_8_invalid_parse_extra_credit_switch_missing_paren() {
    assert_error(
        r#"
        int main(void) {
            switch 3 {
                 //^ Expected '(', but found '3'
                case 3: return 0;
            }
        }
    "#,
    );
}

#[test]
fn test_chapter_8_invalid_parse_extra_credit_switch_no_condition() {
    assert_error(
        r#"
        int main(void) {
            switch {
                 //^ Expected '(', but found '{'
                return 0;
            }
        }
    "#,
    );
}

#[test]
fn test_chapter_8_invalid_parse_extra_for_header_clause() {
    assert_error(
        r#"
        int main(void) {
            for (int i = 0; i < 10; i = i + 1; )
                                           //^ Expected ')', but found ';'
                ;
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_8_invalid_parse_invalid_for_declaration() {
    assert_error(
        r#"
        int main(void) {
            for (; int i = 0; i = i + 1)
                 //^^^ Expected expression, but found 'int'
                ;
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_8_invalid_parse_missing_for_header_clause() {
    assert_error(
        r#"
        int main(void) {
            for (int i = 0;)
                         //^ Expected expression, but found ')'
                ;
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_8_invalid_parse_paren_mismatch() {
    assert_error(
        r#"
        int main(void) {
            for (int i = 2; ))
                          //^ Expected expression, but found ')'
                int a = 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_8_invalid_parse_statement_in_condition() {
    assert_error(
        r#"
        int main(void) {
            while(int a) {
                //^^^ Expected expression, but found 'int'
                2;
            }
        }
    "#,
    );
}

#[test]
fn test_chapter_8_invalid_parse_while_missing_paren() {
    assert_error(
        r#"
        int main(void) {
            while 1 {
                //^ Expected '(', but found '1'
                return 0;
            }
        }
    "#,
    );
}

#[test]
fn test_chapter_8_invalid_semantics_break_not_in_loop() {
    let src = r#"
        int main(void) {
            if (1)
                break;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── If
                ├── Constant [1]
                ╰── Break
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_invalid_semantics_continue_not_in_loop() {
    let src = r#"
        int main(void) {
            {
                int a;
                continue;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Block
            │   ├── Declaration [a]
            │   ╰── Continue
            ╰── Return
                ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_invalid_semantics_extra_credit_case_continue() {
    let src = r#"
        int main(void) {
            int a = 3;
            switch(a + 1) {
                case 0:
                    continue;
                default: a = 1;
            }
            return a;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [3]
            ├── Switch
            │   ├── Binary [+]
            │   │   ├── Var [a]
            │   │   ╰── Constant [1]
            │   ╰── Block
            │       ├── Case
            │       │   ├── Constant [0]
            │       │   ╰── Continue
            │       ╰── Default
            │           ╰── Assign [=]
            │               ├── Var [a]
            │               ╰── Constant [1]
            ╰── Return
                ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_invalid_semantics_extra_credit_case_outside_switch() {
    let src = r#"
        int main(void) {
            for (int i = 0; i < 10; i = i + 1) {
                case 0: return 1;
            }
            return 9;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── For
            │   ├── Declaration [i]
            │   │   ╰── Constant [0]
            │   ├── Binary [<]
            │   │   ├── Var [i]
            │   │   ╰── Constant [10]
            │   ├── Assign [=]
            │   │   ├── Var [i]
            │   │   ╰── Binary [+]
            │   │       ├── Var [i]
            │   │       ╰── Constant [1]
            │   ├── Block
            │   │   ╰── Case
            │   │       ├── Constant [0]
            │   │       ╰── Return
            │   │           ╰── Constant [1]
            ╰── Return
                ╰── Constant [9]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_invalid_semantics_extra_credit_default_continue() {
    let src = r#"
        int main(void) {
            int a = 3;
            switch(a + 1) {
                case 0:
                    a = 1;
                default: continue;
            }
            return a;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [3]
            ├── Switch
            │   ├── Binary [+]
            │   │   ├── Var [a]
            │   │   ╰── Constant [1]
            │   ╰── Block
            │       ├── Case
            │       │   ├── Constant [0]
            │       │   ╰── Assign [=]
            │       │       ├── Var [a]
            │       │       ╰── Constant [1]
            │       ╰── Default
            │           ╰── Continue
            ╰── Return
                ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_invalid_semantics_extra_credit_default_outside_switch() {
    let src = r#"
        int main(void) {
            {
                default: return 0;
            }
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Block
                ╰── Default
                    ╰── Return
                        ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_invalid_semantics_extra_credit_different_cases_same_scope() {
    let src = r#"
        int main(void) {
            int a = 1;
            switch (a) {
                case 1:;
                    int b = 10;
                    break;
                case 2:;
                    int b = 11;
                    break;
                default:
                    break;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [1]
            ├── Switch
            │   ├── Var [a]
            │   ╰── Block
            │       ├── Case
            │       │   ├── Constant [1]
            │       │   ╰── Empty
            │       ├── Declaration [b]
            │       │   ╰── Constant [10]
            │       ├── Break
            │       ├── Case
            │       │   ├── Constant [2]
            │       │   ╰── Empty
            │       ├── Declaration [b]
            │       │   ╰── Constant [11]
            │       ├── Break
            │       ╰── Default
            │           ╰── Break
            ╰── Return
                ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_invalid_semantics_extra_credit_duplicate_case() {
    let src = r#"
        int main(void) {
            switch(4) {
                case 5: return 0;
                case 4: return 1;
                case 5: return 0;
                default: return 2;
            }
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Switch
                ├── Constant [4]
                ╰── Block
                    ├── Case
                    │   ├── Constant [5]
                    │   ╰── Return
                    │       ╰── Constant [0]
                    ├── Case
                    │   ├── Constant [4]
                    │   ╰── Return
                    │       ╰── Constant [1]
                    ├── Case
                    │   ├── Constant [5]
                    │   ╰── Return
                    │       ╰── Constant [0]
                    ╰── Default
                        ╰── Return
                            ╰── Constant [2]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_invalid_semantics_extra_credit_duplicate_case_in_labeled_switch() {
    let src = r#"
        int main(void) {
            int a = 0;
        label:
            switch (a) {
                case 1:
                case 1:
                    break;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [0]
            ├── Label [label]
            │   ╰── Switch
            │       ├── Var [a]
            │       ╰── Block
            │           ╰── Case
            │               ├── Constant [1]
            │               ╰── Case
            │                   ├── Constant [1]
            │                   ╰── Break
            ╰── Return
                ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_invalid_semantics_extra_credit_duplicate_case_in_nested_statement() {
    let src = r#"
        
        int main(void) {
            int a = 10;
            switch (a) {
                case 1: {
                    if(1) {
                        case 1:
                        return 0;
                    }
                }
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [10]
            ├── Switch
            │   ├── Var [a]
            │   ╰── Block
            │       ╰── Case
            │           ├── Constant [1]
            │           ╰── Block
            │               ╰── If
            │                   ├── Constant [1]
            │                   ╰── Block
            │                       ╰── Case
            │                           ├── Constant [1]
            │                           ╰── Return
            │                               ╰── Constant [0]
            ╰── Return
                ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_invalid_semantics_extra_credit_duplicate_default() {
    let src = r#"
        int main(void) {
            int a = 0;
            switch(a) {
                case 0: return 0;
                default: return 1;
                case 2: return 2;
                default: return 2;
            }
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [0]
            ╰── Switch
                ├── Var [a]
                ╰── Block
                    ├── Case
                    │   ├── Constant [0]
                    │   ╰── Return
                    │       ╰── Constant [0]
                    ├── Default
                    │   ╰── Return
                    │       ╰── Constant [1]
                    ├── Case
                    │   ├── Constant [2]
                    │   ╰── Return
                    │       ╰── Constant [2]
                    ╰── Default
                        ╰── Return
                            ╰── Constant [2]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_invalid_semantics_extra_credit_duplicate_default_in_nested_statement() {
    let src = r#"
        
        int main(void) {
            int a = 10;
            switch (a) {
                case 1:
                for (int i = 0; i < 10; i = i + 1) {
                    continue;
                    while(1)
                    default:;
                }
                case 2:
                return 0;
                default:;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [10]
            ├── Switch
            │   ├── Var [a]
            │   ╰── Block
            │       ├── Case
            │       │   ├── Constant [1]
            │       │   ╰── For
            │       │       ├── Declaration [i]
            │       │       │   ╰── Constant [0]
            │       │       ├── Binary [<]
            │       │       │   ├── Var [i]
            │       │       │   ╰── Constant [10]
            │       │       ├── Assign [=]
            │       │       │   ├── Var [i]
            │       │       │   ╰── Binary [+]
            │       │       │       ├── Var [i]
            │       │       │       ╰── Constant [1]
            │       │       ├── Block
            │       │       │   ├── Continue
            │       │       │   ╰── While
            │       │       │       ├── Constant [1]
            │       │       │       ╰── Default
            │       │       │           ╰── Empty
            │       ├── Case
            │       │   ├── Constant [2]
            │       │   ╰── Return
            │       │       ╰── Constant [0]
            │       ╰── Default
            │           ╰── Empty
            ╰── Return
                ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_invalid_semantics_extra_credit_duplicate_label_in_default() {
    let src = r#"
        int main(void) {
                int a = 1;
        label:
            switch (a) {
                case 1:
                    return 0;
                default:
                label:
                    return 1;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [1]
            ├── Label [label]
            │   ╰── Switch
            │       ├── Var [a]
            │       ╰── Block
            │           ├── Case
            │           │   ├── Constant [1]
            │           │   ╰── Return
            │           │       ╰── Constant [0]
            │           ╰── Default
            │               ╰── Label [label]
            │                   ╰── Return
            │                       ╰── Constant [1]
            ╰── Return
                ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_invalid_semantics_extra_credit_duplicate_label_in_loop() {
    let src = r#"
        int main(void) {
            do {
            lbl:
                return 1;
            lbl:
                return 2;
            } while (1);
            return 0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── DoWhile
            │   ├── Block
            │   │   ├── Label [lbl]
            │   │   │   ╰── Return
            │   │   │       ╰── Constant [1]
            │   │   ╰── Label [lbl]
            │   │       ╰── Return
            │   │           ╰── Constant [2]
            │   ╰── Constant [1]
            ╰── Return
                ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_invalid_semantics_extra_credit_duplicate_variable_in_switch() {
    let src = r#"
        int main(void) {
            int a = 1;
            switch (a) {
                int b = 2;
                case 0:
                    a = 3;
                    int b = 2;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [1]
            ├── Switch
            │   ├── Var [a]
            │   ╰── Block
            │       ├── Declaration [b]
            │       │   ╰── Constant [2]
            │       ├── Case
            │       │   ├── Constant [0]
            │       │   ╰── Assign [=]
            │       │       ├── Var [a]
            │       │       ╰── Constant [3]
            │       ╰── Declaration [b]
            │           ╰── Constant [2]
            ╰── Return
                ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_invalid_semantics_extra_credit_labeled_break_outside_loop() {
    let src = r#"
        int main(void) {
            label: break;
            return 0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Label [label]
            │   ╰── Break
            ╰── Return
                ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_invalid_semantics_extra_credit_non_constant_case() {
    let src = r#"
        int main(void) {
            int a = 3;
            switch(a + 1) {
                case 0: return 0;
                case a: return 1;
                case 1: return 2;
            }
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [3]
            ╰── Switch
                ├── Binary [+]
                │   ├── Var [a]
                │   ╰── Constant [1]
                ╰── Block
                    ├── Case
                    │   ├── Constant [0]
                    │   ╰── Return
                    │       ╰── Constant [0]
                    ├── Case
                    │   ├── Var [a]
                    │   ╰── Return
                    │       ╰── Constant [1]
                    ╰── Case
                        ├── Constant [1]
                        ╰── Return
                            ╰── Constant [2]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_invalid_semantics_extra_credit_switch_continue() {
    let src = r#"
        int main(void) {
            int a = 3;
            switch(a + 1) {
                case 0:
                    a = 4;
                    continue;
                default: a = 1;
            }
            return a;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [3]
            ├── Switch
            │   ├── Binary [+]
            │   │   ├── Var [a]
            │   │   ╰── Constant [1]
            │   ╰── Block
            │       ├── Case
            │       │   ├── Constant [0]
            │       │   ╰── Assign [=]
            │       │       ├── Var [a]
            │       │       ╰── Constant [4]
            │       ├── Continue
            │       ╰── Default
            │           ╰── Assign [=]
            │               ├── Var [a]
            │               ╰── Constant [1]
            ╰── Return
                ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_invalid_semantics_extra_credit_undeclared_var_switch_expression() {
    let src = r#"
        int main(void) {
            switch(a) {
                case 1: return 0;
                case 2: return 1;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Switch
            │   ├── Var [a]
            │   ╰── Block
            │       ├── Case
            │       │   ├── Constant [1]
            │       │   ╰── Return
            │       │       ╰── Constant [0]
            │       ╰── Case
            │           ├── Constant [2]
            │           ╰── Return
            │               ╰── Constant [1]
            ╰── Return
                ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_invalid_semantics_extra_credit_undeclared_variable_in_case() {
    let src = r#"
        int main(void) {
            int a = 10;
            switch (a) {
                case 1:
                    return b;
                    break;
                default:
                    break;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [10]
            ├── Switch
            │   ├── Var [a]
            │   ╰── Block
            │       ├── Case
            │       │   ├── Constant [1]
            │       │   ╰── Return
            │       │       ╰── Var [b]
            │       ├── Break
            │       ╰── Default
            │           ╰── Break
            ╰── Return
                ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_invalid_semantics_extra_credit_undeclared_variable_in_default() {
    let src = r#"
        int main(void) {
            int a = 10;
            switch (a) {
                case 1:
                    break;
                default:
                    return b;
                    break;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [10]
            ├── Switch
            │   ├── Var [a]
            │   ╰── Block
            │       ├── Case
            │       │   ├── Constant [1]
            │       │   ╰── Break
            │       ├── Default
            │       │   ╰── Return
            │       │       ╰── Var [b]
            │       ╰── Break
            ╰── Return
                ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_invalid_semantics_extra_credit_undefined_label_in_case() {
    let src = r#"
        
        int main(void) {
            int a = 3;
            switch (a) {
                case 1: goto foo;
                default: return 0;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [3]
            ├── Switch
            │   ├── Var [a]
            │   ╰── Block
            │       ├── Case
            │       │   ├── Constant [1]
            │       │   ╰── Goto [foo]
            │       ╰── Default
            │           ╰── Return
            │               ╰── Constant [0]
            ╰── Return
                ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_invalid_semantics_out_of_scope_do_loop() {
    let src = r#"
        int main(void) {
            do {
                int a = a + 1;
            } while (a < 100);
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── DoWhile
                ├── Block
                │   ╰── Declaration [a]
                │       ╰── Binary [+]
                │           ├── Var [a]
                │           ╰── Constant [1]
                ╰── Binary [<]
                    ├── Var [a]
                    ╰── Constant [100]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_invalid_semantics_out_of_scope_loop_variable() {
    let src = r#"
        int main(void)
        {
            for (i = 0; i < 1; i = i + 1)
            {
                return 0;
            }
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── For
                ├── Assign [=]
                │   ├── Var [i]
                │   ╰── Constant [0]
                ├── Binary [<]
                │   ├── Var [i]
                │   ╰── Constant [1]
                ├── Assign [=]
                │   ├── Var [i]
                │   ╰── Binary [+]
                │       ├── Var [i]
                │       ╰── Constant [1]
                ├── Block
                │   ╰── Return
                │       ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_break() {
    let src = r#"
        int main(void) {
            int a = 10;
            int b = 20;
            for (b = -20; b < 0; b = b + 1) {
                a = a - 1;
                if (a <= 0)
                    break;
            }
            return a == 0 && b == -11;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [10]
            ├── Declaration [b]
            │   ╰── Constant [20]
            ├── For
            │   ├── Assign [=]
            │   │   ├── Var [b]
            │   │   ╰── Unary [-]
            │   │       ╰── Constant [20]
            │   ├── Binary [<]
            │   │   ├── Var [b]
            │   │   ╰── Constant [0]
            │   ├── Assign [=]
            │   │   ├── Var [b]
            │   │   ╰── Binary [+]
            │   │       ├── Var [b]
            │   │       ╰── Constant [1]
            │   ├── Block
            │   │   ├── Assign [=]
            │   │   │   ├── Var [a]
            │   │   │   ╰── Binary [-]
            │   │   │       ├── Var [a]
            │   │   │       ╰── Constant [1]
            │   │   ╰── If
            │   │       ├── Binary [<=]
            │   │       │   ├── Var [a]
            │   │       │   ╰── Constant [0]
            │   │       ╰── Break
            ╰── Return
                ╰── Binary [&&]
                    ├── Binary [==]
                    │   ├── Var [a]
                    │   ╰── Constant [0]
                    ╰── Binary [==]
                        ├── Var [b]
                        ╰── Unary [-]
                            ╰── Constant [11]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_break_immediate() {
    let src = r#"
        int main(void) {
            int a = 10;
            while ((a = 1))
                break;
            return a;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [10]
            ├── While
            │   ├── Assign [=]
            │   │   ├── Var [a]
            │   │   ╰── Constant [1]
            │   ╰── Break
            ╰── Return
                ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_continue() {
    let src = r#"
        int main(void) {
            int sum = 0;
            int counter;
            for (int i = 0; i <= 10; i = i + 1) {
                counter = i;
                if (i % 2 == 0)
                    continue;
                sum = sum + 1;
            }
            return sum == 5 && counter == 10;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [sum]
            │   ╰── Constant [0]
            ├── Declaration [counter]
            ├── For
            │   ├── Declaration [i]
            │   │   ╰── Constant [0]
            │   ├── Binary [<=]
            │   │   ├── Var [i]
            │   │   ╰── Constant [10]
            │   ├── Assign [=]
            │   │   ├── Var [i]
            │   │   ╰── Binary [+]
            │   │       ├── Var [i]
            │   │       ╰── Constant [1]
            │   ├── Block
            │   │   ├── Assign [=]
            │   │   │   ├── Var [counter]
            │   │   │   ╰── Var [i]
            │   │   ├── If
            │   │   │   ├── Binary [==]
            │   │   │   │   ├── Binary [%]
            │   │   │   │   │   ├── Var [i]
            │   │   │   │   │   ╰── Constant [2]
            │   │   │   │   ╰── Constant [0]
            │   │   │   ╰── Continue
            │   │   ╰── Assign [=]
            │   │       ├── Var [sum]
            │   │       ╰── Binary [+]
            │   │           ├── Var [sum]
            │   │           ╰── Constant [1]
            ╰── Return
                ╰── Binary [&&]
                    ├── Binary [==]
                    │   ├── Var [sum]
                    │   ╰── Constant [5]
                    ╰── Binary [==]
                        ├── Var [counter]
                        ╰── Constant [10]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_continue_empty_post() {
    let src = r#"
        int main(void) {
            int sum = 0;
            for (int i = 0; i < 10;) {
                i = i + 1;
                if (i % 2)
                    continue;
                sum = sum + i;
            }
            return sum;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [sum]
            │   ╰── Constant [0]
            ├── For
            │   ├── Declaration [i]
            │   │   ╰── Constant [0]
            │   ├── Binary [<]
            │   │   ├── Var [i]
            │   │   ╰── Constant [10]
            ├── ├── Empty
            │   ├── Block
            │   │   ├── Assign [=]
            │   │   │   ├── Var [i]
            │   │   │   ╰── Binary [+]
            │   │   │       ├── Var [i]
            │   │   │       ╰── Constant [1]
            │   │   ├── If
            │   │   │   ├── Binary [%]
            │   │   │   │   ├── Var [i]
            │   │   │   │   ╰── Constant [2]
            │   │   │   ╰── Continue
            │   │   ╰── Assign [=]
            │   │       ├── Var [sum]
            │   │       ╰── Binary [+]
            │   │           ├── Var [sum]
            │   │           ╰── Var [i]
            ╰── Return
                ╰── Var [sum]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_do_while() {
    let src = r#"
        int main(void) {
            int a = 1;
            do {
                a = a * 2;
            } while(a < 11);
            return a;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [1]
            ├── DoWhile
            │   ├── Block
            │   │   ╰── Assign [=]
            │   │       ├── Var [a]
            │   │       ╰── Binary [*]
            │   │           ├── Var [a]
            │   │           ╰── Constant [2]
            │   ╰── Binary [<]
            │       ├── Var [a]
            │       ╰── Constant [11]
            ╰── Return
                ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_do_while_break_immediate() {
    let src = r#"
        int main(void) {
            int a = 10;
            do
                break;
            while ((a = 1));
            return a;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [10]
            ├── DoWhile
            │   ├── Break
            │   ╰── Assign [=]
            │       ├── Var [a]
            │       ╰── Constant [1]
            ╰── Return
                ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_empty_expression() {
    let src = r#"
        int main(void) {
            return 0;;;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Return
            │   ╰── Constant [0]
            ├── Empty
            ╰── Empty
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_empty_loop_body() {
    let src = r#"
        int main(void) {
            int i = 2147483642;
            do ; while ((i = i - 5) >= 256);
            return i;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [i]
            │   ╰── Constant [2147483642]
            ├── DoWhile
            │   ├── Empty
            │   ╰── Binary [>=]
            │       ├── Assign [=]
            │       │   ├── Var [i]
            │       │   ╰── Binary [-]
            │       │       ├── Var [i]
            │       │       ╰── Constant [5]
            │       ╰── Constant [256]
            ╰── Return
                ╰── Var [i]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_extra_credit_case_block() {
    let src = r#"
        int main(void) {
            int a = 4;
            int b = 0;
            switch(2) {
                case 2: {
                    int a = 8;
                    b = a;
                }
            }
            return (a == 4 && b == 8);
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [4]
            ├── Declaration [b]
            │   ╰── Constant [0]
            ├── Switch
            │   ├── Constant [2]
            │   ╰── Block
            │       ╰── Case
            │           ├── Constant [2]
            │           ╰── Block
            │               ├── Declaration [a]
            │               │   ╰── Constant [8]
            │               ╰── Assign [=]
            │                   ├── Var [b]
            │                   ╰── Var [a]
            ╰── Return
                ╰── Binary [&&]
                    ├── Binary [==]
                    │   ├── Var [a]
                    │   ╰── Constant [4]
                    ╰── Binary [==]
                        ├── Var [b]
                        ╰── Constant [8]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_extra_credit_compound_assignment_controlling_expression() {
    let src = r#"
        int main(void) {
            int i = 100;
            int sum = 0;
            do sum += 2;
            while (i -= 1);
            return (i == 0 && sum == 200);
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [i]
            │   ╰── Constant [100]
            ├── Declaration [sum]
            │   ╰── Constant [0]
            ├── DoWhile
            │   ├── Assign [+=]
            │   │   ├── Var [sum]
            │   │   ╰── Constant [2]
            │   ╰── Assign [-=]
            │       ├── Var [i]
            │       ╰── Constant [1]
            ╰── Return
                ╰── Binary [&&]
                    ├── Binary [==]
                    │   ├── Var [i]
                    │   ╰── Constant [0]
                    ╰── Binary [==]
                        ├── Var [sum]
                        ╰── Constant [200]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_extra_credit_compound_assignment_for_loop() {
    let src = r#"
        int main(void) {
            int i = 1;
            for (i *= -1; i >= -100; i -=3)
                ;
            return (i == -103);
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [i]
            │   ╰── Constant [1]
            ├── For
            │   ├── Assign [*=]
            │   │   ├── Var [i]
            │   │   ╰── Unary [-]
            │   │       ╰── Constant [1]
            │   ├── Binary [>=]
            │   │   ├── Var [i]
            │   │   ╰── Unary [-]
            │   │       ╰── Constant [100]
            │   ├── Assign [-=]
            │   │   ├── Var [i]
            │   │   ╰── Constant [3]
            │   ├── Empty
            ╰── Return
                ╰── Binary [==]
                    ├── Var [i]
                    ╰── Unary [-]
                        ╰── Constant [103]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_extra_credit_duffs_device() {
    let src = r#"
        
        int main(void) {
            int count = 37;
            int iterations = (count + 4) / 5;
            switch (count % 5) {
                case 0:
                    do {
                        count = count - 1;
                        case 4:
                            count = count - 1;
                        case 3:
                            count = count - 1;
                        case 2:
                            count = count - 1;
                        case 1:
                            count = count - 1;
                    } while ((iterations = iterations - 1) > 0);
            }
            return (count == 0 && iterations == 0);
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [count]
            │   ╰── Constant [37]
            ├── Declaration [iterations]
            │   ╰── Binary [/]
            │       ├── Binary [+]
            │       │   ├── Var [count]
            │       │   ╰── Constant [4]
            │       ╰── Constant [5]
            ├── Switch
            │   ├── Binary [%]
            │   │   ├── Var [count]
            │   │   ╰── Constant [5]
            │   ╰── Block
            │       ╰── Case
            │           ├── Constant [0]
            │           ╰── DoWhile
            │               ├── Block
            │               │   ├── Assign [=]
            │               │   │   ├── Var [count]
            │               │   │   ╰── Binary [-]
            │               │   │       ├── Var [count]
            │               │   │       ╰── Constant [1]
            │               │   ├── Case
            │               │   │   ├── Constant [4]
            │               │   │   ╰── Assign [=]
            │               │   │       ├── Var [count]
            │               │   │       ╰── Binary [-]
            │               │   │           ├── Var [count]
            │               │   │           ╰── Constant [1]
            │               │   ├── Case
            │               │   │   ├── Constant [3]
            │               │   │   ╰── Assign [=]
            │               │   │       ├── Var [count]
            │               │   │       ╰── Binary [-]
            │               │   │           ├── Var [count]
            │               │   │           ╰── Constant [1]
            │               │   ├── Case
            │               │   │   ├── Constant [2]
            │               │   │   ╰── Assign [=]
            │               │   │       ├── Var [count]
            │               │   │       ╰── Binary [-]
            │               │   │           ├── Var [count]
            │               │   │           ╰── Constant [1]
            │               │   ╰── Case
            │               │       ├── Constant [1]
            │               │       ╰── Assign [=]
            │               │           ├── Var [count]
            │               │           ╰── Binary [-]
            │               │               ├── Var [count]
            │               │               ╰── Constant [1]
            │               ╰── Binary [>]
            │                   ├── Assign [=]
            │                   │   ├── Var [iterations]
            │                   │   ╰── Binary [-]
            │                   │       ├── Var [iterations]
            │                   │       ╰── Constant [1]
            │                   ╰── Constant [0]
            ╰── Return
                ╰── Binary [&&]
                    ├── Binary [==]
                    │   ├── Var [count]
                    │   ╰── Constant [0]
                    ╰── Binary [==]
                        ├── Var [iterations]
                        ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_extra_credit_goto_bypass_condition() {
    let src = r#"
        int main(void) {
            int i = 1;
            do {
            while_start:
                i = i + 1;
                if (i < 10)
                    goto while_start;
            } while (0);
            return i;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [i]
            │   ╰── Constant [1]
            ├── DoWhile
            │   ├── Block
            │   │   ├── Label [while_start]
            │   │   │   ╰── Assign [=]
            │   │   │       ├── Var [i]
            │   │   │       ╰── Binary [+]
            │   │   │           ├── Var [i]
            │   │   │           ╰── Constant [1]
            │   │   ╰── If
            │   │       ├── Binary [<]
            │   │       │   ├── Var [i]
            │   │       │   ╰── Constant [10]
            │   │       ╰── Goto [while_start]
            │   ╰── Constant [0]
            ╰── Return
                ╰── Var [i]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_extra_credit_goto_bypass_init_exp() {
    let src = r#"
        int main(void) {
            int i = 0;
            goto target;
            for (i = 5; i < 10; i = i + 1)
            target:
                if (i == 0)
                    return 1;
            return 0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [i]
            │   ╰── Constant [0]
            ├── Goto [target]
            ├── For
            │   ├── Assign [=]
            │   │   ├── Var [i]
            │   │   ╰── Constant [5]
            │   ├── Binary [<]
            │   │   ├── Var [i]
            │   │   ╰── Constant [10]
            │   ├── Assign [=]
            │   │   ├── Var [i]
            │   │   ╰── Binary [+]
            │   │       ├── Var [i]
            │   │       ╰── Constant [1]
            │   ├── Label [target]
            │   │   ╰── If
            │   │       ├── Binary [==]
            │   │       │   ├── Var [i]
            │   │       │   ╰── Constant [0]
            │   │       ╰── Return
            │   │           ╰── Constant [1]
            ╰── Return
                ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_extra_credit_goto_bypass_post_exp() {
    let src = r#"
        int main(void) {
            int sum = 0;
            for (int i = 0;; i = 0) {
            lbl:
                sum = sum + 1;
                i = i + 1;
                if (i > 10)
                    break;
                goto lbl;
            }
            return sum;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [sum]
            │   ╰── Constant [0]
            ├── For
            │   ├── Declaration [i]
            │   │   ╰── Constant [0]
            ├── ├── Empty
            │   ├── Assign [=]
            │   │   ├── Var [i]
            │   │   ╰── Constant [0]
            │   ├── Block
            │   │   ├── Label [lbl]
            │   │   │   ╰── Assign [=]
            │   │   │       ├── Var [sum]
            │   │   │       ╰── Binary [+]
            │   │   │           ├── Var [sum]
            │   │   │           ╰── Constant [1]
            │   │   ├── Assign [=]
            │   │   │   ├── Var [i]
            │   │   │   ╰── Binary [+]
            │   │   │       ├── Var [i]
            │   │   │       ╰── Constant [1]
            │   │   ├── If
            │   │   │   ├── Binary [>]
            │   │   │   │   ├── Var [i]
            │   │   │   │   ╰── Constant [10]
            │   │   │   ╰── Break
            │   │   ╰── Goto [lbl]
            ╰── Return
                ╰── Var [sum]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_extra_credit_label_loop_body() {
    let src = r#"
        
        int main(void) {
            int result = 0;
            goto label;
            while (0)
            label: { result = 1; }
            return result;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [result]
            │   ╰── Constant [0]
            ├── Goto [label]
            ├── While
            │   ├── Constant [0]
            │   ╰── Label [label]
            │       ╰── Block
            │           ╰── Assign [=]
            │               ├── Var [result]
            │               ╰── Constant [1]
            ╰── Return
                ╰── Var [result]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_extra_credit_label_loops_breaks_and_continues() {
    let src = r#"
        int main(void) {
            int sum = 0;
            goto do_label;
            return 0;
        do_label:
            do {
                sum = 1;
                goto while_label;
            } while (1);
        while_label:
            while (1) {
                sum = sum + 1;
                goto break_label;
                return 0;
            break_label:
                break;
            };
            goto for_label;
            return 0;
        for_label:
            for (int i = 0; i < 10; i = i + 1) {
                sum = sum + 1;
                goto continue_label;
                return 0;
            continue_label:
                continue;
                return 0;
            }
            return sum;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [sum]
            │   ╰── Constant [0]
            ├── Goto [do_label]
            ├── Return
            │   ╰── Constant [0]
            ├── Label [do_label]
            │   ╰── DoWhile
            │       ├── Block
            │       │   ├── Assign [=]
            │       │   │   ├── Var [sum]
            │       │   │   ╰── Constant [1]
            │       │   ╰── Goto [while_label]
            │       ╰── Constant [1]
            ├── Label [while_label]
            │   ╰── While
            │       ├── Constant [1]
            │       ╰── Block
            │           ├── Assign [=]
            │           │   ├── Var [sum]
            │           │   ╰── Binary [+]
            │           │       ├── Var [sum]
            │           │       ╰── Constant [1]
            │           ├── Goto [break_label]
            │           ├── Return
            │           │   ╰── Constant [0]
            │           ╰── Label [break_label]
            │               ╰── Break
            ├── Empty
            ├── Goto [for_label]
            ├── Return
            │   ╰── Constant [0]
            ├── Label [for_label]
            │   ╰── For
            │       ├── Declaration [i]
            │       │   ╰── Constant [0]
            │       ├── Binary [<]
            │       │   ├── Var [i]
            │       │   ╰── Constant [10]
            │       ├── Assign [=]
            │       │   ├── Var [i]
            │       │   ╰── Binary [+]
            │       │       ├── Var [i]
            │       │       ╰── Constant [1]
            │       ├── Block
            │       │   ├── Assign [=]
            │       │   │   ├── Var [sum]
            │       │   │   ╰── Binary [+]
            │       │   │       ├── Var [sum]
            │       │   │       ╰── Constant [1]
            │       │   ├── Goto [continue_label]
            │       │   ├── Return
            │       │   │   ╰── Constant [0]
            │       │   ├── Label [continue_label]
            │       │   │   ╰── Continue
            │       │   ╰── Return
            │       │       ╰── Constant [0]
            ╰── Return
                ╰── Var [sum]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_extra_credit_loop_header_postfix_and_prefix() {
    let src = r#"
        int main(void) {
            int i = 100;
            int count = 0;
            while (i--) count++;
            if (count != 100)
                return 0;
            i = 100;
            count = 0;
            while (--i) count++;
            if (count != 99)
                return 0;
            return 1;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [i]
            │   ╰── Constant [100]
            ├── Declaration [count]
            │   ╰── Constant [0]
            ├── While
            │   ├── Postfix [--]
            │   │   ╰── Var [i]
            │   ╰── Postfix [++]
            │       ╰── Var [count]
            ├── If
            │   ├── Binary [!=]
            │   │   ├── Var [count]
            │   │   ╰── Constant [100]
            │   ╰── Return
            │       ╰── Constant [0]
            ├── Assign [=]
            │   ├── Var [i]
            │   ╰── Constant [100]
            ├── Assign [=]
            │   ├── Var [count]
            │   ╰── Constant [0]
            ├── While
            │   ├── Unary [--]
            │   │   ╰── Var [i]
            │   ╰── Postfix [++]
            │       ╰── Var [count]
            ├── If
            │   ├── Binary [!=]
            │   │   ├── Var [count]
            │   │   ╰── Constant [99]
            │   ╰── Return
            │       ╰── Constant [0]
            ╰── Return
                ╰── Constant [1]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_extra_credit_loop_in_switch() {
    let src = r#"
        int main(void) {
            int cond = 10;
            switch (cond) {
                case 1:
                    return 0;
                case 10:
                    for (int i = 0; i < 5; i = i + 1) {
                        cond = cond - 1;
                        if (cond == 8)
                            break;
                    }
                    return 123;
                default:
                    return 2;
            }
            return 3;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [cond]
            │   ╰── Constant [10]
            ├── Switch
            │   ├── Var [cond]
            │   ╰── Block
            │       ├── Case
            │       │   ├── Constant [1]
            │       │   ╰── Return
            │       │       ╰── Constant [0]
            │       ├── Case
            │       │   ├── Constant [10]
            │       │   ╰── For
            │       │       ├── Declaration [i]
            │       │       │   ╰── Constant [0]
            │       │       ├── Binary [<]
            │       │       │   ├── Var [i]
            │       │       │   ╰── Constant [5]
            │       │       ├── Assign [=]
            │       │       │   ├── Var [i]
            │       │       │   ╰── Binary [+]
            │       │       │       ├── Var [i]
            │       │       │       ╰── Constant [1]
            │       │       ├── Block
            │       │       │   ├── Assign [=]
            │       │       │   │   ├── Var [cond]
            │       │       │   │   ╰── Binary [-]
            │       │       │   │       ├── Var [cond]
            │       │       │   │       ╰── Constant [1]
            │       │       │   ╰── If
            │       │       │       ├── Binary [==]
            │       │       │       │   ├── Var [cond]
            │       │       │       │   ╰── Constant [8]
            │       │       │       ╰── Break
            │       ├── Return
            │       │   ╰── Constant [123]
            │       ╰── Default
            │           ╰── Return
            │               ╰── Constant [2]
            ╰── Return
                ╰── Constant [3]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_extra_credit_post_exp_incr() {
    let src = r#"
        int main(void) {
            int product = 1;
            for (int i = 0; i < 10; i++) {
                product = product + 2;
            }
            return product;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [product]
            │   ╰── Constant [1]
            ├── For
            │   ├── Declaration [i]
            │   │   ╰── Constant [0]
            │   ├── Binary [<]
            │   │   ├── Var [i]
            │   │   ╰── Constant [10]
            │   ├── Postfix [++]
            │   │   ╰── Var [i]
            │   ├── Block
            │   │   ╰── Assign [=]
            │   │       ├── Var [product]
            │   │       ╰── Binary [+]
            │   │           ├── Var [product]
            │   │           ╰── Constant [2]
            ╰── Return
                ╰── Var [product]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_extra_credit_switch() {
    let src = r#"
        
        int main(void) {
            switch(3) {
                case 0: return 0;
                case 1: return 1;
                case 3: return 3;
                case 5: return 5;
            }
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Switch
                ├── Constant [3]
                ╰── Block
                    ├── Case
                    │   ├── Constant [0]
                    │   ╰── Return
                    │       ╰── Constant [0]
                    ├── Case
                    │   ├── Constant [1]
                    │   ╰── Return
                    │       ╰── Constant [1]
                    ├── Case
                    │   ├── Constant [3]
                    │   ╰── Return
                    │       ╰── Constant [3]
                    ╰── Case
                        ├── Constant [5]
                        ╰── Return
                            ╰── Constant [5]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_extra_credit_switch_assign_in_condition() {
    let src = r#"
        int main(void) {
            int a = 0;
            switch (a = 1) {
                case 0:
                    return 10;
                case 1:
                    a = a * 2;
                    break;
                default:
                    a = 99;
            }
            return a;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [0]
            ├── Switch
            │   ├── Assign [=]
            │   │   ├── Var [a]
            │   │   ╰── Constant [1]
            │   ╰── Block
            │       ├── Case
            │       │   ├── Constant [0]
            │       │   ╰── Return
            │       │       ╰── Constant [10]
            │       ├── Case
            │       │   ├── Constant [1]
            │       │   ╰── Assign [=]
            │       │       ├── Var [a]
            │       │       ╰── Binary [*]
            │       │           ├── Var [a]
            │       │           ╰── Constant [2]
            │       ├── Break
            │       ╰── Default
            │           ╰── Assign [=]
            │               ├── Var [a]
            │               ╰── Constant [99]
            ╰── Return
                ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_extra_credit_switch_break() {
    let src = r#"
        int main(void) {
            int a = 5;
            switch (a) {
                case 5:
                    a = 10;
                    break;
                case 6:
                    a = 0;
                    break;
            }
            return a;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [5]
            ├── Switch
            │   ├── Var [a]
            │   ╰── Block
            │       ├── Case
            │       │   ├── Constant [5]
            │       │   ╰── Assign [=]
            │       │       ├── Var [a]
            │       │       ╰── Constant [10]
            │       ├── Break
            │       ├── Case
            │       │   ├── Constant [6]
            │       │   ╰── Assign [=]
            │       │       ├── Var [a]
            │       │       ╰── Constant [0]
            │       ╰── Break
            ╰── Return
                ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_extra_credit_switch_decl() {
    let src = r#"
        int main(void) {
            int a = 3;
            int b = 0;
            switch(a) {
                int a = (b = 5);
            case 3:
                a = 4;
                b = b + a;
            }
            return a == 3 && b == 4;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [3]
            ├── Declaration [b]
            │   ╰── Constant [0]
            ├── Switch
            │   ├── Var [a]
            │   ╰── Block
            │       ├── Declaration [a]
            │       │   ╰── Assign [=]
            │       │       ├── Var [b]
            │       │       ╰── Constant [5]
            │       ├── Case
            │       │   ├── Constant [3]
            │       │   ╰── Assign [=]
            │       │       ├── Var [a]
            │       │       ╰── Constant [4]
            │       ╰── Assign [=]
            │           ├── Var [b]
            │           ╰── Binary [+]
            │               ├── Var [b]
            │               ╰── Var [a]
            ╰── Return
                ╰── Binary [&&]
                    ├── Binary [==]
                    │   ├── Var [a]
                    │   ╰── Constant [3]
                    ╰── Binary [==]
                        ├── Var [b]
                        ╰── Constant [4]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_extra_credit_switch_default() {
    let src = r#"
        int main(void) {
            int a = 0;
            switch(a) {
                case 1:
                    return 1;
                case 2:
                    return 9;
                case 4:
                    a = 11;
                    break;
                default:
                    a = 22;
            }
            return a;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [0]
            ├── Switch
            │   ├── Var [a]
            │   ╰── Block
            │       ├── Case
            │       │   ├── Constant [1]
            │       │   ╰── Return
            │       │       ╰── Constant [1]
            │       ├── Case
            │       │   ├── Constant [2]
            │       │   ╰── Return
            │       │       ╰── Constant [9]
            │       ├── Case
            │       │   ├── Constant [4]
            │       │   ╰── Assign [=]
            │       │       ├── Var [a]
            │       │       ╰── Constant [11]
            │       ├── Break
            │       ╰── Default
            │           ╰── Assign [=]
            │               ├── Var [a]
            │               ╰── Constant [22]
            ╰── Return
                ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_extra_credit_switch_default_fallthrough() {
    let src = r#"
        int main(void) {
            int a = 5;
            switch(0) {
                default:
                    a = 0;
                case 1:
                    return a;
            }
            return a + 1;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [5]
            ├── Switch
            │   ├── Constant [0]
            │   ╰── Block
            │       ├── Default
            │       │   ╰── Assign [=]
            │       │       ├── Var [a]
            │       │       ╰── Constant [0]
            │       ╰── Case
            │           ├── Constant [1]
            │           ╰── Return
            │               ╰── Var [a]
            ╰── Return
                ╰── Binary [+]
                    ├── Var [a]
                    ╰── Constant [1]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_extra_credit_switch_default_not_last() {
    let src = r#"
        int main(void) {
            int a;
            int b = a = 7;
            switch (a + b) {
                default: return 0;
                case 2: return 1;
            }
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            ├── Declaration [b]
            │   ╰── Assign [=]
            │       ├── Var [a]
            │       ╰── Constant [7]
            ╰── Switch
                ├── Binary [+]
                │   ├── Var [a]
                │   ╰── Var [b]
                ╰── Block
                    ├── Default
                    │   ╰── Return
                    │       ╰── Constant [0]
                    ╰── Case
                        ├── Constant [2]
                        ╰── Return
                            ╰── Constant [1]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_extra_credit_switch_default_only() {
    let src = r#"
        int main(void) {
            int a = 1;
            switch(a) default: return 1;
            return 0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [1]
            ├── Switch
            │   ├── Var [a]
            │   ╰── Default
            │       ╰── Return
            │           ╰── Constant [1]
            ╰── Return
                ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_extra_credit_switch_empty() {
    let src = r#"
        int main(void) {
            int x = 10;
            switch(x = x + 1) {
            }
            switch(x = x + 1)
            ;
            return x;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [x]
            │   ╰── Constant [10]
            ├── Switch
            │   ├── Assign [=]
            │   │   ├── Var [x]
            │   │   ╰── Binary [+]
            │   │       ├── Var [x]
            │   │       ╰── Constant [1]
            │   ╰── Block
            ├── Switch
            │   ├── Assign [=]
            │   │   ├── Var [x]
            │   │   ╰── Binary [+]
            │   │       ├── Var [x]
            │   │       ╰── Constant [1]
            │   ╰── Empty
            ╰── Return
                ╰── Var [x]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_extra_credit_switch_fallthrough() {
    let src = r#"
        int main(void) {
            int a = 4;
            int b = 9;
            int c = 0;
            switch (a ? b : 7) {
                case 0:
                    return 5;
                case 7:
                    c = 1;
                case 9:
                    c = 2;
                case 1:
                    c = c + 4;
            }
            return c;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [4]
            ├── Declaration [b]
            │   ╰── Constant [9]
            ├── Declaration [c]
            │   ╰── Constant [0]
            ├── Switch
            │   ├── Cond [?]
            │   │   ├── Var [a]
            │   │   ├── Var [b]
            │   │   ╰── Constant [7]
            │   ╰── Block
            │       ├── Case
            │       │   ├── Constant [0]
            │       │   ╰── Return
            │       │       ╰── Constant [5]
            │       ├── Case
            │       │   ├── Constant [7]
            │       │   ╰── Assign [=]
            │       │       ├── Var [c]
            │       │       ╰── Constant [1]
            │       ├── Case
            │       │   ├── Constant [9]
            │       │   ╰── Assign [=]
            │       │       ├── Var [c]
            │       │       ╰── Constant [2]
            │       ╰── Case
            │           ├── Constant [1]
            │           ╰── Assign [=]
            │               ├── Var [c]
            │               ╰── Binary [+]
            │                   ├── Var [c]
            │                   ╰── Constant [4]
            ╰── Return
                ╰── Var [c]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_extra_credit_switch_goto_mid_case() {
    let src = r#"
        int main(void) {
            int a = 0;
            goto mid_case;
            switch (4) {
                case 4:
                    a = 5;
                mid_case:
                    a = a + 1;
                    return a;
            }
            return 100;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [0]
            ├── Goto [mid_case]
            ├── Switch
            │   ├── Constant [4]
            │   ╰── Block
            │       ├── Case
            │       │   ├── Constant [4]
            │       │   ╰── Assign [=]
            │       │       ├── Var [a]
            │       │       ╰── Constant [5]
            │       ├── Label [mid_case]
            │       │   ╰── Assign [=]
            │       │       ├── Var [a]
            │       │       ╰── Binary [+]
            │       │           ├── Var [a]
            │       │           ╰── Constant [1]
            │       ╰── Return
            │           ╰── Var [a]
            ╰── Return
                ╰── Constant [100]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_extra_credit_switch_in_loop() {
    let src = r#"
        int main(void) {
            int acc = 0;
            int ctr = 0;
            for (int i = 0; i < 10; i = i + 1) {
                switch(i) {
                    case 0:
                        acc = 2;
                        break;
                    case 1:
                        acc = acc * 3;
                        break;
                    case 2:
                        acc = acc * 4;
                        break;
                    default:
                        acc = acc + 1;
                }
                ctr = ctr + 1;
            }
            return ctr == 10 && acc == 31;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [acc]
            │   ╰── Constant [0]
            ├── Declaration [ctr]
            │   ╰── Constant [0]
            ├── For
            │   ├── Declaration [i]
            │   │   ╰── Constant [0]
            │   ├── Binary [<]
            │   │   ├── Var [i]
            │   │   ╰── Constant [10]
            │   ├── Assign [=]
            │   │   ├── Var [i]
            │   │   ╰── Binary [+]
            │   │       ├── Var [i]
            │   │       ╰── Constant [1]
            │   ├── Block
            │   │   ├── Switch
            │   │   │   ├── Var [i]
            │   │   │   ╰── Block
            │   │   │       ├── Case
            │   │   │       │   ├── Constant [0]
            │   │   │       │   ╰── Assign [=]
            │   │   │       │       ├── Var [acc]
            │   │   │       │       ╰── Constant [2]
            │   │   │       ├── Break
            │   │   │       ├── Case
            │   │   │       │   ├── Constant [1]
            │   │   │       │   ╰── Assign [=]
            │   │   │       │       ├── Var [acc]
            │   │   │       │       ╰── Binary [*]
            │   │   │       │           ├── Var [acc]
            │   │   │       │           ╰── Constant [3]
            │   │   │       ├── Break
            │   │   │       ├── Case
            │   │   │       │   ├── Constant [2]
            │   │   │       │   ╰── Assign [=]
            │   │   │       │       ├── Var [acc]
            │   │   │       │       ╰── Binary [*]
            │   │   │       │           ├── Var [acc]
            │   │   │       │           ╰── Constant [4]
            │   │   │       ├── Break
            │   │   │       ╰── Default
            │   │   │           ╰── Assign [=]
            │   │   │               ├── Var [acc]
            │   │   │               ╰── Binary [+]
            │   │   │                   ├── Var [acc]
            │   │   │                   ╰── Constant [1]
            │   │   ╰── Assign [=]
            │   │       ├── Var [ctr]
            │   │       ╰── Binary [+]
            │   │           ├── Var [ctr]
            │   │           ╰── Constant [1]
            ╰── Return
                ╰── Binary [&&]
                    ├── Binary [==]
                    │   ├── Var [ctr]
                    │   ╰── Constant [10]
                    ╰── Binary [==]
                        ├── Var [acc]
                        ╰── Constant [31]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_extra_credit_switch_nested_cases() {
    let src = r#"
        int main(void) {
            int switch1 = 0;
            int switch2 = 0;
            int switch3 = 0;
            switch(3) {
                case 0: return 0;
                case 1: if (0) {
                    case 3: switch1 = 1; break;
                }
                default: return 0;
            }
            switch(4) {
                case 0: return 0;
                if (1) {
                    return 0;
                } else {
                    case 4: switch2 = 1; break;
                }
                default: return 0;
            }
            switch (5) {
                for (int i = 0; i < 10; i = i + 1) {
                    switch1 = 0;
                    case 5: switch3 = 1; break;
                    default: return 0;
                }
            }
            return (switch1 && switch2 && switch3);
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [switch1]
            │   ╰── Constant [0]
            ├── Declaration [switch2]
            │   ╰── Constant [0]
            ├── Declaration [switch3]
            │   ╰── Constant [0]
            ├── Switch
            │   ├── Constant [3]
            │   ╰── Block
            │       ├── Case
            │       │   ├── Constant [0]
            │       │   ╰── Return
            │       │       ╰── Constant [0]
            │       ├── Case
            │       │   ├── Constant [1]
            │       │   ╰── If
            │       │       ├── Constant [0]
            │       │       ╰── Block
            │       │           ├── Case
            │       │           │   ├── Constant [3]
            │       │           │   ╰── Assign [=]
            │       │           │       ├── Var [switch1]
            │       │           │       ╰── Constant [1]
            │       │           ╰── Break
            │       ╰── Default
            │           ╰── Return
            │               ╰── Constant [0]
            ├── Switch
            │   ├── Constant [4]
            │   ╰── Block
            │       ├── Case
            │       │   ├── Constant [0]
            │       │   ╰── Return
            │       │       ╰── Constant [0]
            │       ├── If
            │       │   ├── Constant [1]
            │       │   ├── Block
            │       │   │   ╰── Return
            │       │   │       ╰── Constant [0]
            │       │   ╰── Block
            │       │       ├── Case
            │       │       │   ├── Constant [4]
            │       │       │   ╰── Assign [=]
            │       │       │       ├── Var [switch2]
            │       │       │       ╰── Constant [1]
            │       │       ╰── Break
            │       ╰── Default
            │           ╰── Return
            │               ╰── Constant [0]
            ├── Switch
            │   ├── Constant [5]
            │   ╰── Block
            │       ╰── For
            │           ├── Declaration [i]
            │           │   ╰── Constant [0]
            │           ├── Binary [<]
            │           │   ├── Var [i]
            │           │   ╰── Constant [10]
            │           ├── Assign [=]
            │           │   ├── Var [i]
            │           │   ╰── Binary [+]
            │           │       ├── Var [i]
            │           │       ╰── Constant [1]
            │           ├── Block
            │           │   ├── Assign [=]
            │           │   │   ├── Var [switch1]
            │           │   │   ╰── Constant [0]
            │           │   ├── Case
            │           │   │   ├── Constant [5]
            │           │   │   ╰── Assign [=]
            │           │   │       ├── Var [switch3]
            │           │   │       ╰── Constant [1]
            │           │   ├── Break
            │           │   ╰── Default
            │           │       ╰── Return
            │           │           ╰── Constant [0]
            ╰── Return
                ╰── Binary [&&]
                    ├── Binary [&&]
                    │   ├── Var [switch1]
                    │   ╰── Var [switch2]
                    ╰── Var [switch3]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_extra_credit_switch_nested_not_taken() {
    let src = r#"
        
        int main(void) {
            int a = 0;
            switch(a) {
                case 1:
                    switch(a) {
                        case 0: return 0;
                        default: return 0;
                    }
                default: a = 2;
            }
            return a;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [0]
            ├── Switch
            │   ├── Var [a]
            │   ╰── Block
            │       ├── Case
            │       │   ├── Constant [1]
            │       │   ╰── Switch
            │       │       ├── Var [a]
            │       │       ╰── Block
            │       │           ├── Case
            │       │           │   ├── Constant [0]
            │       │           │   ╰── Return
            │       │           │       ╰── Constant [0]
            │       │           ╰── Default
            │       │               ╰── Return
            │       │                   ╰── Constant [0]
            │       ╰── Default
            │           ╰── Assign [=]
            │               ├── Var [a]
            │               ╰── Constant [2]
            ╰── Return
                ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_extra_credit_switch_nested_switch() {
    let src = r#"
        int main(void){
            switch(3) {
                case 0:
                    return 0;
                case 3: {
                    switch(4) {
                        case 3: return 0;
                        case 4: return 1;
                        default: return 0;
                    }
                }
                case 4: return 0;
                default: return 0;
            }
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── Switch
                ├── Constant [3]
                ╰── Block
                    ├── Case
                    │   ├── Constant [0]
                    │   ╰── Return
                    │       ╰── Constant [0]
                    ├── Case
                    │   ├── Constant [3]
                    │   ╰── Block
                    │       ╰── Switch
                    │           ├── Constant [4]
                    │           ╰── Block
                    │               ├── Case
                    │               │   ├── Constant [3]
                    │               │   ╰── Return
                    │               │       ╰── Constant [0]
                    │               ├── Case
                    │               │   ├── Constant [4]
                    │               │   ╰── Return
                    │               │       ╰── Constant [1]
                    │               ╰── Default
                    │                   ╰── Return
                    │                       ╰── Constant [0]
                    ├── Case
                    │   ├── Constant [4]
                    │   ╰── Return
                    │       ╰── Constant [0]
                    ╰── Default
                        ╰── Return
                            ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_extra_credit_switch_no_case() {
    let src = r#"
        int main(void) {
            int a = 4;
            switch(a)
                return 0;
            return a;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [4]
            ├── Switch
            │   ├── Var [a]
            │   ╰── Return
            │       ╰── Constant [0]
            ╰── Return
                ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_extra_credit_switch_not_taken() {
    let src = r#"
        int main(void) {
            int a = 1;
            switch(a) {
                case 0: return 0;
                case 2: return 0;
                case 3: return 0;
            }
            return 1;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [1]
            ├── Switch
            │   ├── Var [a]
            │   ╰── Block
            │       ├── Case
            │       │   ├── Constant [0]
            │       │   ╰── Return
            │       │       ╰── Constant [0]
            │       ├── Case
            │       │   ├── Constant [2]
            │       │   ╰── Return
            │       │       ╰── Constant [0]
            │       ╰── Case
            │           ├── Constant [3]
            │           ╰── Return
            │               ╰── Constant [0]
            ╰── Return
                ╰── Constant [1]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_extra_credit_switch_single_case() {
    let src = r#"
        int main(void) {
            int a = 1;
            switch(a) case 1: return 1;
            return 0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [1]
            ├── Switch
            │   ├── Var [a]
            │   ╰── Case
            │       ├── Constant [1]
            │       ╰── Return
            │           ╰── Constant [1]
            ╰── Return
                ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_extra_credit_switch_with_continue() {
    let src = r#"
        int main(void) {
            switch(4) {
                case 0:
                    return 0;
                case 4: {
                    int acc = 0;
                    for (int i = 0; i < 10; i = i + 1) {
                        if (i % 2)
                            continue;
                        acc = acc + 1;
                    }
                    return acc;
                }
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Switch
            │   ├── Constant [4]
            │   ╰── Block
            │       ├── Case
            │       │   ├── Constant [0]
            │       │   ╰── Return
            │       │       ╰── Constant [0]
            │       ╰── Case
            │           ├── Constant [4]
            │           ╰── Block
            │               ├── Declaration [acc]
            │               │   ╰── Constant [0]
            │               ├── For
            │               │   ├── Declaration [i]
            │               │   │   ╰── Constant [0]
            │               │   ├── Binary [<]
            │               │   │   ├── Var [i]
            │               │   │   ╰── Constant [10]
            │               │   ├── Assign [=]
            │               │   │   ├── Var [i]
            │               │   │   ╰── Binary [+]
            │               │   │       ├── Var [i]
            │               │   │       ╰── Constant [1]
            │               │   ├── Block
            │               │   │   ├── If
            │               │   │   │   ├── Binary [%]
            │               │   │   │   │   ├── Var [i]
            │               │   │   │   │   ╰── Constant [2]
            │               │   │   │   ╰── Continue
            │               │   │   ╰── Assign [=]
            │               │   │       ├── Var [acc]
            │               │   │       ╰── Binary [+]
            │               │   │           ├── Var [acc]
            │               │   │           ╰── Constant [1]
            │               ╰── Return
            │                   ╰── Var [acc]
            ╰── Return
                ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_extra_credit_switch_with_continue_2() {
    let src = r#"
        int main(void) {
            int sum = 0;
            for (int i = 0; i < 10; i = i + 1) {
                switch(i % 2) {
                    case 0: continue;
                    default: sum = sum + 1;
                }
            }
            return sum;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [sum]
            │   ╰── Constant [0]
            ├── For
            │   ├── Declaration [i]
            │   │   ╰── Constant [0]
            │   ├── Binary [<]
            │   │   ├── Var [i]
            │   │   ╰── Constant [10]
            │   ├── Assign [=]
            │   │   ├── Var [i]
            │   │   ╰── Binary [+]
            │   │       ├── Var [i]
            │   │       ╰── Constant [1]
            │   ├── Block
            │   │   ╰── Switch
            │   │       ├── Binary [%]
            │   │       │   ├── Var [i]
            │   │       │   ╰── Constant [2]
            │   │       ╰── Block
            │   │           ├── Case
            │   │           │   ├── Constant [0]
            │   │           │   ╰── Continue
            │   │           ╰── Default
            │   │               ╰── Assign [=]
            │   │                   ├── Var [sum]
            │   │                   ╰── Binary [+]
            │   │                       ├── Var [sum]
            │   │                       ╰── Constant [1]
            ╰── Return
                ╰── Var [sum]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_for() {
    let src = r#"
        int main(void) {
            int a = 12345;
            int i;
            for (i = 5; i >= 0; i = i - 1)
                a = a / 3;
            return a;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [12345]
            ├── Declaration [i]
            ├── For
            │   ├── Assign [=]
            │   │   ├── Var [i]
            │   │   ╰── Constant [5]
            │   ├── Binary [>=]
            │   │   ├── Var [i]
            │   │   ╰── Constant [0]
            │   ├── Assign [=]
            │   │   ├── Var [i]
            │   │   ╰── Binary [-]
            │   │       ├── Var [i]
            │   │       ╰── Constant [1]
            │   ├── Assign [=]
            │   │   ├── Var [a]
            │   │   ╰── Binary [/]
            │   │       ├── Var [a]
            │   │       ╰── Constant [3]
            ╰── Return
                ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_for_absent_condition() {
    let src = r#"
        int main(void) {
            for (int i = 400; ; i = i - 100)
                if (i == 100)
                    return 0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ╰── For
                ├── Declaration [i]
                │   ╰── Constant [400]
            ╰── ├── Empty
                ├── Assign [=]
                │   ├── Var [i]
                │   ╰── Binary [-]
                │       ├── Var [i]
                │       ╰── Constant [100]
                ├── If
                │   ├── Binary [==]
                │   │   ├── Var [i]
                │   │   ╰── Constant [100]
                │   ╰── Return
                │       ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_for_absent_post() {
    let src = r#"
        int main(void) {
            int a = -2147483647;
            for (; a % 5 != 0;) {
                a = a + 1;
            }
            return a % 5 || a > 0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Unary [-]
            │       ╰── Constant [2147483647]
            ├── For
            ├── ├── Empty
            │   ├── Binary [!=]
            │   │   ├── Binary [%]
            │   │   │   ├── Var [a]
            │   │   │   ╰── Constant [5]
            │   │   ╰── Constant [0]
            ├── ├── Empty
            │   ├── Block
            │   │   ╰── Assign [=]
            │   │       ├── Var [a]
            │   │       ╰── Binary [+]
            │   │           ├── Var [a]
            │   │           ╰── Constant [1]
            ╰── Return
                ╰── Binary [||]
                    ├── Binary [%]
                    │   ├── Var [a]
                    │   ╰── Constant [5]
                    ╰── Binary [>]
                        ├── Var [a]
                        ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_for_decl() {
    let src = r#"
        int main(void) {
            int a = 0;
            for (int i = -100; i <= 0; i = i + 1)
                a = a + 1;
            return a;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [0]
            ├── For
            │   ├── Declaration [i]
            │   │   ╰── Unary [-]
            │   │       ╰── Constant [100]
            │   ├── Binary [<=]
            │   │   ├── Var [i]
            │   │   ╰── Constant [0]
            │   ├── Assign [=]
            │   │   ├── Var [i]
            │   │   ╰── Binary [+]
            │   │       ├── Var [i]
            │   │       ╰── Constant [1]
            │   ├── Assign [=]
            │   │   ├── Var [a]
            │   │   ╰── Binary [+]
            │   │       ├── Var [a]
            │   │       ╰── Constant [1]
            ╰── Return
                ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_for_nested_shadow() {
    let src = r#"
        int main(void) {
            int i = 0;
            int j = 0;
            int k = 1;
            for (int i = 100; i > 0; i = i - 1) {
                int i = 1;
                int j = i + k;
                k = j;
            }
            return k == 101 && i == 0 && j == 0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [i]
            │   ╰── Constant [0]
            ├── Declaration [j]
            │   ╰── Constant [0]
            ├── Declaration [k]
            │   ╰── Constant [1]
            ├── For
            │   ├── Declaration [i]
            │   │   ╰── Constant [100]
            │   ├── Binary [>]
            │   │   ├── Var [i]
            │   │   ╰── Constant [0]
            │   ├── Assign [=]
            │   │   ├── Var [i]
            │   │   ╰── Binary [-]
            │   │       ├── Var [i]
            │   │       ╰── Constant [1]
            │   ├── Block
            │   │   ├── Declaration [i]
            │   │   │   ╰── Constant [1]
            │   │   ├── Declaration [j]
            │   │   │   ╰── Binary [+]
            │   │   │       ├── Var [i]
            │   │   │       ╰── Var [k]
            │   │   ╰── Assign [=]
            │   │       ├── Var [k]
            │   │       ╰── Var [j]
            ╰── Return
                ╰── Binary [&&]
                    ├── Binary [&&]
                    │   ├── Binary [==]
                    │   │   ├── Var [k]
                    │   │   ╰── Constant [101]
                    │   ╰── Binary [==]
                    │       ├── Var [i]
                    │       ╰── Constant [0]
                    ╰── Binary [==]
                        ├── Var [j]
                        ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_for_shadow() {
    let src = r#"
        int main(void) {
            int shadow = 1;
            int acc = 0;
            for (int shadow = 0; shadow < 10; shadow = shadow + 1) {
                acc = acc + shadow;
            }
            return acc == 45 && shadow == 1;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [shadow]
            │   ╰── Constant [1]
            ├── Declaration [acc]
            │   ╰── Constant [0]
            ├── For
            │   ├── Declaration [shadow]
            │   │   ╰── Constant [0]
            │   ├── Binary [<]
            │   │   ├── Var [shadow]
            │   │   ╰── Constant [10]
            │   ├── Assign [=]
            │   │   ├── Var [shadow]
            │   │   ╰── Binary [+]
            │   │       ├── Var [shadow]
            │   │       ╰── Constant [1]
            │   ├── Block
            │   │   ╰── Assign [=]
            │   │       ├── Var [acc]
            │   │       ╰── Binary [+]
            │   │           ├── Var [acc]
            │   │           ╰── Var [shadow]
            ╰── Return
                ╰── Binary [&&]
                    ├── Binary [==]
                    │   ├── Var [acc]
                    │   ╰── Constant [45]
                    ╰── Binary [==]
                        ├── Var [shadow]
                        ╰── Constant [1]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_multi_break() {
    let src = r#"
        int main(void) {
            int i = 0;
            while (1) {
                i = i + 1;
                if (i > 10)
                    break;
            }
            int j = 10;
            while (1) {
                j = j - 1;
                if (j < 0)
                    break;
            }
            int result = j == -1 && i == 11;
            return result;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [i]
            │   ╰── Constant [0]
            ├── While
            │   ├── Constant [1]
            │   ╰── Block
            │       ├── Assign [=]
            │       │   ├── Var [i]
            │       │   ╰── Binary [+]
            │       │       ├── Var [i]
            │       │       ╰── Constant [1]
            │       ╰── If
            │           ├── Binary [>]
            │           │   ├── Var [i]
            │           │   ╰── Constant [10]
            │           ╰── Break
            ├── Declaration [j]
            │   ╰── Constant [10]
            ├── While
            │   ├── Constant [1]
            │   ╰── Block
            │       ├── Assign [=]
            │       │   ├── Var [j]
            │       │   ╰── Binary [-]
            │       │       ├── Var [j]
            │       │       ╰── Constant [1]
            │       ╰── If
            │           ├── Binary [<]
            │           │   ├── Var [j]
            │           │   ╰── Constant [0]
            │           ╰── Break
            ├── Declaration [result]
            │   ╰── Binary [&&]
            │       ├── Binary [==]
            │       │   ├── Var [j]
            │       │   ╰── Unary [-]
            │       │       ╰── Constant [1]
            │       ╰── Binary [==]
            │           ├── Var [i]
            │           ╰── Constant [11]
            ╰── Return
                ╰── Var [result]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_multi_continue_same_loop() {
    let src = r#"
        int main(void) {
            int x = 10;
            int y = 0;
            int z = 0;
            do {
                z = z + 1;
                if (x <= 0)
                    continue;
                x = x - 1;
                if (y >= 10)
                    continue;
                y = y + 1;
            } while (z != 50);
            return z == 50 && x == 0 && y == 10;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [x]
            │   ╰── Constant [10]
            ├── Declaration [y]
            │   ╰── Constant [0]
            ├── Declaration [z]
            │   ╰── Constant [0]
            ├── DoWhile
            │   ├── Block
            │   │   ├── Assign [=]
            │   │   │   ├── Var [z]
            │   │   │   ╰── Binary [+]
            │   │   │       ├── Var [z]
            │   │   │       ╰── Constant [1]
            │   │   ├── If
            │   │   │   ├── Binary [<=]
            │   │   │   │   ├── Var [x]
            │   │   │   │   ╰── Constant [0]
            │   │   │   ╰── Continue
            │   │   ├── Assign [=]
            │   │   │   ├── Var [x]
            │   │   │   ╰── Binary [-]
            │   │   │       ├── Var [x]
            │   │   │       ╰── Constant [1]
            │   │   ├── If
            │   │   │   ├── Binary [>=]
            │   │   │   │   ├── Var [y]
            │   │   │   │   ╰── Constant [10]
            │   │   │   ╰── Continue
            │   │   ╰── Assign [=]
            │   │       ├── Var [y]
            │   │       ╰── Binary [+]
            │   │           ├── Var [y]
            │   │           ╰── Constant [1]
            │   ╰── Binary [!=]
            │       ├── Var [z]
            │       ╰── Constant [50]
            ╰── Return
                ╰── Binary [&&]
                    ├── Binary [&&]
                    │   ├── Binary [==]
                    │   │   ├── Var [z]
                    │   │   ╰── Constant [50]
                    │   ╰── Binary [==]
                    │       ├── Var [x]
                    │       ╰── Constant [0]
                    ╰── Binary [==]
                        ├── Var [y]
                        ╰── Constant [10]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_nested_break() {
    let src = r#"
        int main(void) {
            int ans = 0;
            for (int i = 0; i < 10; i = i + 1)
                for (int j = 0; j < 10; j = j + 1)
                    if ((i / 2)*2 == i)
                        break;
                    else
                        ans = ans + i;
            return ans;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [ans]
            │   ╰── Constant [0]
            ├── For
            │   ├── Declaration [i]
            │   │   ╰── Constant [0]
            │   ├── Binary [<]
            │   │   ├── Var [i]
            │   │   ╰── Constant [10]
            │   ├── Assign [=]
            │   │   ├── Var [i]
            │   │   ╰── Binary [+]
            │   │       ├── Var [i]
            │   │       ╰── Constant [1]
            │   ├── For
            │   │   ├── Declaration [j]
            │   │   │   ╰── Constant [0]
            │   │   ├── Binary [<]
            │   │   │   ├── Var [j]
            │   │   │   ╰── Constant [10]
            │   │   ├── Assign [=]
            │   │   │   ├── Var [j]
            │   │   │   ╰── Binary [+]
            │   │   │       ├── Var [j]
            │   │   │       ╰── Constant [1]
            │   │   ├── If
            │   │   │   ├── Binary [==]
            │   │   │   │   ├── Binary [*]
            │   │   │   │   │   ├── Binary [/]
            │   │   │   │   │   │   ├── Var [i]
            │   │   │   │   │   │   ╰── Constant [2]
            │   │   │   │   │   ╰── Constant [2]
            │   │   │   │   ╰── Var [i]
            │   │   │   ├── Break
            │   │   │   ╰── Assign [=]
            │   │   │       ├── Var [ans]
            │   │   │       ╰── Binary [+]
            │   │   │           ├── Var [ans]
            │   │   │           ╰── Var [i]
            ╰── Return
                ╰── Var [ans]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_nested_continue() {
    let src = r#"
        int main(void) {
            int x = 5;
            int acc = 0;
            while (x >= 0) {
                int i = x;
                while (i <= 10) {
                    i = i + 1;
                    if (i % 2)
                        continue;
                    acc = acc + 1;
                }
                x = x - 1;
            }
            return acc;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [x]
            │   ╰── Constant [5]
            ├── Declaration [acc]
            │   ╰── Constant [0]
            ├── While
            │   ├── Binary [>=]
            │   │   ├── Var [x]
            │   │   ╰── Constant [0]
            │   ╰── Block
            │       ├── Declaration [i]
            │       │   ╰── Var [x]
            │       ├── While
            │       │   ├── Binary [<=]
            │       │   │   ├── Var [i]
            │       │   │   ╰── Constant [10]
            │       │   ╰── Block
            │       │       ├── Assign [=]
            │       │       │   ├── Var [i]
            │       │       │   ╰── Binary [+]
            │       │       │       ├── Var [i]
            │       │       │       ╰── Constant [1]
            │       │       ├── If
            │       │       │   ├── Binary [%]
            │       │       │   │   ├── Var [i]
            │       │       │   │   ╰── Constant [2]
            │       │       │   ╰── Continue
            │       │       ╰── Assign [=]
            │       │           ├── Var [acc]
            │       │           ╰── Binary [+]
            │       │               ├── Var [acc]
            │       │               ╰── Constant [1]
            │       ╰── Assign [=]
            │           ├── Var [x]
            │           ╰── Binary [-]
            │               ├── Var [x]
            │               ╰── Constant [1]
            ╰── Return
                ╰── Var [acc]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_nested_loop() {
    let src = r#"
        int main(void) {
            int acc = 0;
            int x = 100;
            while (x) {
                int y = 10;
                x = x - y;
                while (y) {
                    acc = acc + 1;
                    y = y - 1;
                }
            }
            return acc == 100 && x == 0;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [acc]
            │   ╰── Constant [0]
            ├── Declaration [x]
            │   ╰── Constant [100]
            ├── While
            │   ├── Var [x]
            │   ╰── Block
            │       ├── Declaration [y]
            │       │   ╰── Constant [10]
            │       ├── Assign [=]
            │       │   ├── Var [x]
            │       │   ╰── Binary [-]
            │       │       ├── Var [x]
            │       │       ╰── Var [y]
            │       ╰── While
            │           ├── Var [y]
            │           ╰── Block
            │               ├── Assign [=]
            │               │   ├── Var [acc]
            │               │   ╰── Binary [+]
            │               │       ├── Var [acc]
            │               │       ╰── Constant [1]
            │               ╰── Assign [=]
            │                   ├── Var [y]
            │                   ╰── Binary [-]
            │                       ├── Var [y]
            │                       ╰── Constant [1]
            ╰── Return
                ╰── Binary [&&]
                    ├── Binary [==]
                    │   ├── Var [acc]
                    │   ╰── Constant [100]
                    ╰── Binary [==]
                        ├── Var [x]
                        ╰── Constant [0]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_null_for_header() {
    let src = r#"
        int main(void) {
            int a = 0;
            for (; ; ) {
                a = a + 1;
                if (a > 3)
                    break;
            }
            return a;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [0]
            ├── For
            ├── ├── Empty
            ├── ├── Empty
            ├── ├── Empty
            │   ├── Block
            │   │   ├── Assign [=]
            │   │   │   ├── Var [a]
            │   │   │   ╰── Binary [+]
            │   │   │       ├── Var [a]
            │   │   │       ╰── Constant [1]
            │   │   ╰── If
            │   │       ├── Binary [>]
            │   │       │   ├── Var [a]
            │   │       │   ╰── Constant [3]
            │   │       ╰── Break
            ╰── Return
                ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_while() {
    let src = r#"
        int main(void) {
            int a = 0;
            while (a < 5)
                a = a + 2;
            return a;
        }
    "#;
    let expected = r#"
        Program
        ╰── Function [main]
            ├── Declaration [a]
            │   ╰── Constant [0]
            ├── While
            │   ├── Binary [<]
            │   │   ├── Var [a]
            │   │   ╰── Constant [5]
            │   ╰── Assign [=]
            │       ├── Var [a]
            │       ╰── Binary [+]
            │           ├── Var [a]
            │           ╰── Constant [2]
            ╰── Return
                ╰── Var [a]
    "#;
    assert_eq!(dump_ast(src), dedent(expected));
}
