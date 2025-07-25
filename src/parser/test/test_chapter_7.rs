use super::{assert_error, assert_parse};

#[test]
fn test_invalid_parse_extra_brace() {
    assert_error(
        r#"
        int main(void) {
            if(0){
                return 1;
            }}
            return 2;
          //^^^^^^ Expected type specifier
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_missing_brace() {
    assert_error(
        r#"
        int main(void) {
            if(0){
                return 1;
            return 2;
        }
    
// Expected statement, but found ''"#,
    );
}

#[test]
fn test_invalid_parse_missing_semicolon() {
    assert_error(
        r#"
        int main(void) {
            int a = 4;
            {
                a = 5;
                return a
            }
          //^ Expected ';', but found '}'
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_ternary_blocks() {
    assert_error(
        r#"
        int main(void) {
            int a;
            return 1 ? { a = 2 } : a = 4;
                     //^ Expected expression, but found '{'
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_double_define() {
    let src = r#"
        int main(void) {
            {
                int a;
                int a;
            }
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Block
                        ├── VarDeclaration
                        │   ├── Name
                        │   │   ╰── a
                        │   ╰── Type
                        │       ╰── Int
                        ╰── VarDeclaration
                            ├── Name
                            │   ╰── a
                            ╰── Type
                                ╰── Int
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_double_define_after_scope() {
    let src = r#"
        int main(void) {
            int a = 3;
            {
                a = 5;
            }
            int a = 2;
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
                    │       ╰── Constant Int [3]
                    ├── Block
                    │   ╰── <15> Assign [=]
                    │       ├── <12> Var [a]
                    │       ╰── Constant Int [5]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [2]
                    ╰── Return
                        ╰── <26> Var [a]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_extra_credit_different_labels_same_scope() {
    let src = r#"
        int main(void) {
        label1:;
            int a = 10;
        label2:;
            int a = 11;
            return 1;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── Label [label1]
                    │   ╰── Empty
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [10]
                    ├── Label [label2]
                    │   ╰── Empty
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [11]
                    ╰── Return
                        ╰── Constant Int [1]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_extra_credit_duplicate_labels_different_scopes() {
    let src = r#"
        int main(void) {
            int x = 0;
            if (x) {
                x = 5;
                goto l;
                return 0;
                l:
                    return x;
            } else {
                goto l;
                return 0;
                l:
                    return x;
            }
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
                    │       ╰── Constant Int [0]
                    ╰── If
                        ├── Condition
                        │   ╰── <12> Var [x]
                        ├── Then
                        │   ╰── Block
                        │       ├── <17> Assign [=]
                        │       │   ├── <14> Var [x]
                        │       │   ╰── Constant Int [5]
                        │       ├── Goto [l]
                        │       ├── Return
                        │       │   ╰── Constant Int [0]
                        │       ╰── Label [l]
                        │           ╰── Return
                        │               ╰── <25> Var [x]
                        ╰── Else
                            ╰── Block
                                ├── Goto [l]
                                ├── Return
                                │   ╰── Constant Int [0]
                                ╰── Label [l]
                                    ╰── Return
                                        ╰── <36> Var [x]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_extra_credit_goto_use_before_declare() {
    let src = r#"
        int main(void) {
            int x = 0;
            if (x != 0) {
                return_y:
                return y;
            }
            int y = 4;
            goto return_y;
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
                    │       ╰── Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <15>  [!=]
                    │   │       ├── <12> Var [x]
                    │   │       ╰── Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Label [return_y]
                    │               ╰── Return
                    │                   ╰── <18> Var [y]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── y
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [4]
                    ╰── Goto [return_y]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_out_of_scope() {
    let src = r#"
        int main(void) {
            {
                int a = 2;
            }
            return a;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── Block
                    │   ╰── VarDeclaration
                    │       ├── Name
                    │       │   ╰── a
                    │       ├── Type
                    │       │   ╰── Int
                    │       ╰── Initializer
                    │           ╰── Constant Int [2]
                    ╰── Return
                        ╰── <14> Var [a]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_use_before_declare() {
    let src = r#"
        int main(void) {
            int a;
            {
                b = 10;
            }
            int b;
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
                    │   ╰── Type
                    │       ╰── Int
                    ├── Block
                    │   ╰── <13> Assign [=]
                    │       ├── <10> Var [b]
                    │       ╰── Constant Int [10]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ╰── Type
                    │       ╰── Int
                    ╰── Return
                        ╰── <22> Var [b]
    "#;
    assert_parse(src, expected);
}

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
                    │       ╰── Constant Int [3]
                    ╰── Block
                        ├── VarDeclaration
                        │   ├── Name
                        │   │   ╰── a
                        │   ├── Type
                        │   │   ╰── Int
                        │   ╰── Initializer
                        │       ╰── <18> Assign [=]
                        │           ├── <15> Var [a]
                        │           ╰── Constant Int [4]
                        ╰── Return
                            ╰── <22> Var [a]
    "#;
    assert_parse(src, expected);
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
                    │       ╰── Constant Int [3]
                    ├── Block
                    │   ╰── VarDeclaration
                    │       ├── Name
                    │       │   ╰── a
                    │       ├── Type
                    │       │   ╰── Int
                    │       ╰── Initializer
                    │           ╰── <18> Assign [=]
                    │               ├── <15> Var [a]
                    │               ╰── Constant Int [4]
                    ╰── Return
                        ╰── <24> Var [a]
    "#;
    assert_parse(src, expected);
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
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ╰── Type
                    │       ╰── Int
                    ├── Block
                    │   ╰── VarDeclaration
                    │       ├── Name
                    │       │   ╰── b
                    │       ├── Type
                    │       │   ╰── Int
                    │       ╰── Initializer
                    │           ╰── <16> Assign [=]
                    │               ├── <13> Var [a]
                    │               ╰── Constant Int [1]
                    ╰── Return
                        ╰── <22> Var [a]
    "#;
    assert_parse(src, expected);
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
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ten
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [10]
                    ├── Block
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── twenty
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <19>  [*]
                    │           ├── Constant Int [10]
                    │           ╰── Constant Int [2]
                    ├── Block
                    │   ╰── Block
                    ╰── Return
                        ╰── <31>  [+]
                            ├── <27> Var [ten]
                            ╰── <30> Var [twenty]
    "#;
    assert_parse(src, expected);
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
                    │       ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <15>  [>]
                    │   │       ├── <12> Var [a]
                    │   │       ╰── Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ├── <20> Assign [-=]
                    │           │   ├── <17> Var [a]
                    │           │   ╰── Constant Int [4]
                    │           ├── VarDeclaration
                    │           │   ├── Name
                    │           │   │   ╰── a
                    │           │   ├── Type
                    │           │   │   ╰── Int
                    │           │   ╰── Initializer
                    │           │       ╰── Constant Int [5]
                    │           ╰── If
                    │               ├── Condition
                    │               │   ╰── <32>  [>]
                    │               │       ├── <29> Var [a]
                    │               │       ╰── Constant Int [4]
                    │               ╰── Then
                    │                   ╰── Block
                    │                       ╰── <37> Assign [-=]
                    │                           ├── <34> Var [a]
                    │                           ╰── Constant Int [4]
                    ╰── Return
                        ╰── <46> Var [a]
    "#;
    assert_parse(src, expected);
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
                    │       ╰── Constant Int [0]
                    ╰── Block
                        ├── If
                        │   ├── Condition
                        │   │   ╰── <15>  [!=]
                        │   │       ├── <12> Var [a]
                        │   │       ╰── Constant Int [0]
                        │   ╰── Then
                        │       ╰── Label [return_a]
                        │           ╰── Return
                        │               ╰── <18> Var [a]
                        ├── VarDeclaration
                        │   ├── Name
                        │   │   ╰── a
                        │   ├── Type
                        │   │   ╰── Int
                        │   ╰── Initializer
                        │       ╰── Constant Int [4]
                        ╰── Goto [return_a]
    "#;
    assert_parse(src, expected);
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
                    │       ╰── Constant Int [5]
                    ├── Goto [inner]
                    ╰── Block
                        ├── VarDeclaration
                        │   ├── Name
                        │   │   ╰── x
                        │   ├── Type
                        │   │   ╰── Int
                        │   ╰── Initializer
                        │       ╰── Constant Int [0]
                        ├── Label [inner]
                        │   ╰── <24> Assign [=]
                        │       ├── <21> Var [x]
                        │       ╰── Constant Int [1]
                        ╰── Return
                            ╰── <28> Var [x]
    "#;
    assert_parse(src, expected);
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
                    │       ╰── Constant Int [10]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <18> Var [a]
                    │   ╰── Then
                    │       ╰── Block
                    │           ├── VarDeclaration
                    │           │   ├── Name
                    │           │   │   ╰── a
                    │           │   ├── Type
                    │           │   │   ╰── Int
                    │           │   ╰── Initializer
                    │           │       ╰── Constant Int [1]
                    │           ├── <30> Assign [=]
                    │           │   ├── <26> Var [b]
                    │           │   ╰── <29> Var [a]
                    │           ╰── Goto [end]
                    ├── <41> Assign [=]
                    │   ├── <38> Var [a]
                    │   ╰── Constant Int [9]
                    ╰── Label [end]
                        ╰── Return
                            ╰── <56>  [&&]
                                ├── <48>  [==]
                                │   ├── <45> Var [a]
                                │   ╰── Constant Int [10]
                                ╰── <54>  [==]
                                    ├── <51> Var [b]
                                    ╰── Constant Int [1]
    "#;
    assert_parse(src, expected);
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
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── sum
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ├── VarDeclaration
                    │           │   ├── Name
                    │           │   │   ╰── a
                    │           │   ├── Type
                    │           │   │   ╰── Int
                    │           │   ╰── Initializer
                    │           │       ╰── Constant Int [5]
                    │           ├── Goto [other_if]
                    │           ├── <24> Assign [=]
                    │           │   ├── <21> Var [sum]
                    │           │   ╰── Constant Int [0]
                    │           ├── Label [first_if]
                    │           │   ╰── <31> Assign [=]
                    │           │       ├── <28> Var [a]
                    │           │       ╰── Constant Int [5]
                    │           ╰── <43> Assign [=]
                    │               ├── <35> Var [sum]
                    │               ╰── <42>  [+]
                    │                   ├── <38> Var [sum]
                    │                   ╰── <41> Var [a]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ├── Label [other_if]
                    │           │   ╰── Empty
                    │           ├── VarDeclaration
                    │           │   ├── Name
                    │           │   │   ╰── a
                    │           │   ├── Type
                    │           │   │   ╰── Int
                    │           │   ╰── Initializer
                    │           │       ╰── Constant Int [6]
                    │           ├── <67> Assign [=]
                    │           │   ├── <59> Var [sum]
                    │           │   ╰── <66>  [+]
                    │           │       ├── <62> Var [sum]
                    │           │       ╰── <65> Var [a]
                    │           ├── Goto [first_if]
                    │           ╰── <75> Assign [=]
                    │               ├── <72> Var [sum]
                    │               ╰── Constant Int [0]
                    ╰── Return
                        ╰── <81> Var [sum]
    "#;
    assert_parse(src, expected);
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
                    │       ╰── Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ╰── Type
                    │       ╰── Int
                    ├── Block
                    │   ├── <21> Assign [=]
                    │   │   ├── <16> Var [a]
                    │   │   ╰── <20> Unary [-]
                    │   │       ╰── Constant Int [4]
                    │   ├── VarDeclaration
                    │   │   ├── Name
                    │   │   │   ╰── a
                    │   │   ├── Type
                    │   │   │   ╰── Int
                    │   │   ╰── Initializer
                    │   │       ╰── Constant Int [7]
                    │   ╰── <37> Assign [=]
                    │       ├── <30> Var [b]
                    │       ╰── <36>  [+]
                    │           ├── <33> Var [a]
                    │           ╰── Constant Int [1]
                    ╰── Return
                        ╰── <54>  [&&]
                            ├── <45>  [==]
                            │   ├── <42> Var [b]
                            │   ╰── Constant Int [8]
                            ╰── <53>  [==]
                                ├── <48> Var [a]
                                ╰── <52> Unary [-]
                                    ╰── Constant Int [4]
    "#;
    assert_parse(src, expected);
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
                    │       ╰── Constant Int [2]
                    ╰── Block
                        ├── VarDeclaration
                        │   ├── Name
                        │   │   ╰── a
                        │   ├── Type
                        │   │   ╰── Int
                        │   ╰── Initializer
                        │       ╰── Constant Int [1]
                        ╰── Return
                            ╰── <18> Var [a]
    "#;
    assert_parse(src, expected);
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
                    │       ╰── Constant Int [4]
                    ├── Block
                    │   ╰── VarDeclaration
                    │       ├── Name
                    │       │   ╰── x
                    │       ╰── Type
                    │           ╰── Int
                    ╰── Return
                        ╰── <18> Var [x]
    "#;
    assert_parse(src, expected);
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
                    │       ╰── Constant Int [0]
                    ├── Block
                    │   ├── VarDeclaration
                    │   │   ├── Name
                    │   │   │   ╰── b
                    │   │   ├── Type
                    │   │   │   ╰── Int
                    │   │   ╰── Initializer
                    │   │       ╰── Constant Int [4]
                    │   ╰── <22> Assign [=]
                    │       ├── <18> Var [a]
                    │       ╰── <21> Var [b]
                    ├── Block
                    │   ├── VarDeclaration
                    │   │   ├── Name
                    │   │   │   ╰── b
                    │   │   ├── Type
                    │   │   │   ╰── Int
                    │   │   ╰── Initializer
                    │   │       ╰── Constant Int [2]
                    │   ╰── <41> Assign [=]
                    │       ├── <33> Var [a]
                    │       ╰── <40>  [-]
                    │           ├── <36> Var [a]
                    │           ╰── <39> Var [b]
                    ╰── Return
                        ╰── <46> Var [a]
    "#;
    assert_parse(src, expected);
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
                    │       ╰── Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <12> Var [a]
                    │   ├── Then
                    │   │   ╰── Block
                    │   │       ├── VarDeclaration
                    │   │       │   ├── Name
                    │   │       │   │   ╰── b
                    │   │       │   ├── Type
                    │   │       │   │   ╰── Int
                    │   │       │   ╰── Initializer
                    │   │       │       ╰── Constant Int [2]
                    │   │       ╰── Return
                    │   │           ╰── <20> Var [b]
                    │   ╰── Else
                    │       ╰── Block
                    │           ├── VarDeclaration
                    │           │   ├── Name
                    │           │   │   ╰── c
                    │           │   ├── Type
                    │           │   │   ╰── Int
                    │           │   ╰── Initializer
                    │           │       ╰── Constant Int [3]
                    │           ╰── If
                    │               ├── Condition
                    │               │   ╰── <35>  [<]
                    │               │       ├── <31> Var [a]
                    │               │       ╰── <34> Var [c]
                    │               ├── Then
                    │               │   ╰── Block
                    │               │       ╰── Return
                    │               │           ╰── <39> Unary [!]
                    │               │               ╰── <38> Var [a]
                    │               ╰── Else
                    │                   ╰── Block
                    │                       ╰── Return
                    │                           ╰── Constant Int [5]
                    ╰── Return
                        ╰── <52> Var [a]
    "#;
    assert_parse(src, expected);
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
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ╰── Type
                    │       ╰── Int
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── result
                    │   ╰── Type
                    │       ╰── Int
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a1
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [1]
                    ├── Block
                    │   ├── VarDeclaration
                    │   │   ├── Name
                    │   │   │   ╰── a
                    │   │   ├── Type
                    │   │   │   ╰── Int
                    │   │   ╰── Initializer
                    │   │       ╰── Constant Int [2]
                    │   ├── VarDeclaration
                    │   │   ├── Name
                    │   │   │   ╰── a1
                    │   │   ├── Type
                    │   │   │   ╰── Int
                    │   │   ╰── Initializer
                    │   │       ╰── Constant Int [2]
                    │   ├── Block
                    │   │   ├── VarDeclaration
                    │   │   │   ├── Name
                    │   │   │   │   ╰── a
                    │   │   │   ╰── Type
                    │   │   │       ╰── Int
                    │   │   ╰── Block
                    │   │       ├── VarDeclaration
                    │   │       │   ├── Name
                    │   │       │   │   ╰── a
                    │   │       │   ╰── Type
                    │   │       │       ╰── Int
                    │   │       ╰── Block
                    │   │           ├── VarDeclaration
                    │   │           │   ├── Name
                    │   │           │   │   ╰── a
                    │   │           │   ╰── Type
                    │   │           │       ╰── Int
                    │   │           ╰── Block
                    │   │               ├── VarDeclaration
                    │   │               │   ├── Name
                    │   │               │   │   ╰── a
                    │   │               │   ╰── Type
                    │   │               │       ╰── Int
                    │   │               ╰── Block
                    │   │                   ├── VarDeclaration
                    │   │                   │   ├── Name
                    │   │                   │   │   ╰── a
                    │   │                   │   ╰── Type
                    │   │                   │       ╰── Int
                    │   │                   ╰── Block
                    │   │                       ├── VarDeclaration
                    │   │                       │   ├── Name
                    │   │                       │   │   ╰── a
                    │   │                       │   ╰── Type
                    │   │                       │       ╰── Int
                    │   │                       ╰── Block
                    │   │                           ├── VarDeclaration
                    │   │                           │   ├── Name
                    │   │                           │   │   ╰── a
                    │   │                           │   ╰── Type
                    │   │                           │       ╰── Int
                    │   │                           ╰── Block
                    │   │                               ├── VarDeclaration
                    │   │                               │   ├── Name
                    │   │                               │   │   ╰── a
                    │   │                               │   ╰── Type
                    │   │                               │       ╰── Int
                    │   │                               ╰── Block
                    │   │                                   ├── VarDeclaration
                    │   │                                   │   ├── Name
                    │   │                                   │   │   ╰── a
                    │   │                                   │   ├── Type
                    │   │                                   │   │   ╰── Int
                    │   │                                   │   ╰── Initializer
                    │   │                                   │       ╰── Constant Int [20]
                    │   │                                   ├── <74> Assign [=]
                    │   │                                   │   ├── <70> Var [result]
                    │   │                                   │   ╰── <73> Var [a]
                    │   │                                   ╰── Block
                    │   │                                       ├── VarDeclaration
                    │   │                                       │   ├── Name
                    │   │                                       │   │   ╰── a
                    │   │                                       │   ╰── Type
                    │   │                                       │       ╰── Int
                    │   │                                       ├── <84> Assign [=]
                    │   │                                       │   ├── <81> Var [a]
                    │   │                                       │   ╰── Constant Int [5]
                    │   │                                       ╰── <95> Assign [=]
                    │   │                                           ├── <87> Var [result]
                    │   │                                           ╰── <94>  [+]
                    │   │                                               ├── <90> Var [result]
                    │   │                                               ╰── <93> Var [a]
                    │   ╰── <126> Assign [=]
                    │       ├── <118> Var [result]
                    │       ╰── <125>  [+]
                    │           ├── <121> Var [result]
                    │           ╰── <124> Var [a1]
                    ╰── Return
                        ╰── <135>  [+]
                            ├── <131> Var [result]
                            ╰── <134> Var [a1]
    "#;
    assert_parse(src, expected);
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
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ╰── Type
                    │       ╰── Int
                    ├── Block
                    │   ╰── <13> Assign [=]
                    │       ├── <10> Var [x]
                    │       ╰── Constant Int [3]
                    ╰── Block
                        ╰── Return
                            ╰── <18> Var [x]
    "#;
    assert_parse(src, expected);
}
