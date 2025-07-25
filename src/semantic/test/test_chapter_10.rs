use super::assert_error;

#[test]
fn test_invalid_declarations_conflicting_local_declarations() {
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
fn test_invalid_declarations_extern_follows_local_var() {
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
fn test_invalid_declarations_extern_follows_static_local_var() {
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
fn test_invalid_declarations_local_var_follows_extern() {
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
fn test_invalid_declarations_out_of_scope_extern_var() {
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
fn test_invalid_declarations_redefine_param_as_identifier_with_linkage() {
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
fn test_invalid_declarations_undeclared_global_variable() {
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
fn test_invalid_labels_extra_credit_goto_global_var() {
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
fn test_invalid_types_conflicting_function_linkage() {
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
fn test_invalid_types_conflicting_function_linkage_2() {
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
fn test_invalid_types_conflicting_global_definitions() {
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
fn test_invalid_types_conflicting_variable_linkage() {
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
fn test_invalid_types_conflicting_variable_linkage_2() {
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
fn test_invalid_types_extern_for_loop_counter() {
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
fn test_invalid_types_extern_variable_initializer() {
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
fn test_invalid_types_extra_credit_static_var_case() {
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
fn test_invalid_types_non_constant_static_initializer() {
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
fn test_invalid_types_non_constant_static_local_initializer() {
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
fn test_invalid_types_redeclare_file_scope_var_as_fun() {
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
fn test_invalid_types_redeclare_fun_as_file_scope_var() {
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
fn test_invalid_types_redeclare_fun_as_var() {
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
fn test_invalid_types_static_block_scope_function_declaration() {
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
fn test_invalid_types_static_for_loop_counter() {
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
fn test_invalid_types_use_file_scope_variable_as_fun() {
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
