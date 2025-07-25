use crate::lexer::TokenKind::*;
use crate::lexer::{tokenize, IntKind};

#[test]
#[should_panic]
fn test_invalid_lex_invalid_suffix() {
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
fn test_invalid_lex_invalid_suffix2() {
    tokenize(
        r#"
        int main(void) {
            return 0LLL;
        }
    "#,
    );
}

#[test]
fn test_valid_explicit_casts_sign_extend() {
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
        IntConstant(IntKind::Int),
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
        Minus,
        IntConstant(IntKind::Long),
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Long,
        Identifier,
        Equal,
        OpenParen,
        Long,
        CloseParen,
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Long),
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
        IntConstant(IntKind::Long),
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
        OpenParen,
        Int,
        CloseParen,
        IntConstant(IntKind::Long),
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
fn test_valid_extra_credit_bitshift() {
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
        IntConstant(IntKind::Long),
        Semicolon,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        GreaterGreater,
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
        LessLess,
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
        LessLess,
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Long),
        LessLess,
        IntConstant(IntKind::Int),
        CloseParen,
        BangEqual,
        IntConstant(IntKind::Long),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Long,
        Identifier,
        Equal,
        IntConstant(IntKind::Long),
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
        If,
        OpenParen,
        Identifier,
        GreaterGreater,
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
        Identifier,
        Equal,
        Minus,
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        GreaterGreater,
        IntConstant(IntKind::Long),
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
fn test_valid_extra_credit_bitwise_long_op() {
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
        IntConstant(IntKind::Long),
        Semicolon,
        Long,
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
        IntConstant(IntKind::Long),
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
        Pipe,
        Identifier,
        CloseParen,
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
        OpenParen,
        Identifier,
        Circumflex,
        Identifier,
        CloseParen,
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
        OpenParen,
        Minus,
        IntConstant(IntKind::Long),
        Ampersand,
        IntConstant(IntKind::Long),
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
        OpenParen,
        IntConstant(IntKind::Long),
        Pipe,
        IntConstant(IntKind::Long),
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
        OpenParen,
        IntConstant(IntKind::Long),
        Circumflex,
        IntConstant(IntKind::Long),
        CloseParen,
        BangEqual,
        IntConstant(IntKind::Long),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Long,
        Identifier,
        Equal,
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
        IntConstant(IntKind::Long),
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
        Pipe,
        Identifier,
        CloseParen,
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
        OpenParen,
        Identifier,
        Circumflex,
        Identifier,
        CloseParen,
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
        OpenParen,
        Identifier,
        Circumflex,
        IntConstant(IntKind::Long),
        CloseParen,
        BangEqual,
        Tilde,
        IntConstant(IntKind::Long),
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
fn test_valid_extra_credit_compound_assign_to_int() {
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
        Identifier,
        PlusEqual,
        IntConstant(IntKind::Long),
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
        Identifier,
        SlashEqual,
        Minus,
        IntConstant(IntKind::Long),
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
        Identifier,
        StarEqual,
        IntConstant(IntKind::Long),
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
fn test_valid_extra_credit_compound_assign_to_long() {
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
        IntConstant(IntKind::Long),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Minus,
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Long),
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
fn test_valid_extra_credit_compound_bitshift() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        LessLessEqual,
        IntConstant(IntKind::Long),
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
        OpenParen,
        Identifier,
        GreaterGreaterEqual,
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
        Identifier,
        BangEqual,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Long,
        Identifier,
        Equal,
        IntConstant(IntKind::Long),
        Semicolon,
        If,
        OpenParen,
        OpenParen,
        Identifier,
        LessLessEqual,
        IntConstant(IntKind::Int),
        CloseParen,
        BangEqual,
        IntConstant(IntKind::Long),
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
        If,
        OpenParen,
        OpenParen,
        Identifier,
        GreaterGreaterEqual,
        IntConstant(IntKind::Int),
        CloseParen,
        BangEqual,
        Minus,
        IntConstant(IntKind::Long),
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
        IntConstant(IntKind::Long),
        Semicolon,
        Long,
        Identifier,
        Equal,
        Minus,
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        AmpersandEqual,
        Identifier,
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
        Identifier,
        PipeEqual,
        IntConstant(IntKind::Long),
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
        Identifier,
        CircumflexEqual,
        Minus,
        IntConstant(IntKind::Long),
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
        Identifier,
        Equal,
        IntConstant(IntKind::Long),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Minus,
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        AmpersandEqual,
        Identifier,
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
        Identifier,
        Equal,
        Minus,
        IntConstant(IntKind::Long),
        Semicolon,
        If,
        OpenParen,
        OpenParen,
        Identifier,
        PipeEqual,
        IntConstant(IntKind::Long),
        CloseParen,
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
fn test_valid_extra_credit_increment_long() {
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
        IntConstant(IntKind::Long),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        PlusPlus,
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
        IntConstant(IntKind::Long),
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
        IntConstant(IntKind::Long),
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
fn test_valid_extra_credit_switch_int() {
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
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
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
        Minus,
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
fn test_valid_extra_credit_switch_long() {
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
        IntConstant(IntKind::Int),
        Colon,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        Case,
        IntConstant(IntKind::Int),
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
        Default,
        Colon,
        Return,
        Minus,
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
            l = 2147483648;
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
        IntConstant(IntKind::Int),
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
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Long),
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
        Minus,
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Long),
        Semicolon,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
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
fn test_valid_implicit_casts_convert_by_assignment() {
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
        IntConstant(IntKind::Long),
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
        Identifier,
        Equal,
        Identifier,
        OpenParen,
        Minus,
        IntConstant(IntKind::Int),
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
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Long),
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
        Bang,
        Identifier,
        OpenParen,
        IntConstant(IntKind::Long),
        Comma,
        IntConstant(IntKind::Int),
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
fn test_valid_implicit_casts_convert_function_arguments() {
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
        IntConstant(IntKind::Long),
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
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
        Identifier,
        BangEqual,
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
        IntConstant(IntKind::Int),
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
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
        Int,
        Identifier,
        Equal,
        Minus,
        IntConstant(IntKind::Int),
        Semicolon,
        Long,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Long,
        Identifier,
        Equal,
        Minus,
        IntConstant(IntKind::Int),
        Semicolon,
        Long,
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
        Long,
        Identifier,
        Equal,
        Minus,
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
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
fn test_valid_implicit_casts_convert_static_initializer() {
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
        IntConstant(IntKind::Long),
        Semicolon,
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
        IntConstant(IntKind::Long),
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
fn test_valid_implicit_casts_long_constants() {
    let src = r#"
        int main(void) {
            if (2147483647l + 2147483647l < 0l) {
                return 1;
            }
            if (19327352832 < 100l) {
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
        IntConstant(IntKind::Long),
        Plus,
        IntConstant(IntKind::Long),
        Less,
        IntConstant(IntKind::Long),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        IntConstant(IntKind::Int),
        Less,
        IntConstant(IntKind::Long),
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
fn test_valid_libraries_long_args() {
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
        Less,
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
fn test_valid_libraries_long_args_client() {
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
        IntConstant(IntKind::Int),
        Comma,
        IntConstant(IntKind::Int),
        Comma,
        IntConstant(IntKind::Int),
        Comma,
        IntConstant(IntKind::Long),
        Comma,
        IntConstant(IntKind::Int),
        Comma,
        IntConstant(IntKind::Long),
        Comma,
        IntConstant(IntKind::Int),
        Comma,
        IntConstant(IntKind::Int),
        Comma,
        IntConstant(IntKind::Long),
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_libraries_long_global_var() {
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
        IntConstant(IntKind::Long),
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
fn test_valid_libraries_long_global_var_client() {
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
        IntConstant(IntKind::Long),
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        CloseParen,
        BangEqual,
        IntConstant(IntKind::Int),
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Minus,
        IntConstant(IntKind::Long),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        CloseParen,
        BangEqual,
        IntConstant(IntKind::Long),
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        CloseParen,
        BangEqual,
        Minus,
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
fn test_valid_libraries_maintain_stack_alignment() {
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
fn test_valid_libraries_maintain_stack_alignment_client() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Long,
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
fn test_valid_libraries_return_long() {
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
fn test_valid_libraries_return_long_client() {
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
        IntConstant(IntKind::Int),
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
fn test_valid_long_expressions_arithmetic_ops() {
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
        Return,
        OpenParen,
        Identifier,
        Minus,
        Identifier,
        EqualEqual,
        Minus,
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
        Return,
        OpenParen,
        Identifier,
        Star,
        IntConstant(IntKind::Long),
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
        Identifier,
        Equal,
        Identifier,
        Slash,
        IntConstant(IntKind::Long),
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
        Identifier,
        Equal,
        Minus,
        Identifier,
        Percent,
        IntConstant(IntKind::Long),
        Semicolon,
        Return,
        OpenParen,
        Identifier,
        EqualEqual,
        Minus,
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
        Return,
        OpenParen,
        Tilde,
        Identifier,
        EqualEqual,
        Minus,
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
        Identifier,
        Equal,
        IntConstant(IntKind::Long),
        Semicolon,
        Identifier,
        Equal,
        IntConstant(IntKind::Long),
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
        Minus,
        IntConstant(IntKind::Long),
        Semicolon,
        Identifier,
        Equal,
        IntConstant(IntKind::Long),
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
        IntConstant(IntKind::Long),
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
        IntConstant(IntKind::Long),
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
        IntConstant(IntKind::Long),
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
        IntConstant(IntKind::Long),
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
fn test_valid_long_expressions_assign() {
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
        IntConstant(IntKind::Long),
        Semicolon,
        Long,
        Identifier,
        Equal,
        IntConstant(IntKind::Long),
        Semicolon,
        Identifier,
        Equal,
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
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_long_expressions_comparisons() {
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
        IntConstant(IntKind::Long),
        Greater,
        IntConstant(IntKind::Long),
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Long),
        Less,
        IntConstant(IntKind::Long),
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
        Return,
        OpenParen,
        IntConstant(IntKind::Long),
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
        Identifier,
        Equal,
        Minus,
        IntConstant(IntKind::Long),
        Semicolon,
        If,
        OpenParen,
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
        IntConstant(IntKind::Long),
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
fn test_valid_long_expressions_large_constants() {
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
        Identifier,
        Plus,
        IntConstant(IntKind::Long),
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
        Identifier,
        Equal,
        Identifier,
        Minus,
        IntConstant(IntKind::Long),
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
        Identifier,
        Equal,
        Identifier,
        Star,
        IntConstant(IntKind::Long),
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
fn test_valid_long_expressions_logical() {
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
        IntConstant(IntKind::Long),
        Semicolon,
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
fn test_valid_long_expressions_long_and_int_locals() {
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
        IntConstant(IntKind::Long),
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
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
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
        Plus,
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
fn test_valid_long_expressions_long_args() {
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
        Less,
        IntConstant(IntKind::Long),
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
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
        IntConstant(IntKind::Long),
        Comma,
        IntConstant(IntKind::Long),
        Comma,
        IntConstant(IntKind::Int),
        Comma,
        IntConstant(IntKind::Int),
        Comma,
        IntConstant(IntKind::Int),
        Comma,
        IntConstant(IntKind::Int),
        Comma,
        IntConstant(IntKind::Int),
        Comma,
        IntConstant(IntKind::Int),
        Comma,
        IntConstant(IntKind::Long),
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_long_expressions_multi_op() {
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
        IntConstant(IntKind::Long),
        Minus,
        IntConstant(IntKind::Long),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        EqualEqual,
        IntConstant(IntKind::Long),
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
        IntConstant(IntKind::Long),
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_long_expressions_return_long() {
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
        IntConstant(IntKind::Int),
        Comma,
        IntConstant(IntKind::Int),
        CloseParen,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        EqualEqual,
        IntConstant(IntKind::Long),
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
fn test_valid_long_expressions_rewrite_large_multiply_regression() {
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
        IntConstant(IntKind::Long),
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
        IntConstant(IntKind::Long),
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
        Plus,
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
fn test_valid_long_expressions_simple() {
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
        IntConstant(IntKind::Long),
        Semicolon,
        Return,
        OpenParen,
        Identifier,
        Minus,
        IntConstant(IntKind::Long),
        EqualEqual,
        IntConstant(IntKind::Long),
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_long_expressions_static_long() {
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
        IntConstant(IntKind::Long),
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
        IntConstant(IntKind::Long),
        EqualEqual,
        IntConstant(IntKind::Long),
        CloseParen,
        OpenBrace,
        Identifier,
        Equal,
        IntConstant(IntKind::Long),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        EqualEqual,
        IntConstant(IntKind::Long),
        CloseParen,
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
fn test_valid_long_expressions_type_specifiers() {
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
        IntConstant(IntKind::Long),
        Semicolon,
        Long,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Long),
        Semicolon,
        Int,
        Long,
        Identifier,
        Equal,
        IntConstant(IntKind::Long),
        Semicolon,
        Extern,
        Long,
        Identifier,
        Semicolon,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        For,
        OpenParen,
        Long,
        Identifier,
        Equal,
        IntConstant(IntKind::Long),
        Semicolon,
        Identifier,
        Greater,
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Slash,
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
        OpenParen,
        Identifier,
        Comma,
        Identifier,
        Comma,
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
