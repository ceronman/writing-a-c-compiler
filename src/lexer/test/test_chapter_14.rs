use crate::lexer::TokenKind::*;
use crate::lexer::{IntKind, tokenize};

#[test]
fn test_valid_casts_cast_between_pointer_types() {
    let src = r#"
        int check_null_ptr_cast(void) {
            static long *long_ptr = 0;
            double *dbl_ptr = (double *)long_ptr;
            unsigned int *int_ptr = (unsigned int *)long_ptr;
            int **ptr_ptr = (int **)long_ptr;
            if (long_ptr) {
                return 1;
            }
            if (dbl_ptr) {
                return 2;
            }
            if (int_ptr) {
                return 3;
            }
            if (ptr_ptr) {
                return 4;
            }
            return 0;
        }
        int check_round_trip(void) {
            long l = -1;
            long *long_ptr = &l;
            double *dbl_ptr = (double *)long_ptr;
            long *other_long_ptr = (long *)dbl_ptr;
            if (*other_long_ptr != -1) {
                return 5;
            }
            return 0;
        }
        int main(void)
        {
            int result = check_null_ptr_cast();
            if (result) {
                return result;
            }
            result = check_round_trip();
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
        Static,
        Long,
        Star,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Double,
        Star,
        Identifier,
        Equal,
        OpenParen,
        Double,
        Star,
        CloseParen,
        Identifier,
        Semicolon,
        Unsigned,
        Int,
        Star,
        Identifier,
        Equal,
        OpenParen,
        Unsigned,
        Int,
        Star,
        CloseParen,
        Identifier,
        Semicolon,
        Int,
        Star,
        Star,
        Identifier,
        Equal,
        OpenParen,
        Int,
        Star,
        Star,
        CloseParen,
        Identifier,
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
        IntConstant(IntKind::Int),
        Semicolon,
        Long,
        Star,
        Identifier,
        Equal,
        Ampersand,
        Identifier,
        Semicolon,
        Double,
        Star,
        Identifier,
        Equal,
        OpenParen,
        Double,
        Star,
        CloseParen,
        Identifier,
        Semicolon,
        Long,
        Star,
        Identifier,
        Equal,
        OpenParen,
        Long,
        Star,
        CloseParen,
        Identifier,
        Semicolon,
        If,
        OpenParen,
        Star,
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
fn test_valid_casts_null_pointer_conversion() {
    let src = r#"
        double *d = 0l;
        int *i = 0ul;
        int *i2 = 0u;
        int expect_null_param(int *val)
        {
            return (val == 0ul);
        }
        long *return_null_ptr(void)
        {
            return 0;
        }
        int main(void)
        {
            int x = 10;
            int *ptr = &x;
            if (d) {
                return 1;
            }
            if (i) {
                return 2;
            }
            if (i2) {
                return 3;
            }
            ptr = 0ul;
            if (ptr) {
                return 4;
            }
            int *y = 0;
            if (y != 0)
                return 5;
            if (!expect_null_param(0)) {
                return 6;
            }
            long *null_ptr = return_null_ptr();
            if (null_ptr != 0) {
                return 7;
            }
            ptr = &x;
            int *ternary_result = 10 ? 0 : ptr;
            if (ternary_result) {
                return 8;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Double,
        Star,
        Identifier,
        Equal,
        IntConstant(IntKind::Long),
        Semicolon,
        Int,
        Star,
        Identifier,
        Equal,
        IntConstant(IntKind::ULong),
        Semicolon,
        Int,
        Star,
        Identifier,
        Equal,
        IntConstant(IntKind::Uint),
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Int,
        Star,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        OpenParen,
        Identifier,
        EqualEqual,
        IntConstant(IntKind::ULong),
        CloseParen,
        Semicolon,
        CloseBrace,
        Long,
        Star,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
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
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Star,
        Identifier,
        Equal,
        Ampersand,
        Identifier,
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
        Identifier,
        Equal,
        IntConstant(IntKind::ULong),
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
        Int,
        Star,
        Identifier,
        Equal,
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
        Bang,
        Identifier,
        OpenParen,
        IntConstant(IntKind::Int),
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Long,
        Star,
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
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        Equal,
        Ampersand,
        Identifier,
        Semicolon,
        Int,
        Star,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Question,
        IntConstant(IntKind::Int),
        Colon,
        Identifier,
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
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_casts_pointer_int_casts() {
    let src = r#"
        int i = 128;
        long l = 128l;
        int int_to_pointer(void) {
            int *a = (int *) i;
            int *b = (int *) l;
            return a == b;
        }
        int pointer_to_int(void) {
            static long l;
            long *ptr = &l;
            unsigned long ptr_as_long = (unsigned long) ptr;
            return (ptr_as_long % 8 == 0);
        }
        int cast_long_round_trip(void) {
            int *ptr = (int *) l;
            long l2 = (long) ptr;
            return (l == l2);
        }
        int cast_ulong_round_trip(void) {
            long *ptr = &l;
            unsigned long ptr_as_ulong = (unsigned long) ptr;
            long *ptr2 = (long *) ptr_as_ulong;
            return (ptr == ptr2);
        }
        int cast_int_round_trip(void) {
            double *a = (double *)i;
            int i2 = (int) a;
            return (i2 == 128);
        }
        int main(void) {
            if (!int_to_pointer()) {
                return 1;
            }
            if (!pointer_to_int()) {
                return 2;
            }
            if (!cast_long_round_trip()) {
                return 3;
            }
            if (!cast_ulong_round_trip()) {
                return 4;
            }
            if (!cast_int_round_trip()) {
                return 5;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
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
        Int,
        Star,
        Identifier,
        Equal,
        OpenParen,
        Int,
        Star,
        CloseParen,
        Identifier,
        Semicolon,
        Int,
        Star,
        Identifier,
        Equal,
        OpenParen,
        Int,
        Star,
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
        Void,
        CloseParen,
        OpenBrace,
        Static,
        Long,
        Identifier,
        Semicolon,
        Long,
        Star,
        Identifier,
        Equal,
        Ampersand,
        Identifier,
        Semicolon,
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
        OpenParen,
        Identifier,
        Percent,
        IntConstant(IntKind::Int),
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
        Int,
        Star,
        Identifier,
        Equal,
        OpenParen,
        Int,
        Star,
        CloseParen,
        Identifier,
        Semicolon,
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
        Long,
        Star,
        Identifier,
        Equal,
        Ampersand,
        Identifier,
        Semicolon,
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
        Long,
        Star,
        Identifier,
        Equal,
        OpenParen,
        Long,
        Star,
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
        Double,
        Star,
        Identifier,
        Equal,
        OpenParen,
        Double,
        Star,
        CloseParen,
        Identifier,
        Semicolon,
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
fn test_valid_comparisons_compare_pointers() {
    let src = r#"
        int main(void) {
            int a = 0;
            int b;
            int *a_ptr = &a;
            int *a_ptr2 = &a;
            int *b_ptr = &b;
            if (a_ptr == b_ptr) {
                return 1;
            }
            if (a_ptr != a_ptr2) {
                return 2;
            }
            if (!(a_ptr == a_ptr2)) {
                return 3;
            }
            if (!(a_ptr != b_ptr)) {
                return 4;
            }
            *b_ptr = *a_ptr;
            if (a_ptr == b_ptr) {
                return 5;
            }
            b_ptr = a_ptr;
            if (b_ptr != a_ptr) {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Semicolon,
        Int,
        Star,
        Identifier,
        Equal,
        Ampersand,
        Identifier,
        Semicolon,
        Int,
        Star,
        Identifier,
        Equal,
        Ampersand,
        Identifier,
        Semicolon,
        Int,
        Star,
        Identifier,
        Equal,
        Ampersand,
        Identifier,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        EqualEqual,
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
        Bang,
        OpenParen,
        Identifier,
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
        Identifier,
        CloseParen,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Star,
        Identifier,
        Equal,
        Star,
        Identifier,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        EqualEqual,
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
        Semicolon,
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
fn test_valid_comparisons_compare_to_null() {
    let src = r#"
        double *get_null_pointer(void) {
            return 0;
        }
        int main(void)
        {
            double x;
            double *null = get_null_pointer();
            double *non_null = &x;
            if (non_null == 0) {
                return 1;
            }
            if (!(null == 0l)) {
                return 2;
            }
            if (!(non_null != 0u)) {
                return 3;
            }
            if (null != 0ul) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Double,
        Star,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
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
        Double,
        Identifier,
        Semicolon,
        Double,
        Star,
        Identifier,
        Equal,
        Identifier,
        OpenParen,
        CloseParen,
        Semicolon,
        Double,
        Star,
        Identifier,
        Equal,
        Ampersand,
        Identifier,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        EqualEqual,
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
        OpenParen,
        Identifier,
        EqualEqual,
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
        Identifier,
        BangEqual,
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
fn test_valid_comparisons_pointers_as_conditions() {
    let src = r#"
        long *get_null_pointer(void) {
            return 0;
        }
        int main(void)
        {
            long x;
            long *ptr = &x;
            long *null_ptr = get_null_pointer();
            if (5.0 && null_ptr) {
                return 1;
            }
            int a = 0;
            if (!(ptr || (a = 10))) {
                return 2;
            }
            if (a != 0) {
                return 3;
            }
            if (!ptr) {
                return 4;
            }
            int j = ptr ? 1 : 2;
            int k = null_ptr ? 3 : 4;
            if (j != 1) {
                return 5;
            }
            if (k != 4) {
                return 6;
            }
            int i = 0;
            while (ptr)
            {
                if (i >= 10) {
                    ptr = 0;
                    continue;
                }
                i = i + 1;
            }
            if (i != 10) {
                return 7;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Long,
        Star,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
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
        Long,
        Identifier,
        Semicolon,
        Long,
        Star,
        Identifier,
        Equal,
        Ampersand,
        Identifier,
        Semicolon,
        Long,
        Star,
        Identifier,
        Equal,
        Identifier,
        OpenParen,
        CloseParen,
        Semicolon,
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
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Bang,
        OpenParen,
        Identifier,
        PipePipe,
        OpenParen,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
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
        Question,
        IntConstant(IntKind::Int),
        Colon,
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
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
        While,
        OpenParen,
        Identifier,
        CloseParen,
        OpenBrace,
        If,
        OpenParen,
        Identifier,
        GreaterEqual,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Continue,
        Semicolon,
        CloseBrace,
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
fn test_valid_declarators_abstract_declarators() {
    let src = r#"
        
        int main(void) {
            long int unsigned *x = 0;
            if (x != (unsigned long (*)) 0)
                return 1;
            if (x != (long unsigned int ((((*))))) 0)
                return 2;
            double ***y = 0;
            if (y != (double *(**)) 0)
                return 3;
            if (y != (double (***)) 0)
                return 4;
            if ((double (*(*(*)))) 0 != y)
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
        Long,
        Int,
        Unsigned,
        Star,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        OpenParen,
        Unsigned,
        Long,
        OpenParen,
        Star,
        CloseParen,
        CloseParen,
        IntConstant(IntKind::Int),
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        OpenParen,
        Long,
        Unsigned,
        Int,
        OpenParen,
        OpenParen,
        OpenParen,
        OpenParen,
        Star,
        CloseParen,
        CloseParen,
        CloseParen,
        CloseParen,
        CloseParen,
        IntConstant(IntKind::Int),
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        Double,
        Star,
        Star,
        Star,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        OpenParen,
        Double,
        Star,
        OpenParen,
        Star,
        Star,
        CloseParen,
        CloseParen,
        IntConstant(IntKind::Int),
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        OpenParen,
        Double,
        OpenParen,
        Star,
        Star,
        Star,
        CloseParen,
        CloseParen,
        IntConstant(IntKind::Int),
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        OpenParen,
        Double,
        OpenParen,
        Star,
        OpenParen,
        Star,
        OpenParen,
        Star,
        CloseParen,
        CloseParen,
        CloseParen,
        CloseParen,
        IntConstant(IntKind::Int),
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
fn test_valid_declarators_declarators() {
    let src = r#"
        int return_3(void);
        int(return_3(void));
        int(return_3)(void);
        int((return_3))(void)
        {
            return 3;
        }
        long l = 100;
        long *two_pointers(double val, double *ptr)
        {
            *ptr = val;
            return &l;
        }
        long(*two_pointers(double val, double(*d)));
        long *(two_pointers)(double val, double *(d));
        long *(two_pointers)(double val, double(*(d)));
        unsigned **pointers_to_pointers(int **p)
        {
            static unsigned u;
            static unsigned *u_ptr;
            u_ptr = &u;
            u = **p;
            return &u_ptr;
        }
        unsigned(**(pointers_to_pointers(int *(*p))));
        unsigned *(*pointers_to_pointers(int(**p)));
        unsigned(*(*((pointers_to_pointers)(int(*(*(p)))))));
        int main(void)
        {
            int i = 0;
            int(*i_ptr) = &i;
            int(**ptr_to_iptr) = &i_ptr;
            double(d1) = 0.0;
            double d2 = 10.0;
            double *(d_ptr) = &d1;
            long(*(l_ptr));
            unsigned *(*(ptr_to_uptr));
            i = return_3();
            if (i != 3)
                return 1;
            if (*i_ptr != 3) {
                return 2;
            }
            l_ptr = two_pointers(d2, d_ptr);
            if (l_ptr != &l) {
                return 3;
            }
            if (*l_ptr != 100) {
                return 4;
            }
            if (*d_ptr != 10.0) {
                return 5;
            }
            if (d1 != 10.0) {
                return 6;
            }
            ptr_to_uptr = pointers_to_pointers(ptr_to_iptr);
            if (**ptr_to_uptr != 3) {
                return 7;
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
        Semicolon,
        Int,
        OpenParen,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        CloseParen,
        Semicolon,
        Int,
        OpenParen,
        Identifier,
        CloseParen,
        OpenParen,
        Void,
        CloseParen,
        Semicolon,
        Int,
        OpenParen,
        OpenParen,
        Identifier,
        CloseParen,
        CloseParen,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Long,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Long,
        Star,
        Identifier,
        OpenParen,
        Double,
        Identifier,
        Comma,
        Double,
        Star,
        Identifier,
        CloseParen,
        OpenBrace,
        Star,
        Identifier,
        Equal,
        Identifier,
        Semicolon,
        Return,
        Ampersand,
        Identifier,
        Semicolon,
        CloseBrace,
        Long,
        OpenParen,
        Star,
        Identifier,
        OpenParen,
        Double,
        Identifier,
        Comma,
        Double,
        OpenParen,
        Star,
        Identifier,
        CloseParen,
        CloseParen,
        CloseParen,
        Semicolon,
        Long,
        Star,
        OpenParen,
        Identifier,
        CloseParen,
        OpenParen,
        Double,
        Identifier,
        Comma,
        Double,
        Star,
        OpenParen,
        Identifier,
        CloseParen,
        CloseParen,
        Semicolon,
        Long,
        Star,
        OpenParen,
        Identifier,
        CloseParen,
        OpenParen,
        Double,
        Identifier,
        Comma,
        Double,
        OpenParen,
        Star,
        OpenParen,
        Identifier,
        CloseParen,
        CloseParen,
        CloseParen,
        Semicolon,
        Unsigned,
        Star,
        Star,
        Identifier,
        OpenParen,
        Int,
        Star,
        Star,
        Identifier,
        CloseParen,
        OpenBrace,
        Static,
        Unsigned,
        Identifier,
        Semicolon,
        Static,
        Unsigned,
        Star,
        Identifier,
        Semicolon,
        Identifier,
        Equal,
        Ampersand,
        Identifier,
        Semicolon,
        Identifier,
        Equal,
        Star,
        Star,
        Identifier,
        Semicolon,
        Return,
        Ampersand,
        Identifier,
        Semicolon,
        CloseBrace,
        Unsigned,
        OpenParen,
        Star,
        Star,
        OpenParen,
        Identifier,
        OpenParen,
        Int,
        Star,
        OpenParen,
        Star,
        Identifier,
        CloseParen,
        CloseParen,
        CloseParen,
        CloseParen,
        Semicolon,
        Unsigned,
        Star,
        OpenParen,
        Star,
        Identifier,
        OpenParen,
        Int,
        OpenParen,
        Star,
        Star,
        Identifier,
        CloseParen,
        CloseParen,
        CloseParen,
        Semicolon,
        Unsigned,
        OpenParen,
        Star,
        OpenParen,
        Star,
        OpenParen,
        OpenParen,
        Identifier,
        CloseParen,
        OpenParen,
        Int,
        OpenParen,
        Star,
        OpenParen,
        Star,
        OpenParen,
        Identifier,
        CloseParen,
        CloseParen,
        CloseParen,
        CloseParen,
        CloseParen,
        CloseParen,
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
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        OpenParen,
        Star,
        Identifier,
        CloseParen,
        Equal,
        Ampersand,
        Identifier,
        Semicolon,
        Int,
        OpenParen,
        Star,
        Star,
        Identifier,
        CloseParen,
        Equal,
        Ampersand,
        Identifier,
        Semicolon,
        Double,
        OpenParen,
        Identifier,
        CloseParen,
        Equal,
        DoubleConstant,
        Semicolon,
        Double,
        Identifier,
        Equal,
        DoubleConstant,
        Semicolon,
        Double,
        Star,
        OpenParen,
        Identifier,
        CloseParen,
        Equal,
        Ampersand,
        Identifier,
        Semicolon,
        Long,
        OpenParen,
        Star,
        OpenParen,
        Identifier,
        CloseParen,
        CloseParen,
        Semicolon,
        Unsigned,
        Star,
        OpenParen,
        Star,
        OpenParen,
        Identifier,
        CloseParen,
        CloseParen,
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
        IntConstant(IntKind::Int),
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Star,
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
        Identifier,
        OpenParen,
        Identifier,
        Comma,
        Identifier,
        CloseParen,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        Ampersand,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Star,
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
        Star,
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
        Identifier,
        Equal,
        Identifier,
        OpenParen,
        Identifier,
        CloseParen,
        Semicolon,
        If,
        OpenParen,
        Star,
        Star,
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
fn test_valid_declarators_declare_pointer_in_for_loop() {
    let src = r#"
        int main(void) {
            int x = 10;
            for (int *i = &x; i != 0; ) {
                *i = 5;
                i = 0;
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
        IntConstant(IntKind::Int),
        Semicolon,
        For,
        OpenParen,
        Int,
        Star,
        Identifier,
        Equal,
        Ampersand,
        Identifier,
        Semicolon,
        Identifier,
        BangEqual,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseParen,
        OpenBrace,
        Star,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        Equal,
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
fn test_valid_dereference_address_of_dereference() {
    let src = r#"
        int main(void) {
            int *null_ptr = 0;
            if (&*null_ptr != 0)
                return 1;
            int **ptr_to_null = &null_ptr;
            if (&**ptr_to_null)
                return 2;
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
        Star,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Ampersand,
        Star,
        Identifier,
        BangEqual,
        IntConstant(IntKind::Int),
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Star,
        Star,
        Identifier,
        Equal,
        Ampersand,
        Identifier,
        Semicolon,
        If,
        OpenParen,
        Ampersand,
        Star,
        Star,
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
fn test_valid_dereference_dereference_expression_result() {
    let src = r#"
        int *return_pointer(void) {
            static int var = 10;
            return &var;
        }
        int one = 1;
        int main(void) {
            int val = 100;
            int *ptr_var = &val;
            if (*return_pointer() != 10) {
                return 1;
            }
            if (*(one ? return_pointer() : ptr_var) != 10)
                return 2;
            if (*(one - 1 ? return_pointer() : ptr_var) != 100) {
                return 3;
            }
            int *ptr_to_one = &one;
            if (*(ptr_var = ptr_to_one) != 1) {
                return 4;
            }
            *return_pointer() = 20;
            *(one ? ptr_var : return_pointer()) = 30;
            if (*return_pointer() != 20) {
                return 5;
            }
            if (*ptr_var != 30) {
                return 6;
            }
            if (one != 30) {
                return 7;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Star,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Static,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Return,
        Ampersand,
        Identifier,
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
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Star,
        Identifier,
        Equal,
        Ampersand,
        Identifier,
        Semicolon,
        If,
        OpenParen,
        Star,
        Identifier,
        OpenParen,
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
        Star,
        OpenParen,
        Identifier,
        Question,
        Identifier,
        OpenParen,
        CloseParen,
        Colon,
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
        Star,
        OpenParen,
        Identifier,
        Minus,
        IntConstant(IntKind::Int),
        Question,
        Identifier,
        OpenParen,
        CloseParen,
        Colon,
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
        Int,
        Star,
        Identifier,
        Equal,
        Ampersand,
        Identifier,
        Semicolon,
        If,
        OpenParen,
        Star,
        OpenParen,
        Identifier,
        Equal,
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
        Star,
        Identifier,
        OpenParen,
        CloseParen,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Star,
        OpenParen,
        Identifier,
        Question,
        Identifier,
        Colon,
        Identifier,
        OpenParen,
        CloseParen,
        CloseParen,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Star,
        Identifier,
        OpenParen,
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
        Star,
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
fn test_valid_dereference_multilevel_indirection() {
    let src = r#"
        
        int main(void) {
            double d = 10.0;
            double *d_ptr = &d;
            double **d_ptr_ptr = &d_ptr;
            double ***d_ptr_ptr_ptr = &d_ptr_ptr;
            if (d != 10.0) {
                return 1;
            }
            if (*d_ptr != 10.0) {
                return 2;
            }
            if (**d_ptr_ptr != 10.0) {
                return 3;
            }
            if (***d_ptr_ptr_ptr != 10.0) {
                return 4;
            }
            if (&d != d_ptr) {
                return 5;
            }
            if (*d_ptr_ptr != d_ptr) {
                return 6;
            }
            if (**d_ptr_ptr_ptr != d_ptr) {
                return 7;
            }
            ***d_ptr_ptr_ptr = 5.0;
            if (d != 5.0) {
                return 8;
            }
            if (*d_ptr != 5.0) {
                return 9;
            }
            if (**d_ptr_ptr != 5.0) {
                return 10;
            }
            if (***d_ptr_ptr_ptr != 5.0) {
                return 11;
            }
            double d2 = 1.0;
            double *d2_ptr = &d2;
            double *d2_ptr2 = d2_ptr;
            double **d2_ptr_ptr = &d2_ptr;
            *d_ptr_ptr_ptr = d2_ptr_ptr;
            if (**d_ptr_ptr_ptr != d2_ptr) {
                return 12;
            }
            if (***d_ptr_ptr_ptr != 1.0) {
                return 13;
            }
            if (d2_ptr_ptr == &d2_ptr2)
                return 14;
            d2_ptr = d_ptr;
            if (**d_ptr_ptr_ptr != d_ptr) {
                return 15;
            }
            if (*d2_ptr_ptr != d_ptr) {
                return 16;
            }
            if (**d_ptr_ptr_ptr == d2_ptr2) {
                return 17;
            }
            if (***d_ptr_ptr_ptr != 5.0) {
                return 18;
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
        Double,
        Star,
        Identifier,
        Equal,
        Ampersand,
        Identifier,
        Semicolon,
        Double,
        Star,
        Star,
        Identifier,
        Equal,
        Ampersand,
        Identifier,
        Semicolon,
        Double,
        Star,
        Star,
        Star,
        Identifier,
        Equal,
        Ampersand,
        Identifier,
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
        Star,
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
        Star,
        Star,
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
        Star,
        Star,
        Star,
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
        Ampersand,
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
        Star,
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
        Star,
        Star,
        Identifier,
        BangEqual,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Star,
        Star,
        Star,
        Identifier,
        Equal,
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
        If,
        OpenParen,
        Star,
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
        Star,
        Star,
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
        Star,
        Star,
        Star,
        Identifier,
        BangEqual,
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
        DoubleConstant,
        Semicolon,
        Double,
        Star,
        Identifier,
        Equal,
        Ampersand,
        Identifier,
        Semicolon,
        Double,
        Star,
        Identifier,
        Equal,
        Identifier,
        Semicolon,
        Double,
        Star,
        Star,
        Identifier,
        Equal,
        Ampersand,
        Identifier,
        Semicolon,
        Star,
        Identifier,
        Equal,
        Identifier,
        Semicolon,
        If,
        OpenParen,
        Star,
        Star,
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
        Star,
        Star,
        Star,
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
        EqualEqual,
        Ampersand,
        Identifier,
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Semicolon,
        If,
        OpenParen,
        Star,
        Star,
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
        Star,
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
        Star,
        Star,
        Identifier,
        EqualEqual,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Star,
        Star,
        Star,
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
fn test_valid_dereference_read_through_pointers() {
    let src = r#"
        
        int main(void) {
            int i = -100;
            unsigned long ul = 13835058055282163712ul;
            double d = 3.5;
            int *i_ptr = &i;
            unsigned long *ul_ptr = &ul;
            double *d_ptr = &d;
            if (*i_ptr != -100) {
                return 1;
            }
            if (*ul_ptr != 13835058055282163712ul) {
                return 2;
            }
            if (*d_ptr != 3.5) {
                return 3;
            }
            i = 12;
            ul = 1000;
            d = -000.001;
            if (*i_ptr != 12) {
                return 4;
            }
            if (*ul_ptr != 1000) {
                return 5;
            }
            if (*d_ptr != -000.001) {
                return 6;
            }
            int i2 = 1;
            unsigned long ul2 = 144115196665790464ul;
            double d2 = -33.3;
            i_ptr = &i2;
            ul_ptr = &ul2;
            d_ptr = &d2;
            if (*i_ptr != 1) {
                return 7;
            }
            if (*ul_ptr != 144115196665790464ul) {
                return 8;
            }
            if (*d_ptr != -33.3) {
                return 9;
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
        Unsigned,
        Long,
        Identifier,
        Equal,
        IntConstant(IntKind::ULong),
        Semicolon,
        Double,
        Identifier,
        Equal,
        DoubleConstant,
        Semicolon,
        Int,
        Star,
        Identifier,
        Equal,
        Ampersand,
        Identifier,
        Semicolon,
        Unsigned,
        Long,
        Star,
        Identifier,
        Equal,
        Ampersand,
        Identifier,
        Semicolon,
        Double,
        Star,
        Identifier,
        Equal,
        Ampersand,
        Identifier,
        Semicolon,
        If,
        OpenParen,
        Star,
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
        Star,
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
        Star,
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
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Minus,
        DoubleConstant,
        Semicolon,
        If,
        OpenParen,
        Star,
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
        Star,
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
        Star,
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
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Unsigned,
        Long,
        Identifier,
        Equal,
        IntConstant(IntKind::ULong),
        Semicolon,
        Double,
        Identifier,
        Equal,
        Minus,
        DoubleConstant,
        Semicolon,
        Identifier,
        Equal,
        Ampersand,
        Identifier,
        Semicolon,
        Identifier,
        Equal,
        Ampersand,
        Identifier,
        Semicolon,
        Identifier,
        Equal,
        Ampersand,
        Identifier,
        Semicolon,
        If,
        OpenParen,
        Star,
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
        Star,
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
        Star,
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
fn test_valid_dereference_simple() {
    let src = r#"
        int main(void) {
            int x = 3;
            int *ptr = &x;
            return *ptr;
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
        Star,
        Identifier,
        Equal,
        Ampersand,
        Identifier,
        Semicolon,
        Return,
        Star,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_dereference_static_var_indirection() {
    let src = r#"
        unsigned int w = 4294967295U;
        int x = 10;
        unsigned int y = 4294967295U;
        double *dbl_ptr;
        long modify_ptr(long *new_ptr) {
            static long *p;
            if (new_ptr)
            {
                p = new_ptr;
            }
            return *p;
        }
        int increment_ptr(void)
        {
            *dbl_ptr = *dbl_ptr + 5.0;
            return 0;
        }
        int main(void) {
            int *pointer_to_static = &x;
            x = 20;
            if (*pointer_to_static != 20) {
                return 1;
            }
            *pointer_to_static = 100;
            if (x != 100) {
                return 2;
            }
            if (w != 4294967295U) {
                return 3;
            }
            if (y != 4294967295U) {
                return 4;
            }
            if (dbl_ptr) {
                return 5;
            }
            long l = 1000l;
            if (modify_ptr(&l) != 1000l) {
                return 6;
            }
            l = -1;
            if (modify_ptr(0) != l) {
                return 7;
            }
            double d = 10.0;
            dbl_ptr = &d;
            increment_ptr();
            if (*dbl_ptr != 15) {
                return 8;
            }
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
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Unsigned,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Uint),
        Semicolon,
        Double,
        Star,
        Identifier,
        Semicolon,
        Long,
        Identifier,
        OpenParen,
        Long,
        Star,
        Identifier,
        CloseParen,
        OpenBrace,
        Static,
        Long,
        Star,
        Identifier,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        CloseParen,
        OpenBrace,
        Identifier,
        Equal,
        Identifier,
        Semicolon,
        CloseBrace,
        Return,
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
        Star,
        Identifier,
        Equal,
        Star,
        Identifier,
        Plus,
        DoubleConstant,
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
        Star,
        Identifier,
        Equal,
        Ampersand,
        Identifier,
        Semicolon,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Star,
        Identifier,
        BangEqual,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Star,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
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
        Identifier,
        OpenParen,
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
        Identifier,
        Equal,
        Minus,
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        OpenParen,
        IntConstant(IntKind::Int),
        CloseParen,
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
        DoubleConstant,
        Semicolon,
        Identifier,
        Equal,
        Ampersand,
        Identifier,
        Semicolon,
        Identifier,
        OpenParen,
        CloseParen,
        Semicolon,
        If,
        OpenParen,
        Star,
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
fn test_valid_dereference_update_through_pointers() {
    let src = r#"
        int main(void) {
            unsigned int i = 2185232384u;
            signed long l = 144115196665790464l;
            double d = 1e50;
            unsigned *i_ptr = &i;
            long *l_ptr = &l;
            double *d_ptr = &d;
            *i_ptr = 10;
            *l_ptr = -20;
            *d_ptr = 30.1;
            if (i != 10) {
                return 1;
            }
            if (l != -20) {
                return 2;
            }
            if (d != 30.1) {
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
        Unsigned,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Uint),
        Semicolon,
        Signed,
        Long,
        Identifier,
        Equal,
        IntConstant(IntKind::Long),
        Semicolon,
        Double,
        Identifier,
        Equal,
        DoubleConstant,
        Semicolon,
        Unsigned,
        Star,
        Identifier,
        Equal,
        Ampersand,
        Identifier,
        Semicolon,
        Long,
        Star,
        Identifier,
        Equal,
        Ampersand,
        Identifier,
        Semicolon,
        Double,
        Star,
        Identifier,
        Equal,
        Ampersand,
        Identifier,
        Semicolon,
        Star,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Star,
        Identifier,
        Equal,
        Minus,
        IntConstant(IntKind::Int),
        Semicolon,
        Star,
        Identifier,
        Equal,
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
fn test_valid_extra_credit_bitshift_dereferenced_ptrs() {
    let src = r#"
        unsigned int ui = 4294967295;
        unsigned int *get_ui_ptr(void){
            return &ui;
        }
        int shiftcount = 5;
        int main(void) {
            if ((*get_ui_ptr() << 2l) != 4294967292) {
                return 1;
            }
            if ((*get_ui_ptr() >> 2) != 1073741823) {
                return 2;
            }
            int *shiftcount_ptr = &shiftcount;
            if ((1000000u >> *shiftcount_ptr) != 31250) {
                return 3;
            }
            if ((1000000u << *shiftcount_ptr) != 32000000) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Unsigned,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Unsigned,
        Int,
        Star,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Ampersand,
        Identifier,
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
        If,
        OpenParen,
        OpenParen,
        Star,
        Identifier,
        OpenParen,
        CloseParen,
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
        Star,
        Identifier,
        OpenParen,
        CloseParen,
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
        Int,
        Star,
        Identifier,
        Equal,
        Ampersand,
        Identifier,
        Semicolon,
        If,
        OpenParen,
        OpenParen,
        IntConstant(IntKind::Uint),
        GreaterGreater,
        Star,
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
        Star,
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
fn test_valid_extra_credit_bitwise_ops_with_dereferenced_ptrs() {
    let src = r#"
        int main(void) {
            unsigned int ui = -1u;
            unsigned long ul = 9223372036854775808ul;
            unsigned int *ui_ptr = &ui;
            unsigned long *ul_ptr = &ul;
            if ((*ui_ptr & *ul_ptr) != 0) {
                return 1;
            }
            if ((*ui_ptr | *ul_ptr) != 9223372041149743103ul) {
                return 2;
            }
            int i = -1;
            signed int *i_ptr = &i;
            if ((*i_ptr & ul) != *ul_ptr) {
                return 3;
            }
            if ((*i_ptr | *ul_ptr) != i) {
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
        Unsigned,
        Long,
        Identifier,
        Equal,
        IntConstant(IntKind::ULong),
        Semicolon,
        Unsigned,
        Int,
        Star,
        Identifier,
        Equal,
        Ampersand,
        Identifier,
        Semicolon,
        Unsigned,
        Long,
        Star,
        Identifier,
        Equal,
        Ampersand,
        Identifier,
        Semicolon,
        If,
        OpenParen,
        OpenParen,
        Star,
        Identifier,
        Ampersand,
        Star,
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
        Star,
        Identifier,
        Pipe,
        Star,
        Identifier,
        CloseParen,
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
        Minus,
        IntConstant(IntKind::Int),
        Semicolon,
        Signed,
        Int,
        Star,
        Identifier,
        Equal,
        Ampersand,
        Identifier,
        Semicolon,
        If,
        OpenParen,
        OpenParen,
        Star,
        Identifier,
        Ampersand,
        Identifier,
        CloseParen,
        BangEqual,
        Star,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        OpenParen,
        Star,
        Identifier,
        Pipe,
        Star,
        Identifier,
        CloseParen,
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
fn test_valid_extra_credit_compound_assign_conversion() {
    let src = r#"
        int main(void) {
            double d = 5.0;
            double *d_ptr = &d;
            *d_ptr *= 1000u;
            if (d != 5000.0) {
                return 1;
            }
            int i = -50;
            int *i_ptr = &i;
            *i_ptr %= 4294967200U;
            if (*i_ptr != 46) {
                return 2;
            }
            unsigned int ui = 4294967295U;
            ui /= *d_ptr;
            if (ui != 858993u) {
                return 3;
            }
            i = -10;
            unsigned long ul = 9223372036854775807ul;
            unsigned long *ul_ptr = &ul;
            *i_ptr -= *ul_ptr;
            if (i != -9) {
                return 4;
            }
            if (ul != 9223372036854775807ul) {
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
        Double,
        Identifier,
        Equal,
        DoubleConstant,
        Semicolon,
        Double,
        Star,
        Identifier,
        Equal,
        Ampersand,
        Identifier,
        Semicolon,
        Star,
        Identifier,
        StarEqual,
        IntConstant(IntKind::Uint),
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
        Int,
        Identifier,
        Equal,
        Minus,
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Star,
        Identifier,
        Equal,
        Ampersand,
        Identifier,
        Semicolon,
        Star,
        Identifier,
        PercentEqual,
        IntConstant(IntKind::Uint),
        Semicolon,
        If,
        OpenParen,
        Star,
        Identifier,
        BangEqual,
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Uint),
        Semicolon,
        Identifier,
        SlashEqual,
        Star,
        Identifier,
        Semicolon,
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
        IntConstant(IntKind::Int),
        Semicolon,
        Unsigned,
        Long,
        Identifier,
        Equal,
        IntConstant(IntKind::ULong),
        Semicolon,
        Unsigned,
        Long,
        Star,
        Identifier,
        Equal,
        Ampersand,
        Identifier,
        Semicolon,
        Star,
        Identifier,
        MinusEqual,
        Star,
        Identifier,
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
fn test_valid_extra_credit_compound_assign_through_pointer() {
    let src = r#"
        int main(void) {
            int x = 10;
            int *ptr = &x;
            *ptr += 5;
            if (x != 15) {
                return 1;
            }
            if ((*ptr -= 12) != 3) {
                return 2;
            }
            if (x != 3) {
                return 3;
            }
            *ptr *= 6;
            if (x != 18) {
                return 4;
            }
            *ptr /= 9;
            if (x != 2) {
                return 5;
            }
            *ptr %= 3;
            if (x != 2) {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Star,
        Identifier,
        Equal,
        Ampersand,
        Identifier,
        Semicolon,
        Star,
        Identifier,
        PlusEqual,
        IntConstant(IntKind::Int),
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
        Star,
        Identifier,
        MinusEqual,
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
        Star,
        Identifier,
        StarEqual,
        IntConstant(IntKind::Int),
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
        Star,
        Identifier,
        SlashEqual,
        IntConstant(IntKind::Int),
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
        Star,
        Identifier,
        PercentEqual,
        IntConstant(IntKind::Int),
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
fn test_valid_extra_credit_compound_bitwise_dereferenced_ptrs() {
    let src = r#"
        unsigned long ul = 18446460386757245432ul;
        int main(void) {
            unsigned long *ul_ptr = &ul;
            *ul_ptr &= -1000;
            if (ul != 18446460386757244952ul ) {
                return 1;
            }
            *ul_ptr |= 4294967040u;
            if (ul != 18446460386824683288ul ) {
                return 2;
            }
            int i = 123456;
            unsigned int ui = 4042322160u;
            long l = -252645136;
            unsigned int *ui_ptr = &ui;
            long *l_ptr = &l;
            if (*ui_ptr ^= *l_ptr) {
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
        Star,
        Identifier,
        Equal,
        Ampersand,
        Identifier,
        Semicolon,
        Star,
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
        Star,
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
        Unsigned,
        Int,
        Star,
        Identifier,
        Equal,
        Ampersand,
        Identifier,
        Semicolon,
        Long,
        Star,
        Identifier,
        Equal,
        Ampersand,
        Identifier,
        Semicolon,
        If,
        OpenParen,
        Star,
        Identifier,
        CircumflexEqual,
        Star,
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
fn test_valid_extra_credit_eval_compound_lhs_once() {
    let src = r#"
        int i = 0;
        int putchar(int c);
        int *print_A(void) {
            putchar(65);
            return &i;
        }
        int *print_B(void) {
            putchar(66);
            return &i;
        }
        int main(void) {
            *print_A() += 5;
            if (i != 5) {
                return 1;
            }
            *print_B() += 5l;
            if (i != 10) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Int,
        Identifier,
        CloseParen,
        Semicolon,
        Int,
        Star,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Identifier,
        OpenParen,
        IntConstant(IntKind::Int),
        CloseParen,
        Semicolon,
        Return,
        Ampersand,
        Identifier,
        Semicolon,
        CloseBrace,
        Int,
        Star,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Identifier,
        OpenParen,
        IntConstant(IntKind::Int),
        CloseParen,
        Semicolon,
        Return,
        Ampersand,
        Identifier,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Star,
        Identifier,
        OpenParen,
        CloseParen,
        PlusEqual,
        IntConstant(IntKind::Int),
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
        Star,
        Identifier,
        OpenParen,
        CloseParen,
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
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_incr_and_decr_through_pointer() {
    let src = r#"
        
        int main(void) {
            int x = 10;
            int *y = &x;
            if (++*y != 11) {
                return 1;
            }
            if (x != 11) {
                return 2;
            }
            if (--*y != 10) {
                return 3;
            }
            if (x != 10) {
                return 4;
            }
            if ((*y)++ != 10) {
                return 5;
            }
            if (x != 11) {
                return 6;
            }
            if ((*y)-- != 11) {
                return 7;
            }
            if (x != 10) {
                return 8;
            }
            unsigned long ul = 0;
            unsigned long *ul_ptr = &ul;
            if ((*ul_ptr)--) {
                return 9;
            }
            if (ul != 18446744073709551615UL) {
                return 10;
            }
            double d = 0.0;
            double *d_ptr = &d;
            if (++(*d_ptr) != 1.0) {
                return 11;
            }
            if (d != 1.0) {
                return 12;
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
        Int,
        Star,
        Identifier,
        Equal,
        Ampersand,
        Identifier,
        Semicolon,
        If,
        OpenParen,
        PlusPlus,
        Star,
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
        MinusMinus,
        Star,
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
        OpenParen,
        Star,
        Identifier,
        CloseParen,
        PlusPlus,
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
        OpenParen,
        Star,
        Identifier,
        CloseParen,
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
        IntConstant(IntKind::Int),
        Semicolon,
        Unsigned,
        Long,
        Star,
        Identifier,
        Equal,
        Ampersand,
        Identifier,
        Semicolon,
        If,
        OpenParen,
        OpenParen,
        Star,
        Identifier,
        CloseParen,
        MinusMinus,
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
        Double,
        Identifier,
        Equal,
        DoubleConstant,
        Semicolon,
        Double,
        Star,
        Identifier,
        Equal,
        Ampersand,
        Identifier,
        Semicolon,
        If,
        OpenParen,
        PlusPlus,
        OpenParen,
        Star,
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
fn test_valid_extra_credit_switch_dereferenced_pointer() {
    let src = r#"
        long l = 4294967300l;
        long *get_ptr(void) {
            return &l;
        }
        int main(void) {
            switch (*get_ptr()) {
                case 1:
                    return 1;
                case 4:
                    return 2;
                case 4294967300l:
                    return 0;
                case 18446744073709551600UL:
                    return 3;
                default:
                    return 4;
            }
        }
    "#;
    let expected = vec![
        Long,
        Identifier,
        Equal,
        IntConstant(IntKind::Long),
        Semicolon,
        Long,
        Star,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Return,
        Ampersand,
        Identifier,
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        OpenBrace,
        Switch,
        OpenParen,
        Star,
        Identifier,
        OpenParen,
        CloseParen,
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
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_function_calls_address_of_argument() {
    let src = r#"
        int addr_of_arg(int a) {
            int *ptr = &a;
            *ptr = 10;
            return a;
        }
        int main(void) {
            int result = addr_of_arg(-20);
            if (result != 10) {
                return 1;
            }
            int var = 100;
            result = addr_of_arg(var);
            if (result != 10) {
                return 2;
            }
            if (var != 100) {
                return 3;
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
        CloseParen,
        OpenBrace,
        Int,
        Star,
        Identifier,
        Equal,
        Ampersand,
        Identifier,
        Semicolon,
        Star,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
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
        Identifier,
        Equal,
        Identifier,
        OpenParen,
        Identifier,
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
fn test_valid_function_calls_return_pointer() {
    let src = r#"
        
        int *return_pointer(int *in) {
            return in;
        }
        int main(void) {
            int x = 10;
            int *x_ptr = return_pointer(&x);
            if (*x_ptr != 10)
                return 1;
            x = 100;
            if (*x_ptr != 100)
                return 2;
            if (x_ptr != &x)
                return 3;
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Star,
        Identifier,
        OpenParen,
        Int,
        Star,
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
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Star,
        Identifier,
        Equal,
        Identifier,
        OpenParen,
        Ampersand,
        Identifier,
        CloseParen,
        Semicolon,
        If,
        OpenParen,
        Star,
        Identifier,
        BangEqual,
        IntConstant(IntKind::Int),
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Star,
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
        Ampersand,
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
fn test_valid_function_calls_update_value_through_pointer_parameter() {
    let src = r#"
        
        int update_value(int *ptr) {
            int old_val = *ptr;
            *ptr = 10;
            return old_val;
        }
        int main(void) {
            int x = 20;
            int result = update_value(&x);
            if (result != 20) {
                return 1;
            }
            if (x != 10) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Int,
        Identifier,
        OpenParen,
        Int,
        Star,
        Identifier,
        CloseParen,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        Star,
        Identifier,
        Semicolon,
        Star,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
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
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        Identifier,
        OpenParen,
        Ampersand,
        Identifier,
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
fn test_valid_libraries_global_pointer() {
    let src = r#"
        double *d_ptr;
        int update_thru_ptr(double new_val) {
            *d_ptr = new_val;
            return 0;
        }
    "#;
    let expected = vec![
        Double,
        Star,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Double,
        Identifier,
        CloseParen,
        OpenBrace,
        Star,
        Identifier,
        Equal,
        Identifier,
        Semicolon,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_libraries_global_pointer_client() {
    let src = r#"
        extern double *d_ptr;
        int update_thru_ptr(double new_val);
        int main(void) {
            double d = 0.0;
            d_ptr = &d;
            update_thru_ptr(10.0);
            return (d == 10.0);
        }
    "#;
    let expected = vec![
        Extern,
        Double,
        Star,
        Identifier,
        Semicolon,
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
        Double,
        Identifier,
        Equal,
        DoubleConstant,
        Semicolon,
        Identifier,
        Equal,
        Ampersand,
        Identifier,
        Semicolon,
        Identifier,
        OpenParen,
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
fn test_valid_libraries_static_pointer() {
    let src = r#"
        static long *long_ptr;
        long *get_pointer(void) {
            return long_ptr;
        }
        int set_pointer(long *new_ptr) {
            long_ptr = new_ptr;
            return 0;
        }
    "#;
    let expected = vec![
        Static,
        Long,
        Star,
        Identifier,
        Semicolon,
        Long,
        Star,
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
        Long,
        Star,
        Identifier,
        CloseParen,
        OpenBrace,
        Identifier,
        Equal,
        Identifier,
        Semicolon,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_libraries_static_pointer_client() {
    let src = r#"
        long *get_pointer(void);
        int set_pointer(long *new_ptr);
        static long private_long = 100l;
        int main(void) {
            long *initial_ptr = get_pointer();
            if (initial_ptr) {
                return 1;
            }
            set_pointer(&private_long);
            long *new_ptr = get_pointer();
            if (initial_ptr == new_ptr) {
                return 2;
            }
            if (*new_ptr != 100l) {
                return 3;
            }
            if (new_ptr != &private_long) {
                return 4;
            }
            set_pointer(0);
            if (get_pointer()) {
                return 5;
            }
            if (new_ptr != &private_long) {
                return 6;
            }
            return 0;
        }
    "#;
    let expected = vec![
        Long,
        Star,
        Identifier,
        OpenParen,
        Void,
        CloseParen,
        Semicolon,
        Int,
        Identifier,
        OpenParen,
        Long,
        Star,
        Identifier,
        CloseParen,
        Semicolon,
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
        Long,
        Star,
        Identifier,
        Equal,
        Identifier,
        OpenParen,
        CloseParen,
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
        Identifier,
        OpenParen,
        Ampersand,
        Identifier,
        CloseParen,
        Semicolon,
        Long,
        Star,
        Identifier,
        Equal,
        Identifier,
        OpenParen,
        CloseParen,
        Semicolon,
        If,
        OpenParen,
        Identifier,
        EqualEqual,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        If,
        OpenParen,
        Star,
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
        Ampersand,
        Identifier,
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        OpenParen,
        IntConstant(IntKind::Int),
        CloseParen,
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
        BangEqual,
        Ampersand,
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
