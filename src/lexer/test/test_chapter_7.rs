use crate::lexer::TokenKind::*;
use crate::lexer::{tokenize, IntKind};

#[test]
fn test_valid_assign_to_self() {
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
        IntConstant(IntKind::Int),
        Semicolon,
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
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_assign_to_self_2() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        OpenBrace,
        Int,
        Identifier,
        Equal,
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
fn test_valid_declaration_only() {
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
fn test_valid_empty_blocks() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        OpenBrace,
        CloseBrace,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Star,
        IntConstant(IntKind::Int),
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
fn test_valid_extra_credit_compound_subtract_in_block() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        Greater,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Identifier,
        MinusEqual,
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
        Greater,
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Identifier,
        MinusEqual,
        IntConstant(IntKind::Int),
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
fn test_valid_extra_credit_goto_before_declaration() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        OpenBrace,
        If,
        OpenParen,
        Identifier,
        BangEqual,
        IntConstant(IntKind::Int),
        CloseParen,
        Identifier,
        Colon,
        Return,
        Identifier,
        Semicolon,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
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
fn test_valid_extra_credit_goto_inner_scope() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Goto,
        Identifier,
        Semicolon,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        Colon,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
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
fn test_valid_extra_credit_goto_outer_scope() {
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
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        Colon,
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
fn test_valid_extra_credit_goto_sibling_scope() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        IntConstant(IntKind::Int),
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
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        Colon,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
        CloseParen,
        OpenBrace,
        Identifier,
        Colon,
        Semicolon,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
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
fn test_valid_hidden_then_visible() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Semicolon,
        OpenBrace,
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
        Identifier,
        Equal,
        Identifier,
        Plus,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
        Return,
        Identifier,
        EqualEqual,
        IntConstant(IntKind::Int),
        AmpersandAmpersand,
        Identifier,
        EqualEqual,
        Minus,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_hidden_variable() {
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
        IntConstant(IntKind::Int),
        Semicolon,
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
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_inner_uninitialized() {
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
        IntConstant(IntKind::Int),
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
fn test_valid_multiple_vars_same_name() {
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
        IntConstant(IntKind::Int),
        Semicolon,
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
        CloseBrace,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
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
fn test_valid_nested_if() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
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
        Else,
        OpenBrace,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
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
fn test_valid_similar_var_names() {
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
        IntConstant(IntKind::Int),
        Semicolon,
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
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
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
fn test_valid_use_in_inner_scope() {
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
        IntConstant(IntKind::Int),
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
