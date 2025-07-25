use super::assert_error;

#[test]
fn test_invalid_semantics_break_not_in_loop() {
    assert_error(
        r#"
        int main(void) {
            if (1)
                break;
              //^^^^^^ 'break' statement not in loop or switch statement
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_continue_not_in_loop() {
    assert_error(
        r#"
        int main(void) {
            {
                int a;
                continue;
              //^^^^^^^^^ 'continue' statement not in loop statement
            }
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_extra_credit_case_continue() {
    assert_error(
        r#"
        int main(void) {
            int a = 3;
            switch(a + 1) {
                case 0:
                    continue;
                  //^^^^^^^^^ 'continue' statement not in loop statement
                default: a = 1;
            }
            return a;
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_extra_credit_case_outside_switch() {
    assert_error(
        r#"
        int main(void) {
            for (int i = 0; i < 10; i = i + 1) {
                case 0: return 1;
              //^^^^^^^^^^^^^^^^^ case label not within a switch statement
            }
            return 9;
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_extra_credit_default_continue() {
    assert_error(
        r#"
        int main(void) {
            int a = 3;
            switch(a + 1) {
                case 0:
                    a = 1;
                default: continue;
                       //^^^^^^^^^ 'continue' statement not in loop statement
            }
            return a;
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_extra_credit_default_outside_switch() {
    assert_error(
        r#"
        int main(void) {
            {
                default: return 0;
              //^^^^^^^^^^^^^^^^^^ default label not within a switch statement
            }
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_extra_credit_different_cases_same_scope() {
    assert_error(
        r#"
        int main(void) {
            int a = 1;
            switch (a) {
                case 1:;
                    int b = 10;
                    break;
                case 2:;
                    int b = 11;
                      //^ Variable 'b' was already declared
                    break;
                default:
                    break;
            }
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_extra_credit_duplicate_case() {
    assert_error(
        r#"
        int main(void) {
            switch(4) {
                case 5: return 0;
                case 4: return 1;
                case 5: return 0;
                   //^ duplicate case value
                default: return 2;
            }
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_extra_credit_duplicate_case_in_labeled_switch() {
    assert_error(
        r#"
        int main(void) {
            int a = 0;
        label:
            switch (a) {
                case 1:
                case 1:
                   //^ duplicate case value
                    break;
            }
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_extra_credit_duplicate_case_in_nested_statement() {
    assert_error(
        r#"
        
        int main(void) {
            int a = 10;
            switch (a) {
                case 1: {
                    if(1) {
                        case 1:
                           //^ duplicate case value
                        return 0;
                    }
                }
            }
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_extra_credit_duplicate_default() {
    assert_error(
        r#"
        int main(void) {
            int a = 0;
            switch(a) {
                case 0: return 0;
                default: return 1;
                case 2: return 2;
                default: return 2;
              //^^^^^^^^^^^^^^^^^^ multiple default labels in one switch
            }
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_extra_credit_duplicate_default_in_nested_statement() {
    assert_error(
        r#"
        
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
              //^^^^^^^^^ multiple default labels in one switch
            }
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_extra_credit_duplicate_label_in_default() {
    assert_error(
        r#"
        int main(void) {
                int a = 1;
        label:
            switch (a) {
                case 1:
                    return 0;
                default:
                label:
              //^^^^^ Label 'label' was already defined
                    return 1;
            }
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_extra_credit_duplicate_label_in_loop() {
    assert_error(
        r#"
        int main(void) {
            do {
            lbl:
                return 1;
            lbl:
          //^^^ Label 'lbl' was already defined
                return 2;
            } while (1);
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_extra_credit_duplicate_variable_in_switch() {
    assert_error(
        r#"
        int main(void) {
            int a = 1;
            switch (a) {
                int b = 2;
                case 0:
                    a = 3;
                    int b = 2;
                      //^ Variable 'b' was already declared
            }
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_extra_credit_labeled_break_outside_loop() {
    assert_error(
        r#"
        int main(void) {
            label: break;
                 //^^^^^^ 'break' statement not in loop or switch statement
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_extra_credit_non_constant_case() {
    assert_error(
        r#"
        int main(void) {
            int a = 3;
            switch(a + 1) {
                case 0: return 0;
                case a: return 1;
                   //^ case label does not reduce to an integer constant
                case 1: return 2;
            }
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_extra_credit_switch_continue() {
    assert_error(
        r#"
        int main(void) {
            int a = 3;
            switch(a + 1) {
                case 0:
                    a = 4;
                    continue;
                  //^^^^^^^^^ 'continue' statement not in loop statement
                default: a = 1;
            }
            return a;
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_extra_credit_undeclared_var_switch_expression() {
    assert_error(
        r#"
        int main(void) {
            switch(a) {
                 //^ Undeclared variable 'a'
                case 1: return 0;
                case 2: return 1;
            }
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_extra_credit_undeclared_variable_in_case() {
    assert_error(
        r#"
        int main(void) {
            int a = 10;
            switch (a) {
                case 1:
                    return b;
                         //^ Undeclared variable 'b'
                    break;
                default:
                    break;
            }
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_extra_credit_undeclared_variable_in_default() {
    assert_error(
        r#"
        int main(void) {
            int a = 10;
            switch (a) {
                case 1:
                    break;
                default:
                    return b;
                         //^ Undeclared variable 'b'
                    break;
            }
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_extra_credit_undefined_label_in_case() {
    assert_error(
        r#"
        
        int main(void) {
            int a = 3;
            switch (a) {
                case 1: goto foo;
                           //^^^ Undefined label 'foo'
                default: return 0;
            }
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_out_of_scope_do_loop() {
    assert_error(
        r#"
        int main(void) {
            do {
                int a = a + 1;
            } while (a < 100);
                   //^ Undeclared variable 'a'
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_out_of_scope_loop_variable() {
    assert_error(
        r#"
        int main(void)
        {
            for (i = 0; i < 1; i = i + 1)
               //^ Undeclared variable 'i'
            {
                return 0;
            }
        }
    "#,
    );
}
