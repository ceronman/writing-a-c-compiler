use crate::lexer::TokenKind::*;
use crate::lexer::{IntKind, tokenize};

#[test]
fn test_valid_distinct_local_and_extern() {
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
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
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
fn test_valid_extern_block_scope_variable() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_bitwise_ops_file_scope_vars() {
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
        IntConstant(IntKind::Int),
        Semicolon,
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
        Identifier,
        Equal,
        Minus,
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        Equal,
        OpenParen,
        Identifier,
        LessLess,
        IntConstant(IntKind::Int),
        CloseParen,
        Pipe,
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
        Identifier,
        Equal,
        OpenParen,
        OpenParen,
        Identifier,
        Ampersand,
        Minus,
        IntConstant(IntKind::Int),
        CloseParen,
        Circumflex,
        IntConstant(IntKind::Int),
        CloseParen,
        GreaterGreater,
        IntConstant(IntKind::Int),
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
fn test_valid_extra_credit_compound_assignment_static_var() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Static,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Static,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Static,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        PlusEqual,
        IntConstant(IntKind::Int),
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
fn test_valid_extra_credit_goto_skip_static_initializer() {
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
        IntConstant(IntKind::Int),
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
fn test_valid_extra_credit_increment_global_vars() {
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
        IntConstant(IntKind::Int),
        Semicolon,
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
        Identifier,
        EqualEqual,
        IntConstant(IntKind::Int),
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
        EqualEqual,
        Minus,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Identifier,
        MinusMinus,
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
        Identifier,
        PlusPlus,
        Question,
        IntConstant(IntKind::Int),
        Colon,
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
        MinusMinus,
        Identifier,
        Question,
        Identifier,
        OpenParen,
        CloseParen,
        Colon,
        IntConstant(IntKind::Int),
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
fn test_valid_extra_credit_label_file_scope_var_same_name() {
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
        IntConstant(IntKind::Int),
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
fn test_valid_extra_credit_label_static_var_same_name() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Goto,
        Identifier,
        Semicolon,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
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
fn test_valid_extra_credit_libraries_same_label_same_fun() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        Colon,
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
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_libraries_same_label_same_fun_client() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        Colon,
        Return,
        IntConstant(IntKind::Int),
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
fn test_valid_extra_credit_switch_on_extern() {
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
        Semicolon,
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
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_switch_skip_extern_decl() {
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
        IntConstant(IntKind::Int),
        Semicolon,
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
        Extern,
        Int,
        Identifier,
        Semicolon,
        Case,
        IntConstant(IntKind::Int),
        Colon,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        Case,
        IntConstant(IntKind::Int),
        Colon,
        If,
        OpenParen,
        Identifier,
        Star,
        IntConstant(IntKind::Int),
        EqualEqual,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Default,
        Colon,
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
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_switch_skip_static_initializer() {
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
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
        Colon,
        Semicolon,
        Static,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Case,
        IntConstant(IntKind::Int),
        Colon,
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
fn test_valid_libraries_external_linkage_function() {
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
fn test_valid_libraries_external_linkage_function_client() {
    let src = r#"
        int add_one_and_two(void) {
            extern int sum(int a, int b);
            int sum(int a, int b);
            return sum(1, 2);
        }
        extern int sum(int x, int y);
        int sum(int x, int y);
        int add_three_and_four(void) {
            int sum = 3;
            if (sum > 2) {
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
        IntConstant(IntKind::Int),
        Comma,
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        Greater,
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
        Comma,
        IntConstant(IntKind::Int),
        CloseParen,
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
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_libraries_external_tentative_var() {
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
fn test_valid_libraries_external_tentative_var_client() {
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
        Identifier,
        OpenParen,
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
fn test_valid_libraries_external_var_scoping() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        EqualEqual,
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_libraries_external_var_scoping_client() {
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
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        EqualEqual,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
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
        Extern,
        Int,
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
fn test_valid_libraries_external_variable() {
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
        Semicolon,
        CloseBrace,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_libraries_external_variable_client() {
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
        IntConstant(IntKind::Int),
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
        OpenParen,
        CloseParen,
        BangEqual,
        IntConstant(IntKind::Int),
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        OpenParen,
        IntConstant(IntKind::Int),
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
        Identifier,
        OpenParen,
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
fn test_valid_libraries_internal_hides_external_linkage() {
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
        IntConstant(IntKind::Int),
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
fn test_valid_libraries_internal_hides_external_linkage_client() {
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
        IntConstant(IntKind::Int),
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
        Identifier,
        OpenParen,
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
fn test_valid_libraries_internal_linkage_function() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Plus,
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
fn test_valid_libraries_internal_linkage_function_client() {
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
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_libraries_internal_linkage_var() {
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
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
        Semicolon,
        Static,
        Int,
        Identifier,
        Semicolon,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_libraries_internal_linkage_var_client() {
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
        IntConstant(IntKind::Int),
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
        Extern,
        Int,
        Identifier,
        Semicolon,
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
        Identifier,
        OpenParen,
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
        Static,
        Int,
        Identifier,
        Semicolon,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_multiple_static_file_scope_vars() {
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
        IntConstant(IntKind::Int),
        Semicolon,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_multiple_static_local() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Star,
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
        Static,
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
fn test_valid_push_arg_on_page_boundary() {
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
        Identifier,
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_shadow_static_local_var() {
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
        IntConstant(IntKind::Int),
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
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
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_static_local_multiple_scopes() {
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
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
        Semicolon,
        Static,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Identifier,
        OpenParen,
        IntConstant(IntKind::Int),
        CloseParen,
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
        For,
        OpenParen,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        Less,
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
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
fn test_valid_static_local_uninitialized() {
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
        Semicolon,
        For,
        OpenParen,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        Less,
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
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
fn test_valid_static_recursive_call() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        OpenParen,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
        CloseParen,
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
        Less,
        IntConstant(IntKind::Int),
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
fn test_valid_static_then_extern() {
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
        IntConstant(IntKind::Int),
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
fn test_valid_static_variables_in_expressions() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Static,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
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
fn test_valid_tentative_definition() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        Less,
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        Equal,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
        CloseParen,
        Identifier,
        Equal,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
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
fn test_valid_type_before_storage_class() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Int,
        Static,
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
