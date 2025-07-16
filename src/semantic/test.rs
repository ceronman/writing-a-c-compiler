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

#[test]
fn test_chapter_9_invalid_declarations_assign_to_fun_call() {
    assert_error(
        r#"
        int x(void);
        int main(void) {
            x() = 1;
          //^ Expression is not assignable
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_9_invalid_declarations_decl_params_with_same_name() {
    assert_error(
        r#"
        int foo(int a, int a);
                         //^ Parameter 'a' was already declared
        int main(void) {
            return foo(1, 2);
        }
        int foo(int a, int b) {
            return a + b;
        }
    "#,
    );
}

#[test]
fn test_chapter_9_invalid_declarations_extra_credit_call_label_as_function() {
    assert_error(
        r#"
        int main(void) {
            int x = 1;
            a:
            x = x + 1;
            a();
          //^ Undeclared function 'a'
            return x;
        }
    "#,
    );
}

#[test]
fn test_chapter_9_invalid_declarations_extra_credit_compound_assign_to_fun_call() {
    assert_error(
        r#"
        int x(void);
        int main(void) {
            x() += 1;
          //^ Expression is not assignable
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_9_invalid_declarations_extra_credit_decrement_fun_call() {
    assert_error(
        r#"
        int x(void);
        int main(void) {
            x()--;
          //^ Expression is not assignable
        }
    "#,
    );
}

#[test]
fn test_chapter_9_invalid_declarations_extra_credit_increment_fun_call() {
    assert_error(
        r#"
        int x(void);
        int main(void) {
            ++x();
            //^ Expression is not assignable
        }
    "#,
    );
}

#[test]
fn test_chapter_9_invalid_declarations_nested_function_definition() {
    assert_error(
        r#"
        int main(void) {
            int foo(void) {
              //^^^ Nested function definitions are not allowed 
                return 1;
            }
            return foo();
        }
    "#,
    );
}

#[test]
fn test_chapter_9_invalid_declarations_params_with_same_name() {
    assert_error(
        r#"
        
        int foo(int a, int a) {
                         //^ Parameter 'a' was already declared
            return a;
        }
        int main(void) {
            return foo(1, 2);
        }
    "#,
    );
}

#[test]
fn test_chapter_9_invalid_declarations_redefine_fun_as_var() {
    assert_error(
        r#"
        int main(void) {
            int foo(void);
            int foo = 1;
              //^^^ Variable 'foo' was already declared
            return foo;
        }
        int foo(void) {
            return 1;
        }
    "#,
    );
}

#[test]
fn test_chapter_9_invalid_declarations_redefine_parameter() {
    assert_error(
        r#"
        int foo(int a) {
            int a = 5;
              //^ Variable 'a' was already declared
            return a;
        }
        int main(void) {
            return foo(3);
        }
    "#,
    );
}

#[test]
fn test_chapter_9_invalid_declarations_redefine_var_as_fun() {
    assert_error(
        r#"
        int main(void) {
            int foo = 1;
            int foo(void);
              //^^^ Variable 'foo' was already declared
            return foo;
        }
        int foo(void) {
            return 1;
        }
    "#,
    );
}

#[test]
fn test_chapter_9_invalid_declarations_undeclared_fun() {
    assert_error(
        r#"
        int main(void) {
            return foo(3);
                 //^^^ Undeclared function 'foo'
        }
        int foo(int a) {
            return 1;
        }
    "#,
    );
}

#[test]
fn test_chapter_9_invalid_declarations_wrong_parameter_names() {
    assert_error(
        r#"
        int foo(int a);
        int main(void) {
            return foo(3);
        }
        int foo(int x) {
            return a;
                 //^ Undeclared variable 'a'
        }
    "#,
    );
}

#[test]
fn test_chapter_9_invalid_labels_extra_credit_goto_cross_function() {
    assert_error(
        r#"
        int foo(void) {
            label:
                return 0;
        }
        int main(void) {
            goto label;
               //^^^^^ Undefined label 'label'
            return 1;
        }
    "#,
    );
}

#[test]
fn test_chapter_9_invalid_labels_extra_credit_goto_function() {
    assert_error(
        r#"
        int foo(void) {
            return 3;
        }
        int main(void) {
            goto foo;
               //^^^ Undefined label 'foo'
            return 3;
        }
    "#,
    );
}

#[test]
fn test_chapter_9_invalid_types_assign_fun_to_variable() {
    assert_error(
        r#"
        int x(void);
        int main(void) {
            int a = 10;
            a = x;
              //^ Function used as variable
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_9_invalid_types_assign_value_to_function() {
    assert_error(
        r#"
        int main(void) {
            int x(void);
            x = 3;
          //^ Function used as variable
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_9_invalid_types_call_variable_as_function() {
    assert_error(
        r#"
        int x(void);
        int main(void) {
            int x = 0;
            return x();
                 //^ Variable used as function name
        }
    "#,
    );
}

#[test]
fn test_chapter_9_invalid_types_conflicting_function_declarations() {
    assert_error(
        r#"
        int foo(int a);
        int main(void) {
            return 5;
        }
        int foo(int a, int b) {
          //^^^ Conflicting declaration types for 'foo'
            return 4;
        }
    "#,
    );
}

#[test]
fn test_chapter_9_invalid_types_conflicting_local_function_declaration() {
    assert_error(
        r#"
        int bar(void);
        int main(void) {
            int foo(int a);
            return bar() + foo(1);
        }
        int bar(void) {
            int foo(int a, int b);
              //^^^ Conflicting declaration types for 'foo'
            return foo(1, 2);
        }
    "#,
    );
}

#[test]
fn test_chapter_9_invalid_types_divide_by_function() {
    assert_error(
        r#"
        int x(void);
        int main(void) {
            int a = 10 / x;
                       //^ Function used as variable
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_9_invalid_types_extra_credit_bitwise_op_function() {
    assert_error(
        r#"
        int x(void);
        int main(void) {
            x >> 2;
          //^ Function used as variable
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_9_invalid_types_extra_credit_compound_assign_function_lhs() {
    assert_error(
        r#"
        int x(void);
        int main(void) {
            x += 3;
          //^ Function used as variable
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_9_invalid_types_extra_credit_compound_assign_function_rhs() {
    assert_error(
        r#"
        int x(void);
        int main(void) {
            int a = 3;
            a += x;
               //^ Function used as variable
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_9_invalid_types_extra_credit_postfix_incr_fun_name() {
    assert_error(
        r#"
        int x(void);
        int main(void) {
            x++;
          //^ Function used as variable
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_9_invalid_types_extra_credit_prefix_decr_fun_name() {
    assert_error(
        r#"
        int x(void);
        int main(void){
            --x;
            //^ Function used as variable
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_9_invalid_types_extra_credit_switch_on_function() {
    assert_error(
        r#"
        int main(void) {
            int f(void);
            switch (f)
                  //^ Function used as variable
                return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_9_invalid_types_multiple_function_definitions() {
    assert_error(
        r#"
        
        int foo(void){
            return 3;
        }
        int main(void) {
            return foo();
        }
        int foo(void){
          //^^^ Function 'foo' is defined more than once
            return 4;
        }
    "#,
    );
}

#[test]
fn test_chapter_9_invalid_types_multiple_function_definitions_2() {
    assert_error(
        r#"
        
        int foo(void){
            return 3;
        }
        int main(void) {
            int foo(void);
            return foo();
        }
        int foo(void){
          //^^^ Function 'foo' is defined more than once
            return 4;
        }
    "#,
    );
}

#[test]
fn test_chapter_9_invalid_types_too_few_args() {
    assert_error(
        r#"
        int foo(int a, int b) {
            return a + 1;
        }
        int main(void) {
            return foo(1);
                 //^^^ Function called with the wrong number of arguments
        }
    "#,
    );
}

#[test]
fn test_chapter_9_invalid_types_too_many_args() {
    assert_error(
        r#"
        int foo(int a) {
            return a + 1;
        }
        int main(void) {
            return foo(1, 2);
                 //^^^ Function called with the wrong number of arguments
        }
    "#,
    );
}

#[test]
fn test_chapter_10_invalid_declarations_conflicting_local_declarations() {
    assert_error(
        r#"
        int main(void) {
            int x = 1;
            static int x;
                     //^ Variable 'x' was already declared
            return x;
        }
    "#,
    );
}

#[test]
fn test_chapter_10_invalid_declarations_extern_follows_local_var() {
    assert_error(
        r#"
        int main(void) {
            int x = 3;
            extern int x;
                     //^ Variable 'x' was already declared
            return x;
        }
    "#,
    );
}

#[test]
fn test_chapter_10_invalid_declarations_extern_follows_static_local_var() {
    assert_error(
        r#"
        int main(void) {
            static int x = 0;
            extern int x;
                     //^ Variable 'x' was already declared
            return x;
        }
    "#,
    );
}

#[test]
fn test_chapter_10_invalid_declarations_local_var_follows_extern() {
    assert_error(
        r#"
        int i = 10;
        int main(void) {
            extern int i;
            int i;
              //^ Variable 'i' was already declared
            return i;
        }
    "#,
    );
}

#[test]
fn test_chapter_10_invalid_declarations_out_of_scope_extern_var() {
    assert_error(
        r#"
        int main(void) {
            {
                extern int a;
            }
            return a;
                 //^ Undeclared variable 'a'
        }
        int a = 1;
    "#,
    );
}

#[test]
fn test_chapter_10_invalid_declarations_redefine_param_as_identifier_with_linkage() {
    assert_error(
        r#"
        int f(int i) {
            extern int i;
                     //^ Variable 'i' was already declared
            return i;
        }
        int main(void) {
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_10_invalid_declarations_undeclared_global_variable() {
    assert_error(
        r#"
        int main(void) {
            return x;
                 //^ Undeclared variable 'x'
        }
        int x = 0;
    "#,
    );
}

#[test]
fn test_chapter_10_invalid_labels_extra_credit_goto_global_var() {
    assert_error(
        r#"
        int x = 10;
        int main(void) {
            goto x;
               //^ Undefined label 'x'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_10_invalid_types_conflicting_function_linkage() {
    assert_error(
        r#"
        int foo(void);
        int main(void) {
            return foo();
        }
        static int foo(void) {
                 //^^^ Function 'foo' was previously declared as non-static
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_10_invalid_types_conflicting_function_linkage_2() {
    assert_error(
        r#"
        int main(void) {
            int foo(void);
            return foo();
        }
        static int foo(void) {
                 //^^^ Function 'foo' was previously declared as non-static
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_10_invalid_types_conflicting_global_definitions() {
    assert_error(
        r#"
        int foo = 3;
        int main(void) {
            return 0;
        }
        int foo = 4;
          //^^^ Variable 'foo' has conflicting definitions
    "#,
    );
}

#[test]
fn test_chapter_10_invalid_types_conflicting_variable_linkage() {
    assert_error(
        r#"
        
        static int foo;
        int main(void) {
            return foo;
        }
        int foo = 3;
          //^^^ Variable 'foo' has conflicting linkage
    "#,
    );
}

#[test]
fn test_chapter_10_invalid_types_conflicting_variable_linkage_2() {
    assert_error(
        r#"
        int main(void) {
            int x = 3;
            {
                extern int x;
            }
            return x;
        }
        static int x = 10;
                 //^ Variable 'x' has conflicting linkage
    "#,
    );
}

#[test]
fn test_chapter_10_invalid_types_extern_for_loop_counter() {
    assert_error(
        r#"
        int main(void) {
            int x = 0;
            for (extern int i = 0; i < 10; i = i + 1) {
               //^^^^^^ Declarations inside for loops can't have storage class
                x = x + 1;
            }
            return x;
        }
    "#,
    );
}

#[test]
fn test_chapter_10_invalid_types_extern_variable_initializer() {
    assert_error(
        r#"
        int main(void) {
            extern int i = 0;
                         //^ Initializers are not allowed in local extern variable declarations
            return i;
        }
    "#,
    );
}

#[test]
fn test_chapter_10_invalid_types_extra_credit_static_var_case() {
    assert_error(
        r#"
        int main(void) {
            static int i = 0;
            switch(0) {
                case i: return 0;
                   //^ case label does not reduce to an integer constant
            }
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_10_invalid_types_non_constant_static_initializer() {
    assert_error(
        r#"
        int a = 10;
        int b = 1 + a;
              //^^^^^ Non-constant initializer on local static variable
        int main(void) {
            return b;
        }
    "#,
    );
}

#[test]
fn test_chapter_10_invalid_types_non_constant_static_local_initializer() {
    assert_error(
        r#"
        int main(void) {
            int a = 1;
            static int b = a * 2;
                         //^^^^^ Non-constant initializer on local static variable
            return b;
        }
    "#,
    );
}

#[test]
fn test_chapter_10_invalid_types_redeclare_file_scope_var_as_fun() {
    assert_error(
        r#"
        int foo = 10;
        int main(void) {
            int foo(void);
              //^^^ Conflicting declaration types for 'foo'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_10_invalid_types_redeclare_fun_as_file_scope_var() {
    assert_error(
        r#"
        int foo(void);
        int foo;
          //^^^ Variable 'foo' is already declared with a different type
        int main(void) {
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_10_invalid_types_redeclare_fun_as_var() {
    assert_error(
        r#"
        int foo(void) {
            return 0;
        }
        int main(void) {
            extern int foo;
                     //^^^ Name 'foo' was already declared
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_10_invalid_types_static_block_scope_function_declaration() {
    assert_error(
        r#"
        int main(void) {
            static int foo(void);
          //^^^^^^ Block scoped function declarations can't be static
            return foo();
        }
        static int foo(void) {
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_10_invalid_types_static_for_loop_counter() {
    assert_error(
        r#"
        int main(void) {
            int x = 0;
            for (static int i = 0; i < 10; i = i + 1) {
               //^^^^^^ Declarations inside for loops can't have storage class
                x = x + 1;
            }
            return x;
        }
    "#,
    );
}

#[test]
fn test_chapter_10_invalid_types_use_file_scope_variable_as_fun() {
    assert_error(
        r#"
        
        extern int foo;
        int main(void) {
            return foo();
                 //^^^ Variable used as function name
        }
    "#,
    );
}

#[test]
fn test_chapter_11_invalid_labels_extra_credit_bitshift_duplicate_cases() {
    assert_error(
        r#"
        int main(void) {
            int x = 100;
            switch (x << 2l) {
                case 34359738768l:
                    return 1;
                case 400:
                   //^^^ duplicate case value
                    return 0;
            }
            return 10;
        }
    "#,
    );
}

#[test]
fn test_chapter_11_invalid_labels_extra_credit_switch_duplicate_cases() {
    assert_error(
        r#"
        int switch_statement(int i) {
            switch(i) {
                case 0: return 0;
                case 17179869184: return 0;
                   //^^^^^^^^^^^ duplicate case value
                default: return 1;
            }
        }
        int main(void) {
            return switch_statement(0);
        }
    "#,
    );
}

#[test]
fn test_chapter_11_invalid_labels_extra_credit_switch_duplicate_cases_2() {
    assert_error(
        r#"
        int switch_statement(int i) {
            switch((long) i) {
                case 100l: return 0;
                case 100: return 0;
                   //^^^ duplicate case value
                default: return 1;
            }
        }
        int main(void) {
            return switch_statement(100);
        }
    "#,
    );
}

#[test]
fn test_chapter_11_invalid_types_call_long_as_function() {
    assert_error(
        r#"
        long x(void);
        int main(void) {
            long x = 0;
            return x();
                 //^ Variable used as function name
        }
    "#,
    );
}

#[test]
fn test_chapter_11_invalid_types_cast_lvalue() {
    assert_error(
        r#"
        int main(void) {
            int i = 0;
            i = (long) i = 10;
              //^^^^^^^^ Expression is not assignable
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_11_invalid_types_conflicting_function_types() {
    assert_error(
        r#"
        int foo(int a);
        int main(void) {
            return 0;
        }
        int foo(long a);
          //^^^ Conflicting declaration types for 'foo'
    "#,
    );
}

#[test]
fn test_chapter_11_invalid_types_conflicting_global_types() {
    assert_error(
        r#"
        int foo = 3;
        long foo;
           //^^^ Variable 'foo' is already declared with a different type
        int main(void) {
            return foo;
        }
    "#,
    );
}

#[test]
fn test_chapter_11_invalid_types_conflicting_variable_types() {
    assert_error(
        r#"
        long a;
        int main(void) {
            extern int a;
                     //^ Name 'a' was already declared
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_12_invalid_labels_extra_credit_switch_duplicate_cases() {
    assert_error(
        r#"
        int main(void) {
            unsigned int ui = 10u;
            switch(ui) {
                case 4294967295u:
                    return 0;
                case 1099511627775l:
                   //^^^^^^^^^^^^^^ duplicate case value
                    return 1;
                default: return 2;
            }
        }
    "#,
    );
}

#[test]
fn test_chapter_12_invalid_types_conflicting_signed_unsigned() {
    assert_error(
        r#"
        unsigned x;
        int x;
          //^ Variable 'x' is already declared with a different type
        int main(void) {
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_12_invalid_types_conflicting_uint_ulong() {
    assert_error(
        r#"
        
        unsigned int foo(void);
        unsigned long foo(void) {
                    //^^^ Conflicting declaration types for 'foo'
            return 0;
        }
        int main(void) {
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_13_invalid_types_complement_double() {
    assert_error(
        r#"
        int main(void) {
            double d = ~10.0;
                      //^^^^ Unary operator requires an integer type
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_13_invalid_types_extra_credit_bitwise_and() {
    assert_error(
        r#"
        int main(void) {
            double d = 10.0 & -1;
                     //^^^^ Operator requires integer operands
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_13_invalid_types_extra_credit_bitwise_or() {
    assert_error(
        r#"
        int main(void) {
            double d = 0.0 | -0.0;
                     //^^^ Operator requires integer operands
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_13_invalid_types_extra_credit_bitwise_shift_double() {
    assert_error(
        r#"
        int main(void) {
            double d = 5.0 << 3;
                     //^^^ Operator requires integer operands
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_13_invalid_types_extra_credit_bitwise_shift_double_2() {
    assert_error(
        r#"
        int main(void) {
            return 1 << 2.0;
                      //^^^ Operator requires integer operands
        }
    "#,
    );
}

#[test]
fn test_chapter_13_invalid_types_extra_credit_bitwise_xor() {
    assert_error(
        r#"
        int main(void) {
            return 1e10 ^ -1e10;
                 //^^^^ Operator requires integer operands
        }
    "#,
    );
}

#[test]
fn test_chapter_13_invalid_types_extra_credit_compound_bitwise_and() {
    assert_error(
        r#"
        int main(void) {
            double d = 1.0;
            d &= 0;
          //^ Assign compound operation requires integer operands
            return (int) d;
        }
    "#,
    );
}

#[test]
fn test_chapter_13_invalid_types_extra_credit_compound_bitwise_xor() {
    assert_error(
        r#"
        int main(void) {
            int i = 0;
            i |= 2.0;
               //^^^ Assign compound operation requires integer operands
            return (int) i;
        }
    "#,
    );
}

#[test]
fn test_chapter_13_invalid_types_extra_credit_compound_left_bitshift() {
    assert_error(
        r#"
        int main(void) {
            double d = 1.0;
            d <<= 1;
          //^ Assign compound operation requires integer operands
            return d;
        }
    "#,
    );
}

#[test]
fn test_chapter_13_invalid_types_extra_credit_compound_mod() {
    assert_error(
        r#"
        
        int main(void) {
            double d = 5.0;
            d %= 2;
          //^ Assign compound operation requires integer operands
            return (int) d;
        }
    "#,
    );
}

#[test]
fn test_chapter_13_invalid_types_extra_credit_compound_mod_2() {
    assert_error(
        r#"
        
        int main(void) {
            int i = 5;
            i %= 1.0;
               //^^^ Assign compound operation requires integer operands
            return i;
        }
    "#,
    );
}

#[test]
fn test_chapter_13_invalid_types_extra_credit_compound_right_bitshift() {
    assert_error(
        r#"
        int main(void) {
            int i = 1000;
            i >>= 2.0;
                //^^^ Assign compound operation requires integer operands
            return i;
        }
    "#,
    );
}

#[test]
fn test_chapter_13_invalid_types_extra_credit_switch_double_case() {
    assert_error(
        r#"
        int main(void) {
            int x = 10;
            switch (x) {
                case 1.0: return 0;
                   //^^^ case label does not reduce to an integer constant
                default: return 4;
            }
        }
    "#,
    );
}

#[test]
fn test_chapter_13_invalid_types_extra_credit_switch_on_double() {
    assert_error(
        r#"
        int main(void) {
            double d = 10;
            switch (d) {
                  //^ Switch statement requires an integer expression
                case 10: return 0;
            }
            return 1;
        }
    "#,
    );
}

#[test]
fn test_chapter_13_invalid_types_mod_double() {
    assert_error(
        r#"
        int main(void) {
            double d = 10.0;
            d = d % 3;
              //^ Operator requires integer operands
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_13_invalid_types_mod_double_2() {
    assert_error(
        r#"
        int main(void) {
            double e = 3.0 % 5;
                     //^^^ Operator requires integer operands
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_14_invalid_declarations_extra_credit_addr_of_label() {
    assert_error(
        r#"
        
        int main(void) {
            int x = 0;
            lbl:
            x = 1;
            if (&lbl == 0) {
               //^^^ Undeclared variable 'lbl'
                return 1;
            }
            goto lbl;
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_14_invalid_declarations_extra_credit_deref_label() {
    assert_error(
        r#"
        
        int main(void) {
            lbl:
            *lbl;
           //^^^ Undeclared variable 'lbl'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_14_invalid_types_address_of_address() {
    assert_error(
        r#"
        int main(void) {
            int x = 0;
            int *y = &x;
            int **z = &(&x);
                    //^^^^^ Can't take address of non-lvalue!
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_14_invalid_types_address_of_assignment() {
    assert_error(
        r#"
        int main(void) {
            int x = 0;
            int y = 0;
            int *ptr = &(x = y);
                     //^^^^^^^^ Can't take address of non-lvalue!
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_14_invalid_types_address_of_constant() {
    assert_error(
        r#"
        
        int main(void) {
            int *ptr = &10;
                     //^^^ Can't take address of non-lvalue!
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_14_invalid_types_address_of_ternary() {
    assert_error(
        r#"
        int main(void) {
            int x = 1;
            int y = 2;
            int z = 3;
            int *ptr = &(x ? y : z);
                     //^^^^^^^^^^^^ Can't take address of non-lvalue!
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_14_invalid_types_assign_int_to_pointer() {
    assert_error(
        r#"
        int main(void) {
            int *x;
            x = 10;
              //^^ Cannot convert type for assignment!
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_14_invalid_types_assign_int_var_to_pointer() {
    assert_error(
        r#"
        int main(void)
        {
            int x = 0;
            int *ptr = x;
                     //^ Cannot convert type for assignment!
        }
    "#,
    );
}

#[test]
fn test_chapter_14_invalid_types_assign_to_address() {
    assert_error(
        r#"
        int main(void)
        {
            int x = 0;
            &x = 10;
          //^^ Expression is not assignable
        }
    "#,
    );
}

#[test]
fn test_chapter_14_invalid_types_assign_wrong_pointer_type() {
    assert_error(
        r#"
        int main(void)
        {
            double *d = 0;
            long *l = 0;
            l = d;
              //^ Cannot convert type for assignment!
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_14_invalid_types_bad_null_pointer_constant() {
    assert_error(
        r#"
        int main(void)
        {
            int *x = 0.0;
                   //^^^ Cannot convert type for assignment!
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_14_invalid_types_cast_double_to_pointer() {
    assert_error(
        r#"
        int main(void) {
            double d = 0.0;
            int *x = (int *) d;
                           //^ Cannot cast a double to a pointer
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_14_invalid_types_cast_pointer_to_double() {
    assert_error(
        r#"
        int main(void) {
            int *x;
            double d = (double) x;
                              //^ Cannot cast a pointer to a double
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_14_invalid_types_compare_mixed_pointer_types() {
    assert_error(
        r#"
        
        int main(void) {
            int *x = 0ul;
            unsigned *y = 0ul;
            return x == y;
                 //^^^^^^ Expressions have incompatible types
        }
    "#,
    );
}

#[test]
fn test_chapter_14_invalid_types_compare_pointer_to_ulong() {
    assert_error(
        r#"
        
        int main(void) {
            int *ptr = 0ul;
            unsigned long ul = 0ul;
            return ptr == ul;
                 //^^^^^^^^^ Expressions have incompatible types
        }
    "#,
    );
}

#[test]
fn test_chapter_14_invalid_types_complement_pointer() {
    assert_error(
        r#"
        
        int main(void) {
            int *x = 0;
            return (int) ~x;
                        //^ Unary operator requires an integer type
        }
    "#,
    );
}

#[test]
fn test_chapter_14_invalid_types_dereference_non_pointer() {
    assert_error(
        r#"
        
        int main(void) {
            unsigned long l = 100ul;
            return *l;
                 //^^ Cannot dereference a non-pointer
        }
    "#,
    );
}

#[test]
fn test_chapter_14_invalid_types_divide_pointer() {
    assert_error(
        r#"
        
        int main(void)
        {
            int x = 10;
            int *y = &x;
            (y / 8);
           //^ Operator is invalid
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_14_invalid_types_extra_credit_bitwise_and_pointer() {
    assert_error(
        r#"
        
        int main(void) {
            long *ptr = 0;
            10 & ptr;
               //^^^ Operator requires integer operands
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_14_invalid_types_extra_credit_bitwise_compound_assign_to_pointer() {
    assert_error(
        r#"
        
        int main(void) {
            int x = 0;
            int *ptr = &x;
            ptr &= 0;
          //^^^ Assign compound operation requires integer operands
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_14_invalid_types_extra_credit_bitwise_compound_assign_with_pointer() {
    assert_error(
        r#"
        int main(void) {
            int *null = 0;
            int x = 100;
            x |= null;
               //^^^^ Assign compound operation requires integer operands
            return 1;
        }
    "#,
    );
}

#[test]
fn test_chapter_14_invalid_types_extra_credit_bitwise_lshift_pointer() {
    assert_error(
        r#"
        
        int main(void) {
            int *ptr = 0;
            int i = 1000;
            i >> ptr;
               //^^^ Operator requires integer operands
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_14_invalid_types_extra_credit_bitwise_or_pointer() {
    assert_error(
        r#"
        
        int main(void) {
            int *x = 0;
            int *y = 0;
            x | y;
          //^ Operator requires integer operands
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_14_invalid_types_extra_credit_bitwise_rshift_pointer() {
    assert_error(
        r#"
        
        int main(void) {
            int *x = 0;
            return (int) (x >> 10);
                        //^ Operator requires integer operands
        }
    "#,
    );
}

#[test]
fn test_chapter_14_invalid_types_extra_credit_bitwise_xor_pointer() {
    assert_error(
        r#"
        
        int main(void) {
            unsigned long *ptr = 0;
            long l = 100;
            ptr ^ l;
          //^^^ Operator requires integer operands
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_14_invalid_types_extra_credit_compound_assign_thru_ptr_not_lval() {
    assert_error(
        r#"
        int main(void) {
            int i = 100;
            int *ptr = &i;
            int *ptr2 = &(*ptr -= 10);
                      //^^^^^^^^^^^^^ Can't take address of non-lvalue!
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_14_invalid_types_extra_credit_compound_assignment_not_lval() {
    assert_error(
        r#"
        int main(void) {
            int i = 100;
            int *ptr = &(i += 200);
                     //^^^^^^^^^^^ Can't take address of non-lvalue!
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_14_invalid_types_extra_credit_compound_divide_pointer() {
    assert_error(
        r#"
        int main(void) {
            int *x = 0;
            int *y = 0;
            x /= y;
          //^ Assign compound operation cannot be used on pointer type
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_14_invalid_types_extra_credit_compound_mod_pointer() {
    assert_error(
        r#"
        int main(void) {
            int i = 10;
            int *ptr = &i;
            i %= ptr;
               //^^^ Assign compound operation requires integer operands
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_14_invalid_types_extra_credit_compound_multiply_pointer() {
    assert_error(
        r#"
        int main(void) {
            int *x = 0;
            x *= 2;
          //^ Assign compound operation cannot be used on pointer type
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_14_invalid_types_extra_credit_postfix_decr_not_lvalue() {
    assert_error(
        r#"
        int main(void) {
            int i = 10;
            int *ptr = &i--;
                     //^^^^ Can't take address of non-lvalue!
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_14_invalid_types_extra_credit_prefix_incr_not_lvalue() {
    assert_error(
        r#"
        int main(void) {
            int i = 10;
            int *ptr = &++i;
                     //^^^^ Can't take address of non-lvalue!
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_14_invalid_types_extra_credit_switch_on_pointer() {
    assert_error(
        r#"
        int main(void) {
            int *x = 0;
            switch(x) {
                 //^ Switch statement requires an integer expression
                case 0: return 0;
                default: return 1;
            }
        }
    "#,
    );
}

#[test]
fn test_chapter_14_invalid_types_invalid_pointer_initializer() {
    assert_error(
        r#"
        int main(void)
        {
            int *ptr = 140732898195768ul;
                     //^^^^^^^^^^^^^^^^^ Cannot convert type for assignment!
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_14_invalid_types_invalid_static_initializer() {
    assert_error(
        r#"
        
        static int *x = 10;
                      //^^ Invalid type of static declaration
        int main(void) {
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_14_invalid_types_multiply_pointers() {
    assert_error(
        r#"
        
        int main(void) {
            int *x = 0;
            int *y = x;
            (x * y);
           //^ Operator is invalid
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_14_invalid_types_multiply_pointers_2() {
    assert_error(
        r#"
        
        int main(void)
        {
            int *x = 0;
            (0 * x);
               //^ Operator is invalid
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_14_invalid_types_negate_pointer() {
    assert_error(
        r#"
        
        int main(void) {
            int *x = 0;
            -x;
           //^ Unary operator requires an non-pointer type
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_14_invalid_types_pass_pointer_as_int() {
    assert_error(
        r#"
        int f(int i) {
            return i;
        }
        int main(void) {
            int x;
            return f(&x);
                   //^^ Cannot convert type for assignment!
        }
    "#,
    );
}

#[test]
fn test_chapter_14_invalid_types_return_wrong_pointer_type() {
    assert_error(
        r#"
        int i;
        long *return_long_pointer(void) {
            return &i;
                 //^^ Cannot convert type for assignment!
        }
        int main(void) {
            long *l = return_long_pointer();
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_14_invalid_types_ternary_mixed_pointer_types() {
    assert_error(
        r#"
        int main(void) {
            long *x = 0;
            int *y = 0;
            int *result = 1 ? x : y;
                        //^^^^^^^^^ Cannot convert type for assignment!
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_15_invalid_types_add_two_pointers() {
    assert_error(
        r#"
        int main(void)
        {
            int *x = 0;
            int *y = 0;
            return (x + y == 0);
                      //^ Operator is invalid
        }
    "#,
    );
}

#[test]
fn test_chapter_15_invalid_types_assign_incompatible_pointer_types() {
    assert_error(
        r#"
        int main(void) {
            int four_element_array[4] = {1, 2, 3, 4};
            int (*arr)[3] = &four_element_array;
                          //^^^^^^^^^^^^^^^^^^^ Cannot convert type for assignment!
        }
    "#,
    );
}

#[test]
fn test_chapter_15_invalid_types_assign_to_array() {
    assert_error(
        r#"
        int main(void)
        {
            int arr[3] = {1, 2, 3};
            int arr2[3] = {4, 5, 6};
            arr = arr2;
          //^^^ Array is not assignable
            return arr[0];
        }
    "#,
    );
}

#[test]
fn test_chapter_15_invalid_types_assign_to_array_2() {
    assert_error(
        r#"
        int main(void)
        {
            int dim2[1][2] = {{1, 2}};
            int dim[2] = {3, 4};
            dim2[0] = dim;
          //^^^^^^^ Array is not assignable
            return dim[0];
        }
    "#,
    );
}

#[test]
fn test_chapter_15_invalid_types_assign_to_array_3() {
    assert_error(
        r#"
        int main(void) {
            int arr[3] = { 1, 2, 3};
            int (*ptr_to_array)[3];
            *ptr_to_array = arr;
          //^^^^^^^^^^^^^ Array is not assignable
        }
    "#,
    );
}

#[test]
fn test_chapter_15_invalid_types_bad_arg_type() {
    assert_error(
        r#"
        int foo(int **x) {
            return x[0][0];
        }
        int main(void) {
            int arr[1] = {10};
            return foo(&arr);
                     //^^^^ Cannot convert type for assignment!
        }
    "#,
    );
}

#[test]
fn test_chapter_15_invalid_types_cast_to_array_type() {
    assert_error(
        r#"
        int main(void)
        {
            int arr[10];
            return (int[10])arr;
                          //^^^ Cannot cast to an array type
        }
    "#,
    );
}

#[test]
fn test_chapter_15_invalid_types_cast_to_array_type_2() {
    assert_error(
        r#"
        int main(void)
        {
            long arr[10];
            return (int *[10])arr;
                            //^^^ Cannot cast to an array type
        }
    "#,
    );
}

#[test]
fn test_chapter_15_invalid_types_cast_to_array_type_3() {
    assert_error(
        r#"
        int main(void)
        {
            long arr[6];
            return ((long(([2])[3]))arr);
                                  //^^^ Cannot cast to an array type
        }
    "#,
    );
}

#[test]
fn test_chapter_15_invalid_types_compare_different_pointer_types() {
    assert_error(
        r#"
        int main(void)
        {
            long x = 10;
            long *ptr = &x + 1;
            long(*array_ptr)[10] = (long (*)[10]) &x;
            return array_ptr < ptr;
                             //^^^ Operator is invalid
        }
    "#,
    );
}

#[test]
fn test_chapter_15_invalid_types_compare_explicit_and_implict_addr() {
    assert_error(
        r#"
        int main(void)
        {
            int arr[10];
            return arr == &arr;
                 //^^^^^^^^^^^ Expressions have incompatible types
        }
    "#,
    );
}

#[test]
fn test_chapter_15_invalid_types_compare_pointer_to_int() {
    assert_error(
        r#"
        int main(void)
        {
            long *l = 0;
            return l <= 100ul;
                      //^^^^^ Operator is invalid
        }
    "#,
    );
}

#[test]
fn test_chapter_15_invalid_types_compare_pointer_to_zero() {
    assert_error(
        r#"
        int main(void)
        {
            int *x = 0;
            return x > 0;
                     //^ Operator is invalid
        }
    "#,
    );
}

#[test]
fn test_chapter_15_invalid_types_compound_initializer_for_scalar() {
    assert_error(
        r#"
        int main(void)
        {
            int x = {1, 2, 3};
                  //^^^^^^^^ Cannot initialize a scalar value with a compound initializer
            return x;
        }
    "#,
    );
}

#[test]
fn test_chapter_15_invalid_types_compound_initializer_for_static_scalar() {
    assert_error(
        r#"
        int main(void)
        {
            static int x = {1, 2, 3};
                         //^^^^^^^^ Cannot initialize a scalar value with a compound initializer
            return x;
        }
    "#,
    );
}

#[test]
fn test_chapter_15_invalid_types_compound_initializer_too_long_static() {
    assert_error(
        r#"
        int main(void) {
            static int arr[3] = {1, 2, 3, 4};
                              //^^^^^^^^^^^ Wrong number of element in the initializer
            return arr[2];
        }
    "#,
    );
}

#[test]
fn test_chapter_15_invalid_types_compound_inititializer_too_long() {
    assert_error(
        r#"
        int main(void) {
            int arr[3] = {1, 2, 3, 4};
                       //^^^^^^^^^^^ Wrong number of element in the initializer
            return arr[2];
        }
    "#,
    );
}

#[test]
fn test_chapter_15_invalid_types_conflicting_array_declarations() {
    assert_error(
        r#"
        int arr[6];
        int main(void) {
            return arr[0];
        }
        int arr[5];
          //^^^ Variable 'arr' is already declared with a different type
    "#,
    );
}

#[test]
fn test_chapter_15_invalid_types_conflicting_function_declarations() {
    assert_error(
        r#"
        int f(int arr[2][3]);
        int f(int arr[2][4]);
          //^ Conflicting declaration types for 'f'
    "#,
    );
}

#[test]
fn test_chapter_15_invalid_types_double_subscript() {
    assert_error(
        r#"
        int main(void) {
            int arr[3] = {4, 5, 6};
            return arr[2.0];
                 //^^^^^^^^ Subscript requires integer and pointer types
        }
    "#,
    );
}

#[test]
fn test_chapter_15_invalid_types_extra_credit_compound_add_double_to_pointer() {
    assert_error(
        r#"
        
        int main(void) {
            int arr[3] = {1, 2, 3};
            int *elem = arr;
            elem += 1.0;
                  //^^^ Assign compound operation on a pointer requires integer operand
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_15_invalid_types_extra_credit_compound_add_two_pointers() {
    assert_error(
        r#"
        
        int main(void) {
            int arr[3] = {1, 2, 3};
            int *elem0 = arr;
            int *elem1 = arr + 1;
            elem0 += elem1;
                   //^^^^^ Assign compound operation on a pointer requires integer operand
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_15_invalid_types_extra_credit_compound_assign_to_array() {
    assert_error(
        r#"
        int main(void) {
            int arr[3] = {1, 2, 3};
            arr -= 1;
          //^^^ Array is not assignable
            0;
        }
    "#,
    );
}

#[test]
fn test_chapter_15_invalid_types_extra_credit_compound_assign_to_nested_array() {
    assert_error(
        r#"
        int main(void) {
            long arr[2][2] = {{1, 2}, {3, 4}};
            arr[1] += 1;
          //^^^^^^ Array is not assignable
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_15_invalid_types_extra_credit_compound_sub_pointer_from_int() {
    assert_error(
        r#"
        
        int main(void) {
            int arr[3] = {1, 2, 3};
            int *elem = arr + 1;
            int i = 0;
            i -= elem;
               //^^^^ Assign compound operator cannot be a pointer type
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_15_invalid_types_extra_credit_postfix_incr_array() {
    assert_error(
        r#"
        int main(void) {
            int arr[3] = {1, 2, 3};
            arr++;
          //^^^ Array is not assignable
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_15_invalid_types_extra_credit_postfix_incr_nested_array() {
    assert_error(
        r#"
        int main(void) {
            int arr[2][3] = {{1, 2, 3}, {4, 5, 6}};
            arr[2]++;
          //^^^^^^ Array is not assignable
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_15_invalid_types_extra_credit_prefix_decr_array() {
    assert_error(
        r#"
        int main(void) {
            int arr[3] = {1, 2, 3};
            --arr;
            //^^^ Array is not assignable
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_15_invalid_types_extra_credit_prefix_decr_nested_array() {
    assert_error(
        r#"
        int main(void) {
            int arr[2][3] = {{1, 2, 3}, {4, 5, 6}};
            --arr[2];
            //^^^^^^ Array is not assignable
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_15_invalid_types_extra_credit_switch_on_array() {
    assert_error(
        r#"
        
        int main(void) {
            int arr[3] = {1, 2, 3};
            switch (arr) {
                  //^^^ Switch statement requires an integer expression
                default:
                    return 0;
            }
            return 1;
        }
    "#,
    );
}

#[test]
fn test_chapter_15_invalid_types_function_returns_array() {
    assert_error(
        r#"
        int(foo(void))[3][4];
          //^^^ A function cannot return array
    "#,
    );
}

#[test]
fn test_chapter_15_invalid_types_incompatible_elem_type_compound_init() {
    assert_error(
        r#"
        int main(void)
        {
            int *arr[3] = {0, 0, 1.0};
                               //^^^ Cannot convert type for assignment!
        }
    "#,
    );
}

#[test]
fn test_chapter_15_invalid_types_incompatible_elem_type_static_compound_init() {
    assert_error(
        r#"
        
        int *arr[3] = {0, 0, 1.0};
                           //^^^ Invalid type of static declaration
        int main(void)
        {
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_15_invalid_types_null_ptr_array_initializer() {
    assert_error(
        r#"
        int main(void)
        {
            int arr[1] = 0;
                       //^ Cannot convert type for assignment!
            return arr[0];
        }
    "#,
    );
}

#[test]
fn test_chapter_15_invalid_types_null_ptr_static_array_initializer() {
    assert_error(
        r#"
        int main(void)
        {
            static int arr[1] = 0;
                              //^ Invalid type of static declaration
            return arr[0];
        }
    "#,
    );
}

#[test]
fn test_chapter_15_invalid_types_scalar_initializer_for_array() {
    assert_error(
        r#"
        int main(void)
        {
            int arr[1] = 4;
                       //^ Cannot convert type for assignment!
            return arr[0];
        }
    "#,
    );
}

#[test]
fn test_chapter_15_invalid_types_scalar_initializer_for_static_array() {
    assert_error(
        r#"
        
        double arr[3] = 1.0;
                      //^^^ Invalid type of static declaration
        int main(void)
        {
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_15_invalid_types_static_non_const_array() {
    assert_error(
        r#"
        int foo(int p) {
            static int arr[3] = { p, p + 1, 0};
                                //^ Non-constant initializer on local static variable
            return arr[2];
        }
        int main(void) {
            return foo(5);
        }
    "#,
    );
}

#[test]
fn test_chapter_15_invalid_types_sub_different_pointer_types() {
    assert_error(
        r#"
        int main(void)
        {
            long x[10];
            long *ptr = x;
            unsigned long *ptr2 = (unsigned long *)ptr;
            return ptr - ptr2;
                       //^^^^ Invalid pointer operator type
        }
    "#,
    );
}

#[test]
fn test_chapter_15_invalid_types_sub_double_from_ptr() {
    assert_error(
        r#"
        int main(void)
        {
            int *y = 0;
            return (y - 0.0 == 0.0);
                      //^^^ Operator is invalid
        }
    "#,
    );
}

#[test]
fn test_chapter_15_invalid_types_sub_ptr_from_int() {
    assert_error(
        r#"
        int main(void)
        {
            int *x = 0;
            return 0 - x == 0;
                     //^ Cannot substract a pointer from an int
        }
    "#,
    );
}

#[test]
fn test_chapter_15_invalid_types_subscript_both_pointers() {
    assert_error(
        r#"
        int main(void)
        {
            int x = 10;
            int *ptr = &x;
            int *subscript = 0;
            return ptr[subscript];
                 //^^^^^^^^^^^^^^ Subscript requires integer and pointer types
        }
    "#,
    );
}

#[test]
fn test_chapter_15_invalid_types_subscript_non_ptr() {
    assert_error(
        r#"
        int main(void) {
            int a = 3;
            return a[4];
                 //^^^^ Subscript requires integer and pointer types
        }
    "#,
    );
}

#[test]
fn test_chapter_16_invalid_labels_extra_credit_duplicate_case_char_const() {
    assert_error(
        r#"
        int main(void) {
            static int i = 120;
            switch (i) {
                case 'x':
                    return 1;
                case 120:
                   //^^^ duplicate case value
                    return 2;
                default:
                    return 3;
            }
        }
    "#,
    );
}

#[test]
fn test_chapter_16_invalid_types_assign_to_string_literal() {
    assert_error(
        r#"
        int main(void) {
            "foo" = "bar";
          //^^^^^ Array is not assignable
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_16_invalid_types_char_and_schar_conflict() {
    assert_error(
        r#"
        char c = 10;
        int main(void)
        {
            extern signed char c;
                             //^ Name 'c' was already declared
            return c;
        }
    "#,
    );
}

#[test]
fn test_chapter_16_invalid_types_char_and_uchar_conflict() {
    assert_error(
        r#"
        int foo(unsigned char c) {
            return c;
        }
        int main(void) {
            return foo(0);
        }
        int foo(char c);
          //^^^ Conflicting declaration types for 'foo'
    "#,
    );
}

#[test]
fn test_chapter_16_invalid_types_compound_initializer_for_pointer() {
    assert_error(
        r#"
        
        int main(void) {
            char *ptr = {'a', 'b', 'c'};
                      //^^^^^^^^^^^^^^ Cannot initialize a scalar value with a compound initializer
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_16_invalid_types_extra_credit_bit_shift_string() {
    assert_error(
        r#"
        
        int main(void) {
            "foo" << 3;
          //^^^^^ Operator requires integer operands
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_16_invalid_types_extra_credit_bitwise_operation_on_string() {
    assert_error(
        r#"
        
        int main(void) {
            "My string" & 100;
          //^^^^^^^^^^^ Operator requires integer operands
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_16_invalid_types_extra_credit_case_statement_string() {
    assert_error(
        r#"
        
        int main(void) {
            switch (0) {
                case "foo":
                   //^^^^^ case label does not reduce to an integer constant
                    return 1;
                default:
                    return 0;
            }
        }
    "#,
    );
}

#[test]
fn test_chapter_16_invalid_types_extra_credit_compound_assign_from_string() {
    assert_error(
        r#"
        
        int main(void) {
            char * s = "some string ";
            s += "another str";
               //^^^^^^^^^^^^^ Assign compound operation on a pointer requires integer operand
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_16_invalid_types_extra_credit_compound_assign_to_string() {
    assert_error(
        r#"
        
        int main(void) {
            "My string" += 1;
          //^^^^^^^^^^^ Array is not assignable
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_16_invalid_types_extra_credit_postfix_incr_string() {
    assert_error(
        r#"
        
        int main(void) {
            "foo"++;
          //^^^^^ Array is not assignable
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_16_invalid_types_extra_credit_prefix_incr_string() {
    assert_error(
        r#"
        
        int main(void) {
            ++"foo";
            //^^^^^ Array is not assignable
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_16_invalid_types_extra_credit_switch_on_string() {
    assert_error(
        r#"
        
        int main(void) {
            switch ("foo") {
                  //^^^^^ Switch statement requires an integer expression
                default:
                return 0;
            }
        }
    "#,
    );
}

#[test]
fn test_chapter_16_invalid_types_implicit_conversion_between_char_pointers() {
    assert_error(
        r#"
        
        int main(void) {
            char *c = 0;
            signed char *s = c;
                           //^ Cannot convert type for assignment!
            return (int) s;
        }
    "#,
    );
}

#[test]
fn test_chapter_16_invalid_types_implicit_conversion_pointers_to_different_size_arrays() {
    assert_error(
        r#"
        int main(void) {
            char(*string_pointer)[10] = &"x";
                                      //^^^^ Cannot convert type for assignment!
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_16_invalid_types_negate_char_pointer() {
    assert_error(
        r#"
        
        int main(void) {
            char *x = "foo";
            return -x;
                  //^ Unary operator requires an non-pointer type
        }
    "#,
    );
}

#[test]
fn test_chapter_16_invalid_types_string_initializer_for_multidim_array() {
    assert_error(
        r#"
        char arr[3][3] = "hello";
                       //^^^^^^^ Can't initialize a non-character type with a string literal
        int main(void)
        {
            return arr[0][2];
        }
    "#,
    );
}

#[test]
fn test_chapter_16_invalid_types_string_initializer_too_long() {
    assert_error(
        r#"
        int main(void) {
            char too_long[3] = "abcd";
                             //^^^^^^ Initializer string has more characters than destination array
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_16_invalid_types_string_initializer_too_long_nested() {
    assert_error(
        r#"
        int main(void) {
            char array[3][3] = {"a", "bcde"};
                                   //^^^^^^ Initializer string has more characters than destination array
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_16_invalid_types_string_initializer_too_long_nested_static() {
    assert_error(
        r#"
        char array[3][3] = {"a", "bcde"};
                               //^^^^^^ Initializer string has more characters than destination array
        int main(void)
        {
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_16_invalid_types_string_initializer_too_long_static() {
    assert_error(
        r#"
        int main(void) {
            static char too_long[3] = "abcd";
                                    //^^^^^^ Initializer string has more characters than destination array
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_16_invalid_types_string_initializer_wrong_type() {
    assert_error(
        r#"
        int main(void) {
            long ints[4] = "abc";
                         //^^^^^ Can't initialize a non-character type with a string literal
            return ints[1];
        }
    "#,
    );
}

#[test]
fn test_chapter_16_invalid_types_string_initializer_wrong_type_nested() {
    assert_error(
        r#"
        int main(void)
        {
            unsigned int nested[1][2] = {"a"};
                                       //^^^ Can't initialize a non-character type with a string literal
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_16_invalid_types_string_initializer_wrong_type_nested_static() {
    assert_error(
        r#"
        int main(void)
        {
            static long int nested[1][2] = {"a"};
                                          //^^^ Can't initialize a non-character type with a string literal
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_16_invalid_types_string_literal_is_plain_char_pointer() {
    assert_error(
        r#"
        int main(void) {
            signed char *ptr = "foo";
                             //^^^^^ Cannot convert type for assignment!
            return 0;
        }
    "#,
    );
}

#[test]
fn test_chapter_16_invalid_types_string_literal_is_plain_char_pointer_static() {
    assert_error(
        r#"
        int main(void) {
            static signed char *ptr = "foo";
                                    //^^^^^ Can't initialize a non-character pointer to a string literal
            return 0;
        }
    "#,
    );
}
