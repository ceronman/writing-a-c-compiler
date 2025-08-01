use super::{assert_error, assert_parse};

#[test]
fn test_invalid_parse_decl_as_loop_body() {
    assert_error(
        r#"
        int main(void) {
            while (1)
                int i = 0;
              //^^^ Expected statement, but found 'int'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_do_extra_semicolon() {
    assert_error(
        r#"
        int main(void) {
            do {
                int a;
            }; while(1);
           //^ Expected 'while', but found ';'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_do_missing_semicolon() {
    assert_error(
        r#"
        int main(void) {
            do {
                4;
            } while(1)
            return 0;
          //^^^^^^ Expected ';', but found 'return'
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_do_while_empty_parens() {
    assert_error(
        r#"
        int main(void) {
            do
                1;
            while ();
                 //^ Expected expression, but found ')'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_extra_credit_compound_assignment_invalid_decl() {
    assert_error(
        r#"
        int main(void) {
            for (int i += 1; i < 10; i += 1) {
                     //^^ Expected ';', but found '+='
                return 0;
            }
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_extra_credit_label_in_loop_header() {
    assert_error(
        r#"
        int main(void) {
            for (int i = 0; label: i < 10; i = i + 1) {
                               //^ Expected ';', but found ':'
                ;
            }
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_extra_credit_label_is_not_block() {
    assert_error(
        r#"
        int main(void) {
            int a = 0;
            int b = 0;
            do
            do_body:
                a = a + 1;
                b = b - 1;
              //^ Expected 'while', but found 'b'
            while (a < 10)
                ;
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_extra_credit_switch_case_declaration() {
    assert_error(
        r#"
        int main(void) {
            switch(3) {
                case 3:
                    int i = 0;
                  //^^^ Expected statement, but found 'int'
                    return i;
            }
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_extra_credit_switch_goto_case() {
    assert_error(
        r#"
        int main(void) {
            goto 3;
               //^ Expected identifier, but found '3'
            switch (3) {
                case 3: return 0;
            }
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_extra_credit_switch_missing_case_value() {
    assert_error(
        r#"
        int main(void) {
            switch(0) {
                case: return 0;
                  //^ Expected expression, but found ':'
            }
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_extra_credit_switch_missing_paren() {
    assert_error(
        r#"
        int main(void) {
            switch 3 {
                 //^ Expected '(', but found '3'
                case 3: return 0;
            }
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_extra_credit_switch_no_condition() {
    assert_error(
        r#"
        int main(void) {
            switch {
                 //^ Expected '(', but found '{'
                return 0;
            }
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_extra_for_header_clause() {
    assert_error(
        r#"
        int main(void) {
            for (int i = 0; i < 10; i = i + 1; )
                                           //^ Expected ')', but found ';'
                ;
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_invalid_for_declaration() {
    assert_error(
        r#"
        int main(void) {
            for (; int i = 0; i = i + 1)
                 //^^^ Expected expression, but found 'int'
                ;
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_missing_for_header_clause() {
    assert_error(
        r#"
        int main(void) {
            for (int i = 0;)
                         //^ Expected expression, but found ')'
                ;
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_paren_mismatch() {
    assert_error(
        r#"
        int main(void) {
            for (int i = 2; ))
                          //^ Expected expression, but found ')'
                int a = 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_statement_in_condition() {
    assert_error(
        r#"
        int main(void) {
            while(int a) {
                //^^^ Expected expression, but found 'int'
                2;
            }
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_while_missing_paren() {
    assert_error(
        r#"
        int main(void) {
            while 1 {
                //^ Expected '(', but found '1'
                return 0;
            }
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_break_not_in_loop() {
    let src = r#"
        int main(void) {
            if (1)
                break;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── If
                        ├── Condition
                        │   ╰── <5> Constant Int [1]
                        ╰── Then
                            ╰── Break
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_continue_not_in_loop() {
    let src = r#"
        int main(void) {
            {
                int a;
                continue;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── Block
                    │   ├── VarDeclaration
                    │   │   ├── Name
                    │   │   │   ╰── a
                    │   │   ╰── Type
                    │   │       ╰── Int
                    │   ╰── Continue
                    ╰── Return
                        ╰── <12> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_extra_credit_case_continue() {
    let src = r#"
        int main(void) {
            int a = 3;
            switch(a + 1) {
                case 0:
                    continue;
                default: a = 1;
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
                    │       ╰── <8> Constant Int [3]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <15>  [+]
                    │   │       ├── <12> Var [a]
                    │   │       ╰── <14> Constant Int [1]
                    │   ╰── Block
                    │       ├── Case [0]
                    │       │   ╰── Continue
                    │       ╰── Default
                    │           ╰── <23> Assign [=]
                    │               ├── <20> Var [a]
                    │               ╰── <22> Constant Int [1]
                    ╰── Return
                        ╰── <30> Var [a]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_extra_credit_case_outside_switch() {
    let src = r#"
        int main(void) {
            for (int i = 0; i < 10; i = i + 1) {
                case 0: return 1;
            }
            return 9;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <8> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <16>  [<]
                    │   │       ├── <13> Var [i]
                    │   │       ╰── <15> Constant Int [10]
                    │   ├── Condition
                    │   │   ╰── <25> Assign [=]
                    │   │       ├── <18> Var [i]
                    │   │       ╰── <24>  [+]
                    │   │           ├── <21> Var [i]
                    │   │           ╰── <23> Constant Int [1]
                    │   ╰── Block
                    │       ╰── Case [0]
                    │           ╰── Return
                    │               ╰── <27> Constant Int [1]
                    ╰── Return
                        ╰── <33> Constant Int [9]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_extra_credit_default_continue() {
    let src = r#"
        int main(void) {
            int a = 3;
            switch(a + 1) {
                case 0:
                    a = 1;
                default: continue;
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
                    │       ╰── <8> Constant Int [3]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <15>  [+]
                    │   │       ├── <12> Var [a]
                    │   │       ╰── <14> Constant Int [1]
                    │   ╰── Block
                    │       ├── Case [0]
                    │       │   ╰── <21> Assign [=]
                    │       │       ├── <18> Var [a]
                    │       │       ╰── <20> Constant Int [1]
                    │       ╰── Default
                    │           ╰── Continue
                    ╰── Return
                        ╰── <30> Var [a]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_extra_credit_default_outside_switch() {
    let src = r#"
        int main(void) {
            {
                default: return 0;
            }
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Block
                        ╰── Default
                            ╰── Return
                                ╰── <5> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_extra_credit_different_cases_same_scope() {
    let src = r#"
        int main(void) {
            int a = 1;
            switch (a) {
                case 1:;
                    int b = 10;
                    break;
                case 2:;
                    int b = 11;
                    break;
                default:
                    break;
            }
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
                    │       ╰── <8> Constant Int [1]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <12> Var [a]
                    │   ╰── Block
                    │       ├── Case [1]
                    │       │   ╰── Empty
                    │       ├── VarDeclaration
                    │       │   ├── Name
                    │       │   │   ╰── b
                    │       │   ├── Type
                    │       │   │   ╰── Int
                    │       │   ╰── Initializer
                    │       │       ╰── <19> Constant Int [10]
                    │       ├── Break
                    │       ├── Case [2]
                    │       │   ╰── Empty
                    │       ├── VarDeclaration
                    │       │   ├── Name
                    │       │   │   ╰── b
                    │       │   ├── Type
                    │       │   │   ╰── Int
                    │       │   ╰── Initializer
                    │       │       ╰── <29> Constant Int [11]
                    │       ├── Break
                    │       ╰── Default
                    │           ╰── Break
                    ╰── Return
                        ╰── <38> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_extra_credit_duplicate_case() {
    let src = r#"
        int main(void) {
            switch(4) {
                case 5: return 0;
                case 4: return 1;
                case 5: return 0;
                default: return 2;
            }
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Switch
                        ├── Expression
                        │   ╰── <5> Constant Int [4]
                        ╰── Block
                            ├── Case [5]
                            │   ╰── Return
                            │       ╰── <7> Constant Int [0]
                            ├── Case [4]
                            │   ╰── Return
                            │       ╰── <11> Constant Int [1]
                            ├── Case [5]
                            │   ╰── Return
                            │       ╰── <15> Constant Int [0]
                            ╰── Default
                                ╰── Return
                                    ╰── <18> Constant Int [2]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_extra_credit_duplicate_case_in_labeled_switch() {
    let src = r#"
        int main(void) {
            int a = 0;
        label:
            switch (a) {
                case 1:
                case 1:
                    break;
            }
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
                    │       ╰── <8> Constant Int [0]
                    ├── Label [label]
                    │   ╰── Switch
                    │       ├── Expression
                    │       │   ╰── <13> Var [a]
                    │       ╰── Block
                    │           ╰── Case [1]
                    │               ╰── Case [1]
                    │                   ╰── Break
                    ╰── Return
                        ╰── <23> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_extra_credit_duplicate_case_in_nested_statement() {
    let src = r#"
        
        int main(void) {
            int a = 10;
            switch (a) {
                case 1: {
                    if(1) {
                        case 1:
                        return 0;
                    }
                }
            }
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
                    │       ╰── <8> Constant Int [10]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <12> Var [a]
                    │   ╰── Block
                    │       ╰── Case [1]
                    │           ╰── Block
                    │               ╰── If
                    │                   ├── Condition
                    │                   │   ╰── <14> Constant Int [1]
                    │                   ╰── Then
                    │                       ╰── Block
                    │                           ╰── Case [1]
                    │                               ╰── Return
                    │                                   ╰── <16> Constant Int [0]
                    ╰── Return
                        ╰── <28> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_extra_credit_duplicate_default() {
    let src = r#"
        int main(void) {
            int a = 0;
            switch(a) {
                case 0: return 0;
                default: return 1;
                case 2: return 2;
                default: return 2;
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
                    │       ╰── <8> Constant Int [0]
                    ╰── Switch
                        ├── Expression
                        │   ╰── <12> Var [a]
                        ╰── Block
                            ├── Case [0]
                            │   ╰── Return
                            │       ╰── <14> Constant Int [0]
                            ├── Default
                            │   ╰── Return
                            │       ╰── <17> Constant Int [1]
                            ├── Case [2]
                            │   ╰── Return
                            │       ╰── <21> Constant Int [2]
                            ╰── Default
                                ╰── Return
                                    ╰── <24> Constant Int [2]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_extra_credit_duplicate_default_in_nested_statement() {
    let src = r#"
        
        int main(void) {
            int a = 10;
            switch (a) {
                case 1:
                for (int i = 0; i < 10; i = i + 1) {
                    continue;
                    while(1)
                    default:;
                }
                case 2:
                return 0;
                default:;
            }
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
                    │       ╰── <8> Constant Int [10]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <12> Var [a]
                    │   ╰── Block
                    │       ├── Case [1]
                    │       │   ╰── For
                    │       │       ├── Init
                    │       │       │   ╰── VarDeclaration
                    │       │       │       ├── Name
                    │       │       │       │   ╰── i
                    │       │       │       ├── Type
                    │       │       │       │   ╰── Int
                    │       │       │       ╰── Initializer
                    │       │       │           ╰── <17> Constant Int [0]
                    │       │       ├── Condition
                    │       │       │   ╰── <25>  [<]
                    │       │       │       ├── <22> Var [i]
                    │       │       │       ╰── <24> Constant Int [10]
                    │       │       ├── Condition
                    │       │       │   ╰── <34> Assign [=]
                    │       │       │       ├── <27> Var [i]
                    │       │       │       ╰── <33>  [+]
                    │       │       │           ├── <30> Var [i]
                    │       │       │           ╰── <32> Constant Int [1]
                    │       │       ╰── Block
                    │       │           ├── Continue
                    │       │           ╰── While
                    │       │               ├── Condition
                    │       │               │   ╰── <36> Constant Int [1]
                    │       │               ╰── Body
                    │       │                   ╰── Default
                    │       │                       ╰── Empty
                    │       ├── Case [2]
                    │       │   ╰── Return
                    │       │       ╰── <45> Constant Int [0]
                    │       ╰── Default
                    │           ╰── Empty
                    ╰── Return
                        ╰── <53> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_extra_credit_duplicate_label_in_default() {
    let src = r#"
        int main(void) {
                int a = 1;
        label:
            switch (a) {
                case 1:
                    return 0;
                default:
                label:
                    return 1;
            }
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
                    │       ╰── <8> Constant Int [1]
                    ├── Label [label]
                    │   ╰── Switch
                    │       ├── Expression
                    │       │   ╰── <13> Var [a]
                    │       ╰── Block
                    │           ├── Case [1]
                    │           │   ╰── Return
                    │           │       ╰── <15> Constant Int [0]
                    │           ╰── Default
                    │               ╰── Label [label]
                    │                   ╰── Return
                    │                       ╰── <19> Constant Int [1]
                    ╰── Return
                        ╰── <27> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_extra_credit_duplicate_label_in_loop() {
    let src = r#"
        int main(void) {
            do {
            lbl:
                return 1;
            lbl:
                return 2;
            } while (1);
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── DoWhile
                    │   ├── Body
                    │   │   ╰── Block
                    │   │       ├── Label [lbl]
                    │   │       │   ╰── Return
                    │   │       │       ╰── <6> Constant Int [1]
                    │   │       ╰── Label [lbl]
                    │   │           ╰── Return
                    │   │               ╰── <10> Constant Int [2]
                    │   ╰── Condition
                    │       ╰── <15> Constant Int [1]
                    ╰── Return
                        ╰── <17> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_extra_credit_duplicate_variable_in_switch() {
    let src = r#"
        int main(void) {
            int a = 1;
            switch (a) {
                int b = 2;
                case 0:
                    a = 3;
                    int b = 2;
            }
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
                    │       ╰── <8> Constant Int [1]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <12> Var [a]
                    │   ╰── Block
                    │       ├── VarDeclaration
                    │       │   ├── Name
                    │       │   │   ╰── b
                    │       │   ├── Type
                    │       │   │   ╰── Int
                    │       │   ╰── Initializer
                    │       │       ╰── <16> Constant Int [2]
                    │       ├── Case [0]
                    │       │   ╰── <24> Assign [=]
                    │       │       ├── <21> Var [a]
                    │       │       ╰── <23> Constant Int [3]
                    │       ╰── VarDeclaration
                    │           ├── Name
                    │           │   ╰── b
                    │           ├── Type
                    │           │   ╰── Int
                    │           ╰── Initializer
                    │               ╰── <30> Constant Int [2]
                    ╰── Return
                        ╰── <36> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_extra_credit_labeled_break_outside_loop() {
    let src = r#"
        int main(void) {
            label: break;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── Label [label]
                    │   ╰── Break
                    ╰── Return
                        ╰── <8> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_extra_credit_non_constant_case() {
    let src = r#"
        int main(void) {
            int a = 3;
            switch(a + 1) {
                case 0: return 0;
                case a: return 1;
                case 1: return 2;
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
                    │       ╰── <8> Constant Int [3]
                    ╰── Switch
                        ├── Expression
                        │   ╰── <15>  [+]
                        │       ├── <12> Var [a]
                        │       ╰── <14> Constant Int [1]
                        ╰── Block
                            ├── Case [0]
                            │   ╰── Return
                            │       ╰── <17> Constant Int [0]
                            ├── Case [invalid]
                            │   ├── Value
                            │   │   ╰── <21> Var [a]
                            │   ╰── Return
                            │       ╰── <22> Constant Int [1]
                            ╰── Case [1]
                                ╰── Return
                                    ╰── <26> Constant Int [2]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_extra_credit_switch_continue() {
    let src = r#"
        int main(void) {
            int a = 3;
            switch(a + 1) {
                case 0:
                    a = 4;
                    continue;
                default: a = 1;
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
                    │       ╰── <8> Constant Int [3]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <15>  [+]
                    │   │       ├── <12> Var [a]
                    │   │       ╰── <14> Constant Int [1]
                    │   ╰── Block
                    │       ├── Case [0]
                    │       │   ╰── <21> Assign [=]
                    │       │       ├── <18> Var [a]
                    │       │       ╰── <20> Constant Int [4]
                    │       ├── Continue
                    │       ╰── Default
                    │           ╰── <29> Assign [=]
                    │               ├── <26> Var [a]
                    │               ╰── <28> Constant Int [1]
                    ╰── Return
                        ╰── <36> Var [a]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_extra_credit_undeclared_var_switch_expression() {
    let src = r#"
        int main(void) {
            switch(a) {
                case 1: return 0;
                case 2: return 1;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <6> Var [a]
                    │   ╰── Block
                    │       ├── Case [1]
                    │       │   ╰── Return
                    │       │       ╰── <8> Constant Int [0]
                    │       ╰── Case [2]
                    │           ╰── Return
                    │               ╰── <12> Constant Int [1]
                    ╰── Return
                        ╰── <18> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_extra_credit_undeclared_variable_in_case() {
    let src = r#"
        int main(void) {
            int a = 10;
            switch (a) {
                case 1:
                    return b;
                    break;
                default:
                    break;
            }
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
                    │       ╰── <8> Constant Int [10]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <12> Var [a]
                    │   ╰── Block
                    │       ├── Case [1]
                    │       │   ╰── Return
                    │       │       ╰── <15> Var [b]
                    │       ├── Break
                    │       ╰── Default
                    │           ╰── Break
                    ╰── Return
                        ╰── <24> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_extra_credit_undeclared_variable_in_default() {
    let src = r#"
        int main(void) {
            int a = 10;
            switch (a) {
                case 1:
                    break;
                default:
                    return b;
                    break;
            }
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
                    │       ╰── <8> Constant Int [10]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <12> Var [a]
                    │   ╰── Block
                    │       ├── Case [1]
                    │       │   ╰── Break
                    │       ├── Default
                    │       │   ╰── Return
                    │       │       ╰── <17> Var [b]
                    │       ╰── Break
                    ╰── Return
                        ╰── <24> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_extra_credit_undefined_label_in_case() {
    let src = r#"
        
        int main(void) {
            int a = 3;
            switch (a) {
                case 1: goto foo;
                default: return 0;
            }
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
                    │       ╰── <8> Constant Int [3]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <12> Var [a]
                    │   ╰── Block
                    │       ├── Case [1]
                    │       │   ╰── Goto [foo]
                    │       ╰── Default
                    │           ╰── Return
                    │               ╰── <17> Constant Int [0]
                    ╰── Return
                        ╰── <23> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_out_of_scope_do_loop() {
    let src = r#"
        int main(void) {
            do {
                int a = a + 1;
            } while (a < 100);
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── DoWhile
                        ├── Body
                        │   ╰── Block
                        │       ╰── VarDeclaration
                        │           ├── Name
                        │           │   ╰── a
                        │           ├── Type
                        │           │   ╰── Int
                        │           ╰── Initializer
                        │               ╰── <12>  [+]
                        │                   ├── <9> Var [a]
                        │                   ╰── <11> Constant Int [1]
                        ╰── Condition
                            ╰── <21>  [<]
                                ├── <18> Var [a]
                                ╰── <20> Constant Int [100]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_out_of_scope_loop_variable() {
    let src = r#"
        int main(void)
        {
            for (i = 0; i < 1; i = i + 1)
            {
                return 0;
            }
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── For
                        ├── Init
                        │   ╰── <9> Assign [=]
                        │       ├── <6> Var [i]
                        │       ╰── <8> Constant Int [0]
                        ├── Condition
                        │   ╰── <14>  [<]
                        │       ├── <11> Var [i]
                        │       ╰── <13> Constant Int [1]
                        ├── Condition
                        │   ╰── <23> Assign [=]
                        │       ├── <16> Var [i]
                        │       ╰── <22>  [+]
                        │           ├── <19> Var [i]
                        │           ╰── <21> Constant Int [1]
                        ╰── Block
                            ╰── Return
                                ╰── <24> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_break() {
    let src = r#"
        int main(void) {
            int a = 10;
            int b = 20;
            for (b = -20; b < 0; b = b + 1) {
                a = a - 1;
                if (a <= 0)
                    break;
            }
            return a == 0 && b == -11;
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
                    │       ╰── <8> Constant Int [10]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <14> Constant Int [20]
                    ├── For
                    │   ├── Init
                    │   │   ╰── <23> Assign [=]
                    │   │       ├── <18> Var [b]
                    │   │       ╰── <22> Unary [-]
                    │   │           ╰── <21> Constant Int [20]
                    │   ├── Condition
                    │   │   ╰── <28>  [<]
                    │   │       ├── <25> Var [b]
                    │   │       ╰── <27> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <37> Assign [=]
                    │   │       ├── <30> Var [b]
                    │   │       ╰── <36>  [+]
                    │   │           ├── <33> Var [b]
                    │   │           ╰── <35> Constant Int [1]
                    │   ╰── Block
                    │       ├── <46> Assign [=]
                    │       │   ├── <39> Var [a]
                    │       │   ╰── <45>  [-]
                    │       │       ├── <42> Var [a]
                    │       │       ╰── <44> Constant Int [1]
                    │       ╰── If
                    │           ├── Condition
                    │           │   ╰── <52>  [<=]
                    │           │       ├── <49> Var [a]
                    │           │       ╰── <51> Constant Int [0]
                    │           ╰── Then
                    │               ╰── Break
                    ╰── Return
                        ╰── <71>  [&&]
                            ├── <62>  [==]
                            │   ├── <59> Var [a]
                            │   ╰── <61> Constant Int [0]
                            ╰── <70>  [==]
                                ├── <65> Var [b]
                                ╰── <69> Unary [-]
                                    ╰── <68> Constant Int [11]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_break_immediate() {
    let src = r#"
        int main(void) {
            int a = 10;
            while ((a = 1))
                break;
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
                    │       ╰── <8> Constant Int [10]
                    ├── While
                    │   ├── Condition
                    │   │   ╰── <16> Assign [=]
                    │   │       ├── <12> Var [a]
                    │   │       ╰── <14> Constant Int [1]
                    │   ╰── Body
                    │       ╰── Break
                    ╰── Return
                        ╰── <20> Var [a]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_continue() {
    let src = r#"
        int main(void) {
            int sum = 0;
            int counter;
            for (int i = 0; i <= 10; i = i + 1) {
                counter = i;
                if (i % 2 == 0)
                    continue;
                sum = sum + 1;
            }
            return sum == 5 && counter == 10;
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
                    │       ╰── <8> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── counter
                    │   ╰── Type
                    │       ╰── Int
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <18> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <26>  [<=]
                    │   │       ├── <23> Var [i]
                    │   │       ╰── <25> Constant Int [10]
                    │   ├── Condition
                    │   │   ╰── <35> Assign [=]
                    │   │       ├── <28> Var [i]
                    │   │       ╰── <34>  [+]
                    │   │           ├── <31> Var [i]
                    │   │           ╰── <33> Constant Int [1]
                    │   ╰── Block
                    │       ├── <41> Assign [=]
                    │       │   ├── <37> Var [counter]
                    │       │   ╰── <40> Var [i]
                    │       ├── If
                    │       │   ├── Condition
                    │       │   │   ╰── <50>  [==]
                    │       │   │       ├── <47>  [%]
                    │       │   │       │   ├── <44> Var [i]
                    │       │   │       │   ╰── <46> Constant Int [2]
                    │       │   │       ╰── <49> Constant Int [0]
                    │       │   ╰── Then
                    │       │       ╰── Continue
                    │       ╰── <61> Assign [=]
                    │           ├── <54> Var [sum]
                    │           ╰── <60>  [+]
                    │               ├── <57> Var [sum]
                    │               ╰── <59> Constant Int [1]
                    ╰── Return
                        ╰── <77>  [&&]
                            ├── <70>  [==]
                            │   ├── <67> Var [sum]
                            │   ╰── <69> Constant Int [5]
                            ╰── <76>  [==]
                                ├── <73> Var [counter]
                                ╰── <75> Constant Int [10]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_continue_empty_post() {
    let src = r#"
        int main(void) {
            int sum = 0;
            for (int i = 0; i < 10;) {
                i = i + 1;
                if (i % 2)
                    continue;
                sum = sum + i;
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
                    │       ╰── <8> Constant Int [0]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <14> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <22>  [<]
                    │   │       ├── <19> Var [i]
                    │   │       ╰── <21> Constant Int [10]
                    │   ╰── Block
                    │       ├── <31> Assign [=]
                    │       │   ├── <24> Var [i]
                    │       │   ╰── <30>  [+]
                    │       │       ├── <27> Var [i]
                    │       │       ╰── <29> Constant Int [1]
                    │       ├── If
                    │       │   ├── Condition
                    │       │   │   ╰── <37>  [%]
                    │       │   │       ├── <34> Var [i]
                    │       │   │       ╰── <36> Constant Int [2]
                    │       │   ╰── Then
                    │       │       ╰── Continue
                    │       ╰── <49> Assign [=]
                    │           ├── <41> Var [sum]
                    │           ╰── <48>  [+]
                    │               ├── <44> Var [sum]
                    │               ╰── <47> Var [i]
                    ╰── Return
                        ╰── <55> Var [sum]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_do_while() {
    let src = r#"
        int main(void) {
            int a = 1;
            do {
                a = a * 2;
            } while(a < 11);
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
                    │       ╰── <8> Constant Int [1]
                    ├── DoWhile
                    │   ├── Body
                    │   │   ╰── Block
                    │   │       ╰── <19> Assign [=]
                    │   │           ├── <12> Var [a]
                    │   │           ╰── <18>  [*]
                    │   │               ├── <15> Var [a]
                    │   │               ╰── <17> Constant Int [2]
                    │   ╰── Condition
                    │       ╰── <27>  [<]
                    │           ├── <24> Var [a]
                    │           ╰── <26> Constant Int [11]
                    ╰── Return
                        ╰── <30> Var [a]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_do_while_break_immediate() {
    let src = r#"
        int main(void) {
            int a = 10;
            do
                break;
            while ((a = 1));
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
                    │       ╰── <8> Constant Int [10]
                    ├── DoWhile
                    │   ├── Body
                    │   │   ╰── Break
                    │   ╰── Condition
                    │       ╰── <17> Assign [=]
                    │           ├── <13> Var [a]
                    │           ╰── <15> Constant Int [1]
                    ╰── Return
                        ╰── <20> Var [a]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_empty_expression() {
    let src = r#"
        int main(void) {
            return 0;;;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── Return
                    │   ╰── <5> Constant Int [0]
                    ├── Empty
                    ╰── Empty
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_empty_loop_body() {
    let src = r#"
        int main(void) {
            int i = 2147;
            do ; while ((i = i - 5) >= 256);
            return i;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <8> Constant Int [2147]
                    ├── DoWhile
                    │   ├── Body
                    │   │   ╰── Empty
                    │   ╰── Condition
                    │       ╰── <24>  [>=]
                    │           ├── <21> Assign [=]
                    │           │   ├── <13> Var [i]
                    │           │   ╰── <19>  [-]
                    │           │       ├── <16> Var [i]
                    │           │       ╰── <18> Constant Int [5]
                    │           ╰── <23> Constant Int [256]
                    ╰── Return
                        ╰── <27> Var [i]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_case_block() {
    let src = r#"
        int main(void) {
            int a = 4;
            int b = 0;
            switch(2) {
                case 2: {
                    int a = 8;
                    b = a;
                }
            }
            return (a == 4 && b == 8);
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
                    │       ╰── <8> Constant Int [4]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <14> Constant Int [0]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <17> Constant Int [2]
                    │   ╰── Block
                    │       ╰── Case [2]
                    │           ╰── Block
                    │               ├── VarDeclaration
                    │               │   ├── Name
                    │               │   │   ╰── a
                    │               │   ├── Type
                    │               │   │   ╰── Int
                    │               │   ╰── Initializer
                    │               │       ╰── <22> Constant Int [8]
                    │               ╰── <30> Assign [=]
                    │                   ├── <26> Var [b]
                    │                   ╰── <29> Var [a]
                    ╰── Return
                        ╰── <50>  [&&]
                            ├── <42>  [==]
                            │   ├── <39> Var [a]
                            │   ╰── <41> Constant Int [4]
                            ╰── <48>  [==]
                                ├── <45> Var [b]
                                ╰── <47> Constant Int [8]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_compound_assignment_controlling_expression() {
    let src = r#"
        int main(void) {
            int i = 100;
            int sum = 0;
            do sum += 2;
            while (i -= 1);
            return (i == 0 && sum == 200);
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <8> Constant Int [100]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── sum
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <14> Constant Int [0]
                    ├── DoWhile
                    │   ├── Body
                    │   │   ╰── <21> Assign [+=]
                    │   │       ├── <18> Var [sum]
                    │   │       ╰── <20> Constant Int [2]
                    │   ╰── Condition
                    │       ╰── <27> Assign [-=]
                    │           ├── <24> Var [i]
                    │           ╰── <26> Constant Int [1]
                    ╰── Return
                        ╰── <41>  [&&]
                            ├── <33>  [==]
                            │   ├── <30> Var [i]
                            │   ╰── <32> Constant Int [0]
                            ╰── <39>  [==]
                                ├── <36> Var [sum]
                                ╰── <38> Constant Int [200]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_compound_assignment_for_loop() {
    let src = r#"
        int main(void) {
            int i = 1;
            for (i *= -1; i >= -100; i -=3)
                ;
            return (i == -103);
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <8> Constant Int [1]
                    ├── For
                    │   ├── Init
                    │   │   ╰── <17> Assign [*=]
                    │   │       ├── <12> Var [i]
                    │   │       ╰── <16> Unary [-]
                    │   │           ╰── <15> Constant Int [1]
                    │   ├── Condition
                    │   │   ╰── <24>  [>=]
                    │   │       ├── <19> Var [i]
                    │   │       ╰── <23> Unary [-]
                    │   │           ╰── <22> Constant Int [100]
                    │   ├── Condition
                    │   │   ╰── <29> Assign [-=]
                    │   │       ├── <26> Var [i]
                    │   │       ╰── <28> Constant Int [3]
                    │   ╰── Empty
                    ╰── Return
                        ╰── <39>  [==]
                            ├── <33> Var [i]
                            ╰── <37> Unary [-]
                                ╰── <36> Constant Int [103]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_duffs_device() {
    let src = r#"
        
        int main(void) {
            int count = 37;
            int iterations = (count + 4) / 5;
            switch (count % 5) {
                case 0:
                    do {
                        count = count - 1;
                        case 4:
                            count = count - 1;
                        case 3:
                            count = count - 1;
                        case 2:
                            count = count - 1;
                        case 1:
                            count = count - 1;
                    } while ((iterations = iterations - 1) > 0);
            }
            return (count == 0 && iterations == 0);
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── count
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <8> Constant Int [37]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── iterations
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <22>  [/]
                    │           ├── <19>  [+]
                    │           │   ├── <15> Var [count]
                    │           │   ╰── <17> Constant Int [4]
                    │           ╰── <21> Constant Int [5]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <29>  [%]
                    │   │       ├── <26> Var [count]
                    │   │       ╰── <28> Constant Int [5]
                    │   ╰── Block
                    │       ╰── Case [0]
                    │           ╰── DoWhile
                    │               ├── Body
                    │               │   ╰── Block
                    │               │       ├── <39> Assign [=]
                    │               │       │   ├── <32> Var [count]
                    │               │       │   ╰── <38>  [-]
                    │               │       │       ├── <35> Var [count]
                    │               │       │       ╰── <37> Constant Int [1]
                    │               │       ├── Case [4]
                    │               │       │   ╰── <50> Assign [=]
                    │               │       │       ├── <43> Var [count]
                    │               │       │       ╰── <49>  [-]
                    │               │       │           ├── <46> Var [count]
                    │               │       │           ╰── <48> Constant Int [1]
                    │               │       ├── Case [3]
                    │               │       │   ╰── <62> Assign [=]
                    │               │       │       ├── <55> Var [count]
                    │               │       │       ╰── <61>  [-]
                    │               │       │           ├── <58> Var [count]
                    │               │       │           ╰── <60> Constant Int [1]
                    │               │       ├── Case [2]
                    │               │       │   ╰── <74> Assign [=]
                    │               │       │       ├── <67> Var [count]
                    │               │       │       ╰── <73>  [-]
                    │               │       │           ├── <70> Var [count]
                    │               │       │           ╰── <72> Constant Int [1]
                    │               │       ╰── Case [1]
                    │               │           ╰── <86> Assign [=]
                    │               │               ├── <79> Var [count]
                    │               │               ╰── <85>  [-]
                    │               │                   ├── <82> Var [count]
                    │               │                   ╰── <84> Constant Int [1]
                    │               ╰── Condition
                    │                   ╰── <103>  [>]
                    │                       ├── <100> Assign [=]
                    │                       │   ├── <92> Var [iterations]
                    │                       │   ╰── <98>  [-]
                    │                       │       ├── <95> Var [iterations]
                    │                       │       ╰── <97> Constant Int [1]
                    │                       ╰── <102> Constant Int [0]
                    ╰── Return
                        ╰── <121>  [&&]
                            ├── <113>  [==]
                            │   ├── <110> Var [count]
                            │   ╰── <112> Constant Int [0]
                            ╰── <119>  [==]
                                ├── <116> Var [iterations]
                                ╰── <118> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_goto_bypass_condition() {
    let src = r#"
        int main(void) {
            int i = 1;
            do {
            while_start:
                i = i + 1;
                if (i < 10)
                    goto while_start;
            } while (0);
            return i;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <8> Constant Int [1]
                    ├── DoWhile
                    │   ├── Body
                    │   │   ╰── Block
                    │   │       ├── Label [while_start]
                    │   │       │   ╰── <20> Assign [=]
                    │   │       │       ├── <13> Var [i]
                    │   │       │       ╰── <19>  [+]
                    │   │       │           ├── <16> Var [i]
                    │   │       │           ╰── <18> Constant Int [1]
                    │   │       ╰── If
                    │   │           ├── Condition
                    │   │           │   ╰── <27>  [<]
                    │   │           │       ├── <24> Var [i]
                    │   │           │       ╰── <26> Constant Int [10]
                    │   │           ╰── Then
                    │   │               ╰── Goto [while_start]
                    │   ╰── Condition
                    │       ╰── <33> Constant Int [0]
                    ╰── Return
                        ╰── <36> Var [i]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_goto_bypass_init_exp() {
    let src = r#"
        int main(void) {
            int i = 0;
            goto target;
            for (i = 5; i < 10; i = i + 1)
            target:
                if (i == 0)
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
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <8> Constant Int [0]
                    ├── Goto [target]
                    ├── For
                    │   ├── Init
                    │   │   ╰── <17> Assign [=]
                    │   │       ├── <14> Var [i]
                    │   │       ╰── <16> Constant Int [5]
                    │   ├── Condition
                    │   │   ╰── <22>  [<]
                    │   │       ├── <19> Var [i]
                    │   │       ╰── <21> Constant Int [10]
                    │   ├── Condition
                    │   │   ╰── <31> Assign [=]
                    │   │       ├── <24> Var [i]
                    │   │       ╰── <30>  [+]
                    │   │           ├── <27> Var [i]
                    │   │           ╰── <29> Constant Int [1]
                    │   ╰── Label [target]
                    │       ╰── If
                    │           ├── Condition
                    │           │   ╰── <37>  [==]
                    │           │       ├── <34> Var [i]
                    │           │       ╰── <36> Constant Int [0]
                    │           ╰── Then
                    │               ╰── Return
                    │                   ╰── <38> Constant Int [1]
                    ╰── Return
                        ╰── <43> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_goto_bypass_post_exp() {
    let src = r#"
        int main(void) {
            int sum = 0;
            for (int i = 0;; i = 0) {
            lbl:
                sum = sum + 1;
                i = i + 1;
                if (i > 10)
                    break;
                goto lbl;
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
                    │       ╰── <8> Constant Int [0]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <14> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <22> Assign [=]
                    │   │       ├── <19> Var [i]
                    │   │       ╰── <21> Constant Int [0]
                    │   ╰── Block
                    │       ├── Label [lbl]
                    │       │   ╰── <32> Assign [=]
                    │       │       ├── <25> Var [sum]
                    │       │       ╰── <31>  [+]
                    │       │           ├── <28> Var [sum]
                    │       │           ╰── <30> Constant Int [1]
                    │       ├── <43> Assign [=]
                    │       │   ├── <36> Var [i]
                    │       │   ╰── <42>  [+]
                    │       │       ├── <39> Var [i]
                    │       │       ╰── <41> Constant Int [1]
                    │       ├── If
                    │       │   ├── Condition
                    │       │   │   ╰── <49>  [>]
                    │       │   │       ├── <46> Var [i]
                    │       │   │       ╰── <48> Constant Int [10]
                    │       │   ╰── Then
                    │       │       ╰── Break
                    │       ╰── Goto [lbl]
                    ╰── Return
                        ╰── <58> Var [sum]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_label_loop_body() {
    let src = r#"
        
        int main(void) {
            int result = 0;
            goto label;
            while (0)
            label: { result = 1; }
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
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <8> Constant Int [0]
                    ├── Goto [label]
                    ├── While
                    │   ├── Condition
                    │   │   ╰── <13> Constant Int [0]
                    │   ╰── Body
                    │       ╰── Label [label]
                    │           ╰── Block
                    │               ╰── <19> Assign [=]
                    │                   ├── <16> Var [result]
                    │                   ╰── <18> Constant Int [1]
                    ╰── Return
                        ╰── <26> Var [result]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_label_loops_breaks_and_continues() {
    let src = r#"
        int main(void) {
            int sum = 0;
            goto do_label;
            return 0;
        do_label:
            do {
                sum = 1;
                goto while_label;
            } while (1);
        while_label:
            while (1) {
                sum = sum + 1;
                goto break_label;
                return 0;
            break_label:
                break;
            };
            goto for_label;
            return 0;
        for_label:
            for (int i = 0; i < 10; i = i + 1) {
                sum = sum + 1;
                goto continue_label;
                return 0;
            continue_label:
                continue;
                return 0;
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
                    │       ╰── <8> Constant Int [0]
                    ├── Goto [do_label]
                    ├── Return
                    │   ╰── <13> Constant Int [0]
                    ├── Label [do_label]
                    │   ╰── DoWhile
                    │       ├── Body
                    │       │   ╰── Block
                    │       │       ├── <20> Assign [=]
                    │       │       │   ├── <17> Var [sum]
                    │       │       │   ╰── <19> Constant Int [1]
                    │       │       ╰── Goto [while_label]
                    │       ╰── Condition
                    │           ╰── <26> Constant Int [1]
                    ├── Label [while_label]
                    │   ╰── While
                    │       ├── Condition
                    │       │   ╰── <30> Constant Int [1]
                    │       ╰── Body
                    │           ╰── Block
                    │               ├── <39> Assign [=]
                    │               │   ├── <32> Var [sum]
                    │               │   ╰── <38>  [+]
                    │               │       ├── <35> Var [sum]
                    │               │       ╰── <37> Constant Int [1]
                    │               ├── Goto [break_label]
                    │               ├── Return
                    │               │   ╰── <43> Constant Int [0]
                    │               ╰── Label [break_label]
                    │                   ╰── Break
                    ├── Empty
                    ├── Goto [for_label]
                    ├── Return
                    │   ╰── <55> Constant Int [0]
                    ├── Label [for_label]
                    │   ╰── For
                    │       ├── Init
                    │       │   ╰── VarDeclaration
                    │       │       ├── Name
                    │       │       │   ╰── i
                    │       │       ├── Type
                    │       │       │   ╰── Int
                    │       │       ╰── Initializer
                    │       │           ╰── <61> Constant Int [0]
                    │       ├── Condition
                    │       │   ╰── <69>  [<]
                    │       │       ├── <66> Var [i]
                    │       │       ╰── <68> Constant Int [10]
                    │       ├── Condition
                    │       │   ╰── <78> Assign [=]
                    │       │       ├── <71> Var [i]
                    │       │       ╰── <77>  [+]
                    │       │           ├── <74> Var [i]
                    │       │           ╰── <76> Constant Int [1]
                    │       ╰── Block
                    │           ├── <87> Assign [=]
                    │           │   ├── <80> Var [sum]
                    │           │   ╰── <86>  [+]
                    │           │       ├── <83> Var [sum]
                    │           │       ╰── <85> Constant Int [1]
                    │           ├── Goto [continue_label]
                    │           ├── Return
                    │           │   ╰── <91> Constant Int [0]
                    │           ├── Label [continue_label]
                    │           │   ╰── Continue
                    │           ╰── Return
                    │               ╰── <96> Constant Int [0]
                    ╰── Return
                        ╰── <103> Var [sum]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_loop_header_postfix_and_prefix() {
    let src = r#"
        int main(void) {
            int i = 100;
            int count = 0;
            while (i--) count++;
            if (count != 100)
                return 0;
            i = 100;
            count = 0;
            while (--i) count++;
            if (count != 99)
                return 0;
            return 1;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <8> Constant Int [100]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── count
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <14> Constant Int [0]
                    ├── While
                    │   ├── Condition
                    │   │   ╰── <20> Postfix [--]
                    │   │       ╰── <18> Var [i]
                    │   ╰── Body
                    │       ╰── <24> Postfix [++]
                    │           ╰── <22> Var [count]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <31>  [!=]
                    │   │       ├── <28> Var [count]
                    │   │       ╰── <30> Constant Int [100]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <32> Constant Int [0]
                    ├── <39> Assign [=]
                    │   ├── <36> Var [i]
                    │   ╰── <38> Constant Int [100]
                    ├── <45> Assign [=]
                    │   ├── <42> Var [count]
                    │   ╰── <44> Constant Int [0]
                    ├── While
                    │   ├── Condition
                    │   │   ╰── <50> Unary [--]
                    │   │       ╰── <49> Var [i]
                    │   ╰── Body
                    │       ╰── <54> Postfix [++]
                    │           ╰── <52> Var [count]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <61>  [!=]
                    │   │       ├── <58> Var [count]
                    │   │       ╰── <60> Constant Int [99]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <62> Constant Int [0]
                    ╰── Return
                        ╰── <65> Constant Int [1]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_loop_in_switch() {
    let src = r#"
        int main(void) {
            int cond = 10;
            switch (cond) {
                case 1:
                    return 0;
                case 10:
                    for (int i = 0; i < 5; i = i + 1) {
                        cond = cond - 1;
                        if (cond == 8)
                            break;
                    }
                    return 123;
                default:
                    return 2;
            }
            return 3;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── cond
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <8> Constant Int [10]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <12> Var [cond]
                    │   ╰── Block
                    │       ├── Case [1]
                    │       │   ╰── Return
                    │       │       ╰── <14> Constant Int [0]
                    │       ├── Case [10]
                    │       │   ╰── For
                    │       │       ├── Init
                    │       │       │   ╰── VarDeclaration
                    │       │       │       ├── Name
                    │       │       │       │   ╰── i
                    │       │       │       ├── Type
                    │       │       │       │   ╰── Int
                    │       │       │       ╰── Initializer
                    │       │       │           ╰── <21> Constant Int [0]
                    │       │       ├── Condition
                    │       │       │   ╰── <29>  [<]
                    │       │       │       ├── <26> Var [i]
                    │       │       │       ╰── <28> Constant Int [5]
                    │       │       ├── Condition
                    │       │       │   ╰── <38> Assign [=]
                    │       │       │       ├── <31> Var [i]
                    │       │       │       ╰── <37>  [+]
                    │       │       │           ├── <34> Var [i]
                    │       │       │           ╰── <36> Constant Int [1]
                    │       │       ╰── Block
                    │       │           ├── <47> Assign [=]
                    │       │           │   ├── <40> Var [cond]
                    │       │           │   ╰── <46>  [-]
                    │       │           │       ├── <43> Var [cond]
                    │       │           │       ╰── <45> Constant Int [1]
                    │       │           ╰── If
                    │       │               ├── Condition
                    │       │               │   ╰── <53>  [==]
                    │       │               │       ├── <50> Var [cond]
                    │       │               │       ╰── <52> Constant Int [8]
                    │       │               ╰── Then
                    │       │                   ╰── Break
                    │       ├── Return
                    │       │   ╰── <60> Constant Int [123]
                    │       ╰── Default
                    │           ╰── Return
                    │               ╰── <62> Constant Int [2]
                    ╰── Return
                        ╰── <68> Constant Int [3]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_post_exp_incr() {
    let src = r#"
        int main(void) {
            int product = 1;
            for (int i = 0; i < 10; i++) {
                product = product + 2;
            }
            return product;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── product
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <8> Constant Int [1]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <14> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <22>  [<]
                    │   │       ├── <19> Var [i]
                    │   │       ╰── <21> Constant Int [10]
                    │   ├── Condition
                    │   │   ╰── <26> Postfix [++]
                    │   │       ╰── <24> Var [i]
                    │   ╰── Block
                    │       ╰── <35> Assign [=]
                    │           ├── <28> Var [product]
                    │           ╰── <34>  [+]
                    │               ├── <31> Var [product]
                    │               ╰── <33> Constant Int [2]
                    ╰── Return
                        ╰── <41> Var [product]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_switch() {
    let src = r#"
        
        int main(void) {
            switch(3) {
                case 0: return 0;
                case 1: return 1;
                case 3: return 3;
                case 5: return 5;
            }
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Switch
                        ├── Expression
                        │   ╰── <5> Constant Int [3]
                        ╰── Block
                            ├── Case [0]
                            │   ╰── Return
                            │       ╰── <7> Constant Int [0]
                            ├── Case [1]
                            │   ╰── Return
                            │       ╰── <11> Constant Int [1]
                            ├── Case [3]
                            │   ╰── Return
                            │       ╰── <15> Constant Int [3]
                            ╰── Case [5]
                                ╰── Return
                                    ╰── <19> Constant Int [5]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_switch_assign_in_condition() {
    let src = r#"
        int main(void) {
            int a = 0;
            switch (a = 1) {
                case 0:
                    return 10;
                case 1:
                    a = a * 2;
                    break;
                default:
                    a = 99;
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
                    │       ╰── <8> Constant Int [0]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <15> Assign [=]
                    │   │       ├── <12> Var [a]
                    │   │       ╰── <14> Constant Int [1]
                    │   ╰── Block
                    │       ├── Case [0]
                    │       │   ╰── Return
                    │       │       ╰── <17> Constant Int [10]
                    │       ├── Case [1]
                    │       │   ╰── <29> Assign [=]
                    │       │       ├── <22> Var [a]
                    │       │       ╰── <28>  [*]
                    │       │           ├── <25> Var [a]
                    │       │           ╰── <27> Constant Int [2]
                    │       ├── Break
                    │       ╰── Default
                    │           ╰── <37> Assign [=]
                    │               ├── <34> Var [a]
                    │               ╰── <36> Constant Int [99]
                    ╰── Return
                        ╰── <44> Var [a]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_switch_break() {
    let src = r#"
        int main(void) {
            int a = 5;
            switch (a) {
                case 5:
                    a = 10;
                    break;
                case 6:
                    a = 0;
                    break;
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
                    │       ╰── <8> Constant Int [5]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <12> Var [a]
                    │   ╰── Block
                    │       ├── Case [5]
                    │       │   ╰── <18> Assign [=]
                    │       │       ├── <15> Var [a]
                    │       │       ╰── <17> Constant Int [10]
                    │       ├── Break
                    │       ├── Case [6]
                    │       │   ╰── <27> Assign [=]
                    │       │       ├── <24> Var [a]
                    │       │       ╰── <26> Constant Int [0]
                    │       ╰── Break
                    ╰── Return
                        ╰── <35> Var [a]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_switch_decl() {
    let src = r#"
        int main(void) {
            int a = 3;
            int b = 0;
            switch(a) {
                int a = (b = 5);
            case 3:
                a = 4;
                b = b + a;
            }
            return a == 3 && b == 4;
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
                    │       ╰── <8> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <14> Constant Int [0]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <18> Var [a]
                    │   ╰── Block
                    │       ├── VarDeclaration
                    │       │   ├── Name
                    │       │   │   ╰── a
                    │       │   ├── Type
                    │       │   │   ╰── Int
                    │       │   ╰── Initializer
                    │       │       ╰── <27> Assign [=]
                    │       │           ├── <23> Var [b]
                    │       │           ╰── <25> Constant Int [5]
                    │       ├── Case [3]
                    │       │   ╰── <35> Assign [=]
                    │       │       ├── <32> Var [a]
                    │       │       ╰── <34> Constant Int [4]
                    │       ╰── <47> Assign [=]
                    │           ├── <39> Var [b]
                    │           ╰── <46>  [+]
                    │               ├── <42> Var [b]
                    │               ╰── <45> Var [a]
                    ╰── Return
                        ╰── <63>  [&&]
                            ├── <56>  [==]
                            │   ├── <53> Var [a]
                            │   ╰── <55> Constant Int [3]
                            ╰── <62>  [==]
                                ├── <59> Var [b]
                                ╰── <61> Constant Int [4]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_switch_default() {
    let src = r#"
        int main(void) {
            int a = 0;
            switch(a) {
                case 1:
                    return 1;
                case 2:
                    return 9;
                case 4:
                    a = 11;
                    break;
                default:
                    a = 22;
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
                    │       ╰── <8> Constant Int [0]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <12> Var [a]
                    │   ╰── Block
                    │       ├── Case [1]
                    │       │   ╰── Return
                    │       │       ╰── <14> Constant Int [1]
                    │       ├── Case [2]
                    │       │   ╰── Return
                    │       │       ╰── <18> Constant Int [9]
                    │       ├── Case [4]
                    │       │   ╰── <26> Assign [=]
                    │       │       ├── <23> Var [a]
                    │       │       ╰── <25> Constant Int [11]
                    │       ├── Break
                    │       ╰── Default
                    │           ╰── <34> Assign [=]
                    │               ├── <31> Var [a]
                    │               ╰── <33> Constant Int [22]
                    ╰── Return
                        ╰── <41> Var [a]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_switch_default_fallthrough() {
    let src = r#"
        int main(void) {
            int a = 5;
            switch(0) {
                default:
                    a = 0;
                case 1:
                    return a;
            }
            return a + 1;
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
                    │       ╰── <8> Constant Int [5]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <11> Constant Int [0]
                    │   ╰── Block
                    │       ├── Default
                    │       │   ╰── <16> Assign [=]
                    │       │       ├── <13> Var [a]
                    │       │       ╰── <15> Constant Int [0]
                    │       ╰── Case [1]
                    │           ╰── Return
                    │               ╰── <21> Var [a]
                    ╰── Return
                        ╰── <31>  [+]
                            ├── <28> Var [a]
                            ╰── <30> Constant Int [1]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_switch_default_not_last() {
    let src = r#"
        int main(void) {
            int a;
            int b = a = 7;
            switch (a + b) {
                default: return 0;
                case 2: return 1;
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
                    │   ╰── Type
                    │       ╰── Int
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <16> Assign [=]
                    │           ├── <13> Var [a]
                    │           ╰── <15> Constant Int [7]
                    ╰── Switch
                        ├── Expression
                        │   ╰── <24>  [+]
                        │       ├── <20> Var [a]
                        │       ╰── <23> Var [b]
                        ╰── Block
                            ├── Default
                            │   ╰── Return
                            │       ╰── <25> Constant Int [0]
                            ╰── Case [2]
                                ╰── Return
                                    ╰── <29> Constant Int [1]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_switch_default_only() {
    let src = r#"
        int main(void) {
            int a = 1;
            switch(a) default: return 1;
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
                    │       ╰── <8> Constant Int [1]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <12> Var [a]
                    │   ╰── Default
                    │       ╰── Return
                    │           ╰── <13> Constant Int [1]
                    ╰── Return
                        ╰── <17> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_switch_empty() {
    let src = r#"
        int main(void) {
            int x = 10;
            switch(x = x + 1) {
            }
            switch(x = x + 1)
            ;
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
                    │       ╰── <8> Constant Int [10]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <19> Assign [=]
                    │   │       ├── <12> Var [x]
                    │   │       ╰── <18>  [+]
                    │   │           ├── <15> Var [x]
                    │   │           ╰── <17> Constant Int [1]
                    │   ╰── Block
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <31> Assign [=]
                    │   │       ├── <24> Var [x]
                    │   │       ╰── <30>  [+]
                    │   │           ├── <27> Var [x]
                    │   │           ╰── <29> Constant Int [1]
                    │   ╰── Empty
                    ╰── Return
                        ╰── <35> Var [x]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_switch_fallthrough() {
    let src = r#"
        int main(void) {
            int a = 4;
            int b = 9;
            int c = 0;
            switch (a ? b : 7) {
                case 0:
                    return 5;
                case 7:
                    c = 1;
                case 9:
                    c = 2;
                case 1:
                    c = c + 4;
            }
            return c;
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
                    │       ╰── <8> Constant Int [4]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <14> Constant Int [9]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <20> Constant Int [0]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <28> Conditional [?]
                    │   │       ├── <24> Var [a]
                    │   │       ├── Then
                    │   │       │   ╰── <26> Var [b]
                    │   │       ╰── Else
                    │   │           ╰── <27> Constant Int [7]
                    │   ╰── Block
                    │       ├── Case [0]
                    │       │   ╰── Return
                    │       │       ╰── <30> Constant Int [5]
                    │       ├── Case [7]
                    │       │   ╰── <38> Assign [=]
                    │       │       ├── <35> Var [c]
                    │       │       ╰── <37> Constant Int [1]
                    │       ├── Case [9]
                    │       │   ╰── <46> Assign [=]
                    │       │       ├── <43> Var [c]
                    │       │       ╰── <45> Constant Int [2]
                    │       ╰── Case [1]
                    │           ╰── <58> Assign [=]
                    │               ├── <51> Var [c]
                    │               ╰── <57>  [+]
                    │                   ├── <54> Var [c]
                    │                   ╰── <56> Constant Int [4]
                    ╰── Return
                        ╰── <65> Var [c]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_switch_goto_mid_case() {
    let src = r#"
        int main(void) {
            int a = 0;
            goto mid_case;
            switch (4) {
                case 4:
                    a = 5;
                mid_case:
                    a = a + 1;
                    return a;
            }
            return 100;
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
                    │       ╰── <8> Constant Int [0]
                    ├── Goto [mid_case]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <13> Constant Int [4]
                    │   ╰── Block
                    │       ├── Case [4]
                    │       │   ╰── <19> Assign [=]
                    │       │       ├── <16> Var [a]
                    │       │       ╰── <18> Constant Int [5]
                    │       ├── Label [mid_case]
                    │       │   ╰── <31> Assign [=]
                    │       │       ├── <24> Var [a]
                    │       │       ╰── <30>  [+]
                    │       │           ├── <27> Var [a]
                    │       │           ╰── <29> Constant Int [1]
                    │       ╰── Return
                    │           ╰── <35> Var [a]
                    ╰── Return
                        ╰── <40> Constant Int [100]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_switch_in_loop() {
    let src = r#"
        int main(void) {
            int acc = 0;
            int ctr = 0;
            for (int i = 0; i < 10; i = i + 1) {
                switch(i) {
                    case 0:
                        acc = 2;
                        break;
                    case 1:
                        acc = acc * 3;
                        break;
                    case 2:
                        acc = acc * 4;
                        break;
                    default:
                        acc = acc + 1;
                }
                ctr = ctr + 1;
            }
            return ctr == 10 && acc == 31;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── acc
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <8> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ctr
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <14> Constant Int [0]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <20> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <28>  [<]
                    │   │       ├── <25> Var [i]
                    │   │       ╰── <27> Constant Int [10]
                    │   ├── Condition
                    │   │   ╰── <37> Assign [=]
                    │   │       ├── <30> Var [i]
                    │   │       ╰── <36>  [+]
                    │   │           ├── <33> Var [i]
                    │   │           ╰── <35> Constant Int [1]
                    │   ╰── Block
                    │       ├── Switch
                    │       │   ├── Expression
                    │       │   │   ╰── <39> Var [i]
                    │       │   ╰── Block
                    │       │       ├── Case [0]
                    │       │       │   ╰── <45> Assign [=]
                    │       │       │       ├── <42> Var [acc]
                    │       │       │       ╰── <44> Constant Int [2]
                    │       │       ├── Break
                    │       │       ├── Case [1]
                    │       │       │   ╰── <58> Assign [=]
                    │       │       │       ├── <51> Var [acc]
                    │       │       │       ╰── <57>  [*]
                    │       │       │           ├── <54> Var [acc]
                    │       │       │           ╰── <56> Constant Int [3]
                    │       │       ├── Break
                    │       │       ├── Case [2]
                    │       │       │   ╰── <71> Assign [=]
                    │       │       │       ├── <64> Var [acc]
                    │       │       │       ╰── <70>  [*]
                    │       │       │           ├── <67> Var [acc]
                    │       │       │           ╰── <69> Constant Int [4]
                    │       │       ├── Break
                    │       │       ╰── Default
                    │       │           ╰── <83> Assign [=]
                    │       │               ├── <76> Var [acc]
                    │       │               ╰── <82>  [+]
                    │       │                   ├── <79> Var [acc]
                    │       │                   ╰── <81> Constant Int [1]
                    │       ╰── <97> Assign [=]
                    │           ├── <90> Var [ctr]
                    │           ╰── <96>  [+]
                    │               ├── <93> Var [ctr]
                    │               ╰── <95> Constant Int [1]
                    ╰── Return
                        ╰── <113>  [&&]
                            ├── <106>  [==]
                            │   ├── <103> Var [ctr]
                            │   ╰── <105> Constant Int [10]
                            ╰── <112>  [==]
                                ├── <109> Var [acc]
                                ╰── <111> Constant Int [31]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_switch_nested_cases() {
    let src = r#"
        int main(void) {
            int switch1 = 0;
            int switch2 = 0;
            int switch3 = 0;
            switch(3) {
                case 0: return 0;
                case 1: if (0) {
                    case 3: switch1 = 1; break;
                }
                default: return 0;
            }
            switch(4) {
                case 0: return 0;
                if (1) {
                    return 0;
                } else {
                    case 4: switch2 = 1; break;
                }
                default: return 0;
            }
            switch (5) {
                for (int i = 0; i < 10; i = i + 1) {
                    switch1 = 0;
                    case 5: switch3 = 1; break;
                    default: return 0;
                }
            }
            return (switch1 && switch2 && switch3);
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── switch1
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <8> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── switch2
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <14> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── switch3
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <20> Constant Int [0]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <23> Constant Int [3]
                    │   ╰── Block
                    │       ├── Case [0]
                    │       │   ╰── Return
                    │       │       ╰── <25> Constant Int [0]
                    │       ├── Case [1]
                    │       │   ╰── If
                    │       │       ├── Condition
                    │       │       │   ╰── <29> Constant Int [0]
                    │       │       ╰── Then
                    │       │           ╰── Block
                    │       │               ├── Case [3]
                    │       │               │   ╰── <35> Assign [=]
                    │       │               │       ├── <32> Var [switch1]
                    │       │               │       ╰── <34> Constant Int [1]
                    │       │               ╰── Break
                    │       ╰── Default
                    │           ╰── Return
                    │               ╰── <43> Constant Int [0]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <49> Constant Int [4]
                    │   ╰── Block
                    │       ├── Case [0]
                    │       │   ╰── Return
                    │       │       ╰── <51> Constant Int [0]
                    │       ├── If
                    │       │   ├── Condition
                    │       │   │   ╰── <54> Constant Int [1]
                    │       │   ├── Then
                    │       │   │   ╰── Block
                    │       │   │       ╰── Return
                    │       │   │           ╰── <55> Constant Int [0]
                    │       │   ╰── Else
                    │       │       ╰── Block
                    │       │           ├── Case [4]
                    │       │           │   ╰── <64> Assign [=]
                    │       │           │       ├── <61> Var [switch2]
                    │       │           │       ╰── <63> Constant Int [1]
                    │       │           ╰── Break
                    │       ╰── Default
                    │           ╰── Return
                    │               ╰── <71> Constant Int [0]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <77> Constant Int [5]
                    │   ╰── Block
                    │       ╰── For
                    │           ├── Init
                    │           │   ╰── VarDeclaration
                    │           │       ├── Name
                    │           │       │   ╰── i
                    │           │       ├── Type
                    │           │       │   ╰── Int
                    │           │       ╰── Initializer
                    │           │           ╰── <81> Constant Int [0]
                    │           ├── Condition
                    │           │   ╰── <89>  [<]
                    │           │       ├── <86> Var [i]
                    │           │       ╰── <88> Constant Int [10]
                    │           ├── Condition
                    │           │   ╰── <98> Assign [=]
                    │           │       ├── <91> Var [i]
                    │           │       ╰── <97>  [+]
                    │           │           ├── <94> Var [i]
                    │           │           ╰── <96> Constant Int [1]
                    │           ╰── Block
                    │               ├── <103> Assign [=]
                    │               │   ├── <100> Var [switch1]
                    │               │   ╰── <102> Constant Int [0]
                    │               ├── Case [5]
                    │               │   ╰── <110> Assign [=]
                    │               │       ├── <107> Var [switch3]
                    │               │       ╰── <109> Constant Int [1]
                    │               ├── Break
                    │               ╰── Default
                    │                   ╰── Return
                    │                       ╰── <114> Constant Int [0]
                    ╰── Return
                        ╰── <133>  [&&]
                            ├── <128>  [&&]
                            │   ├── <124> Var [switch1]
                            │   ╰── <127> Var [switch2]
                            ╰── <131> Var [switch3]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_switch_nested_not_taken() {
    let src = r#"
        
        int main(void) {
            int a = 0;
            switch(a) {
                case 1:
                    switch(a) {
                        case 0: return 0;
                        default: return 0;
                    }
                default: a = 2;
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
                    │       ╰── <8> Constant Int [0]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <12> Var [a]
                    │   ╰── Block
                    │       ├── Case [1]
                    │       │   ╰── Switch
                    │       │       ├── Expression
                    │       │       │   ╰── <15> Var [a]
                    │       │       ╰── Block
                    │       │           ├── Case [0]
                    │       │           │   ╰── Return
                    │       │           │       ╰── <17> Constant Int [0]
                    │       │           ╰── Default
                    │       │               ╰── Return
                    │       │                   ╰── <20> Constant Int [0]
                    │       ╰── Default
                    │           ╰── <31> Assign [=]
                    │               ├── <28> Var [a]
                    │               ╰── <30> Constant Int [2]
                    ╰── Return
                        ╰── <38> Var [a]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_switch_nested_switch() {
    let src = r#"
        int main(void){
            switch(3) {
                case 0:
                    return 0;
                case 3: {
                    switch(4) {
                        case 3: return 0;
                        case 4: return 1;
                        default: return 0;
                    }
                }
                case 4: return 0;
                default: return 0;
            }
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Switch
                        ├── Expression
                        │   ╰── <5> Constant Int [3]
                        ╰── Block
                            ├── Case [0]
                            │   ╰── Return
                            │       ╰── <7> Constant Int [0]
                            ├── Case [3]
                            │   ╰── Block
                            │       ╰── Switch
                            │           ├── Expression
                            │           │   ╰── <11> Constant Int [4]
                            │           ╰── Block
                            │               ├── Case [3]
                            │               │   ╰── Return
                            │               │       ╰── <13> Constant Int [0]
                            │               ├── Case [4]
                            │               │   ╰── Return
                            │               │       ╰── <17> Constant Int [1]
                            │               ╰── Default
                            │                   ╰── Return
                            │                       ╰── <20> Constant Int [0]
                            ├── Case [4]
                            │   ╰── Return
                            │       ╰── <30> Constant Int [0]
                            ╰── Default
                                ╰── Return
                                    ╰── <33> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_switch_no_case() {
    let src = r#"
        int main(void) {
            int a = 4;
            switch(a)
                return 0;
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
                    │       ╰── <8> Constant Int [4]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <12> Var [a]
                    │   ╰── Return
                    │       ╰── <13> Constant Int [0]
                    ╰── Return
                        ╰── <17> Var [a]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_switch_not_taken() {
    let src = r#"
        int main(void) {
            int a = 1;
            switch(a) {
                case 0: return 0;
                case 2: return 0;
                case 3: return 0;
            }
            return 1;
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
                    │       ╰── <8> Constant Int [1]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <12> Var [a]
                    │   ╰── Block
                    │       ├── Case [0]
                    │       │   ╰── Return
                    │       │       ╰── <14> Constant Int [0]
                    │       ├── Case [2]
                    │       │   ╰── Return
                    │       │       ╰── <18> Constant Int [0]
                    │       ╰── Case [3]
                    │           ╰── Return
                    │               ╰── <22> Constant Int [0]
                    ╰── Return
                        ╰── <28> Constant Int [1]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_switch_single_case() {
    let src = r#"
        int main(void) {
            int a = 1;
            switch(a) case 1: return 1;
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
                    │       ╰── <8> Constant Int [1]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <12> Var [a]
                    │   ╰── Case [1]
                    │       ╰── Return
                    │           ╰── <14> Constant Int [1]
                    ╰── Return
                        ╰── <18> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_switch_with_continue() {
    let src = r#"
        int main(void) {
            switch(4) {
                case 0:
                    return 0;
                case 4: {
                    int acc = 0;
                    for (int i = 0; i < 10; i = i + 1) {
                        if (i % 2)
                            continue;
                        acc = acc + 1;
                    }
                    return acc;
                }
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <5> Constant Int [4]
                    │   ╰── Block
                    │       ├── Case [0]
                    │       │   ╰── Return
                    │       │       ╰── <7> Constant Int [0]
                    │       ╰── Case [4]
                    │           ╰── Block
                    │               ├── VarDeclaration
                    │               │   ├── Name
                    │               │   │   ╰── acc
                    │               │   ├── Type
                    │               │   │   ╰── Int
                    │               │   ╰── Initializer
                    │               │       ╰── <14> Constant Int [0]
                    │               ├── For
                    │               │   ├── Init
                    │               │   │   ╰── VarDeclaration
                    │               │   │       ├── Name
                    │               │   │       │   ╰── i
                    │               │   │       ├── Type
                    │               │   │       │   ╰── Int
                    │               │   │       ╰── Initializer
                    │               │   │           ╰── <20> Constant Int [0]
                    │               │   ├── Condition
                    │               │   │   ╰── <28>  [<]
                    │               │   │       ├── <25> Var [i]
                    │               │   │       ╰── <27> Constant Int [10]
                    │               │   ├── Condition
                    │               │   │   ╰── <37> Assign [=]
                    │               │   │       ├── <30> Var [i]
                    │               │   │       ╰── <36>  [+]
                    │               │   │           ├── <33> Var [i]
                    │               │   │           ╰── <35> Constant Int [1]
                    │               │   ╰── Block
                    │               │       ├── If
                    │               │       │   ├── Condition
                    │               │       │   │   ╰── <42>  [%]
                    │               │       │   │       ├── <39> Var [i]
                    │               │       │   │       ╰── <41> Constant Int [2]
                    │               │       │   ╰── Then
                    │               │       │       ╰── Continue
                    │               │       ╰── <53> Assign [=]
                    │               │           ├── <46> Var [acc]
                    │               │           ╰── <52>  [+]
                    │               │               ├── <49> Var [acc]
                    │               │               ╰── <51> Constant Int [1]
                    │               ╰── Return
                    │                   ╰── <59> Var [acc]
                    ╰── Return
                        ╰── <67> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_switch_with_continue_2() {
    let src = r#"
        int main(void) {
            int sum = 0;
            for (int i = 0; i < 10; i = i + 1) {
                switch(i % 2) {
                    case 0: continue;
                    default: sum = sum + 1;
                }
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
                    │       ╰── <8> Constant Int [0]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <14> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <22>  [<]
                    │   │       ├── <19> Var [i]
                    │   │       ╰── <21> Constant Int [10]
                    │   ├── Condition
                    │   │   ╰── <31> Assign [=]
                    │   │       ├── <24> Var [i]
                    │   │       ╰── <30>  [+]
                    │   │           ├── <27> Var [i]
                    │   │           ╰── <29> Constant Int [1]
                    │   ╰── Block
                    │       ╰── Switch
                    │           ├── Expression
                    │           │   ╰── <36>  [%]
                    │           │       ├── <33> Var [i]
                    │           │       ╰── <35> Constant Int [2]
                    │           ╰── Block
                    │               ├── Case [0]
                    │               │   ╰── Continue
                    │               ╰── Default
                    │                   ╰── <48> Assign [=]
                    │                       ├── <41> Var [sum]
                    │                       ╰── <47>  [+]
                    │                           ├── <44> Var [sum]
                    │                           ╰── <46> Constant Int [1]
                    ╰── Return
                        ╰── <58> Var [sum]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_for() {
    let src = r#"
        int main(void) {
            int a = 12345;
            int i;
            for (i = 5; i >= 0; i = i - 1)
                a = a / 3;
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
                    │       ╰── <8> Constant Int [12345]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ╰── Type
                    │       ╰── Int
                    ├── For
                    │   ├── Init
                    │   │   ╰── <19> Assign [=]
                    │   │       ├── <16> Var [i]
                    │   │       ╰── <18> Constant Int [5]
                    │   ├── Condition
                    │   │   ╰── <24>  [>=]
                    │   │       ├── <21> Var [i]
                    │   │       ╰── <23> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <33> Assign [=]
                    │   │       ├── <26> Var [i]
                    │   │       ╰── <32>  [-]
                    │   │           ├── <29> Var [i]
                    │   │           ╰── <31> Constant Int [1]
                    │   ╰── <42> Assign [=]
                    │       ├── <35> Var [a]
                    │       ╰── <41>  [/]
                    │           ├── <38> Var [a]
                    │           ╰── <40> Constant Int [3]
                    ╰── Return
                        ╰── <46> Var [a]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_for_absent_condition() {
    let src = r#"
        int main(void) {
            for (int i = 400; ; i = i - 100)
                if (i == 100)
                    return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── For
                        ├── Init
                        │   ╰── VarDeclaration
                        │       ├── Name
                        │       │   ╰── i
                        │       ├── Type
                        │       │   ╰── Int
                        │       ╰── Initializer
                        │           ╰── <8> Constant Int [400]
                        ├── Condition
                        │   ╰── <20> Assign [=]
                        │       ├── <13> Var [i]
                        │       ╰── <19>  [-]
                        │           ├── <16> Var [i]
                        │           ╰── <18> Constant Int [100]
                        ╰── If
                            ├── Condition
                            │   ╰── <25>  [==]
                            │       ├── <22> Var [i]
                            │       ╰── <24> Constant Int [100]
                            ╰── Then
                                ╰── Return
                                    ╰── <26> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_for_absent_post() {
    let src = r#"
        int main(void) {
            int a = -2147;
            for (; a % 5 != 0;) {
                a = a + 1;
            }
            return a % 5 || a > 0;
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
                    │       ╰── <10> Unary [-]
                    │           ╰── <9> Constant Int [2147]
                    ├── For
                    │   ├── Condition
                    │   │   ╰── <20>  [!=]
                    │   │       ├── <17>  [%]
                    │   │       │   ├── <14> Var [a]
                    │   │       │   ╰── <16> Constant Int [5]
                    │   │       ╰── <19> Constant Int [0]
                    │   ╰── Block
                    │       ╰── <29> Assign [=]
                    │           ├── <22> Var [a]
                    │           ╰── <28>  [+]
                    │               ├── <25> Var [a]
                    │               ╰── <27> Constant Int [1]
                    ╰── Return
                        ╰── <45>  [||]
                            ├── <38>  [%]
                            │   ├── <35> Var [a]
                            │   ╰── <37> Constant Int [5]
                            ╰── <44>  [>]
                                ├── <41> Var [a]
                                ╰── <43> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_for_decl() {
    let src = r#"
        int main(void) {
            int a = 0;
            for (int i = -100; i <= 0; i = i + 1)
                a = a + 1;
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
                    │       ╰── <8> Constant Int [0]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <16> Unary [-]
                    │   │               ╰── <15> Constant Int [100]
                    │   ├── Condition
                    │   │   ╰── <24>  [<=]
                    │   │       ├── <21> Var [i]
                    │   │       ╰── <23> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <33> Assign [=]
                    │   │       ├── <26> Var [i]
                    │   │       ╰── <32>  [+]
                    │   │           ├── <29> Var [i]
                    │   │           ╰── <31> Constant Int [1]
                    │   ╰── <42> Assign [=]
                    │       ├── <35> Var [a]
                    │       ╰── <41>  [+]
                    │           ├── <38> Var [a]
                    │           ╰── <40> Constant Int [1]
                    ╰── Return
                        ╰── <46> Var [a]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_for_nested_shadow() {
    let src = r#"
        int main(void) {
            int i = 0;
            int j = 0;
            int k = 1;
            for (int i = 100; i > 0; i = i - 1) {
                int i = 1;
                int j = i + k;
                k = j;
            }
            return k == 101 && i == 0 && j == 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <8> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── j
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <14> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── k
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <20> Constant Int [1]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <26> Constant Int [100]
                    │   ├── Condition
                    │   │   ╰── <34>  [>]
                    │   │       ├── <31> Var [i]
                    │   │       ╰── <33> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <43> Assign [=]
                    │   │       ├── <36> Var [i]
                    │   │       ╰── <42>  [-]
                    │   │           ├── <39> Var [i]
                    │   │           ╰── <41> Constant Int [1]
                    │   ╰── Block
                    │       ├── VarDeclaration
                    │       │   ├── Name
                    │       │   │   ╰── i
                    │       │   ├── Type
                    │       │   │   ╰── Int
                    │       │   ╰── Initializer
                    │       │       ╰── <47> Constant Int [1]
                    │       ├── VarDeclaration
                    │       │   ├── Name
                    │       │   │   ╰── j
                    │       │   ├── Type
                    │       │   │   ╰── Int
                    │       │   ╰── Initializer
                    │       │       ╰── <58>  [+]
                    │       │           ├── <54> Var [i]
                    │       │           ╰── <57> Var [k]
                    │       ╰── <66> Assign [=]
                    │           ├── <62> Var [k]
                    │           ╰── <65> Var [j]
                    ╰── Return
                        ╰── <89>  [&&]
                            ├── <82>  [&&]
                            │   ├── <75>  [==]
                            │   │   ├── <72> Var [k]
                            │   │   ╰── <74> Constant Int [101]
                            │   ╰── <81>  [==]
                            │       ├── <78> Var [i]
                            │       ╰── <80> Constant Int [0]
                            ╰── <88>  [==]
                                ├── <85> Var [j]
                                ╰── <87> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_for_shadow() {
    let src = r#"
        int main(void) {
            int shadow = 1;
            int acc = 0;
            for (int shadow = 0; shadow < 10; shadow = shadow + 1) {
                acc = acc + shadow;
            }
            return acc == 45 && shadow == 1;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── shadow
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <8> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── acc
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <14> Constant Int [0]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── shadow
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <20> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <28>  [<]
                    │   │       ├── <25> Var [shadow]
                    │   │       ╰── <27> Constant Int [10]
                    │   ├── Condition
                    │   │   ╰── <37> Assign [=]
                    │   │       ├── <30> Var [shadow]
                    │   │       ╰── <36>  [+]
                    │   │           ├── <33> Var [shadow]
                    │   │           ╰── <35> Constant Int [1]
                    │   ╰── Block
                    │       ╰── <47> Assign [=]
                    │           ├── <39> Var [acc]
                    │           ╰── <46>  [+]
                    │               ├── <42> Var [acc]
                    │               ╰── <45> Var [shadow]
                    ╰── Return
                        ╰── <63>  [&&]
                            ├── <56>  [==]
                            │   ├── <53> Var [acc]
                            │   ╰── <55> Constant Int [45]
                            ╰── <62>  [==]
                                ├── <59> Var [shadow]
                                ╰── <61> Constant Int [1]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_multi_break() {
    let src = r#"
        int main(void) {
            int i = 0;
            while (1) {
                i = i + 1;
                if (i > 10)
                    break;
            }
            int j = 10;
            while (1) {
                j = j - 1;
                if (j < 0)
                    break;
            }
            int result = j == -1 && i == 11;
            return result;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <8> Constant Int [0]
                    ├── While
                    │   ├── Condition
                    │   │   ╰── <11> Constant Int [1]
                    │   ╰── Body
                    │       ╰── Block
                    │           ├── <20> Assign [=]
                    │           │   ├── <13> Var [i]
                    │           │   ╰── <19>  [+]
                    │           │       ├── <16> Var [i]
                    │           │       ╰── <18> Constant Int [1]
                    │           ╰── If
                    │               ├── Condition
                    │               │   ╰── <26>  [>]
                    │               │       ├── <23> Var [i]
                    │               │       ╰── <25> Constant Int [10]
                    │               ╰── Then
                    │                   ╰── Break
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── j
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <35> Constant Int [10]
                    ├── While
                    │   ├── Condition
                    │   │   ╰── <38> Constant Int [1]
                    │   ╰── Body
                    │       ╰── Block
                    │           ├── <47> Assign [=]
                    │           │   ├── <40> Var [j]
                    │           │   ╰── <46>  [-]
                    │           │       ├── <43> Var [j]
                    │           │       ╰── <45> Constant Int [1]
                    │           ╰── If
                    │               ├── Condition
                    │               │   ╰── <53>  [<]
                    │               │       ├── <50> Var [j]
                    │               │       ╰── <52> Constant Int [0]
                    │               ╰── Then
                    │                   ╰── Break
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── result
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <75>  [&&]
                    │           ├── <68>  [==]
                    │           │   ├── <63> Var [j]
                    │           │   ╰── <67> Unary [-]
                    │           │       ╰── <66> Constant Int [1]
                    │           ╰── <74>  [==]
                    │               ├── <71> Var [i]
                    │               ╰── <73> Constant Int [11]
                    ╰── Return
                        ╰── <79> Var [result]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_multi_continue_same_loop() {
    let src = r#"
        int main(void) {
            int x = 10;
            int y = 0;
            int z = 0;
            do {
                z = z + 1;
                if (x <= 0)
                    continue;
                x = x - 1;
                if (y >= 10)
                    continue;
                y = y + 1;
            } while (z != 50);
            return z == 50 && x == 0 && y == 10;
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
                    │       ╰── <8> Constant Int [10]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── y
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <14> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── z
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <20> Constant Int [0]
                    ├── DoWhile
                    │   ├── Body
                    │   │   ╰── Block
                    │   │       ├── <31> Assign [=]
                    │   │       │   ├── <24> Var [z]
                    │   │       │   ╰── <30>  [+]
                    │   │       │       ├── <27> Var [z]
                    │   │       │       ╰── <29> Constant Int [1]
                    │   │       ├── If
                    │   │       │   ├── Condition
                    │   │       │   │   ╰── <37>  [<=]
                    │   │       │   │       ├── <34> Var [x]
                    │   │       │   │       ╰── <36> Constant Int [0]
                    │   │       │   ╰── Then
                    │   │       │       ╰── Continue
                    │   │       ├── <48> Assign [=]
                    │   │       │   ├── <41> Var [x]
                    │   │       │   ╰── <47>  [-]
                    │   │       │       ├── <44> Var [x]
                    │   │       │       ╰── <46> Constant Int [1]
                    │   │       ├── If
                    │   │       │   ├── Condition
                    │   │       │   │   ╰── <54>  [>=]
                    │   │       │   │       ├── <51> Var [y]
                    │   │       │   │       ╰── <53> Constant Int [10]
                    │   │       │   ╰── Then
                    │   │       │       ╰── Continue
                    │   │       ╰── <65> Assign [=]
                    │   │           ├── <58> Var [y]
                    │   │           ╰── <64>  [+]
                    │   │               ├── <61> Var [y]
                    │   │               ╰── <63> Constant Int [1]
                    │   ╰── Condition
                    │       ╰── <73>  [!=]
                    │           ├── <70> Var [z]
                    │           ╰── <72> Constant Int [50]
                    ╰── Return
                        ╰── <93>  [&&]
                            ├── <86>  [&&]
                            │   ├── <79>  [==]
                            │   │   ├── <76> Var [z]
                            │   │   ╰── <78> Constant Int [50]
                            │   ╰── <85>  [==]
                            │       ├── <82> Var [x]
                            │       ╰── <84> Constant Int [0]
                            ╰── <92>  [==]
                                ├── <89> Var [y]
                                ╰── <91> Constant Int [10]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_nested_break() {
    let src = r#"
        int main(void) {
            int ans = 0;
            for (int i = 0; i < 10; i = i + 1)
                for (int j = 0; j < 10; j = j + 1)
                    if ((i / 2)*2 == i)
                        break;
                    else
                        ans = ans + i;
            return ans;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ans
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <8> Constant Int [0]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <14> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <22>  [<]
                    │   │       ├── <19> Var [i]
                    │   │       ╰── <21> Constant Int [10]
                    │   ├── Condition
                    │   │   ╰── <31> Assign [=]
                    │   │       ├── <24> Var [i]
                    │   │       ╰── <30>  [+]
                    │   │           ├── <27> Var [i]
                    │   │           ╰── <29> Constant Int [1]
                    │   ╰── For
                    │       ├── Init
                    │       │   ╰── VarDeclaration
                    │       │       ├── Name
                    │       │       │   ╰── j
                    │       │       ├── Type
                    │       │       │   ╰── Int
                    │       │       ╰── Initializer
                    │       │           ╰── <35> Constant Int [0]
                    │       ├── Condition
                    │       │   ╰── <43>  [<]
                    │       │       ├── <40> Var [j]
                    │       │       ╰── <42> Constant Int [10]
                    │       ├── Condition
                    │       │   ╰── <52> Assign [=]
                    │       │       ├── <45> Var [j]
                    │       │       ╰── <51>  [+]
                    │       │           ├── <48> Var [j]
                    │       │           ╰── <50> Constant Int [1]
                    │       ╰── If
                    │           ├── Condition
                    │           │   ╰── <65>  [==]
                    │           │       ├── <61>  [*]
                    │           │       │   ├── <58>  [/]
                    │           │       │   │   ├── <54> Var [i]
                    │           │       │   │   ╰── <56> Constant Int [2]
                    │           │       │   ╰── <60> Constant Int [2]
                    │           │       ╰── <64> Var [i]
                    │           ├── Then
                    │           │   ╰── Break
                    │           ╰── Else
                    │               ╰── <76> Assign [=]
                    │                   ├── <68> Var [ans]
                    │                   ╰── <75>  [+]
                    │                       ├── <71> Var [ans]
                    │                       ╰── <74> Var [i]
                    ╰── Return
                        ╰── <82> Var [ans]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_nested_continue() {
    let src = r#"
        int main(void) {
            int x = 5;
            int acc = 0;
            while (x >= 0) {
                int i = x;
                while (i <= 10) {
                    i = i + 1;
                    if (i % 2)
                        continue;
                    acc = acc + 1;
                }
                x = x - 1;
            }
            return acc;
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
                    │       ╰── <8> Constant Int [5]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── acc
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <14> Constant Int [0]
                    ├── While
                    │   ├── Condition
                    │   │   ╰── <21>  [>=]
                    │   │       ├── <18> Var [x]
                    │   │       ╰── <20> Constant Int [0]
                    │   ╰── Body
                    │       ╰── Block
                    │           ├── VarDeclaration
                    │           │   ├── Name
                    │           │   │   ╰── i
                    │           │   ├── Type
                    │           │   │   ╰── Int
                    │           │   ╰── Initializer
                    │           │       ╰── <26> Var [x]
                    │           ├── While
                    │           │   ├── Condition
                    │           │   │   ╰── <33>  [<=]
                    │           │   │       ├── <30> Var [i]
                    │           │   │       ╰── <32> Constant Int [10]
                    │           │   ╰── Body
                    │           │       ╰── Block
                    │           │           ├── <42> Assign [=]
                    │           │           │   ├── <35> Var [i]
                    │           │           │   ╰── <41>  [+]
                    │           │           │       ├── <38> Var [i]
                    │           │           │       ╰── <40> Constant Int [1]
                    │           │           ├── If
                    │           │           │   ├── Condition
                    │           │           │   │   ╰── <48>  [%]
                    │           │           │   │       ├── <45> Var [i]
                    │           │           │   │       ╰── <47> Constant Int [2]
                    │           │           │   ╰── Then
                    │           │           │       ╰── Continue
                    │           │           ╰── <59> Assign [=]
                    │           │               ├── <52> Var [acc]
                    │           │               ╰── <58>  [+]
                    │           │                   ├── <55> Var [acc]
                    │           │                   ╰── <57> Constant Int [1]
                    │           ╰── <72> Assign [=]
                    │               ├── <65> Var [x]
                    │               ╰── <71>  [-]
                    │                   ├── <68> Var [x]
                    │                   ╰── <70> Constant Int [1]
                    ╰── Return
                        ╰── <78> Var [acc]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_nested_loop() {
    let src = r#"
        int main(void) {
            int acc = 0;
            int x = 100;
            while (x) {
                int y = 10;
                x = x - y;
                while (y) {
                    acc = acc + 1;
                    y = y - 1;
                }
            }
            return acc == 100 && x == 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── acc
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <8> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <14> Constant Int [100]
                    ├── While
                    │   ├── Condition
                    │   │   ╰── <18> Var [x]
                    │   ╰── Body
                    │       ╰── Block
                    │           ├── VarDeclaration
                    │           │   ├── Name
                    │           │   │   ╰── y
                    │           │   ├── Type
                    │           │   │   ╰── Int
                    │           │   ╰── Initializer
                    │           │       ╰── <22> Constant Int [10]
                    │           ├── <34> Assign [=]
                    │           │   ├── <26> Var [x]
                    │           │   ╰── <33>  [-]
                    │           │       ├── <29> Var [x]
                    │           │       ╰── <32> Var [y]
                    │           ╰── While
                    │               ├── Condition
                    │               │   ╰── <37> Var [y]
                    │               ╰── Body
                    │                   ╰── Block
                    │                       ├── <46> Assign [=]
                    │                       │   ├── <39> Var [acc]
                    │                       │   ╰── <45>  [+]
                    │                       │       ├── <42> Var [acc]
                    │                       │       ╰── <44> Constant Int [1]
                    │                       ╰── <56> Assign [=]
                    │                           ├── <49> Var [y]
                    │                           ╰── <55>  [-]
                    │                               ├── <52> Var [y]
                    │                               ╰── <54> Constant Int [1]
                    ╰── Return
                        ╰── <75>  [&&]
                            ├── <68>  [==]
                            │   ├── <65> Var [acc]
                            │   ╰── <67> Constant Int [100]
                            ╰── <74>  [==]
                                ├── <71> Var [x]
                                ╰── <73> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_null_for_header() {
    let src = r#"
        int main(void) {
            int a = 0;
            for (; ; ) {
                a = a + 1;
                if (a > 3)
                    break;
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
                    │       ╰── <8> Constant Int [0]
                    ├── For
                    │   ╰── Block
                    │       ├── <19> Assign [=]
                    │       │   ├── <12> Var [a]
                    │       │   ╰── <18>  [+]
                    │       │       ├── <15> Var [a]
                    │       │       ╰── <17> Constant Int [1]
                    │       ╰── If
                    │           ├── Condition
                    │           │   ╰── <25>  [>]
                    │           │       ├── <22> Var [a]
                    │           │       ╰── <24> Constant Int [3]
                    │           ╰── Then
                    │               ╰── Break
                    ╰── Return
                        ╰── <32> Var [a]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_while() {
    let src = r#"
        int main(void) {
            int a = 0;
            while (a < 5)
                a = a + 2;
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
                    │       ╰── <8> Constant Int [0]
                    ├── While
                    │   ├── Condition
                    │   │   ╰── <15>  [<]
                    │   │       ├── <12> Var [a]
                    │   │       ╰── <14> Constant Int [5]
                    │   ╰── Body
                    │       ╰── <24> Assign [=]
                    │           ├── <17> Var [a]
                    │           ╰── <23>  [+]
                    │               ├── <20> Var [a]
                    │               ╰── <22> Constant Int [2]
                    ╰── Return
                        ╰── <28> Var [a]
    "#;
    assert_parse(src, expected);
}
