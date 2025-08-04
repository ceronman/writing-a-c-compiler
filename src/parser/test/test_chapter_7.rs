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
                    │       ╰── <9> Constant Int [3]
                    ├── Block
                    │   ╰── <16> Assign [=]
                    │       ├── <13> Var [a]
                    │       ╰── <15> Constant Int [5]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <23> Constant Int [2]
                    ╰── Return
                        ╰── <27> Var [a]
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
                    │       ╰── <12> Constant Int [10]
                    ├── Label [label2]
                    │   ╰── Empty
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <21> Constant Int [11]
                    ╰── Return
                        ╰── <24> Constant Int [1]
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
                    │       ╰── <9> Constant Int [0]
                    ╰── If
                        ├── Condition
                        │   ╰── <13> Var [x]
                        ├── Then
                        │   ╰── Block
                        │       ├── <18> Assign [=]
                        │       │   ├── <15> Var [x]
                        │       │   ╰── <17> Constant Int [5]
                        │       ├── Goto [l]
                        │       ├── Return
                        │       │   ╰── <22> Constant Int [0]
                        │       ╰── Label [l]
                        │           ╰── Return
                        │               ╰── <26> Var [x]
                        ╰── Else
                            ╰── Block
                                ├── Goto [l]
                                ├── Return
                                │   ╰── <33> Constant Int [0]
                                ╰── Label [l]
                                    ╰── Return
                                        ╰── <37> Var [x]
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
                    │       ╰── <9> Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <16>  [!=]
                    │   │       ├── <13> Var [x]
                    │   │       ╰── <15> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Label [return_y]
                    │               ╰── Return
                    │                   ╰── <19> Var [y]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── y
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <28> Constant Int [4]
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
                    │           ╰── <9> Constant Int [2]
                    ╰── Return
                        ╰── <15> Var [a]
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
                    │   ╰── <14> Assign [=]
                    │       ├── <11> Var [b]
                    │       ╰── <13> Constant Int [10]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ╰── Type
                    │       ╰── Int
                    ╰── Return
                        ╰── <23> Var [b]
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
                    │       ╰── <9> Constant Int [3]
                    ╰── Block
                        ├── VarDeclaration
                        │   ├── Name
                        │   │   ╰── a
                        │   ├── Type
                        │   │   ╰── Int
                        │   ╰── Initializer
                        │       ╰── <19> Assign [=]
                        │           ├── <16> Var [a]
                        │           ╰── <18> Constant Int [4]
                        ╰── Return
                            ╰── <23> Var [a]
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
                    │       ╰── <9> Constant Int [3]
                    ├── Block
                    │   ╰── VarDeclaration
                    │       ├── Name
                    │       │   ╰── a
                    │       ├── Type
                    │       │   ╰── Int
                    │       ╰── Initializer
                    │           ╰── <19> Assign [=]
                    │               ├── <16> Var [a]
                    │               ╰── <18> Constant Int [4]
                    ╰── Return
                        ╰── <25> Var [a]
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
                    │           ╰── <17> Assign [=]
                    │               ├── <14> Var [a]
                    │               ╰── <16> Constant Int [1]
                    ╰── Return
                        ╰── <23> Var [a]
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
                    │       ╰── <9> Constant Int [10]
                    ├── Block
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── twenty
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <20>  [*]
                    │           ├── <17> Constant Int [10]
                    │           ╰── <19> Constant Int [2]
                    ├── Block
                    │   ╰── Block
                    ╰── Return
                        ╰── <32>  [+]
                            ├── <28> Var [ten]
                            ╰── <31> Var [twenty]
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
                    │       ╰── <9> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <16>  [>]
                    │   │       ├── <13> Var [a]
                    │   │       ╰── <15> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ├── <21> Assign [-=]
                    │           │   ├── <18> Var [a]
                    │           │   ╰── <20> Constant Int [4]
                    │           ├── VarDeclaration
                    │           │   ├── Name
                    │           │   │   ╰── a
                    │           │   ├── Type
                    │           │   │   ╰── Int
                    │           │   ╰── Initializer
                    │           │       ╰── <26> Constant Int [5]
                    │           ╰── If
                    │               ├── Condition
                    │               │   ╰── <33>  [>]
                    │               │       ├── <30> Var [a]
                    │               │       ╰── <32> Constant Int [4]
                    │               ╰── Then
                    │                   ╰── Block
                    │                       ╰── <38> Assign [-=]
                    │                           ├── <35> Var [a]
                    │                           ╰── <37> Constant Int [4]
                    ╰── Return
                        ╰── <47> Var [a]
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
                    │       ╰── <9> Constant Int [0]
                    ╰── Block
                        ├── If
                        │   ├── Condition
                        │   │   ╰── <16>  [!=]
                        │   │       ├── <13> Var [a]
                        │   │       ╰── <15> Constant Int [0]
                        │   ╰── Then
                        │       ╰── Label [return_a]
                        │           ╰── Return
                        │               ╰── <19> Var [a]
                        ├── VarDeclaration
                        │   ├── Name
                        │   │   ╰── a
                        │   ├── Type
                        │   │   ╰── Int
                        │   ╰── Initializer
                        │       ╰── <26> Constant Int [4]
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
                    │       ╰── <9> Constant Int [5]
                    ├── Goto [inner]
                    ╰── Block
                        ├── VarDeclaration
                        │   ├── Name
                        │   │   ╰── x
                        │   ├── Type
                        │   │   ╰── Int
                        │   ╰── Initializer
                        │       ╰── <17> Constant Int [0]
                        ├── Label [inner]
                        │   ╰── <25> Assign [=]
                        │       ├── <22> Var [x]
                        │       ╰── <24> Constant Int [1]
                        ╰── Return
                            ╰── <29> Var [x]
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
                    │       ╰── <9> Constant Int [10]
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
                    │       ╰── Block
                    │           ├── VarDeclaration
                    │           │   ├── Name
                    │           │   │   ╰── a
                    │           │   ├── Type
                    │           │   │   ╰── Int
                    │           │   ╰── Initializer
                    │           │       ╰── <23> Constant Int [1]
                    │           ├── <31> Assign [=]
                    │           │   ├── <27> Var [b]
                    │           │   ╰── <30> Var [a]
                    │           ╰── Goto [end]
                    ├── <42> Assign [=]
                    │   ├── <39> Var [a]
                    │   ╰── <41> Constant Int [9]
                    ╰── Label [end]
                        ╰── Return
                            ╰── <57>  [&&]
                                ├── <49>  [==]
                                │   ├── <46> Var [a]
                                │   ╰── <48> Constant Int [10]
                                ╰── <55>  [==]
                                    ├── <52> Var [b]
                                    ╰── <54> Constant Int [1]
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
                    │       ╰── <9> Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <12> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ├── VarDeclaration
                    │           │   ├── Name
                    │           │   │   ╰── a
                    │           │   ├── Type
                    │           │   │   ╰── Int
                    │           │   ╰── Initializer
                    │           │       ╰── <16> Constant Int [5]
                    │           ├── Goto [other_if]
                    │           ├── <25> Assign [=]
                    │           │   ├── <22> Var [sum]
                    │           │   ╰── <24> Constant Int [0]
                    │           ├── Label [first_if]
                    │           │   ╰── <32> Assign [=]
                    │           │       ├── <29> Var [a]
                    │           │       ╰── <31> Constant Int [5]
                    │           ╰── <44> Assign [=]
                    │               ├── <36> Var [sum]
                    │               ╰── <43>  [+]
                    │                   ├── <39> Var [sum]
                    │                   ╰── <42> Var [a]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <49> Constant Int [0]
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
                    │           │       ╰── <56> Constant Int [6]
                    │           ├── <68> Assign [=]
                    │           │   ├── <60> Var [sum]
                    │           │   ╰── <67>  [+]
                    │           │       ├── <63> Var [sum]
                    │           │       ╰── <66> Var [a]
                    │           ├── Goto [first_if]
                    │           ╰── <76> Assign [=]
                    │               ├── <73> Var [sum]
                    │               ╰── <75> Constant Int [0]
                    ╰── Return
                        ╰── <82> Var [sum]
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
                    │       ╰── <9> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ╰── Type
                    │       ╰── Int
                    ├── Block
                    │   ├── <22> Assign [=]
                    │   │   ├── <17> Var [a]
                    │   │   ╰── <21> Unary [-]
                    │   │       ╰── <20> Constant Int [4]
                    │   ├── VarDeclaration
                    │   │   ├── Name
                    │   │   │   ╰── a
                    │   │   ├── Type
                    │   │   │   ╰── Int
                    │   │   ╰── Initializer
                    │   │       ╰── <27> Constant Int [7]
                    │   ╰── <38> Assign [=]
                    │       ├── <31> Var [b]
                    │       ╰── <37>  [+]
                    │           ├── <34> Var [a]
                    │           ╰── <36> Constant Int [1]
                    ╰── Return
                        ╰── <55>  [&&]
                            ├── <46>  [==]
                            │   ├── <43> Var [b]
                            │   ╰── <45> Constant Int [8]
                            ╰── <54>  [==]
                                ├── <49> Var [a]
                                ╰── <53> Unary [-]
                                    ╰── <52> Constant Int [4]
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
                    │       ╰── <9> Constant Int [2]
                    ╰── Block
                        ├── VarDeclaration
                        │   ├── Name
                        │   │   ╰── a
                        │   ├── Type
                        │   │   ╰── Int
                        │   ╰── Initializer
                        │       ╰── <15> Constant Int [1]
                        ╰── Return
                            ╰── <19> Var [a]
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
                    │       ╰── <9> Constant Int [4]
                    ├── Block
                    │   ╰── VarDeclaration
                    │       ├── Name
                    │       │   ╰── x
                    │       ╰── Type
                    │           ╰── Int
                    ╰── Return
                        ╰── <19> Var [x]
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
                    │       ╰── <9> Constant Int [0]
                    ├── Block
                    │   ├── VarDeclaration
                    │   │   ├── Name
                    │   │   │   ╰── b
                    │   │   ├── Type
                    │   │   │   ╰── Int
                    │   │   ╰── Initializer
                    │   │       ╰── <15> Constant Int [4]
                    │   ╰── <23> Assign [=]
                    │       ├── <19> Var [a]
                    │       ╰── <22> Var [b]
                    ├── Block
                    │   ├── VarDeclaration
                    │   │   ├── Name
                    │   │   │   ╰── b
                    │   │   ├── Type
                    │   │   │   ╰── Int
                    │   │   ╰── Initializer
                    │   │       ╰── <30> Constant Int [2]
                    │   ╰── <42> Assign [=]
                    │       ├── <34> Var [a]
                    │       ╰── <41>  [-]
                    │           ├── <37> Var [a]
                    │           ╰── <40> Var [b]
                    ╰── Return
                        ╰── <47> Var [a]
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
                    │       ╰── <9> Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <13> Var [a]
                    │   ├── Then
                    │   │   ╰── Block
                    │   │       ├── VarDeclaration
                    │   │       │   ├── Name
                    │   │       │   │   ╰── b
                    │   │       │   ├── Type
                    │   │       │   │   ╰── Int
                    │   │       │   ╰── Initializer
                    │   │       │       ╰── <17> Constant Int [2]
                    │   │       ╰── Return
                    │   │           ╰── <21> Var [b]
                    │   ╰── Else
                    │       ╰── Block
                    │           ├── VarDeclaration
                    │           │   ├── Name
                    │           │   │   ╰── c
                    │           │   ├── Type
                    │           │   │   ╰── Int
                    │           │   ╰── Initializer
                    │           │       ╰── <28> Constant Int [3]
                    │           ╰── If
                    │               ├── Condition
                    │               │   ╰── <36>  [<]
                    │               │       ├── <32> Var [a]
                    │               │       ╰── <35> Var [c]
                    │               ├── Then
                    │               │   ╰── Block
                    │               │       ╰── Return
                    │               │           ╰── <40> Unary [!]
                    │               │               ╰── <39> Var [a]
                    │               ╰── Else
                    │                   ╰── Block
                    │                       ╰── Return
                    │                           ╰── <44> Constant Int [5]
                    ╰── Return
                        ╰── <53> Var [a]
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
                    │       ╰── <17> Constant Int [1]
                    ├── Block
                    │   ├── VarDeclaration
                    │   │   ├── Name
                    │   │   │   ╰── a
                    │   │   ├── Type
                    │   │   │   ╰── Int
                    │   │   ╰── Initializer
                    │   │       ╰── <23> Constant Int [2]
                    │   ├── VarDeclaration
                    │   │   ├── Name
                    │   │   │   ╰── a1
                    │   │   ├── Type
                    │   │   │   ╰── Int
                    │   │   ╰── Initializer
                    │   │       ╰── <29> Constant Int [2]
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
                    │   │                                   │       ╰── <67> Constant Int [20]
                    │   │                                   ├── <75> Assign [=]
                    │   │                                   │   ├── <71> Var [result]
                    │   │                                   │   ╰── <74> Var [a]
                    │   │                                   ╰── Block
                    │   │                                       ├── VarDeclaration
                    │   │                                       │   ├── Name
                    │   │                                       │   │   ╰── a
                    │   │                                       │   ╰── Type
                    │   │                                       │       ╰── Int
                    │   │                                       ├── <85> Assign [=]
                    │   │                                       │   ├── <82> Var [a]
                    │   │                                       │   ╰── <84> Constant Int [5]
                    │   │                                       ╰── <96> Assign [=]
                    │   │                                           ├── <88> Var [result]
                    │   │                                           ╰── <95>  [+]
                    │   │                                               ├── <91> Var [result]
                    │   │                                               ╰── <94> Var [a]
                    │   ╰── <127> Assign [=]
                    │       ├── <119> Var [result]
                    │       ╰── <126>  [+]
                    │           ├── <122> Var [result]
                    │           ╰── <125> Var [a1]
                    ╰── Return
                        ╰── <136>  [+]
                            ├── <132> Var [result]
                            ╰── <135> Var [a1]
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
                    │   ╰── <14> Assign [=]
                    │       ├── <11> Var [x]
                    │       ╰── <13> Constant Int [3]
                    ╰── Block
                        ╰── Return
                            ╰── <19> Var [x]
    "#;
    assert_parse(src, expected);
}
