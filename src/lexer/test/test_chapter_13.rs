use crate::lexer::TokenKind::*;
use crate::lexer::{IntKind, tokenize};

#[test]
#[should_panic]
fn test_invalid_lex_another_bad_constant() {
    tokenize(
        r#"
        int main(void)
        {
            return 1.ex;
        }
    "#,
    );
}

#[test]
#[should_panic]
fn test_invalid_lex_bad_exponent_suffix() {
    tokenize(
        r#"
        int main(void)
        {
            double foo = 1E2x;
        }
    "#,
    );
}

#[test]
#[should_panic]
fn test_invalid_lex_malformed_const() {
    tokenize(
        r#"
        int main(void)
        {
            return 2._;
        }
    "#,
    );
}

#[test]
#[should_panic]
fn test_invalid_lex_malformed_exponent() {
    tokenize(
        r#"
        int main(void)
        {
            double d = 1.0e10.0;
            return 0;
        }
    "#,
    );
}

#[test]
#[should_panic]
fn test_invalid_lex_missing_exponent() {
    tokenize(
        r#"
        int main(void)
        {
            double foo = 30.e;
            return 4;
        }
    "#,
    );
}

#[test]
#[should_panic]
fn test_invalid_lex_missing_negative_exponent() {
    tokenize(
        r#"
        int main(void) {
            double foo = 24e-;
        }
    "#,
    );
}

#[test]
#[should_panic]
fn test_invalid_lex_yet_another_bad_constant() {
    tokenize(
        r#"
        int main(void)
        {
            return 1.e-10x;
        }
    "#,
    );
}

#[test]
fn test_valid_constants_constant_doubles() {
    let src = r#"
        int main(void) {
            double a = 1.0;
            double b = 1.;
            double c = 1E0;
            double d = .01e+2;
            if (! (a == b && a == c && a == d) )
                return 1;
            if (a + b + c + d != 4.0)
                return 2;
            double e = .125;
            double f = 12.5e-2;
            double g = 125.E-3;
            double h = 1250000000e-10;
            if (! (e == f && e == g && e == h) )
                return 3;
            if (e + f + g + h != 0.5)
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
        Double,
        Identifier,
        Equal,
        DoubleConstant,
        Semicolon,
        Double,
        Identifier,
        Equal,
        DoubleConstant,
        Semicolon,
        Double,
        Identifier,
        Equal,
        DoubleConstant,
        Semicolon,
        Double,
        Identifier,
        Equal,
        DoubleConstant,
        Semicolon,
        If,
        OpenParen,
        Bang,
        OpenParen,
        Identifier,
        EqualEqual,
        Identifier,
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Identifier,
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Identifier,
        CloseParen,
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        Plus,
        Identifier,
        Plus,
        Identifier,
        Plus,
        Identifier,
        BangEqual,
        DoubleConstant,
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        Double,
        Identifier,
        Equal,
        DoubleConstant,
        Semicolon,
        Double,
        Identifier,
        Equal,
        DoubleConstant,
        Semicolon,
        Double,
        Identifier,
        Equal,
        DoubleConstant,
        Semicolon,
        Double,
        Identifier,
        Equal,
        DoubleConstant,
        Semicolon,
        If,
        OpenParen,
        Bang,
        OpenParen,
        Identifier,
        EqualEqual,
        Identifier,
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Identifier,
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Identifier,
        CloseParen,
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        Plus,
        Identifier,
        Plus,
        Identifier,
        Plus,
        Identifier,
        BangEqual,
        DoubleConstant,
        CloseParen,
        Return,
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
fn test_valid_constants_round_constants() {
    let src = r#"
        int main(void) {
            if (1.00000000000000033306690738754696212708950042724609375 != 1.0000000000000004) {
                return 1;
            }
            if (9223372036854776832.5 != 9223372036854777856.0) {
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
        DoubleConstant,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        DoubleConstant,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_explicit_casts_cvttsd2si_rewrite() {
    let src = r#"
        double glob = 3.0;
        int main(void) {
            long l = -1l;
            int i = -1;
            int j = (int) glob;
            int k = 20;
            if (l != -1l) {
                return 1;
            }
            if (i != -1) {
                return 2;
            }
            if (j != 3) {
                return 3;
            }
            if (k != 20) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Double,
        Identifier,
        Equal,
        DoubleConstant,
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
        Minus,
        IntConstant(IntKind::Long),
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
        OpenParen,
        Int,
        CloseParen,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        IntConstant(IntKind::Long),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_explicit_casts_double_to_signed() {
    let src = r#"
        int double_to_int(double d) {
            return (int) d;
        }
        long double_to_long(double d) {
            return (long) d;
        }
        int main(void) {
            long l = double_to_long(2148429099.3);
            if (l != 2148429099l) {
                return 1;
            }
            int i = double_to_int(-200000.9999);
            if (i != -200000) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Double,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        OpenParen,
        Int,
        CloseParen,
        Identifier,
        Semicolon,
        CloseBrace,
        Long,
        Identifier,
        OpenParen,
        Double,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
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
        DoubleConstant,
        CloseParen,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        IntConstant(IntKind::Long),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        Equal,
        Identifier,
        OpenParen,
        Minus,
        DoubleConstant,
        CloseParen,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_explicit_casts_double_to_unsigned() {
    let src = r#"
        unsigned int double_to_uint(double d) {
            return (unsigned int) d;
        }
        unsigned long double_to_ulong(double d) {
            return (unsigned long) d;
        }
        int main(void) {
            if (double_to_uint(10.9) != 10u) {
                return 1;
            }
            if (double_to_uint(2147483750.5) != 2147483750) {
                return 2;
            }
            if (double_to_ulong(34359738368.5) != 34359738368ul) {
                return 3;
            }
            if (double_to_ulong(3458764513821589504.0) != 3458764513821589504ul) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Unsigned,
        Int,
        Identifier,
        OpenParen,
        Double,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        OpenParen,
        Unsigned,
        Int,
        CloseParen,
        Identifier,
        Semicolon,
        CloseBrace,
        Unsigned,
        Long,
        Identifier,
        OpenParen,
        Double,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        OpenParen,
        Unsigned,
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
        If,
        OpenParen,
        Identifier,
        OpenParen,
        DoubleConstant,
        CloseParen,
        BangEqual,
        IntConstant(IntKind::Uint),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        DoubleConstant,
        CloseParen,
        BangEqual,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        DoubleConstant,
        CloseParen,
        BangEqual,
        IntConstant(IntKind::ULong),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        DoubleConstant,
        CloseParen,
        BangEqual,
        IntConstant(IntKind::ULong),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_explicit_casts_rewrite_cvttsd2si_regression() {
    let src = r#"
        int check_12_ints(int start, int a, int b, int c, int d, int e, int f, int g,
                          int h, int i, int j, int k, int l);
        double glob = 5000.;
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
            int thirteen = glob - 4987;
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
            if (should_spill != 5000) {
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
        Double,
        Identifier,
        Equal,
        DoubleConstant,
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
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        Star,
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        Minus,
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Minus,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Plus,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Minus,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Minus,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        Star,
        IntConstant(IntKind::Int),
        Minus,
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Plus,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
        CloseParen,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        Minus,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_explicit_casts_signed_to_double() {
    let src = r#"
        
        double int_to_double(int i) {
            return (double) i;
        }
        double long_to_double(long l) {
            return (double) l;
        }
        int main(void) {
            if (int_to_double(-100000) != -100000.0) {
                return 1;
            }
            if (long_to_double(-9007199254751227l) != -9007199254751228.0) {
                return 2;
            }
            double d = (double) 1152921504606846977l;
            if (d != 1152921504606846976.0) {
                return 3;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Double,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        OpenParen,
        Double,
        CloseParen,
        Identifier,
        Semicolon,
        CloseBrace,
        Double,
        Identifier,
        OpenParen,
        Long,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        OpenParen,
        Double,
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
        If,
        OpenParen,
        Identifier,
        OpenParen,
        Minus,
        IntConstant(IntKind::Int),
        CloseParen,
        BangEqual,
        Minus,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        Minus,
        IntConstant(IntKind::Long),
        CloseParen,
        BangEqual,
        Minus,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Double,
        Identifier,
        Equal,
        OpenParen,
        Double,
        CloseParen,
        IntConstant(IntKind::Long),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_explicit_casts_unsigned_to_double() {
    let src = r#"
        
        double uint_to_double(unsigned int ui) {
            return (double) ui;
        }
        double ulong_to_double(unsigned long ul) {
            return (double) ul;
        }
        int main(void) {
            if (uint_to_double(1000u) != 1000.0) {
                return 1;
            }
            if (uint_to_double(4294967200u) != 4294967200.0) {
                return 2;
            }
            if (ulong_to_double(138512825844ul) != 138512825844.0) {
                return 3;
            }
            if (ulong_to_double(10223372036854775816ul) != 10223372036854775808.0) {
                return 4;
            }
            if (ulong_to_double(9223372036854776832ul) != 9223372036854775808.0) {
                return 5;
            }
            if (ulong_to_double(9223372036854776833ul) != 9223372036854777856.0) {
                return 6;
            }
            if (ulong_to_double(9223372036854776831ul) != 9223372036854775808.0) {
                return 7;
            }
            if (ulong_to_double(9223372036854776830ul) != 9223372036854775808.0) {
                return 8;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Double,
        Identifier,
        OpenParen,
        Unsigned,
        Int,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        OpenParen,
        Double,
        CloseParen,
        Identifier,
        Semicolon,
        CloseBrace,
        Double,
        Identifier,
        OpenParen,
        Unsigned,
        Long,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        OpenParen,
        Double,
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
        If,
        OpenParen,
        Identifier,
        OpenParen,
        IntConstant(IntKind::Uint),
        CloseParen,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        IntConstant(IntKind::Uint),
        CloseParen,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        IntConstant(IntKind::ULong),
        CloseParen,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        IntConstant(IntKind::ULong),
        CloseParen,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        IntConstant(IntKind::ULong),
        CloseParen,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        IntConstant(IntKind::ULong),
        CloseParen,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        IntConstant(IntKind::ULong),
        CloseParen,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        IntConstant(IntKind::ULong),
        CloseParen,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_compound_assign() {
    let src = r#"
        
        int main(void) {
            double d = 10.0;
            d /= 4.0;
            if (d != 2.5) {
                return 1;
            }
            d *= 10000.0;
            if (d != 25000.0) {
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
        Double,
        Identifier,
        Equal,
        DoubleConstant,
        Semicolon,
        Identifier,
        SlashEqual,
        DoubleConstant,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        StarEqual,
        DoubleConstant,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_compound_assign_implicit_cast() {
    let src = r#"
        int main(void) {
            double d = 1000.5;
            d += 1000;
            if (d != 2000.5) {
                return 1;
            }
            unsigned long ul = 18446744073709551586ul;
            ul -= 1.5E19;
            if (ul != 3446744073709551616ul) {
                return 2;
            }
            int i = 10;
            i += 0.99999;
            if (i != 10) {
                return 3;
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
        Double,
        Identifier,
        Equal,
        DoubleConstant,
        Semicolon,
        Identifier,
        PlusEqual,
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Unsigned,
        Long,
        Identifier,
        Equal,
        IntConstant(IntKind::ULong),
        Semicolon,
        Identifier,
        MinusEqual,
        DoubleConstant,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        IntConstant(IntKind::ULong),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        PlusEqual,
        DoubleConstant,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_incr_and_decr() {
    let src = r#"
        
        int main(void) {
            static double d = 0.75;
            if (d++ != 0.75) {
                return 1;
            }
            if (d != 1.75) {
                return 2;
            }
            d = -100.2;
            if (++d != -99.2) {
                return 3;
            }
            if (d != -99.2) {
                return 4;
            }
            if (d-- != -99.2) {
                return 5;
            }
            if (d != -100.2) {
                return 6;
            }
            if (--d != -101.2) {
                return 7;
            }
            if (d != -101.2) {
                return 8;
            }
            d = 0.000000000000000000001;
            d++;
            if (d != 1.0) {
                return 9;
            }
            d = 10e20;
            d--;
            if (d != 10e20) {
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
        Static,
        Double,
        Identifier,
        Equal,
        DoubleConstant,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        PlusPlus,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Minus,
        DoubleConstant,
        Semicolon,
        If,
        OpenParen,
        PlusPlus,
        Identifier,
        BangEqual,
        Minus,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        MinusMinus,
        BangEqual,
        Minus,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        MinusMinus,
        Identifier,
        BangEqual,
        Minus,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        DoubleConstant,
        Semicolon,
        Identifier,
        PlusPlus,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        DoubleConstant,
        Semicolon,
        Identifier,
        MinusMinus,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_nan() {
    let src = r#"
        int double_isnan(double d);
        int main(void) {
            static double zero = 0.0;
            double nan = 0.0 / zero;
            if (nan < 0.0 || nan == 0.0 || nan > 0.0 || nan <= 0.0 || nan >= 0.0)
                return 1;
            if (1 < nan || 1 == nan || 1 > nan || 1 <= nan || 1 >= nan)
                return 2;
            if (nan == nan)
                return 3;
            if (!(nan != nan)) {
                return 4;
            }
            if (!double_isnan(nan)) {
                return 5;
            }
            if (!double_isnan(4 * nan)) {
                return 6;
            }
            if (!double_isnan(22e2 / nan)) {
                return 7;
            }
            if (!double_isnan(-nan)) {
                return 8;
            }
            if (!nan) {
                return 9;
            }
            if (nan) {
            } else {
                return 10;
            }
            int nan_is_nonzero;
            for (nan_is_nonzero = 0; nan;) {
                nan_is_nonzero = 1;
                break;
            }
            if (!nan_is_nonzero) {
                return 11;
            }
            nan_is_nonzero = 0;
            while (nan) {
                nan_is_nonzero = 1;
                break;
            }
            if (!nan_is_nonzero) {
                return 12;
            }
            nan_is_nonzero = -1;
            do {
                nan_is_nonzero = nan_is_nonzero + 1;
                if (nan_is_nonzero) {
                    break;
                }
            } while (nan);
            if (!nan_is_nonzero) {
                return 13;
            }
            nan_is_nonzero = nan ? 1 : 0;
            if (!nan_is_nonzero) {
                return 14;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Double,
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
        Double,
        Identifier,
        Equal,
        DoubleConstant,
        Semicolon,
        Double,
        Identifier,
        Equal,
        DoubleConstant,
        Slash,
        Identifier,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        Less,
        DoubleConstant,
        PipePipe,
        Identifier,
        EqualEqual,
        DoubleConstant,
        PipePipe,
        Identifier,
        Greater,
        DoubleConstant,
        PipePipe,
        Identifier,
        LessEqual,
        DoubleConstant,
        PipePipe,
        Identifier,
        GreaterEqual,
        DoubleConstant,
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        IntConstant(IntKind::Int),
        Less,
        Identifier,
        PipePipe,
        IntConstant(IntKind::Int),
        EqualEqual,
        Identifier,
        PipePipe,
        IntConstant(IntKind::Int),
        Greater,
        Identifier,
        PipePipe,
        IntConstant(IntKind::Int),
        LessEqual,
        Identifier,
        PipePipe,
        IntConstant(IntKind::Int),
        GreaterEqual,
        Identifier,
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        EqualEqual,
        Identifier,
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Bang,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        IntConstant(IntKind::Int),
        Star,
        Identifier,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        DoubleConstant,
        Slash,
        Identifier,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        Minus,
        Identifier,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        CloseParen,
        OpenBrace,
        CloseBrace,
        Else,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        Semicolon,
        For,
        OpenParen,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        Semicolon,
        CloseParen,
        OpenBrace,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Break,
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        While,
        OpenParen,
        Identifier,
        CloseParen,
        OpenBrace,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Break,
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Minus,
        IntConstant(IntKind::Int),
        Semicolon,
        Do,
        OpenBrace,
        Identifier,
        Equal,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        CloseParen,
        OpenBrace,
        Break,
        Semicolon,
        CloseBrace,
        CloseBrace,
        While,
        OpenParen,
        Identifier,
        CloseParen,
        Semicolon,
        If,
        OpenParen,
        Bang,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Identifier,
        Question,
        IntConstant(IntKind::Int),
        Colon,
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Bang,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_nan_compound_assign() {
    let src = r#"
        int double_isnan(double d);
        int main(void) {
            static double zero = 0.0;
            double nan = 0.0 / zero;
            if (!double_isnan(nan += 99.2)) {
                return 1;
            }
            if (!double_isnan(nan -= nan)) {
                return 2;
            }
            if (!double_isnan(nan *= 4.0)) {
                return 3;
            }
            if (!double_isnan(nan /= 0.0)) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Double,
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
        Double,
        Identifier,
        Equal,
        DoubleConstant,
        Semicolon,
        Double,
        Identifier,
        Equal,
        DoubleConstant,
        Slash,
        Identifier,
        Semicolon,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        Identifier,
        PlusEqual,
        DoubleConstant,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        Identifier,
        MinusEqual,
        Identifier,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        Identifier,
        StarEqual,
        DoubleConstant,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        Identifier,
        SlashEqual,
        DoubleConstant,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_nan_incr_and_decr() {
    let src = r#"
        int double_isnan(double d);
        int main(void) {
            static double zero = 0.0;
            double nan = 0.0 / zero;
            if (!double_isnan(++nan)) {
                return 1;
            }
            if (!double_isnan(--nan)) {
                return 2;
            }
            if (!double_isnan(nan++)) {
                return 3;
            }
            if (!double_isnan(nan--)) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Double,
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
        Double,
        Identifier,
        Equal,
        DoubleConstant,
        Semicolon,
        Double,
        Identifier,
        Equal,
        DoubleConstant,
        Slash,
        Identifier,
        Semicolon,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        PlusPlus,
        Identifier,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        MinusMinus,
        Identifier,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        Identifier,
        PlusPlus,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        Identifier,
        MinusMinus,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_floating_expressions_arithmetic_ops() {
    let src = r#"
        double point_one = 0.1;
        double point_two = 0.2;
        double point_three = 0.3;
        double two = 2.0;
        double three = 3.0;
        double four = 4.0;
        double twelveE30 = 12e30;
        int addition(void) {
            return (point_one + point_two == 0.30000000000000004);
        }
        int subtraction(void) {
            return (four - 1.0 == 3.0);
        }
        int multiplication(void) {
            return (0.01 * point_three == 0.003);
        }
        int division(void) {
            return (7.0 / two == 3.5);
        }
        int negation(void) {
            double neg = -twelveE30;
            return !(12e30 + neg);
        }
        int complex_expression(void) {
            double complex_expression = (two + three) - 127.5 * four;
            return complex_expression == -505.0;
        }
        int main(void) {
            if (!addition()) {
                return 1;
            }
            if (!subtraction()){
                return 2;
            }
            if (!multiplication()) {
                return 3;
            }
            if (!division()) {
                return 4;
            }
            if (!negation()) {
                return 5;
            }
            if (!complex_expression()) {
                return 5;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Double,
        Identifier,
        Equal,
        DoubleConstant,
        Semicolon,
        Double,
        Identifier,
        Equal,
        DoubleConstant,
        Semicolon,
        Double,
        Identifier,
        Equal,
        DoubleConstant,
        Semicolon,
        Double,
        Identifier,
        Equal,
        DoubleConstant,
        Semicolon,
        Double,
        Identifier,
        Equal,
        DoubleConstant,
        Semicolon,
        Double,
        Identifier,
        Equal,
        DoubleConstant,
        Semicolon,
        Double,
        Identifier,
        Equal,
        DoubleConstant,
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
        DoubleConstant,
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
        DoubleConstant,
        EqualEqual,
        DoubleConstant,
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
        DoubleConstant,
        Star,
        Identifier,
        EqualEqual,
        DoubleConstant,
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
        DoubleConstant,
        Slash,
        Identifier,
        EqualEqual,
        DoubleConstant,
        CloseParen,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Double,
        Identifier,
        Equal,
        Minus,
        Identifier,
        Semicolon,
        Return,
        Bang,
        OpenParen,
        DoubleConstant,
        Plus,
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
        Double,
        Identifier,
        Equal,
        OpenParen,
        Identifier,
        Plus,
        Identifier,
        CloseParen,
        Minus,
        DoubleConstant,
        Star,
        Identifier,
        Semicolon,
        Return,
        Identifier,
        EqualEqual,
        Minus,
        DoubleConstant,
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
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_floating_expressions_comparisons() {
    let src = r#"
        double fifty_fiveE5 = 55e5;
        double fifty_fourE4 = 54e4;
        double tiny = .00004;
        double four = 4.;
        double point_one = 0.1;
        int main(void) {
            if (fifty_fiveE5 < fifty_fourE4) {
                return 1;
            }
            if (four > 4.0) {
                return 2;
            }
            if (tiny <= 0.0) {
                return 3;
            }
            if (fifty_fourE4 >= fifty_fiveE5) {
                return 4;
            }
            if (tiny == 0.0) {
                return 5;
            }
            if (point_one != point_one) {
                return 6;
            }
            if (!(tiny > 00.000005)) {
                return 7;
            }
            if (!(-.00004 < four)) {
                return 8;
            }
            if (!(tiny <= tiny)) {
                return 9;
            }
            if (!(fifty_fiveE5 >= fifty_fiveE5)) {
                return 10;
            }
            if (!(0.1 == point_one)) {
                return 11;
            }
            if (!(tiny != .00003)) {
                return 12;
            }
            if (0.00003 < 0.000000000003) {
                return 13;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Double,
        Identifier,
        Equal,
        DoubleConstant,
        Semicolon,
        Double,
        Identifier,
        Equal,
        DoubleConstant,
        Semicolon,
        Double,
        Identifier,
        Equal,
        DoubleConstant,
        Semicolon,
        Double,
        Identifier,
        Equal,
        DoubleConstant,
        Semicolon,
        Double,
        Identifier,
        Equal,
        DoubleConstant,
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
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        Greater,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        LessEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        GreaterEqual,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        EqualEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        OpenParen,
        Identifier,
        Greater,
        DoubleConstant,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        OpenParen,
        Minus,
        DoubleConstant,
        Less,
        Identifier,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        OpenParen,
        Identifier,
        LessEqual,
        Identifier,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        OpenParen,
        Identifier,
        GreaterEqual,
        Identifier,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        OpenParen,
        DoubleConstant,
        EqualEqual,
        Identifier,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        OpenParen,
        Identifier,
        BangEqual,
        DoubleConstant,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        DoubleConstant,
        Less,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_floating_expressions_logical() {
    let src = r#"
        double zero = 0.0;
        double non_zero = 1E-20;
        double one = 1.0;
        double rounded_to_zero = 1e-330;
        int main(void) {
            if (zero) {
                return 1;
            }
            if (rounded_to_zero) {
                return 2;
            }
            if (non_zero) {
            } else {
                return 3;
            }
            if (0.e10) {
                return 4;
            }
            if (!non_zero) {
                return 4;
            }
            if (!(!zero)) {
                return 5;
            }
            if (!(!rounded_to_zero)) {
                return 6;
            }
            if (!(non_zero && 1.0)) {
                return 8;
            }
            if (3.0 && zero) {
                return 8;
            }
            if (rounded_to_zero && 1000e10) {
                return 9;
            }
            if (18446744073709551615UL && zero) {
                return 10;
            }
            if (!(non_zero && 5l)) {
                return 11;
            }
            if (!(5.0 || zero)) {
                return 12;
            }
            if (zero || rounded_to_zero) {
                return 13;
            }
            if (!(rounded_to_zero || 0.0001)) {
                return 14;
            }
            if (!(non_zero || 0u)) {
                return 15;
            }
            if (!(0 || 0.0000005)) {
                return 16;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Double,
        Identifier,
        Equal,
        DoubleConstant,
        Semicolon,
        Double,
        Identifier,
        Equal,
        DoubleConstant,
        Semicolon,
        Double,
        Identifier,
        Equal,
        DoubleConstant,
        Semicolon,
        Double,
        Identifier,
        Equal,
        DoubleConstant,
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
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        CloseParen,
        OpenBrace,
        CloseBrace,
        Else,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        OpenParen,
        Bang,
        Identifier,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        OpenParen,
        Bang,
        Identifier,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        OpenParen,
        Identifier,
        AmpersandAmpersand,
        DoubleConstant,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        DoubleConstant,
        AmpersandAmpersand,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        AmpersandAmpersand,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        IntConstant(IntKind::ULong),
        AmpersandAmpersand,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        OpenParen,
        Identifier,
        AmpersandAmpersand,
        IntConstant(IntKind::Long),
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        OpenParen,
        DoubleConstant,
        PipePipe,
        Identifier,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        PipePipe,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        OpenParen,
        Identifier,
        PipePipe,
        DoubleConstant,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        OpenParen,
        Identifier,
        PipePipe,
        IntConstant(IntKind::Uint),
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        OpenParen,
        IntConstant(IntKind::Int),
        PipePipe,
        DoubleConstant,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_floating_expressions_loop_controlling_expression() {
    let src = r#"
        int main(void) {
            int a = 0;
            for(double d = 100.0; d > 0.0; d = d - 1.0) {
                a = a + 1;
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
        IntConstant(IntKind::Int),
        Semicolon,
        For,
        OpenParen,
        Double,
        Identifier,
        Equal,
        DoubleConstant,
        Semicolon,
        Identifier,
        Greater,
        DoubleConstant,
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Minus,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Identifier,
        Equal,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
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
fn test_valid_floating_expressions_simple() {
    let src = r#"
        
        int main(void) {
            double x = 2.0;
            return (x * 2.0 == 4.0);
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Double,
        Identifier,
        Equal,
        DoubleConstant,
        Semicolon,
        Return,
        OpenParen,
        Identifier,
        Star,
        DoubleConstant,
        EqualEqual,
        DoubleConstant,
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_floating_expressions_static_initialized_double() {
    let src = r#"
        double return_static_variable(void) {
            static double d = 0.5;
            double ret = d;
            d = d + 1.0;
            return ret;
        }
        int main(void) {
            double d1 = return_static_variable();
            double d2 = return_static_variable();
            double d3 = return_static_variable();
            if (d1 != 0.5) {
                return 1;
            }
            if (d2 != 1.5) {
                return 2;
            }
            if (d3 != 2.5) {
                return 3;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Double,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Static,
        Double,
        Identifier,
        Equal,
        DoubleConstant,
        Semicolon,
        Double,
        Identifier,
        Equal,
        Identifier,
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        DoubleConstant,
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
        Double,
        Identifier,
        Equal,
        Identifier,
        OpenParen,
        CloseParen,
        Semicolon,
        Double,
        Identifier,
        Equal,
        Identifier,
        OpenParen,
        CloseParen,
        Semicolon,
        Double,
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
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_function_calls_double_and_int_parameters() {
    let src = r#"
        int check_arguments(double d1, double d2, int i1, double d3, double d4, int i2, int i3,
                            int i4, double d5, double d6, double d7, int i5, double d8) {
            if (d1 != 1.0) {
                return 1;
            }
            if (d2 != 2.0) {
                return 2;
            }
            if (d3 != 3.0) {
                return 3;
            }
            if (d4 != 4.0 ){
                return 4;
            }
            if (d5 != 5.0){
                return 5;
            }
            if (d6 != 6.0 ){
                return 6;
            }
            if (d7 != 7.0 ){
                return 7;
            }
            if (d8 != 8.0 ){
                return 8;
            }
            if (i1 != 101 ){
                return 9;
            }
            if (i2 != 102 ){
                return 10;
            }
            if (i3 != 103){
                return 11;
            }
            if (i4 != 104) {
                return 12;
            }
            if (i5 != 105) {
                return 13;
            }
            return 0;
        }
        int main(void) {
            return check_arguments(1.0, 2.0, 101, 3.0, 4.0, 102, 103, 104, 5.0, 6.0, 7.0, 105, 8.0);
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Double,
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
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Double,
        Identifier,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        IntConstant(IntKind::Int),
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
        DoubleConstant,
        Comma,
        DoubleConstant,
        Comma,
        IntConstant(IntKind::Int),
        Comma,
        DoubleConstant,
        Comma,
        DoubleConstant,
        Comma,
        IntConstant(IntKind::Int),
        Comma,
        IntConstant(IntKind::Int),
        Comma,
        IntConstant(IntKind::Int),
        Comma,
        DoubleConstant,
        Comma,
        DoubleConstant,
        Comma,
        DoubleConstant,
        Comma,
        IntConstant(IntKind::Int),
        Comma,
        DoubleConstant,
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_function_calls_double_and_int_params_recursive() {
    let src = r#"
        int fun(int i1, double d1, int i2, double d2, int i3, double d3,
                int i4, double d4, int i5, double d5, int i6, double d6,
                int i7, double d7, int i8, double d8, int i9, double d9) {
            if (i1 != d9) {
                int call1 = fun(i1 + 1, d1, i2 + 1, d2, i3 + 1, d3, i4 + 1, d4, i5 + 1, d5, i6 + 1, d6, i7 + 1, d7, i8 + 1, d8, i9 + 1, d9);
                int call2 = fun(i1, d1 - 1, i2, d2 - 1, i3, d3 - 1, i4, d4 - 1, i5, d5 - 1, i6, d6 - 1, i7, d7 - 1, i8, d8 - 1, i9, d9 - 1);
                if (call1) {
                    return call1;
                }
                if (call2) {
                    return call2;
                }
            }
            if (i2 != i1 + 2) {
                return 2;
            }
            if (i3 != i1 + 4) {
                return 3;
            }
            if (i4 != i1 + 6) {
                return 4;
            }
            if (i5 != i1 + 8) {
                return 5;
            }
            if (i6 != i1 + 10) {
                return 6;
            }
            if (i7 != i1 + 12) {
                return 7;
            }
            if (i8 != i1 + 14) {
                return 8;
            }
            if (i9 != i1 + 16) {
                return 9;
            }
            if (d1 != d9 - 16) {
                return 11;
            }
            if (d2 != d9 - 14) {
                return 12;
            }
            if (d3 != d9 - 12) {
                return 13;
            }
            if (d4 != d9 - 10) {
                return 14;
            }
            if (d5 != d9 - 8) {
                return 15;
            }
            if (d6 != d9 - 6) {
                return 16;
            }
            if (d7 != d9 - 4) {
                return 17;
            }
            if (d8 != d9 - 2) {
                return 18;
            }
            return 0;
        }
        int main(void) {
            return fun(1, 2.0, 3, 4.0, 5, 6.0, 7, 8.0, 9, 10.0, 11, 12.0, 13, 14.0, 15, 16.0, 17, 18.0);
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Double,
        Identifier,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Identifier,
        OpenParen,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
        Comma,
        Identifier,
        Comma,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
        Comma,
        Identifier,
        Comma,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
        Comma,
        Identifier,
        Comma,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
        Comma,
        Identifier,
        Comma,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
        Comma,
        Identifier,
        Comma,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
        Comma,
        Identifier,
        Comma,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
        Comma,
        Identifier,
        Comma,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
        Comma,
        Identifier,
        Comma,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
        Comma,
        Identifier,
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        OpenParen,
        Identifier,
        Comma,
        Identifier,
        Minus,
        IntConstant(IntKind::Int),
        Comma,
        Identifier,
        Comma,
        Identifier,
        Minus,
        IntConstant(IntKind::Int),
        Comma,
        Identifier,
        Comma,
        Identifier,
        Minus,
        IntConstant(IntKind::Int),
        Comma,
        Identifier,
        Comma,
        Identifier,
        Minus,
        IntConstant(IntKind::Int),
        Comma,
        Identifier,
        Comma,
        Identifier,
        Minus,
        IntConstant(IntKind::Int),
        Comma,
        Identifier,
        Comma,
        Identifier,
        Minus,
        IntConstant(IntKind::Int),
        Comma,
        Identifier,
        Comma,
        Identifier,
        Minus,
        IntConstant(IntKind::Int),
        Comma,
        Identifier,
        Comma,
        Identifier,
        Minus,
        IntConstant(IntKind::Int),
        Comma,
        Identifier,
        Comma,
        Identifier,
        Minus,
        IntConstant(IntKind::Int),
        CloseParen,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        Minus,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        Minus,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        Minus,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        Minus,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        Minus,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        Minus,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        Minus,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        Minus,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
        Comma,
        DoubleConstant,
        Comma,
        IntConstant(IntKind::Int),
        Comma,
        DoubleConstant,
        Comma,
        IntConstant(IntKind::Int),
        Comma,
        DoubleConstant,
        Comma,
        IntConstant(IntKind::Int),
        Comma,
        DoubleConstant,
        Comma,
        IntConstant(IntKind::Int),
        Comma,
        DoubleConstant,
        Comma,
        IntConstant(IntKind::Int),
        Comma,
        DoubleConstant,
        Comma,
        IntConstant(IntKind::Int),
        Comma,
        DoubleConstant,
        Comma,
        IntConstant(IntKind::Int),
        Comma,
        DoubleConstant,
        Comma,
        IntConstant(IntKind::Int),
        Comma,
        DoubleConstant,
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_function_calls_double_parameters() {
    let src = r#"
        
        int check_arguments(double a, double b, double c, double d, double e, double f, double g, double h);
        int main(void) {
            return check_arguments(1.0, 2.0, 3.0, 4.0, -1.0, -2.0, -3.0, -4.0);
        }
        int check_arguments(double a, double b, double c, double d, double e, double f, double g, double h) {
            if (a != 1.0) {
                return 1;
            }
            if (b != 2.0) {
                return 2;
            }
            if (c != 3.0) {
                return 3;
            }
            if (d != 4.0) {
                return 4;
            }
            if (e != -1.0) {
                return 5;
            }
            if (f != -2.0) {
                return 6;
            }
            if (g != -3.0) {
                return 7;
            }
            if (h != -4.0) {
                return 8;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Double,
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
        DoubleConstant,
        Comma,
        DoubleConstant,
        Comma,
        DoubleConstant,
        Comma,
        DoubleConstant,
        Comma,
        Minus,
        DoubleConstant,
        Comma,
        Minus,
        DoubleConstant,
        Comma,
        Minus,
        DoubleConstant,
        Comma,
        Minus,
        DoubleConstant,
        CloseParen,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_function_calls_push_xmm() {
    let src = r#"
        int callee(double a, double b, double c, double d, double e, double f, double g,
                   double h, double i, double j, double k) {
            if (a != 0.) {
                return 1;
            }
            if (b != 1.) {
                return 2;
            }
            if (c != 2.) {
                return 3;
            }
            if (d != 3.) {
                return 4;
            }
            if (e != 4.) {
                return 5;
            }
            if (f != 5.) {
                return 6;
            }
            if (g != 6.) {
                return 7;
            }
            if (h != 7.) {
                return 8;
            }
            if (i != 8.) {
                return 9;
            }
            if (j != 9.) {
                return 10;
            }
            if (k != 10.) {
                return 11;
            }
            return 0;
        }
        int target(int a, int b, int c, int d, int e) {
            return callee(0., 1., 2., 3., 4., 5., e + 1., d + 3., c + 5., b + 7.,
                          a + 9.);
        }
        int main(void) {
            return target(1, 2, 3, 4, 5);
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        IntConstant(IntKind::Int),
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
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        OpenParen,
        DoubleConstant,
        Comma,
        DoubleConstant,
        Comma,
        DoubleConstant,
        Comma,
        DoubleConstant,
        Comma,
        DoubleConstant,
        Comma,
        DoubleConstant,
        Comma,
        Identifier,
        Plus,
        DoubleConstant,
        Comma,
        Identifier,
        Plus,
        DoubleConstant,
        Comma,
        Identifier,
        Plus,
        DoubleConstant,
        Comma,
        Identifier,
        Plus,
        DoubleConstant,
        Comma,
        Identifier,
        Plus,
        DoubleConstant,
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
        IntConstant(IntKind::Int),
        Comma,
        IntConstant(IntKind::Int),
        Comma,
        IntConstant(IntKind::Int),
        Comma,
        IntConstant(IntKind::Int),
        Comma,
        IntConstant(IntKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_function_calls_return_double() {
    let src = r#"
        
        double d(void) {
            return 1234.e75;
        }
        int main(void) {
            double retval = d();
            return retval == 1234.e75;
        }
    "#;
    let expected = vec![
        Double,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        DoubleConstant,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Double,
        Identifier,
        Equal,
        Identifier,
        OpenParen,
        CloseParen,
        Semicolon,
        Return,
        Identifier,
        EqualEqual,
        DoubleConstant,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_function_calls_standard_library_call() {
    let src = r#"
        double fma(double x, double y, double z);
        double ldexp(double x, int exp);
        int main(void) {
            double fma_result = fma(5.0, 1E22, 4000000.0);
            double ldexp_result = ldexp(92E73, 5);
            if (fma_result != 50000000000000004194304.0) {
                return 1;
            }
            if (ldexp_result != 2.944E76) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Double,
        Identifier,
        OpenParen,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        CloseParen,
        Semicolon,
        Double,
        Identifier,
        OpenParen,
        Double,
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
        Double,
        Identifier,
        Equal,
        Identifier,
        OpenParen,
        DoubleConstant,
        Comma,
        DoubleConstant,
        Comma,
        DoubleConstant,
        CloseParen,
        Semicolon,
        Double,
        Identifier,
        Equal,
        Identifier,
        OpenParen,
        DoubleConstant,
        Comma,
        IntConstant(IntKind::Int),
        CloseParen,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_function_calls_use_arg_after_fun_call() {
    let src = r#"
        double fun(double x) {
            if (x > 2)
                return x;
            else {
                double ret = fun(x + 2);
                return ret + x;
            }
        }
        int main(void) {
            return fun(1.0);
        }
    "#;
    let expected = vec![
        Double,
        Identifier,
        OpenParen,
        Double,
        Identifier,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Identifier,
        Greater,
        IntConstant(IntKind::Int),
        CloseParen,
        Return,
        Identifier,
        Semicolon,
        Else,
        OpenBrace,
        Double,
        Identifier,
        Equal,
        Identifier,
        OpenParen,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
        CloseParen,
        Semicolon,
        Return,
        Identifier,
        Plus,
        Identifier,
        Semicolon,
        CloseBrace,
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
        DoubleConstant,
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_implicit_casts_common_type() {
    let src = r#"
        int lt(double d, long l) {
            return d < l;
        }
        double tern_double_flag(double flag) {
            return (double) (flag ? -30 : 10ul);
        }
        double tern_double_result(int flag) {
            return flag ? 5.0 : 9223372036854777850ul;
        }
        int ten = 10;
        int multiply(void) {
            int i = 10.75 * ten;
            return i == 107;
        }
        int main(void) {
            if (lt(-9007199254751228.0, -9007199254751227l)) {
                return 1;
            }
            if (tern_double_flag(20.0) != 18446744073709551586.0) {
                return 2;
            }
            if (tern_double_flag(0.0) != 10.0) {
                return 3;
            }
            if (tern_double_result(1) != 5.0) {
                return 4;
            }
            if (tern_double_result(0) != 9223372036854777856.0) {
                return 5;
            }
            if (!multiply()) {
                return 6;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Double,
        Identifier,
        Comma,
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
        Double,
        Identifier,
        OpenParen,
        Double,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        OpenParen,
        Double,
        CloseParen,
        OpenParen,
        Identifier,
        Question,
        Minus,
        IntConstant(IntKind::Int),
        Colon,
        IntConstant(IntKind::ULong),
        CloseParen,
        Semicolon,
        CloseBrace,
        Double,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Question,
        DoubleConstant,
        Colon,
        IntConstant(IntKind::ULong),
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
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
        DoubleConstant,
        Star,
        Identifier,
        Semicolon,
        Return,
        Identifier,
        EqualEqual,
        IntConstant(IntKind::Int),
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
        Minus,
        DoubleConstant,
        Comma,
        Minus,
        IntConstant(IntKind::Long),
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        DoubleConstant,
        CloseParen,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        DoubleConstant,
        CloseParen,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        IntConstant(IntKind::Int),
        CloseParen,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        IntConstant(IntKind::Int),
        CloseParen,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_implicit_casts_complex_arithmetic_common_type() {
    let src = r#"
        unsigned long ul = 10000ul;
        int main(void) {
            int i = -50;
            double d = (ul + i) * 3.125;
            return d == 31093.75;
        }
    "#;
    let expected = vec![
        Unsigned,
        Long,
        Identifier,
        Equal,
        IntConstant(IntKind::ULong),
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
        Minus,
        IntConstant(IntKind::Int),
        Semicolon,
        Double,
        Identifier,
        Equal,
        OpenParen,
        Identifier,
        Plus,
        Identifier,
        CloseParen,
        Star,
        DoubleConstant,
        Semicolon,
        Return,
        Identifier,
        EqualEqual,
        DoubleConstant,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_implicit_casts_convert_for_assignment() {
    let src = r#"
        int check_args(long l, double d) {
            return l == 2 && d == -6.0;
        }
        double return_double(void) {
            return 18446744073709551586ul;
        }
        int check_assignment(double arg) {
            int i = 0;
            i = arg;
            return i == 4;
        }
        int main(void) {
            if (!check_args(2.4, -6)) {
                return 1;
            }
            if (return_double() != 18446744073709551616.0) {
                return 2;
            }
            if (!check_assignment(4.9)) {
                return 3;
            }
            double d = 18446744073709551586ul;
            if (d != 18446744073709551616.) {
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
        Comma,
        Double,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        EqualEqual,
        IntConstant(IntKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Minus,
        DoubleConstant,
        Semicolon,
        CloseBrace,
        Double,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::ULong),
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Double,
        Identifier,
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
        Semicolon,
        Return,
        Identifier,
        EqualEqual,
        IntConstant(IntKind::Int),
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
        DoubleConstant,
        Comma,
        Minus,
        IntConstant(IntKind::Int),
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        CloseParen,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Bang,
        Identifier,
        OpenParen,
        DoubleConstant,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Double,
        Identifier,
        Equal,
        IntConstant(IntKind::ULong),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_implicit_casts_static_initializers() {
    let src = r#"
        double d1 = 2147483647;
        double d2 = 4294967295u;
        double d3 = 4611686018427389440l;
        double d4 = 4611686018427389955l;
        double d5 = 9223372036854775810ul;
        double d6 = 4611686018427389955ul;
        double d7 = 9223372036854776832ul;
        double uninitialized;
        static int i = 4.9;
        int unsigned u = 42949.672923E5;
        long l = 4611686018427389440.;
        unsigned long ul = 18446744073709549568.;
        int main(void) {
            if (d1 != 2147483647.) {
                return 1;
            }
            if (d2 != 4294967295.) {
                return 2;
            }
            if (d3 != 4611686018427389952.) {
                return 3;
            }
            if (d4 != d3) {
                return 4;
            }
            if (d5 != 9223372036854775808.) {
                return 5;
            }
            if (d6 != d3) {
                return 6;
            }
            if (d7 != d5) {
                return 7;
            }
            if (uninitialized) {
                return 8;
            }
            if (i != 4) {
                return 9;
            }
            if (u != 4294967292u) {
                return 10;
            }
            if (l != 4611686018427389952l) {
                return 11;
            }
            if (ul != 18446744073709549568ul) {
                return 12;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Double,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Double,
        Identifier,
        Equal,
        IntConstant(IntKind::Uint),
        Semicolon,
        Double,
        Identifier,
        Equal,
        IntConstant(IntKind::Long),
        Semicolon,
        Double,
        Identifier,
        Equal,
        IntConstant(IntKind::Long),
        Semicolon,
        Double,
        Identifier,
        Equal,
        IntConstant(IntKind::ULong),
        Semicolon,
        Double,
        Identifier,
        Equal,
        IntConstant(IntKind::ULong),
        Semicolon,
        Double,
        Identifier,
        Equal,
        IntConstant(IntKind::ULong),
        Semicolon,
        Double,
        Identifier,
        Semicolon,
        Static,
        Int,
        Identifier,
        Equal,
        DoubleConstant,
        Semicolon,
        Int,
        Unsigned,
        Identifier,
        Equal,
        DoubleConstant,
        Semicolon,
        Long,
        Identifier,
        Equal,
        DoubleConstant,
        Semicolon,
        Unsigned,
        Long,
        Identifier,
        Equal,
        DoubleConstant,
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
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        IntConstant(IntKind::Uint),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        IntConstant(IntKind::Long),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        IntConstant(IntKind::ULong),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_libraries_double_and_int_params_recursive() {
    let src = r#"
        int fun(int i1, double d1, int i2, double d2, int i3, double d3,
                int i4, double d4, int i5, double d5, int i6, double d6,
                int i7, double d7, int i8, double d8, int i9, double d9) {
            if (i1 != d9) {
                int call1 = fun(i1 + 1, d1, i2 + 1, d2, i3 + 1, d3, i4 + 1, d4, i5 + 1, d5, i6 + 1, d6, i7 + 1, d7, i8 + 1, d8, i9 + 1, d9);
                int call2 = fun(i1, d1 - 1, i2, d2 - 1, i3, d3 - 1, i4, d4 - 1, i5, d5 - 1, i6, d6 - 1, i7, d7 - 1, i8, d8 - 1, i9, d9 - 1);
                if (call1) {
                    return call1;
                }
                if (call2) {
                    return call2;
                }
            }
            if (i2 != i1 + 2) {
                return 2;
            }
            if (i3 != i1 + 4) {
                return 3;
            }
            if (i4 != i1 + 6) {
                return 4;
            }
            if (i5 != i1 + 8) {
                return 5;
            }
            if (i6 != i1 + 10) {
                return 6;
            }
            if (i7 != i1 + 12) {
                return 7;
            }
            if (i8 != i1 + 14) {
                return 8;
            }
            if (i9 != i1 + 16) {
                return 9;
            }
            if (d1 != d9 - 16) {
                return 11;
            }
            if (d2 != d9 - 14) {
                return 12;
            }
            if (d3 != d9 - 12) {
                return 13;
            }
            if (d4 != d9 - 10) {
                return 14;
            }
            if (d5 != d9 - 8) {
                return 15;
            }
            if (d6 != d9 - 6) {
                return 16;
            }
            if (d7 != d9 - 4) {
                return 17;
            }
            if (d8 != d9 - 2) {
                return 18;
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
        Double,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Double,
        Identifier,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Identifier,
        OpenParen,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
        Comma,
        Identifier,
        Comma,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
        Comma,
        Identifier,
        Comma,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
        Comma,
        Identifier,
        Comma,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
        Comma,
        Identifier,
        Comma,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
        Comma,
        Identifier,
        Comma,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
        Comma,
        Identifier,
        Comma,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
        Comma,
        Identifier,
        Comma,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
        Comma,
        Identifier,
        Comma,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
        Comma,
        Identifier,
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        OpenParen,
        Identifier,
        Comma,
        Identifier,
        Minus,
        IntConstant(IntKind::Int),
        Comma,
        Identifier,
        Comma,
        Identifier,
        Minus,
        IntConstant(IntKind::Int),
        Comma,
        Identifier,
        Comma,
        Identifier,
        Minus,
        IntConstant(IntKind::Int),
        Comma,
        Identifier,
        Comma,
        Identifier,
        Minus,
        IntConstant(IntKind::Int),
        Comma,
        Identifier,
        Comma,
        Identifier,
        Minus,
        IntConstant(IntKind::Int),
        Comma,
        Identifier,
        Comma,
        Identifier,
        Minus,
        IntConstant(IntKind::Int),
        Comma,
        Identifier,
        Comma,
        Identifier,
        Minus,
        IntConstant(IntKind::Int),
        Comma,
        Identifier,
        Comma,
        Identifier,
        Minus,
        IntConstant(IntKind::Int),
        Comma,
        Identifier,
        Comma,
        Identifier,
        Minus,
        IntConstant(IntKind::Int),
        CloseParen,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Semicolon,
        CloseBrace,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        Minus,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        Minus,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        Minus,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        Minus,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        Minus,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        Minus,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        Minus,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        Minus,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_libraries_double_and_int_params_recursive_client() {
    let src = r#"
        int fun(int i1, double d1, int i2, double d2, int i3, double d3,
                int i4, double d4, int i5, double d5, int i6, double d6,
                int i7, double d7, int i8, double d8, int i9, double d9);
        int main(void) {
            double d = fun(1, 2.0, 3, 4.0, 5, 6.0, 7, 8.0, 9, 10.0, 11, 12.0, 13, 14.0, 15, 16.0, 17, 18.0);
            return (d == 78.00);
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Int,
        Identifier,
        Comma,
        Double,
        Identifier,
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Double,
        Identifier,
        Equal,
        Identifier,
        OpenParen,
        IntConstant(IntKind::Int),
        Comma,
        DoubleConstant,
        Comma,
        IntConstant(IntKind::Int),
        Comma,
        DoubleConstant,
        Comma,
        IntConstant(IntKind::Int),
        Comma,
        DoubleConstant,
        Comma,
        IntConstant(IntKind::Int),
        Comma,
        DoubleConstant,
        Comma,
        IntConstant(IntKind::Int),
        Comma,
        DoubleConstant,
        Comma,
        IntConstant(IntKind::Int),
        Comma,
        DoubleConstant,
        Comma,
        IntConstant(IntKind::Int),
        Comma,
        DoubleConstant,
        Comma,
        IntConstant(IntKind::Int),
        Comma,
        DoubleConstant,
        Comma,
        IntConstant(IntKind::Int),
        Comma,
        DoubleConstant,
        CloseParen,
        Semicolon,
        Return,
        OpenParen,
        Identifier,
        EqualEqual,
        DoubleConstant,
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_libraries_double_parameters() {
    let src = r#"
        int check_arguments(double a, double b, double c, double d, double e, double f, double g, double h) {
            if (a != 1.0) {
                return 1;
            }
            if (b != 2.0) {
                return 2;
            }
            if (c != 3.0) {
                return 3;
            }
            if (d != 4.0) {
                return 4;
            }
            if (e != -1.0) {
                return 5;
            }
            if (f != -2.0) {
                return 6;
            }
            if (g != -3.0) {
                return 7;
            }
            if (h != -4.0) {
                return 8;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_libraries_double_parameters_client() {
    let src = r#"
        int check_arguments(double a, double b, double c, double d, double e, double f, double g, double h);
        int main(void) {
            return check_arguments(1.0, 2.0, 3.0, 4.0, -1.0, -2.0, -3.0, -4.0);
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Double,
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
        DoubleConstant,
        Comma,
        DoubleConstant,
        Comma,
        DoubleConstant,
        Comma,
        DoubleConstant,
        Comma,
        Minus,
        DoubleConstant,
        Comma,
        Minus,
        DoubleConstant,
        Comma,
        Minus,
        DoubleConstant,
        Comma,
        Minus,
        DoubleConstant,
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_libraries_double_params_and_result() {
    let src = r#"
        double fmax(double x, double y);
        double get_max(double a, double b, double c, double d,
                       double e, double f, double g, double h,
                       double i, double j, double k)
        {
            double max = fmax(
                fmax(
                    fmax(
                        fmax(a, b),
                        fmax(c, d)),
                    fmax(
                        fmax(e, f),
                        fmax(g, h))),
                fmax(i, fmax(j, k)));
            return max;
        }
    "#;
    let expected = vec![
        Double, Identifier, OpenParen, Double, Identifier, Comma, Double, Identifier, CloseParen,
        Semicolon, Double, Identifier, OpenParen, Double, Identifier, Comma, Double, Identifier,
        Comma, Double, Identifier, Comma, Double, Identifier, Comma, Double, Identifier, Comma,
        Double, Identifier, Comma, Double, Identifier, Comma, Double, Identifier, Comma, Double,
        Identifier, Comma, Double, Identifier, Comma, Double, Identifier, CloseParen, OpenBrace,
        Double, Identifier, Equal, Identifier, OpenParen, Identifier, OpenParen, Identifier,
        OpenParen, Identifier, OpenParen, Identifier, Comma, Identifier, CloseParen, Comma,
        Identifier, OpenParen, Identifier, Comma, Identifier, CloseParen, CloseParen, Comma,
        Identifier, OpenParen, Identifier, OpenParen, Identifier, Comma, Identifier, CloseParen,
        Comma, Identifier, OpenParen, Identifier, Comma, Identifier, CloseParen, CloseParen,
        CloseParen, Comma, Identifier, OpenParen, Identifier, Comma, Identifier, OpenParen,
        Identifier, Comma, Identifier, CloseParen, CloseParen, CloseParen, Semicolon, Return,
        Identifier, Semicolon, CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_libraries_double_params_and_result_client() {
    let src = r#"
        double get_max(double a, double b, double c, double d,
                       double e, double f, double g, double h,
                       double i, double j, double k);
        int main(void)
        {
            double result = get_max(100.3, 200.1, 0.01, 1.00004e5, 55.555, -4., 6543.2,
                                    9e9, 8e8, 7.6, 10e3 * 11e5);
            return result == 10e3 * 11e5;
        }
    "#;
    let expected = vec![
        Double,
        Identifier,
        OpenParen,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Double,
        Identifier,
        Equal,
        Identifier,
        OpenParen,
        DoubleConstant,
        Comma,
        DoubleConstant,
        Comma,
        DoubleConstant,
        Comma,
        DoubleConstant,
        Comma,
        DoubleConstant,
        Comma,
        Minus,
        DoubleConstant,
        Comma,
        DoubleConstant,
        Comma,
        DoubleConstant,
        Comma,
        DoubleConstant,
        Comma,
        DoubleConstant,
        Comma,
        DoubleConstant,
        Star,
        DoubleConstant,
        CloseParen,
        Semicolon,
        Return,
        Identifier,
        EqualEqual,
        DoubleConstant,
        Star,
        DoubleConstant,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_libraries_extern_double() {
    let src = r#"
        double d = 1e20;
    "#;
    let expected = vec![Double, Identifier, Equal, DoubleConstant, Semicolon];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_libraries_extern_double_client() {
    let src = r#"
        
        extern double d;
        int main(void) {
            return d == 1e20;
        }
    "#;
    let expected = vec![
        Extern,
        Double,
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
        EqualEqual,
        DoubleConstant,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_libraries_use_arg_after_fun_call() {
    let src = r#"
        double fun(double x) {
            if (x > 2)
                return x;
            else {
                double ret = fun(x + 2);
                return ret + x;
            }
        }
    "#;
    let expected = vec![
        Double,
        Identifier,
        OpenParen,
        Double,
        Identifier,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Identifier,
        Greater,
        IntConstant(IntKind::Int),
        CloseParen,
        Return,
        Identifier,
        Semicolon,
        Else,
        OpenBrace,
        Double,
        Identifier,
        Equal,
        Identifier,
        OpenParen,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
        CloseParen,
        Semicolon,
        Return,
        Identifier,
        Plus,
        Identifier,
        Semicolon,
        CloseBrace,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_libraries_use_arg_after_fun_call_client() {
    let src = r#"
        double fun(double x);
        int main(void) {
            return fun(1.0);
        }
    "#;
    let expected = vec![
        Double,
        Identifier,
        OpenParen,
        Double,
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
        DoubleConstant,
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_special_values_infinity() {
    let src = r#"
        double inf = 2e308;
        double very_large = 1.79E308;
        double zero = 0.0;
        int main(void) {
            if (inf != 11e330) {
                return 1;
            }
            if (inf <= very_large) {
                return 2;
            }
            if(very_large * 10.0 != inf) {
                return 3;
            }
            if (1.0 / zero != inf) {
                return 4;
            }
            double negated_inf = -inf;
            double negated_inf2 = -1.0 / zero;
            if (negated_inf >= -very_large) {
                return 5;
            }
            if (negated_inf != negated_inf2) {
                return 6;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Double,
        Identifier,
        Equal,
        DoubleConstant,
        Semicolon,
        Double,
        Identifier,
        Equal,
        DoubleConstant,
        Semicolon,
        Double,
        Identifier,
        Equal,
        DoubleConstant,
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
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        LessEqual,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        Star,
        DoubleConstant,
        BangEqual,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        DoubleConstant,
        Slash,
        Identifier,
        BangEqual,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Double,
        Identifier,
        Equal,
        Minus,
        Identifier,
        Semicolon,
        Double,
        Identifier,
        Equal,
        Minus,
        DoubleConstant,
        Slash,
        Identifier,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        GreaterEqual,
        Minus,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_special_values_negative_zero() {
    let src = r#"
        double copysign(double x, double y);
        double zero = 0.0;
        int main(void) {
            double negative_zero = -zero;
            if (negative_zero != 0)
                return 1;
            if ( 1/negative_zero != -10e308 )
                return 2;
            if ( (-10)/negative_zero != 10e308)
                return 3;
            int fail = 0;
            negative_zero && (fail = 1);
            if (fail)
                return 4;
            if (negative_zero) {
                return 5;
            }
            if (zero != -0.0) {
                return 6;
            }
            double negated = copysign(4.0, -0.0);
            double positive = copysign(-5.0, 0.0);
            if (negated != -4.0) {
                return 7;
            }
            if (positive != 5.0) {
                return 8;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Double,
        Identifier,
        OpenParen,
        Double,
        Identifier,
        Comma,
        Double,
        Identifier,
        CloseParen,
        Semicolon,
        Double,
        Identifier,
        Equal,
        DoubleConstant,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Double,
        Identifier,
        Equal,
        Minus,
        Identifier,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        IntConstant(IntKind::Int),
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        IntConstant(IntKind::Int),
        Slash,
        Identifier,
        BangEqual,
        Minus,
        DoubleConstant,
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        OpenParen,
        Minus,
        IntConstant(IntKind::Int),
        CloseParen,
        Slash,
        Identifier,
        BangEqual,
        DoubleConstant,
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        AmpersandAmpersand,
        OpenParen,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        CloseParen,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Double,
        Identifier,
        Equal,
        Identifier,
        OpenParen,
        DoubleConstant,
        Comma,
        Minus,
        DoubleConstant,
        CloseParen,
        Semicolon,
        Double,
        Identifier,
        Equal,
        Identifier,
        OpenParen,
        Minus,
        DoubleConstant,
        Comma,
        DoubleConstant,
        CloseParen,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_special_values_subnormal_not_zero() {
    let src = r#"
        int non_zero(double d) {
            return !d;
        }
        double multiply_by_large_num(double d) {
            return d * 2e20;
        }
        int main(void) {
            double subnormal = 2.5e-320;
            if (multiply_by_large_num(subnormal) != 4.99994433591341498562e-300) {
                return 2;
            }
            return non_zero(subnormal);
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Double,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Bang,
        Identifier,
        Semicolon,
        CloseBrace,
        Double,
        Identifier,
        OpenParen,
        Double,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        Identifier,
        Star,
        DoubleConstant,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Double,
        Identifier,
        Equal,
        DoubleConstant,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        Identifier,
        CloseParen,
        BangEqual,
        DoubleConstant,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
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
