use super::{assert_error, assert_parse};

#[test]
fn test_invalid_declarations_assign_to_fun_call() {
    let src = r#"
        int x(void);
        int main(void) {
            x() = 1;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [x]
            ╰── Function [main]
                ╰── Body
                    ├── <14> Assign [=]
                    │   ├── <11> FunctionCall [x]
                    │   ╰── Constant Int [1]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_declarations_decl_params_with_same_name() {
    let src = r#"
        int foo(int a, int a);
        int main(void) {
            return foo(1, 2);
        }
        int foo(int a, int b) {
            return a + b;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [foo]
            │   ╰── Parameters
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── a
            │       │   ╰── Type
            │       │       ╰── Int
            │       ╰── Param
            │           ├── Name
            │           │   ╰── a
            │           ╰── Type
            │               ╰── Int
            ├── Function [main]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <19> FunctionCall [foo]
            │               ├── Constant Int [1]
            │               ╰── Constant Int [2]
            ╰── Function [foo]
                ├── Parameters
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── a
                │   │   ╰── Type
                │   │       ╰── Int
                │   ╰── Param
                │       ├── Name
                │       │   ╰── b
                │       ╰── Type
                │           ╰── Int
                ╰── Body
                    ╰── Return
                        ╰── <38>  [+]
                            ├── <34> Var [a]
                            ╰── <37> Var [b]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_declarations_extra_credit_call_label_as_function() {
    let src = r#"
        int main(void) {
            int x = 1;
            a:
            x = x + 1;
            a();
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
                    │       ╰── Constant Int [1]
                    ├── Label [a]
                    │   ╰── <20> Assign [=]
                    │       ├── <13> Var [x]
                    │       ╰── <19>  [+]
                    │           ├── <16> Var [x]
                    │           ╰── Constant Int [1]
                    ├── <24> FunctionCall [a]
                    ╰── Return
                        ╰── <27> Var [x]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_declarations_extra_credit_compound_assign_to_fun_call() {
    let src = r#"
        int x(void);
        int main(void) {
            x() += 1;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [x]
            ╰── Function [main]
                ╰── Body
                    ├── <14> Assign [+=]
                    │   ├── <11> FunctionCall [x]
                    │   ╰── Constant Int [1]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_declarations_extra_credit_decrement_fun_call() {
    let src = r#"
        int x(void);
        int main(void) {
            x()--;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [x]
            ╰── Function [main]
                ╰── Body
                    ╰── <13> Postfix [--]
                        ╰── <11> FunctionCall [x]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_declarations_extra_credit_increment_fun_call() {
    let src = r#"
        int x(void);
        int main(void) {
            ++x();
        }
    "#;
    let expected = r#"
        Program
            ├── Function [x]
            ╰── Function [main]
                ╰── Body
                    ╰── <13> Unary [++]
                        ╰── <12> FunctionCall [x]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_declarations_nested_function_definition() {
    let src = r#"
        int main(void) {
            int foo(void) {
                return 1;
            }
            return foo();
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── Function [foo]
                    │   ╰── Body
                    │       ╰── Return
                    │           ╰── Constant Int [1]
                    ╰── Return
                        ╰── <14> FunctionCall [foo]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_declarations_params_with_same_name() {
    let src = r#"
        
        int foo(int a, int a) {
            return a;
        }
        int main(void) {
            return foo(1, 2);
        }
    "#;
    let expected = r#"
        Program
            ├── Function [foo]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── a
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── a
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <12> Var [a]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <23> FunctionCall [foo]
                            ├── Constant Int [1]
                            ╰── Constant Int [2]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_declarations_redefine_fun_as_var() {
    let src = r#"
        int main(void) {
            int foo(void);
            int foo = 1;
            return foo;
        }
        int foo(void) {
            return 1;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [main]
            │   ╰── Body
            │       ├── Function [foo]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── foo
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── Constant Int [1]
            │       ╰── Return
            │           ╰── <17> Var [foo]
            ╰── Function [foo]
                ╰── Body
                    ╰── Return
                        ╰── Constant Int [1]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_declarations_redefine_parameter() {
    let src = r#"
        int foo(int a) {
            int a = 5;
            return a;
        }
        int main(void) {
            return foo(3);
        }
    "#;
    let expected = r#"
        Program
            ├── Function [foo]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── a
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── a
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── Constant Int [5]
            │       ╰── Return
            │           ╰── <15> Var [a]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <25> FunctionCall [foo]
                            ╰── Constant Int [3]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_declarations_redefine_var_as_fun() {
    let src = r#"
        int main(void) {
            int foo = 1;
            int foo(void);
            return foo;
        }
        int foo(void) {
            return 1;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [main]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── foo
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── Constant Int [1]
            │       ├── Function [foo]
            │       ╰── Return
            │           ╰── <17> Var [foo]
            ╰── Function [foo]
                ╰── Body
                    ╰── Return
                        ╰── Constant Int [1]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_declarations_undeclared_fun() {
    let src = r#"
        int main(void) {
            return foo(3);
        }
        int foo(int a) {
            return 1;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [main]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <7> FunctionCall [foo]
            │               ╰── Constant Int [3]
            ╰── Function [foo]
                ├── Parameters
                │   ╰── Param
                │       ├── Name
                │       │   ╰── a
                │       ╰── Type
                │           ╰── Int
                ╰── Body
                    ╰── Return
                        ╰── Constant Int [1]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_declarations_wrong_parameter_names() {
    let src = r#"
        int foo(int a);
        int main(void) {
            return foo(3);
        }
        int foo(int x) {
            return a;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [foo]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── a
            │           ╰── Type
            │               ╰── Int
            ├── Function [main]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <15> FunctionCall [foo]
            │               ╰── Constant Int [3]
            ╰── Function [foo]
                ├── Parameters
                │   ╰── Param
                │       ├── Name
                │       │   ╰── x
                │       ╰── Type
                │           ╰── Int
                ╰── Body
                    ╰── Return
                        ╰── <27> Var [a]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_labels_extra_credit_goto_cross_function() {
    let src = r#"
        int foo(void) {
            label:
                return 0;
        }
        int main(void) {
            goto label;
            return 1;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [foo]
            │   ╰── Body
            │       ╰── Label [label]
            │           ╰── Return
            │               ╰── Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ├── Goto [label]
                    ╰── Return
                        ╰── Constant Int [1]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_labels_extra_credit_goto_function() {
    let src = r#"
        int foo(void) {
            return 3;
        }
        int main(void) {
            goto foo;
            return 3;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [foo]
            │   ╰── Body
            │       ╰── Return
            │           ╰── Constant Int [3]
            ╰── Function [main]
                ╰── Body
                    ├── Goto [foo]
                    ╰── Return
                        ╰── Constant Int [3]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_parse_call_non_identifier() {
    assert_error(
        r#"
        int main(void) {
            return 1();
                  //^ Expected ';', but found '('
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_decl_wrong_closing_delim() {
    assert_error(
        r#"
        int foo(int x, int y} { return x + y; }
                          //^ Expected ')', but found '}'
        int main(void) { return 0;}
    "#,
    );
}

#[test]
fn test_invalid_parse_fun_decl_for_loop() {
    assert_error(
        r#"
        int main(void) {
            for (int f(void); ; ) {
               //^^^^^^^^^^^^ Expected variable declaration, but found function declaration
                return 0;
            }
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_funcall_wrong_closing_delim() {
    assert_error(
        r#"
        int foo(int x, int y) {
            return x + y;
        }
        int main(void) { return foo(1, 2};}
                                      //^ Expected ')', but found '}'
    "#,
    );
}

#[test]
fn test_invalid_parse_function_call_declaration() {
    assert_error(
        r#"
        int foo(int a) {
            return 0;
        }
        int main(void) {
            return foo(int a);
                     //^^^ Expected expression, but found 'int'
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_function_returning_function() {
    assert_error(
        r#"
        int foo(void)(void);
                   //^ Expected ';', but found '('
        int main(void) {
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_initialize_function_as_variable() {
    assert_error(
        r#"
        int foo(void) = 3;
                    //^ Expected ';', but found '='
        int main(void) {
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_trailing_comma() {
    assert_error(
        r#"
        int foo(int a, int b, int c) {
            return a + b + c;
        }
        int main(void) {
            return foo(1, 2, 3,);
                             //^ Expected expression, but found ')'
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_trailing_comma_decl() {
    assert_error(
        r#"
        
        int foo(int a,) {
                    //^ Expected type specifier
            return a + 1;
        }
        int main(void) {
            return foo(4);
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_unclosed_paren_decl() {
    assert_error(
        r#"
        int foo(int a, int b {
                           //^ Expected ')', but found '{'
            return 0;
        }
        int main(void) {
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_var_init_in_param_list() {
    assert_error(
        r#"
        
        int bad_params(int a = 3) {
                           //^ Expected ')', but found '='
            return 1;
        }
        int main(void) {
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_assign_fun_to_variable() {
    let src = r#"
        int x(void);
        int main(void) {
            int a = 10;
            a = x;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [x]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [10]
                    ├── <21> Assign [=]
                    │   ├── <17> Var [a]
                    │   ╰── <20> Var [x]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_assign_value_to_function() {
    let src = r#"
        int main(void) {
            int x(void);
            x = 3;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── Function [x]
                    ├── <14> Assign [=]
                    │   ├── <11> Var [x]
                    │   ╰── Constant Int [3]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_call_variable_as_function() {
    let src = r#"
        int x(void);
        int main(void) {
            int x = 0;
            return x();
        }
    "#;
    let expected = r#"
        Program
            ├── Function [x]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [0]
                    ╰── Return
                        ╰── <17> FunctionCall [x]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_conflicting_function_declarations() {
    let src = r#"
        int foo(int a);
        int main(void) {
            return 5;
        }
        int foo(int a, int b) {
            return 4;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [foo]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── a
            │           ╰── Type
            │               ╰── Int
            ├── Function [main]
            │   ╰── Body
            │       ╰── Return
            │           ╰── Constant Int [5]
            ╰── Function [foo]
                ├── Parameters
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── a
                │   │   ╰── Type
                │   │       ╰── Int
                │   ╰── Param
                │       ├── Name
                │       │   ╰── b
                │       ╰── Type
                │           ╰── Int
                ╰── Body
                    ╰── Return
                        ╰── Constant Int [4]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_conflicting_local_function_declaration() {
    let src = r#"
        int bar(void);
        int main(void) {
            int foo(int a);
            return bar() + foo(1);
        }
        int bar(void) {
            int foo(int a, int b);
            return foo(1, 2);
        }
    "#;
    let expected = r#"
        Program
            ├── Function [bar]
            ├── Function [main]
            │   ╰── Body
            │       ├── Function [foo]
            │       │   ╰── Parameters
            │       │       ╰── Param
            │       │           ├── Name
            │       │           │   ╰── a
            │       │           ╰── Type
            │       │               ╰── Int
            │       ╰── Return
            │           ╰── <24>  [+]
            │               ├── <19> FunctionCall [bar]
            │               ╰── <23> FunctionCall [foo]
            │                   ╰── Constant Int [1]
            ╰── Function [bar]
                ╰── Body
                    ├── Function [foo]
                    │   ╰── Parameters
                    │       ├── Param
                    │       │   ├── Name
                    │       │   │   ╰── a
                    │       │   ╰── Type
                    │       │       ╰── Int
                    │       ╰── Param
                    │           ├── Name
                    │           │   ╰── b
                    │           ╰── Type
                    │               ╰── Int
                    ╰── Return
                        ╰── <46> FunctionCall [foo]
                            ├── Constant Int [1]
                            ╰── Constant Int [2]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_divide_by_function() {
    let src = r#"
        int x(void);
        int main(void) {
            int a = 10 / x;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [x]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <17>  [/]
                    │           ├── Constant Int [10]
                    │           ╰── <16> Var [x]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_bitwise_op_function() {
    let src = r#"
        int x(void);
        int main(void) {
            x >> 2;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [x]
            ╰── Function [main]
                ╰── Body
                    ├── <14>  [>>]
                    │   ├── <11> Var [x]
                    │   ╰── Constant Int [2]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_compound_assign_function_lhs() {
    let src = r#"
        int x(void);
        int main(void) {
            x += 3;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [x]
            ╰── Function [main]
                ╰── Body
                    ├── <14> Assign [+=]
                    │   ├── <11> Var [x]
                    │   ╰── Constant Int [3]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_compound_assign_function_rhs() {
    let src = r#"
        int x(void);
        int main(void) {
            int a = 3;
            a += x;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [x]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [3]
                    ├── <21> Assign [+=]
                    │   ├── <17> Var [a]
                    │   ╰── <20> Var [x]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_postfix_incr_fun_name() {
    let src = r#"
        int x(void);
        int main(void) {
            x++;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [x]
            ╰── Function [main]
                ╰── Body
                    ├── <13> Postfix [++]
                    │   ╰── <11> Var [x]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_prefix_decr_fun_name() {
    let src = r#"
        int x(void);
        int main(void){
            --x;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [x]
            ╰── Function [main]
                ╰── Body
                    ├── <13> Unary [--]
                    │   ╰── <12> Var [x]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_switch_on_function() {
    let src = r#"
        int main(void) {
            int f(void);
            switch (f)
                return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── Function [f]
                    ╰── Switch
                        ├── Expression
                        │   ╰── <11> Var [f]
                        ╰── Return
                            ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_multiple_function_definitions() {
    let src = r#"
        
        int foo(void){
            return 3;
        }
        int main(void) {
            return foo();
        }
        int foo(void){
            return 4;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [foo]
            │   ╰── Body
            │       ╰── Return
            │           ╰── Constant Int [3]
            ├── Function [main]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <14> FunctionCall [foo]
            ╰── Function [foo]
                ╰── Body
                    ╰── Return
                        ╰── Constant Int [4]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_multiple_function_definitions_2() {
    let src = r#"
        
        int foo(void){
            return 3;
        }
        int main(void) {
            int foo(void);
            return foo();
        }
        int foo(void){
            return 4;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [foo]
            │   ╰── Body
            │       ╰── Return
            │           ╰── Constant Int [3]
            ├── Function [main]
            │   ╰── Body
            │       ├── Function [foo]
            │       ╰── Return
            │           ╰── <19> FunctionCall [foo]
            ╰── Function [foo]
                ╰── Body
                    ╰── Return
                        ╰── Constant Int [4]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_too_few_args() {
    let src = r#"
        int foo(int a, int b) {
            return a + 1;
        }
        int main(void) {
            return foo(1);
        }
    "#;
    let expected = r#"
        Program
            ├── Function [foo]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── a
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── b
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <15>  [+]
            │               ├── <12> Var [a]
            │               ╰── Constant Int [1]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <25> FunctionCall [foo]
                            ╰── Constant Int [1]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_too_many_args() {
    let src = r#"
        int foo(int a) {
            return a + 1;
        }
        int main(void) {
            return foo(1, 2);
        }
    "#;
    let expected = r#"
        Program
            ├── Function [foo]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── a
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <12>  [+]
            │               ├── <9> Var [a]
            │               ╰── Constant Int [1]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <23> FunctionCall [foo]
                            ├── Constant Int [1]
                            ╰── Constant Int [2]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_arguments_in_registers_dont_clobber_edx() {
    let src = r#"
        int x(int a, int b, int c, int d, int e, int f) {
            return a == 1 && b == 2 && c == 3 && d == 4 && e == 5 && f == 6;
        }
        int main(void) {
            int a = 4;
            return x(1, 2, 3, 4, 5, 24 / a);
        }
    "#;
    let expected = r#"
        Program
            ├── Function [x]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── a
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── b
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── c
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── d
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── e
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── f
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <62>  [&&]
            │               ├── <55>  [&&]
            │               │   ├── <48>  [&&]
            │               │   │   ├── <41>  [&&]
            │               │   │   │   ├── <34>  [&&]
            │               │   │   │   │   ├── <27>  [==]
            │               │   │   │   │   │   ├── <24> Var [a]
            │               │   │   │   │   │   ╰── Constant Int [1]
            │               │   │   │   │   ╰── <33>  [==]
            │               │   │   │   │       ├── <30> Var [b]
            │               │   │   │   │       ╰── Constant Int [2]
            │               │   │   │   ╰── <40>  [==]
            │               │   │   │       ├── <37> Var [c]
            │               │   │   │       ╰── Constant Int [3]
            │               │   │   ╰── <47>  [==]
            │               │   │       ├── <44> Var [d]
            │               │   │       ╰── Constant Int [4]
            │               │   ╰── <54>  [==]
            │               │       ├── <51> Var [e]
            │               │       ╰── Constant Int [5]
            │               ╰── <61>  [==]
            │                   ├── <58> Var [f]
            │                   ╰── Constant Int [6]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [4]
                    ╰── Return
                        ╰── <87> FunctionCall [x]
                            ├── Constant Int [1]
                            ├── Constant Int [2]
                            ├── Constant Int [3]
                            ├── Constant Int [4]
                            ├── Constant Int [5]
                            ╰── <86>  [/]
                                ├── Constant Int [24]
                                ╰── <85> Var [a]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_arguments_in_registers_expression_args() {
    let src = r#"
        int sub(int a, int b) {
            return a - b;
        }
        int main(void) {
            int sum = sub(1 + 2, 1);
            return sum;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [sub]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── a
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── b
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <16>  [-]
            │               ├── <12> Var [a]
            │               ╰── <15> Var [b]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── sum
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <33> FunctionCall [sub]
                    │           ├── <31>  [+]
                    │           │   ├── Constant Int [1]
                    │           │   ╰── Constant Int [2]
                    │           ╰── Constant Int [1]
                    ╰── Return
                        ╰── <37> Var [sum]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_arguments_in_registers_fibonacci() {
    let src = r#"
        int fib(int n) {
            if (n == 0 || n == 1) {
                return n;
            } else {
                return fib(n - 1) + fib(n - 2);
            }
        }
        int main(void) {
            int n = 6;
            return fib(n);
        }
    "#;
    let expected = r#"
        Program
            ├── Function [fib]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── n
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ╰── If
            │           ├── Condition
            │           │   ╰── <19>  [||]
            │           │       ├── <12>  [==]
            │           │       │   ├── <9> Var [n]
            │           │       │   ╰── Constant Int [0]
            │           │       ╰── <18>  [==]
            │           │           ├── <15> Var [n]
            │           │           ╰── Constant Int [1]
            │           ├── Then
            │           │   ╰── Block
            │           │       ╰── Return
            │           │           ╰── <21> Var [n]
            │           ╰── Else
            │               ╰── Block
            │                   ╰── Return
            │                       ╰── <40>  [+]
            │                           ├── <31> FunctionCall [fib]
            │                           │   ╰── <30>  [-]
            │                           │       ├── <27> Var [n]
            │                           │       ╰── Constant Int [1]
            │                           ╰── <39> FunctionCall [fib]
            │                               ╰── <38>  [-]
            │                                   ├── <35> Var [n]
            │                                   ╰── Constant Int [2]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── n
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [6]
                    ╰── Return
                        ╰── <60> FunctionCall [fib]
                            ╰── <59> Var [n]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_arguments_in_registers_forward_decl_multi_arg() {
    let src = r#"
        int foo(int a, int b);
        int main(void) {
            return foo(2, 1);
        }
        int foo(int x, int y){
            return x - y;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [foo]
            │   ╰── Parameters
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── a
            │       │   ╰── Type
            │       │       ╰── Int
            │       ╰── Param
            │           ├── Name
            │           │   ╰── b
            │           ╰── Type
            │               ╰── Int
            ├── Function [main]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <19> FunctionCall [foo]
            │               ├── Constant Int [2]
            │               ╰── Constant Int [1]
            ╰── Function [foo]
                ├── Parameters
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── x
                │   │   ╰── Type
                │   │       ╰── Int
                │   ╰── Param
                │       ├── Name
                │       │   ╰── y
                │       ╰── Type
                │           ╰── Int
                ╰── Body
                    ╰── Return
                        ╰── <38>  [-]
                            ├── <34> Var [x]
                            ╰── <37> Var [y]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_arguments_in_registers_hello_world() {
    let src = r#"
        int putchar(int c);
        int main(void) {
            putchar(72);
            putchar(101);
            putchar(108);
            putchar(108);
            putchar(111);
            putchar(44);
            putchar(32);
            putchar(87);
            putchar(111);
            putchar(114);
            putchar(108);
            putchar(100);
            putchar(33);
            putchar(10);
        }
    "#;
    let expected = r#"
        Program
            ├── Function [putchar]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── c
            │           ╰── Type
            │               ╰── Int
            ╰── Function [main]
                ╰── Body
                    ├── <15> FunctionCall [putchar]
                    │   ╰── Constant Int [72]
                    ├── <19> FunctionCall [putchar]
                    │   ╰── Constant Int [101]
                    ├── <23> FunctionCall [putchar]
                    │   ╰── Constant Int [108]
                    ├── <27> FunctionCall [putchar]
                    │   ╰── Constant Int [108]
                    ├── <31> FunctionCall [putchar]
                    │   ╰── Constant Int [111]
                    ├── <35> FunctionCall [putchar]
                    │   ╰── Constant Int [44]
                    ├── <39> FunctionCall [putchar]
                    │   ╰── Constant Int [32]
                    ├── <43> FunctionCall [putchar]
                    │   ╰── Constant Int [87]
                    ├── <47> FunctionCall [putchar]
                    │   ╰── Constant Int [111]
                    ├── <51> FunctionCall [putchar]
                    │   ╰── Constant Int [114]
                    ├── <55> FunctionCall [putchar]
                    │   ╰── Constant Int [108]
                    ├── <59> FunctionCall [putchar]
                    │   ╰── Constant Int [100]
                    ├── <63> FunctionCall [putchar]
                    │   ╰── Constant Int [33]
                    ╰── <67> FunctionCall [putchar]
                        ╰── Constant Int [10]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_arguments_in_registers_param_shadows_local_var() {
    let src = r#"
        int main(void) {
            int a = 10;
            int f(int a);
            return f(a);
        }
        int f(int a) {
            return a * 2;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [main]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── a
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── Constant Int [10]
            │       ├── Function [f]
            │       │   ╰── Parameters
            │       │       ╰── Param
            │       │           ├── Name
            │       │           │   ╰── a
            │       │           ╰── Type
            │       │               ╰── Int
            │       ╰── Return
            │           ╰── <22> FunctionCall [f]
            │               ╰── <21> Var [a]
            ╰── Function [f]
                ├── Parameters
                │   ╰── Param
                │       ├── Name
                │       │   ╰── a
                │       ╰── Type
                │           ╰── Int
                ╰── Body
                    ╰── Return
                        ╰── <37>  [*]
                            ├── <34> Var [a]
                            ╰── Constant Int [2]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_arguments_in_registers_parameter_shadows_function() {
    let src = r#"
        int a(void) {
            return 1;
        }
        int b(int a) {
            return a;
        }
        int main(void) {
            return a() + b(2);
        }
    "#;
    let expected = r#"
        Program
            ├── Function [a]
            │   ╰── Body
            │       ╰── Return
            │           ╰── Constant Int [1]
            ├── Function [b]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── a
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <17> Var [a]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <31>  [+]
                            ├── <26> FunctionCall [a]
                            ╰── <30> FunctionCall [b]
                                ╰── Constant Int [2]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_arguments_in_registers_parameter_shadows_own_function() {
    let src = r#"
        int a(int a) {
            return a * 2;
        }
        int main(void) {
            return a(1);
        }
    "#;
    let expected = r#"
        Program
            ├── Function [a]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── a
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <12>  [*]
            │               ├── <9> Var [a]
            │               ╰── Constant Int [2]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <22> FunctionCall [a]
                            ╰── Constant Int [1]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_arguments_in_registers_parameters_are_preserved() {
    let src = r#"
        int g(int w, int x, int y, int z) {
            if (w == 2 && x == 4 && y == 6 && z == 8)
                return 1;
            return 0;
        }
        int f(int a, int b, int c, int d) {
            int result = g(a * 2, b * 2, c * 2, d * 2);
            return (result == 1 && a == 1 && b == 2 && c == 3 && d == 4);
        }
        int main(void) {
            return f(1, 2, 3, 4);
        }
    "#;
    let expected = r#"
        Program
            ├── Function [g]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── w
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── x
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── y
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── z
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <42>  [&&]
            │       │   │       ├── <35>  [&&]
            │       │   │       │   ├── <28>  [&&]
            │       │   │       │   │   ├── <21>  [==]
            │       │   │       │   │   │   ├── <18> Var [w]
            │       │   │       │   │   │   ╰── Constant Int [2]
            │       │   │       │   │   ╰── <27>  [==]
            │       │   │       │   │       ├── <24> Var [x]
            │       │   │       │   │       ╰── Constant Int [4]
            │       │   │       │   ╰── <34>  [==]
            │       │   │       │       ├── <31> Var [y]
            │       │   │       │       ╰── Constant Int [6]
            │       │   │       ╰── <41>  [==]
            │       │   │           ├── <38> Var [z]
            │       │   │           ╰── Constant Int [8]
            │       │   ╰── Then
            │       │       ╰── Return
            │       │           ╰── Constant Int [1]
            │       ╰── Return
            │           ╰── Constant Int [0]
            ├── Function [f]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── a
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── b
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── c
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── d
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── result
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <90> FunctionCall [g]
            │       │           ├── <74>  [*]
            │       │           │   ├── <71> Var [a]
            │       │           │   ╰── Constant Int [2]
            │       │           ├── <79>  [*]
            │       │           │   ├── <76> Var [b]
            │       │           │   ╰── Constant Int [2]
            │       │           ├── <84>  [*]
            │       │           │   ├── <81> Var [c]
            │       │           │   ╰── Constant Int [2]
            │       │           ╰── <89>  [*]
            │       │               ├── <86> Var [d]
            │       │               ╰── Constant Int [2]
            │       ╰── Return
            │           ╰── <126>  [&&]
            │               ├── <118>  [&&]
            │               │   ├── <111>  [&&]
            │               │   │   ├── <104>  [&&]
            │               │   │   │   ├── <97>  [==]
            │               │   │   │   │   ├── <94> Var [result]
            │               │   │   │   │   ╰── Constant Int [1]
            │               │   │   │   ╰── <103>  [==]
            │               │   │   │       ├── <100> Var [a]
            │               │   │   │       ╰── Constant Int [1]
            │               │   │   ╰── <110>  [==]
            │               │   │       ├── <107> Var [b]
            │               │   │       ╰── Constant Int [2]
            │               │   ╰── <117>  [==]
            │               │       ├── <114> Var [c]
            │               │       ╰── Constant Int [3]
            │               ╰── <124>  [==]
            │                   ├── <121> Var [d]
            │                   ╰── Constant Int [4]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <139> FunctionCall [f]
                            ├── Constant Int [1]
                            ├── Constant Int [2]
                            ├── Constant Int [3]
                            ╰── Constant Int [4]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_arguments_in_registers_single_arg() {
    let src = r#"
        int twice(int x){
            return 2 * x;
        }
        int main(void) {
            return twice(3);
        }
    "#;
    let expected = r#"
        Program
            ├── Function [twice]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── x
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <12>  [*]
            │               ├── Constant Int [2]
            │               ╰── <11> Var [x]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <22> FunctionCall [twice]
                            ╰── Constant Int [3]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_compound_assign_function_result() {
    let src = r#"
        int foo(void) {
            return 2;
        }
        int main(void) {
            int x = 3;
            x -= foo();
            return x;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [foo]
            │   ╰── Body
            │       ╰── Return
            │           ╰── Constant Int [2]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [3]
                    ├── <24> Assign [-=]
                    │   ├── <20> Var [x]
                    │   ╰── <23> FunctionCall [foo]
                    ╰── Return
                        ╰── <27> Var [x]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_dont_clobber_ecx() {
    let src = r#"
        int x(int a, int b, int c, int d, int e, int f) {
            return a == 1 && b == 2 && c == 3 && d == 4 && e == 5 && f == 6;
        }
        int main(void) {
            int a = 4;
            return x(1, 2, 3, 4, 5, 24 >> (a / 2));
        }
    "#;
    let expected = r#"
        Program
            ├── Function [x]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── a
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── b
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── c
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── d
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── e
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── f
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <62>  [&&]
            │               ├── <55>  [&&]
            │               │   ├── <48>  [&&]
            │               │   │   ├── <41>  [&&]
            │               │   │   │   ├── <34>  [&&]
            │               │   │   │   │   ├── <27>  [==]
            │               │   │   │   │   │   ├── <24> Var [a]
            │               │   │   │   │   │   ╰── Constant Int [1]
            │               │   │   │   │   ╰── <33>  [==]
            │               │   │   │   │       ├── <30> Var [b]
            │               │   │   │   │       ╰── Constant Int [2]
            │               │   │   │   ╰── <40>  [==]
            │               │   │   │       ├── <37> Var [c]
            │               │   │   │       ╰── Constant Int [3]
            │               │   │   ╰── <47>  [==]
            │               │   │       ├── <44> Var [d]
            │               │   │       ╰── Constant Int [4]
            │               │   ╰── <54>  [==]
            │               │       ├── <51> Var [e]
            │               │       ╰── Constant Int [5]
            │               ╰── <61>  [==]
            │                   ├── <58> Var [f]
            │                   ╰── Constant Int [6]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [4]
                    ╰── Return
                        ╰── <91> FunctionCall [x]
                            ├── Constant Int [1]
                            ├── Constant Int [2]
                            ├── Constant Int [3]
                            ├── Constant Int [4]
                            ├── Constant Int [5]
                            ╰── <90>  [>>]
                                ├── Constant Int [24]
                                ╰── <89>  [/]
                                    ├── <85> Var [a]
                                    ╰── Constant Int [2]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_goto_label_multiple_functions() {
    let src = r#"
        
        int foo(void) {
            goto label;
            return 0;
            label:
                return 5;
        }
        int main(void) {
            goto label;
            return 0;
            label:
                return foo();
        }
    "#;
    let expected = r#"
        Program
            ├── Function [foo]
            │   ╰── Body
            │       ├── Goto [label]
            │       ├── Return
            │       │   ╰── Constant Int [0]
            │       ╰── Label [label]
            │           ╰── Return
            │               ╰── Constant Int [5]
            ╰── Function [main]
                ╰── Body
                    ├── Goto [label]
                    ├── Return
                    │   ╰── Constant Int [0]
                    ╰── Label [label]
                        ╰── Return
                            ╰── <25> FunctionCall [foo]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_goto_shared_name() {
    let src = r#"
        int foo(void) {
            goto foo;
            return 0;
            foo:
                return 1;
        }
        int main(void) {
            return foo();
        }
    "#;
    let expected = r#"
        Program
            ├── Function [foo]
            │   ╰── Body
            │       ├── Goto [foo]
            │       ├── Return
            │       │   ╰── Constant Int [0]
            │       ╰── Label [foo]
            │           ╰── Return
            │               ╰── Constant Int [1]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <20> FunctionCall [foo]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_label_naming_scheme() {
    let src = r#"
        int main(void) {
            _label:
            label_:
            return 0;
        }
        int main_(void) {
            label:
            return 0;
        }
        int _main(void) {
            label: return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [main]
            │   ╰── Body
            │       ╰── Label [_label]
            │           ╰── Label [label_]
            │               ╰── Return
            │                   ╰── Constant Int [0]
            ├── Function [main_]
            │   ╰── Body
            │       ╰── Label [label]
            │           ╰── Return
            │               ╰── Constant Int [0]
            ╰── Function [_main]
                ╰── Body
                    ╰── Label [label]
                        ╰── Return
                            ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_addition() {
    let src = r#"
        int add(int x, int y) {
            return x + y;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [add]
                ├── Parameters
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── x
                │   │   ╰── Type
                │   │       ╰── Int
                │   ╰── Param
                │       ├── Name
                │       │   ╰── y
                │       ╰── Type
                │           ╰── Int
                ╰── Body
                    ╰── Return
                        ╰── <16>  [+]
                            ├── <12> Var [x]
                            ╰── <15> Var [y]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_addition_client() {
    let src = r#"
        int add(int x, int y);
        int main(void) {
            return add(1, 2);
        }
    "#;
    let expected = r#"
        Program
            ├── Function [add]
            │   ╰── Parameters
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── x
            │       │   ╰── Type
            │       │       ╰── Int
            │       ╰── Param
            │           ├── Name
            │           │   ╰── y
            │           ╰── Type
            │               ╰── Int
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <19> FunctionCall [add]
                            ├── Constant Int [1]
                            ╰── Constant Int [2]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_many_args() {
    let src = r#"
        int fib(int n) {
            if (n == 0 || n == 1) {
                return n;
            } else {
                return fib(n - 1) + fib(n - 2);
            }
        }
        int multiply_many_args(int a, int b, int c, int d, int e, int f, int g, int h) {
            return a * b * c * d * e * f * fib(g) * fib(h);
        }
    "#;
    let expected = r#"
        Program
            ├── Function [fib]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── n
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ╰── If
            │           ├── Condition
            │           │   ╰── <19>  [||]
            │           │       ├── <12>  [==]
            │           │       │   ├── <9> Var [n]
            │           │       │   ╰── Constant Int [0]
            │           │       ╰── <18>  [==]
            │           │           ├── <15> Var [n]
            │           │           ╰── Constant Int [1]
            │           ├── Then
            │           │   ╰── Block
            │           │       ╰── Return
            │           │           ╰── <21> Var [n]
            │           ╰── Else
            │               ╰── Block
            │                   ╰── Return
            │                       ╰── <40>  [+]
            │                           ├── <31> FunctionCall [fib]
            │                           │   ╰── <30>  [-]
            │                           │       ├── <27> Var [n]
            │                           │       ╰── Constant Int [1]
            │                           ╰── <39> FunctionCall [fib]
            │                               ╰── <38>  [-]
            │                                   ├── <35> Var [n]
            │                                   ╰── Constant Int [2]
            ╰── Function [multiply_many_args]
                ├── Parameters
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── a
                │   │   ╰── Type
                │   │       ╰── Int
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── b
                │   │   ╰── Type
                │   │       ╰── Int
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── c
                │   │   ╰── Type
                │   │       ╰── Int
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── d
                │   │   ╰── Type
                │   │       ╰── Int
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── e
                │   │   ╰── Type
                │   │       ╰── Int
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── f
                │   │   ╰── Type
                │   │       ╰── Int
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── g
                │   │   ╰── Type
                │   │       ╰── Int
                │   ╰── Param
                │       ├── Name
                │       │   ╰── h
                │       ╰── Type
                │           ╰── Int
                ╰── Body
                    ╰── Return
                        ╰── <108>  [*]
                            ├── <102>  [*]
                            │   ├── <96>  [*]
                            │   │   ├── <92>  [*]
                            │   │   │   ├── <88>  [*]
                            │   │   │   │   ├── <84>  [*]
                            │   │   │   │   │   ├── <80>  [*]
                            │   │   │   │   │   │   ├── <76> Var [a]
                            │   │   │   │   │   │   ╰── <79> Var [b]
                            │   │   │   │   │   ╰── <83> Var [c]
                            │   │   │   │   ╰── <87> Var [d]
                            │   │   │   ╰── <91> Var [e]
                            │   │   ╰── <95> Var [f]
                            │   ╰── <101> FunctionCall [fib]
                            │       ╰── <100> Var [g]
                            ╰── <107> FunctionCall [fib]
                                ╰── <106> Var [h]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_many_args_client() {
    let src = r#"
        int fib(int a);
        int multiply_many_args(int a, int b, int c, int d, int e, int f, int g, int h);
        int main(void) {
            int x = fib(4);
            int seven = 7;
            int eight = fib(6);
            int y = multiply_many_args(x, 2, 3, 4, 5, 6, seven, eight);
            if (x != 3) {
                return 1;
            }
            if (y != 589680) {
                return 2;
            }
            return x + (y % 256);
        }
    "#;
    let expected = r#"
        Program
            ├── Function [fib]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── a
            │           ╰── Type
            │               ╰── Int
            ├── Function [multiply_many_args]
            │   ╰── Parameters
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── a
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── b
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── c
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── d
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── e
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── f
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── g
            │       │   ╰── Type
            │       │       ╰── Int
            │       ╰── Param
            │           ├── Name
            │           │   ╰── h
            │           ╰── Type
            │               ╰── Int
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <47> FunctionCall [fib]
                    │           ╰── Constant Int [4]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── seven
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [7]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── eight
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <61> FunctionCall [fib]
                    │           ╰── Constant Int [6]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── y
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <79> FunctionCall [multiply_many_args]
                    │           ├── <69> Var [x]
                    │           ├── Constant Int [2]
                    │           ├── Constant Int [3]
                    │           ├── Constant Int [4]
                    │           ├── Constant Int [5]
                    │           ├── Constant Int [6]
                    │           ├── <76> Var [seven]
                    │           ╰── <78> Var [eight]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <86>  [!=]
                    │   │       ├── <83> Var [x]
                    │   │       ╰── Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <96>  [!=]
                    │   │       ├── <93> Var [y]
                    │   │       ╰── Constant Int [589680]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ╰── Return
                        ╰── <111>  [+]
                            ├── <103> Var [x]
                            ╰── <110>  [%]
                                ├── <106> Var [y]
                                ╰── Constant Int [256]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_no_function_calls_division() {
    let src = r#"
        int f(int a, int b, int c, int d) {
            int x = a / b;
            if (a == 10 && b == 2 && c == 100 && d == 4 && x == 5)
                return 1;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [f]
                ├── Parameters
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── a
                │   │   ╰── Type
                │   │       ╰── Int
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── b
                │   │   ╰── Type
                │   │       ╰── Int
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── c
                │   │   ╰── Type
                │   │       ╰── Int
                │   ╰── Param
                │       ├── Name
                │       │   ╰── d
                │       ╰── Type
                │           ╰── Int
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <25>  [/]
                    │           ├── <21> Var [a]
                    │           ╰── <24> Var [b]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <60>  [&&]
                    │   │       ├── <53>  [&&]
                    │   │       │   ├── <46>  [&&]
                    │   │       │   │   ├── <39>  [&&]
                    │   │       │   │   │   ├── <32>  [==]
                    │   │       │   │   │   │   ├── <29> Var [a]
                    │   │       │   │   │   │   ╰── Constant Int [10]
                    │   │       │   │   │   ╰── <38>  [==]
                    │   │       │   │   │       ├── <35> Var [b]
                    │   │       │   │   │       ╰── Constant Int [2]
                    │   │       │   │   ╰── <45>  [==]
                    │   │       │   │       ├── <42> Var [c]
                    │   │       │   │       ╰── Constant Int [100]
                    │   │       │   ╰── <52>  [==]
                    │   │       │       ├── <49> Var [d]
                    │   │       │       ╰── Constant Int [4]
                    │   │       ╰── <59>  [==]
                    │   │           ├── <56> Var [x]
                    │   │           ╰── Constant Int [5]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [1]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_no_function_calls_division_client() {
    let src = r#"
        int f(int a, int b, int c, int d);
        int main(void) {
            return f(10, 2, 100, 4);
        }
    "#;
    let expected = r#"
        Program
            ├── Function [f]
            │   ╰── Parameters
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── a
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── b
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── c
            │       │   ╰── Type
            │       │       ╰── Int
            │       ╰── Param
            │           ├── Name
            │           │   ╰── d
            │           ╰── Type
            │               ╰── Int
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <27> FunctionCall [f]
                            ├── Constant Int [10]
                            ├── Constant Int [2]
                            ├── Constant Int [100]
                            ╰── Constant Int [4]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_no_function_calls_local_stack_variables() {
    let src = r#"
        
        int f(int reg1, int reg2, int reg3, int reg4, int reg5, int reg6,
            int stack1, int stack2, int stack3) {
            int x = 10;
            if (reg1 == 1 && reg2 == 2 && reg3 == 3 && reg4 == 4 && reg5 == 5
                && reg6 == 6 && stack1 == -1 && stack2 == -2 && stack3 == -3
                && x == 10) {
                stack2 = 100;
                return stack2;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [f]
                ├── Parameters
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── reg1
                │   │   ╰── Type
                │   │       ╰── Int
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── reg2
                │   │   ╰── Type
                │   │       ╰── Int
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── reg3
                │   │   ╰── Type
                │   │       ╰── Int
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── reg4
                │   │   ╰── Type
                │   │       ╰── Int
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── reg5
                │   │   ╰── Type
                │   │       ╰── Int
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── reg6
                │   │   ╰── Type
                │   │       ╰── Int
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── stack1
                │   │   ╰── Type
                │   │       ╰── Int
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── stack2
                │   │   ╰── Type
                │   │       ╰── Int
                │   ╰── Param
                │       ├── Name
                │       │   ╰── stack3
                │       ╰── Type
                │           ╰── Int
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <111>  [&&]
                    │   │       ├── <104>  [&&]
                    │   │       │   ├── <95>  [&&]
                    │   │       │   │   ├── <86>  [&&]
                    │   │       │   │   │   ├── <77>  [&&]
                    │   │       │   │   │   │   ├── <70>  [&&]
                    │   │       │   │   │   │   │   ├── <63>  [&&]
                    │   │       │   │   │   │   │   │   ├── <56>  [&&]
                    │   │       │   │   │   │   │   │   │   ├── <49>  [&&]
                    │   │       │   │   │   │   │   │   │   │   ├── <42>  [==]
                    │   │       │   │   │   │   │   │   │   │   │   ├── <39> Var [reg1]
                    │   │       │   │   │   │   │   │   │   │   │   ╰── Constant Int [1]
                    │   │       │   │   │   │   │   │   │   │   ╰── <48>  [==]
                    │   │       │   │   │   │   │   │   │   │       ├── <45> Var [reg2]
                    │   │       │   │   │   │   │   │   │   │       ╰── Constant Int [2]
                    │   │       │   │   │   │   │   │   │   ╰── <55>  [==]
                    │   │       │   │   │   │   │   │   │       ├── <52> Var [reg3]
                    │   │       │   │   │   │   │   │   │       ╰── Constant Int [3]
                    │   │       │   │   │   │   │   │   ╰── <62>  [==]
                    │   │       │   │   │   │   │   │       ├── <59> Var [reg4]
                    │   │       │   │   │   │   │   │       ╰── Constant Int [4]
                    │   │       │   │   │   │   │   ╰── <69>  [==]
                    │   │       │   │   │   │   │       ├── <66> Var [reg5]
                    │   │       │   │   │   │   │       ╰── Constant Int [5]
                    │   │       │   │   │   │   ╰── <76>  [==]
                    │   │       │   │   │   │       ├── <73> Var [reg6]
                    │   │       │   │   │   │       ╰── Constant Int [6]
                    │   │       │   │   │   ╰── <85>  [==]
                    │   │       │   │   │       ├── <80> Var [stack1]
                    │   │       │   │   │       ╰── <84> Unary [-]
                    │   │       │   │   │           ╰── Constant Int [1]
                    │   │       │   │   ╰── <94>  [==]
                    │   │       │   │       ├── <89> Var [stack2]
                    │   │       │   │       ╰── <93> Unary [-]
                    │   │       │   │           ╰── Constant Int [2]
                    │   │       │   ╰── <103>  [==]
                    │   │       │       ├── <98> Var [stack3]
                    │   │       │       ╰── <102> Unary [-]
                    │   │       │           ╰── Constant Int [3]
                    │   │       ╰── <110>  [==]
                    │   │           ├── <107> Var [x]
                    │   │           ╰── Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ├── <116> Assign [=]
                    │           │   ├── <113> Var [stack2]
                    │           │   ╰── Constant Int [100]
                    │           ╰── Return
                    │               ╰── <119> Var [stack2]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_no_function_calls_local_stack_variables_client() {
    let src = r#"
        int f(int reg1, int reg2, int reg3, int reg4, int reg5, int reg6,
            int stack1, int stack2, int stack3);
        int main(void) {
            return f(1, 2, 3, 4, 5, 6, -1, -2, -3);
        }
    "#;
    let expected = r#"
        Program
            ├── Function [f]
            │   ╰── Parameters
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── reg1
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── reg2
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── reg3
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── reg4
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── reg5
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── reg6
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── stack1
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── stack2
            │       │   ╰── Type
            │       │       ╰── Int
            │       ╰── Param
            │           ├── Name
            │           │   ╰── stack3
            │           ╰── Type
            │               ╰── Int
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <53> FunctionCall [f]
                            ├── Constant Int [1]
                            ├── Constant Int [2]
                            ├── Constant Int [3]
                            ├── Constant Int [4]
                            ├── Constant Int [5]
                            ├── Constant Int [6]
                            ├── <46> Unary [-]
                            │   ╰── Constant Int [1]
                            ├── <49> Unary [-]
                            │   ╰── Constant Int [2]
                            ╰── <52> Unary [-]
                                ╰── Constant Int [3]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_system_call() {
    let src = r#"
        int putchar(int c);
        int incr_and_print(int b) {
            return putchar(b + 2);
        }
    "#;
    let expected = r#"
        Program
            ├── Function [putchar]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── c
            │           ╰── Type
            │               ╰── Int
            ╰── Function [incr_and_print]
                ├── Parameters
                │   ╰── Param
                │       ├── Name
                │       │   ╰── b
                │       ╰── Type
                │           ╰── Int
                ╰── Body
                    ╰── Return
                        ╰── <22> FunctionCall [putchar]
                            ╰── <21>  [+]
                                ├── <18> Var [b]
                                ╰── Constant Int [2]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_system_call_client() {
    let src = r#"
        int incr_and_print(int c);
        int main(void) {
            incr_and_print(70);
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [incr_and_print]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── c
            │           ╰── Type
            │               ╰── Int
            ╰── Function [main]
                ╰── Body
                    ├── <15> FunctionCall [incr_and_print]
                    │   ╰── Constant Int [70]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_no_arguments_forward_decl() {
    let src = r#"
        int foo(void);
        int main(void) {
            return foo();
        }
        int foo(void) {
            return 3;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [foo]
            ├── Function [main]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <11> FunctionCall [foo]
            ╰── Function [foo]
                ╰── Body
                    ╰── Return
                        ╰── Constant Int [3]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_no_arguments_function_shadows_variable() {
    let src = r#"
        int main(void) {
            int foo = 3;
            int bar = 4;
            if (foo + bar > 0) {
                int foo(void);
                bar = foo();
            }
            return foo + bar;
        }
        int foo(void) {
            return 8;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [main]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── foo
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── Constant Int [3]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── bar
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── Constant Int [4]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <25>  [>]
            │       │   │       ├── <22>  [+]
            │       │   │       │   ├── <18> Var [foo]
            │       │   │       │   ╰── <21> Var [bar]
            │       │   │       ╰── Constant Int [0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ├── Function [foo]
            │       │           ╰── <36> Assign [=]
            │       │               ├── <32> Var [bar]
            │       │               ╰── <35> FunctionCall [foo]
            │       ╰── Return
            │           ╰── <46>  [+]
            │               ├── <42> Var [foo]
            │               ╰── <45> Var [bar]
            ╰── Function [foo]
                ╰── Body
                    ╰── Return
                        ╰── Constant Int [8]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_no_arguments_multiple_declarations() {
    let src = r#"
        int main(void) {
            int f(void);
            int f(void);
            return f();
        }
        int f(void) {
            return 3;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [main]
            │   ╰── Body
            │       ├── Function [f]
            │       ├── Function [f]
            │       ╰── Return
            │           ╰── <16> FunctionCall [f]
            ╰── Function [f]
                ╰── Body
                    ╰── Return
                        ╰── Constant Int [3]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_no_arguments_no_return_value() {
    let src = r#"
        int foo(void) {
            int x = 1;
        }
        int main(void) {
            foo();
            return 3;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [foo]
            │   ╰── Body
            │       ╰── VarDeclaration
            │           ├── Name
            │           │   ╰── x
            │           ├── Type
            │           │   ╰── Int
            │           ╰── Initializer
            │               ╰── Constant Int [1]
            ╰── Function [main]
                ╰── Body
                    ├── <18> FunctionCall [foo]
                    ╰── Return
                        ╰── Constant Int [3]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_no_arguments_precedence() {
    let src = r#"
        int three(void) {
            return 3;
        }
        int main(void) {
            return !three();
        }
    "#;
    let expected = r#"
        Program
            ├── Function [three]
            │   ╰── Body
            │       ╰── Return
            │           ╰── Constant Int [3]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <16> Unary [!]
                            ╰── <15> FunctionCall [three]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_no_arguments_use_function_in_expression() {
    let src = r#"
        int bar(void) {
            return 9;
        }
        int foo(void) {
            return 2 * bar();
        }
        int main(void) {
            return foo() + bar() / 3;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [bar]
            │   ╰── Body
            │       ╰── Return
            │           ╰── Constant Int [9]
            ├── Function [foo]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <17>  [*]
            │               ├── Constant Int [2]
            │               ╰── <16> FunctionCall [bar]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <33>  [+]
                            ├── <26> FunctionCall [foo]
                            ╰── <32>  [/]
                                ├── <29> FunctionCall [bar]
                                ╰── Constant Int [3]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_no_arguments_variable_shadows_function() {
    let src = r#"
        int main(void) {
            int foo(void);
            int x = foo();
            if (x > 0) {
                int foo = 3;
                x = x + foo;
            }
            return x;
        }
        int foo(void) {
            return 4;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [main]
            │   ╰── Body
            │       ├── Function [foo]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── x
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <14> FunctionCall [foo]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <21>  [>]
            │       │   │       ├── <18> Var [x]
            │       │   │       ╰── Constant Int [0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ├── VarDeclaration
            │       │           │   ├── Name
            │       │           │   │   ╰── foo
            │       │           │   ├── Type
            │       │           │   │   ╰── Int
            │       │           │   ╰── Initializer
            │       │           │       ╰── Constant Int [3]
            │       │           ╰── <37> Assign [=]
            │       │               ├── <29> Var [x]
            │       │               ╰── <36>  [+]
            │       │                   ├── <32> Var [x]
            │       │                   ╰── <35> Var [foo]
            │       ╰── Return
            │           ╰── <43> Var [x]
            ╰── Function [foo]
                ╰── Body
                    ╰── Return
                        ╰── Constant Int [4]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_stack_arguments_call_putchar() {
    let src = r#"
        int putchar(int c);
        int foo(int a, int b, int c, int d, int e, int f, int g, int h) {
            putchar(h);
            return a + g;
        }
        int main(void) {
            return foo(1, 2, 3, 4, 5, 6, 7, 65);
        }
    "#;
    let expected = r#"
        Program
            ├── Function [putchar]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── c
            │           ╰── Type
            │               ╰── Int
            ├── Function [foo]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── a
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── b
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── c
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── d
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── e
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── f
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── g
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── h
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ├── <40> FunctionCall [putchar]
            │       │   ╰── <39> Var [h]
            │       ╰── Return
            │           ╰── <47>  [+]
            │               ├── <43> Var [a]
            │               ╰── <46> Var [g]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <64> FunctionCall [foo]
                            ├── Constant Int [1]
                            ├── Constant Int [2]
                            ├── Constant Int [3]
                            ├── Constant Int [4]
                            ├── Constant Int [5]
                            ├── Constant Int [6]
                            ├── Constant Int [7]
                            ╰── Constant Int [65]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_stack_arguments_lots_of_arguments() {
    let src = r#"
        int foo(int a, int b, int c, int d, int e, int f, int g, int h) {
            return (a == 1 && b == 2 && c == 3 && d == 4 && e == 5
                    && f == 6 && g == 7 && h == 8);
        }
        int main(void) {
            return foo(1, 2, 3, 4, 5, 6, 7, 8);
        }
    "#;
    let expected = r#"
        Program
            ├── Function [foo]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── a
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── b
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── c
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── d
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── e
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── f
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── g
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── h
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <83>  [&&]
            │               ├── <75>  [&&]
            │               │   ├── <68>  [&&]
            │               │   │   ├── <61>  [&&]
            │               │   │   │   ├── <54>  [&&]
            │               │   │   │   │   ├── <47>  [&&]
            │               │   │   │   │   │   ├── <40>  [&&]
            │               │   │   │   │   │   │   ├── <33>  [==]
            │               │   │   │   │   │   │   │   ├── <30> Var [a]
            │               │   │   │   │   │   │   │   ╰── Constant Int [1]
            │               │   │   │   │   │   │   ╰── <39>  [==]
            │               │   │   │   │   │   │       ├── <36> Var [b]
            │               │   │   │   │   │   │       ╰── Constant Int [2]
            │               │   │   │   │   │   ╰── <46>  [==]
            │               │   │   │   │   │       ├── <43> Var [c]
            │               │   │   │   │   │       ╰── Constant Int [3]
            │               │   │   │   │   ╰── <53>  [==]
            │               │   │   │   │       ├── <50> Var [d]
            │               │   │   │   │       ╰── Constant Int [4]
            │               │   │   │   ╰── <60>  [==]
            │               │   │   │       ├── <57> Var [e]
            │               │   │   │       ╰── Constant Int [5]
            │               │   │   ╰── <67>  [==]
            │               │   │       ├── <64> Var [f]
            │               │   │       ╰── Constant Int [6]
            │               │   ╰── <74>  [==]
            │               │       ├── <71> Var [g]
            │               │       ╰── Constant Int [7]
            │               ╰── <81>  [==]
            │                   ├── <78> Var [h]
            │                   ╰── Constant Int [8]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <100> FunctionCall [foo]
                            ├── Constant Int [1]
                            ├── Constant Int [2]
                            ├── Constant Int [3]
                            ├── Constant Int [4]
                            ├── Constant Int [5]
                            ├── Constant Int [6]
                            ├── Constant Int [7]
                            ╰── Constant Int [8]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_stack_arguments_stack_alignment() {
    let src = r#"
        int even_arguments(int a, int b, int c, int d, int e, int f, int g, int h);
        int odd_arguments(int a, int b, int c, int d, int e, int f, int g, int h, int i);
        int main(void) {
            int x = 3;
            even_arguments(1, 2, 3, 4, 5, 6, 7, 8);
            odd_arguments(1, 2, 3, 4, 5, 6, 7, 8, 9);
            return x;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [even_arguments]
            │   ╰── Parameters
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── a
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── b
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── c
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── d
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── e
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── f
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── g
            │       │   ╰── Type
            │       │       ╰── Int
            │       ╰── Param
            │           ├── Name
            │           │   ╰── h
            │           ╰── Type
            │               ╰── Int
            ├── Function [odd_arguments]
            │   ╰── Parameters
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── a
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── b
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── c
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── d
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── e
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── f
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── g
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── h
            │       │   ╰── Type
            │       │       ╰── Int
            │       ╰── Param
            │           ├── Name
            │           │   ╰── i
            │           ╰── Type
            │               ╰── Int
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [3]
                    ├── <81> FunctionCall [even_arguments]
                    │   ├── Constant Int [1]
                    │   ├── Constant Int [2]
                    │   ├── Constant Int [3]
                    │   ├── Constant Int [4]
                    │   ├── Constant Int [5]
                    │   ├── Constant Int [6]
                    │   ├── Constant Int [7]
                    │   ╰── Constant Int [8]
                    ├── <93> FunctionCall [odd_arguments]
                    │   ├── Constant Int [1]
                    │   ├── Constant Int [2]
                    │   ├── Constant Int [3]
                    │   ├── Constant Int [4]
                    │   ├── Constant Int [5]
                    │   ├── Constant Int [6]
                    │   ├── Constant Int [7]
                    │   ├── Constant Int [8]
                    │   ╰── Constant Int [9]
                    ╰── Return
                        ╰── <96> Var [x]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_stack_arguments_test_for_memory_leaks() {
    let src = r#"
        int lots_of_args(int a, int b, int c, int d, int e, int f, int g, int h, int i, int j, int k, int l, int m, int n, int o) {
            return l + o;
        }
        int main(void) {
            int ret = 0;
            for (int i = 0; i < 10000000; i = i + 1) {
                ret = lots_of_args(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, ret, 13, 14, 15);
            }
            return ret == 150000000;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [lots_of_args]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── a
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── b
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── c
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── d
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── e
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── f
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── g
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── h
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── i
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── j
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── k
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── l
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── m
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── n
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── o
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <55>  [+]
            │               ├── <51> Var [l]
            │               ╰── <54> Var [o]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ret
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [0]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <80>  [<]
                    │   │       ├── <77> Var [i]
                    │   │       ╰── Constant Int [10000000]
                    │   ├── Condition
                    │   │   ╰── <89> Assign [=]
                    │   │       ├── <82> Var [i]
                    │   │       ╰── <88>  [+]
                    │   │           ├── <85> Var [i]
                    │   │           ╰── Constant Int [1]
                    │   ╰── Block
                    │       ╰── <111> Assign [=]
                    │           ├── <91> Var [ret]
                    │           ╰── <110> FunctionCall [lots_of_args]
                    │               ├── Constant Int [1]
                    │               ├── Constant Int [2]
                    │               ├── Constant Int [3]
                    │               ├── Constant Int [4]
                    │               ├── Constant Int [5]
                    │               ├── Constant Int [6]
                    │               ├── Constant Int [7]
                    │               ├── Constant Int [8]
                    │               ├── Constant Int [9]
                    │               ├── Constant Int [10]
                    │               ├── Constant Int [11]
                    │               ├── <106> Var [ret]
                    │               ├── Constant Int [13]
                    │               ├── Constant Int [14]
                    │               ╰── Constant Int [15]
                    ╰── Return
                        ╰── <120>  [==]
                            ├── <117> Var [ret]
                            ╰── Constant Int [150000000]
    "#;
    assert_parse(src, expected);
}
