use crate::lexer::TokenKind::*;
use crate::lexer::{IntKind, tokenize};

#[test]
fn test_valid_add_variables() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
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
fn test_valid_allocate_temps_and_vars() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        Slash,
        IntConstant(IntKind::Int),
        Plus,
        Bang,
        Identifier,
        Semicolon,
        Return,
        Identifier,
        Star,
        IntConstant(IntKind::Int),
        EqualEqual,
        Identifier,
        Minus,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_assign() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_assign_val_in_initializer() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_assignment_in_initializer() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_assignment_lowest_precedence() {
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
        IntConstant(IntKind::Int),
        PipePipe,
        IntConstant(IntKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_empty_function_body() {
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
fn test_valid_exp_then_declaration() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Percent,
        IntConstant(IntKind::Int),
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
fn test_valid_extra_credit_bitwise_in_initializer() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        Circumflex,
        IntConstant(IntKind::Int),
        Semicolon,
        Return,
        IntConstant(IntKind::Int),
        Pipe,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_bitwise_ops_vars() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
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
fn test_valid_extra_credit_bitwise_shiftl_variable() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Return,
        Identifier,
        LessLess,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_bitwise_shiftr_assign() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        GreaterGreater,
        IntConstant(IntKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_compound_assignment_chained() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Minus,
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
        Semicolon,
        Return,
        Identifier,
        EqualEqual,
        IntConstant(IntKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        IntConstant(IntKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Minus,
        IntConstant(IntKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Minus,
        IntConstant(IntKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Minus,
        IntConstant(IntKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Minus,
        IntConstant(IntKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_compound_assignment_lowest_precedence() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        PlusEqual,
        IntConstant(IntKind::Int),
        PipePipe,
        Identifier,
        Semicolon,
        Identifier,
        StarEqual,
        Identifier,
        AmpersandAmpersand,
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        IntConstant(IntKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        IntConstant(IntKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        IntConstant(IntKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_compound_assignment_use_result() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        PlusEqual,
        IntConstant(IntKind::Int),
        Semicolon,
        Return,
        OpenParen,
        Identifier,
        EqualEqual,
        IntConstant(IntKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        IntConstant(IntKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_compound_bitwise_and() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        AmpersandEqual,
        IntConstant(IntKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_compound_bitwise_assignment_lowest_precedence() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        AmpersandEqual,
        IntConstant(IntKind::Int),
        PipePipe,
        Identifier,
        Semicolon,
        Identifier,
        CircumflexEqual,
        Identifier,
        PipePipe,
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        IntConstant(IntKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        IntConstant(IntKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        IntConstant(IntKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        IntConstant(IntKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_compound_bitwise_chained() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
        Semicolon,
        Return,
        OpenParen,
        Identifier,
        EqualEqual,
        IntConstant(IntKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        IntConstant(IntKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        IntConstant(IntKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        IntConstant(IntKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        IntConstant(IntKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        IntConstant(IntKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        IntConstant(IntKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        IntConstant(IntKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        IntConstant(IntKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        IntConstant(IntKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_compound_bitwise_or() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        PipeEqual,
        IntConstant(IntKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_compound_bitwise_shiftl() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        LessLessEqual,
        IntConstant(IntKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_compound_bitwise_shiftr() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        GreaterGreaterEqual,
        IntConstant(IntKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_compound_bitwise_xor() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        CircumflexEqual,
        IntConstant(IntKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_compound_divide() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        SlashEqual,
        IntConstant(IntKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_compound_minus() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        MinusEqual,
        IntConstant(IntKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_compound_mod() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        PercentEqual,
        IntConstant(IntKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_compound_multiply() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        StarEqual,
        IntConstant(IntKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_compound_plus() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        PlusEqual,
        IntConstant(IntKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_incr_expression_statement() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Minus,
        IntConstant(IntKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_incr_in_binary_expr() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Plus,
        Identifier,
        PlusPlus,
        Semicolon,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Plus,
        PlusPlus,
        Identifier,
        Semicolon,
        Return,
        OpenParen,
        Identifier,
        EqualEqual,
        IntConstant(IntKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        IntConstant(IntKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        IntConstant(IntKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_incr_parenthesized() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        IntConstant(IntKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Minus,
        IntConstant(IntKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        IntConstant(IntKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_postfix_incr_and_decr() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        IntConstant(IntKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        IntConstant(IntKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        IntConstant(IntKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_postfix_precedence() {
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
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        IntConstant(IntKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_prefix_incr_and_decr() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        IntConstant(IntKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        IntConstant(IntKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        IntConstant(IntKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_kw_var_names() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
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
fn test_valid_local_var_missing_return() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_mixed_precedence_assignment() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
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
fn test_valid_non_short_circuit_or() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        IntConstant(IntKind::Int),
        PipePipe,
        OpenParen,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
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
fn test_valid_null_statement() {
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
fn test_valid_null_then_return() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_return_var() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_short_circuit_and_fail() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        IntConstant(IntKind::Int),
        AmpersandAmpersand,
        OpenParen,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
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
fn test_valid_short_circuit_or() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        IntConstant(IntKind::Int),
        PipePipe,
        OpenParen,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
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
fn test_valid_unused_exp() {
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
        IntConstant(IntKind::Int),
        Plus,
        IntConstant(IntKind::Int),
        Semicolon,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_use_assignment_result() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Return,
        Identifier,
        Equal,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_use_val_in_own_initializer() {
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
        IntConstant(IntKind::Int),
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
