use crate::lexer::TokenKind::*;
use crate::lexer::{IntKind, tokenize};

#[test]
#[should_panic]
fn test_invalid_lex_invalid_suffix() {
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
fn test_invalid_lex_invalid_suffix_2() {
    tokenize(
        r#"
        int main(void) {
            return 0lul;
        }
    "#,
    );
}

#[test]
fn test_valid_explicit_casts_chained_casts() {
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
        IntConstant(IntKind::Uint),
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
        IntConstant(IntKind::Long),
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::ULong),
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
fn test_valid_explicit_casts_extension() {
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
        IntConstant(IntKind::Int),
        Comma,
        IntConstant(IntKind::ULong),
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
        IntConstant(IntKind::Int),
        Comma,
        IntConstant(IntKind::ULong),
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
        IntConstant(IntKind::Uint),
        Comma,
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
        Identifier,
        OpenParen,
        IntConstant(IntKind::Uint),
        Comma,
        IntConstant(IntKind::ULong),
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        OpenParen,
        Unsigned,
        Long,
        CloseParen,
        IntConstant(IntKind::Uint),
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
fn test_valid_explicit_casts_rewrite_movz_regression() {
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
        IntConstant(IntKind::Uint),
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
        IntConstant(IntKind::Uint),
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
        IntConstant(IntKind::Long),
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
fn test_valid_explicit_casts_round_trip_casts() {
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
        IntConstant(IntKind::ULong),
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
        IntConstant(IntKind::ULong),
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::ULong),
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
fn test_valid_explicit_casts_same_size_conversion() {
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
        IntConstant(IntKind::Int),
        Comma,
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
        Identifier,
        OpenParen,
        IntConstant(IntKind::Uint),
        Comma,
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
        Bang,
        Identifier,
        OpenParen,
        Minus,
        IntConstant(IntKind::Long),
        Comma,
        IntConstant(IntKind::ULong),
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
        IntConstant(IntKind::ULong),
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
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_explicit_casts_truncate() {
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
        IntConstant(IntKind::Long),
        Comma,
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
        Identifier,
        OpenParen,
        Minus,
        IntConstant(IntKind::Long),
        Comma,
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
        Identifier,
        OpenParen,
        IntConstant(IntKind::ULong),
        Comma,
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
        Bang,
        Identifier,
        OpenParen,
        IntConstant(IntKind::ULong),
        Comma,
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
        Identifier,
        OpenParen,
        IntConstant(IntKind::ULong),
        Comma,
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
        Identifier,
        OpenParen,
        IntConstant(IntKind::ULong),
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
        Bang,
        Identifier,
        OpenParen,
        IntConstant(IntKind::ULong),
        Comma,
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
        Identifier,
        OpenParen,
        IntConstant(IntKind::ULong),
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
        Unsigned,
        Int,
        Identifier,
        Equal,
        OpenParen,
        Unsigned,
        Int,
        CloseParen,
        IntConstant(IntKind::ULong),
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
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_bitwise_unsigned_ops() {
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
        IntConstant(IntKind::Uint),
        Semicolon,
        Unsigned,
        Long,
        Identifier,
        Equal,
        IntConstant(IntKind::ULong),
        Semicolon,
        If,
        OpenParen,
        OpenParen,
        Identifier,
        Ampersand,
        Identifier,
        CloseParen,
        BangEqual,
        IntConstant(IntKind::Int),
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        OpenParen,
        Identifier,
        Pipe,
        Identifier,
        CloseParen,
        BangEqual,
        IntConstant(IntKind::ULong),
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        Signed,
        Int,
        Identifier,
        Equal,
        Minus,
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
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
fn test_valid_extra_credit_bitwise_unsigned_shift() {
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
        IntConstant(IntKind::Uint),
        Semicolon,
        If,
        OpenParen,
        OpenParen,
        Identifier,
        LessLess,
        IntConstant(IntKind::Long),
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
        OpenParen,
        Identifier,
        GreaterGreater,
        IntConstant(IntKind::Int),
        CloseParen,
        BangEqual,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Static,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        OpenParen,
        IntConstant(IntKind::Uint),
        GreaterGreater,
        Identifier,
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
        OpenParen,
        IntConstant(IntKind::Uint),
        LessLess,
        Identifier,
        CloseParen,
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
fn test_valid_extra_credit_compound_assign_uint() {
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
        IntConstant(IntKind::Uint),
        Semicolon,
        Identifier,
        SlashEqual,
        Minus,
        IntConstant(IntKind::Long),
        Semicolon,
        Return,
        OpenParen,
        Identifier,
        EqualEqual,
        IntConstant(IntKind::Uint),
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_compound_bitshift() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        GreaterGreaterEqual,
        IntConstant(IntKind::Uint),
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
        Unsigned,
        Long,
        Identifier,
        Equal,
        IntConstant(IntKind::ULong),
        Semicolon,
        Identifier,
        LessLessEqual,
        IntConstant(IntKind::Int),
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
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_compound_bitwise() {
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
        IntConstant(IntKind::ULong),
        Semicolon,
        Identifier,
        AmpersandEqual,
        Minus,
        IntConstant(IntKind::Int),
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
        Identifier,
        PipeEqual,
        IntConstant(IntKind::Uint),
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
        Unsigned,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Uint),
        Semicolon,
        Long,
        Identifier,
        Equal,
        Minus,
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        CircumflexEqual,
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
fn test_valid_extra_credit_postfix_precedence() {
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
        IntConstant(IntKind::Uint),
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
fn test_valid_extra_credit_switch_uint() {
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
        IntConstant(IntKind::Uint),
        Colon,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        Case,
        IntConstant(IntKind::Long),
        Colon,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        Case,
        IntConstant(IntKind::ULong),
        Colon,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        Default,
        Colon,
        Return,
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
        CloseParen,
        BangEqual,
        IntConstant(IntKind::Int),
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        IntConstant(IntKind::Int),
        CloseParen,
        BangEqual,
        IntConstant(IntKind::Int),
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        IntConstant(IntKind::Int),
        CloseParen,
        BangEqual,
        IntConstant(IntKind::Int),
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
fn test_valid_extra_credit_unsigned_incr_decr() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        MinusMinus,
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
        MinusMinus,
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
        IntConstant(IntKind::Uint),
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
        If,
        OpenParen,
        Identifier,
        PlusPlus,
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
        PlusPlus,
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
fn test_valid_implicit_casts_common_type() {
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
        IntConstant(IntKind::Long),
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
        IntConstant(IntKind::Int),
        Comma,
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
        Identifier,
        OpenParen,
        Minus,
        IntConstant(IntKind::Int),
        Comma,
        IntConstant(IntKind::ULong),
        CloseParen,
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
        IntConstant(IntKind::Uint),
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
        Bang,
        Identifier,
        OpenParen,
        IntConstant(IntKind::Uint),
        Comma,
        IntConstant(IntKind::ULong),
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
        IntConstant(IntKind::Long),
        Comma,
        IntConstant(IntKind::ULong),
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
        Comma,
        Minus,
        IntConstant(IntKind::Int),
        Comma,
        IntConstant(IntKind::Uint),
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
fn test_valid_implicit_casts_convert_by_assignment() {
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
        IntConstant(IntKind::ULong),
        Comma,
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
        Bang,
        Identifier,
        OpenParen,
        IntConstant(IntKind::Uint),
        Comma,
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
        Identifier,
        OpenParen,
        Minus,
        IntConstant(IntKind::Int),
        Comma,
        IntConstant(IntKind::ULong),
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
        IntConstant(IntKind::Uint),
        CloseParen,
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
        OpenParen,
        Minus,
        IntConstant(IntKind::Int),
        CloseParen,
        BangEqual,
        IntConstant(IntKind::ULong),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Long,
        Identifier,
        Equal,
        Identifier,
        OpenParen,
        IntConstant(IntKind::ULong),
        CloseParen,
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
        Bang,
        Identifier,
        OpenParen,
        IntConstant(IntKind::Uint),
        Comma,
        IntConstant(IntKind::Long),
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Uint),
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
fn test_valid_implicit_casts_promote_constants() {
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
        IntConstant(IntKind::Long),
        Semicolon,
        Long,
        Identifier,
        Equal,
        IntConstant(IntKind::Long),
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
        IntConstant(IntKind::Uint),
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
        Minus,
        IntConstant(IntKind::Int),
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
        Bang,
        OpenParen,
        IntConstant(IntKind::ULong),
        Plus,
        IntConstant(IntKind::ULong),
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
fn test_valid_implicit_casts_static_initializers() {
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
        IntConstant(IntKind::Long),
        Semicolon,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Uint),
        Semicolon,
        Long,
        Identifier,
        Equal,
        IntConstant(IntKind::Uint),
        Semicolon,
        Long,
        Identifier,
        Equal,
        IntConstant(IntKind::Uint),
        Semicolon,
        Unsigned,
        Long,
        Identifier,
        Equal,
        IntConstant(IntKind::Uint),
        Semicolon,
        Unsigned,
        Long,
        Identifier,
        Equal,
        IntConstant(IntKind::Long),
        Semicolon,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::ULong),
        Semicolon,
        Unsigned,
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
        If,
        OpenParen,
        Identifier,
        BangEqual,
        IntConstant(IntKind::Uint),
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        IntConstant(IntKind::Int),
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        IntConstant(IntKind::Long),
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        IntConstant(IntKind::Long),
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        IntConstant(IntKind::ULong),
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        IntConstant(IntKind::ULong),
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Minus,
        IntConstant(IntKind::Int),
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        IntConstant(IntKind::Uint),
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
fn test_valid_libraries_unsigned_args() {
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
fn test_valid_libraries_unsigned_args_client() {
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
        IntConstant(IntKind::Int),
        Comma,
        Minus,
        IntConstant(IntKind::Int),
        Comma,
        Minus,
        IntConstant(IntKind::Int),
        Comma,
        IntConstant(IntKind::ULong),
        Comma,
        IntConstant(IntKind::ULong),
        Comma,
        IntConstant(IntKind::Int),
        Comma,
        IntConstant(IntKind::Int),
        Comma,
        IntConstant(IntKind::Uint),
        Comma,
        IntConstant(IntKind::ULong),
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_libraries_unsigned_global_var() {
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
        IntConstant(IntKind::Uint),
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
fn test_valid_libraries_unsigned_global_var_client() {
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
        IntConstant(IntKind::Uint),
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Minus,
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Long),
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Long),
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Long),
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
fn test_valid_type_specifiers_signed_type_specifiers() {
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
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
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
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        For,
        OpenParen,
        Signed,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        Greater,
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Minus,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Identifier,
        Equal,
        Identifier,
        Plus,
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
fn test_valid_type_specifiers_unsigned_type_specifiers() {
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
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Uint),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
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
        For,
        OpenParen,
        Unsigned,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        Less,
        IntConstant(IntKind::Uint),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Minus,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Identifier,
        Equal,
        Identifier,
        Plus,
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
fn test_valid_unsigned_expressions_arithmetic_ops() {
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
        IntConstant(IntKind::Uint),
        EqualEqual,
        IntConstant(IntKind::Uint),
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
        IntConstant(IntKind::ULong),
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
        IntConstant(IntKind::Uint),
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
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::ULong),
        EqualEqual,
        IntConstant(IntKind::ULong),
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
        IntConstant(IntKind::ULong),
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
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Uint),
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
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        IntConstant(IntKind::ULong),
        Semicolon,
        Identifier,
        Equal,
        IntConstant(IntKind::ULong),
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
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        IntConstant(IntKind::Uint),
        Semicolon,
        Identifier,
        Equal,
        IntConstant(IntKind::Uint),
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
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        IntConstant(IntKind::Uint),
        Semicolon,
        Identifier,
        Equal,
        IntConstant(IntKind::Uint),
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
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        IntConstant(IntKind::Uint),
        Semicolon,
        Identifier,
        Equal,
        IntConstant(IntKind::Uint),
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
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        IntConstant(IntKind::ULong),
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
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        IntConstant(IntKind::ULong),
        Semicolon,
        Identifier,
        Equal,
        IntConstant(IntKind::ULong),
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
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        IntConstant(IntKind::Uint),
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
fn test_valid_unsigned_expressions_arithmetic_wraparound() {
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
            ui_b = 3u;
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
        IntConstant(IntKind::Uint),
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
        IntConstant(IntKind::ULong),
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
        IntConstant(IntKind::ULong),
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
        IntConstant(IntKind::Uint),
        Semicolon,
        Identifier,
        Equal,
        IntConstant(IntKind::Uint),
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
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        IntConstant(IntKind::ULong),
        Semicolon,
        Identifier,
        Equal,
        IntConstant(IntKind::ULong),
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
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        IntConstant(IntKind::ULong),
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
fn test_valid_unsigned_expressions_comparisons() {
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
        IntConstant(IntKind::Uint),
        Semicolon,
        Unsigned,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Uint),
        Semicolon,
        Unsigned,
        Long,
        Identifier,
        Equal,
        IntConstant(IntKind::ULong),
        Semicolon,
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
        If,
        OpenParen,
        Identifier,
        Less,
        Identifier,
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        LessEqual,
        Identifier,
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        GreaterEqual,
        Identifier,
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        Greater,
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
        LessEqual,
        Identifier,
        CloseParen,
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        Less,
        Identifier,
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        LessEqual,
        Identifier,
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        GreaterEqual,
        Identifier,
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        Greater,
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
        LessEqual,
        Identifier,
        CloseParen,
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
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
fn test_valid_unsigned_expressions_locals() {
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
        IntConstant(IntKind::ULong),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Minus,
        IntConstant(IntKind::Int),
        Semicolon,
        Long,
        Identifier,
        Equal,
        Minus,
        IntConstant(IntKind::Long),
        Semicolon,
        Unsigned,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Uint),
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
        IntConstant(IntKind::Uint),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        IntConstant(IntKind::Long),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Star,
        IntConstant(IntKind::Uint),
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
        IntConstant(IntKind::Uint),
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
fn test_valid_unsigned_expressions_logical() {
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
        IntConstant(IntKind::ULong),
        Semicolon,
        Unsigned,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Uint),
        Semicolon,
        Unsigned,
        Long,
        Identifier,
        Equal,
        IntConstant(IntKind::Long),
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
        Identifier,
        OpenParen,
        Identifier,
        Comma,
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
        Bang,
        Identifier,
        OpenParen,
        IntConstant(IntKind::Int),
        Comma,
        Identifier,
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
fn test_valid_unsigned_expressions_simple() {
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
        IntConstant(IntKind::Uint),
        Semicolon,
        Return,
        OpenParen,
        Identifier,
        Plus,
        IntConstant(IntKind::Uint),
        EqualEqual,
        IntConstant(IntKind::Uint),
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_unsigned_expressions_static_variables() {
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
        IntConstant(IntKind::ULong),
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
        IntConstant(IntKind::ULong),
        CloseParen,
        Return,
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
        IntConstant(IntKind::ULong),
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        PipePipe,
        Identifier,
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
