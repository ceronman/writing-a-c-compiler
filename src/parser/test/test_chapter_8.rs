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
                        │   ╰── <6> Constant Int [1]
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
                        ╰── <13> Constant Int [0]
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
                    │       ╰── <9> Constant Int [3]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <16>  [+]
                    │   │       ├── <13> Var [a]
                    │   │       ╰── <15> Constant Int [1]
                    │   ╰── Block
                    │       ├── Case [0]
                    │       │   ╰── Continue
                    │       ╰── Default
                    │           ╰── <24> Assign [=]
                    │               ├── <21> Var [a]
                    │               ╰── <23> Constant Int [1]
                    ╰── Return
                        ╰── <31> Var [a]
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
                    │   │           ╰── <9> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <17>  [<]
                    │   │       ├── <14> Var [i]
                    │   │       ╰── <16> Constant Int [10]
                    │   ├── Condition
                    │   │   ╰── <26> Assign [=]
                    │   │       ├── <19> Var [i]
                    │   │       ╰── <25>  [+]
                    │   │           ├── <22> Var [i]
                    │   │           ╰── <24> Constant Int [1]
                    │   ╰── Block
                    │       ╰── Case [0]
                    │           ╰── Return
                    │               ╰── <28> Constant Int [1]
                    ╰── Return
                        ╰── <34> Constant Int [9]
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
                    │       ╰── <9> Constant Int [3]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <16>  [+]
                    │   │       ├── <13> Var [a]
                    │   │       ╰── <15> Constant Int [1]
                    │   ╰── Block
                    │       ├── Case [0]
                    │       │   ╰── <22> Assign [=]
                    │       │       ├── <19> Var [a]
                    │       │       ╰── <21> Constant Int [1]
                    │       ╰── Default
                    │           ╰── Continue
                    ╰── Return
                        ╰── <31> Var [a]
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
                                ╰── <6> Constant Int [0]
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
                    │       ╰── <9> Constant Int [1]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <13> Var [a]
                    │   ╰── Block
                    │       ├── Case [1]
                    │       │   ╰── Empty
                    │       ├── VarDeclaration
                    │       │   ├── Name
                    │       │   │   ╰── b
                    │       │   ├── Type
                    │       │   │   ╰── Int
                    │       │   ╰── Initializer
                    │       │       ╰── <20> Constant Int [10]
                    │       ├── Break
                    │       ├── Case [2]
                    │       │   ╰── Empty
                    │       ├── VarDeclaration
                    │       │   ├── Name
                    │       │   │   ╰── b
                    │       │   ├── Type
                    │       │   │   ╰── Int
                    │       │   ╰── Initializer
                    │       │       ╰── <30> Constant Int [11]
                    │       ├── Break
                    │       ╰── Default
                    │           ╰── Break
                    ╰── Return
                        ╰── <39> Constant Int [0]
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
                        │   ╰── <6> Constant Int [4]
                        ╰── Block
                            ├── Case [5]
                            │   ╰── Return
                            │       ╰── <8> Constant Int [0]
                            ├── Case [4]
                            │   ╰── Return
                            │       ╰── <12> Constant Int [1]
                            ├── Case [5]
                            │   ╰── Return
                            │       ╰── <16> Constant Int [0]
                            ╰── Default
                                ╰── Return
                                    ╰── <19> Constant Int [2]
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
                    │       ╰── <9> Constant Int [0]
                    ├── Label [label]
                    │   ╰── Switch
                    │       ├── Expression
                    │       │   ╰── <14> Var [a]
                    │       ╰── Block
                    │           ╰── Case [1]
                    │               ╰── Case [1]
                    │                   ╰── Break
                    ╰── Return
                        ╰── <24> Constant Int [0]
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
                    │       ╰── <9> Constant Int [10]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <13> Var [a]
                    │   ╰── Block
                    │       ╰── Case [1]
                    │           ╰── Block
                    │               ╰── If
                    │                   ├── Condition
                    │                   │   ╰── <15> Constant Int [1]
                    │                   ╰── Then
                    │                       ╰── Block
                    │                           ╰── Case [1]
                    │                               ╰── Return
                    │                                   ╰── <17> Constant Int [0]
                    ╰── Return
                        ╰── <29> Constant Int [0]
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
                    │       ╰── <9> Constant Int [0]
                    ╰── Switch
                        ├── Expression
                        │   ╰── <13> Var [a]
                        ╰── Block
                            ├── Case [0]
                            │   ╰── Return
                            │       ╰── <15> Constant Int [0]
                            ├── Default
                            │   ╰── Return
                            │       ╰── <18> Constant Int [1]
                            ├── Case [2]
                            │   ╰── Return
                            │       ╰── <22> Constant Int [2]
                            ╰── Default
                                ╰── Return
                                    ╰── <25> Constant Int [2]
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
                    │       ╰── <9> Constant Int [10]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <13> Var [a]
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
                    │       │       │           ╰── <18> Constant Int [0]
                    │       │       ├── Condition
                    │       │       │   ╰── <26>  [<]
                    │       │       │       ├── <23> Var [i]
                    │       │       │       ╰── <25> Constant Int [10]
                    │       │       ├── Condition
                    │       │       │   ╰── <35> Assign [=]
                    │       │       │       ├── <28> Var [i]
                    │       │       │       ╰── <34>  [+]
                    │       │       │           ├── <31> Var [i]
                    │       │       │           ╰── <33> Constant Int [1]
                    │       │       ╰── Block
                    │       │           ├── Continue
                    │       │           ╰── While
                    │       │               ├── Condition
                    │       │               │   ╰── <37> Constant Int [1]
                    │       │               ╰── Body
                    │       │                   ╰── Default
                    │       │                       ╰── Empty
                    │       ├── Case [2]
                    │       │   ╰── Return
                    │       │       ╰── <46> Constant Int [0]
                    │       ╰── Default
                    │           ╰── Empty
                    ╰── Return
                        ╰── <54> Constant Int [0]
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
                    │       ╰── <9> Constant Int [1]
                    ├── Label [label]
                    │   ╰── Switch
                    │       ├── Expression
                    │       │   ╰── <14> Var [a]
                    │       ╰── Block
                    │           ├── Case [1]
                    │           │   ╰── Return
                    │           │       ╰── <16> Constant Int [0]
                    │           ╰── Default
                    │               ╰── Label [label]
                    │                   ╰── Return
                    │                       ╰── <20> Constant Int [1]
                    ╰── Return
                        ╰── <28> Constant Int [0]
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
                    │   │       │       ╰── <7> Constant Int [1]
                    │   │       ╰── Label [lbl]
                    │   │           ╰── Return
                    │   │               ╰── <11> Constant Int [2]
                    │   ╰── Condition
                    │       ╰── <16> Constant Int [1]
                    ╰── Return
                        ╰── <18> Constant Int [0]
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
                    │       ╰── <9> Constant Int [1]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <13> Var [a]
                    │   ╰── Block
                    │       ├── VarDeclaration
                    │       │   ├── Name
                    │       │   │   ╰── b
                    │       │   ├── Type
                    │       │   │   ╰── Int
                    │       │   ╰── Initializer
                    │       │       ╰── <17> Constant Int [2]
                    │       ├── Case [0]
                    │       │   ╰── <25> Assign [=]
                    │       │       ├── <22> Var [a]
                    │       │       ╰── <24> Constant Int [3]
                    │       ╰── VarDeclaration
                    │           ├── Name
                    │           │   ╰── b
                    │           ├── Type
                    │           │   ╰── Int
                    │           ╰── Initializer
                    │               ╰── <31> Constant Int [2]
                    ╰── Return
                        ╰── <37> Constant Int [0]
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
                        ╰── <9> Constant Int [0]
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
                    │       ╰── <9> Constant Int [3]
                    ╰── Switch
                        ├── Expression
                        │   ╰── <16>  [+]
                        │       ├── <13> Var [a]
                        │       ╰── <15> Constant Int [1]
                        ╰── Block
                            ├── Case [0]
                            │   ╰── Return
                            │       ╰── <18> Constant Int [0]
                            ├── Case [invalid]
                            │   ├── Value
                            │   │   ╰── <22> Var [a]
                            │   ╰── Return
                            │       ╰── <23> Constant Int [1]
                            ╰── Case [1]
                                ╰── Return
                                    ╰── <27> Constant Int [2]
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
                    │       ╰── <9> Constant Int [3]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <16>  [+]
                    │   │       ├── <13> Var [a]
                    │   │       ╰── <15> Constant Int [1]
                    │   ╰── Block
                    │       ├── Case [0]
                    │       │   ╰── <22> Assign [=]
                    │       │       ├── <19> Var [a]
                    │       │       ╰── <21> Constant Int [4]
                    │       ├── Continue
                    │       ╰── Default
                    │           ╰── <30> Assign [=]
                    │               ├── <27> Var [a]
                    │               ╰── <29> Constant Int [1]
                    ╰── Return
                        ╰── <37> Var [a]
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
                    │   │   ╰── <7> Var [a]
                    │   ╰── Block
                    │       ├── Case [1]
                    │       │   ╰── Return
                    │       │       ╰── <9> Constant Int [0]
                    │       ╰── Case [2]
                    │           ╰── Return
                    │               ╰── <13> Constant Int [1]
                    ╰── Return
                        ╰── <19> Constant Int [0]
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
                    │       ╰── <9> Constant Int [10]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <13> Var [a]
                    │   ╰── Block
                    │       ├── Case [1]
                    │       │   ╰── Return
                    │       │       ╰── <16> Var [b]
                    │       ├── Break
                    │       ╰── Default
                    │           ╰── Break
                    ╰── Return
                        ╰── <25> Constant Int [0]
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
                    │       ╰── <9> Constant Int [10]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <13> Var [a]
                    │   ╰── Block
                    │       ├── Case [1]
                    │       │   ╰── Break
                    │       ├── Default
                    │       │   ╰── Return
                    │       │       ╰── <18> Var [b]
                    │       ╰── Break
                    ╰── Return
                        ╰── <25> Constant Int [0]
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
                    │       ╰── <9> Constant Int [3]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <13> Var [a]
                    │   ╰── Block
                    │       ├── Case [1]
                    │       │   ╰── Goto [foo]
                    │       ╰── Default
                    │           ╰── Return
                    │               ╰── <18> Constant Int [0]
                    ╰── Return
                        ╰── <24> Constant Int [0]
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
                        │               ╰── <13>  [+]
                        │                   ├── <10> Var [a]
                        │                   ╰── <12> Constant Int [1]
                        ╰── Condition
                            ╰── <22>  [<]
                                ├── <19> Var [a]
                                ╰── <21> Constant Int [100]
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
                        │   ╰── <10> Assign [=]
                        │       ├── <7> Var [i]
                        │       ╰── <9> Constant Int [0]
                        ├── Condition
                        │   ╰── <15>  [<]
                        │       ├── <12> Var [i]
                        │       ╰── <14> Constant Int [1]
                        ├── Condition
                        │   ╰── <24> Assign [=]
                        │       ├── <17> Var [i]
                        │       ╰── <23>  [+]
                        │           ├── <20> Var [i]
                        │           ╰── <22> Constant Int [1]
                        ╰── Block
                            ╰── Return
                                ╰── <25> Constant Int [0]
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
                    │       ╰── <9> Constant Int [10]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [20]
                    ├── For
                    │   ├── Init
                    │   │   ╰── <24> Assign [=]
                    │   │       ├── <19> Var [b]
                    │   │       ╰── <23> Unary [-]
                    │   │           ╰── <22> Constant Int [20]
                    │   ├── Condition
                    │   │   ╰── <29>  [<]
                    │   │       ├── <26> Var [b]
                    │   │       ╰── <28> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <38> Assign [=]
                    │   │       ├── <31> Var [b]
                    │   │       ╰── <37>  [+]
                    │   │           ├── <34> Var [b]
                    │   │           ╰── <36> Constant Int [1]
                    │   ╰── Block
                    │       ├── <47> Assign [=]
                    │       │   ├── <40> Var [a]
                    │       │   ╰── <46>  [-]
                    │       │       ├── <43> Var [a]
                    │       │       ╰── <45> Constant Int [1]
                    │       ╰── If
                    │           ├── Condition
                    │           │   ╰── <53>  [<=]
                    │           │       ├── <50> Var [a]
                    │           │       ╰── <52> Constant Int [0]
                    │           ╰── Then
                    │               ╰── Break
                    ╰── Return
                        ╰── <72>  [&&]
                            ├── <63>  [==]
                            │   ├── <60> Var [a]
                            │   ╰── <62> Constant Int [0]
                            ╰── <71>  [==]
                                ├── <66> Var [b]
                                ╰── <70> Unary [-]
                                    ╰── <69> Constant Int [11]
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
                    │       ╰── <9> Constant Int [10]
                    ├── While
                    │   ├── Condition
                    │   │   ╰── <17> Assign [=]
                    │   │       ├── <13> Var [a]
                    │   │       ╰── <15> Constant Int [1]
                    │   ╰── Body
                    │       ╰── Break
                    ╰── Return
                        ╰── <21> Var [a]
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
                    │       ╰── <9> Constant Int [0]
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
                    │   │           ╰── <19> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <27>  [<=]
                    │   │       ├── <24> Var [i]
                    │   │       ╰── <26> Constant Int [10]
                    │   ├── Condition
                    │   │   ╰── <36> Assign [=]
                    │   │       ├── <29> Var [i]
                    │   │       ╰── <35>  [+]
                    │   │           ├── <32> Var [i]
                    │   │           ╰── <34> Constant Int [1]
                    │   ╰── Block
                    │       ├── <42> Assign [=]
                    │       │   ├── <38> Var [counter]
                    │       │   ╰── <41> Var [i]
                    │       ├── If
                    │       │   ├── Condition
                    │       │   │   ╰── <51>  [==]
                    │       │   │       ├── <48>  [%]
                    │       │   │       │   ├── <45> Var [i]
                    │       │   │       │   ╰── <47> Constant Int [2]
                    │       │   │       ╰── <50> Constant Int [0]
                    │       │   ╰── Then
                    │       │       ╰── Continue
                    │       ╰── <62> Assign [=]
                    │           ├── <55> Var [sum]
                    │           ╰── <61>  [+]
                    │               ├── <58> Var [sum]
                    │               ╰── <60> Constant Int [1]
                    ╰── Return
                        ╰── <78>  [&&]
                            ├── <71>  [==]
                            │   ├── <68> Var [sum]
                            │   ╰── <70> Constant Int [5]
                            ╰── <77>  [==]
                                ├── <74> Var [counter]
                                ╰── <76> Constant Int [10]
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
                    │       ╰── <9> Constant Int [0]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <15> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <23>  [<]
                    │   │       ├── <20> Var [i]
                    │   │       ╰── <22> Constant Int [10]
                    │   ╰── Block
                    │       ├── <32> Assign [=]
                    │       │   ├── <25> Var [i]
                    │       │   ╰── <31>  [+]
                    │       │       ├── <28> Var [i]
                    │       │       ╰── <30> Constant Int [1]
                    │       ├── If
                    │       │   ├── Condition
                    │       │   │   ╰── <38>  [%]
                    │       │   │       ├── <35> Var [i]
                    │       │   │       ╰── <37> Constant Int [2]
                    │       │   ╰── Then
                    │       │       ╰── Continue
                    │       ╰── <50> Assign [=]
                    │           ├── <42> Var [sum]
                    │           ╰── <49>  [+]
                    │               ├── <45> Var [sum]
                    │               ╰── <48> Var [i]
                    ╰── Return
                        ╰── <56> Var [sum]
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
                    │       ╰── <9> Constant Int [1]
                    ├── DoWhile
                    │   ├── Body
                    │   │   ╰── Block
                    │   │       ╰── <20> Assign [=]
                    │   │           ├── <13> Var [a]
                    │   │           ╰── <19>  [*]
                    │   │               ├── <16> Var [a]
                    │   │               ╰── <18> Constant Int [2]
                    │   ╰── Condition
                    │       ╰── <28>  [<]
                    │           ├── <25> Var [a]
                    │           ╰── <27> Constant Int [11]
                    ╰── Return
                        ╰── <31> Var [a]
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
                    │       ╰── <9> Constant Int [10]
                    ├── DoWhile
                    │   ├── Body
                    │   │   ╰── Break
                    │   ╰── Condition
                    │       ╰── <18> Assign [=]
                    │           ├── <14> Var [a]
                    │           ╰── <16> Constant Int [1]
                    ╰── Return
                        ╰── <21> Var [a]
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
                    │   ╰── <6> Constant Int [0]
                    ├── Empty
                    ╰── Empty
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_empty_loop_body() {
    let src = r#"
        int main(void) {
            int i = 2147483642;
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
                    │       ╰── <9> Constant Int [2147483642]
                    ├── DoWhile
                    │   ├── Body
                    │   │   ╰── Empty
                    │   ╰── Condition
                    │       ╰── <25>  [>=]
                    │           ├── <22> Assign [=]
                    │           │   ├── <14> Var [i]
                    │           │   ╰── <20>  [-]
                    │           │       ├── <17> Var [i]
                    │           │       ╰── <19> Constant Int [5]
                    │           ╰── <24> Constant Int [256]
                    ╰── Return
                        ╰── <28> Var [i]
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
                    │       ╰── <9> Constant Int [4]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [0]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <18> Constant Int [2]
                    │   ╰── Block
                    │       ╰── Case [2]
                    │           ╰── Block
                    │               ├── VarDeclaration
                    │               │   ├── Name
                    │               │   │   ╰── a
                    │               │   ├── Type
                    │               │   │   ╰── Int
                    │               │   ╰── Initializer
                    │               │       ╰── <23> Constant Int [8]
                    │               ╰── <31> Assign [=]
                    │                   ├── <27> Var [b]
                    │                   ╰── <30> Var [a]
                    ╰── Return
                        ╰── <51>  [&&]
                            ├── <43>  [==]
                            │   ├── <40> Var [a]
                            │   ╰── <42> Constant Int [4]
                            ╰── <49>  [==]
                                ├── <46> Var [b]
                                ╰── <48> Constant Int [8]
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
                    │       ╰── <9> Constant Int [100]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── sum
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [0]
                    ├── DoWhile
                    │   ├── Body
                    │   │   ╰── <22> Assign [+=]
                    │   │       ├── <19> Var [sum]
                    │   │       ╰── <21> Constant Int [2]
                    │   ╰── Condition
                    │       ╰── <28> Assign [-=]
                    │           ├── <25> Var [i]
                    │           ╰── <27> Constant Int [1]
                    ╰── Return
                        ╰── <42>  [&&]
                            ├── <34>  [==]
                            │   ├── <31> Var [i]
                            │   ╰── <33> Constant Int [0]
                            ╰── <40>  [==]
                                ├── <37> Var [sum]
                                ╰── <39> Constant Int [200]
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
                    │       ╰── <9> Constant Int [1]
                    ├── For
                    │   ├── Init
                    │   │   ╰── <18> Assign [*=]
                    │   │       ├── <13> Var [i]
                    │   │       ╰── <17> Unary [-]
                    │   │           ╰── <16> Constant Int [1]
                    │   ├── Condition
                    │   │   ╰── <25>  [>=]
                    │   │       ├── <20> Var [i]
                    │   │       ╰── <24> Unary [-]
                    │   │           ╰── <23> Constant Int [100]
                    │   ├── Condition
                    │   │   ╰── <30> Assign [-=]
                    │   │       ├── <27> Var [i]
                    │   │       ╰── <29> Constant Int [3]
                    │   ╰── Empty
                    ╰── Return
                        ╰── <40>  [==]
                            ├── <34> Var [i]
                            ╰── <38> Unary [-]
                                ╰── <37> Constant Int [103]
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
                    │       ╰── <9> Constant Int [37]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── iterations
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <23>  [/]
                    │           ├── <20>  [+]
                    │           │   ├── <16> Var [count]
                    │           │   ╰── <18> Constant Int [4]
                    │           ╰── <22> Constant Int [5]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <30>  [%]
                    │   │       ├── <27> Var [count]
                    │   │       ╰── <29> Constant Int [5]
                    │   ╰── Block
                    │       ╰── Case [0]
                    │           ╰── DoWhile
                    │               ├── Body
                    │               │   ╰── Block
                    │               │       ├── <40> Assign [=]
                    │               │       │   ├── <33> Var [count]
                    │               │       │   ╰── <39>  [-]
                    │               │       │       ├── <36> Var [count]
                    │               │       │       ╰── <38> Constant Int [1]
                    │               │       ├── Case [4]
                    │               │       │   ╰── <51> Assign [=]
                    │               │       │       ├── <44> Var [count]
                    │               │       │       ╰── <50>  [-]
                    │               │       │           ├── <47> Var [count]
                    │               │       │           ╰── <49> Constant Int [1]
                    │               │       ├── Case [3]
                    │               │       │   ╰── <63> Assign [=]
                    │               │       │       ├── <56> Var [count]
                    │               │       │       ╰── <62>  [-]
                    │               │       │           ├── <59> Var [count]
                    │               │       │           ╰── <61> Constant Int [1]
                    │               │       ├── Case [2]
                    │               │       │   ╰── <75> Assign [=]
                    │               │       │       ├── <68> Var [count]
                    │               │       │       ╰── <74>  [-]
                    │               │       │           ├── <71> Var [count]
                    │               │       │           ╰── <73> Constant Int [1]
                    │               │       ╰── Case [1]
                    │               │           ╰── <87> Assign [=]
                    │               │               ├── <80> Var [count]
                    │               │               ╰── <86>  [-]
                    │               │                   ├── <83> Var [count]
                    │               │                   ╰── <85> Constant Int [1]
                    │               ╰── Condition
                    │                   ╰── <104>  [>]
                    │                       ├── <101> Assign [=]
                    │                       │   ├── <93> Var [iterations]
                    │                       │   ╰── <99>  [-]
                    │                       │       ├── <96> Var [iterations]
                    │                       │       ╰── <98> Constant Int [1]
                    │                       ╰── <103> Constant Int [0]
                    ╰── Return
                        ╰── <122>  [&&]
                            ├── <114>  [==]
                            │   ├── <111> Var [count]
                            │   ╰── <113> Constant Int [0]
                            ╰── <120>  [==]
                                ├── <117> Var [iterations]
                                ╰── <119> Constant Int [0]
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
                    │       ╰── <9> Constant Int [1]
                    ├── DoWhile
                    │   ├── Body
                    │   │   ╰── Block
                    │   │       ├── Label [while_start]
                    │   │       │   ╰── <21> Assign [=]
                    │   │       │       ├── <14> Var [i]
                    │   │       │       ╰── <20>  [+]
                    │   │       │           ├── <17> Var [i]
                    │   │       │           ╰── <19> Constant Int [1]
                    │   │       ╰── If
                    │   │           ├── Condition
                    │   │           │   ╰── <28>  [<]
                    │   │           │       ├── <25> Var [i]
                    │   │           │       ╰── <27> Constant Int [10]
                    │   │           ╰── Then
                    │   │               ╰── Goto [while_start]
                    │   ╰── Condition
                    │       ╰── <34> Constant Int [0]
                    ╰── Return
                        ╰── <37> Var [i]
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
                    │       ╰── <9> Constant Int [0]
                    ├── Goto [target]
                    ├── For
                    │   ├── Init
                    │   │   ╰── <18> Assign [=]
                    │   │       ├── <15> Var [i]
                    │   │       ╰── <17> Constant Int [5]
                    │   ├── Condition
                    │   │   ╰── <23>  [<]
                    │   │       ├── <20> Var [i]
                    │   │       ╰── <22> Constant Int [10]
                    │   ├── Condition
                    │   │   ╰── <32> Assign [=]
                    │   │       ├── <25> Var [i]
                    │   │       ╰── <31>  [+]
                    │   │           ├── <28> Var [i]
                    │   │           ╰── <30> Constant Int [1]
                    │   ╰── Label [target]
                    │       ╰── If
                    │           ├── Condition
                    │           │   ╰── <38>  [==]
                    │           │       ├── <35> Var [i]
                    │           │       ╰── <37> Constant Int [0]
                    │           ╰── Then
                    │               ╰── Return
                    │                   ╰── <39> Constant Int [1]
                    ╰── Return
                        ╰── <44> Constant Int [0]
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
                    │       ╰── <9> Constant Int [0]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <15> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <23> Assign [=]
                    │   │       ├── <20> Var [i]
                    │   │       ╰── <22> Constant Int [0]
                    │   ╰── Block
                    │       ├── Label [lbl]
                    │       │   ╰── <33> Assign [=]
                    │       │       ├── <26> Var [sum]
                    │       │       ╰── <32>  [+]
                    │       │           ├── <29> Var [sum]
                    │       │           ╰── <31> Constant Int [1]
                    │       ├── <44> Assign [=]
                    │       │   ├── <37> Var [i]
                    │       │   ╰── <43>  [+]
                    │       │       ├── <40> Var [i]
                    │       │       ╰── <42> Constant Int [1]
                    │       ├── If
                    │       │   ├── Condition
                    │       │   │   ╰── <50>  [>]
                    │       │   │       ├── <47> Var [i]
                    │       │   │       ╰── <49> Constant Int [10]
                    │       │   ╰── Then
                    │       │       ╰── Break
                    │       ╰── Goto [lbl]
                    ╰── Return
                        ╰── <59> Var [sum]
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
                    │       ╰── <9> Constant Int [0]
                    ├── Goto [label]
                    ├── While
                    │   ├── Condition
                    │   │   ╰── <14> Constant Int [0]
                    │   ╰── Body
                    │       ╰── Label [label]
                    │           ╰── Block
                    │               ╰── <20> Assign [=]
                    │                   ├── <17> Var [result]
                    │                   ╰── <19> Constant Int [1]
                    ╰── Return
                        ╰── <27> Var [result]
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
                    │       ╰── <9> Constant Int [0]
                    ├── Goto [do_label]
                    ├── Return
                    │   ╰── <14> Constant Int [0]
                    ├── Label [do_label]
                    │   ╰── DoWhile
                    │       ├── Body
                    │       │   ╰── Block
                    │       │       ├── <21> Assign [=]
                    │       │       │   ├── <18> Var [sum]
                    │       │       │   ╰── <20> Constant Int [1]
                    │       │       ╰── Goto [while_label]
                    │       ╰── Condition
                    │           ╰── <27> Constant Int [1]
                    ├── Label [while_label]
                    │   ╰── While
                    │       ├── Condition
                    │       │   ╰── <31> Constant Int [1]
                    │       ╰── Body
                    │           ╰── Block
                    │               ├── <40> Assign [=]
                    │               │   ├── <33> Var [sum]
                    │               │   ╰── <39>  [+]
                    │               │       ├── <36> Var [sum]
                    │               │       ╰── <38> Constant Int [1]
                    │               ├── Goto [break_label]
                    │               ├── Return
                    │               │   ╰── <44> Constant Int [0]
                    │               ╰── Label [break_label]
                    │                   ╰── Break
                    ├── Empty
                    ├── Goto [for_label]
                    ├── Return
                    │   ╰── <56> Constant Int [0]
                    ├── Label [for_label]
                    │   ╰── For
                    │       ├── Init
                    │       │   ╰── VarDeclaration
                    │       │       ├── Name
                    │       │       │   ╰── i
                    │       │       ├── Type
                    │       │       │   ╰── Int
                    │       │       ╰── Initializer
                    │       │           ╰── <62> Constant Int [0]
                    │       ├── Condition
                    │       │   ╰── <70>  [<]
                    │       │       ├── <67> Var [i]
                    │       │       ╰── <69> Constant Int [10]
                    │       ├── Condition
                    │       │   ╰── <79> Assign [=]
                    │       │       ├── <72> Var [i]
                    │       │       ╰── <78>  [+]
                    │       │           ├── <75> Var [i]
                    │       │           ╰── <77> Constant Int [1]
                    │       ╰── Block
                    │           ├── <88> Assign [=]
                    │           │   ├── <81> Var [sum]
                    │           │   ╰── <87>  [+]
                    │           │       ├── <84> Var [sum]
                    │           │       ╰── <86> Constant Int [1]
                    │           ├── Goto [continue_label]
                    │           ├── Return
                    │           │   ╰── <92> Constant Int [0]
                    │           ├── Label [continue_label]
                    │           │   ╰── Continue
                    │           ╰── Return
                    │               ╰── <97> Constant Int [0]
                    ╰── Return
                        ╰── <104> Var [sum]
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
                    │       ╰── <9> Constant Int [100]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── count
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [0]
                    ├── While
                    │   ├── Condition
                    │   │   ╰── <21> Postfix [--]
                    │   │       ╰── <19> Var [i]
                    │   ╰── Body
                    │       ╰── <25> Postfix [++]
                    │           ╰── <23> Var [count]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <32>  [!=]
                    │   │       ├── <29> Var [count]
                    │   │       ╰── <31> Constant Int [100]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <33> Constant Int [0]
                    ├── <40> Assign [=]
                    │   ├── <37> Var [i]
                    │   ╰── <39> Constant Int [100]
                    ├── <46> Assign [=]
                    │   ├── <43> Var [count]
                    │   ╰── <45> Constant Int [0]
                    ├── While
                    │   ├── Condition
                    │   │   ╰── <51> Unary [--]
                    │   │       ╰── <50> Var [i]
                    │   ╰── Body
                    │       ╰── <55> Postfix [++]
                    │           ╰── <53> Var [count]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <62>  [!=]
                    │   │       ├── <59> Var [count]
                    │   │       ╰── <61> Constant Int [99]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <63> Constant Int [0]
                    ╰── Return
                        ╰── <66> Constant Int [1]
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
                    │       ╰── <9> Constant Int [10]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <13> Var [cond]
                    │   ╰── Block
                    │       ├── Case [1]
                    │       │   ╰── Return
                    │       │       ╰── <15> Constant Int [0]
                    │       ├── Case [10]
                    │       │   ╰── For
                    │       │       ├── Init
                    │       │       │   ╰── VarDeclaration
                    │       │       │       ├── Name
                    │       │       │       │   ╰── i
                    │       │       │       ├── Type
                    │       │       │       │   ╰── Int
                    │       │       │       ╰── Initializer
                    │       │       │           ╰── <22> Constant Int [0]
                    │       │       ├── Condition
                    │       │       │   ╰── <30>  [<]
                    │       │       │       ├── <27> Var [i]
                    │       │       │       ╰── <29> Constant Int [5]
                    │       │       ├── Condition
                    │       │       │   ╰── <39> Assign [=]
                    │       │       │       ├── <32> Var [i]
                    │       │       │       ╰── <38>  [+]
                    │       │       │           ├── <35> Var [i]
                    │       │       │           ╰── <37> Constant Int [1]
                    │       │       ╰── Block
                    │       │           ├── <48> Assign [=]
                    │       │           │   ├── <41> Var [cond]
                    │       │           │   ╰── <47>  [-]
                    │       │           │       ├── <44> Var [cond]
                    │       │           │       ╰── <46> Constant Int [1]
                    │       │           ╰── If
                    │       │               ├── Condition
                    │       │               │   ╰── <54>  [==]
                    │       │               │       ├── <51> Var [cond]
                    │       │               │       ╰── <53> Constant Int [8]
                    │       │               ╰── Then
                    │       │                   ╰── Break
                    │       ├── Return
                    │       │   ╰── <61> Constant Int [123]
                    │       ╰── Default
                    │           ╰── Return
                    │               ╰── <63> Constant Int [2]
                    ╰── Return
                        ╰── <69> Constant Int [3]
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
                    │       ╰── <9> Constant Int [1]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <15> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <23>  [<]
                    │   │       ├── <20> Var [i]
                    │   │       ╰── <22> Constant Int [10]
                    │   ├── Condition
                    │   │   ╰── <27> Postfix [++]
                    │   │       ╰── <25> Var [i]
                    │   ╰── Block
                    │       ╰── <36> Assign [=]
                    │           ├── <29> Var [product]
                    │           ╰── <35>  [+]
                    │               ├── <32> Var [product]
                    │               ╰── <34> Constant Int [2]
                    ╰── Return
                        ╰── <42> Var [product]
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
                        │   ╰── <6> Constant Int [3]
                        ╰── Block
                            ├── Case [0]
                            │   ╰── Return
                            │       ╰── <8> Constant Int [0]
                            ├── Case [1]
                            │   ╰── Return
                            │       ╰── <12> Constant Int [1]
                            ├── Case [3]
                            │   ╰── Return
                            │       ╰── <16> Constant Int [3]
                            ╰── Case [5]
                                ╰── Return
                                    ╰── <20> Constant Int [5]
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
                    │       ╰── <9> Constant Int [0]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <16> Assign [=]
                    │   │       ├── <13> Var [a]
                    │   │       ╰── <15> Constant Int [1]
                    │   ╰── Block
                    │       ├── Case [0]
                    │       │   ╰── Return
                    │       │       ╰── <18> Constant Int [10]
                    │       ├── Case [1]
                    │       │   ╰── <30> Assign [=]
                    │       │       ├── <23> Var [a]
                    │       │       ╰── <29>  [*]
                    │       │           ├── <26> Var [a]
                    │       │           ╰── <28> Constant Int [2]
                    │       ├── Break
                    │       ╰── Default
                    │           ╰── <38> Assign [=]
                    │               ├── <35> Var [a]
                    │               ╰── <37> Constant Int [99]
                    ╰── Return
                        ╰── <45> Var [a]
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
                    │       ╰── <9> Constant Int [5]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <13> Var [a]
                    │   ╰── Block
                    │       ├── Case [5]
                    │       │   ╰── <19> Assign [=]
                    │       │       ├── <16> Var [a]
                    │       │       ╰── <18> Constant Int [10]
                    │       ├── Break
                    │       ├── Case [6]
                    │       │   ╰── <28> Assign [=]
                    │       │       ├── <25> Var [a]
                    │       │       ╰── <27> Constant Int [0]
                    │       ╰── Break
                    ╰── Return
                        ╰── <36> Var [a]
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
                    │       ╰── <9> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [0]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <19> Var [a]
                    │   ╰── Block
                    │       ├── VarDeclaration
                    │       │   ├── Name
                    │       │   │   ╰── a
                    │       │   ├── Type
                    │       │   │   ╰── Int
                    │       │   ╰── Initializer
                    │       │       ╰── <28> Assign [=]
                    │       │           ├── <24> Var [b]
                    │       │           ╰── <26> Constant Int [5]
                    │       ├── Case [3]
                    │       │   ╰── <36> Assign [=]
                    │       │       ├── <33> Var [a]
                    │       │       ╰── <35> Constant Int [4]
                    │       ╰── <48> Assign [=]
                    │           ├── <40> Var [b]
                    │           ╰── <47>  [+]
                    │               ├── <43> Var [b]
                    │               ╰── <46> Var [a]
                    ╰── Return
                        ╰── <64>  [&&]
                            ├── <57>  [==]
                            │   ├── <54> Var [a]
                            │   ╰── <56> Constant Int [3]
                            ╰── <63>  [==]
                                ├── <60> Var [b]
                                ╰── <62> Constant Int [4]
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
                    │       ╰── <9> Constant Int [0]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <13> Var [a]
                    │   ╰── Block
                    │       ├── Case [1]
                    │       │   ╰── Return
                    │       │       ╰── <15> Constant Int [1]
                    │       ├── Case [2]
                    │       │   ╰── Return
                    │       │       ╰── <19> Constant Int [9]
                    │       ├── Case [4]
                    │       │   ╰── <27> Assign [=]
                    │       │       ├── <24> Var [a]
                    │       │       ╰── <26> Constant Int [11]
                    │       ├── Break
                    │       ╰── Default
                    │           ╰── <35> Assign [=]
                    │               ├── <32> Var [a]
                    │               ╰── <34> Constant Int [22]
                    ╰── Return
                        ╰── <42> Var [a]
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
                    │       ╰── <9> Constant Int [5]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <12> Constant Int [0]
                    │   ╰── Block
                    │       ├── Default
                    │       │   ╰── <17> Assign [=]
                    │       │       ├── <14> Var [a]
                    │       │       ╰── <16> Constant Int [0]
                    │       ╰── Case [1]
                    │           ╰── Return
                    │               ╰── <22> Var [a]
                    ╰── Return
                        ╰── <32>  [+]
                            ├── <29> Var [a]
                            ╰── <31> Constant Int [1]
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
                    │       ╰── <17> Assign [=]
                    │           ├── <14> Var [a]
                    │           ╰── <16> Constant Int [7]
                    ╰── Switch
                        ├── Expression
                        │   ╰── <25>  [+]
                        │       ├── <21> Var [a]
                        │       ╰── <24> Var [b]
                        ╰── Block
                            ├── Default
                            │   ╰── Return
                            │       ╰── <26> Constant Int [0]
                            ╰── Case [2]
                                ╰── Return
                                    ╰── <30> Constant Int [1]
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
                    │       ╰── <9> Constant Int [1]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <13> Var [a]
                    │   ╰── Default
                    │       ╰── Return
                    │           ╰── <14> Constant Int [1]
                    ╰── Return
                        ╰── <18> Constant Int [0]
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
                    │       ╰── <9> Constant Int [10]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <20> Assign [=]
                    │   │       ├── <13> Var [x]
                    │   │       ╰── <19>  [+]
                    │   │           ├── <16> Var [x]
                    │   │           ╰── <18> Constant Int [1]
                    │   ╰── Block
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <32> Assign [=]
                    │   │       ├── <25> Var [x]
                    │   │       ╰── <31>  [+]
                    │   │           ├── <28> Var [x]
                    │   │           ╰── <30> Constant Int [1]
                    │   ╰── Empty
                    ╰── Return
                        ╰── <36> Var [x]
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
                    │       ╰── <9> Constant Int [4]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [9]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <21> Constant Int [0]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <29> Conditional [?]
                    │   │       ├── <25> Var [a]
                    │   │       ├── Then
                    │   │       │   ╰── <27> Var [b]
                    │   │       ╰── Else
                    │   │           ╰── <28> Constant Int [7]
                    │   ╰── Block
                    │       ├── Case [0]
                    │       │   ╰── Return
                    │       │       ╰── <31> Constant Int [5]
                    │       ├── Case [7]
                    │       │   ╰── <39> Assign [=]
                    │       │       ├── <36> Var [c]
                    │       │       ╰── <38> Constant Int [1]
                    │       ├── Case [9]
                    │       │   ╰── <47> Assign [=]
                    │       │       ├── <44> Var [c]
                    │       │       ╰── <46> Constant Int [2]
                    │       ╰── Case [1]
                    │           ╰── <59> Assign [=]
                    │               ├── <52> Var [c]
                    │               ╰── <58>  [+]
                    │                   ├── <55> Var [c]
                    │                   ╰── <57> Constant Int [4]
                    ╰── Return
                        ╰── <66> Var [c]
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
                    │       ╰── <9> Constant Int [0]
                    ├── Goto [mid_case]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <14> Constant Int [4]
                    │   ╰── Block
                    │       ├── Case [4]
                    │       │   ╰── <20> Assign [=]
                    │       │       ├── <17> Var [a]
                    │       │       ╰── <19> Constant Int [5]
                    │       ├── Label [mid_case]
                    │       │   ╰── <32> Assign [=]
                    │       │       ├── <25> Var [a]
                    │       │       ╰── <31>  [+]
                    │       │           ├── <28> Var [a]
                    │       │           ╰── <30> Constant Int [1]
                    │       ╰── Return
                    │           ╰── <36> Var [a]
                    ╰── Return
                        ╰── <41> Constant Int [100]
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
                    │       ╰── <9> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ctr
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [0]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <21> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <29>  [<]
                    │   │       ├── <26> Var [i]
                    │   │       ╰── <28> Constant Int [10]
                    │   ├── Condition
                    │   │   ╰── <38> Assign [=]
                    │   │       ├── <31> Var [i]
                    │   │       ╰── <37>  [+]
                    │   │           ├── <34> Var [i]
                    │   │           ╰── <36> Constant Int [1]
                    │   ╰── Block
                    │       ├── Switch
                    │       │   ├── Expression
                    │       │   │   ╰── <40> Var [i]
                    │       │   ╰── Block
                    │       │       ├── Case [0]
                    │       │       │   ╰── <46> Assign [=]
                    │       │       │       ├── <43> Var [acc]
                    │       │       │       ╰── <45> Constant Int [2]
                    │       │       ├── Break
                    │       │       ├── Case [1]
                    │       │       │   ╰── <59> Assign [=]
                    │       │       │       ├── <52> Var [acc]
                    │       │       │       ╰── <58>  [*]
                    │       │       │           ├── <55> Var [acc]
                    │       │       │           ╰── <57> Constant Int [3]
                    │       │       ├── Break
                    │       │       ├── Case [2]
                    │       │       │   ╰── <72> Assign [=]
                    │       │       │       ├── <65> Var [acc]
                    │       │       │       ╰── <71>  [*]
                    │       │       │           ├── <68> Var [acc]
                    │       │       │           ╰── <70> Constant Int [4]
                    │       │       ├── Break
                    │       │       ╰── Default
                    │       │           ╰── <84> Assign [=]
                    │       │               ├── <77> Var [acc]
                    │       │               ╰── <83>  [+]
                    │       │                   ├── <80> Var [acc]
                    │       │                   ╰── <82> Constant Int [1]
                    │       ╰── <98> Assign [=]
                    │           ├── <91> Var [ctr]
                    │           ╰── <97>  [+]
                    │               ├── <94> Var [ctr]
                    │               ╰── <96> Constant Int [1]
                    ╰── Return
                        ╰── <114>  [&&]
                            ├── <107>  [==]
                            │   ├── <104> Var [ctr]
                            │   ╰── <106> Constant Int [10]
                            ╰── <113>  [==]
                                ├── <110> Var [acc]
                                ╰── <112> Constant Int [31]
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
                    │       ╰── <9> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── switch2
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── switch3
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <21> Constant Int [0]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <24> Constant Int [3]
                    │   ╰── Block
                    │       ├── Case [0]
                    │       │   ╰── Return
                    │       │       ╰── <26> Constant Int [0]
                    │       ├── Case [1]
                    │       │   ╰── If
                    │       │       ├── Condition
                    │       │       │   ╰── <30> Constant Int [0]
                    │       │       ╰── Then
                    │       │           ╰── Block
                    │       │               ├── Case [3]
                    │       │               │   ╰── <36> Assign [=]
                    │       │               │       ├── <33> Var [switch1]
                    │       │               │       ╰── <35> Constant Int [1]
                    │       │               ╰── Break
                    │       ╰── Default
                    │           ╰── Return
                    │               ╰── <44> Constant Int [0]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <50> Constant Int [4]
                    │   ╰── Block
                    │       ├── Case [0]
                    │       │   ╰── Return
                    │       │       ╰── <52> Constant Int [0]
                    │       ├── If
                    │       │   ├── Condition
                    │       │   │   ╰── <55> Constant Int [1]
                    │       │   ├── Then
                    │       │   │   ╰── Block
                    │       │   │       ╰── Return
                    │       │   │           ╰── <56> Constant Int [0]
                    │       │   ╰── Else
                    │       │       ╰── Block
                    │       │           ├── Case [4]
                    │       │           │   ╰── <65> Assign [=]
                    │       │           │       ├── <62> Var [switch2]
                    │       │           │       ╰── <64> Constant Int [1]
                    │       │           ╰── Break
                    │       ╰── Default
                    │           ╰── Return
                    │               ╰── <72> Constant Int [0]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <78> Constant Int [5]
                    │   ╰── Block
                    │       ╰── For
                    │           ├── Init
                    │           │   ╰── VarDeclaration
                    │           │       ├── Name
                    │           │       │   ╰── i
                    │           │       ├── Type
                    │           │       │   ╰── Int
                    │           │       ╰── Initializer
                    │           │           ╰── <82> Constant Int [0]
                    │           ├── Condition
                    │           │   ╰── <90>  [<]
                    │           │       ├── <87> Var [i]
                    │           │       ╰── <89> Constant Int [10]
                    │           ├── Condition
                    │           │   ╰── <99> Assign [=]
                    │           │       ├── <92> Var [i]
                    │           │       ╰── <98>  [+]
                    │           │           ├── <95> Var [i]
                    │           │           ╰── <97> Constant Int [1]
                    │           ╰── Block
                    │               ├── <104> Assign [=]
                    │               │   ├── <101> Var [switch1]
                    │               │   ╰── <103> Constant Int [0]
                    │               ├── Case [5]
                    │               │   ╰── <111> Assign [=]
                    │               │       ├── <108> Var [switch3]
                    │               │       ╰── <110> Constant Int [1]
                    │               ├── Break
                    │               ╰── Default
                    │                   ╰── Return
                    │                       ╰── <115> Constant Int [0]
                    ╰── Return
                        ╰── <134>  [&&]
                            ├── <129>  [&&]
                            │   ├── <125> Var [switch1]
                            │   ╰── <128> Var [switch2]
                            ╰── <132> Var [switch3]
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
                    │       ╰── <9> Constant Int [0]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <13> Var [a]
                    │   ╰── Block
                    │       ├── Case [1]
                    │       │   ╰── Switch
                    │       │       ├── Expression
                    │       │       │   ╰── <16> Var [a]
                    │       │       ╰── Block
                    │       │           ├── Case [0]
                    │       │           │   ╰── Return
                    │       │           │       ╰── <18> Constant Int [0]
                    │       │           ╰── Default
                    │       │               ╰── Return
                    │       │                   ╰── <21> Constant Int [0]
                    │       ╰── Default
                    │           ╰── <32> Assign [=]
                    │               ├── <29> Var [a]
                    │               ╰── <31> Constant Int [2]
                    ╰── Return
                        ╰── <39> Var [a]
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
                        │   ╰── <6> Constant Int [3]
                        ╰── Block
                            ├── Case [0]
                            │   ╰── Return
                            │       ╰── <8> Constant Int [0]
                            ├── Case [3]
                            │   ╰── Block
                            │       ╰── Switch
                            │           ├── Expression
                            │           │   ╰── <12> Constant Int [4]
                            │           ╰── Block
                            │               ├── Case [3]
                            │               │   ╰── Return
                            │               │       ╰── <14> Constant Int [0]
                            │               ├── Case [4]
                            │               │   ╰── Return
                            │               │       ╰── <18> Constant Int [1]
                            │               ╰── Default
                            │                   ╰── Return
                            │                       ╰── <21> Constant Int [0]
                            ├── Case [4]
                            │   ╰── Return
                            │       ╰── <31> Constant Int [0]
                            ╰── Default
                                ╰── Return
                                    ╰── <34> Constant Int [0]
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
                    │       ╰── <9> Constant Int [4]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <13> Var [a]
                    │   ╰── Return
                    │       ╰── <14> Constant Int [0]
                    ╰── Return
                        ╰── <18> Var [a]
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
                    │       ╰── <9> Constant Int [1]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <13> Var [a]
                    │   ╰── Block
                    │       ├── Case [0]
                    │       │   ╰── Return
                    │       │       ╰── <15> Constant Int [0]
                    │       ├── Case [2]
                    │       │   ╰── Return
                    │       │       ╰── <19> Constant Int [0]
                    │       ╰── Case [3]
                    │           ╰── Return
                    │               ╰── <23> Constant Int [0]
                    ╰── Return
                        ╰── <29> Constant Int [1]
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
                    │       ╰── <9> Constant Int [1]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <13> Var [a]
                    │   ╰── Case [1]
                    │       ╰── Return
                    │           ╰── <15> Constant Int [1]
                    ╰── Return
                        ╰── <19> Constant Int [0]
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
                    │   │   ╰── <6> Constant Int [4]
                    │   ╰── Block
                    │       ├── Case [0]
                    │       │   ╰── Return
                    │       │       ╰── <8> Constant Int [0]
                    │       ╰── Case [4]
                    │           ╰── Block
                    │               ├── VarDeclaration
                    │               │   ├── Name
                    │               │   │   ╰── acc
                    │               │   ├── Type
                    │               │   │   ╰── Int
                    │               │   ╰── Initializer
                    │               │       ╰── <15> Constant Int [0]
                    │               ├── For
                    │               │   ├── Init
                    │               │   │   ╰── VarDeclaration
                    │               │   │       ├── Name
                    │               │   │       │   ╰── i
                    │               │   │       ├── Type
                    │               │   │       │   ╰── Int
                    │               │   │       ╰── Initializer
                    │               │   │           ╰── <21> Constant Int [0]
                    │               │   ├── Condition
                    │               │   │   ╰── <29>  [<]
                    │               │   │       ├── <26> Var [i]
                    │               │   │       ╰── <28> Constant Int [10]
                    │               │   ├── Condition
                    │               │   │   ╰── <38> Assign [=]
                    │               │   │       ├── <31> Var [i]
                    │               │   │       ╰── <37>  [+]
                    │               │   │           ├── <34> Var [i]
                    │               │   │           ╰── <36> Constant Int [1]
                    │               │   ╰── Block
                    │               │       ├── If
                    │               │       │   ├── Condition
                    │               │       │   │   ╰── <43>  [%]
                    │               │       │   │       ├── <40> Var [i]
                    │               │       │   │       ╰── <42> Constant Int [2]
                    │               │       │   ╰── Then
                    │               │       │       ╰── Continue
                    │               │       ╰── <54> Assign [=]
                    │               │           ├── <47> Var [acc]
                    │               │           ╰── <53>  [+]
                    │               │               ├── <50> Var [acc]
                    │               │               ╰── <52> Constant Int [1]
                    │               ╰── Return
                    │                   ╰── <60> Var [acc]
                    ╰── Return
                        ╰── <68> Constant Int [0]
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
                    │       ╰── <9> Constant Int [0]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <15> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <23>  [<]
                    │   │       ├── <20> Var [i]
                    │   │       ╰── <22> Constant Int [10]
                    │   ├── Condition
                    │   │   ╰── <32> Assign [=]
                    │   │       ├── <25> Var [i]
                    │   │       ╰── <31>  [+]
                    │   │           ├── <28> Var [i]
                    │   │           ╰── <30> Constant Int [1]
                    │   ╰── Block
                    │       ╰── Switch
                    │           ├── Expression
                    │           │   ╰── <37>  [%]
                    │           │       ├── <34> Var [i]
                    │           │       ╰── <36> Constant Int [2]
                    │           ╰── Block
                    │               ├── Case [0]
                    │               │   ╰── Continue
                    │               ╰── Default
                    │                   ╰── <49> Assign [=]
                    │                       ├── <42> Var [sum]
                    │                       ╰── <48>  [+]
                    │                           ├── <45> Var [sum]
                    │                           ╰── <47> Constant Int [1]
                    ╰── Return
                        ╰── <59> Var [sum]
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
                    │       ╰── <9> Constant Int [12345]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ╰── Type
                    │       ╰── Int
                    ├── For
                    │   ├── Init
                    │   │   ╰── <20> Assign [=]
                    │   │       ├── <17> Var [i]
                    │   │       ╰── <19> Constant Int [5]
                    │   ├── Condition
                    │   │   ╰── <25>  [>=]
                    │   │       ├── <22> Var [i]
                    │   │       ╰── <24> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <34> Assign [=]
                    │   │       ├── <27> Var [i]
                    │   │       ╰── <33>  [-]
                    │   │           ├── <30> Var [i]
                    │   │           ╰── <32> Constant Int [1]
                    │   ╰── <43> Assign [=]
                    │       ├── <36> Var [a]
                    │       ╰── <42>  [/]
                    │           ├── <39> Var [a]
                    │           ╰── <41> Constant Int [3]
                    ╰── Return
                        ╰── <47> Var [a]
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
                        │           ╰── <9> Constant Int [400]
                        ├── Condition
                        │   ╰── <21> Assign [=]
                        │       ├── <14> Var [i]
                        │       ╰── <20>  [-]
                        │           ├── <17> Var [i]
                        │           ╰── <19> Constant Int [100]
                        ╰── If
                            ├── Condition
                            │   ╰── <26>  [==]
                            │       ├── <23> Var [i]
                            │       ╰── <25> Constant Int [100]
                            ╰── Then
                                ╰── Return
                                    ╰── <27> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_for_absent_post() {
    let src = r#"
        int main(void) {
            int a = -2147483647;
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
                    │       ╰── <11> Unary [-]
                    │           ╰── <10> Constant Int [2147483647]
                    ├── For
                    │   ├── Condition
                    │   │   ╰── <21>  [!=]
                    │   │       ├── <18>  [%]
                    │   │       │   ├── <15> Var [a]
                    │   │       │   ╰── <17> Constant Int [5]
                    │   │       ╰── <20> Constant Int [0]
                    │   ╰── Block
                    │       ╰── <30> Assign [=]
                    │           ├── <23> Var [a]
                    │           ╰── <29>  [+]
                    │               ├── <26> Var [a]
                    │               ╰── <28> Constant Int [1]
                    ╰── Return
                        ╰── <46>  [||]
                            ├── <39>  [%]
                            │   ├── <36> Var [a]
                            │   ╰── <38> Constant Int [5]
                            ╰── <45>  [>]
                                ├── <42> Var [a]
                                ╰── <44> Constant Int [0]
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
                    │       ╰── <9> Constant Int [0]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <17> Unary [-]
                    │   │               ╰── <16> Constant Int [100]
                    │   ├── Condition
                    │   │   ╰── <25>  [<=]
                    │   │       ├── <22> Var [i]
                    │   │       ╰── <24> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <34> Assign [=]
                    │   │       ├── <27> Var [i]
                    │   │       ╰── <33>  [+]
                    │   │           ├── <30> Var [i]
                    │   │           ╰── <32> Constant Int [1]
                    │   ╰── <43> Assign [=]
                    │       ├── <36> Var [a]
                    │       ╰── <42>  [+]
                    │           ├── <39> Var [a]
                    │           ╰── <41> Constant Int [1]
                    ╰── Return
                        ╰── <47> Var [a]
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
                    │       ╰── <9> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── j
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── k
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <21> Constant Int [1]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <27> Constant Int [100]
                    │   ├── Condition
                    │   │   ╰── <35>  [>]
                    │   │       ├── <32> Var [i]
                    │   │       ╰── <34> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <44> Assign [=]
                    │   │       ├── <37> Var [i]
                    │   │       ╰── <43>  [-]
                    │   │           ├── <40> Var [i]
                    │   │           ╰── <42> Constant Int [1]
                    │   ╰── Block
                    │       ├── VarDeclaration
                    │       │   ├── Name
                    │       │   │   ╰── i
                    │       │   ├── Type
                    │       │   │   ╰── Int
                    │       │   ╰── Initializer
                    │       │       ╰── <48> Constant Int [1]
                    │       ├── VarDeclaration
                    │       │   ├── Name
                    │       │   │   ╰── j
                    │       │   ├── Type
                    │       │   │   ╰── Int
                    │       │   ╰── Initializer
                    │       │       ╰── <59>  [+]
                    │       │           ├── <55> Var [i]
                    │       │           ╰── <58> Var [k]
                    │       ╰── <67> Assign [=]
                    │           ├── <63> Var [k]
                    │           ╰── <66> Var [j]
                    ╰── Return
                        ╰── <90>  [&&]
                            ├── <83>  [&&]
                            │   ├── <76>  [==]
                            │   │   ├── <73> Var [k]
                            │   │   ╰── <75> Constant Int [101]
                            │   ╰── <82>  [==]
                            │       ├── <79> Var [i]
                            │       ╰── <81> Constant Int [0]
                            ╰── <89>  [==]
                                ├── <86> Var [j]
                                ╰── <88> Constant Int [0]
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
                    │       ╰── <9> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── acc
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [0]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── shadow
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <21> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <29>  [<]
                    │   │       ├── <26> Var [shadow]
                    │   │       ╰── <28> Constant Int [10]
                    │   ├── Condition
                    │   │   ╰── <38> Assign [=]
                    │   │       ├── <31> Var [shadow]
                    │   │       ╰── <37>  [+]
                    │   │           ├── <34> Var [shadow]
                    │   │           ╰── <36> Constant Int [1]
                    │   ╰── Block
                    │       ╰── <48> Assign [=]
                    │           ├── <40> Var [acc]
                    │           ╰── <47>  [+]
                    │               ├── <43> Var [acc]
                    │               ╰── <46> Var [shadow]
                    ╰── Return
                        ╰── <64>  [&&]
                            ├── <57>  [==]
                            │   ├── <54> Var [acc]
                            │   ╰── <56> Constant Int [45]
                            ╰── <63>  [==]
                                ├── <60> Var [shadow]
                                ╰── <62> Constant Int [1]
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
                    │       ╰── <9> Constant Int [0]
                    ├── While
                    │   ├── Condition
                    │   │   ╰── <12> Constant Int [1]
                    │   ╰── Body
                    │       ╰── Block
                    │           ├── <21> Assign [=]
                    │           │   ├── <14> Var [i]
                    │           │   ╰── <20>  [+]
                    │           │       ├── <17> Var [i]
                    │           │       ╰── <19> Constant Int [1]
                    │           ╰── If
                    │               ├── Condition
                    │               │   ╰── <27>  [>]
                    │               │       ├── <24> Var [i]
                    │               │       ╰── <26> Constant Int [10]
                    │               ╰── Then
                    │                   ╰── Break
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── j
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <36> Constant Int [10]
                    ├── While
                    │   ├── Condition
                    │   │   ╰── <39> Constant Int [1]
                    │   ╰── Body
                    │       ╰── Block
                    │           ├── <48> Assign [=]
                    │           │   ├── <41> Var [j]
                    │           │   ╰── <47>  [-]
                    │           │       ├── <44> Var [j]
                    │           │       ╰── <46> Constant Int [1]
                    │           ╰── If
                    │               ├── Condition
                    │               │   ╰── <54>  [<]
                    │               │       ├── <51> Var [j]
                    │               │       ╰── <53> Constant Int [0]
                    │               ╰── Then
                    │                   ╰── Break
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── result
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <76>  [&&]
                    │           ├── <69>  [==]
                    │           │   ├── <64> Var [j]
                    │           │   ╰── <68> Unary [-]
                    │           │       ╰── <67> Constant Int [1]
                    │           ╰── <75>  [==]
                    │               ├── <72> Var [i]
                    │               ╰── <74> Constant Int [11]
                    ╰── Return
                        ╰── <80> Var [result]
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
                    │       ╰── <9> Constant Int [10]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── y
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── z
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <21> Constant Int [0]
                    ├── DoWhile
                    │   ├── Body
                    │   │   ╰── Block
                    │   │       ├── <32> Assign [=]
                    │   │       │   ├── <25> Var [z]
                    │   │       │   ╰── <31>  [+]
                    │   │       │       ├── <28> Var [z]
                    │   │       │       ╰── <30> Constant Int [1]
                    │   │       ├── If
                    │   │       │   ├── Condition
                    │   │       │   │   ╰── <38>  [<=]
                    │   │       │   │       ├── <35> Var [x]
                    │   │       │   │       ╰── <37> Constant Int [0]
                    │   │       │   ╰── Then
                    │   │       │       ╰── Continue
                    │   │       ├── <49> Assign [=]
                    │   │       │   ├── <42> Var [x]
                    │   │       │   ╰── <48>  [-]
                    │   │       │       ├── <45> Var [x]
                    │   │       │       ╰── <47> Constant Int [1]
                    │   │       ├── If
                    │   │       │   ├── Condition
                    │   │       │   │   ╰── <55>  [>=]
                    │   │       │   │       ├── <52> Var [y]
                    │   │       │   │       ╰── <54> Constant Int [10]
                    │   │       │   ╰── Then
                    │   │       │       ╰── Continue
                    │   │       ╰── <66> Assign [=]
                    │   │           ├── <59> Var [y]
                    │   │           ╰── <65>  [+]
                    │   │               ├── <62> Var [y]
                    │   │               ╰── <64> Constant Int [1]
                    │   ╰── Condition
                    │       ╰── <74>  [!=]
                    │           ├── <71> Var [z]
                    │           ╰── <73> Constant Int [50]
                    ╰── Return
                        ╰── <94>  [&&]
                            ├── <87>  [&&]
                            │   ├── <80>  [==]
                            │   │   ├── <77> Var [z]
                            │   │   ╰── <79> Constant Int [50]
                            │   ╰── <86>  [==]
                            │       ├── <83> Var [x]
                            │       ╰── <85> Constant Int [0]
                            ╰── <93>  [==]
                                ├── <90> Var [y]
                                ╰── <92> Constant Int [10]
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
                    │       ╰── <9> Constant Int [0]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <15> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <23>  [<]
                    │   │       ├── <20> Var [i]
                    │   │       ╰── <22> Constant Int [10]
                    │   ├── Condition
                    │   │   ╰── <32> Assign [=]
                    │   │       ├── <25> Var [i]
                    │   │       ╰── <31>  [+]
                    │   │           ├── <28> Var [i]
                    │   │           ╰── <30> Constant Int [1]
                    │   ╰── For
                    │       ├── Init
                    │       │   ╰── VarDeclaration
                    │       │       ├── Name
                    │       │       │   ╰── j
                    │       │       ├── Type
                    │       │       │   ╰── Int
                    │       │       ╰── Initializer
                    │       │           ╰── <36> Constant Int [0]
                    │       ├── Condition
                    │       │   ╰── <44>  [<]
                    │       │       ├── <41> Var [j]
                    │       │       ╰── <43> Constant Int [10]
                    │       ├── Condition
                    │       │   ╰── <53> Assign [=]
                    │       │       ├── <46> Var [j]
                    │       │       ╰── <52>  [+]
                    │       │           ├── <49> Var [j]
                    │       │           ╰── <51> Constant Int [1]
                    │       ╰── If
                    │           ├── Condition
                    │           │   ╰── <66>  [==]
                    │           │       ├── <62>  [*]
                    │           │       │   ├── <59>  [/]
                    │           │       │   │   ├── <55> Var [i]
                    │           │       │   │   ╰── <57> Constant Int [2]
                    │           │       │   ╰── <61> Constant Int [2]
                    │           │       ╰── <65> Var [i]
                    │           ├── Then
                    │           │   ╰── Break
                    │           ╰── Else
                    │               ╰── <77> Assign [=]
                    │                   ├── <69> Var [ans]
                    │                   ╰── <76>  [+]
                    │                       ├── <72> Var [ans]
                    │                       ╰── <75> Var [i]
                    ╰── Return
                        ╰── <83> Var [ans]
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
                    │       ╰── <9> Constant Int [5]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── acc
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [0]
                    ├── While
                    │   ├── Condition
                    │   │   ╰── <22>  [>=]
                    │   │       ├── <19> Var [x]
                    │   │       ╰── <21> Constant Int [0]
                    │   ╰── Body
                    │       ╰── Block
                    │           ├── VarDeclaration
                    │           │   ├── Name
                    │           │   │   ╰── i
                    │           │   ├── Type
                    │           │   │   ╰── Int
                    │           │   ╰── Initializer
                    │           │       ╰── <27> Var [x]
                    │           ├── While
                    │           │   ├── Condition
                    │           │   │   ╰── <34>  [<=]
                    │           │   │       ├── <31> Var [i]
                    │           │   │       ╰── <33> Constant Int [10]
                    │           │   ╰── Body
                    │           │       ╰── Block
                    │           │           ├── <43> Assign [=]
                    │           │           │   ├── <36> Var [i]
                    │           │           │   ╰── <42>  [+]
                    │           │           │       ├── <39> Var [i]
                    │           │           │       ╰── <41> Constant Int [1]
                    │           │           ├── If
                    │           │           │   ├── Condition
                    │           │           │   │   ╰── <49>  [%]
                    │           │           │   │       ├── <46> Var [i]
                    │           │           │   │       ╰── <48> Constant Int [2]
                    │           │           │   ╰── Then
                    │           │           │       ╰── Continue
                    │           │           ╰── <60> Assign [=]
                    │           │               ├── <53> Var [acc]
                    │           │               ╰── <59>  [+]
                    │           │                   ├── <56> Var [acc]
                    │           │                   ╰── <58> Constant Int [1]
                    │           ╰── <73> Assign [=]
                    │               ├── <66> Var [x]
                    │               ╰── <72>  [-]
                    │                   ├── <69> Var [x]
                    │                   ╰── <71> Constant Int [1]
                    ╰── Return
                        ╰── <79> Var [acc]
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
                    │       ╰── <9> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [100]
                    ├── While
                    │   ├── Condition
                    │   │   ╰── <19> Var [x]
                    │   ╰── Body
                    │       ╰── Block
                    │           ├── VarDeclaration
                    │           │   ├── Name
                    │           │   │   ╰── y
                    │           │   ├── Type
                    │           │   │   ╰── Int
                    │           │   ╰── Initializer
                    │           │       ╰── <23> Constant Int [10]
                    │           ├── <35> Assign [=]
                    │           │   ├── <27> Var [x]
                    │           │   ╰── <34>  [-]
                    │           │       ├── <30> Var [x]
                    │           │       ╰── <33> Var [y]
                    │           ╰── While
                    │               ├── Condition
                    │               │   ╰── <38> Var [y]
                    │               ╰── Body
                    │                   ╰── Block
                    │                       ├── <47> Assign [=]
                    │                       │   ├── <40> Var [acc]
                    │                       │   ╰── <46>  [+]
                    │                       │       ├── <43> Var [acc]
                    │                       │       ╰── <45> Constant Int [1]
                    │                       ╰── <57> Assign [=]
                    │                           ├── <50> Var [y]
                    │                           ╰── <56>  [-]
                    │                               ├── <53> Var [y]
                    │                               ╰── <55> Constant Int [1]
                    ╰── Return
                        ╰── <76>  [&&]
                            ├── <69>  [==]
                            │   ├── <66> Var [acc]
                            │   ╰── <68> Constant Int [100]
                            ╰── <75>  [==]
                                ├── <72> Var [x]
                                ╰── <74> Constant Int [0]
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
                    │       ╰── <9> Constant Int [0]
                    ├── For
                    │   ╰── Block
                    │       ├── <20> Assign [=]
                    │       │   ├── <13> Var [a]
                    │       │   ╰── <19>  [+]
                    │       │       ├── <16> Var [a]
                    │       │       ╰── <18> Constant Int [1]
                    │       ╰── If
                    │           ├── Condition
                    │           │   ╰── <26>  [>]
                    │           │       ├── <23> Var [a]
                    │           │       ╰── <25> Constant Int [3]
                    │           ╰── Then
                    │               ╰── Break
                    ╰── Return
                        ╰── <33> Var [a]
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
                    │       ╰── <9> Constant Int [0]
                    ├── While
                    │   ├── Condition
                    │   │   ╰── <16>  [<]
                    │   │       ├── <13> Var [a]
                    │   │       ╰── <15> Constant Int [5]
                    │   ╰── Body
                    │       ╰── <25> Assign [=]
                    │           ├── <18> Var [a]
                    │           ╰── <24>  [+]
                    │               ├── <21> Var [a]
                    │               ╰── <23> Constant Int [2]
                    ╰── Return
                        ╰── <29> Var [a]
    "#;
    assert_parse(src, expected);
}
