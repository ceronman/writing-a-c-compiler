use crate::lexer::TokenKind::*;
use crate::lexer::{tokenize, LiteralKind};

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
fn test_chapter_1_valid_multi_digit() {
    let src = r#"
        int main(void) {
            return 100;
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
        Constant(LiteralKind::Int),
        Semicolon,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Tilde,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Tilde,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Tilde,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Tilde,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Minus,
        Tilde,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        OpenParen,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Tilde,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Minus,
        OpenParen,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Minus,
        OpenParen,
        OpenParen,
        OpenParen,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        CloseParen,
        CloseParen,
        CloseParen,
        Semicolon,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Minus,
        Constant(LiteralKind::Int),
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Slash,
        Constant(LiteralKind::Int),
        Slash,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        OpenParen,
        Constant(LiteralKind::Int),
        Slash,
        Constant(LiteralKind::Int),
        Star,
        Constant(LiteralKind::Int),
        CloseParen,
        Plus,
        OpenParen,
        Constant(LiteralKind::Int),
        Minus,
        Constant(LiteralKind::Int),
        Plus,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Star,
        Constant(LiteralKind::Int),
        Slash,
        Constant(LiteralKind::Int),
        Minus,
        Constant(LiteralKind::Int),
        Percent,
        OpenParen,
        Constant(LiteralKind::Int),
        Plus,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Slash,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        OpenParen,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        Slash,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Ampersand,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Pipe,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Constant(LiteralKind::Int),
        GreaterGreater,
        Constant(LiteralKind::Int),
        Pipe,
        Constant(LiteralKind::Int),
        Circumflex,
        Constant(LiteralKind::Int),
        Ampersand,
        Constant(LiteralKind::Int),
        LessLess,
        Constant(LiteralKind::Int),
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
        Constant(LiteralKind::Int),
        LessLess,
        Constant(LiteralKind::Int),
        GreaterGreater,
        Constant(LiteralKind::Int),
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
        Constant(LiteralKind::Int),
        GreaterGreater,
        Constant(LiteralKind::Int),
        LessLess,
        Constant(LiteralKind::Int),
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
        Constant(LiteralKind::Int),
        LessLess,
        Constant(LiteralKind::Int),
        Plus,
        Constant(LiteralKind::Int),
        GreaterGreater,
        Constant(LiteralKind::Int),
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        LessLess,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Constant(LiteralKind::Int),
        GreaterGreater,
        Constant(LiteralKind::Int),
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
        Constant(LiteralKind::Int),
        GreaterGreater,
        Constant(LiteralKind::Int),
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
        Constant(LiteralKind::Int),
        LessLess,
        OpenParen,
        Constant(LiteralKind::Int),
        Star,
        Constant(LiteralKind::Int),
        CloseParen,
        CloseParen,
        Plus,
        OpenParen,
        Constant(LiteralKind::Int),
        GreaterGreater,
        OpenParen,
        Constant(LiteralKind::Int),
        Plus,
        Constant(LiteralKind::Int),
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Circumflex,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Percent,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Star,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Star,
        OpenParen,
        Constant(LiteralKind::Int),
        Plus,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Plus,
        Constant(LiteralKind::Int),
        Star,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Minus,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Tilde,
        Constant(LiteralKind::Int),
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Tilde,
        OpenParen,
        Constant(LiteralKind::Int),
        Plus,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
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
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Constant(LiteralKind::Int),
        CloseParen,
        Plus,
        OpenParen,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Constant(LiteralKind::Int),
        CloseParen,
        Plus,
        OpenParen,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Constant(LiteralKind::Int),
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
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        OpenParen,
        Constant(LiteralKind::Int),
        Slash,
        Constant(LiteralKind::Int),
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
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Minus,
        Constant(LiteralKind::Int),
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
        Constant(LiteralKind::Int),
        GreaterEqual,
        Constant(LiteralKind::Int),
        Greater,
        Constant(LiteralKind::Int),
        LessEqual,
        Constant(LiteralKind::Int),
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Tilde,
        Constant(LiteralKind::Int),
        Star,
        Minus,
        Constant(LiteralKind::Int),
        EqualEqual,
        Constant(LiteralKind::Int),
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        EqualEqual,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        EqualEqual,
        Constant(LiteralKind::Int),
        BangEqual,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        EqualEqual,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Ampersand,
        Constant(LiteralKind::Int),
        EqualEqual,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Pipe,
        Constant(LiteralKind::Int),
        BangEqual,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Constant(LiteralKind::Int),
        GreaterGreater,
        Constant(LiteralKind::Int),
        LessEqual,
        Constant(LiteralKind::Int),
        LessLess,
        Constant(LiteralKind::Int),
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Circumflex,
        Constant(LiteralKind::Int),
        Less,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Constant(LiteralKind::Int),
        GreaterEqual,
        Constant(LiteralKind::Int),
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
        Constant(LiteralKind::Int),
        GreaterEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Plus,
        OpenParen,
        Constant(LiteralKind::Int),
        GreaterEqual,
        Minus,
        Constant(LiteralKind::Int),
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        OpenParen,
        Constant(LiteralKind::Int),
        Greater,
        Constant(LiteralKind::Int),
        CloseParen,
        Plus,
        OpenParen,
        Constant(LiteralKind::Int),
        Greater,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Greater,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        LessEqual,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        OpenParen,
        Constant(LiteralKind::Int),
        LessEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Plus,
        OpenParen,
        Constant(LiteralKind::Int),
        LessEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Less,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Less,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Constant(LiteralKind::Int),
        PipePipe,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        OpenParen,
        Constant(LiteralKind::Int),
        Slash,
        Constant(LiteralKind::Int),
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        BangEqual,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Minus,
        Constant(LiteralKind::Int),
        BangEqual,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Bang,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Bang,
        Constant(LiteralKind::Int),
        Semicolon,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Bang,
        OpenParen,
        Constant(LiteralKind::Int),
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Bang,
        OpenParen,
        Constant(LiteralKind::Int),
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Bang,
        Constant(LiteralKind::Int),
        Semicolon,
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
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Constant(LiteralKind::Int),
        CloseParen,
        Minus,
        Minus,
        OpenParen,
        Constant(LiteralKind::Int),
        PipePipe,
        Constant(LiteralKind::Int),
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        PipePipe,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        PipePipe,
        OpenParen,
        Constant(LiteralKind::Int),
        Slash,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        OpenParen,
        Constant(LiteralKind::Int),
        PipePipe,
        Constant(LiteralKind::Int),
        CloseParen,
        Plus,
        OpenParen,
        Constant(LiteralKind::Int),
        PipePipe,
        Constant(LiteralKind::Int),
        CloseParen,
        Plus,
        OpenParen,
        Constant(LiteralKind::Int),
        PipePipe,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
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
        Constant(LiteralKind::Int),
        PipePipe,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Constant(LiteralKind::Int),
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
        Constant(LiteralKind::Int),
        PipePipe,
        Constant(LiteralKind::Int),
        CloseParen,
        AmpersandAmpersand,
        Constant(LiteralKind::Int),
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
        Constant(LiteralKind::Int),
        EqualEqual,
        Constant(LiteralKind::Int),
        GreaterEqual,
        Constant(LiteralKind::Int),
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        EqualEqual,
        Constant(LiteralKind::Int),
        PipePipe,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Constant(LiteralKind::Int),
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Constant(LiteralKind::Int),
        EqualEqual,
        Constant(LiteralKind::Int),
        Plus,
        Constant(LiteralKind::Int),
        Greater,
        Constant(LiteralKind::Int),
        CloseParen,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        Plus,
        Identifier,
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        Slash,
        Constant(LiteralKind::Int),
        Plus,
        Bang,
        Identifier,
        Semicolon,
        Return,
        Identifier,
        Star,
        Constant(LiteralKind::Int),
        EqualEqual,
        Identifier,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Semicolon,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Semicolon,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        PipePipe,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Percent,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Minus,
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
fn test_chapter_5_valid_extra_credit_bitwise_in_initializer() {
    let src = r#"
        int main(void) {
            int a = 15;
            int b = a ^ 5;
            return 1 | b;
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
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        Circumflex,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Pipe,
        Identifier,
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        Ampersand,
        Identifier,
        Pipe,
        Identifier,
        Semicolon,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        LessLess,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        GreaterGreater,
        Constant(LiteralKind::Int),
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
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
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
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Minus,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Minus,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Minus,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Minus,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
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
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        PlusEqual,
        Constant(LiteralKind::Int),
        PipePipe,
        Identifier,
        Semicolon,
        Identifier,
        StarEqual,
        Identifier,
        AmpersandAmpersand,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
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
        Constant(LiteralKind::Int),
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
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
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
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        PlusEqual,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        OpenParen,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
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
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        AmpersandEqual,
        Constant(LiteralKind::Int),
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
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        AmpersandEqual,
        Constant(LiteralKind::Int),
        PipePipe,
        Identifier,
        Semicolon,
        Identifier,
        CircumflexEqual,
        Identifier,
        PipePipe,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
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
        Constant(LiteralKind::Int),
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
        Constant(LiteralKind::Int),
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
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
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
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
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
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        OpenParen,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        PipeEqual,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
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
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        LessLessEqual,
        Constant(LiteralKind::Int),
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
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        GreaterGreaterEqual,
        Constant(LiteralKind::Int),
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
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        CircumflexEqual,
        Constant(LiteralKind::Int),
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        SlashEqual,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        MinusEqual,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
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
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        PercentEqual,
        Constant(LiteralKind::Int),
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        StarEqual,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        PlusEqual,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
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
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
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
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Minus,
        Constant(LiteralKind::Int),
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
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Plus,
        Identifier,
        PlusPlus,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Plus,
        PlusPlus,
        Identifier,
        Semicolon,
        Return,
        OpenParen,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
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
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
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
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Minus,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
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
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
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
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
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
        Constant(LiteralKind::Int),
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
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
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
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
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
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        Plus,
        Identifier,
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Star,
        OpenParen,
        Identifier,
        Equal,
        Identifier,
        CloseParen,
        Semicolon,
        Return,
        Identifier,
        Plus,
        Identifier,
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Constant(LiteralKind::Int),
        PipePipe,
        OpenParen,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
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
        Constant(LiteralKind::Int),
        Semicolon,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        OpenParen,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Constant(LiteralKind::Int),
        PipePipe,
        OpenParen,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
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
fn test_chapter_5_valid_unused_exp() {
    let src = r#"
        int main(void) {
            2 + 2;
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Constant(LiteralKind::Int),
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        Equal,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Constant(LiteralKind::Int),
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
fn test_chapter_6_valid_assign_ternary() {
    let src = r#"
        int main(void) {
            int a = 0;
            a = 1 ? 2 : 3;
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
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Question,
        Constant(LiteralKind::Int),
        Colon,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Constant(LiteralKind::Int),
        Plus,
        Constant(LiteralKind::Int),
        EqualEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Constant(LiteralKind::Int),
        Plus,
        Constant(LiteralKind::Int),
        EqualEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Else,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Semicolon,
        Constant(LiteralKind::Int),
        Circumflex,
        Constant(LiteralKind::Int),
        Question,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Colon,
        OpenParen,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
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
fn test_chapter_6_valid_extra_credit_compound_assign_ternary() {
    let src = r#"
        int main(void) {
            int a = 4;
            a *= 1 ? 2 : 3;
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
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        StarEqual,
        Constant(LiteralKind::Int),
        Question,
        Constant(LiteralKind::Int),
        Colon,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        PlusEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Identifier,
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Constant(LiteralKind::Int),
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
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        Identifier,
        Colon,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        OpenParen,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        Identifier,
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Goto,
        Identifier,
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Goto,
        Identifier,
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Goto,
        Identifier,
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Colon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Goto,
        Identifier,
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Goto,
        Identifier,
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Goto,
        Identifier,
        Semicolon,
        Identifier,
        Colon,
        Identifier,
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Colon,
        If,
        OpenParen,
        Identifier,
        CloseParen,
        Goto,
        Identifier,
        Semicolon,
        Else,
        Goto,
        Identifier,
        Semicolon,
        Identifier,
        Colon,
        Goto,
        Identifier,
        Semicolon,
        If,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        Identifier,
        Colon,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Goto,
        Identifier,
        Semicolon,
        Identifier,
        Colon,
        Return,
        Identifier,
        Semicolon,
        Identifier,
        Colon,
        Semicolon,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Goto,
        Identifier,
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Goto,
        Identifier,
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        OpenParen,
        Identifier,
        MinusEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Question,
        OpenParen,
        Identifier,
        SlashEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Colon,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        MinusMinus,
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Else,
        If,
        OpenParen,
        Identifier,
        MinusMinus,
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Minus,
        Constant(LiteralKind::Int),
        Question,
        Constant(LiteralKind::Int),
        Colon,
        Identifier,
        MinusMinus,
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        PlusPlus,
        Identifier,
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Else,
        If,
        OpenParen,
        PlusPlus,
        Identifier,
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        OpenParen,
        PlusPlus,
        Identifier,
        Question,
        PlusPlus,
        Identifier,
        Colon,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Identifier,
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Goto,
        Identifier,
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Colon,
        Identifier,
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        CloseParen,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Else,
        If,
        OpenParen,
        Identifier,
        CloseParen,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        CloseParen,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Else,
        If,
        OpenParen,
        Tilde,
        Identifier,
        CloseParen,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        OpenParen,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        CloseParen,
        CloseParen,
        If,
        OpenParen,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Else,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Bang,
        Identifier,
        CloseParen,
        If,
        OpenParen,
        Constant(LiteralKind::Int),
        Slash,
        Constant(LiteralKind::Int),
        CloseParen,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Else,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Slash,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        If,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Else,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Else,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        CloseParen,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        Else,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        CloseParen,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
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
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        OpenParen,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        CloseParen,
        Question,
        Identifier,
        Colon,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        OpenParen,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        CloseParen,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Else,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        CloseParen,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Else,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        Plus,
        Identifier,
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        Greater,
        Identifier,
        Question,
        Constant(LiteralKind::Int),
        Colon,
        Identifier,
        Question,
        Constant(LiteralKind::Int),
        Colon,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Question,
        Constant(LiteralKind::Int),
        Question,
        Constant(LiteralKind::Int),
        Colon,
        Constant(LiteralKind::Int),
        Colon,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Question,
        Constant(LiteralKind::Int),
        Question,
        Constant(LiteralKind::Int),
        Colon,
        Constant(LiteralKind::Int),
        Colon,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        Star,
        Identifier,
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Question,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Colon,
        OpenParen,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
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
fn test_chapter_6_valid_ternary() {
    let src = r#"
        int main(void) {
            int a = 0;
            return a > -1 ? 4 : 5;
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
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        Greater,
        Minus,
        Constant(LiteralKind::Int),
        Question,
        Constant(LiteralKind::Int),
        Colon,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        Question,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Colon,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Question,
        Constant(LiteralKind::Int),
        Percent,
        Constant(LiteralKind::Int),
        Colon,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        PipePipe,
        Constant(LiteralKind::Int),
        Question,
        Constant(LiteralKind::Int),
        Colon,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Question,
        Constant(LiteralKind::Int),
        Colon,
        Constant(LiteralKind::Int),
        PipePipe,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Question,
        OpenParen,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        CloseParen,
        Colon,
        OpenParen,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Question,
        OpenParen,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        CloseParen,
        Colon,
        OpenParen,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Semicolon,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        OpenBrace,
        CloseBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Star,
        Constant(LiteralKind::Int),
        Semicolon,
        OpenBrace,
        OpenBrace,
        CloseBrace,
        CloseBrace,
        Return,
        Identifier,
        Plus,
        Identifier,
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        Greater,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Identifier,
        MinusEqual,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        Greater,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Identifier,
        MinusEqual,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        CloseBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        OpenBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Identifier,
        Colon,
        Return,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Goto,
        Identifier,
        Semicolon,
        CloseBrace,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Goto,
        Identifier,
        Semicolon,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Colon,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        CloseBrace,
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
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
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
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Colon,
        Return,
        OpenParen,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Goto,
        Identifier,
        Semicolon,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Colon,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Identifier,
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Identifier,
        Colon,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Identifier,
        Semicolon,
        Goto,
        Identifier,
        Semicolon,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
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
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Semicolon,
        OpenBrace,
        Identifier,
        Equal,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Minus,
        Constant(LiteralKind::Int),
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        OpenBrace,
        Int,
        Identifier,
        Semicolon,
        CloseBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Semicolon,
        CloseBrace,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Minus,
        Identifier,
        Semicolon,
        CloseBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Else,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        Less,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Bang,
        Identifier,
        Semicolon,
        CloseBrace,
        Else,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        CloseBrace,
        Return,
        Identifier,
        Semicolon,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        OpenBrace,
        Int,
        Identifier,
        Semicolon,
        OpenBrace,
        Int,
        Identifier,
        Semicolon,
        OpenBrace,
        Int,
        Identifier,
        Semicolon,
        OpenBrace,
        Int,
        Identifier,
        Semicolon,
        OpenBrace,
        Int,
        Identifier,
        Semicolon,
        OpenBrace,
        Int,
        Identifier,
        Semicolon,
        OpenBrace,
        Int,
        Identifier,
        Semicolon,
        OpenBrace,
        Int,
        Identifier,
        Semicolon,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Semicolon,
        OpenBrace,
        Int,
        Identifier,
        Semicolon,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Identifier,
        Semicolon,
        CloseBrace,
        CloseBrace,
        CloseBrace,
        CloseBrace,
        CloseBrace,
        CloseBrace,
        CloseBrace,
        CloseBrace,
        CloseBrace,
        CloseBrace,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Identifier,
        Semicolon,
        CloseBrace,
        Return,
        Identifier,
        Plus,
        Identifier,
        Semicolon,
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
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Semicolon,
        OpenBrace,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        OpenBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        For,
        OpenParen,
        Identifier,
        Equal,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Less,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Identifier,
        Equal,
        Identifier,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        LessEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Break,
        Semicolon,
        CloseBrace,
        Return,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Constant(LiteralKind::Int),
        Semicolon,
        While,
        OpenParen,
        OpenParen,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        CloseParen,
        CloseParen,
        Break,
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Semicolon,
        For,
        OpenParen,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        LessEqual,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Identifier,
        Equal,
        Identifier,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        Percent,
        Constant(LiteralKind::Int),
        EqualEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Continue,
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Constant(LiteralKind::Int),
        Semicolon,
        For,
        OpenParen,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Less,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseParen,
        OpenBrace,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        Percent,
        Constant(LiteralKind::Int),
        CloseParen,
        Continue,
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Identifier,
        Semicolon,
        CloseBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Constant(LiteralKind::Int),
        Semicolon,
        Do,
        OpenBrace,
        Identifier,
        Equal,
        Identifier,
        Star,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        While,
        OpenParen,
        Identifier,
        Less,
        Constant(LiteralKind::Int),
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
        Constant(LiteralKind::Int),
        Semicolon,
        Do,
        Break,
        Semicolon,
        While,
        OpenParen,
        OpenParen,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        CloseParen,
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
fn test_chapter_8_valid_empty_expression() {
    let src = r#"
        int main(void) {
            return 0;;;
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
        Constant(LiteralKind::Int),
        Semicolon,
        Semicolon,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_8_valid_empty_loop_body() {
    let src = r#"
        int main(void) {
            int i = 2147;
            do ; while ((i = i - 5) >= 256);
            return i;
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
        Constant(LiteralKind::Int),
        Semicolon,
        Do,
        Semicolon,
        While,
        OpenParen,
        OpenParen,
        Identifier,
        Equal,
        Identifier,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        GreaterEqual,
        Constant(LiteralKind::Int),
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
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Switch,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Semicolon,
        CloseBrace,
        CloseBrace,
        Return,
        OpenParen,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Do,
        Identifier,
        PlusEqual,
        Constant(LiteralKind::Int),
        Semicolon,
        While,
        OpenParen,
        Identifier,
        MinusEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        Return,
        OpenParen,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Constant(LiteralKind::Int),
        Semicolon,
        For,
        OpenParen,
        Identifier,
        StarEqual,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        GreaterEqual,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        MinusEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        Return,
        OpenParen,
        Identifier,
        EqualEqual,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        OpenParen,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        CloseParen,
        Slash,
        Constant(LiteralKind::Int),
        Semicolon,
        Switch,
        OpenParen,
        Identifier,
        Percent,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Do,
        OpenBrace,
        Identifier,
        Equal,
        Identifier,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Identifier,
        Equal,
        Identifier,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Identifier,
        Equal,
        Identifier,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Identifier,
        Equal,
        Identifier,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Identifier,
        Equal,
        Identifier,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        While,
        OpenParen,
        OpenParen,
        Identifier,
        Equal,
        Identifier,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        Greater,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
        Return,
        OpenParen,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Constant(LiteralKind::Int),
        Semicolon,
        Do,
        OpenBrace,
        Identifier,
        Colon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        Less,
        Constant(LiteralKind::Int),
        CloseParen,
        Goto,
        Identifier,
        Semicolon,
        CloseBrace,
        While,
        OpenParen,
        Constant(LiteralKind::Int),
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
        Constant(LiteralKind::Int),
        Semicolon,
        Goto,
        Identifier,
        Semicolon,
        For,
        OpenParen,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Less,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        CloseParen,
        Identifier,
        Colon,
        If,
        OpenParen,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Constant(LiteralKind::Int),
        Semicolon,
        For,
        OpenParen,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Semicolon,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Identifier,
        Colon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        Greater,
        Constant(LiteralKind::Int),
        CloseParen,
        Break,
        Semicolon,
        Goto,
        Identifier,
        Semicolon,
        CloseBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Constant(LiteralKind::Int),
        Semicolon,
        Goto,
        Identifier,
        Semicolon,
        While,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        Identifier,
        Colon,
        OpenBrace,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Constant(LiteralKind::Int),
        Semicolon,
        Goto,
        Identifier,
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Colon,
        Do,
        OpenBrace,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Goto,
        Identifier,
        Semicolon,
        CloseBrace,
        While,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        Identifier,
        Colon,
        While,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        Goto,
        Identifier,
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Colon,
        Break,
        Semicolon,
        CloseBrace,
        Semicolon,
        Goto,
        Identifier,
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Colon,
        For,
        OpenParen,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Less,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        Goto,
        Identifier,
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Colon,
        Continue,
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        While,
        OpenParen,
        Identifier,
        MinusMinus,
        CloseParen,
        Identifier,
        PlusPlus,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        While,
        OpenParen,
        MinusMinus,
        Identifier,
        CloseParen,
        Identifier,
        PlusPlus,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Constant(LiteralKind::Int),
        Semicolon,
        Switch,
        OpenParen,
        Identifier,
        CloseParen,
        OpenBrace,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        For,
        OpenParen,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Less,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Identifier,
        Equal,
        Identifier,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Break,
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Default,
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Constant(LiteralKind::Int),
        Semicolon,
        For,
        OpenParen,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Less,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        PlusPlus,
        CloseParen,
        OpenBrace,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Switch,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Constant(LiteralKind::Int),
        Semicolon,
        Switch,
        OpenParen,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Identifier,
        Equal,
        Identifier,
        Star,
        Constant(LiteralKind::Int),
        Semicolon,
        Break,
        Semicolon,
        Default,
        Colon,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Constant(LiteralKind::Int),
        Semicolon,
        Switch,
        OpenParen,
        Identifier,
        CloseParen,
        OpenBrace,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Break,
        Semicolon,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Break,
        Semicolon,
        CloseBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Switch,
        OpenParen,
        Identifier,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        OpenParen,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Identifier,
        Semicolon,
        CloseBrace,
        Return,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Constant(LiteralKind::Int),
        Semicolon,
        Switch,
        OpenParen,
        Identifier,
        CloseParen,
        OpenBrace,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Break,
        Semicolon,
        Default,
        Colon,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Constant(LiteralKind::Int),
        Semicolon,
        Switch,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Default,
        Colon,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Return,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Switch,
        OpenParen,
        Identifier,
        Plus,
        Identifier,
        CloseParen,
        OpenBrace,
        Default,
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Constant(LiteralKind::Int),
        Semicolon,
        Switch,
        OpenParen,
        Identifier,
        CloseParen,
        Default,
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Constant(LiteralKind::Int),
        Semicolon,
        Switch,
        OpenParen,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        CloseBrace,
        Switch,
        OpenParen,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
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
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Switch,
        OpenParen,
        Identifier,
        Question,
        Identifier,
        Colon,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Constant(LiteralKind::Int),
        Semicolon,
        Goto,
        Identifier,
        Semicolon,
        Switch,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Colon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        For,
        OpenParen,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Less,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Switch,
        OpenParen,
        Identifier,
        CloseParen,
        OpenBrace,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Break,
        Semicolon,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Identifier,
        Equal,
        Identifier,
        Star,
        Constant(LiteralKind::Int),
        Semicolon,
        Break,
        Semicolon,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Identifier,
        Equal,
        Identifier,
        Star,
        Constant(LiteralKind::Int),
        Semicolon,
        Break,
        Semicolon,
        Default,
        Colon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Switch,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        If,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Break,
        Semicolon,
        CloseBrace,
        Default,
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Switch,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Else,
        OpenBrace,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Break,
        Semicolon,
        CloseBrace,
        Default,
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Switch,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        For,
        OpenParen,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Less,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Break,
        Semicolon,
        Default,
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        CloseBrace,
        Return,
        OpenParen,
        Identifier,
        AmpersandAmpersand,
        Identifier,
        AmpersandAmpersand,
        Identifier,
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Constant(LiteralKind::Int),
        Semicolon,
        Switch,
        OpenParen,
        Identifier,
        CloseParen,
        OpenBrace,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Switch,
        OpenParen,
        Identifier,
        CloseParen,
        OpenBrace,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Default,
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Default,
        Colon,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Switch,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        OpenBrace,
        Switch,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Default,
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        CloseBrace,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Default,
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Constant(LiteralKind::Int),
        Semicolon,
        Switch,
        OpenParen,
        Identifier,
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Constant(LiteralKind::Int),
        Semicolon,
        Switch,
        OpenParen,
        Identifier,
        CloseParen,
        OpenBrace,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Constant(LiteralKind::Int),
        Semicolon,
        Switch,
        OpenParen,
        Identifier,
        CloseParen,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Switch,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        For,
        OpenParen,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Less,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Identifier,
        Percent,
        Constant(LiteralKind::Int),
        CloseParen,
        Continue,
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Constant(LiteralKind::Int),
        Semicolon,
        For,
        OpenParen,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Less,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Switch,
        OpenParen,
        Identifier,
        Percent,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Continue,
        Semicolon,
        Default,
        Colon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        CloseBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Semicolon,
        For,
        OpenParen,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        GreaterEqual,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        Identifier,
        Equal,
        Identifier,
        Slash,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        For,
        OpenParen,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        If,
        OpenParen,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_8_valid_for_absent_post() {
    let src = r#"
        int main(void) {
            int a = -2147;
            for (; a % 5 != 0;) {
                a = a + 1;
            }
            return a % 5 || a > 0;
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
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        For,
        OpenParen,
        Semicolon,
        Identifier,
        Percent,
        Constant(LiteralKind::Int),
        BangEqual,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseParen,
        OpenBrace,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Identifier,
        Percent,
        Constant(LiteralKind::Int),
        PipePipe,
        Identifier,
        Greater,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Constant(LiteralKind::Int),
        Semicolon,
        For,
        OpenParen,
        Int,
        Identifier,
        Equal,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        LessEqual,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        CloseParen,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        For,
        OpenParen,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Greater,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Identifier,
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Semicolon,
        CloseBrace,
        Return,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        For,
        OpenParen,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Less,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Identifier,
        Semicolon,
        CloseBrace,
        Return,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Constant(LiteralKind::Int),
        Semicolon,
        While,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        Greater,
        Constant(LiteralKind::Int),
        CloseParen,
        Break,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        While,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Identifier,
        Equal,
        Identifier,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        Less,
        Constant(LiteralKind::Int),
        CloseParen,
        Break,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        Equal,
        Identifier,
        EqualEqual,
        Minus,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Do,
        OpenBrace,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        LessEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Continue,
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        GreaterEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Continue,
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        While,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        Return,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Constant(LiteralKind::Int),
        Semicolon,
        For,
        OpenParen,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Less,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        CloseParen,
        For,
        OpenParen,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Less,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        CloseParen,
        If,
        OpenParen,
        OpenParen,
        Identifier,
        Slash,
        Constant(LiteralKind::Int),
        CloseParen,
        Star,
        Constant(LiteralKind::Int),
        EqualEqual,
        Identifier,
        CloseParen,
        Break,
        Semicolon,
        Else,
        Identifier,
        Equal,
        Identifier,
        Plus,
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
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        While,
        OpenParen,
        Identifier,
        GreaterEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Identifier,
        Semicolon,
        While,
        OpenParen,
        Identifier,
        LessEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        Percent,
        Constant(LiteralKind::Int),
        CloseParen,
        Continue,
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Identifier,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        While,
        OpenParen,
        Identifier,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Minus,
        Identifier,
        Semicolon,
        While,
        OpenParen,
        Identifier,
        CloseParen,
        OpenBrace,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        CloseBrace,
        Return,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Constant(LiteralKind::Int),
        Semicolon,
        For,
        OpenParen,
        Semicolon,
        Semicolon,
        CloseParen,
        OpenBrace,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        Greater,
        Constant(LiteralKind::Int),
        CloseParen,
        Break,
        Semicolon,
        CloseBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
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
        Constant(LiteralKind::Int),
        Semicolon,
        While,
        OpenParen,
        Identifier,
        Less,
        Constant(LiteralKind::Int),
        CloseParen,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_9_valid_arguments_in_registers_dont_clobber_edx() {
    let src = r#"
        int x(int a, int b, int c, int d, int e, int f) {
            return a == 1 && b == 2 && c == 3 && d == 4 && e == 5 && f == 6;
        }
        int main(void) {
            int a = 4;
            return x(1, 2, 3, 4, 5, 24 / a);
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Slash,
        Identifier,
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_9_valid_arguments_in_registers_expression_args() {
    let src = r#"
        int sub(int a, int b) {
            return a - b;
        }
        int main(void) {
            int sum = sub(1 + 2, 1);
            return sum;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Minus,
        Identifier,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        Plus,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
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
fn test_chapter_9_valid_arguments_in_registers_fibonacci() {
    let src = r#"
        int fib(int n) {
            if (n == 0 || n == 1) {
                return n;
            } else {
                return fib(n - 1) + fib(n - 2);
            }
        }
        int main(void) {
            int n = 6;
            return fib(n);
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        PipePipe,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Else,
        OpenBrace,
        Return,
        Identifier,
        OpenParen,
        Identifier,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        Plus,
        Identifier,
        OpenParen,
        Identifier,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        OpenParen,
        Identifier,
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_9_valid_arguments_in_registers_forward_decl_multi_arg() {
    let src = r#"
        int foo(int a, int b);
        int main(void) {
            return foo(2, 1);
        }
        int foo(int x, int y){
            return x - y;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Minus,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_9_valid_arguments_in_registers_hello_world() {
    let src = r#"
        int putchar(int c);
        int main(void) {
            putchar(72);
            putchar(101);
            putchar(108);
            putchar(108);
            putchar(111);
            putchar(44);
            putchar(32);
            putchar(87);
            putchar(111);
            putchar(114);
            putchar(108);
            putchar(100);
            putchar(33);
            putchar(10);
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_9_valid_arguments_in_registers_param_shadows_local_var() {
    let src = r#"
        int main(void) {
            int a = 10;
            int f(int a);
            return f(a);
        }
        int f(int a) {
            return a * 2;
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
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        CloseParen,
        Semicolon,
        Return,
        Identifier,
        OpenParen,
        Identifier,
        CloseParen,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Star,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_9_valid_arguments_in_registers_parameter_shadows_function() {
    let src = r#"
        int a(void) {
            return 1;
        }
        int b(int a) {
            return a;
        }
        int main(void) {
            return a() + b(2);
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
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        OpenParen,
        CloseParen,
        Plus,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_9_valid_arguments_in_registers_parameter_shadows_own_function() {
    let src = r#"
        int a(int a) {
            return a * 2;
        }
        int main(void) {
            return a(1);
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Star,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_9_valid_arguments_in_registers_parameters_are_preserved() {
    let src = r#"
        int g(int w, int x, int y, int z) {
            if (w == 2 && x == 4 && y == 6 && z == 8)
                return 1;
            return 0;
        }
        int f(int a, int b, int c, int d) {
            int result = g(a * 2, b * 2, c * 2, d * 2);
            return (result == 1 && a == 1 && b == 2 && c == 3 && d == 4);
        }
        int main(void) {
            return f(1, 2, 3, 4);
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Identifier,
        OpenParen,
        Identifier,
        Star,
        Constant(LiteralKind::Int),
        Comma,
        Identifier,
        Star,
        Constant(LiteralKind::Int),
        Comma,
        Identifier,
        Star,
        Constant(LiteralKind::Int),
        Comma,
        Identifier,
        Star,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        Return,
        OpenParen,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_9_valid_arguments_in_registers_single_arg() {
    let src = r#"
        int twice(int x){
            return 2 * x;
        }
        int main(void) {
            return twice(3);
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Star,
        Identifier,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_9_valid_extra_credit_compound_assign_function_result() {
    let src = r#"
        int foo(void) {
            return 2;
        }
        int main(void) {
            int x = 3;
            x -= foo();
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
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        MinusEqual,
        Identifier,
        OpenParen,
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
fn test_chapter_9_valid_extra_credit_dont_clobber_ecx() {
    let src = r#"
        int x(int a, int b, int c, int d, int e, int f) {
            return a == 1 && b == 2 && c == 3 && d == 4 && e == 5 && f == 6;
        }
        int main(void) {
            int a = 4;
            return x(1, 2, 3, 4, 5, 24 >> (a / 2));
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        GreaterGreater,
        OpenParen,
        Identifier,
        Slash,
        Constant(LiteralKind::Int),
        CloseParen,
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_9_valid_extra_credit_goto_label_multiple_functions() {
    let src = r#"
        
        int foo(void) {
            goto label;
            return 0;
            label:
                return 5;
        }
        int main(void) {
            goto label;
            return 0;
            label:
                return foo();
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Goto,
        Identifier,
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Goto,
        Identifier,
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Colon,
        Return,
        Identifier,
        OpenParen,
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_9_valid_extra_credit_goto_shared_name() {
    let src = r#"
        int foo(void) {
            goto foo;
            return 0;
            foo:
                return 1;
        }
        int main(void) {
            return foo();
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Goto,
        Identifier,
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        OpenParen,
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_9_valid_extra_credit_label_naming_scheme() {
    let src = r#"
        int main(void) {
            _label:
            label_:
            return 0;
        }
        int main_(void) {
            label:
            return 0;
        }
        int _main(void) {
            label: return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Identifier,
        Colon,
        Identifier,
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Identifier,
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Identifier,
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_9_valid_libraries_addition() {
    let src = r#"
        int add(int x, int y) {
            return x + y;
        }
    "#;
    let expected = vec![
        Int, Identifier, OpenParen, Int, Identifier, Comma, Int, Identifier, CloseParen, OpenBrace,
        Return, Identifier, Plus, Identifier, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_9_valid_libraries_addition_client() {
    let src = r#"
        int add(int x, int y);
        int main(void) {
            return add(1, 2);
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_9_valid_libraries_many_args() {
    let src = r#"
        int fib(int n) {
            if (n == 0 || n == 1) {
                return n;
            } else {
                return fib(n - 1) + fib(n - 2);
            }
        }
        int multiply_many_args(int a, int b, int c, int d, int e, int f, int g, int h) {
            return a * b * c * d * e * f * fib(g) * fib(h);
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        PipePipe,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Else,
        OpenBrace,
        Return,
        Identifier,
        OpenParen,
        Identifier,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        Plus,
        Identifier,
        OpenParen,
        Identifier,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Star,
        Identifier,
        Star,
        Identifier,
        Star,
        Identifier,
        Star,
        Identifier,
        Star,
        Identifier,
        Star,
        Identifier,
        OpenParen,
        Identifier,
        CloseParen,
        Star,
        Identifier,
        OpenParen,
        Identifier,
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_9_valid_libraries_many_args_client() {
    let src = r#"
        int fib(int a);
        int multiply_many_args(int a, int b, int c, int d, int e, int f, int g, int h);
        int main(void) {
            int x = fib(4);
            int seven = 7;
            int eight = fib(6);
            int y = multiply_many_args(x, 2, 3, 4, 5, 6, seven, eight);
            if (x != 3) {
                return 1;
            }
            if (y != 589680) {
                return 2;
            }
            return x + (y % 256);
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        OpenParen,
        Identifier,
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Identifier,
        Comma,
        Identifier,
        CloseParen,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Identifier,
        Plus,
        OpenParen,
        Identifier,
        Percent,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_9_valid_libraries_no_function_calls_division() {
    let src = r#"
        int f(int a, int b, int c, int d) {
            int x = a / b;
            if (a == 10 && b == 2 && c == 100 && d == 4 && x == 5)
                return 1;
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Identifier,
        Slash,
        Identifier,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_9_valid_libraries_no_function_calls_division_client() {
    let src = r#"
        int f(int a, int b, int c, int d);
        int main(void) {
            return f(10, 2, 100, 4);
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_9_valid_libraries_no_function_calls_local_stack_variables() {
    let src = r#"
        
        int f(int reg1, int reg2, int reg3, int reg4, int reg5, int reg6,
            int stack1, int stack2, int stack3) {
            int x = 10;
            if (reg1 == 1 && reg2 == 2 && reg3 == 3 && reg4 == 4 && reg5 == 5
                && reg6 == 6 && stack1 == -1 && stack2 == -2 && stack3 == -3
                && x == 10) {
                stack2 = 100;
                return stack2;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Minus,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Minus,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Minus,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_9_valid_libraries_no_function_calls_local_stack_variables_client() {
    let src = r#"
        int f(int reg1, int reg2, int reg3, int reg4, int reg5, int reg6,
            int stack1, int stack2, int stack3);
        int main(void) {
            return f(1, 2, 3, 4, 5, 6, -1, -2, -3);
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Minus,
        Constant(LiteralKind::Int),
        Comma,
        Minus,
        Constant(LiteralKind::Int),
        Comma,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_9_valid_libraries_system_call() {
    let src = r#"
        int putchar(int c);
        int incr_and_print(int b) {
            return putchar(b + 2);
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        OpenParen,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_9_valid_libraries_system_call_client() {
    let src = r#"
        int incr_and_print(int c);
        int main(void) {
            incr_and_print(70);
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_9_valid_no_arguments_forward_decl() {
    let src = r#"
        int foo(void);
        int main(void) {
            return foo();
        }
        int foo(void) {
            return 3;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        OpenParen,
        CloseParen,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_9_valid_no_arguments_function_shadows_variable() {
    let src = r#"
        int main(void) {
            int foo = 3;
            int bar = 4;
            if (foo + bar > 0) {
                int foo(void);
                bar = foo();
            }
            return foo + bar;
        }
        int foo(void) {
            return 8;
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
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        Plus,
        Identifier,
        Greater,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        OpenParen,
        CloseParen,
        Semicolon,
        CloseBrace,
        Return,
        Identifier,
        Plus,
        Identifier,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_9_valid_no_arguments_multiple_declarations() {
    let src = r#"
        int main(void) {
            int f(void);
            int f(void);
            return f();
        }
        int f(void) {
            return 3;
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
        OpenParen,
        Void,
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        Semicolon,
        Return,
        Identifier,
        OpenParen,
        CloseParen,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_9_valid_no_arguments_no_return_value() {
    let src = r#"
        int foo(void) {
            int x = 1;
        }
        int main(void) {
            foo();
            return 3;
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
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Identifier,
        OpenParen,
        CloseParen,
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_9_valid_no_arguments_precedence() {
    let src = r#"
        int three(void) {
            return 3;
        }
        int main(void) {
            return !three();
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
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Bang,
        Identifier,
        OpenParen,
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_9_valid_no_arguments_use_function_in_expression() {
    let src = r#"
        int bar(void) {
            return 9;
        }
        int foo(void) {
            return 2 * bar();
        }
        int main(void) {
            return foo() + bar() / 3;
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
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Star,
        Identifier,
        OpenParen,
        CloseParen,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        OpenParen,
        CloseParen,
        Plus,
        Identifier,
        OpenParen,
        CloseParen,
        Slash,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_9_valid_no_arguments_variable_shadows_function() {
    let src = r#"
        int main(void) {
            int foo(void);
            int x = foo();
            if (x > 0) {
                int foo = 3;
                x = x + foo;
            }
            return x;
        }
        int foo(void) {
            return 4;
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
        OpenParen,
        Void,
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        OpenParen,
        CloseParen,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        Greater,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Identifier,
        Semicolon,
        CloseBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_9_valid_stack_arguments_call_putchar() {
    let src = r#"
        int putchar(int c);
        int foo(int a, int b, int c, int d, int e, int f, int g, int h) {
            putchar(h);
            return a + g;
        }
        int main(void) {
            return foo(1, 2, 3, 4, 5, 6, 7, 65);
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        CloseParen,
        OpenBrace,
        Identifier,
        OpenParen,
        Identifier,
        CloseParen,
        Semicolon,
        Return,
        Identifier,
        Plus,
        Identifier,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_9_valid_stack_arguments_lots_of_arguments() {
    let src = r#"
        int foo(int a, int b, int c, int d, int e, int f, int g, int h) {
            return (a == 1 && b == 2 && c == 3 && d == 4 && e == 5
                    && f == 6 && g == 7 && h == 8);
        }
        int main(void) {
            return foo(1, 2, 3, 4, 5, 6, 7, 8);
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        OpenParen,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_9_valid_stack_arguments_stack_alignment() {
    let src = r#"
        int even_arguments(int a, int b, int c, int d, int e, int f, int g, int h);
        int odd_arguments(int a, int b, int c, int d, int e, int f, int g, int h, int i);
        int main(void) {
            int x = 3;
            even_arguments(1, 2, 3, 4, 5, 6, 7, 8);
            odd_arguments(1, 2, 3, 4, 5, 6, 7, 8, 9);
            return x;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
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
fn test_chapter_9_valid_stack_arguments_test_for_memory_leaks() {
    let src = r#"
        int lots_of_args(int a, int b, int c, int d, int e, int f, int g, int h, int i, int j, int k, int l, int m, int n, int o) {
            return l + o;
        }
        int main(void) {
            int ret = 0;
            for (int i = 0; i < 10000000; i = i + 1) {
                ret = lots_of_args(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, ret, 13, 14, 15);
            }
            return ret == 150000000;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Plus,
        Identifier,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        For,
        OpenParen,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Less,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Identifier,
        Equal,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Identifier,
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
        Return,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_10_valid_distinct_local_and_extern() {
    let src = r#"
        int a = 5;
        int return_a(void) {
            return a;
        }
        int main(void) {
            int a = 3;
            {
                extern int a;
                if (a != 5)
                    return 1;
                a = 4;
            }
            return a + return_a();
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        OpenBrace,
        Extern,
        Int,
        Identifier,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Identifier,
        Plus,
        Identifier,
        OpenParen,
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_10_valid_extern_block_scope_variable() {
    let src = r#"
        int main(void) {
            int outer = 1;
            int foo = 0;
            if (outer) {
                extern int foo;
                extern int foo;
                return foo;
            }
            return 0;
        }
        int foo = 3;
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
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        CloseParen,
        OpenBrace,
        Extern,
        Int,
        Identifier,
        Semicolon,
        Extern,
        Int,
        Identifier,
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_10_valid_extra_credit_bitwise_ops_file_scope_vars() {
    let src = r#"
        int x = 1;
        int y = 0;
        int main(void) {
            y = -1;
            x = (x << 1) | 1;
            if (x != 3) {
                return 1;
            }
            y = ((y & -5) ^ 12) >> 2;
            if (y != -3) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Identifier,
        Equal,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        OpenParen,
        Identifier,
        LessLess,
        Constant(LiteralKind::Int),
        CloseParen,
        Pipe,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        OpenParen,
        OpenParen,
        Identifier,
        Ampersand,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        Circumflex,
        Constant(LiteralKind::Int),
        CloseParen,
        GreaterGreater,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_10_valid_extra_credit_compound_assignment_static_var() {
    let src = r#"
        int f(void) {
            static int i = 0;
            static int j = 0;
            static int k = 1;
            static int l = 48;
            i += 1;
            j -= i;
            k *= j;
            l /= 2;
            if (i != 3) {
                return 1;
            }
            if (j != -6) {
                return 2;
            }
            if (k != -18) {
                return 3;
            }
            if (l != 6) {
                return 4;
            }
            return 0;
        }
        int main(void) {
            f();
            f();
            return f();
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Static,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Static,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Static,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Static,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        PlusEqual,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        MinusEqual,
        Identifier,
        Semicolon,
        Identifier,
        StarEqual,
        Identifier,
        Semicolon,
        Identifier,
        SlashEqual,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Identifier,
        OpenParen,
        CloseParen,
        Semicolon,
        Identifier,
        OpenParen,
        CloseParen,
        Semicolon,
        Return,
        Identifier,
        OpenParen,
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_10_valid_extra_credit_goto_skip_static_initializer() {
    let src = r#"
        int main(void) {
            goto end;
            static int x = 10;
            end:
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
        Goto,
        Identifier,
        Semicolon,
        Static,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Colon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_10_valid_extra_credit_increment_global_vars() {
    let src = r#"
        
        int i = 0;
        int j = 0;
        int incr_i(void){
            if (i == 1) {
                i++;
                ++i;
            }
            return 0;
        }
        int decr_j(void) {
            if (j == -1) {
                j--;
            }
            return 0;
        }
        int main(void) {
            i++ ? 0 : incr_i();
            if (i != 3) {
                return 1;
            }
            --j? decr_j(): 0;
            if (j != -2) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Identifier,
        PlusPlus,
        Semicolon,
        PlusPlus,
        Identifier,
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Identifier,
        EqualEqual,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Identifier,
        MinusMinus,
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Identifier,
        PlusPlus,
        Question,
        Constant(LiteralKind::Int),
        Colon,
        Identifier,
        OpenParen,
        CloseParen,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        MinusMinus,
        Identifier,
        Question,
        Identifier,
        OpenParen,
        CloseParen,
        Colon,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_10_valid_extra_credit_label_file_scope_var_same_name() {
    let src = r#"
        int x;
        int main(void) {
            int x = 10;
            goto x;
            return x;
            {
                extern int x;
            x:
                return x;
            }
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Goto,
        Identifier,
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        OpenBrace,
        Extern,
        Int,
        Identifier,
        Semicolon,
        Identifier,
        Colon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_10_valid_extra_credit_label_static_var_same_name() {
    let src = r#"
        int main(void) {
            static int x = 5;
            goto x;
            x = 0;
        x:
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
        Static,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Goto,
        Identifier,
        Semicolon,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Colon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_10_valid_extra_credit_libraries_same_label_same_fun() {
    let src = r#"
        static int f(void) {
            goto x;
            return 0;
            x:
            return 2;
        }
        int f_caller(void) {
            return f();
        }
    "#;
    let expected = vec![
        Static,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Goto,
        Identifier,
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        OpenParen,
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_10_valid_extra_credit_libraries_same_label_same_fun_client() {
    let src = r#"
        int f(void) {
            goto x;
            return 0;
        x:
            return 1;
        }
        int f_caller(void);
        int main(void) {
            if (f() != 1) {
                return 1;
            }
            if (f_caller() !=
                2) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Goto,
        Identifier,
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        CloseParen,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        CloseParen,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_10_valid_extra_credit_switch_on_extern() {
    let src = r#"
        int update_x(void);
        int main(void) {
            update_x();
            extern int x;
            switch(x) {
                case 0: return 1;
                case 1: return 2;
                case 4: return 0;
                default: return 4;
            }
        }
        int x;
        int update_x(void) {
            x = 4;
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Identifier,
        OpenParen,
        CloseParen,
        Semicolon,
        Extern,
        Int,
        Identifier,
        Semicolon,
        Switch,
        OpenParen,
        Identifier,
        CloseParen,
        OpenBrace,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Default,
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        CloseBrace,
        Int,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_10_valid_extra_credit_switch_skip_extern_decl() {
    let src = r#"
        int main(void) {
            int a = 10;
            switch(a) {
                case 1: return 1;
                extern int x;
                case 2: return 2;
                case 10:
                if (x * 2 == 30) {
                    return 0;
                }
                default: return 5;
            }
            return 6;
        }
        int x = 15;
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
        Constant(LiteralKind::Int),
        Semicolon,
        Switch,
        OpenParen,
        Identifier,
        CloseParen,
        OpenBrace,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Extern,
        Int,
        Identifier,
        Semicolon,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        If,
        OpenParen,
        Identifier,
        Star,
        Constant(LiteralKind::Int),
        EqualEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Default,
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_10_valid_extra_credit_switch_skip_static_initializer() {
    let src = r#"
        int a = 3;
        int main(void) {
            switch (a) {
                case 1:;
                    static int x = 10;
                    x = 0;
                case 3:
                    return x;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Switch,
        OpenParen,
        Identifier,
        CloseParen,
        OpenBrace,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Semicolon,
        Static,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_10_valid_libraries_external_linkage_function() {
    let src = r#"
        extern int sum(int a, int b);
        int sum(int i, int j) {
            return i + j;
        }
        int sum(int x, int y);
    "#;
    let expected = vec![
        Extern, Int, Identifier, OpenParen, Int, Identifier, Comma, Int, Identifier, CloseParen,
        Semicolon, Int, Identifier, OpenParen, Int, Identifier, Comma, Int, Identifier, CloseParen,
        OpenBrace, Return, Identifier, Plus, Identifier, Semicolon, CloseBrace, Int, Identifier,
        OpenParen, Int, Identifier, Comma, Int, Identifier, CloseParen, Semicolon,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_10_valid_libraries_external_linkage_function_client() {
    let src = r#"
        int add_one_and_two(void) {
            extern int sum(int a, int b);
            int sum(int a, int b);
            return sum(1, 2);
        }
        extern int sum(int x, int y);
        int sum(int x, int y);
        int add_three_and_four(void) {
            int f = 3;
            if (f > 2) {
                extern int sum(int one, int two);
                return sum(3, 4);
            }
            return 1;
        }
        int main(void) {
            if (add_three_and_four() != 7)
                return 1;
            if (add_one_and_two() != 3)
                return 1;
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Extern,
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        CloseParen,
        Semicolon,
        Return,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
        Extern,
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        Greater,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Extern,
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        CloseParen,
        Semicolon,
        Return,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        CloseParen,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        CloseParen,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_10_valid_libraries_external_tentative_var() {
    let src = r#"
        
        int x;
        int read_x(void) {
            return x;
        }
    "#;
    let expected = vec![
        Int, Identifier, Semicolon, Int, Identifier, OpenParen, Void, CloseParen, OpenBrace,
        Return, Identifier, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_10_valid_libraries_external_tentative_var_client() {
    let src = r#"
        int read_x(void);
        int main(void) {
            extern int x;
            if (x != 0)
                return 1;
            x = 3;
            if (read_x() != 3)
                return 1;
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Extern,
        Int,
        Identifier,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        CloseParen,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_10_valid_libraries_external_var_scoping() {
    let src = r#"
        int read_x(void) {
            int x = 4;
            if (x == 4) {
                extern int x;
                return x;
            } else {
                return -1;
            }
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
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Extern,
        Int,
        Identifier,
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Else,
        OpenBrace,
        Return,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_10_valid_libraries_external_var_scoping_client() {
    let src = r#"
        int x = 10;
        int read_x(void);
        int main(void) {
            int x = 0;
            if (x == 0) {
                if (read_x() != 10)
                    return 1;
                extern int x;
                if (x != 10)
                    return 1;
                return 0;
            }
            return 1;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        CloseParen,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Extern,
        Int,
        Identifier,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_10_valid_libraries_external_variable() {
    let src = r#"
        int x;
        extern int x;
        int x;
        int update_x(int new_val) {
            x = new_val;
            return 0;
        }
        int read_x(void) {
            return x;
        }
        int x = 3;
    "#;
    let expected = vec![
        Int,
        Identifier,
        Semicolon,
        Extern,
        Int,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        CloseParen,
        OpenBrace,
        Identifier,
        Equal,
        Identifier,
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_10_valid_libraries_external_variable_client() {
    let src = r#"
        int update_x(int new_val);
        int read_x(void);
        extern int x;
        int main(void) {
            if (x != 3)
                return 1;
            if (read_x() != 3)
                return 1;
            x = 4;
            if (x != 4)
                return 1;
            if (read_x() != 4)
                return 1;
            update_x(5);
            if (x != 5)
                return 1;
            if (read_x() != 5)
                return 1;
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        Semicolon,
        Extern,
        Int,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        CloseParen,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        CloseParen,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        CloseParen,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_10_valid_libraries_internal_hides_external_linkage() {
    let src = r#"
        int x = 10;
        int read_x(void){
            return x;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_10_valid_libraries_internal_hides_external_linkage_client() {
    let src = r#"
        static int x = 1;
        int read_internal_x(void);
        int read_x(void);
        int main(void) {
            extern int x;
            if (x != 1)
                return 1;
            x = 2;
            if (read_internal_x() != 2)
                return 1;
            if (read_x() != 10)
                return 1;
            return 0;
        }
        extern int x;
        int read_internal_x(void) {
            return x;
        }
    "#;
    let expected = vec![
        Static,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Extern,
        Int,
        Identifier,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        CloseParen,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        CloseParen,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Extern,
        Int,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_10_valid_libraries_internal_linkage_function() {
    let src = r#"
        
        static int my_fun(void);
        int call_static_my_fun(void) {
            return my_fun();
        }
        int call_static_my_fun_2(void) {
            int my_fun(void);
            return my_fun();
        }
        extern int my_fun(void);
        static int my_fun(void);
        int my_fun(void) {
            static int i = 0;
            i = i + 1;
            return i;
        }
    "#;
    let expected = vec![
        Static,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        OpenParen,
        CloseParen,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        Semicolon,
        Return,
        Identifier,
        OpenParen,
        CloseParen,
        Semicolon,
        CloseBrace,
        Extern,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        Semicolon,
        Static,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Static,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_10_valid_libraries_internal_linkage_function_client() {
    let src = r#"
        extern int my_fun(void);
        int call_static_my_fun(void);
        int call_static_my_fun_2(void);
        int main(void) {
            if (call_static_my_fun() != 1)
                return 1;
            if (my_fun() != 100)
                return 1;
            if (call_static_my_fun_2() != 2)
                return 1;
            return 0;
        }
        int my_fun(void) {
            return 100;
        }
    "#;
    let expected = vec![
        Extern,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        CloseParen,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        CloseParen,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        CloseParen,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_10_valid_libraries_internal_linkage_var() {
    let src = r#"
        static int x;
        int read_x(void) {
            return x;
        }
        int update_x(int new_val) {
            extern int x;
            x = new_val;
            return 0;
        }
        extern int x;
        static int x = 5;
        static int x;
    "#;
    let expected = vec![
        Static,
        Int,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        CloseParen,
        OpenBrace,
        Extern,
        Int,
        Identifier,
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Extern,
        Int,
        Identifier,
        Semicolon,
        Static,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Static,
        Int,
        Identifier,
        Semicolon,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_10_valid_libraries_internal_linkage_var_client() {
    let src = r#"
        static int x;
        static int x;
        int read_x(void);
        int update_x(int x);
        int main(void) {
            if (x != 0)
                return 1;
            if (read_x() != 5)
                return 1;
            extern int x;
            update_x(10);
            if (read_x() != 10)
                return 1;
            if (x != 0)
                return 1;
            x = 20;
            if (x != 20)
                return 1;
            if (read_x() != 10)
                return 1;
            return 0;
        }
        static int x;
    "#;
    let expected = vec![
        Static,
        Int,
        Identifier,
        Semicolon,
        Static,
        Int,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        CloseParen,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Extern,
        Int,
        Identifier,
        Semicolon,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        CloseParen,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        CloseParen,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Static,
        Int,
        Identifier,
        Semicolon,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_10_valid_multiple_static_file_scope_vars() {
    let src = r#"
        static int foo;
        int main(void) {
            return foo;
        }
        extern int foo;
        static int foo = 4;
    "#;
    let expected = vec![
        Static,
        Int,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Extern,
        Int,
        Identifier,
        Semicolon,
        Static,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_10_valid_multiple_static_local() {
    let src = r#"
        int foo(void) {
            static int a = 3;
            a = a * 2;
            return a;
        }
        int bar(void) {
            static int a = 4;
            a = a + 1;
            return a;
        }
        int main(void) {
            return foo() + bar() + foo() + bar();
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Static,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Star,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Static,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        OpenParen,
        CloseParen,
        Plus,
        Identifier,
        OpenParen,
        CloseParen,
        Plus,
        Identifier,
        OpenParen,
        CloseParen,
        Plus,
        Identifier,
        OpenParen,
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_10_valid_push_arg_on_page_boundary() {
    let src = r#"
        extern int zed;
        int foo(int a, int b, int c, int d, int e, int f, int g) {
            return g + 1;
        }
        int main(void) {
            return foo(0, 0, 0, 0, 0, 0, zed);
        }
    "#;
    let expected = vec![
        Extern,
        Int,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Identifier,
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_10_valid_shadow_static_local_var() {
    let src = r#"
        int i;
        int update_static_or_global(int update_global, int new_val)
        {
            static int i;
            if (update_global)
            {
                extern int i;
                i = new_val;
            }
            else
                i = new_val;
            return i;
        }
        int main(void)
        {
            if (i != 0)
                return 1;
            int result = update_static_or_global(1, 10);
            if (result != 0)
                return 1;
            if (i != 10)
                return 1;
            result = update_static_or_global(0, 9);
            if (result != 9)
                return 1;
            if (i != 10)
                return 1;
            result = update_static_or_global(1, 11);
            if (result != 9)
                return 1;
            if (i != 11)
                return 1;
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        CloseParen,
        OpenBrace,
        Static,
        Int,
        Identifier,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        CloseParen,
        OpenBrace,
        Extern,
        Int,
        Identifier,
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Semicolon,
        CloseBrace,
        Else,
        Identifier,
        Equal,
        Identifier,
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_10_valid_static_local_multiple_scopes() {
    let src = r#"
        int putchar (int ch);
        int print_letters(void) {
            static int i = 65;
            putchar(i);
            {
                i = i + 1;
                static int i = 97;
                putchar(i);
                i = i + 1;
            }
            putchar(10);
            return 0;
        }
        int main(void) {
            for (int i = 0; i < 26; i = i + 1)
                print_letters();
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Static,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        OpenParen,
        Identifier,
        CloseParen,
        Semicolon,
        OpenBrace,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        Static,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        OpenParen,
        Identifier,
        CloseParen,
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        For,
        OpenParen,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Less,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        CloseParen,
        Identifier,
        OpenParen,
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_10_valid_static_local_uninitialized() {
    let src = r#"
        int foo(void) {
            static int x;
            x = x + 1;
            return x;
        }
        int main(void) {
            int ret;
            for (int i = 0; i < 4; i = i + 1)
                ret = foo();
            return ret;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Static,
        Int,
        Identifier,
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Semicolon,
        For,
        OpenParen,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Less,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        CloseParen,
        Identifier,
        Equal,
        Identifier,
        OpenParen,
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
fn test_chapter_10_valid_static_recursive_call() {
    let src = r#"
        int putchar (int ch);
        int print_alphabet(void) {
            static int count = 0;
            putchar(count + 65);
            count = count + 1;
            if (count < 26) {
                print_alphabet();
            }
            return count;
        }
        int main(void) {
            print_alphabet();
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Static,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        OpenParen,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        Less,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Identifier,
        OpenParen,
        CloseParen,
        Semicolon,
        CloseBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Identifier,
        OpenParen,
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_10_valid_static_then_extern() {
    let src = r#"
        static int foo = 3;
        int main(void) {
            return foo;
        }
        extern int foo;
    "#;
    let expected = vec![
        Static,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Extern,
        Int,
        Identifier,
        Semicolon,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_10_valid_static_variables_in_expressions() {
    let src = r#"
        int main(void) {
            static int i = 2;
            static int j = 3;
            int cmp = i < j;
            if (!cmp)
                return 1;
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Static,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Static,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        Less,
        Identifier,
        Semicolon,
        If,
        OpenParen,
        Bang,
        Identifier,
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_10_valid_tentative_definition() {
    let src = r#"
        extern int foo;
        int foo;
        int foo;
        int main(void) {
            for (int i = 0; i < 5; i = i + 1)
                foo = foo + 1;
            return foo;
        }
        int foo;
    "#;
    let expected = vec![
        Extern,
        Int,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        For,
        OpenParen,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Less,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        CloseParen,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        Semicolon,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_10_valid_type_before_storage_class() {
    let src = r#"
        int static foo(void) {
            return 3;
        }
        int static bar = 4;
        int main(void) {
            int extern foo(void);
            int extern bar;
            return foo() + bar;
        }
    "#;
    let expected = vec![
        Int,
        Static,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Int,
        Static,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Extern,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        Semicolon,
        Int,
        Extern,
        Identifier,
        Semicolon,
        Return,
        Identifier,
        OpenParen,
        CloseParen,
        Plus,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
#[should_panic]
fn test_chapter_11_invalid_lex_invalid_suffix() {
    tokenize(
        r#"
        int main(void) {
            return 0lL;
        }
    "#,
    );
}

#[test]
#[should_panic]
fn test_chapter_11_invalid_lex_invalid_suffix2() {
    tokenize(
        r#"
        int main(void) {
            return 0LLL;
        }
    "#,
    );
}

#[test]
fn test_chapter_11_valid_explicit_casts_sign_extend() {
    let src = r#"
        long sign_extend(int i, long expected) {
            long extended = (long) i;
            return (extended == expected);
        }
        int main(void) {
            if (!sign_extend(10, 10l)) {
                return 1;
            }
            if (!sign_extend(-10, -10l)) {
                return 2;
            }
            long l = (long) 100;
            if (l != 100l) {
                return 3;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Long,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        Comma,
        Long,
        Identifier,
        CloseParen,
        OpenBrace,
        Long,
        Identifier,
        Equal,
        OpenParen,
        Long,
        CloseParen,
        Identifier,
        Semicolon,
        Return,
        OpenParen,
        Identifier,
        EqualEqual,
        Identifier,
        CloseParen,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Long),
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        Minus,
        Constant(LiteralKind::Int),
        Comma,
        Minus,
        Constant(LiteralKind::Long),
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Long,
        Identifier,
        Equal,
        OpenParen,
        Long,
        CloseParen,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Long),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_11_valid_explicit_casts_truncate() {
    let src = r#"
        int truncate(long l, int expected) {
            int result = (int) l;
            return (result == expected);
        }
        int main(void)
        {
            if (!truncate(10l, 10)) {
                return 1;
            }
            if (!truncate(-10l, -10)) {
                return 2;
            }
            if (!truncate(17179869189l,
                          5)) {
                return 3;
            }
            if (!truncate(-17179869179l,
                          5l)) {
                return 4;
            }
            int i = (int)17179869189l;
            if (i != 5)
                return 5;
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Long,
        Identifier,
        Comma,
        Int,
        Identifier,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        OpenParen,
        Int,
        CloseParen,
        Identifier,
        Semicolon,
        Return,
        OpenParen,
        Identifier,
        EqualEqual,
        Identifier,
        CloseParen,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Long),
        Comma,
        Constant(LiteralKind::Int),
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        Minus,
        Constant(LiteralKind::Long),
        Comma,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Long),
        Comma,
        Constant(LiteralKind::Int),
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        Minus,
        Constant(LiteralKind::Long),
        Comma,
        Constant(LiteralKind::Long),
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        Equal,
        OpenParen,
        Int,
        CloseParen,
        Constant(LiteralKind::Long),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_11_valid_extra_credit_bitshift() {
    let src = r#"
        int main(void) {
            long l = 137438953472l;
            int shiftcount = 2;
            if (l >> shiftcount != 34359738368l ) {
                return 1;
            }
            if (l << shiftcount != 549755813888 ) {
                return 2;
            }
            if (l << 2 != 549755813888 ) {
                return 3;
            }
            if ((40l << 40) != 43980465111040l) {
                return 4;
            }
            long long_shiftcount = 3l;
            int i_neighbor1 = 0;
            int i = -2147483645;
            int i_neighbor2 = 0;
            if (i >> long_shiftcount != -268435456) {
                return 5;
            }
            i = -1;
            if (i >> 10l != -1) {
                return 6;
            }
            if (i_neighbor1) {
                return 7;
            }
            if (i_neighbor2) {
                return 8;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Long,
        Identifier,
        Equal,
        Constant(LiteralKind::Long),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        GreaterGreater,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Long),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        LessLess,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        LessLess,
        Constant(LiteralKind::Int),
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        OpenParen,
        Constant(LiteralKind::Long),
        LessLess,
        Constant(LiteralKind::Int),
        CloseParen,
        BangEqual,
        Constant(LiteralKind::Long),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Long,
        Identifier,
        Equal,
        Constant(LiteralKind::Long),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        GreaterGreater,
        Identifier,
        BangEqual,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        GreaterGreater,
        Constant(LiteralKind::Long),
        BangEqual,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_11_valid_extra_credit_bitwise_long_op() {
    let src = r#"
        int main(void) {
            long l1 = 71777214294589695l;
            long l2 = -4294967296;
            if ((l1 & l2) != 71777214277877760l ) {
                return 1;
            }
            if ((l1 | l2) != -4278255361 ) {
                return 2;
            }
            if ((l1 ^ l2) != -71777218556133121 ) {
                return 3;
            }
            if ((-1l & 34359738368l) != 34359738368l) {
                return 4;
            }
            if ((0l | 34359738368l) != 34359738368l) {
                return 5;
            }
            if ((34359738368l ^ 137438953472l) != 171798691840l) {
                return 6;
            }
            long l = 4611686018427387903l;
            int i = -1073741824;
            int i2 = -1;
            if ((i & l) != 4611686017353646080l) {
                return 7;
            }
            if ((l | i) != -1) {
                return 8;
            }
            if ((l ^ i) != -4611686017353646081) {
                return 9;
            }
            if ((i2 ^ 4611686018427387903l) != ~4611686018427387903l) {
                return 10;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Long,
        Identifier,
        Equal,
        Constant(LiteralKind::Long),
        Semicolon,
        Long,
        Identifier,
        Equal,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        OpenParen,
        Identifier,
        Ampersand,
        Identifier,
        CloseParen,
        BangEqual,
        Constant(LiteralKind::Long),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        OpenParen,
        Identifier,
        Pipe,
        Identifier,
        CloseParen,
        BangEqual,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        OpenParen,
        Identifier,
        Circumflex,
        Identifier,
        CloseParen,
        BangEqual,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        OpenParen,
        Minus,
        Constant(LiteralKind::Long),
        Ampersand,
        Constant(LiteralKind::Long),
        CloseParen,
        BangEqual,
        Constant(LiteralKind::Long),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        OpenParen,
        Constant(LiteralKind::Long),
        Pipe,
        Constant(LiteralKind::Long),
        CloseParen,
        BangEqual,
        Constant(LiteralKind::Long),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        OpenParen,
        Constant(LiteralKind::Long),
        Circumflex,
        Constant(LiteralKind::Long),
        CloseParen,
        BangEqual,
        Constant(LiteralKind::Long),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Long,
        Identifier,
        Equal,
        Constant(LiteralKind::Long),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        OpenParen,
        Identifier,
        Ampersand,
        Identifier,
        CloseParen,
        BangEqual,
        Constant(LiteralKind::Long),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        OpenParen,
        Identifier,
        Pipe,
        Identifier,
        CloseParen,
        BangEqual,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        OpenParen,
        Identifier,
        Circumflex,
        Identifier,
        CloseParen,
        BangEqual,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        OpenParen,
        Identifier,
        Circumflex,
        Constant(LiteralKind::Long),
        CloseParen,
        BangEqual,
        Tilde,
        Constant(LiteralKind::Long),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_11_valid_extra_credit_compound_assign_to_int() {
    let src = r#"
        int main(void) {
            int i = -20;
            int b = 2147483647;
            int c = -5000000;
            i += 2147483648l;
            if (i != 2147483628) {
                return 1;
            }
            if (b != 2147483647) {
                return 2;
            }
            b /= -34359738367l;
            if (b) {
                return 3;
            }
            if (i != 2147483628) {
                return 4;
            }
            if (c != -5000000) {
                return 5;
            }
            c *= 10000l;
            if (c != 1539607552) {
                return 6;
            }
            return 0;
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
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        PlusEqual,
        Constant(LiteralKind::Long),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        SlashEqual,
        Minus,
        Constant(LiteralKind::Long),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        StarEqual,
        Constant(LiteralKind::Long),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_11_valid_extra_credit_compound_assign_to_long() {
    let src = r#"
        int main(void) {
            long l = -34359738368l;
            int i = -10;
            l -= i;
            if (l != -34359738358l) {
                return 1;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Long,
        Identifier,
        Equal,
        Minus,
        Constant(LiteralKind::Long),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        MinusEqual,
        Identifier,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        Constant(LiteralKind::Long),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_11_valid_extra_credit_compound_bitshift() {
    let src = r#"
        int main(void) {
            int x = 100;
            x <<= 22l;
            if (x != 419430400) {
                return 1;
            }
            if ((x >>= 4l) != 26214400) {
                return 2;
            }
            if (x != 26214400) {
                return 3;
            }
            long l = 12345l;
            if ((l <<= 33) != 106042742538240l) {
                return 4;
            }
            l = -l;
            if ((l >>= 10) != -103557365760l) {
                return 5;
            }
            return 0;
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
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        LessLessEqual,
        Constant(LiteralKind::Long),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        OpenParen,
        Identifier,
        GreaterGreaterEqual,
        Constant(LiteralKind::Long),
        CloseParen,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Long,
        Identifier,
        Equal,
        Constant(LiteralKind::Long),
        Semicolon,
        If,
        OpenParen,
        OpenParen,
        Identifier,
        LessLessEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        BangEqual,
        Constant(LiteralKind::Long),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Minus,
        Identifier,
        Semicolon,
        If,
        OpenParen,
        OpenParen,
        Identifier,
        GreaterGreaterEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        BangEqual,
        Minus,
        Constant(LiteralKind::Long),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_11_valid_extra_credit_compound_bitwise() {
    let src = r#"
        int main(void) {
            long l1 = 71777214294589695l;
            long l2 = -4294967296;
            l1 &= l2;
            if (l1 != 71777214277877760l) {
                return 1;
            }
            l2 |= 100l;
            if (l2 != -4294967196) {
                return 2;
            }
            l1 ^= -9223372036854775807l;
            if (l1 != -9151594822576898047l ) {
                return 3;
            }
            l1 = 4611686018427387903l;
            int i = -1073741824;
            l1 &= i;
            if (l1 != 4611686017353646080l) {
                return 4;
            }
            i = -2147483648l;
            if ((i |= 71777214294589695l) != -2130771713) {
                return 5;
            }
            if (i != -2130771713) {
                return 6;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Long,
        Identifier,
        Equal,
        Constant(LiteralKind::Long),
        Semicolon,
        Long,
        Identifier,
        Equal,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        AmpersandEqual,
        Identifier,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Long),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        PipeEqual,
        Constant(LiteralKind::Long),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        CircumflexEqual,
        Minus,
        Constant(LiteralKind::Long),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        Constant(LiteralKind::Long),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Constant(LiteralKind::Long),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        AmpersandEqual,
        Identifier,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Long),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Minus,
        Constant(LiteralKind::Long),
        Semicolon,
        If,
        OpenParen,
        OpenParen,
        Identifier,
        PipeEqual,
        Constant(LiteralKind::Long),
        CloseParen,
        BangEqual,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_11_valid_extra_credit_increment_long() {
    let src = r#"
        
        int main(void) {
            long x = -9223372036854775807l;
            if (x++ != -9223372036854775807l) {
                return 1;
            }
            if (x != -9223372036854775806l) {
                return 2;
            }
            if (--x != -9223372036854775807l) {
                return 3;
            }
            if (x != -9223372036854775807l) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Long,
        Identifier,
        Equal,
        Minus,
        Constant(LiteralKind::Long),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        PlusPlus,
        BangEqual,
        Minus,
        Constant(LiteralKind::Long),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        Constant(LiteralKind::Long),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        MinusMinus,
        Identifier,
        BangEqual,
        Minus,
        Constant(LiteralKind::Long),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        Constant(LiteralKind::Long),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_11_valid_extra_credit_switch_int() {
    let src = r#"
        int switch_on_int(int i) {
            switch(i) {
                case 5:
                    return 0;
                case 8589934592l:
                    return 1;
                case 34359738367:
                    return 2;
                default:
                    return 3;
            }
        }
        int main(void) {
            if (switch_on_int(5) != 0)
                return 1;
            if (switch_on_int(0) != 1)
                return 2;
            if (switch_on_int(-1) != 2)
                return 3;
            if (switch_on_int(17179869184) != 1)
                return 4;
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        CloseParen,
        OpenBrace,
        Switch,
        OpenParen,
        Identifier,
        CloseParen,
        OpenBrace,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Case,
        Constant(LiteralKind::Long),
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Default,
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_11_valid_extra_credit_switch_long() {
    let src = r#"
        int switch_on_long(long l) {
            switch (l) {
                case 0: return 0;
                case 100: return 1;
                case 8589934592l:
                    return 2;
                default:
                    return -1;
            }
        }
        int main(void) {
            if (switch_on_long(8589934592) != 2)
                return 1;
            if (switch_on_long(100) != 1)
                return 2;
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Long,
        Identifier,
        CloseParen,
        OpenBrace,
        Switch,
        OpenParen,
        Identifier,
        CloseParen,
        OpenBrace,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Case,
        Constant(LiteralKind::Int),
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Case,
        Constant(LiteralKind::Long),
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Default,
        Colon,
        Return,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_11_valid_implicit_casts_common_type() {
    let src = r#"
        long l;
        int i;
        int addition(void) {
            long result = i + l;
            return (result == 2147483663l);
        }
        int division(void) {
            int int_result = l / i;
            return (int_result == 214748364);
        }
        int comparison(void) {
            return (i <= l);
        }
        int conditional(void) {
            long result = 1 ? l : i;
            return (result == 8589934592l);
        }
        int main(void) {
            l = 2147483653;
            i = 10;
            if (!addition()) {
                return 1;
            }
            l = 2147483649l;
            if (!division()) {
                return 2;
            }
            i = -100;
            l = 4294967296;
            if (!comparison()) {
                return 3;
            }
            l = 8589934592l;
            i = 10;
            if (!conditional()) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Long,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Long,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Identifier,
        Semicolon,
        Return,
        OpenParen,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Long),
        CloseParen,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Identifier,
        Slash,
        Identifier,
        Semicolon,
        Return,
        OpenParen,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        OpenParen,
        Identifier,
        LessEqual,
        Identifier,
        CloseParen,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Long,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Question,
        Identifier,
        Colon,
        Identifier,
        Semicolon,
        Return,
        OpenParen,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Long),
        CloseParen,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Constant(LiteralKind::Long),
        Semicolon,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Constant(LiteralKind::Long),
        Semicolon,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_11_valid_implicit_casts_convert_by_assignment() {
    let src = r#"
        int return_truncated_long(long l) {
            return l;
        }
        long return_extended_int(int i) {
            return i;
        }
        int truncate_on_assignment(long l, int expected) {
            int result = l;
            return result == expected;
        }
        int main(void) {
            long result = return_truncated_long(4294967298l);
            if (result != 2l) {
                return 1;
            }
            result = return_extended_int(-10);
            if (result != -10) {
                return 2;
            }
            int i = 4294967298l;
            if (i != 2) {
                return 3;
            }
            if (!truncate_on_assignment(17179869184l, 0)) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Long,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Long,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Long,
        Identifier,
        Comma,
        Int,
        Identifier,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Identifier,
        Semicolon,
        Return,
        Identifier,
        EqualEqual,
        Identifier,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Long,
        Identifier,
        Equal,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Long),
        CloseParen,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Long),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Identifier,
        OpenParen,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Long),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Long),
        Comma,
        Constant(LiteralKind::Int),
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_11_valid_implicit_casts_convert_function_arguments() {
    let src = r#"
        int foo(long a, int b, int c, int d, long e, int f, long g, int h) {
            if (a != -1l)
                return 1;
            if (b != 2)
                return 2;
            if (c != 0)
                return 3;
            if (d != -5)
                return 4;
            if (e != -101l)
                return 5;
            if (f != -123)
                return 6;
            if (g != -10l)
                return 7;
            if (h != 1234)
                return 8;
            return 0;
        }
        int main(void) {
            int a = -1;
            long int b = 4294967298;
            long c = -4294967296;
            long d =
                21474836475;
            int e = -101;
            long f = -123;
            int g = -10;
            long h = -9223372036854774574;
            return foo(a, b, c, d, e, f, g, h);
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Long,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Long,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Long,
        Identifier,
        Comma,
        Int,
        Identifier,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        Constant(LiteralKind::Long),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        Constant(LiteralKind::Long),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        Constant(LiteralKind::Long),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        Long,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Long,
        Identifier,
        Equal,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        Long,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        Long,
        Identifier,
        Equal,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        Long,
        Identifier,
        Equal,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        OpenParen,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Identifier,
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_11_valid_implicit_casts_convert_static_initializer() {
    let src = r#"
        int i = 8589934592l;
        long j = 123456;
        int main(void) {
            if (i != 0) {
                return 1;
            }
            if (j != 123456l) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Long),
        Semicolon,
        Long,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Long),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_11_valid_implicit_casts_long_constants() {
    let src = r#"
        int main(void) {
            if (2147483647l + 2147483647l < 0l) {
                return 1;
            }
            if (17179869184 < 100l) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Constant(LiteralKind::Long),
        Plus,
        Constant(LiteralKind::Long),
        Less,
        Constant(LiteralKind::Long),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Constant(LiteralKind::Int),
        Less,
        Constant(LiteralKind::Long),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_11_valid_libraries_long_args() {
    let src = r#"
        int test_sum(int a, int b, int c, long d, int e, long f, int g, int h, long i) {
            if (d + f < 100l) {
                return 1;
            }
            if (i < 100l)
                return 2;
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Long,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Long,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Long,
        Identifier,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Identifier,
        Plus,
        Identifier,
        Less,
        Constant(LiteralKind::Long),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        Less,
        Constant(LiteralKind::Long),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_11_valid_libraries_long_args_client() {
    let src = r#"
        int test_sum(int a, int b, int c, long d, int e, long f, int g, int h, long i);
        int main(void) {
            return test_sum(0, 0, 0, 34359738368l, 0, 34359738368l, 0, 0, 34359738368l);
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Long,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Long,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Long,
        Identifier,
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Long),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Long),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Long),
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_11_valid_libraries_long_global_var() {
    let src = r#"
        long int l = 8589934592l;
        long return_l(void) {
            return l;
        }
        int return_l_as_int(void) {
            return l;
        }
    "#;
    let expected = vec![
        Long,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Long),
        Semicolon,
        Long,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_11_valid_libraries_long_global_var_client() {
    let src = r#"
        extern long int l;
        long return_l(void);
        int return_l_as_int(void);
        int main(void) {
            if (return_l() != 8589934592l)
                return 1;
            if (return_l_as_int() != 0)
                return 2;
            l = l - 10l;
            if (return_l() != 8589934582l)
                return 3;
            if (return_l_as_int() != -10)
                return 4;
            return 0;
        }
    "#;
    let expected = vec![
        Extern,
        Long,
        Int,
        Identifier,
        Semicolon,
        Long,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        CloseParen,
        BangEqual,
        Constant(LiteralKind::Long),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        CloseParen,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Minus,
        Constant(LiteralKind::Long),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        CloseParen,
        BangEqual,
        Constant(LiteralKind::Long),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        CloseParen,
        BangEqual,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_11_valid_libraries_maintain_stack_alignment() {
    let src = r#"
        long add_variables(long x, long y, int z){
            return x + y + z;
        }
    "#;
    let expected = vec![
        Long, Identifier, OpenParen, Long, Identifier, Comma, Long, Identifier, Comma, Int,
        Identifier, CloseParen, OpenBrace, Return, Identifier, Plus, Identifier, Plus, Identifier,
        Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_11_valid_libraries_maintain_stack_alignment_client() {
    let src = r#"
        long add_variables(long x, long y, int z);
        int main(void) {
            long x = 3;
            long y = 4;
            int z = 5;
            return add_variables(x, y, z);
        }
    "#;
    let expected = vec![
        Long,
        Identifier,
        OpenParen,
        Long,
        Identifier,
        Comma,
        Long,
        Identifier,
        Comma,
        Int,
        Identifier,
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Long,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Long,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Identifier,
        OpenParen,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Identifier,
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_11_valid_libraries_return_long() {
    let src = r#"
        long add(int a, int b) {
            return (long) a + (long) b;
        }
    "#;
    let expected = vec![
        Long, Identifier, OpenParen, Int, Identifier, Comma, Int, Identifier, CloseParen,
        OpenBrace, Return, OpenParen, Long, CloseParen, Identifier, Plus, OpenParen, Long,
        CloseParen, Identifier, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_11_valid_libraries_return_long_client() {
    let src = r#"
        long add(int a, int b);
        int main(void) {
            long a = add(2147483645, 2147483645);
            if (a != 4294967290l) {
                return 1;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Long,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Long,
        Identifier,
        Equal,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Long),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_11_valid_long_expressions_arithmetic_ops() {
    let src = r#"
        long a;
        long b;
        int addition(void) {
            return (a + b == 4294967295l);
        }
        int subtraction(void) {
            return (a - b == -4294967380l);
        }
        int multiplication(void) {
            return (a * 4l == 17179869160l);
        }
        int division(void) {
            b = a / 128l;
            return (b == 33554431l);
        }
        int remaind(void) {
            b = -a % 4294967290l;
            return (b == -5l);
        }
        int complement(void) {
            return (~a == -9223372036854775807l);
        }
        int main(void) {
            a = 4294967290l;
            b = 5l;
            if (!addition()) {
                return 1;
            }
            a = -4294967290l;
            b = 90l;
            if (!subtraction()) {
                return 2;
            }
            a = 4294967290l;
            if (!multiplication()) {
                return 3;
            }
            a = 4294967290l;
            if (!division()) {
                return 4;
            }
            a = 8589934585l;
            if (!remaind()) {
                return 5;
            }
            a = 9223372036854775806l;
            if (!complement()) {
                return 6;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Long,
        Identifier,
        Semicolon,
        Long,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        OpenParen,
        Identifier,
        Plus,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Long),
        CloseParen,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        OpenParen,
        Identifier,
        Minus,
        Identifier,
        EqualEqual,
        Minus,
        Constant(LiteralKind::Long),
        CloseParen,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        OpenParen,
        Identifier,
        Star,
        Constant(LiteralKind::Long),
        EqualEqual,
        Constant(LiteralKind::Long),
        CloseParen,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Identifier,
        Equal,
        Identifier,
        Slash,
        Constant(LiteralKind::Long),
        Semicolon,
        Return,
        OpenParen,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Long),
        CloseParen,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Identifier,
        Equal,
        Minus,
        Identifier,
        Percent,
        Constant(LiteralKind::Long),
        Semicolon,
        Return,
        OpenParen,
        Identifier,
        EqualEqual,
        Minus,
        Constant(LiteralKind::Long),
        CloseParen,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        OpenParen,
        Tilde,
        Identifier,
        EqualEqual,
        Minus,
        Constant(LiteralKind::Long),
        CloseParen,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Identifier,
        Equal,
        Constant(LiteralKind::Long),
        Semicolon,
        Identifier,
        Equal,
        Constant(LiteralKind::Long),
        Semicolon,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Minus,
        Constant(LiteralKind::Long),
        Semicolon,
        Identifier,
        Equal,
        Constant(LiteralKind::Long),
        Semicolon,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Constant(LiteralKind::Long),
        Semicolon,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Constant(LiteralKind::Long),
        Semicolon,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Constant(LiteralKind::Long),
        Semicolon,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Constant(LiteralKind::Long),
        Semicolon,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_11_valid_long_expressions_assign() {
    let src = r#"
        int main(void) {
            long a = 4294967290l;
            long b = 0l;
            b = a;
            return (b == 4294967290l);
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Long,
        Identifier,
        Equal,
        Constant(LiteralKind::Long),
        Semicolon,
        Long,
        Identifier,
        Equal,
        Constant(LiteralKind::Long),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Semicolon,
        Return,
        OpenParen,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Long),
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_11_valid_long_expressions_comparisons() {
    let src = r#"
        long l;
        long l2;
        int compare_constants(void) {
            return 8589934593l > 255l;
        }
        int compare_constants_2(void) {
            return 255l < 8589934593l;
        }
        int l_geq_2_60(void) {
            return (l >= 1152921504606846976l);
        }
        int uint_max_leq_l(void) {
            return (4294967295l <= l);
        }
        int l_eq_l2(void) {
            return (l == l2);
        }
        int main(void) {
            if (!compare_constants()) {
                return 1;
            }
            if (!compare_constants_2()) {
                return 2;
            }
            l = -9223372036854775807l;
            if (l_geq_2_60()) {
                return 3;
            }
            if (uint_max_leq_l()) {
                return 4;
            }
            l = 1152921504606846976l;
            if (!l_geq_2_60()) {
                return 5;
            }
            if (!uint_max_leq_l()) {
                return 6;
            }
            l2 = l;
            if (!l_eq_l2()) {
                return 7;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Long,
        Identifier,
        Semicolon,
        Long,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Long),
        Greater,
        Constant(LiteralKind::Long),
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Long),
        Less,
        Constant(LiteralKind::Long),
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        OpenParen,
        Identifier,
        GreaterEqual,
        Constant(LiteralKind::Long),
        CloseParen,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        OpenParen,
        Constant(LiteralKind::Long),
        LessEqual,
        Identifier,
        CloseParen,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        OpenParen,
        Identifier,
        EqualEqual,
        Identifier,
        CloseParen,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Minus,
        Constant(LiteralKind::Long),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Constant(LiteralKind::Long),
        Semicolon,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Identifier,
        Semicolon,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_11_valid_long_expressions_large_constants() {
    let src = r#"
        long x = 5l;
        int add_large(void) {
            x = x + 4294967290l;
            return (x == 4294967295l);
        }
        int subtract_large(void) {
            x = x - 4294967290l;
            return (x == 5l);
        }
        int multiply_by_large(void) {
            x = x * 4294967290l;
            return (x == 21474836450l);
        }
        int main(void) {
            if (!add_large()) {
                return 1;
            }
            if (!subtract_large()) {
                return 2;
            }
            if (!multiply_by_large()) {
                return 3;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Long,
        Identifier,
        Equal,
        Constant(LiteralKind::Long),
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Long),
        Semicolon,
        Return,
        OpenParen,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Long),
        CloseParen,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Identifier,
        Equal,
        Identifier,
        Minus,
        Constant(LiteralKind::Long),
        Semicolon,
        Return,
        OpenParen,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Long),
        CloseParen,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Identifier,
        Equal,
        Identifier,
        Star,
        Constant(LiteralKind::Long),
        Semicolon,
        Return,
        OpenParen,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Long),
        CloseParen,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_11_valid_long_expressions_logical() {
    let src = r#"
        int not(long l) {
            return !l;
        }
        int if_cond(long l) {
            if (l) {
                return 1;
            }
            return 0;
        }
        int and(long l1, int l2) {
            return l1 && l2;
        }
        int or(int l1, long l2) {
            return l1 || l2;
        }
        int main(void) {
            long l = 1152921504606846976l;
            long zero = 0l;
            if (not(l)) {
                return 1;
            }
            if (!not(zero)) {
                return 2;
            }
            if(!if_cond(l)) {
                return 3;
            }
            if(if_cond(zero)) {
                return 4;
            }
            if (and(zero, 1)) {
                return 5;
            }
            if (!or(1, l)) {
                return 6;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Long,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Bang,
        Identifier,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Long,
        Identifier,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Long,
        Identifier,
        Comma,
        Int,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        AmpersandAmpersand,
        Identifier,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        Comma,
        Long,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        PipePipe,
        Identifier,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Long,
        Identifier,
        Equal,
        Constant(LiteralKind::Long),
        Semicolon,
        Long,
        Identifier,
        Equal,
        Constant(LiteralKind::Long),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        Identifier,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        Identifier,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        Identifier,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        Identifier,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        Identifier,
        Comma,
        Constant(LiteralKind::Int),
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        Comma,
        Identifier,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_11_valid_long_expressions_long_and_int_locals() {
    let src = r#"
        int main(void) {
            long a = 8589934592l;
            int b = -1;
            long c = -8589934592l;
            int d = 10;
            if (a != 8589934592l) {
                return 1;
            }
            if (b != -1){
                return 2;
            }
            if (c != -8589934592l) {
                return 3;
            }
            if (d != 10) {
                return 4;
            }
            a = -a;
            b = b - 1;
            c = c + 8589934594l;
            d = d + 10;
            if (a != -8589934592l) {
                return 5;
            }
            if (b != -2) {
                return 6;
            }
            if (c != 2) {
                return 7;
            }
            if (d != 20) {
                return 8;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Long,
        Identifier,
        Equal,
        Constant(LiteralKind::Long),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        Long,
        Identifier,
        Equal,
        Minus,
        Constant(LiteralKind::Long),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Long),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        Constant(LiteralKind::Long),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Minus,
        Identifier,
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Long),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        Constant(LiteralKind::Long),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_11_valid_long_expressions_long_args() {
    let src = r#"
        int test_sum(long a, long b, int c, int d, int e, int f, int g, int h, long i) {
            if (a + b < 100l) {
                return 1;
            }
            if (i < 100l)
                return 2;
            return 0;
        }
        int main(void) {
            return test_sum(34359738368l, 34359738368l, 0, 0, 0, 0, 0, 0, 34359738368l);
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Long,
        Identifier,
        Comma,
        Long,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Long,
        Identifier,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Identifier,
        Plus,
        Identifier,
        Less,
        Constant(LiteralKind::Long),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        Less,
        Constant(LiteralKind::Long),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Long),
        Comma,
        Constant(LiteralKind::Long),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Long),
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_11_valid_long_expressions_multi_op() {
    let src = r#"
        int target(long a) {
            long b = a * 5l - 10l;
            if (b == 21474836440l) {
                return 1;
            }
            return 0;
        }
        int main(void) {
            return target(4294967290l);
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Long,
        Identifier,
        CloseParen,
        OpenBrace,
        Long,
        Identifier,
        Equal,
        Identifier,
        Star,
        Constant(LiteralKind::Long),
        Minus,
        Constant(LiteralKind::Long),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Long),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Long),
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_11_valid_long_expressions_return_long() {
    let src = r#"
        long add(int a, int b) {
            return (long) a + (long) b;
        }
        int main(void) {
            long a = add(2147483645, 2147483645);
            if (a == 4294967290l) {
                return 1;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Long,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        OpenParen,
        Long,
        CloseParen,
        Identifier,
        Plus,
        OpenParen,
        Long,
        CloseParen,
        Identifier,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Long,
        Identifier,
        Equal,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Long),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_11_valid_long_expressions_rewrite_large_multiply_regression() {
    let src = r#"
        int check_12_ints(int start, int a, int b, int c, int d, int e, int f, int g,
                          int h, int i, int j, int k, int l);
        long glob = 5l;
        int main(void) {
            long should_spill = glob * 4294967307l;
            int one = glob - 4;
            int two = one + one;
            int three = 2 + one;
            int four = two * two;
            int five = 6 - one;
            int six = two * three;
            int seven = one + 6;
            int eight = two * 4;
            int nine = three * three;
            int ten = four + six;
            int eleven = 16 - five;
            int twelve = six + six;
            check_12_ints(one, two, three, four, five, six, seven, eight, nine, ten,
                          eleven, twelve, 1);
            int thirteen = glob + 8;
            int fourteen = thirteen + 1;
            int fifteen = 28 - thirteen;
            int sixteen = fourteen + 2;
            int seventeen = 4 + thirteen;
            int eighteen = 32 - fourteen;
            int nineteen = 35 - sixteen;
            int twenty = fifteen + 5;
            int twenty_one = thirteen * 2 - 5;
            int twenty_two = fifteen + 7;
            int twenty_three = 6 + seventeen;
            int twenty_four = thirteen + 11;
            check_12_ints(thirteen, fourteen, fifteen, sixteen, seventeen, eighteen,
                          nineteen, twenty, twenty_one, twenty_two, twenty_three,
                          twenty_four, 13);
            if (should_spill != 21474836535l) {
                return -1;
            }
            return 0;
        }
        int check_12_ints(int a, int b, int c, int d, int e, int f, int g, int h, int i,
                          int j, int k, int l, int start) {
            int expected = 0;
            expected = start + 0;
            if (a != expected) {
                return expected;
            }
            expected = start + 1;
            if (b != expected) {
                return expected;
            }
            expected = start + 2;
            if (c != expected) {
                return expected;
            }
            expected = start + 3;
            if (d != expected) {
                return expected;
            }
            expected = start + 4;
            if (e != expected) {
                return expected;
            }
            expected = start + 5;
            if (f != expected) {
                return expected;
            }
            expected = start + 6;
            if (g != expected) {
                return expected;
            }
            expected = start + 7;
            if (h != expected) {
                return expected;
            }
            expected = start + 8;
            if (i != expected) {
                return expected;
            }
            expected = start + 9;
            if (j != expected) {
                return expected;
            }
            expected = start + 10;
            if (k != expected) {
                return expected;
            }
            expected = start + 11;
            if (l != expected) {
                return expected;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        CloseParen,
        Semicolon,
        Long,
        Identifier,
        Equal,
        Constant(LiteralKind::Long),
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Long,
        Identifier,
        Equal,
        Identifier,
        Star,
        Constant(LiteralKind::Long),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Plus,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        Star,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Minus,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        Star,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        Star,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        Star,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Minus,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Identifier,
        Semicolon,
        Identifier,
        OpenParen,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Minus,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Plus,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Minus,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Minus,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        Star,
        Constant(LiteralKind::Int),
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Plus,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        OpenParen,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Long),
        CloseParen,
        OpenBrace,
        Return,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_11_valid_long_expressions_simple() {
    let src = r#"
        int main(void) {
            long l = 9223372036854775807l;
            return (l - 2l == 9223372036854775805l);
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Long,
        Identifier,
        Equal,
        Constant(LiteralKind::Long),
        Semicolon,
        Return,
        OpenParen,
        Identifier,
        Minus,
        Constant(LiteralKind::Long),
        EqualEqual,
        Constant(LiteralKind::Long),
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_11_valid_long_expressions_static_long() {
    let src = r#"
        
        static long foo = 4294967290l;
        int main(void)
        {
            if (foo + 5l == 4294967295l)
            {
                foo = 1152921504606846988l;
                if (foo == 1152921504606846988l)
                    return 1;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Static,
        Long,
        Identifier,
        Equal,
        Constant(LiteralKind::Long),
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Identifier,
        Plus,
        Constant(LiteralKind::Long),
        EqualEqual,
        Constant(LiteralKind::Long),
        CloseParen,
        OpenBrace,
        Identifier,
        Equal,
        Constant(LiteralKind::Long),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Long),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_11_valid_long_expressions_type_specifiers() {
    let src = r#"
        static int long a;
        int static long a;
        long static a;
        int my_function(long a, long int b, int long c);
        int my_function(long int x, int long y, long z) {
            return x + y + z;
        }
        int main(void) {
            long x = 1l;
            long int y = 2l;
            int long z = 3l;
            extern long a;
            a = 4;
           int sum = 0;
            for (long i = 1099511627776l; i > 0; i = i / 2) {
                sum = sum + 1;
            }
            if (x != 1) {
                return 1;
            }
            if (y != 2) {
                return 2;
            }
            if (a != 4) {
                return 3;
            }
            if (my_function(x, y, z) != 6) {
                return 4;
            }
            if (sum != 41) {
                return 5;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Static,
        Int,
        Long,
        Identifier,
        Semicolon,
        Int,
        Static,
        Long,
        Identifier,
        Semicolon,
        Long,
        Static,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Long,
        Identifier,
        Comma,
        Long,
        Int,
        Identifier,
        Comma,
        Int,
        Long,
        Identifier,
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Long,
        Int,
        Identifier,
        Comma,
        Int,
        Long,
        Identifier,
        Comma,
        Long,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Plus,
        Identifier,
        Plus,
        Identifier,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Long,
        Identifier,
        Equal,
        Constant(LiteralKind::Long),
        Semicolon,
        Long,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Long),
        Semicolon,
        Int,
        Long,
        Identifier,
        Equal,
        Constant(LiteralKind::Long),
        Semicolon,
        Extern,
        Long,
        Identifier,
        Semicolon,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        For,
        OpenParen,
        Long,
        Identifier,
        Equal,
        Constant(LiteralKind::Long),
        Semicolon,
        Identifier,
        Greater,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Slash,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Identifier,
        CloseParen,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
#[should_panic]
fn test_chapter_12_invalid_lex_invalid_suffix() {
    tokenize(
        r#"
        int main(void) {
            return 0uu;
        }
    "#,
    );
}

#[test]
#[should_panic]
fn test_chapter_12_invalid_lex_invalid_suffix_2() {
    tokenize(
        r#"
        int main(void) {
            return 0lul;
        }
    "#,
    );
}

#[test]
fn test_chapter_12_valid_explicit_casts_chained_casts() {
    let src = r#"
        unsigned int ui = 4294967200u;
        int main(void) {
            if ((long) (signed) ui != -96l)
                return 1;
            if ((unsigned long) (signed) ui != 18446744073709551520ul)
                return 2;
            return 0;
        }
    "#;
    let expected = vec![
        Unsigned,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Uint),
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        OpenParen,
        Long,
        CloseParen,
        OpenParen,
        Signed,
        CloseParen,
        Identifier,
        BangEqual,
        Minus,
        Constant(LiteralKind::Long),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        OpenParen,
        Unsigned,
        Long,
        CloseParen,
        OpenParen,
        Signed,
        CloseParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::ULong),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_12_valid_explicit_casts_extension() {
    let src = r#"
        int int_to_ulong(int i, unsigned long expected) {
            unsigned long result = (unsigned long) i;
            return result == expected;
        }
        int uint_to_long(unsigned int ui, long expected) {
            long result = (long) ui;
            return result == expected;
        }
        int uint_to_ulong(unsigned ui, unsigned long expected){
            return (unsigned long) ui == expected;
        }
        int main(void) {
            if (!int_to_ulong(10, 10ul)) {
                return 1;
            }
            if (!int_to_ulong(-10, 18446744073709551606ul)) {
                return 2;
            }
            if (!uint_to_long(4294967200u, 4294967200l)) {
                return 3;
            }
            if (!uint_to_ulong(4294967200u, 4294967200ul)) {
                return 4;
            }
            if ((unsigned long) 4294967200u != 4294967200ul) {
                return 5;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        Comma,
        Unsigned,
        Long,
        Identifier,
        CloseParen,
        OpenBrace,
        Unsigned,
        Long,
        Identifier,
        Equal,
        OpenParen,
        Unsigned,
        Long,
        CloseParen,
        Identifier,
        Semicolon,
        Return,
        Identifier,
        EqualEqual,
        Identifier,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Unsigned,
        Int,
        Identifier,
        Comma,
        Long,
        Identifier,
        CloseParen,
        OpenBrace,
        Long,
        Identifier,
        Equal,
        OpenParen,
        Long,
        CloseParen,
        Identifier,
        Semicolon,
        Return,
        Identifier,
        EqualEqual,
        Identifier,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Unsigned,
        Identifier,
        Comma,
        Unsigned,
        Long,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        OpenParen,
        Unsigned,
        Long,
        CloseParen,
        Identifier,
        EqualEqual,
        Identifier,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::ULong),
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        Minus,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::ULong),
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Uint),
        Comma,
        Constant(LiteralKind::Long),
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Uint),
        Comma,
        Constant(LiteralKind::ULong),
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        OpenParen,
        Unsigned,
        Long,
        CloseParen,
        Constant(LiteralKind::Uint),
        BangEqual,
        Constant(LiteralKind::ULong),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_12_valid_explicit_casts_rewrite_movz_regression() {
    let src = r#"
        int check_12_ints(int start, int a, int b, int c, int d, int e, int f, int g,
                          int h, int i, int j, int k, int l);
        unsigned glob = 5000u;
        int main(void) {
            long should_spill = (long)glob;
            int one = glob - 4999;
            int two = one + one;
            int three = 2 + one;
            int four = two * two;
            int five = 6 - one;
            int six = two * three;
            int seven = one + 6;
            int eight = two * 4;
            int nine = three * three;
            int ten = four + six;
            int eleven = 16 - five;
            int twelve = six + six;
            check_12_ints(one, two, three, four, five, six, seven, eight, nine, ten,
                          eleven, twelve, 1);
            int thirteen = glob - 4987u;
            int fourteen = thirteen + 1;
            int fifteen = 28 - thirteen;
            int sixteen = fourteen + 2;
            int seventeen = 4 + thirteen;
            int eighteen = 32 - fourteen;
            int nineteen = 35 - sixteen;
            int twenty = fifteen + 5;
            int twenty_one = thirteen * 2 - 5;
            int twenty_two = fifteen + 7;
            int twenty_three = 6 + seventeen;
            int twenty_four = thirteen + 11;
            check_12_ints(thirteen, fourteen, fifteen, sixteen, seventeen, eighteen,
                          nineteen, twenty, twenty_one, twenty_two, twenty_three,
                          twenty_four, 13);
            if (should_spill != 5000l) {
                return -1;
            }
            return 0;
        }
        int check_12_ints(int a, int b, int c, int d, int e, int f, int g, int h, int i,
                          int j, int k, int l, int start) {
            int expected = 0;
            expected = start + 0;
            if (a != expected) {
                return expected;
            }
            expected = start + 1;
            if (b != expected) {
                return expected;
            }
            expected = start + 2;
            if (c != expected) {
                return expected;
            }
            expected = start + 3;
            if (d != expected) {
                return expected;
            }
            expected = start + 4;
            if (e != expected) {
                return expected;
            }
            expected = start + 5;
            if (f != expected) {
                return expected;
            }
            expected = start + 6;
            if (g != expected) {
                return expected;
            }
            expected = start + 7;
            if (h != expected) {
                return expected;
            }
            expected = start + 8;
            if (i != expected) {
                return expected;
            }
            expected = start + 9;
            if (j != expected) {
                return expected;
            }
            expected = start + 10;
            if (k != expected) {
                return expected;
            }
            expected = start + 11;
            if (l != expected) {
                return expected;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        CloseParen,
        Semicolon,
        Unsigned,
        Identifier,
        Equal,
        Constant(LiteralKind::Uint),
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Long,
        Identifier,
        Equal,
        OpenParen,
        Long,
        CloseParen,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Plus,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        Star,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Minus,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        Star,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        Star,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        Star,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Minus,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Identifier,
        Semicolon,
        Identifier,
        OpenParen,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        Minus,
        Constant(LiteralKind::Uint),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Minus,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Plus,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Minus,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Minus,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        Star,
        Constant(LiteralKind::Int),
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Plus,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        OpenParen,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Identifier,
        Comma,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Long),
        CloseParen,
        OpenBrace,
        Return,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_12_valid_explicit_casts_round_trip_casts() {
    let src = r#"
        unsigned long a = 8589934580ul;
        int main(void) {
            unsigned long b = (unsigned long) (unsigned int) a;
            if (b != 4294967284ul)
                return 1;
            b = (unsigned long) (signed int) a;
            if (b != 18446744073709551604ul)
                return 2;
            return 0;
        }
    "#;
    let expected = vec![
        Unsigned,
        Long,
        Identifier,
        Equal,
        Constant(LiteralKind::ULong),
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Unsigned,
        Long,
        Identifier,
        Equal,
        OpenParen,
        Unsigned,
        Long,
        CloseParen,
        OpenParen,
        Unsigned,
        Int,
        CloseParen,
        Identifier,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::ULong),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        OpenParen,
        Unsigned,
        Long,
        CloseParen,
        OpenParen,
        Signed,
        Int,
        CloseParen,
        Identifier,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::ULong),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_12_valid_explicit_casts_same_size_conversion() {
    let src = r#"
        int uint_to_int(unsigned int ui, int expected) {
            return (int) ui == expected;
        }
        int int_to_uint(int i, unsigned int expected) {
            return (unsigned int) i == expected;
        }
        int ulong_to_long(unsigned long ul, signed long expected) {
            return (signed long) ul == expected;
        }
        int long_to_ulong(long l, unsigned long expected) {
            return (unsigned long) l == expected;
        }
        int main(void) {
            if (!int_to_uint(10, 10u)) {
                return 1;
            }
            if (!uint_to_int(10u, 10)) {
                return 2;
            }
            if (!long_to_ulong(-1000l, 18446744073709550616ul)) {
                return 3;
            }
            if (!ulong_to_long(18446744073709550616ul, -1000l)) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Unsigned,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        OpenParen,
        Int,
        CloseParen,
        Identifier,
        EqualEqual,
        Identifier,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        Comma,
        Unsigned,
        Int,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        OpenParen,
        Unsigned,
        Int,
        CloseParen,
        Identifier,
        EqualEqual,
        Identifier,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Unsigned,
        Long,
        Identifier,
        Comma,
        Signed,
        Long,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        OpenParen,
        Signed,
        Long,
        CloseParen,
        Identifier,
        EqualEqual,
        Identifier,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Long,
        Identifier,
        Comma,
        Unsigned,
        Long,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        OpenParen,
        Unsigned,
        Long,
        CloseParen,
        Identifier,
        EqualEqual,
        Identifier,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Uint),
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Uint),
        Comma,
        Constant(LiteralKind::Int),
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        Minus,
        Constant(LiteralKind::Long),
        Comma,
        Constant(LiteralKind::ULong),
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        Constant(LiteralKind::ULong),
        Comma,
        Minus,
        Constant(LiteralKind::Long),
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_12_valid_explicit_casts_truncate() {
    let src = r#"
        
        int ulong_to_int(unsigned long ul, int expected) {
            int result = (int) ul;
            return (result == expected);
        }
        int ulong_to_uint(unsigned long ul, unsigned expected) {
            return ((unsigned int) ul == expected);
        }
        int long_to_uint(long l, unsigned int expected) {
            return (unsigned int) l == expected;
        }
        int main(void) {
            if (!long_to_uint(100l, 100u)) {
                return 1;
            }
            if (!long_to_uint(-9223372036854774574l, 1234u)) {
                return 2;
            }
            if (!ulong_to_int(100ul, 100)) {
                return 3;
            }
            if (!ulong_to_uint(100ul, 100u)) {
                return 4;
            }
            if (!ulong_to_uint(4294967200ul, 4294967200u)) {
                return 5;
            }
            if (!ulong_to_int(4294967200ul, -96)) {
                return 6;
            }
            if (!ulong_to_uint(1152921506754330624ul, 2147483648u)) {
                return 7;
            }
            if (!ulong_to_int(1152921506754330624ul, -2147483648)){
                return 8;
            }
            unsigned int ui = (unsigned int)17179869189ul;
            if (ui != 5)
                return 9;
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Unsigned,
        Long,
        Identifier,
        Comma,
        Int,
        Identifier,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        OpenParen,
        Int,
        CloseParen,
        Identifier,
        Semicolon,
        Return,
        OpenParen,
        Identifier,
        EqualEqual,
        Identifier,
        CloseParen,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Unsigned,
        Long,
        Identifier,
        Comma,
        Unsigned,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        OpenParen,
        OpenParen,
        Unsigned,
        Int,
        CloseParen,
        Identifier,
        EqualEqual,
        Identifier,
        CloseParen,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Long,
        Identifier,
        Comma,
        Unsigned,
        Int,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        OpenParen,
        Unsigned,
        Int,
        CloseParen,
        Identifier,
        EqualEqual,
        Identifier,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Long),
        Comma,
        Constant(LiteralKind::Uint),
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        Minus,
        Constant(LiteralKind::Long),
        Comma,
        Constant(LiteralKind::Uint),
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        Constant(LiteralKind::ULong),
        Comma,
        Constant(LiteralKind::Int),
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        Constant(LiteralKind::ULong),
        Comma,
        Constant(LiteralKind::Uint),
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        Constant(LiteralKind::ULong),
        Comma,
        Constant(LiteralKind::Uint),
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        Constant(LiteralKind::ULong),
        Comma,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        Constant(LiteralKind::ULong),
        Comma,
        Constant(LiteralKind::Uint),
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        Constant(LiteralKind::ULong),
        Comma,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Unsigned,
        Int,
        Identifier,
        Equal,
        OpenParen,
        Unsigned,
        Int,
        CloseParen,
        Constant(LiteralKind::ULong),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_12_valid_extra_credit_bitwise_unsigned_ops() {
    let src = r#"
        int main(void) {
            unsigned int ui = -1u;
            unsigned long ul = 9223372036854775808ul;
            if ((ui & ul) != 0)
                return 1;
            if ((ui | ul) != 9223372041149743103ul)
                return 2;
            signed int i = -1;
            if ((i & ul) != ul)
                return 3;
            if ((i | ul) != i)
                return 4;
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Unsigned,
        Int,
        Identifier,
        Equal,
        Minus,
        Constant(LiteralKind::Uint),
        Semicolon,
        Unsigned,
        Long,
        Identifier,
        Equal,
        Constant(LiteralKind::ULong),
        Semicolon,
        If,
        OpenParen,
        OpenParen,
        Identifier,
        Ampersand,
        Identifier,
        CloseParen,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        OpenParen,
        Identifier,
        Pipe,
        Identifier,
        CloseParen,
        BangEqual,
        Constant(LiteralKind::ULong),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Signed,
        Int,
        Identifier,
        Equal,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        OpenParen,
        Identifier,
        Ampersand,
        Identifier,
        CloseParen,
        BangEqual,
        Identifier,
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        OpenParen,
        Identifier,
        Pipe,
        Identifier,
        CloseParen,
        BangEqual,
        Identifier,
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_12_valid_extra_credit_bitwise_unsigned_shift() {
    let src = r#"
        int main(void) {
            unsigned int ui = -1u;
            if ((ui << 2l) != 4294967292) {
                return 1;
            }
            if ((ui >> 2) != 1073741823) {
                return 2;
            }
            static int shiftcount = 5;
            if ((1000000u >> shiftcount) != 31250) {
                return 3;
            }
            if ((1000000u << shiftcount) != 32000000) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Unsigned,
        Int,
        Identifier,
        Equal,
        Minus,
        Constant(LiteralKind::Uint),
        Semicolon,
        If,
        OpenParen,
        OpenParen,
        Identifier,
        LessLess,
        Constant(LiteralKind::Long),
        CloseParen,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        OpenParen,
        Identifier,
        GreaterGreater,
        Constant(LiteralKind::Int),
        CloseParen,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Static,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        OpenParen,
        Constant(LiteralKind::Uint),
        GreaterGreater,
        Identifier,
        CloseParen,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        OpenParen,
        Constant(LiteralKind::Uint),
        LessLess,
        Identifier,
        CloseParen,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_12_valid_extra_credit_compound_assign_uint() {
    let src = r#"
        int main(void) {
            unsigned int x = -1u;
            x /= -10l;
            return (x == 3865470567u);
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Unsigned,
        Int,
        Identifier,
        Equal,
        Minus,
        Constant(LiteralKind::Uint),
        Semicolon,
        Identifier,
        SlashEqual,
        Minus,
        Constant(LiteralKind::Long),
        Semicolon,
        Return,
        OpenParen,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Uint),
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_12_valid_extra_credit_compound_bitshift() {
    let src = r#"
        int main(void) {
            int i = -2;
            i >>= 3u;
            if (i != -1) {
                return 1;
            }
            unsigned long ul = 18446744073709551615UL;
            ul <<= 44;
            if (ul != 18446726481523507200ul) {
                return 2;
            }
            return 0;
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
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        GreaterGreaterEqual,
        Constant(LiteralKind::Uint),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Unsigned,
        Long,
        Identifier,
        Equal,
        Constant(LiteralKind::ULong),
        Semicolon,
        Identifier,
        LessLessEqual,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::ULong),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_12_valid_extra_credit_compound_bitwise() {
    let src = r#"
        int main(void) {
            unsigned long ul = 18446460386757245432ul;
            ul &= -1000;
            if (ul != 18446460386757244952ul ) {
                return 1;
            }
            ul |= 4294967040u;
            if (ul != 18446460386824683288ul ) {
                return 2;
            }
            int i = 123456;
            unsigned int ui = 4042322160u;
            long l = -252645136;
            if (ui ^= l) {
                return 3;
            }
            if (ui) {
                return 4;
            }
            if (i != 123456) {
                return 5;
            }
            if (l != -252645136) {
                return 6;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Unsigned,
        Long,
        Identifier,
        Equal,
        Constant(LiteralKind::ULong),
        Semicolon,
        Identifier,
        AmpersandEqual,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::ULong),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        PipeEqual,
        Constant(LiteralKind::Uint),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::ULong),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Unsigned,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Uint),
        Semicolon,
        Long,
        Identifier,
        Equal,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        CircumflexEqual,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_12_valid_extra_credit_postfix_precedence() {
    let src = r#"
        int main(void) {
            unsigned int ui = 4294967295U;
            if (((unsigned long)ui++) != 4294967295U) {
                return 1;
            }
            if (ui) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Unsigned,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Uint),
        Semicolon,
        If,
        OpenParen,
        OpenParen,
        OpenParen,
        Unsigned,
        Long,
        CloseParen,
        Identifier,
        PlusPlus,
        CloseParen,
        BangEqual,
        Constant(LiteralKind::Uint),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_12_valid_extra_credit_switch_uint() {
    let src = r#"
        int switch_on_uint(unsigned int ui) {
            switch (ui) {
                case 5u:
                    return 0;
                case 4294967286l:
                    return 1;
                case 34359738378ul:
                    return 2;
                default:
                    return 3;
            }
        }
        int main(void) {
            if (switch_on_uint(5) != 0)
                return 1;
            if (switch_on_uint(4294967286) != 1)
                return 2;
            if (switch_on_uint(10) != 2)
                return 3;
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Unsigned,
        Int,
        Identifier,
        CloseParen,
        OpenBrace,
        Switch,
        OpenParen,
        Identifier,
        CloseParen,
        OpenBrace,
        Case,
        Constant(LiteralKind::Uint),
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Case,
        Constant(LiteralKind::Long),
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Case,
        Constant(LiteralKind::ULong),
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Default,
        Colon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        CloseParen,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_12_valid_extra_credit_unsigned_incr_decr() {
    let src = r#"
        
        int main(void) {
            unsigned int i = 0;
            if (i-- != 0) {
                return 1;
            }
            if (i != 4294967295U) {
                return 2;
            }
            if (--i != 4294967294U) {
                return 3;
            }
            if (i != 4294967294U) {
                return 4;
            }
            unsigned long l = 18446744073709551614UL;
            if (l++ != 18446744073709551614UL) {
                return 5;
            }
            if (l != 18446744073709551615UL) {
                return 6;
            }
            if (++l != 0) {
                return 7;
            }
            if (l != 0) {
                return 8;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Unsigned,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        MinusMinus,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Uint),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        MinusMinus,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Uint),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Uint),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Unsigned,
        Long,
        Identifier,
        Equal,
        Constant(LiteralKind::ULong),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        PlusPlus,
        BangEqual,
        Constant(LiteralKind::ULong),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::ULong),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        PlusPlus,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_12_valid_implicit_casts_common_type() {
    let src = r#"
        int int_gt_uint(int i, unsigned int u) {
            return i > u;
        }
        int int_gt_ulong(int i, unsigned long ul) {
            return i > ul;
        }
        int uint_gt_long(unsigned int u, long l) {
            return u > l;
        }
        int uint_lt_ulong(unsigned int u, unsigned long ul) {
            return u < ul;
        }
        int long_gt_ulong(long l, unsigned long ul) {
            return l > ul;
        }
        int ternary_int_uint(int flag, int i, unsigned int ui) {
            long result = flag ? i : ui;
            return (result == 4294967295l);
        }
        int main(void) {
            if (!int_gt_uint(-100, 100u)) {
                return 1;
            }
            if (!(int_gt_ulong(-1, 18446744073709551606ul))) {
                return 2;
            }
            if (!uint_gt_long(100u, -100l)) {
                return 3;
            }
            if (!uint_lt_ulong(1073741824u, 34359738368ul)) {
                return 4;
            }
            if (!long_gt_ulong(-1l, 1000ul)) {
                return 5;
            }
            if (!ternary_int_uint(1, -1, 1u)) {
                return 6;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        Comma,
        Unsigned,
        Int,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Greater,
        Identifier,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        Comma,
        Unsigned,
        Long,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Greater,
        Identifier,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Unsigned,
        Int,
        Identifier,
        Comma,
        Long,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Greater,
        Identifier,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Unsigned,
        Int,
        Identifier,
        Comma,
        Unsigned,
        Long,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Less,
        Identifier,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Long,
        Identifier,
        Comma,
        Unsigned,
        Long,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Greater,
        Identifier,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Unsigned,
        Int,
        Identifier,
        CloseParen,
        OpenBrace,
        Long,
        Identifier,
        Equal,
        Identifier,
        Question,
        Identifier,
        Colon,
        Identifier,
        Semicolon,
        Return,
        OpenParen,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Long),
        CloseParen,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        Minus,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Uint),
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        OpenParen,
        Identifier,
        OpenParen,
        Minus,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::ULong),
        CloseParen,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Uint),
        Comma,
        Minus,
        Constant(LiteralKind::Long),
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Uint),
        Comma,
        Constant(LiteralKind::ULong),
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        Minus,
        Constant(LiteralKind::Long),
        Comma,
        Constant(LiteralKind::ULong),
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        Comma,
        Minus,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Uint),
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_12_valid_implicit_casts_convert_by_assignment() {
    let src = r#"
        int check_int(int converted, int expected) {
            return (converted == expected);
        }
        int check_long(long converted, long expected) {
            return (converted == expected);
        }
        int check_ulong(unsigned long converted, unsigned long expected) {
            return (converted == expected);
        }
        long return_extended_uint(unsigned int u) {
            return u;
        }
        unsigned long return_extended_int(int i) {
            return i;
        }
        int return_truncated_ulong(unsigned long ul) {
            return ul;
        }
        int extend_on_assignment(unsigned int ui, long expected) {
            long result = ui;
            return result == expected;
        }
        int main(void) {
            if (!check_int(9223372036854775813ul, 5)) {
                return 1;
            }
            if (!check_long(2147483658u, 2147483658l)) {
                return 2;
            }
            if (!check_ulong(-1, 18446744073709551615UL)) {
                return 3;
            }
            if (return_extended_uint(2147483658u) != 2147483658l) {
                return 4;
            }
            if (return_extended_int(-1) != 18446744073709551615UL) {
                return 5;
            }
            long l = return_truncated_ulong(1125902054326372ul);
            if (l != -2147483548l) {
                return 6;
            }
            if (!extend_on_assignment(2147483658u, 2147483658l)){
                return 7;
            }
            int i = 4294967196u;
            if (i != -100) {
                return 8;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        Comma,
        Int,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        OpenParen,
        Identifier,
        EqualEqual,
        Identifier,
        CloseParen,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Long,
        Identifier,
        Comma,
        Long,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        OpenParen,
        Identifier,
        EqualEqual,
        Identifier,
        CloseParen,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Unsigned,
        Long,
        Identifier,
        Comma,
        Unsigned,
        Long,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        OpenParen,
        Identifier,
        EqualEqual,
        Identifier,
        CloseParen,
        Semicolon,
        CloseBrace,
        Long,
        Identifier,
        OpenParen,
        Unsigned,
        Int,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Unsigned,
        Long,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Unsigned,
        Long,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Unsigned,
        Int,
        Identifier,
        Comma,
        Long,
        Identifier,
        CloseParen,
        OpenBrace,
        Long,
        Identifier,
        Equal,
        Identifier,
        Semicolon,
        Return,
        Identifier,
        EqualEqual,
        Identifier,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        Constant(LiteralKind::ULong),
        Comma,
        Constant(LiteralKind::Int),
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Uint),
        Comma,
        Constant(LiteralKind::Long),
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        Minus,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::ULong),
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Uint),
        CloseParen,
        BangEqual,
        Constant(LiteralKind::Long),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        BangEqual,
        Constant(LiteralKind::ULong),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Long,
        Identifier,
        Equal,
        Identifier,
        OpenParen,
        Constant(LiteralKind::ULong),
        CloseParen,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        Constant(LiteralKind::Long),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Uint),
        Comma,
        Constant(LiteralKind::Long),
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Uint),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_12_valid_implicit_casts_promote_constants() {
    let src = r#"
        long negative_one = 1l;
        long zero = 0l;
        int main(void) {
            negative_one = -negative_one;
            if (68719476736u >= negative_one) {
                return 1;
            }
            if (-2147483658 >= zero) {
                return 2;
            }
            if (!(3ul + 4294967293ul)) {
                return 3;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Long,
        Identifier,
        Equal,
        Constant(LiteralKind::Long),
        Semicolon,
        Long,
        Identifier,
        Equal,
        Constant(LiteralKind::Long),
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Identifier,
        Equal,
        Minus,
        Identifier,
        Semicolon,
        If,
        OpenParen,
        Constant(LiteralKind::Uint),
        GreaterEqual,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Minus,
        Constant(LiteralKind::Int),
        GreaterEqual,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        OpenParen,
        Constant(LiteralKind::ULong),
        Plus,
        Constant(LiteralKind::ULong),
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_12_valid_implicit_casts_static_initializers() {
    let src = r#"
        unsigned int u = 1152921506754330636l;
        int i = 2147483650u;
        long l = 9223372036854775900u;
        long l2 = 2147483650u;
        unsigned long ul = 4294967294u;
        unsigned long ul2 = 9223372036854775798l;
        int i2 = 9223372039002259606ul;
        unsigned ui2 = 9223372039002259606ul;
        int main(void)
        {
            if (u != 2147483660u)
                return 1;
            if (i != -2147483646)
                return 2;
            if (l != -9223372036854775716l)
                return 3;
            if (l2 != 2147483650l)
                return 4;
            if (ul != 4294967294ul)
                return 5;
            if (ul2 != 9223372036854775798ul)
                return 6;
            if (i2 != -2147483498)
                return 7;
            if (ui2 != 2147483798u)
                return 8;
            return 0;
        }
    "#;
    let expected = vec![
        Unsigned,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Long),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Uint),
        Semicolon,
        Long,
        Identifier,
        Equal,
        Constant(LiteralKind::Uint),
        Semicolon,
        Long,
        Identifier,
        Equal,
        Constant(LiteralKind::Uint),
        Semicolon,
        Unsigned,
        Long,
        Identifier,
        Equal,
        Constant(LiteralKind::Uint),
        Semicolon,
        Unsigned,
        Long,
        Identifier,
        Equal,
        Constant(LiteralKind::Long),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::ULong),
        Semicolon,
        Unsigned,
        Identifier,
        Equal,
        Constant(LiteralKind::ULong),
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Uint),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        Constant(LiteralKind::Long),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Long),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::ULong),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::ULong),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Uint),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_12_valid_libraries_unsigned_args() {
    let src = r#"
        int accept_unsigned(unsigned int a, unsigned int b, unsigned long c, unsigned long d,
                         unsigned int e, unsigned int f, unsigned long g, unsigned int h,
                         unsigned long i) {
            if (a != 1u) {
                return 1;
            }
            if (b != 4294967295U) {
                return 2;
            }
            if (c != 18446744073709551615UL) {
                return 3;
            }
            if (d != 9223372036854775808ul) {
                return 4;
            }
            if (e != 2147483648u) {
                return 5;
            }
            if (f != 0u) {
                return 8;
            }
            if (g != 123456u) {
                return 9;
            }
            if (h != 2147487744u) {
                return 10;
            }
            if (i != 9223372041149743104ul) {
                return 11;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Unsigned,
        Int,
        Identifier,
        Comma,
        Unsigned,
        Int,
        Identifier,
        Comma,
        Unsigned,
        Long,
        Identifier,
        Comma,
        Unsigned,
        Long,
        Identifier,
        Comma,
        Unsigned,
        Int,
        Identifier,
        Comma,
        Unsigned,
        Int,
        Identifier,
        Comma,
        Unsigned,
        Long,
        Identifier,
        Comma,
        Unsigned,
        Int,
        Identifier,
        Comma,
        Unsigned,
        Long,
        Identifier,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Uint),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Uint),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::ULong),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::ULong),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Uint),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Uint),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Uint),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Uint),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::ULong),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_12_valid_libraries_unsigned_args_client() {
    let src = r#"
        
        int accept_unsigned(unsigned int a, unsigned int b, unsigned long c, unsigned long d,
                         unsigned int e, unsigned int f, unsigned long g, unsigned int h,
                         unsigned long i);
        int main(void) {
            return accept_unsigned(1, -1, -1, 9223372036854775808ul, 2147483648ul, 0, 123456, 2147487744u, 9223372041149743104ul);
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Unsigned,
        Int,
        Identifier,
        Comma,
        Unsigned,
        Int,
        Identifier,
        Comma,
        Unsigned,
        Long,
        Identifier,
        Comma,
        Unsigned,
        Long,
        Identifier,
        Comma,
        Unsigned,
        Int,
        Identifier,
        Comma,
        Unsigned,
        Int,
        Identifier,
        Comma,
        Unsigned,
        Long,
        Identifier,
        Comma,
        Unsigned,
        Int,
        Identifier,
        Comma,
        Unsigned,
        Long,
        Identifier,
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        Comma,
        Minus,
        Constant(LiteralKind::Int),
        Comma,
        Minus,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::ULong),
        Comma,
        Constant(LiteralKind::ULong),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Int),
        Comma,
        Constant(LiteralKind::Uint),
        Comma,
        Constant(LiteralKind::ULong),
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_12_valid_libraries_unsigned_global_var() {
    let src = r#"
        unsigned int ui = 4294967200u;
        unsigned int return_uint(void) {
            return ui;
        }
        int return_uint_as_signed(void) {
            return ui;
        }
        long return_uint_as_long(void) {
            return ui;
        }
    "#;
    let expected = vec![
        Unsigned,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Uint),
        Semicolon,
        Unsigned,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        Long,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_12_valid_libraries_unsigned_global_var_client() {
    let src = r#"
        extern unsigned int ui;
        unsigned int return_uint(void);
        int return_uint_as_signed(void);
        long return_uint_as_long(void);
        int main(void) {
            if (ui != 4294967200u)
                return 0;
            ui = -1;
            long result = (long) return_uint();
            if (result != 4294967295l)
                return 0;
            result = (long) return_uint_as_signed();
            if (result != -1l)
                return 0;
            result = return_uint_as_long();
            if (result != 4294967295l)
                return 0;
            return 1;
        }
    "#;
    let expected = vec![
        Extern,
        Unsigned,
        Int,
        Identifier,
        Semicolon,
        Unsigned,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        Semicolon,
        Long,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Uint),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        Long,
        Identifier,
        Equal,
        OpenParen,
        Long,
        CloseParen,
        Identifier,
        OpenParen,
        CloseParen,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Long),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        OpenParen,
        Long,
        CloseParen,
        Identifier,
        OpenParen,
        CloseParen,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        Constant(LiteralKind::Long),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        OpenParen,
        CloseParen,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Long),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_12_valid_type_specifiers_signed_type_specifiers() {
    let src = r#"
        static int i;
        signed extern i;
        int static signed i = 5;
        signed int static i;
        long signed l;
        long l = 7;
        int long l;
        signed long int l;
        int main(void) {
            int signed extern i;
            extern signed long l;
            if (i != 5) {
                return 1;
            }
            if (l != 7) {
                return 2;
            }
            int counter = 0;
            for (signed int index = 10; index > 0; index = index - 1) {
                counter = counter + 1;
            }
            if (counter != 10) {
                return 3;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Static,
        Int,
        Identifier,
        Semicolon,
        Signed,
        Extern,
        Identifier,
        Semicolon,
        Int,
        Static,
        Signed,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Signed,
        Int,
        Static,
        Identifier,
        Semicolon,
        Long,
        Signed,
        Identifier,
        Semicolon,
        Long,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Long,
        Identifier,
        Semicolon,
        Signed,
        Long,
        Int,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Int,
        Signed,
        Extern,
        Identifier,
        Semicolon,
        Extern,
        Signed,
        Long,
        Identifier,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        For,
        OpenParen,
        Signed,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Greater,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_12_valid_type_specifiers_unsigned_type_specifiers() {
    let src = r#"
        unsigned u;
        int unsigned u;
        unsigned int u = 6;
        unsigned long ul;
        long unsigned ul;
        long int unsigned ul;
        unsigned int long ul = 4;
        int main(void) {
            if (u != 6u) {
                return 1;
            }
            long extern unsigned ul;
            unsigned long extern ul;
            int extern unsigned long ul;
            if (ul != 4ul) {
                return 2;
            }
            int counter = 0;
            for (unsigned int index = 10; index < 4294967295U; index = index - 1) {
                counter = counter + 1;
            }
            if (counter != 11) {
                return 3;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Unsigned,
        Identifier,
        Semicolon,
        Int,
        Unsigned,
        Identifier,
        Semicolon,
        Unsigned,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Unsigned,
        Long,
        Identifier,
        Semicolon,
        Long,
        Unsigned,
        Identifier,
        Semicolon,
        Long,
        Int,
        Unsigned,
        Identifier,
        Semicolon,
        Unsigned,
        Int,
        Long,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Uint),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Long,
        Extern,
        Unsigned,
        Identifier,
        Semicolon,
        Unsigned,
        Long,
        Extern,
        Identifier,
        Semicolon,
        Int,
        Extern,
        Unsigned,
        Long,
        Identifier,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::ULong),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        For,
        OpenParen,
        Unsigned,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Less,
        Constant(LiteralKind::Uint),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_12_valid_unsigned_expressions_arithmetic_ops() {
    let src = r#"
        unsigned int ui_a;
        unsigned int ui_b;
        unsigned long ul_a;
        unsigned long ul_b;
        int addition(void) {
            return (ui_a + 2147483653u == 2147483663u);
        }
        int subtraction(void) {
            return (ul_a - ul_b == 18446744072635808792ul);
        }
        int multiplication(void) {
            return (ui_a * ui_b == 3221225472u);
        }
        int division(void) {
            return (ui_a / ui_b == 0);
        }
        int division_large_dividend(void) {
            return (ui_a / ui_b == 2);
        }
        int division_by_literal(void) {
            return (ul_a / 5ul == 219902325555ul);
        }
        int remaind(void) {
            return (ul_b % ul_a == 5ul);
        }
        int complement(void) {
            return (~ui_a == 0);
        }
        int main(void) {
            ui_a = 10u;
            if (!addition()) {
                return 1;
            }
            ul_a = 18446744072635809792ul;
            ul_b = 1000ul;
            if (!subtraction()) {
                return 2;
            }
            ui_a = 1073741824u;
            ui_b = 3u;
            if (!multiplication()) {
                return 3;
            }
            ui_a = 100u;
            ui_b = 4294967294u;
            if (!division()) {
                return 4;
            }
            ui_a = 4294967294u;
            ui_b = 2147483647u;
            if (!division_large_dividend()) {
                return 5;
            }
            ul_a = 1099511627775ul;
            if (!division_by_literal()) {
                return 6;
            }
            ul_a = 100ul;
            ul_b = 18446744073709551605ul;
            if (!remaind()) {
                return 7;
            }
            ui_a = 4294967295U;
            if (!complement()) {
                return 8;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Unsigned,
        Int,
        Identifier,
        Semicolon,
        Unsigned,
        Int,
        Identifier,
        Semicolon,
        Unsigned,
        Long,
        Identifier,
        Semicolon,
        Unsigned,
        Long,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        OpenParen,
        Identifier,
        Plus,
        Constant(LiteralKind::Uint),
        EqualEqual,
        Constant(LiteralKind::Uint),
        CloseParen,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        OpenParen,
        Identifier,
        Minus,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::ULong),
        CloseParen,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        OpenParen,
        Identifier,
        Star,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Uint),
        CloseParen,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        OpenParen,
        Identifier,
        Slash,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        OpenParen,
        Identifier,
        Slash,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        OpenParen,
        Identifier,
        Slash,
        Constant(LiteralKind::ULong),
        EqualEqual,
        Constant(LiteralKind::ULong),
        CloseParen,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        OpenParen,
        Identifier,
        Percent,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::ULong),
        CloseParen,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        OpenParen,
        Tilde,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Identifier,
        Equal,
        Constant(LiteralKind::Uint),
        Semicolon,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Constant(LiteralKind::ULong),
        Semicolon,
        Identifier,
        Equal,
        Constant(LiteralKind::ULong),
        Semicolon,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Constant(LiteralKind::Uint),
        Semicolon,
        Identifier,
        Equal,
        Constant(LiteralKind::Uint),
        Semicolon,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Constant(LiteralKind::Uint),
        Semicolon,
        Identifier,
        Equal,
        Constant(LiteralKind::Uint),
        Semicolon,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Constant(LiteralKind::Uint),
        Semicolon,
        Identifier,
        Equal,
        Constant(LiteralKind::Uint),
        Semicolon,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Constant(LiteralKind::ULong),
        Semicolon,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Constant(LiteralKind::ULong),
        Semicolon,
        Identifier,
        Equal,
        Constant(LiteralKind::ULong),
        Semicolon,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Constant(LiteralKind::Uint),
        Semicolon,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_12_valid_unsigned_expressions_arithmetic_wraparound() {
    let src = r#"
        unsigned int ui_a;
        unsigned int ui_b;
        unsigned long ul_a;
        unsigned long ul_b;
        int addition(void) {
            return ui_a + ui_b == 0u;
        }
        int subtraction(void) {
            return (ul_a - ul_b == 18446744073709551606ul);
        }
        int neg(void) {
            return -ul_a == 18446744073709551615UL;
        }
        int main(void) {
            ui_a = 4294967293u;
            ui_b = 2u;
            if (!addition()) {
                return 1;
            }
            ul_a = 10ul;
            ul_b = 20ul;
            if (!subtraction()) {
                return 2;
            }
            ul_a = 1ul;
            if (!neg()) {
                return 3;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Unsigned,
        Int,
        Identifier,
        Semicolon,
        Unsigned,
        Int,
        Identifier,
        Semicolon,
        Unsigned,
        Long,
        Identifier,
        Semicolon,
        Unsigned,
        Long,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Plus,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::Uint),
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        OpenParen,
        Identifier,
        Minus,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::ULong),
        CloseParen,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Minus,
        Identifier,
        EqualEqual,
        Constant(LiteralKind::ULong),
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Identifier,
        Equal,
        Constant(LiteralKind::Uint),
        Semicolon,
        Identifier,
        Equal,
        Constant(LiteralKind::Uint),
        Semicolon,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Constant(LiteralKind::ULong),
        Semicolon,
        Identifier,
        Equal,
        Constant(LiteralKind::ULong),
        Semicolon,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Constant(LiteralKind::ULong),
        Semicolon,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_12_valid_unsigned_expressions_comparisons() {
    let src = r#"
        unsigned int one_hundred = 100u;
        unsigned int large_uint = 4294967294u;
        unsigned long one_hundred_ulong = 100ul;
        unsigned long large_ulong = 4294967294ul;
        int main(void) {
            if (large_uint < one_hundred)
                return 1;
            if (large_uint <= one_hundred)
                return 2;
            if (one_hundred >= large_uint)
                return 3;
            if (one_hundred > large_uint)
                return 4;
            if (!(one_hundred <= large_uint))
                return 5;
            if (!(one_hundred < large_uint))
                return 6;
            if (!(large_uint > one_hundred))
                return 7;
            if (!(large_uint >= one_hundred))
                return 8;
            if (large_ulong < one_hundred_ulong)
                return 9;
            if (large_ulong <= one_hundred_ulong)
                return 10;
            if (one_hundred_ulong >= large_ulong)
                return 11;
            if (one_hundred_ulong > large_ulong)
                return 12;
            if (!(one_hundred_ulong <= large_ulong))
                return 13;
            if (!(one_hundred_ulong < large_ulong))
                return 14;
            if (!(large_ulong > one_hundred_ulong))
                return 15;
            if (!(large_ulong >= one_hundred_ulong))
                return 16;
            return 0;
        }
    "#;
    let expected = vec![
        Unsigned,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Uint),
        Semicolon,
        Unsigned,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Uint),
        Semicolon,
        Unsigned,
        Long,
        Identifier,
        Equal,
        Constant(LiteralKind::ULong),
        Semicolon,
        Unsigned,
        Long,
        Identifier,
        Equal,
        Constant(LiteralKind::ULong),
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Identifier,
        Less,
        Identifier,
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        LessEqual,
        Identifier,
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        GreaterEqual,
        Identifier,
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        Greater,
        Identifier,
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Bang,
        OpenParen,
        Identifier,
        LessEqual,
        Identifier,
        CloseParen,
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Bang,
        OpenParen,
        Identifier,
        Less,
        Identifier,
        CloseParen,
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Bang,
        OpenParen,
        Identifier,
        Greater,
        Identifier,
        CloseParen,
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Bang,
        OpenParen,
        Identifier,
        GreaterEqual,
        Identifier,
        CloseParen,
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        Less,
        Identifier,
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        LessEqual,
        Identifier,
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        GreaterEqual,
        Identifier,
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        Greater,
        Identifier,
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Bang,
        OpenParen,
        Identifier,
        LessEqual,
        Identifier,
        CloseParen,
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Bang,
        OpenParen,
        Identifier,
        Less,
        Identifier,
        CloseParen,
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Bang,
        OpenParen,
        Identifier,
        Greater,
        Identifier,
        CloseParen,
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Bang,
        OpenParen,
        Identifier,
        GreaterEqual,
        Identifier,
        CloseParen,
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_12_valid_unsigned_expressions_locals() {
    let src = r#"
        int main(void) {
            unsigned long a = 8589934592ul;
            int b = -1;
            long c = -8589934592l;
            unsigned int d = 10u;
            if (a != 8589934592ul) {
                return 1;
            }
            if (b != -1){
                return 2;
            }
            if (c != -8589934592l) {
                return 3;
            }
            if (d != 10u) {
                return 4;
            }
            a = -a;
            b = b - 1;
            c = c + 8589934594l;
            d = d * 268435456u;
            if (a != 18446744065119617024ul) {
                return 5;
            }
            if (b != -2) {
                return 6;
            }
            if (c != 2) {
                return 7;
            }
            if (d != 2684354560u) {
                return 8;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Unsigned,
        Long,
        Identifier,
        Equal,
        Constant(LiteralKind::ULong),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        Long,
        Identifier,
        Equal,
        Minus,
        Constant(LiteralKind::Long),
        Semicolon,
        Unsigned,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Uint),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::ULong),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        Constant(LiteralKind::Long),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Uint),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Minus,
        Identifier,
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Minus,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Long),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Star,
        Constant(LiteralKind::Uint),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::ULong),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::Uint),
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_12_valid_unsigned_expressions_logical() {
    let src = r#"
        int not(unsigned long ul) {
            return !ul;
        }
        int if_cond(unsigned u) {
            if (u) {
                return 1;
            }
            return 0;
        }
        int and(unsigned long ul, int i) {
            return ul && i;
        }
        int or(int i, unsigned u) {
            return i || u;
        }
        int main(void) {
            unsigned long ul = 1152921504606846976ul;
            unsigned int u = 2147483648u;
            unsigned long zero = 0l;
            if (not(ul)) {
                return 1;
            }
            if (!not(zero)) {
                return 2;
            }
            if(!if_cond(u)) {
                return 3;
            }
            if(if_cond(zero)) {
                return 4;
            }
            if (and(zero, 1)) {
                return 5;
            }
            if (!or(1, u)) {
                return 6;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Unsigned,
        Long,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Bang,
        Identifier,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Unsigned,
        Identifier,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Unsigned,
        Long,
        Identifier,
        Comma,
        Int,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        AmpersandAmpersand,
        Identifier,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        Comma,
        Unsigned,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        PipePipe,
        Identifier,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Unsigned,
        Long,
        Identifier,
        Equal,
        Constant(LiteralKind::ULong),
        Semicolon,
        Unsigned,
        Int,
        Identifier,
        Equal,
        Constant(LiteralKind::Uint),
        Semicolon,
        Unsigned,
        Long,
        Identifier,
        Equal,
        Constant(LiteralKind::Long),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        Identifier,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        Identifier,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        Identifier,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        Identifier,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        Identifier,
        Comma,
        Constant(LiteralKind::Int),
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        Constant(LiteralKind::Int),
        Comma,
        Identifier,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_12_valid_unsigned_expressions_simple() {
    let src = r#"
        int main(void) {
            unsigned u = 2147483647u;
            return (u + 2u == 2147483649u);
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Unsigned,
        Identifier,
        Equal,
        Constant(LiteralKind::Uint),
        Semicolon,
        Return,
        OpenParen,
        Identifier,
        Plus,
        Constant(LiteralKind::Uint),
        EqualEqual,
        Constant(LiteralKind::Uint),
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_chapter_12_valid_unsigned_expressions_static_variables() {
    let src = r#"
        
        static unsigned long x = 9223372036854775803ul;
        unsigned long zero_long;
        unsigned zero_int;
        int main(void)
        {
            if (x != 9223372036854775803ul)
                return 0;
            x = x + 10;
            if (x != 9223372036854775813ul)
                return 0;
            if (zero_long || zero_int)
                return 0;
            return 1;
        }
    "#;
    let expected = vec![
        Static,
        Unsigned,
        Long,
        Identifier,
        Equal,
        Constant(LiteralKind::ULong),
        Semicolon,
        Unsigned,
        Long,
        Identifier,
        Semicolon,
        Unsigned,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::ULong),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Constant(LiteralKind::ULong),
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        PipePipe,
        Identifier,
        CloseParen,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        Return,
        Constant(LiteralKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}
