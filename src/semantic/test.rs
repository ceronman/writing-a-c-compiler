use crate::parser::parse;
use crate::pretty::{annotate, remove_annotation};
use crate::semantic::validate;

fn assert_error(expected_annotated: &str) {
    let clean_source = remove_annotation(expected_annotated);
    let ast = parse(&clean_source).expect("Parse error");
    let Err(error) = validate(ast) else {
        panic!("No error produced!")
    };
    let actual_annotated = annotate(&clean_source, &error);
    assert_eq!(actual_annotated, expected_annotated);
}

#[test]
fn test_chapter_5_invalid_semantics_declared_after_use() {
    assert_error(
        r#"
        int main(void) {
            a = 1 + 2;
          //^ Undeclared variable 'a'
            int a;
            return a;
        }
    "#,
    );
}

#[test]
fn test_chapter_5_invalid_semantics_extra_credit_compound_invalid_lvalue() {
    assert_error(
        r#"
        int main(void) {
            int a = 0;
            -a += 1;
          //^^ Expression is not assignable
            return a;
        }
    "#,
    );
}

#[test]
fn test_chapter_5_invalid_semantics_extra_credit_compound_invalid_lvalue_2() {
    assert_error(
        r#"
        int main(void) {
            int a = 10;
            (a += 1) -= 2;
          //^^^^^^^^ Expression is not assignable
        }
    "#,
    );
}

#[test]
fn test_chapter_5_invalid_semantics_extra_credit_postfix_decr_non_lvalue() {
    assert_error(
        r#"
        int main(void) {
            int a = 10;
            return a++--;
                 //^^^ Expression is not assignable
        }
    "#,
    );
}

#[test]
fn test_chapter_5_invalid_semantics_extra_credit_postfix_incr_non_lvalue() {
    assert_error(
        r#"
        int main(void) {
            int a = 0;
            (a = 4)++;
          //^^^^^^^ Expression is not assignable
        }
    "#,
    );
}

#[test]
fn test_chapter_5_invalid_semantics_extra_credit_prefix_decr_non_lvalue() {
    assert_error(
        r#"
        int main(void) {
            return --3;
                   //^ Expression is not assignable
        }
    "#,
    );
}

#[test]
fn test_chapter_5_invalid_semantics_extra_credit_prefix_incr_non_lvalue() {
    assert_error(
        r#"
        int main(void) {
            int a = 1;
            ++(a+1);
            //^^^^^ Expression is not assignable
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_5_invalid_semantics_extra_credit_undeclared_bitwise_op() {
    assert_error(
        r#"
        int main(void){
            return a >> 2;
                 //^ Undeclared variable 'a'
        }
    "#,
    );
}

#[test]
fn test_chapter_5_invalid_semantics_extra_credit_undeclared_compound_assignment() {
    assert_error(
        r#"
        int main(void) {
            a += 1;
          //^ Undeclared variable 'a'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_5_invalid_semantics_extra_credit_undeclared_compound_assignment_use() {
    assert_error(
        r#"
        int main(void) {
            int b = 10;
            b *= a;
               //^ Undeclared variable 'a'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_5_invalid_semantics_extra_credit_undeclared_postfix_decr() {
    assert_error(
        r#"
        int main(void) {
            a--;
          //^ Undeclared variable 'a'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_5_invalid_semantics_extra_credit_undeclared_prefix_incr() {
    assert_error(
        r#"
        int main(void) {
            a++;
          //^ Undeclared variable 'a'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_5_invalid_semantics_invalid_lvalue() {
    assert_error(
        r#"
        int main(void) {
            int a = 2;
            a + 3 = 4;
          //^^^^^ Expression is not assignable
            return a;
        }
    "#,
    );
}

#[test]
fn test_chapter_5_invalid_semantics_invalid_lvalue_2() {
    assert_error(
        r#"
        int main(void) {
            int a = 2;
            !a = 3;
          //^^ Expression is not assignable
            return a;
        }
    "#,
    );
}

#[test]
fn test_chapter_5_invalid_semantics_mixed_precedence_assignment() {
    assert_error(
        r#"
        int main(void) {
            int a = 1;
            int b = 2;
            a = 3 * b = a;
              //^^^^^ Expression is not assignable
        }
    "#,
    );
}

#[test]
fn test_chapter_5_invalid_semantics_redefine() {
    assert_error(
        r#"
        int main(void) {
            int a = 1;
            int a = 2;
              //^ Variable 'a' was already declared
            return a;
        }
    "#,
    );
}

#[test]
fn test_chapter_5_invalid_semantics_undeclared_var() {
    assert_error(
        r#"
        int main(void) {
            return a;
                 //^ Undeclared variable 'a'
        }
    "#,
    );
}

#[test]
fn test_chapter_5_invalid_semantics_undeclared_var_and() {
    assert_error(
        r#"
        int main(void) {
            return 0 && a;
                      //^ Undeclared variable 'a'
        }
    "#,
    );
}

#[test]
fn test_chapter_5_invalid_semantics_undeclared_var_compare() {
    assert_error(
        r#"
        int main(void) {
            return a < 5;
                 //^ Undeclared variable 'a'
        }
    "#,
    );
}

#[test]
fn test_chapter_5_invalid_semantics_undeclared_var_unary() {
    assert_error(
        r#"
        int main(void) {
            return -a;
                  //^ Undeclared variable 'a'
        }
    "#,
    );
}

#[test]
fn test_chapter_5_invalid_semantics_use_then_redefine() {
    assert_error(
        r#"
        int main(void) {
            int a = 0;
            return a;
            int a = 1;
              //^ Variable 'a' was already declared
            return a;
        }
    "#,
    );
}

#[test]
fn test_chapter_6_invalid_semantics_extra_credit_duplicate_labels() {
    assert_error(
        r#"
        
        int main(void) {
            int x = 0;
        label:
            x = 1;
        label:
      //^^^^^ Label 'label' was already defined
            return 2;
        }
    "#,
    );
}

#[test]
fn test_chapter_6_invalid_semantics_extra_credit_goto_missing_label() {
    assert_error(
        r#"
        int main(void) {
            goto label;
               //^^^^^ Undefined label 'label'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_6_invalid_semantics_extra_credit_goto_variable() {
    assert_error(
        r#"
        int main(void) {
            int a;
            goto a;
               //^ Undefined label 'a'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_6_invalid_semantics_extra_credit_undeclared_var_in_labeled_statement() {
    assert_error(
        r#"
        int main(void) {
        lbl:
            return a;
                 //^ Undeclared variable 'a'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_6_invalid_semantics_extra_credit_use_label_as_variable() {
    assert_error(
        r#"
        int main(void) {
            int x = 0;
            a:
            x = a;
              //^ Undeclared variable 'a'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_6_invalid_semantics_invalid_var_in_if() {
    assert_error(
        r#"
        int main(void) {
            if (1)
                return c;
                     //^ Undeclared variable 'c'
            int c = 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_6_invalid_semantics_ternary_assign() {
    assert_error(
        r#"
        int main(void) {
            int a = 2;
            int b = 1;
            a > b ? a = 1 : a = 0;
          //^^^^^^^^^^^^^^^^^ Expression is not assignable
            return a;
        }
    "#,
    );
}

#[test]
fn test_chapter_6_invalid_semantics_undeclared_var_in_ternary() {
    assert_error(
        r#"
        int main(void) {
            return a > 0 ? 1 : 2;
                 //^ Undeclared variable 'a'
            int a = 5;
        }
    "#,
    );
}

#[test]
fn test_chapter_7_invalid_semantics_double_define() {
    assert_error(
        r#"
        int main(void) {
            {
                int a;
                int a;
                  //^ Variable 'a' was already declared
            }
        }
    "#,
    );
}

#[test]
fn test_chapter_7_invalid_semantics_double_define_after_scope() {
    assert_error(
        r#"
        int main(void) {
            int a = 3;
            {
                a = 5;
            }
            int a = 2;
              //^ Variable 'a' was already declared
            return a;
        }
    "#,
    );
}

#[test]
fn test_chapter_7_invalid_semantics_extra_credit_different_labels_same_scope() {
    assert_error(
        r#"
        int main(void) {
        label1:;
            int a = 10;
        label2:;
            int a = 11;
              //^ Variable 'a' was already declared
            return 1;
        }
    "#,
    );
}

#[test]
fn test_chapter_7_invalid_semantics_extra_credit_duplicate_labels_different_scopes() {
    assert_error(
        r#"
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
              //^ Label 'l' was already defined
                    return x;
            }
        }
    "#,
    );
}

#[test]
fn test_chapter_7_invalid_semantics_extra_credit_goto_use_before_declare() {
    assert_error(
        r#"
        int main(void) {
            int x = 0;
            if (x != 0) {
                return_y:
                return y;
                     //^ Undeclared variable 'y'
            }
            int y = 4;
            goto return_y;
        }
    "#,
    );
}

#[test]
fn test_chapter_7_invalid_semantics_out_of_scope() {
    assert_error(
        r#"
        int main(void) {
            {
                int a = 2;
            }
            return a;
                 //^ Undeclared variable 'a'
        }
    "#,
    );
}

#[test]
fn test_chapter_7_invalid_semantics_use_before_declare() {
    assert_error(
        r#"
        int main(void) {
            int a;
            {
                b = 10;
              //^ Undeclared variable 'b'
            }
            int b;
            return b;
        }
    "#,
    );
}

#[test]
fn test_chapter_8_invalid_semantics_break_not_in_loop() {
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
fn test_chapter_8_invalid_semantics_continue_not_in_loop() {
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
fn test_chapter_8_invalid_semantics_extra_credit_case_continue() {
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
fn test_chapter_8_invalid_semantics_extra_credit_case_outside_switch() {
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
fn test_chapter_8_invalid_semantics_extra_credit_default_continue() {
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
fn test_chapter_8_invalid_semantics_extra_credit_default_outside_switch() {
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
fn test_chapter_8_invalid_semantics_extra_credit_different_cases_same_scope() {
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
fn test_chapter_8_invalid_semantics_extra_credit_duplicate_case() {
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
fn test_chapter_8_invalid_semantics_extra_credit_duplicate_case_in_labeled_switch() {
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
fn test_chapter_8_invalid_semantics_extra_credit_duplicate_case_in_nested_statement() {
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
fn test_chapter_8_invalid_semantics_extra_credit_duplicate_default() {
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
fn test_chapter_8_invalid_semantics_extra_credit_duplicate_default_in_nested_statement() {
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
fn test_chapter_8_invalid_semantics_extra_credit_duplicate_label_in_default() {
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
fn test_chapter_8_invalid_semantics_extra_credit_duplicate_label_in_loop() {
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
fn test_chapter_8_invalid_semantics_extra_credit_duplicate_variable_in_switch() {
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
fn test_chapter_8_invalid_semantics_extra_credit_labeled_break_outside_loop() {
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
fn test_chapter_8_invalid_semantics_extra_credit_non_constant_case() {
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
fn test_chapter_8_invalid_semantics_extra_credit_switch_continue() {
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
fn test_chapter_8_invalid_semantics_extra_credit_undeclared_var_switch_expression() {
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
fn test_chapter_8_invalid_semantics_extra_credit_undeclared_variable_in_case() {
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
fn test_chapter_8_invalid_semantics_extra_credit_undeclared_variable_in_default() {
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
fn test_chapter_8_invalid_semantics_extra_credit_undefined_label_in_case() {
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
fn test_chapter_8_invalid_semantics_out_of_scope_do_loop() {
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
fn test_chapter_8_invalid_semantics_out_of_scope_loop_variable() {
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
