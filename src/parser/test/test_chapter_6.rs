use super::{assert_error, assert_parse};

#[test]
fn test_invalid_parse_declaration_as_statement() {
    assert_error(
        r#"
        int main(void) {
            if (5)
                int i = 0;
              //^^^ Expected statement, but found 'int'
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_empty_if_body() {
    assert_error(
        r#"
        int main(void) {
            if (0) else return 0;
                 //^^^^ Expected statement, but found 'else'
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_extra_credit_goto_without_label() {
    assert_error(
        r#"
        int main(void) {
            goto;
              //^ Expected identifier, but found ';'
        lbl:
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_extra_credit_kw_label() {
    assert_error(
        r#"
        int main(void) {
            return: return 0;
                //^ Expected expression, but found ':'
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_extra_credit_label_declaration() {
    assert_error(
        r#"
        int main(void) {
        label:
            int a = 0;
          //^^^ Expected statement, but found 'int'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_extra_credit_label_expression_clause() {
    assert_error(
        r#"
        int main(void) {
            1 && label: 2;
                    //^ Expected ';', but found ':'
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_extra_credit_label_outside_function() {
    assert_error(
        r#"
        label:
      //^^^^^ Expected type specifier
        int main(void) {
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_extra_credit_label_without_statement() {
    assert_error(
        r#"
        int main(void) {
            foo:
        }
      //^ Expected statement, but found '}'
    "#,
    );
}

#[test]
fn test_invalid_parse_extra_credit_parenthesized_label() {
    assert_error(
        r#"
        int main(void) {
            goto(a);
              //^ Expected identifier, but found '('
        a:
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_if_assignment() {
    assert_error(
        r#"
        int main(void) {
            int flag = 0;
            int a = if (flag)
                  //^^ Expected expression, but found 'if'
                        2;
                    else
                        3;
            return a;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_if_no_parens() {
    assert_error(
        r#"
        int main(void) {
            if 0 return 1;
             //^ Expected '(', but found '0'
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_incomplete_ternary() {
    assert_error(
        r#"
        int main(void) {
            return 1 ? 2;
                      //^ Expected ':', but found ';'
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_malformed_ternary() {
    assert_error(
        r#"
        int main(void) {
            return 1 ? 2 : 3 : 4;
                           //^ Expected ';', but found ':'
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_malformed_ternary_2() {
    assert_error(
        r#"
        int main(void) {
            return 1 ? 2 ? 3 : 4;
                              //^ Expected ':', but found ';'
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_mismatched_nesting() {
    assert_error(
        r#"
        int main(void) {
            int a = 0;
            if (1)
                return 1;
            else
                return 2;
            else
          //^^^^ Expected statement, but found 'else'
                return 3;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_wrong_ternary_delimiter() {
    assert_error(
        r#"
        int main(void) {
            int x = 10;
            return x ? 1 = 2;
                          //^ Expected ':', but found ';'
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_extra_credit_duplicate_labels() {
    let src = r#"
        
        int main(void) {
            int x = 0;
        label:
            x = 1;
        label:
            return 2;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <9> Constant Int [0]
                    ├── Label [label]
                    │   ╰── <17> Assign [=]
                    │       ├── <14> Var [x]
                    │       ╰── <16> Constant Int [1]
                    ╰── Label [label]
                        ╰── Return
                            ╰── <21> Constant Int [2]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_extra_credit_goto_missing_label() {
    let src = r#"
        int main(void) {
            goto label;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── Goto [label]
                    ╰── Return
                        ╰── <8> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_extra_credit_goto_variable() {
    let src = r#"
        int main(void) {
            int a;
            goto a;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ╰── Type
                    │       ╰── Int
                    ├── Goto [a]
                    ╰── Return
                        ╰── <12> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_extra_credit_undeclared_var_in_labeled_statement() {
    let src = r#"
        int main(void) {
        lbl:
            return a;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── Label [lbl]
                    │   ╰── Return
                    │       ╰── <8> Var [a]
                    ╰── Return
                        ╰── <11> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_extra_credit_use_label_as_variable() {
    let src = r#"
        int main(void) {
            int x = 0;
            a:
            x = a;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <9> Constant Int [0]
                    ├── Label [a]
                    │   ╰── <18> Assign [=]
                    │       ├── <14> Var [x]
                    │       ╰── <17> Var [a]
                    ╰── Return
                        ╰── <21> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_invalid_var_in_if() {
    let src = r#"
        int main(void) {
            if (1)
                return c;
            int c = 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <6> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <8> Var [c]
                    ╰── VarDeclaration
                        ├── Name
                        │   ╰── c
                        ├── Type
                        │   ╰── Int
                        ╰── Initializer
                            ╰── <14> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_ternary_assign() {
    let src = r#"
        int main(void) {
            int a = 2;
            int b = 1;
            a > b ? a = 1 : a = 0;
            return a;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <9> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [1]
                    ├── <34> Assign [=]
                    │   ├── <31> Conditional [?]
                    │   │   ├── <23>  [>]
                    │   │   │   ├── <19> Var [a]
                    │   │   │   ╰── <22> Var [b]
                    │   │   ├── Then
                    │   │   │   ╰── <28> Assign [=]
                    │   │   │       ├── <25> Var [a]
                    │   │   │       ╰── <27> Constant Int [1]
                    │   │   ╰── Else
                    │   │       ╰── <30> Var [a]
                    │   ╰── <33> Constant Int [0]
                    ╰── Return
                        ╰── <37> Var [a]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_undeclared_var_in_ternary() {
    let src = r#"
        int main(void) {
            return a > 0 ? 1 : 2;
            int a = 5;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── Return
                    │   ╰── <13> Conditional [?]
                    │       ├── <10>  [>]
                    │       │   ├── <7> Var [a]
                    │       │   ╰── <9> Constant Int [0]
                    │       ├── Then
                    │       │   ╰── <11> Constant Int [1]
                    │       ╰── Else
                    │           ╰── <12> Constant Int [2]
                    ╰── VarDeclaration
                        ├── Name
                        │   ╰── a
                        ├── Type
                        │   ╰── Int
                        ╰── Initializer
                            ╰── <18> Constant Int [5]
    "#;
    assert_parse(src, expected);
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
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <9> Constant Int [0]
                    ├── <19> Assign [=]
                    │   ├── <13> Var [a]
                    │   ╰── <18> Conditional [?]
                    │       ├── <15> Constant Int [1]
                    │       ├── Then
                    │       │   ╰── <16> Constant Int [2]
                    │       ╰── Else
                    │           ╰── <17> Constant Int [3]
                    ╰── Return
                        ╰── <22> Var [a]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_binary_condition() {
    let src = r#"
        int main(void) {
            if (1 + 2 == 3)
                return 5;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── If
                        ├── Condition
                        │   ╰── <12>  [==]
                        │       ├── <9>  [+]
                        │       │   ├── <6> Constant Int [1]
                        │       │   ╰── <8> Constant Int [2]
                        │       ╰── <11> Constant Int [3]
                        ╰── Then
                            ╰── Return
                                ╰── <13> Constant Int [5]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_binary_false_condition() {
    let src = r#"
        int main(void) {
            if (1 + 2 == 4)
                return 5;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── If
                        ├── Condition
                        │   ╰── <12>  [==]
                        │       ├── <9>  [+]
                        │       │   ├── <6> Constant Int [1]
                        │       │   ╰── <8> Constant Int [2]
                        │       ╰── <11> Constant Int [4]
                        ╰── Then
                            ╰── Return
                                ╰── <13> Constant Int [5]
    "#;
    assert_parse(src, expected);
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
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <9> Constant Int [0]
                    ╰── If
                        ├── Condition
                        │   ╰── <13> Var [a]
                        ├── Then
                        │   ╰── Return
                        │       ╰── <14> Constant Int [1]
                        ╰── Else
                            ╰── Return
                                ╰── <16> Constant Int [2]
    "#;
    assert_parse(src, expected);
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
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── result
                    │   ╰── Type
                    │       ╰── Int
                    ├── <25> Conditional [?]
                    │   ├── <13>  [^]
                    │   │   ├── <10> Constant Int [1]
                    │   │   ╰── <12> Constant Int [1]
                    │   ├── Then
                    │   │   ╰── <18> Assign [=]
                    │   │       ├── <15> Var [result]
                    │   │       ╰── <17> Constant Int [4]
                    │   ╰── Else
                    │       ╰── <24> Assign [=]
                    │           ├── <20> Var [result]
                    │           ╰── <22> Constant Int [5]
                    ╰── Return
                        ╰── <28> Var [result]
    "#;
    assert_parse(src, expected);
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
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <9> Constant Int [4]
                    ├── <19> Assign [*=]
                    │   ├── <13> Var [a]
                    │   ╰── <18> Conditional [?]
                    │       ├── <15> Constant Int [1]
                    │       ├── Then
                    │       │   ╰── <16> Constant Int [2]
                    │       ╰── Else
                    │           ╰── <17> Constant Int [3]
                    ╰── Return
                        ╰── <22> Var [a]
    "#;
    assert_parse(src, expected);
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
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <9> Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <16> Assign [+=]
                    │   │       ├── <13> Var [a]
                    │   │       ╰── <15> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <18> Var [a]
                    ╰── Return
                        ╰── <21> Constant Int [10]
    "#;
    assert_parse(src, expected);
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
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <9> Constant Int [1]
                    ├── Goto [post_declaration]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <22> Assign [=]
                    │           ├── <18> Var [x]
                    │           ╰── <20> Constant Int [0]
                    ├── Label [post_declaration]
                    │   ╰── <30> Assign [=]
                    │       ├── <27> Var [i]
                    │       ╰── <29> Constant Int [5]
                    ╰── Return
                        ╰── <45>  [&&]
                            ├── <37>  [==]
                            │   ├── <34> Var [x]
                            │   ╰── <36> Constant Int [1]
                            ╰── <43>  [==]
                                ├── <40> Var [i]
                                ╰── <42> Constant Int [5]
    "#;
    assert_parse(src, expected);
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
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <6> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Label [label]
                    │           ╰── Return
                    │               ╰── <8> Constant Int [5]
                    ├── Goto [label]
                    ╰── Return
                        ╰── <14> Constant Int [0]
    "#;
    assert_parse(src, expected);
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
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── Goto [label]
                    ├── Return
                    │   ╰── <8> Constant Int [0]
                    ╰── Label [label]
                        ╰── Return
                            ╰── <11> Constant Int [1]
    "#;
    assert_parse(src, expected);
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
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ident
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <9> Constant Int [5]
                    ├── Goto [ident]
                    ├── Return
                    │   ╰── <14> Constant Int [0]
                    ╰── Label [ident]
                        ╰── Return
                            ╰── <18> Var [ident]
    "#;
    assert_parse(src, expected);
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
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── Goto [main]
                    ├── Return
                    │   ╰── <8> Constant Int [5]
                    ╰── Label [main]
                        ╰── Return
                            ╰── <11> Constant Int [0]
    "#;
    assert_parse(src, expected);
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
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── Goto [_main]
                    ├── Return
                    │   ╰── <8> Constant Int [0]
                    ╰── Label [_main]
                        ╰── Return
                            ╰── <11> Constant Int [1]
    "#;
    assert_parse(src, expected);
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
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── Goto [labelB]
                    ├── Label [labelA]
                    │   ╰── Label [labelB]
                    │       ╰── Return
                    │           ╰── <10> Constant Int [5]
                    ╰── Return
                        ╰── <14> Constant Int [0]
    "#;
    assert_parse(src, expected);
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
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <9> Constant Int [1]
                    ├── Label [label_if]
                    │   ╰── If
                    │       ├── Condition
                    │       │   ╰── <14> Var [a]
                    │       ├── Then
                    │       │   ╰── Goto [label_expression]
                    │       ╰── Else
                    │           ╰── Goto [label_empty]
                    ├── Label [label_goto]
                    │   ╰── Goto [label_return]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <25> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Label [label_expression]
                    │           ╰── <31> Assign [=]
                    │               ├── <28> Var [a]
                    │               ╰── <30> Constant Int [0]
                    ├── Goto [label_if]
                    ├── Label [label_return]
                    │   ╰── Return
                    │       ╰── <39> Var [a]
                    ├── Label [label_empty]
                    │   ╰── Empty
                    ├── <49> Assign [=]
                    │   ├── <46> Var [a]
                    │   ╰── <48> Constant Int [100]
                    ╰── Goto [label_goto]
    "#;
    assert_parse(src, expected);
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
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── Goto [_foo_1_]
                    ├── Return
                    │   ╰── <8> Constant Int [0]
                    ╰── Label [_foo_1_]
                        ╰── Return
                            ╰── <11> Constant Int [1]
    "#;
    assert_parse(src, expected);
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
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <9> Constant Int [10]
                    ├── <25> Conditional [?]
                    │   ├── <17> Assign [-=]
                    │   │   ├── <13> Var [x]
                    │   │   ╰── <15> Constant Int [1]
                    │   ├── Then
                    │   │   ╰── <23> Assign [/=]
                    │   │       ├── <19> Var [x]
                    │   │       ╰── <21> Constant Int [2]
                    │   ╰── Else
                    │       ╰── <24> Constant Int [0]
                    ╰── Return
                        ╰── <31>  [==]
                            ├── <28> Var [x]
                            ╰── <30> Constant Int [4]
    "#;
    assert_parse(src, expected);
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
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <9> Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <15> Postfix [--]
                    │   │       ╰── <13> Var [a]
                    │   ├── Then
                    │   │   ╰── Return
                    │   │       ╰── <16> Constant Int [0]
                    │   ╰── Else
                    │       ╰── If
                    │           ├── Condition
                    │           │   ╰── <21> Postfix [--]
                    │           │       ╰── <19> Var [a]
                    │           ╰── Then
                    │               ╰── Return
                    │                   ╰── <22> Constant Int [1]
                    ╰── Return
                        ╰── <26> Constant Int [0]
    "#;
    assert_parse(src, expected);
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
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <9> Constant Int [10]
                    ├── <22> Conditional [?]
                    │   ├── <16>  [-]
                    │   │   ├── <13> Var [x]
                    │   │   ╰── <15> Constant Int [10]
                    │   ├── Then
                    │   │   ╰── <17> Constant Int [0]
                    │   ╰── Else
                    │       ╰── <21> Postfix [--]
                    │           ╰── <19> Var [x]
                    ╰── Return
                        ╰── <25> Var [x]
    "#;
    assert_parse(src, expected);
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
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <11> Unary [-]
                    │           ╰── <10> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <17> Unary [++]
                    │   │       ╰── <16> Var [a]
                    │   ├── Then
                    │   │   ╰── Return
                    │   │       ╰── <18> Constant Int [0]
                    │   ╰── Else
                    │       ╰── If
                    │           ├── Condition
                    │           │   ╰── <23> Unary [++]
                    │           │       ╰── <22> Var [a]
                    │           ╰── Then
                    │               ╰── Return
                    │                   ╰── <24> Constant Int [1]
                    ╰── Return
                        ╰── <28> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_prefix_in_ternary() {
    let src = r#"
        int main(void) {
            int a = 0;
            return (++a ? ++a : 0);
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <9> Constant Int [0]
                    ╰── Return
                        ╰── <22> Conditional [?]
                            ├── <15> Unary [++]
                            │   ╰── <14> Var [a]
                            ├── Then
                            │   ╰── <19> Unary [++]
                            │       ╰── <18> Var [a]
                            ╰── Else
                                ╰── <20> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_unused_label() {
    let src = r#"
        int main(void) {
        unused:
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Label [unused]
                        ╰── Return
                            ╰── <7> Constant Int [0]
    "#;
    assert_parse(src, expected);
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
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── Goto [label2]
                    ├── Return
                    │   ╰── <8> Constant Int [0]
                    ╰── Label [label1]
                        ╰── Label [label2]
                            ╰── Return
                                ╰── <12> Constant Int [1]
    "#;
    assert_parse(src, expected);
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
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <9> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <19> Var [a]
                    │   ├── Then
                    │   │   ╰── <24> Assign [=]
                    │   │       ├── <21> Var [b]
                    │   │       ╰── <23> Constant Int [1]
                    │   ╰── Else
                    │       ╰── If
                    │           ├── Condition
                    │           │   ╰── <27> Var [b]
                    │           ╰── Then
                    │               ╰── <32> Assign [=]
                    │                   ├── <29> Var [b]
                    │                   ╰── <31> Constant Int [2]
                    ╰── Return
                        ╰── <37> Var [b]
    "#;
    assert_parse(src, expected);
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
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <9> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <19> Var [a]
                    │   ├── Then
                    │   │   ╰── <24> Assign [=]
                    │   │       ├── <21> Var [b]
                    │   │       ╰── <23> Constant Int [1]
                    │   ╰── Else
                    │       ╰── If
                    │           ├── Condition
                    │           │   ╰── <29> Unary [~]
                    │           │       ╰── <28> Var [b]
                    │           ╰── Then
                    │               ╰── <34> Assign [=]
                    │                   ├── <31> Var [b]
                    │                   ╰── <33> Constant Int [2]
                    ╰── Return
                        ╰── <39> Var [b]
    "#;
    assert_parse(src, expected);
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
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <9> Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <17> Assign [=]
                    │   │       ├── <13> Var [a]
                    │   │       ╰── <15> Constant Int [1]
                    │   ╰── Then
                    │       ╰── If
                    │           ├── Condition
                    │           │   ╰── <22>  [==]
                    │           │       ├── <19> Var [a]
                    │           │       ╰── <21> Constant Int [1]
                    │           ├── Then
                    │           │   ╰── <27> Assign [=]
                    │           │       ├── <24> Var [a]
                    │           │       ╰── <26> Constant Int [3]
                    │           ╰── Else
                    │               ╰── <33> Assign [=]
                    │                   ├── <30> Var [a]
                    │                   ╰── <32> Constant Int [4]
                    ╰── Return
                        ╰── <38> Var [a]
    "#;
    assert_parse(src, expected);
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
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <9> Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <15> Unary [!]
                    │   │       ╰── <14> Var [a]
                    │   ╰── Then
                    │       ╰── If
                    │           ├── Condition
                    │           │   ╰── <19>  [/]
                    │           │       ├── <16> Constant Int [3]
                    │           │       ╰── <18> Constant Int [4]
                    │           ├── Then
                    │           │   ╰── <24> Assign [=]
                    │           │       ├── <21> Var [a]
                    │           │       ╰── <23> Constant Int [3]
                    │           ╰── Else
                    │               ╰── <33> Assign [=]
                    │                   ├── <27> Var [a]
                    │                   ╰── <32>  [/]
                    │                       ├── <29> Constant Int [8]
                    │                       ╰── <31> Constant Int [2]
                    ╰── Return
                        ╰── <38> Var [a]
    "#;
    assert_parse(src, expected);
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
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <9> Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <12> Constant Int [0]
                    │   ├── Then
                    │   │   ╰── If
                    │   │       ├── Condition
                    │   │       │   ╰── <13> Constant Int [0]
                    │   │       ├── Then
                    │   │       │   ╰── <18> Assign [=]
                    │   │       │       ├── <15> Var [a]
                    │   │       │       ╰── <17> Constant Int [3]
                    │   │       ╰── Else
                    │   │           ╰── <24> Assign [=]
                    │   │               ├── <21> Var [a]
                    │   │               ╰── <23> Constant Int [4]
                    │   ╰── Else
                    │       ╰── <31> Assign [=]
                    │           ├── <28> Var [a]
                    │           ╰── <30> Constant Int [1]
                    ╰── Return
                        ╰── <35> Var [a]
    "#;
    assert_parse(src, expected);
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
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <9> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <19> Var [a]
                    │   ╰── Then
                    │       ╰── <24> Assign [=]
                    │           ├── <21> Var [b]
                    │           ╰── <23> Constant Int [1]
                    ╰── Return
                        ╰── <28> Var [b]
    "#;
    assert_parse(src, expected);
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
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <9> Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <12> Constant Int [0]
                    │   ├── Then
                    │   │   ╰── Empty
                    │   ╰── Else
                    │       ╰── <18> Assign [=]
                    │           ├── <15> Var [x]
                    │           ╰── <17> Constant Int [1]
                    ╰── Return
                        ╰── <22> Var [x]
    "#;
    assert_parse(src, expected);
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
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <9> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <19> Var [a]
                    │   ╰── Then
                    │       ╰── <24> Assign [=]
                    │           ├── <21> Var [b]
                    │           ╰── <23> Constant Int [1]
                    ╰── Return
                        ╰── <28> Var [b]
    "#;
    assert_parse(src, expected);
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
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <9> Constant Int [10]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── y
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [0]
                    ├── <31> Assign [=]
                    │   ├── <19> Var [y]
                    │   ╰── <30> Conditional [?]
                    │       ├── <26> Assign [=]
                    │       │   ├── <22> Var [x]
                    │       │   ╰── <24> Constant Int [5]
                    │       ├── Then
                    │       │   ╰── <28> Var [x]
                    │       ╰── Else
                    │           ╰── <29> Constant Int [2]
                    ╰── Return
                        ╰── <45>  [&&]
                            ├── <37>  [==]
                            │   ├── <34> Var [x]
                            │   ╰── <36> Constant Int [5]
                            ╰── <43>  [==]
                                ├── <40> Var [y]
                                ╰── <42> Constant Int [5]
    "#;
    assert_parse(src, expected);
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
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <9> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <19> Var [a]
                    │   ├── Then
                    │   │   ╰── <24> Assign [=]
                    │   │       ├── <21> Var [a]
                    │   │       ╰── <23> Constant Int [2]
                    │   ╰── Else
                    │       ╰── <30> Assign [=]
                    │           ├── <27> Var [a]
                    │           ╰── <29> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <34> Var [b]
                    │   ├── Then
                    │   │   ╰── <39> Assign [=]
                    │   │       ├── <36> Var [b]
                    │   │       ╰── <38> Constant Int [4]
                    │   ╰── Else
                    │       ╰── <45> Assign [=]
                    │           ├── <42> Var [b]
                    │           ╰── <44> Constant Int [5]
                    ╰── Return
                        ╰── <53>  [+]
                            ├── <49> Var [a]
                            ╰── <52> Var [b]
    "#;
    assert_parse(src, expected);
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
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <9> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── flag
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <21> Constant Int [0]
                    ╰── Return
                        ╰── <36> Conditional [?]
                            ├── <29>  [>]
                            │   ├── <25> Var [a]
                            │   ╰── <28> Var [b]
                            ├── Then
                            │   ╰── <30> Constant Int [5]
                            ╰── Else
                                ╰── <35> Conditional [?]
                                    ├── <32> Var [flag]
                                    ├── Then
                                    │   ╰── <33> Constant Int [6]
                                    ╰── Else
                                        ╰── <34> Constant Int [7]
    "#;
    assert_parse(src, expected);
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
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Conditional [?]
                    │           ├── <9> Constant Int [1]
                    │           ├── Then
                    │           │   ╰── <13> Conditional [?]
                    │           │       ├── <10> Constant Int [2]
                    │           │       ├── Then
                    │           │       │   ╰── <11> Constant Int [3]
                    │           │       ╰── Else
                    │           │           ╰── <12> Constant Int [4]
                    │           ╰── Else
                    │               ╰── <14> Constant Int [5]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <27> Conditional [?]
                    │           ├── <21> Constant Int [0]
                    │           ├── Then
                    │           │   ╰── <25> Conditional [?]
                    │           │       ├── <22> Constant Int [2]
                    │           │       ├── Then
                    │           │       │   ╰── <23> Constant Int [3]
                    │           │       ╰── Else
                    │           │           ╰── <24> Constant Int [4]
                    │           ╰── Else
                    │               ╰── <26> Constant Int [5]
                    ╰── Return
                        ╰── <35>  [*]
                            ├── <31> Var [a]
                            ╰── <34> Var [b]
    "#;
    assert_parse(src, expected);
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
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── flag
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <9> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [0]
                    ├── <31> Conditional [?]
                    │   ├── <19> Var [flag]
                    │   ├── Then
                    │   │   ╰── <24> Assign [=]
                    │   │       ├── <21> Var [a]
                    │   │       ╰── <23> Constant Int [1]
                    │   ╰── Else
                    │       ╰── <30> Assign [=]
                    │           ├── <26> Var [a]
                    │           ╰── <28> Constant Int [0]
                    ╰── Return
                        ╰── <34> Var [a]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_ternary() {
    let src = r#"
        int main(void) {
            int a = 0;
            return a > -1 ? 4 : 5;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <9> Constant Int [0]
                    ╰── Return
                        ╰── <21> Conditional [?]
                            ├── <18>  [>]
                            │   ├── <13> Var [a]
                            │   ╰── <17> Unary [-]
                            │       ╰── <16> Constant Int [1]
                            ├── Then
                            │   ╰── <19> Constant Int [4]
                            ╰── Else
                                ╰── <20> Constant Int [5]
    "#;
    assert_parse(src, expected);
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
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <9> Constant Int [1]
                    ├── <23> Conditional [?]
                    │   ├── <16>  [!=]
                    │   │   ├── <13> Var [a]
                    │   │   ╰── <15> Constant Int [2]
                    │   ├── Then
                    │   │   ╰── <21> Assign [=]
                    │   │       ├── <18> Var [a]
                    │   │       ╰── <20> Constant Int [2]
                    │   ╰── Else
                    │       ╰── <22> Constant Int [0]
                    ╰── Return
                        ╰── <26> Var [a]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_ternary_middle_binop() {
    let src = r#"
        int main(void) {
            int a = 1 ? 3 % 2 : 4;
            return a;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Conditional [?]
                    │           ├── <9> Constant Int [1]
                    │           ├── Then
                    │           │   ╰── <13>  [%]
                    │           │       ├── <10> Constant Int [3]
                    │           │       ╰── <12> Constant Int [2]
                    │           ╰── Else
                    │               ╰── <14> Constant Int [4]
                    ╰── Return
                        ╰── <19> Var [a]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_ternary_precedence() {
    let src = r#"
        int main(void) {
            int a = 10;
            return a || 0 ? 20 : 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <9> Constant Int [10]
                    ╰── Return
                        ╰── <19> Conditional [?]
                            ├── <16>  [||]
                            │   ├── <13> Var [a]
                            │   ╰── <15> Constant Int [0]
                            ├── Then
                            │   ╰── <17> Constant Int [20]
                            ╰── Else
                                ╰── <18> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_ternary_rh_binop() {
    let src = r#"
        int main(void) {
            return 0 ? 1 : 0 || 2;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <12> Conditional [?]
                            ├── <6> Constant Int [0]
                            ├── Then
                            │   ╰── <7> Constant Int [1]
                            ╰── Else
                                ╰── <11>  [||]
                                    ├── <8> Constant Int [0]
                                    ╰── <10> Constant Int [2]
    "#;
    assert_parse(src, expected);
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
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <9> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [0]
                    ├── <32> Conditional [?]
                    │   ├── <19> Var [a]
                    │   ├── Then
                    │   │   ╰── <25> Assign [=]
                    │   │       ├── <21> Var [b]
                    │   │       ╰── <23> Constant Int [1]
                    │   ╰── Else
                    │       ╰── <31> Assign [=]
                    │           ├── <27> Var [b]
                    │           ╰── <29> Constant Int [2]
                    ╰── Return
                        ╰── <35> Var [b]
    "#;
    assert_parse(src, expected);
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
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <9> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [0]
                    ├── <32> Conditional [?]
                    │   ├── <19> Var [a]
                    │   ├── Then
                    │   │   ╰── <25> Assign [=]
                    │   │       ├── <21> Var [b]
                    │   │       ╰── <23> Constant Int [1]
                    │   ╰── Else
                    │       ╰── <31> Assign [=]
                    │           ├── <27> Var [b]
                    │           ╰── <29> Constant Int [2]
                    ╰── Return
                        ╰── <35> Var [b]
    "#;
    assert_parse(src, expected);
}
