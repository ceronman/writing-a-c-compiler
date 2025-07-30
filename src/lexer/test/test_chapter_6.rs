use crate::lexer::TokenKind::*;
use crate::lexer::{IntKind, tokenize};

#[test]
#[should_panic]
fn test_invalid_lex_extra_credit_bad_label() {
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
fn test_valid_assign_ternary() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Question,
        IntConstant(IntKind::Int),
        Colon,
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
fn test_valid_binary_condition() {
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
        IntConstant(IntKind::Int),
        Plus,
        IntConstant(IntKind::Int),
        EqualEqual,
        IntConstant(IntKind::Int),
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_binary_false_condition() {
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
        IntConstant(IntKind::Int),
        Plus,
        IntConstant(IntKind::Int),
        EqualEqual,
        IntConstant(IntKind::Int),
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_else() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        Else,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_bitwise_ternary() {
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
        IntConstant(IntKind::Int),
        Circumflex,
        IntConstant(IntKind::Int),
        Question,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Colon,
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
fn test_valid_extra_credit_compound_assign_ternary() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        StarEqual,
        IntConstant(IntKind::Int),
        Question,
        IntConstant(IntKind::Int),
        Colon,
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
fn test_valid_extra_credit_compound_if_expression() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        PlusEqual,
        IntConstant(IntKind::Int),
        CloseParen,
        Return,
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
fn test_valid_extra_credit_goto_after_declaration() {
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
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
        CloseParen,
        Semicolon,
        Identifier,
        Colon,
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
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_goto_backwards() {
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
        IntConstant(IntKind::Int),
        CloseParen,
        Identifier,
        Colon,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        Goto,
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
fn test_valid_extra_credit_goto_label() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        Colon,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_goto_label_and_var() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Goto,
        Identifier,
        Semicolon,
        Return,
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
fn test_valid_extra_credit_goto_label_main() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        Colon,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_goto_label_main_2() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        Colon,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_goto_nested_label() {
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
fn test_valid_extra_credit_label_all_statements() {
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
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
        CloseParen,
        Identifier,
        Colon,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
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
        IntConstant(IntKind::Int),
        Semicolon,
        Goto,
        Identifier,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_label_token() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        Colon,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_lh_compound_assignment() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        OpenParen,
        Identifier,
        MinusEqual,
        IntConstant(IntKind::Int),
        CloseParen,
        Question,
        OpenParen,
        Identifier,
        SlashEqual,
        IntConstant(IntKind::Int),
        CloseParen,
        Colon,
        IntConstant(IntKind::Int),
        Semicolon,
        Return,
        Identifier,
        EqualEqual,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_postfix_if() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        MinusMinus,
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        Else,
        If,
        OpenParen,
        Identifier,
        MinusMinus,
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
fn test_valid_extra_credit_postfix_in_ternary() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        Minus,
        IntConstant(IntKind::Int),
        Question,
        IntConstant(IntKind::Int),
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
fn test_valid_extra_credit_prefix_if() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        PlusPlus,
        Identifier,
        CloseParen,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        Else,
        If,
        OpenParen,
        PlusPlus,
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
fn test_valid_extra_credit_prefix_in_ternary() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Return,
        OpenParen,
        PlusPlus,
        Identifier,
        Question,
        PlusPlus,
        Identifier,
        Colon,
        IntConstant(IntKind::Int),
        CloseParen,
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_unused_label() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_extra_credit_whitespace_after_label() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        Colon,
        Identifier,
        Colon,
        Return,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_if_nested() {
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
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Else,
        If,
        OpenParen,
        Identifier,
        CloseParen,
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
fn test_valid_if_nested_2() {
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
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Else,
        If,
        OpenParen,
        Tilde,
        Identifier,
        CloseParen,
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
fn test_valid_if_nested_3() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        OpenParen,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        CloseParen,
        CloseParen,
        If,
        OpenParen,
        Identifier,
        EqualEqual,
        IntConstant(IntKind::Int),
        CloseParen,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Else,
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
fn test_valid_if_nested_4() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Bang,
        Identifier,
        CloseParen,
        If,
        OpenParen,
        IntConstant(IntKind::Int),
        Slash,
        IntConstant(IntKind::Int),
        CloseParen,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Else,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Slash,
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
fn test_valid_if_nested_5() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        IntConstant(IntKind::Int),
        CloseParen,
        If,
        OpenParen,
        IntConstant(IntKind::Int),
        CloseParen,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Else,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Else,
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
fn test_valid_if_not_taken() {
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
fn test_valid_if_null_body() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        IntConstant(IntKind::Int),
        CloseParen,
        Semicolon,
        Else,
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
fn test_valid_if_taken() {
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
fn test_valid_lh_assignment() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        Equal,
        OpenParen,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        CloseParen,
        Question,
        Identifier,
        Colon,
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
fn test_valid_multiple_if() {
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
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Else,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        If,
        OpenParen,
        Identifier,
        CloseParen,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Else,
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
fn test_valid_nested_ternary() {
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
        Greater,
        Identifier,
        Question,
        IntConstant(IntKind::Int),
        Colon,
        Identifier,
        Question,
        IntConstant(IntKind::Int),
        Colon,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_nested_ternary_2() {
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
        IntConstant(IntKind::Int),
        Question,
        IntConstant(IntKind::Int),
        Question,
        IntConstant(IntKind::Int),
        Colon,
        IntConstant(IntKind::Int),
        Colon,
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Question,
        IntConstant(IntKind::Int),
        Question,
        IntConstant(IntKind::Int),
        Colon,
        IntConstant(IntKind::Int),
        Colon,
        IntConstant(IntKind::Int),
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
fn test_valid_rh_assignment() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        Question,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Colon,
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
fn test_valid_ternary() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Return,
        Identifier,
        Greater,
        Minus,
        IntConstant(IntKind::Int),
        Question,
        IntConstant(IntKind::Int),
        Colon,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_ternary_middle_assignment() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        BangEqual,
        IntConstant(IntKind::Int),
        Question,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Colon,
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
fn test_valid_ternary_middle_binop() {
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
        IntConstant(IntKind::Int),
        Question,
        IntConstant(IntKind::Int),
        Percent,
        IntConstant(IntKind::Int),
        Colon,
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
fn test_valid_ternary_precedence() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Return,
        Identifier,
        PipePipe,
        IntConstant(IntKind::Int),
        Question,
        IntConstant(IntKind::Int),
        Colon,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_ternary_rh_binop() {
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
        IntConstant(IntKind::Int),
        Question,
        IntConstant(IntKind::Int),
        Colon,
        IntConstant(IntKind::Int),
        PipePipe,
        IntConstant(IntKind::Int),
        Semicolon,
        CloseBrace,
    ];
    assert_eq!(tokenize(src), expected);
}

#[test]
fn test_valid_ternary_short_circuit() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        Question,
        OpenParen,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        CloseParen,
        Colon,
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
fn test_valid_ternary_short_circuit_2() {
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
        IntConstant(IntKind::Int),
        Semicolon,
        Int,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        Semicolon,
        Identifier,
        Question,
        OpenParen,
        Identifier,
        Equal,
        IntConstant(IntKind::Int),
        CloseParen,
        Colon,
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
