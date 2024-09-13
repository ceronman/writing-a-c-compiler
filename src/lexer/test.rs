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
fn test_chapter_5_invalid_parse_extra_credit_binary_decrement() {
    let src = r#"
        int main(void) {
            int a = 0;
            return a -- 1;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Return, Identifier, MinusMinus, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_5_invalid_parse_extra_credit_binary_increment() {
    let src = r#"
        int main(void) {
            int a = 0;
            return a ++ 1;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Return, Identifier, PlusPlus, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_5_invalid_parse_extra_credit_compound_initializer() {
    let src = r#"
        int main(void) {
            int a += 0;
            return a;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, PlusEqual,
        Constant, Semicolon, Return, Identifier, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_5_invalid_parse_extra_credit_increment_declaration() {
    let src = r#"
        int main(void) {
            int a++;
            return 0;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, PlusPlus,
        Semicolon, Return, Constant, Semicolon, CloseBrace,
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
fn test_chapter_5_invalid_semantics_extra_credit_compound_invalid_lvalue() {
    let src = r#"
        int main(void) {
            int a = 0;
            -a += 1;
            return a;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Minus, Identifier, PlusEqual, Constant, Semicolon, Return, Identifier,
        Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_5_invalid_semantics_extra_credit_compound_invalid_lvalue_2() {
    let src = r#"
        int main(void) {
            int a = 10;
            (a += 1) -= 2;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, OpenParen, Identifier, PlusEqual, Constant, CloseParen, MinusEqual, Constant,
        Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_5_invalid_semantics_extra_credit_postfix_decr_non_lvalue() {
    let src = r#"
        int main(void) {
            int a = 10;
            return a++--;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Return, Identifier, PlusPlus, MinusMinus, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_5_invalid_semantics_extra_credit_postfix_incr_non_lvalue() {
    let src = r#"
        int main(void) {
            int a = 0;
            (a = 4)++;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, OpenParen, Identifier, Equal, Constant, CloseParen, PlusPlus, Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_5_invalid_semantics_extra_credit_prefix_decr_non_lvalue() {
    let src = r#"
        int main(void) {
            return --3;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, MinusMinus, Constant,
        Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, PlusPlus, OpenParen, Identifier, Plus, Constant, CloseParen, Semicolon, Return,
        Constant, Semicolon, CloseBrace,
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
fn test_chapter_5_invalid_semantics_extra_credit_undeclared_compound_assignment() {
    let src = r#"
        int main(void) {
            a += 1;
            return 0;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Identifier, PlusEqual, Constant,
        Semicolon, Return, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Identifier, StarEqual, Identifier, Semicolon, Return, Constant, Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_5_invalid_semantics_extra_credit_undeclared_postfix_decr() {
    let src = r#"
        int main(void) {
            a--;
            return 0;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Identifier, MinusMinus, Semicolon,
        Return, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_5_invalid_semantics_extra_credit_undeclared_prefix_incr() {
    let src = r#"
        int main(void) {
            a++;
            return 0;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Identifier, PlusPlus, Semicolon,
        Return, Constant, Semicolon, CloseBrace,
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
        Int,
        Identifier,
        Equal,
        Minus,
        Constant,
        Semicolon,
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
        PlusEqual,
        Identifier,
        MinusEqual,
        Identifier,
        StarEqual,
        Identifier,
        SlashEqual,
        Identifier,
        PercentEqual,
        Identifier,
        Equal,
        Minus,
        Constant,
        Semicolon,
        Return,
        Identifier,
        EqualEqual,
        Constant,
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant,
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Minus,
        Constant,
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Minus,
        Constant,
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Minus,
        Constant,
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Minus,
        Constant,
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        PlusEqual,
        Constant,
        PipePipe,
        Identifier,
        Semicolon,
        Identifier,
        StarEqual,
        Identifier,
        AmpersandAmpersand,
        Constant,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant,
        Semicolon,
        Identifier,
        MinusEqual,
        Identifier,
        PipePipe,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant,
        Semicolon,
        Identifier,
        SlashEqual,
        Identifier,
        PipePipe,
        Identifier,
        Semicolon,
        Return,
        OpenParen,
        Identifier,
        EqualEqual,
        Constant,
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant,
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant,
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant,
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Identifier,
        PlusEqual,
        Constant,
        Semicolon,
        Return,
        OpenParen,
        Identifier,
        EqualEqual,
        Constant,
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant,
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Identifier,
        AmpersandEqual,
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
        AmpersandEqual,
        Constant,
        PipePipe,
        Identifier,
        Semicolon,
        Identifier,
        CircumflexEqual,
        Identifier,
        PipePipe,
        Constant,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant,
        Semicolon,
        Identifier,
        PipeEqual,
        Identifier,
        PipePipe,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant,
        Semicolon,
        Identifier,
        GreaterGreaterEqual,
        Identifier,
        PipePipe,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant,
        Semicolon,
        Identifier,
        LessLessEqual,
        Identifier,
        PipePipe,
        Identifier,
        Semicolon,
        Return,
        OpenParen,
        Identifier,
        EqualEqual,
        Constant,
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant,
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant,
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant,
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant,
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        AmpersandEqual,
        Identifier,
        StarEqual,
        Identifier,
        PipeEqual,
        Identifier,
        Equal,
        Identifier,
        CircumflexEqual,
        Identifier,
        PlusEqual,
        Identifier,
        GreaterGreaterEqual,
        Identifier,
        LessLessEqual,
        Identifier,
        Equal,
        Constant,
        Semicolon,
        Return,
        OpenParen,
        Identifier,
        EqualEqual,
        Constant,
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant,
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant,
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant,
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant,
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant,
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant,
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant,
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant,
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant,
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Identifier, PipeEqual, Constant, Semicolon, Return, Identifier, Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Identifier,
        LessLessEqual,
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
fn test_chapter_5_valid_extra_credit_compound_bitwise_shiftr() {
    let src = r#"
        int main(void) {
            int to_shiftr = 382574;
            to_shiftr >>= 4;
            return to_shiftr;
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
        Identifier,
        GreaterGreaterEqual,
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
fn test_chapter_5_valid_extra_credit_compound_bitwise_xor() {
    let src = r#"
        int main(void) {
            int to_xor = 7;
            to_xor ^= 5;
            return to_xor;
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
        Identifier,
        CircumflexEqual,
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
fn test_chapter_5_valid_extra_credit_compound_divide() {
    let src = r#"
        int main(void) {
            int to_divide = 8;
            to_divide /= 4;
            return to_divide;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Identifier, SlashEqual, Constant, Semicolon, Return, Identifier, Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Identifier, MinusEqual, Constant, Semicolon, Return, Identifier, Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Identifier,
        PercentEqual,
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
fn test_chapter_5_valid_extra_credit_compound_multiply() {
    let src = r#"
        int main(void) {
            int to_multiply = 4;
            to_multiply *= 3;
            return to_multiply;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Identifier, StarEqual, Constant, Semicolon, Return, Identifier, Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Identifier, PlusEqual, Constant, Semicolon, Return, Identifier, Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        PlusPlus,
        Semicolon,
        PlusPlus,
        Identifier,
        Semicolon,
        PlusPlus,
        Identifier,
        Semicolon,
        Identifier,
        MinusMinus,
        Semicolon,
        MinusMinus,
        Identifier,
        Semicolon,
        Return,
        OpenParen,
        Identifier,
        EqualEqual,
        Constant,
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Minus,
        Constant,
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Plus,
        Identifier,
        PlusPlus,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant,
        Plus,
        PlusPlus,
        Identifier,
        Semicolon,
        Return,
        OpenParen,
        Identifier,
        EqualEqual,
        Constant,
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant,
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant,
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Int,
        Identifier,
        Equal,
        Minus,
        PlusPlus,
        OpenParen,
        Identifier,
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Bang,
        OpenParen,
        Identifier,
        CloseParen,
        MinusMinus,
        Semicolon,
        Return,
        OpenParen,
        Identifier,
        EqualEqual,
        Constant,
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant,
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Minus,
        Constant,
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant,
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Int,
        Identifier,
        Equal,
        Identifier,
        PlusPlus,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        MinusMinus,
        Semicolon,
        Return,
        OpenParen,
        Identifier,
        EqualEqual,
        Constant,
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant,
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant,
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant,
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Bang,
        Identifier,
        PlusPlus,
        Semicolon,
        Return,
        OpenParen,
        Identifier,
        EqualEqual,
        Constant,
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant,
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Int,
        Identifier,
        Equal,
        PlusPlus,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        Equal,
        MinusMinus,
        Identifier,
        Semicolon,
        Return,
        OpenParen,
        Identifier,
        EqualEqual,
        Constant,
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant,
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant,
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant,
        CloseParen,
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

#[test]
#[should_panic]
fn test_chapter_6_invalid_lex_extra_credit_bad_label() {
    tokenize(
        r#"
        int main(void) {
            0invalid_label:
                return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_6_invalid_parse_declaration_as_statement() {
    let src = r#"
        int main(void) {
            if (5)
                int i = 0;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, If, OpenParen, Constant,
        CloseParen, Int, Identifier, Equal, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_6_invalid_parse_empty_if_body() {
    let src = r#"
        int main(void) {
            if (0) else return 0;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, If, OpenParen, Constant,
        CloseParen, Else, Return, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_6_invalid_parse_extra_credit_goto_without_label() {
    let src = r#"
        int main(void) {
            goto;
        lbl:
            return 0;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Goto, Semicolon, Identifier,
        Colon, Return, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_6_invalid_parse_extra_credit_kw_label() {
    let src = r#"
        int main(void) {
            return: return 0;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Colon, Return, Constant,
        Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_6_invalid_parse_extra_credit_label_declaration() {
    let src = r#"
        int main(void) {
        label:
            int a = 0;
            return 0;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Identifier, Colon, Int,
        Identifier, Equal, Constant, Semicolon, Return, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_6_invalid_parse_extra_credit_label_expression_clause() {
    let src = r#"
        int main(void) {
            1 && label: 2;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Constant,
        AmpersandAmpersand,
        Identifier,
        Colon,
        Constant,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_6_invalid_parse_extra_credit_label_outside_function() {
    let src = r#"
        label:
        int main(void) {
            return 0;
        }
    "#;
    let expected = vec![
        Identifier, Colon, Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return,
        Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_6_invalid_parse_extra_credit_label_without_statement() {
    let src = r#"
        int main(void) {
            foo:
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Identifier, Colon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_6_invalid_parse_extra_credit_parenthesized_label() {
    let src = r#"
        int main(void) {
            goto(a);
        a:
            return 0;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Goto, OpenParen, Identifier,
        CloseParen, Semicolon, Identifier, Colon, Return, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_6_invalid_parse_if_assignment() {
    let src = r#"
        int main(void) {
            int flag = 0;
            int a = if (flag)
                        2;
                    else
                        3;
            return a;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Int, Identifier, Equal, If, OpenParen, Identifier, CloseParen, Constant,
        Semicolon, Else, Constant, Semicolon, Return, Identifier, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_6_invalid_parse_if_no_parens() {
    let src = r#"
        int main(void) {
            if 0 return 1;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, If, Constant, Return, Constant,
        Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_6_invalid_parse_incomplete_ternary() {
    let src = r#"
        int main(void) {
            return 1 ? 2;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, Question,
        Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_6_invalid_parse_malformed_ternary() {
    let src = r#"
        int main(void) {
            return 1 ? 2 : 3 : 4;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, Question,
        Constant, Colon, Constant, Colon, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_6_invalid_parse_malformed_ternary_2() {
    let src = r#"
        int main(void) {
            return 1 ? 2 ? 3 : 4;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, Question,
        Constant, Question, Constant, Colon, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_6_invalid_parse_mismatched_nesting() {
    let src = r#"
        int main(void) {
            int a = 0;
            if (1)
                return 1;
            else
                return 2;
            else
                return 3;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, If, OpenParen, Constant, CloseParen, Return, Constant, Semicolon, Else, Return,
        Constant, Semicolon, Else, Return, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_6_invalid_parse_wrong_ternary_delimiter() {
    let src = r#"
        int main(void) {
            int x = 10;
            return x ? 1 = 2;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Return, Identifier, Question, Constant, Equal, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Identifier, Colon, Identifier, Equal, Constant, Semicolon, Identifier, Colon,
        Return, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_6_invalid_semantics_extra_credit_goto_missing_label() {
    let src = r#"
        int main(void) {
            goto label;
            return 0;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Goto, Identifier, Semicolon,
        Return, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Semicolon, Goto,
        Identifier, Semicolon, Return, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Identifier, Colon, Return,
        Identifier, Semicolon, Return, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Identifier, Colon, Identifier, Equal, Identifier, Semicolon, Return, Constant,
        Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, If, OpenParen, Constant,
        CloseParen, Return, Identifier, Semicolon, Int, Identifier, Equal, Constant, Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Int, Identifier, Equal, Constant, Semicolon, Identifier, Greater, Identifier,
        Question, Identifier, Equal, Constant, Colon, Identifier, Equal, Constant, Semicolon,
        Return, Identifier, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_6_invalid_semantics_undeclared_var_in_ternary() {
    let src = r#"
        int main(void) {
            return a > 0 ? 1 : 2;
            int a = 5;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Identifier, Greater,
        Constant, Question, Constant, Colon, Constant, Semicolon, Int, Identifier, Equal, Constant,
        Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Identifier, Equal, Constant, Question, Constant, Colon, Constant, Semicolon,
        Return, Identifier, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_6_valid_binary_condition() {
    let src = r#"
        int main(void) {
            if (1 + 2 == 3)
                return 5;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, If, OpenParen, Constant, Plus,
        Constant, EqualEqual, Constant, CloseParen, Return, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_6_valid_binary_false_condition() {
    let src = r#"
        int main(void) {
            if (1 + 2 == 4)
                return 5;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, If, OpenParen, Constant, Plus,
        Constant, EqualEqual, Constant, CloseParen, Return, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, If, OpenParen, Identifier, CloseParen, Return, Constant, Semicolon, Else,
        Return, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Semicolon,
        Constant, Circumflex, Constant, Question, Identifier, Equal, Constant, Colon, OpenParen,
        Identifier, Equal, Constant, CloseParen, Semicolon, Return, Identifier, Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Identifier, StarEqual, Constant, Question, Constant, Colon, Constant, Semicolon,
        Return, Identifier, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, If, OpenParen, Identifier, PlusEqual, Constant, CloseParen, Return, Identifier,
        Semicolon, Return, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Goto,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        Equal,
        OpenParen,
        Identifier,
        Equal,
        Constant,
        CloseParen,
        Semicolon,
        Identifier,
        Colon,
        Identifier,
        Equal,
        Constant,
        Semicolon,
        Return,
        OpenParen,
        Identifier,
        EqualEqual,
        Constant,
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant,
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, If, OpenParen, Constant,
        CloseParen, Identifier, Colon, Return, Constant, Semicolon, Goto, Identifier, Semicolon,
        Return, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Goto, Identifier, Semicolon,
        Return, Constant, Semicolon, Identifier, Colon, Return, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Goto, Identifier, Semicolon, Return, Constant, Semicolon, Identifier, Colon,
        Return, Identifier, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Goto, Identifier, Semicolon,
        Return, Constant, Semicolon, Identifier, Colon, Return, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Goto, Identifier, Semicolon,
        Return, Constant, Semicolon, Identifier, Colon, Return, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Goto, Identifier, Semicolon,
        Identifier, Colon, Identifier, Colon, Return, Constant, Semicolon, Return, Constant,
        Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Identifier, Colon, If, OpenParen, Identifier, CloseParen, Goto, Identifier,
        Semicolon, Else, Goto, Identifier, Semicolon, Identifier, Colon, Goto, Identifier,
        Semicolon, If, OpenParen, Constant, CloseParen, Identifier, Colon, Identifier, Equal,
        Constant, Semicolon, Goto, Identifier, Semicolon, Identifier, Colon, Return, Identifier,
        Semicolon, Identifier, Colon, Semicolon, Identifier, Equal, Constant, Semicolon, Goto,
        Identifier, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Goto, Identifier, Semicolon,
        Return, Constant, Semicolon, Identifier, Colon, Return, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, OpenParen, Identifier, MinusEqual, Constant, CloseParen, Question, OpenParen,
        Identifier, SlashEqual, Constant, CloseParen, Colon, Constant, Semicolon, Return,
        Identifier, EqualEqual, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, If, OpenParen, Identifier, MinusMinus, CloseParen, Return, Constant, Semicolon,
        Else, If, OpenParen, Identifier, MinusMinus, CloseParen, Return, Constant, Semicolon,
        Return, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Identifier, Minus, Constant, Question, Constant, Colon, Identifier, MinusMinus,
        Semicolon, Return, Identifier, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Minus,
        Constant, Semicolon, If, OpenParen, PlusPlus, Identifier, CloseParen, Return, Constant,
        Semicolon, Else, If, OpenParen, PlusPlus, Identifier, CloseParen, Return, Constant,
        Semicolon, Return, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_6_valid_extra_credit_prefix_in_ternary() {
    let src = r#"
        int main(void) {
            int a = 0;
            return (++a ? ++a : 0);
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Return, OpenParen, PlusPlus, Identifier, Question, PlusPlus, Identifier, Colon,
        Constant, CloseParen, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_6_valid_extra_credit_unused_label() {
    let src = r#"
        int main(void) {
        unused:
            return 0;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Identifier, Colon, Return,
        Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Goto, Identifier, Semicolon,
        Return, Constant, Semicolon, Identifier, Colon, Identifier, Colon, Return, Constant,
        Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Int, Identifier, Equal, Constant, Semicolon, If, OpenParen, Identifier,
        CloseParen, Identifier, Equal, Constant, Semicolon, Else, If, OpenParen, Identifier,
        CloseParen, Identifier, Equal, Constant, Semicolon, Return, Identifier, Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Int, Identifier, Equal, Constant, Semicolon, If, OpenParen, Identifier,
        CloseParen, Identifier, Equal, Constant, Semicolon, Else, If, OpenParen, Tilde, Identifier,
        CloseParen, Identifier, Equal, Constant, Semicolon, Return, Identifier, Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, If, OpenParen, OpenParen, Identifier, Equal, Constant, CloseParen, CloseParen,
        If, OpenParen, Identifier, EqualEqual, Constant, CloseParen, Identifier, Equal, Constant,
        Semicolon, Else, Identifier, Equal, Constant, Semicolon, Return, Identifier, Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, If, OpenParen, Bang, Identifier, CloseParen, If, OpenParen, Constant, Slash,
        Constant, CloseParen, Identifier, Equal, Constant, Semicolon, Else, Identifier, Equal,
        Constant, Slash, Constant, Semicolon, Return, Identifier, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, If, OpenParen, Constant, CloseParen, If, OpenParen, Constant, CloseParen,
        Identifier, Equal, Constant, Semicolon, Else, Identifier, Equal, Constant, Semicolon, Else,
        Identifier, Equal, Constant, Semicolon, Return, Identifier, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Int, Identifier, Equal, Constant, Semicolon, If, OpenParen, Identifier,
        CloseParen, Identifier, Equal, Constant, Semicolon, Return, Identifier, Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, If, OpenParen, Constant, CloseParen, Semicolon, Else, Identifier, Equal,
        Constant, Semicolon, Return, Identifier, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Int, Identifier, Equal, Constant, Semicolon, If, OpenParen, Identifier,
        CloseParen, Identifier, Equal, Constant, Semicolon, Return, Identifier, Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        OpenParen,
        Identifier,
        Equal,
        Constant,
        CloseParen,
        Question,
        Identifier,
        Colon,
        Constant,
        Semicolon,
        Return,
        OpenParen,
        Identifier,
        EqualEqual,
        Constant,
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant,
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Int, Identifier, Equal, Constant, Semicolon, If, OpenParen, Identifier,
        CloseParen, Identifier, Equal, Constant, Semicolon, Else, Identifier, Equal, Constant,
        Semicolon, If, OpenParen, Identifier, CloseParen, Identifier, Equal, Constant, Semicolon,
        Else, Identifier, Equal, Constant, Semicolon, Return, Identifier, Plus, Identifier,
        Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Int, Identifier, Equal, Constant, Semicolon, Int, Identifier, Equal, Constant,
        Semicolon, Return, Identifier, Greater, Identifier, Question, Constant, Colon, Identifier,
        Question, Constant, Colon, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Question, Constant, Question, Constant, Colon, Constant, Colon, Constant, Semicolon, Int,
        Identifier, Equal, Constant, Question, Constant, Question, Constant, Colon, Constant,
        Colon, Constant, Semicolon, Return, Identifier, Star, Identifier, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Int, Identifier, Equal, Constant, Semicolon, Identifier, Question, Identifier,
        Equal, Constant, Colon, OpenParen, Identifier, Equal, Constant, CloseParen, Semicolon,
        Return, Identifier, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_6_valid_ternary() {
    let src = r#"
        int main(void) {
            int a = 0;
            return a > -1 ? 4 : 5;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Return, Identifier, Greater, Minus, Constant, Question, Constant, Colon,
        Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Identifier, BangEqual, Constant, Question, Identifier, Equal, Constant, Colon,
        Constant, Semicolon, Return, Identifier, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_6_valid_ternary_middle_binop() {
    let src = r#"
        int main(void) {
            int a = 1 ? 3 % 2 : 4;
            return a;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Question, Constant, Percent, Constant, Colon, Constant, Semicolon, Return, Identifier,
        Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_6_valid_ternary_precedence() {
    let src = r#"
        int main(void) {
            int a = 10;
            return a || 0 ? 20 : 0;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Return, Identifier, PipePipe, Constant, Question, Constant, Colon, Constant,
        Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_6_valid_ternary_rh_binop() {
    let src = r#"
        int main(void) {
            return 0 ? 1 : 0 || 2;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Return, Constant, Question,
        Constant, Colon, Constant, PipePipe, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Int, Identifier, Equal, Constant, Semicolon, Identifier, Question, OpenParen,
        Identifier, Equal, Constant, CloseParen, Colon, OpenParen, Identifier, Equal, Constant,
        CloseParen, Semicolon, Return, Identifier, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Int, Identifier, Equal, Constant, Semicolon, Identifier, Question, OpenParen,
        Identifier, Equal, Constant, CloseParen, Colon, OpenParen, Identifier, Equal, Constant,
        CloseParen, Semicolon, Return, Identifier, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_7_invalid_parse_extra_brace() {
    let src = r#"
        int main(void) {
            if(0){
                return 1;
            }}
            return 2;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, If, OpenParen, Constant,
        CloseParen, OpenBrace, Return, Constant, Semicolon, CloseBrace, CloseBrace, Return,
        Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_7_invalid_parse_missing_brace() {
    let src = r#"
        int main(void) {
            if(0){
                return 1;
            return 2;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, If, OpenParen, Constant,
        CloseParen, OpenBrace, Return, Constant, Semicolon, Return, Constant, Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_7_invalid_parse_missing_semicolon() {
    let src = r#"
        int main(void) {
            int a = 4;
            {
                a = 5;
                return a
            }
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, OpenBrace, Identifier, Equal, Constant, Semicolon, Return, Identifier,
        CloseBrace, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_7_invalid_parse_ternary_blocks() {
    let src = r#"
        int main(void) {
            int a;
            return 1 ? { a = 2 } : a = 4;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Semicolon,
        Return, Constant, Question, OpenBrace, Identifier, Equal, Constant, CloseBrace, Colon,
        Identifier, Equal, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, OpenBrace, Int, Identifier,
        Semicolon, Int, Identifier, Semicolon, CloseBrace, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, OpenBrace, Identifier, Equal, Constant, Semicolon, CloseBrace, Int, Identifier,
        Equal, Constant, Semicolon, Return, Identifier, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Identifier, Colon, Semicolon, Int,
        Identifier, Equal, Constant, Semicolon, Identifier, Colon, Semicolon, Int, Identifier,
        Equal, Constant, Semicolon, Return, Constant, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, If, OpenParen, Identifier, CloseParen, OpenBrace, Identifier, Equal, Constant,
        Semicolon, Goto, Identifier, Semicolon, Return, Constant, Semicolon, Identifier, Colon,
        Return, Identifier, Semicolon, CloseBrace, Else, OpenBrace, Goto, Identifier, Semicolon,
        Return, Constant, Semicolon, Identifier, Colon, Return, Identifier, Semicolon, CloseBrace,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, If, OpenParen, Identifier, BangEqual, Constant, CloseParen, OpenBrace,
        Identifier, Colon, Return, Identifier, Semicolon, CloseBrace, Int, Identifier, Equal,
        Constant, Semicolon, Goto, Identifier, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, OpenBrace, Int, Identifier, Equal,
        Constant, Semicolon, CloseBrace, Return, Identifier, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Semicolon,
        OpenBrace, Identifier, Equal, Constant, Semicolon, CloseBrace, Int, Identifier, Semicolon,
        Return, Identifier, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, OpenBrace, Int, Identifier, Equal, Identifier, Equal, Constant, Semicolon,
        Return, Identifier, Semicolon, CloseBrace, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, OpenBrace, Int, Identifier, Equal, Identifier, Equal, Constant, Semicolon,
        CloseBrace, Return, Identifier, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Semicolon,
        OpenBrace, Int, Identifier, Equal, Identifier, Equal, Constant, Semicolon, CloseBrace,
        Return, Identifier, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, OpenBrace, CloseBrace, Int, Identifier, Equal, Constant, Star, Constant,
        Semicolon, OpenBrace, OpenBrace, CloseBrace, CloseBrace, Return, Identifier, Plus,
        Identifier, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, If, OpenParen, Identifier, Greater, Constant, CloseParen, OpenBrace, Identifier,
        MinusEqual, Constant, Semicolon, Int, Identifier, Equal, Constant, Semicolon, If,
        OpenParen, Identifier, Greater, Constant, CloseParen, OpenBrace, Identifier, MinusEqual,
        Constant, Semicolon, CloseBrace, CloseBrace, Return, Identifier, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, OpenBrace, If, OpenParen, Identifier, BangEqual, Constant, CloseParen,
        Identifier, Colon, Return, Identifier, Semicolon, Int, Identifier, Equal, Constant,
        Semicolon, Goto, Identifier, Semicolon, CloseBrace, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Goto, Identifier, Semicolon, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, Identifier, Colon, Identifier, Equal, Constant, Semicolon, Return, Identifier,
        Semicolon, CloseBrace, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        If,
        OpenParen,
        Identifier,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant,
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Semicolon,
        Goto,
        Identifier,
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Constant,
        Semicolon,
        Identifier,
        Colon,
        Return,
        OpenParen,
        Identifier,
        EqualEqual,
        Constant,
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant,
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, If, OpenParen, Constant, CloseParen, OpenBrace, Int, Identifier, Equal,
        Constant, Semicolon, Goto, Identifier, Semicolon, Identifier, Equal, Constant, Semicolon,
        Identifier, Colon, Identifier, Equal, Constant, Semicolon, Identifier, Equal, Identifier,
        Plus, Identifier, Semicolon, CloseBrace, If, OpenParen, Constant, CloseParen, OpenBrace,
        Identifier, Colon, Semicolon, Int, Identifier, Equal, Constant, Semicolon, Identifier,
        Equal, Identifier, Plus, Identifier, Semicolon, Goto, Identifier, Semicolon, Identifier,
        Equal, Constant, Semicolon, CloseBrace, Return, Identifier, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Semicolon,
        OpenBrace,
        Identifier,
        Equal,
        Minus,
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
        Plus,
        Constant,
        Semicolon,
        CloseBrace,
        Return,
        Identifier,
        EqualEqual,
        Constant,
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Minus,
        Constant,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, OpenBrace, Int, Identifier, Equal, Constant, Semicolon, Return, Identifier,
        Semicolon, CloseBrace, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, OpenBrace, Int, Identifier, Semicolon, CloseBrace, Return, Identifier,
        Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, OpenBrace, Int, Identifier, Equal, Constant, Semicolon, Identifier, Equal,
        Identifier, Semicolon, CloseBrace, OpenBrace, Int, Identifier, Equal, Constant, Semicolon,
        Identifier, Equal, Identifier, Minus, Identifier, Semicolon, CloseBrace, Return,
        Identifier, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Equal, Constant,
        Semicolon, If, OpenParen, Identifier, CloseParen, OpenBrace, Int, Identifier, Equal,
        Constant, Semicolon, Return, Identifier, Semicolon, CloseBrace, Else, OpenBrace, Int,
        Identifier, Equal, Constant, Semicolon, If, OpenParen, Identifier, Less, Identifier,
        CloseParen, OpenBrace, Return, Bang, Identifier, Semicolon, CloseBrace, Else, OpenBrace,
        Return, Constant, Semicolon, CloseBrace, CloseBrace, Return, Identifier, Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Semicolon, Int,
        Identifier, Semicolon, Int, Identifier, Equal, Constant, Semicolon, OpenBrace, Int,
        Identifier, Equal, Constant, Semicolon, Int, Identifier, Equal, Constant, Semicolon,
        OpenBrace, Int, Identifier, Semicolon, OpenBrace, Int, Identifier, Semicolon, OpenBrace,
        Int, Identifier, Semicolon, OpenBrace, Int, Identifier, Semicolon, OpenBrace, Int,
        Identifier, Semicolon, OpenBrace, Int, Identifier, Semicolon, OpenBrace, Int, Identifier,
        Semicolon, OpenBrace, Int, Identifier, Semicolon, OpenBrace, Int, Identifier, Equal,
        Constant, Semicolon, Identifier, Equal, Identifier, Semicolon, OpenBrace, Int, Identifier,
        Semicolon, Identifier, Equal, Constant, Semicolon, Identifier, Equal, Identifier, Plus,
        Identifier, Semicolon, CloseBrace, CloseBrace, CloseBrace, CloseBrace, CloseBrace,
        CloseBrace, CloseBrace, CloseBrace, CloseBrace, CloseBrace, Identifier, Equal, Identifier,
        Plus, Identifier, Semicolon, CloseBrace, Return, Identifier, Plus, Identifier, Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int, Identifier, OpenParen, Void, CloseParen, OpenBrace, Int, Identifier, Semicolon,
        OpenBrace, Identifier, Equal, Constant, Semicolon, CloseBrace, OpenBrace, Return,
        Identifier, Semicolon, CloseBrace, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}
