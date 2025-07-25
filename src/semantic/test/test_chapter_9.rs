use super::assert_error;

#[test]
fn test_invalid_declarations_assign_to_fun_call() {
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
fn test_invalid_declarations_decl_params_with_same_name() {
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
fn test_invalid_declarations_extra_credit_call_label_as_function() {
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
fn test_invalid_declarations_extra_credit_compound_assign_to_fun_call() {
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
fn test_invalid_declarations_extra_credit_decrement_fun_call() {
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
fn test_invalid_declarations_extra_credit_increment_fun_call() {
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
fn test_invalid_declarations_nested_function_definition() {
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
fn test_invalid_declarations_params_with_same_name() {
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
fn test_invalid_declarations_redefine_fun_as_var() {
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
fn test_invalid_declarations_redefine_parameter() {
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
fn test_invalid_declarations_redefine_var_as_fun() {
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
fn test_invalid_declarations_undeclared_fun() {
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
fn test_invalid_declarations_wrong_parameter_names() {
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
fn test_invalid_labels_extra_credit_goto_cross_function() {
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
fn test_invalid_labels_extra_credit_goto_function() {
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
fn test_invalid_types_assign_fun_to_variable() {
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
fn test_invalid_types_assign_value_to_function() {
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
fn test_invalid_types_call_variable_as_function() {
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
fn test_invalid_types_conflicting_function_declarations() {
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
fn test_invalid_types_conflicting_local_function_declaration() {
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
fn test_invalid_types_divide_by_function() {
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
fn test_invalid_types_extra_credit_bitwise_op_function() {
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
fn test_invalid_types_extra_credit_compound_assign_function_lhs() {
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
fn test_invalid_types_extra_credit_compound_assign_function_rhs() {
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
fn test_invalid_types_extra_credit_postfix_incr_fun_name() {
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
fn test_invalid_types_extra_credit_prefix_decr_fun_name() {
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
fn test_invalid_types_extra_credit_switch_on_function() {
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
fn test_invalid_types_multiple_function_definitions() {
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
fn test_invalid_types_multiple_function_definitions_2() {
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
fn test_invalid_types_too_few_args() {
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
fn test_invalid_types_too_many_args() {
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
