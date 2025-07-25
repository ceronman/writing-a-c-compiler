use crate::lexer::TokenKind::*;
use crate::lexer::{tokenize, IntKind};

#[test]
fn test_valid_add() {
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
        IntConstant(IntKind::Int),
        Plus,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_associativity() {
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
        IntConstant(IntKind::Int),
        Minus,
        IntConstant(IntKind::Int),
        Minus,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_associativity_2() {
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
        IntConstant(IntKind::Int),
        Slash,
        IntConstant(IntKind::Int),
        Slash,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_associativity_3() {
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
        IntConstant(IntKind::Int),
        Slash,
        IntConstant(IntKind::Int),
        Star,
        IntConstant(IntKind::Int),
        CloseParen,
        Plus,
        OpenParen,
        IntConstant(IntKind::Int),
        Minus,
        IntConstant(IntKind::Int),
        Plus,
        IntConstant(IntKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_associativity_and_precedence() {
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
        IntConstant(IntKind::Int),
        Star,
        IntConstant(IntKind::Int),
        Slash,
        IntConstant(IntKind::Int),
        Minus,
        IntConstant(IntKind::Int),
        Percent,
        OpenParen,
        IntConstant(IntKind::Int),
        Plus,
        IntConstant(IntKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_div() {
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
        IntConstant(IntKind::Int),
        Slash,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_div_neg() {
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
        IntConstant(IntKind::Int),
        CloseParen,
        Slash,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_bitwise_and() {
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
        IntConstant(IntKind::Int),
        Ampersand,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_bitwise_or() {
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
        IntConstant(IntKind::Int),
        Pipe,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_bitwise_precedence() {
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
        IntConstant(IntKind::Int),
        GreaterGreater,
        IntConstant(IntKind::Int),
        Pipe,
        IntConstant(IntKind::Int),
        Circumflex,
        IntConstant(IntKind::Int),
        Ampersand,
        IntConstant(IntKind::Int),
        LessLess,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_bitwise_shift_associativity() {
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
        IntConstant(IntKind::Int),
        LessLess,
        IntConstant(IntKind::Int),
        GreaterGreater,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_bitwise_shift_associativity_2() {
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
        IntConstant(IntKind::Int),
        GreaterGreater,
        IntConstant(IntKind::Int),
        LessLess,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_bitwise_shift_precedence() {
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
        IntConstant(IntKind::Int),
        LessLess,
        IntConstant(IntKind::Int),
        Plus,
        IntConstant(IntKind::Int),
        GreaterGreater,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_bitwise_shiftl() {
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
        IntConstant(IntKind::Int),
        LessLess,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_bitwise_shiftr() {
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
        IntConstant(IntKind::Int),
        GreaterGreater,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_bitwise_shiftr_negative() {
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
        IntConstant(IntKind::Int),
        GreaterGreater,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_bitwise_variable_shift_count() {
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
        IntConstant(IntKind::Int),
        LessLess,
        OpenParen,
        IntConstant(IntKind::Int),
        Star,
        IntConstant(IntKind::Int),
        CloseParen,
        CloseParen,
        Plus,
        OpenParen,
        IntConstant(IntKind::Int),
        GreaterGreater,
        OpenParen,
        IntConstant(IntKind::Int),
        Plus,
        IntConstant(IntKind::Int),
        CloseParen,
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_bitwise_xor() {
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
        IntConstant(IntKind::Int),
        Circumflex,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_mod() {
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
        IntConstant(IntKind::Int),
        Percent,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_mult() {
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
        IntConstant(IntKind::Int),
        Star,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_parens() {
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
        IntConstant(IntKind::Int),
        Star,
        OpenParen,
        IntConstant(IntKind::Int),
        Plus,
        IntConstant(IntKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_precedence() {
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
        IntConstant(IntKind::Int),
        Plus,
        IntConstant(IntKind::Int),
        Star,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_sub() {
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
        IntConstant(IntKind::Int),
        Minus,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_sub_neg() {
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
        IntConstant(IntKind::Int),
        Minus,
        Minus,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_unop_add() {
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
        IntConstant(IntKind::Int),
        Plus,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_unop_parens() {
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
        IntConstant(IntKind::Int),
        Plus,
        IntConstant(IntKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}
