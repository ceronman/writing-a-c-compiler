use crate::lexer::tokenize;
use crate::lexer::TokenKind::*;

#[test]
#[should_panic]
fn test_chapter_1_invalid_lex_at_sign() {
    tokenize(
        r#"
        int main(void) {
            return 0@1;
        }
    "#,
    );
}

#[test]
#[should_panic]
fn test_chapter_1_invalid_lex_backslash() {
    tokenize(
        r#"
        
        \
    "#,
    );
}

#[test]
#[should_panic]
fn test_chapter_1_invalid_lex_backtick() {
    tokenize(
        r#"
        
        `
    "#,
    );
}

#[test]
#[should_panic]
fn test_chapter_1_invalid_lex_invalid_identifier() {
    tokenize(
        r#"
        
        int main(void) {
            return 1foo;
        }
    "#,
    );
}

#[test]
#[should_panic]
fn test_chapter_1_invalid_lex_invalid_identifier_2() {
    tokenize(
        r#"
        int main(void)
        {
            return @b;
        }
    "#,
    );
}

#[test]
fn test_chapter_1_invalid_parse_end_before_expr() {
    let src = r#"
        int main(void) {
            return
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_1_invalid_parse_extra_junk() {
    let src = r#"
        int main(void)
        {
            return 2;
        }
        foo
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, Semicolon,
        CloseBrace, Identifier,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_1_invalid_parse_invalid_function_name() {
    let src = r#"
        
        int 3 (void) {
            return 0;
        }
    "#;
    let expected = vec![
        Int, Constant, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_1_invalid_parse_keyword_wrong_case() {
    let src = r#"
        int main(void) {
            RETURN 0;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Identifier, Constant, Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_1_invalid_parse_missing_type() {
    let src = r#"
        main(void) {
            return 0;
        }
    "#;
    let expected = vec![
        Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_1_invalid_parse_misspelled_keyword() {
    let src = r#"
        int main(void) {
            returns 0;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Identifier, Constant, Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_1_invalid_parse_no_semicolon() {
    let src = r#"
        int main (void) {
            return 0
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_1_invalid_parse_not_expression() {
    let src = r#"
        int main(void) {
            return int;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Int, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_1_invalid_parse_space_in_keyword() {
    let src = r#"
        int main(void){
            retur n 0;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Identifier, Identifier, Constant,
        Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_1_invalid_parse_switched_parens() {
    let src = r#"
        int main )( {
            return 0;
        }
    "#;
    let expected = vec![
        Int, Identifier, CloseParen, OpenParen, OpenBrace, Return, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_1_invalid_parse_unclosed_brace() {
    let src = r#"
        int main(void) {
            return 0;
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, Semicolon,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_1_invalid_parse_unclosed_paren() {
    let src = r#"
        int main( {
            return 0;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, OpenBrace, Return, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_1_valid_multi_digit() {
    let src = r#"
        int main(void) {
            return 100;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_1_valid_no_newlines() {
    let src = r#"
        int main(void){return 0;}
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_1_valid_return_0() {
    let src = r#"
        int main(void) {
            return 0;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_1_valid_return_2() {
    let src = r#"
        int main(void) {
            return 2;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_1_valid_spaces() {
    let src = r#"
           int main ( void) { return 0 ; }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_1_valid_tabs() {
    let src = r#"
        int main ( void) { return 0 ; }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_2_invalid_parse_extra_paren() {
    let src = r#"
        int main(void)
        {
            return (3));
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, OpenParen, Constant,
        CloseParen, CloseParen, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_2_invalid_parse_missing_const() {
    let src = r#"
        int main(void) {
            return ~;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Tilde, Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_2_invalid_parse_missing_semicolon() {
    let src = r#"
        int main(void) {
            return -5
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Minus, Constant,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_2_invalid_parse_nested_missing_const() {
    let src = r#"
        int main(void)
        {
            return -~;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Minus, Tilde, Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_2_invalid_parse_parenthesize_operand() {
    let src = r#"
        int main(void) {
            return (-)3;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, OpenParen, Minus,
        CloseParen, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_2_invalid_parse_unclosed_paren() {
    let src = r#"
        int main(void)
        {
            return (1;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, OpenParen, Constant,
        Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_2_invalid_parse_wrong_order() {
    let src = r#"
        int main(void) {
            return 4-;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, Minus,
        Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_2_valid_bitwise() {
    let src = r#"
        int main(void) {
            return ~12;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Tilde, Constant,
        Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_2_valid_bitwise_int_min() {
    let src = r#"
        int main(void) {
            return ~-2147483647;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Tilde, Minus, Constant,
        Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_2_valid_bitwise_zero() {
    let src = r#"
        int main(void) {
            return ~0;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Tilde, Constant,
        Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_2_valid_neg() {
    let src = r#"
        int main(void) {
            return -5;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Minus, Constant,
        Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_2_valid_neg_zero() {
    let src = r#"
        int main(void) {
            return -0;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Minus, Constant,
        Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_2_valid_negate_int_max() {
    let src = r#"
        int main(void) {
            return -2147483647;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Minus, Constant,
        Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_2_valid_nested_ops() {
    let src = r#"
        int main(void) {
            return ~-3;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Tilde, Minus, Constant,
        Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_2_valid_nested_ops_2() {
    let src = r#"
        int main(void) {
            return -~0;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Minus, Tilde, Constant,
        Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_2_valid_parens() {
    let src = r#"
        int main(void) {
            return (-2);
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, OpenParen, Minus,
        Constant, CloseParen, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_2_valid_parens_2() {
    let src = r#"
        int main(void) {
            return ~(2);
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Tilde, OpenParen,
        Constant, CloseParen, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_2_valid_parens_3() {
    let src = r#"
        int main(void) {
            return -(-4);
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Minus, OpenParen, Minus,
        Constant, CloseParen, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_2_valid_redundant_parens() {
    let src = r#"
        int main(void)
        {
            return -((((10))));
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Minus, OpenParen,
        OpenParen, OpenParen, OpenParen, Constant, CloseParen, CloseParen, CloseParen, CloseParen,
        Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_3_invalid_parse_double_operation() {
    let src = r#"
        int main(void) {
            return 1 * / 2;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, Star, Slash,
        Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_3_invalid_parse_extra_credit_bitwise_double_operator() {
    let src = r#"
        int main(void) {
            return 1 | | 2;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, Pipe, Pipe,
        Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_3_invalid_parse_imbalanced_paren() {
    let src = r#"
        int main(void) {
            return 1 + (2;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, Plus, OpenParen,
        Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_3_invalid_parse_malformed_paren() {
    let src = r#"
        int main(void) {
            return 2 (- 3);
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, OpenParen,
        Minus, Constant, CloseParen, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_3_invalid_parse_misplaced_semicolon() {
    let src = r#"
        int main(void) {
            return 1 + (2;)
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, Plus, OpenParen,
        Constant, Semicolon, CloseParen, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_3_invalid_parse_missing_first_op() {
    let src = r#"
        int main(void) {
            return /3;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Slash, Constant,
        Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_3_invalid_parse_missing_open_paren() {
    let src = r#"
        int main(void) {
            return 1 + 2);
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, Plus, Constant,
        CloseParen, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_3_invalid_parse_missing_second_op() {
    let src = r#"
        int main(void) {
            return 1 + ;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, Plus, Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_3_invalid_parse_no_semicolon() {
    let src = r#"
        int main(void) {
            return 2*2
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, Star, Constant,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_3_valid_add() {
    let src = r#"
        int main(void) {
            return 1 + 2;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, Plus, Constant,
        Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_3_valid_associativity() {
    let src = r#"
        int main(void) {
            return 1 - 2 - 3;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, Minus, Constant,
        Minus, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_3_valid_associativity_2() {
    let src = r#"
        int main(void) {
            return 6 / 3 / 2;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, Slash, Constant,
        Slash, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_3_valid_associativity_3() {
    let src = r#"
        int main(void) {
            return (3 / 2 * 4) + (5 - 4 + 3);
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, OpenParen, Constant,
        Slash, Constant, Star, Constant, CloseParen, Plus, OpenParen, Constant, Minus, Constant,
        Plus, Constant, CloseParen, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_3_valid_associativity_and_precedence() {
    let src = r#"
        int main(void) {
            return 5 * 4 / 2 -
                3 % (2 + 1);
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, Star, Constant,
        Slash, Constant, Minus, Constant, Percent, OpenParen, Constant, Plus, Constant, CloseParen,
        Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_3_valid_div() {
    let src = r#"
        int main(void) {
            return 4 / 2;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, Slash, Constant,
        Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_3_valid_div_neg() {
    let src = r#"
        int main(void) {
            return (-12) / 5;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, OpenParen, Minus,
        Constant, CloseParen, Slash, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_3_valid_extra_credit_bitwise_and() {
    let src = r#"
        int main(void) {
            return 3 & 5;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, Ampersand,
        Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_3_valid_extra_credit_bitwise_or() {
    let src = r#"
        int main(void) {
            return 1 | 2;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, Pipe, Constant,
        Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_3_valid_extra_credit_bitwise_precedence() {
    let src = r#"
        int main(void) {
            return 80 >> 2 | 1 ^ 5 & 7 << 1;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant,
        GreaterGreater,
        Constant,
        Pipe,
        Constant,
        Circumflex,
        Constant,
        Ampersand,
        Constant,
        LessLess,
        Constant,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_3_valid_extra_credit_bitwise_shift_associativity() {
    let src = r#"
        int main(void) {
            return 33 << 4 >> 2;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant,
        LessLess,
        Constant,
        GreaterGreater,
        Constant,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_3_valid_extra_credit_bitwise_shift_associativity_2() {
    let src = r#"
        int main(void) {
            return 33 >> 2 << 1;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant,
        GreaterGreater,
        Constant,
        LessLess,
        Constant,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_3_valid_extra_credit_bitwise_shift_precedence() {
    let src = r#"
        int main(void) {
            return 40 << 4 + 12 >> 1;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant,
        LessLess,
        Constant,
        Plus,
        Constant,
        GreaterGreater,
        Constant,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_3_valid_extra_credit_bitwise_shiftl() {
    let src = r#"
        int main(void) {
            return 35 << 2;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, LessLess,
        Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_3_valid_extra_credit_bitwise_shiftr() {
    let src = r#"
        int main(void) {
            return 1000 >> 4;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant,
        GreaterGreater,
        Constant,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_3_valid_extra_credit_bitwise_shiftr_negative() {
    let src = r#"
        int main(void) {
            return -5 >> 30;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Minus,
        Constant,
        GreaterGreater,
        Constant,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_3_valid_extra_credit_bitwise_variable_shift_count() {
    let src = r#"
        int main(void) {
            return (4 << (2 * 2)) + (100 >> (1 + 2));
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        OpenParen,
        Constant,
        LessLess,
        OpenParen,
        Constant,
        Star,
        Constant,
        CloseParen,
        CloseParen,
        Plus,
        OpenParen,
        Constant,
        GreaterGreater,
        OpenParen,
        Constant,
        Plus,
        Constant,
        CloseParen,
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_3_valid_extra_credit_bitwise_xor() {
    let src = r#"
        int main(void) {
            return 7 ^ 1;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, Circumflex,
        Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_3_valid_mod() {
    let src = r#"
        int main(void) {
            return 4 % 2;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, Percent,
        Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_3_valid_mult() {
    let src = r#"
        int main(void) {
            return 2 * 3;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, Star, Constant,
        Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_3_valid_parens() {
    let src = r#"
        int main(void) {
            return 2 * (3 + 4);
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, Star, OpenParen,
        Constant, Plus, Constant, CloseParen, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_3_valid_precedence() {
    let src = r#"
        int main(void) {
            return 2 + 3 * 4;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, Plus, Constant,
        Star, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_3_valid_sub() {
    let src = r#"
        int main(void) {
            return 1 - 2;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, Minus, Constant,
        Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_3_valid_sub_neg() {
    let src = r#"
        int main(void) {
            return 2- -1;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, Minus, Minus,
        Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_3_valid_unop_add() {
    let src = r#"
        int main(void) {
            return ~2 + 3;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Tilde, Constant, Plus,
        Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_3_valid_unop_parens() {
    let src = r#"
        int main(void) {
            return ~(1 + 1);
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Tilde, OpenParen,
        Constant, Plus, Constant, CloseParen, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_4_invalid_parse_missing_const() {
    let src = r#"
        int main(void)
        {
            10 <= !;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Constant, LessEqual, Bang,
        Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_4_invalid_parse_missing_first_op() {
    let src = r#"
        int main(void) {
            return <= 2;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, LessEqual, Constant,
        Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_4_invalid_parse_missing_operand() {
    let src = r#"
        int main(void) {
            return 1 < > 3;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, Less, Greater,
        Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_4_invalid_parse_missing_second_op() {
    let src = r#"
        int main(void) {
            return 2 && ~;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant,
        AmpersandAmpersand,
        Tilde,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_4_invalid_parse_missing_semicolon() {
    let src = r#"
        int main(void) {
            return 1 || 2
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, PipePipe,
        Constant, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_4_invalid_parse_unary_missing_semicolon() {
    let src = r#"
        int main(void)
        {
            return !10
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Bang, Constant, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_4_valid_and_false() {
    let src = r#"
        int main(void) {
            return (10 && 0) + (0 && 4) + (0 && 0);
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        OpenParen,
        Constant,
        AmpersandAmpersand,
        Constant,
        CloseParen,
        Plus,
        OpenParen,
        Constant,
        AmpersandAmpersand,
        Constant,
        CloseParen,
        Plus,
        OpenParen,
        Constant,
        AmpersandAmpersand,
        Constant,
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_4_valid_and_short_circuit() {
    let src = r#"
        int main(void) {
            return 0 && (1 / 0);
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant,
        AmpersandAmpersand,
        OpenParen,
        Constant,
        Slash,
        Constant,
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_4_valid_and_true() {
    let src = r#"
        int main(void) {
            return 1 && -1;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant,
        AmpersandAmpersand,
        Minus,
        Constant,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_4_valid_associativity() {
    let src = r#"
        int main(void) {
            return 5 >= 0 > 1 <= 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant,
        GreaterEqual,
        Constant,
        Greater,
        Constant,
        LessEqual,
        Constant,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_4_valid_compare_arithmetic_results() {
    let src = r#"
        int main(void) {
            return ~2 * -2 == 1 + 5;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Tilde, Constant, Star,
        Minus, Constant, EqualEqual, Constant, Plus, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_4_valid_eq_false() {
    let src = r#"
        int main(void) {
            return 1 == 2;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, EqualEqual,
        Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_4_valid_eq_precedence() {
    let src = r#"
        int main(void) {
            return 3 == 1 != 2;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, EqualEqual,
        Constant, BangEqual, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_4_valid_eq_true() {
    let src = r#"
        int main(void) {
            return 1 == 1;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, EqualEqual,
        Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_4_valid_extra_credit_bitwise_and_precedence() {
    let src = r#"
        int main(void) {
            return 5 & 7 == 5;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, Ampersand,
        Constant, EqualEqual, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_4_valid_extra_credit_bitwise_or_precedence() {
    let src = r#"
        int main(void) {
            return 5 | 7 != 5;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, Pipe, Constant,
        BangEqual, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_4_valid_extra_credit_bitwise_shift_precedence() {
    let src = r#"
        int main(void) {
            return 20 >> 4 <= 3 << 1;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant,
        GreaterGreater,
        Constant,
        LessEqual,
        Constant,
        LessLess,
        Constant,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_4_valid_extra_credit_bitwise_xor_precedence() {
    let src = r#"
        int main(void) {
            return 5 ^ 7 < 5;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, Circumflex,
        Constant, Less, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_4_valid_ge_false() {
    let src = r#"
        int main(void) {
            return 1 >= 2;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant,
        GreaterEqual,
        Constant,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_4_valid_ge_true() {
    let src = r#"
        int main(void) {
            return (1 >= 1) + (1 >= -4);
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        OpenParen,
        Constant,
        GreaterEqual,
        Constant,
        CloseParen,
        Plus,
        OpenParen,
        Constant,
        GreaterEqual,
        Minus,
        Constant,
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_4_valid_gt_false() {
    let src = r#"
        int main(void) {
            return (1 > 2) + (1 > 1);
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, OpenParen, Constant,
        Greater, Constant, CloseParen, Plus, OpenParen, Constant, Greater, Constant, CloseParen,
        Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_4_valid_gt_true() {
    let src = r#"
        int main(void) {
            return 15 > 10;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, Greater,
        Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_4_valid_le_false() {
    let src = r#"
        int main(void) {
            return 1 <= -1;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, LessEqual,
        Minus, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_4_valid_le_true() {
    let src = r#"
        int main(void) {
            return (0 <= 2) + (0 <= 0);
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, OpenParen, Constant,
        LessEqual, Constant, CloseParen, Plus, OpenParen, Constant, LessEqual, Constant,
        CloseParen, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_4_valid_lt_false() {
    let src = r#"
        int main(void) {
            return 2 < 1;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, Less, Constant,
        Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_4_valid_lt_true() {
    let src = r#"
        int main(void) {
            return 1 < 2;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, Less, Constant,
        Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_4_valid_multi_short_circuit() {
    let src = r#"
        int main(void) {
            return 0 || 0 && (1 / 0);
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant,
        PipePipe,
        Constant,
        AmpersandAmpersand,
        OpenParen,
        Constant,
        Slash,
        Constant,
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_4_valid_ne_false() {
    let src = r#"
        int main(void) {
            return 0 != 0;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, BangEqual,
        Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_4_valid_ne_true() {
    let src = r#"
        int main(void) {
            return -1 != -2;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Minus, Constant,
        BangEqual, Minus, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_4_valid_nested_ops() {
    let src = r#"
        int main(void) {
            return !-3;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Bang, Minus, Constant,
        Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_4_valid_not() {
    let src = r#"
        int main(void) {
            return !5;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Bang, Constant, Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_4_valid_not_sum() {
    let src = r#"
        int main(void) {
            return !(4-4);
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Bang, OpenParen, Constant,
        Minus, Constant, CloseParen, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_4_valid_not_sum_2() {
    let src = r#"
        int main(void) {
            return !(3 - 44);
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Bang, OpenParen, Constant,
        Minus, Constant, CloseParen, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_4_valid_not_zero() {
    let src = r#"
        int main(void) {
            return !0;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Bang, Constant, Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_4_valid_operate_on_booleans() {
    let src = r#"
        int main(void) {
            return ~(0 && 1) - -(4 || 3);
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Tilde,
        OpenParen,
        Constant,
        AmpersandAmpersand,
        Constant,
        CloseParen,
        Minus,
        Minus,
        OpenParen,
        Constant,
        PipePipe,
        Constant,
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_4_valid_or_false() {
    let src = r#"
        int main(void) {
            return 0 || 0;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, PipePipe,
        Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_4_valid_or_short_circuit() {
    let src = r#"
        int main(void) {
            return 1 || (1 / 0);
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, PipePipe,
        OpenParen, Constant, Slash, Constant, CloseParen, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_4_valid_or_true() {
    let src = r#"
        int main(void) {
            return (4 || 0) + (0 || 3) + (5 || 5);
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, OpenParen, Constant,
        PipePipe, Constant, CloseParen, Plus, OpenParen, Constant, PipePipe, Constant, CloseParen,
        Plus, OpenParen, Constant, PipePipe, Constant, CloseParen, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_4_valid_precedence() {
    let src = r#"
        int main(void) {
            return 1 || 0 && 2;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant,
        PipePipe,
        Constant,
        AmpersandAmpersand,
        Constant,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_4_valid_precedence_2() {
    let src = r#"
        int main(void) {
            return (1 || 0) && 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        OpenParen,
        Constant,
        PipePipe,
        Constant,
        CloseParen,
        AmpersandAmpersand,
        Constant,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_4_valid_precedence_3() {
    let src = r#"
        int main(void) {
            return 2 == 2 >= 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant,
        EqualEqual,
        Constant,
        GreaterEqual,
        Constant,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_4_valid_precedence_4() {
    let src = r#"
        int main(void) {
            return 2 == 2 || 0;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, EqualEqual,
        Constant, PipePipe, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_4_valid_precedence_5() {
    let src = r#"
        int main(void) {
            return (0 == 0 && 3 == 2 + 1 > 1) + 1;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        OpenParen,
        Constant,
        EqualEqual,
        Constant,
        AmpersandAmpersand,
        Constant,
        EqualEqual,
        Constant,
        Plus,
        Constant,
        Greater,
        Constant,
        CloseParen,
        Plus,
        Constant,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_5_invalid_parse_compound_invalid_operator() {
    let src = r#"
        int main(void) {
            int a = 0;
            a + = 1;
            return a;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Identifier, Plus, Equal, Constant, Semicolon, Return, Identifier, Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_5_invalid_parse_declare_keyword_as_var() {
    let src = r#"
        int main(void) {
            int return = 4;
            return return + 1;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Return, Equal, Constant,
        Semicolon, Return, Return, Plus, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_5_invalid_parse_invalid_specifier() {
    let src = r#"
        int main(void) {
            int foo bar = 3;
            return bar;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Identifier,
        Equal, Constant, Semicolon, Return, Identifier, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_5_invalid_parse_invalid_type() {
    let src = r#"
        int main(void) {
            ints a = 1;
            return a;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Identifier, Identifier, Equal,
        Constant, Semicolon, Return, Identifier, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_5_invalid_parse_invalid_variable_name() {
    let src = r#"
        int main(void)
        {
            int 10 = 0;
            return 10;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Constant, Equal, Constant,
        Semicolon, Return, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_5_invalid_parse_malformed_compound_assignment() {
    let src = r#"
        int main(void) {
            int a = 10;
            a =/ 1;
            return a;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Identifier, Equal, Slash, Constant, Semicolon, Return, Identifier, Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_5_invalid_parse_malformed_decrement() {
    let src = r#"
        int main(void) {
            int a = 0;
            a - -;
            return a;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Identifier, Minus, Minus, Semicolon, Return, Identifier, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_5_invalid_parse_malformed_increment() {
    let src = r#"
        int main(void) {
            int a = 0;
            a + +;
            return a;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Identifier, Plus, Plus, Semicolon, Return, Identifier, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_5_invalid_parse_malformed_less_equal() {
    let src = r#"
        int main(void)
        {
            return 1 < = 2;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, Less, Equal,
        Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_5_invalid_parse_malformed_not_equal() {
    let src = r#"
        int main(void)
        {
            return 1 ! = 0;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, Bang, Equal,
        Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_5_invalid_parse_missing_semicolon() {
    let src = r#"
        int main(void) {
            int a = 2
            a = a + 4;
            return a;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Identifier, Equal, Identifier, Plus, Constant, Semicolon, Return, Identifier, Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_5_invalid_parse_return_in_assignment() {
    let src = r#"
        int main(void)
        {
            int 10 = return 0;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Constant, Equal, Return,
        Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Identifier, Equal, Constant, Plus,
        Constant, Semicolon, Int, Identifier, Semicolon, Return, Identifier, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_5_invalid_semantics_extra_credit_undeclared_bitwise_op() {
    let src = r#"
        int main(void){
            return a >> 2;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        GreaterGreater,
        Constant,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Identifier, Plus, Constant, Equal, Constant, Semicolon, Return, Identifier,
        Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Bang, Identifier, Equal, Constant, Semicolon, Return, Identifier, Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Int, Identifier, Equal, Constant, Semicolon, Identifier, Equal, Constant, Star,
        Identifier, Equal, Identifier, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Int, Identifier, Equal, Constant, Semicolon, Return, Identifier, Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_5_invalid_semantics_undeclared_var() {
    let src = r#"
        int main(void) {
            return a;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Identifier, Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_5_invalid_semantics_undeclared_var_and() {
    let src = r#"
        int main(void) {
            return 0 && a;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant,
        AmpersandAmpersand,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_5_invalid_semantics_undeclared_var_compare() {
    let src = r#"
        int main(void) {
            return a < 5;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Identifier, Less,
        Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_5_invalid_semantics_undeclared_var_unary() {
    let src = r#"
        int main(void) {
            return -a;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Minus, Identifier,
        Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Return, Identifier, Semicolon, Int, Identifier, Equal, Constant, Semicolon,
        Return, Identifier, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Int, Identifier, Equal, Constant, Semicolon, Return, Identifier, Plus,
        Identifier, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Int, Identifier, Equal, Constant, Semicolon, Int, Identifier, Equal, Identifier,
        Slash, Constant, Plus, Bang, Identifier, Semicolon, Return, Identifier, Star, Constant,
        EqualEqual, Identifier, Minus, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Semicolon,
        Identifier, Equal, Constant, Semicolon, Return, Identifier, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_5_valid_assign_val_in_initializer() {
    let src = r#"
        int main(void) {
            int a = a = 5;
            return a;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal,
        Identifier, Equal, Constant, Semicolon, Return, Identifier, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Semicolon, Int,
        Identifier, Equal, Identifier, Equal, Constant, Semicolon, Return, Identifier, Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Semicolon,
        Identifier, Equal, Constant, PipePipe, Constant, Semicolon, Return, Identifier, Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_5_valid_empty_function_body() {
    let src = r#"
        int main(void) {
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Minus,
        Constant, Semicolon, Identifier, Equal, Identifier, Percent, Constant, Semicolon, Int,
        Identifier, Equal, Minus, Identifier, Semicolon, Return, Identifier, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Int, Identifier, Equal, Identifier, Circumflex, Constant, Semicolon, Return,
        Constant, Pipe, Identifier, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Int, Identifier, Equal, Constant, Semicolon, Int, Identifier, Equal, Constant,
        Semicolon, Return, Identifier, Ampersand, Identifier, Pipe, Identifier, Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_5_valid_extra_credit_bitwise_shiftl_variable() {
    let src = r#"
        int main(void) {
            int x = 3;
            return x << 3;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Return, Identifier, LessLess, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant,
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        GreaterGreater,
        Constant,
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Int, Identifier, Equal, Constant, Semicolon, Return, Identifier, Plus,
        Identifier, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_5_valid_local_var_missing_return() {
    let src = r#"
        int main(void) {
            int a = 3;
            a = a + 5;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Identifier, Equal, Identifier, Plus, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Int, Identifier, Equal, Constant, Semicolon, Identifier, Equal, Constant, Star,
        OpenParen, Identifier, Equal, Identifier, CloseParen, Semicolon, Return, Identifier, Plus,
        Identifier, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Constant, PipePipe, OpenParen, Identifier, Equal, Constant, CloseParen,
        Semicolon, Return, Identifier, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_5_valid_null_statement() {
    let src = r#"
        int main(void) {
            ;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_5_valid_null_then_return() {
    let src = r#"
        int main(void) {
            ;
            return 0;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Semicolon, Return, Constant,
        Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_5_valid_return_var() {
    let src = r#"
        int main(void) {
            int a = 2;
            return a;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Return, Identifier, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant,
        Semicolon,
        Constant,
        AmpersandAmpersand,
        OpenParen,
        Identifier,
        Equal,
        Constant,
        CloseParen,
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Constant, PipePipe, OpenParen, Identifier, Equal, Constant, CloseParen,
        Semicolon, Return, Identifier, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_5_valid_unused_exp() {
    let src = r#"
        int main(void) {
            2 + 2;
            return 0;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Constant, Plus, Constant,
        Semicolon, Return, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Int, Identifier, Equal, Constant, Semicolon, Return, Identifier, Equal,
        Identifier, Equal, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_5_valid_use_val_in_own_initializer() {
    let src = r#"
        int main(void) {
            int a = 0 && a;
            return a;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant,
        AmpersandAmpersand,
        Identifier,
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}
