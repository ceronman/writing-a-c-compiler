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
                    ├── <16> Assign [=]
                    │   ├── <13> FunctionCall [x]
                    │   ╰── <15> Constant Int [1]
                    ╰── Return
                        ╰── <18> Constant Int [0]
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
            │           ╰── <21> FunctionCall [foo]
            │               ├── <19> Constant Int [1]
            │               ╰── <20> Constant Int [2]
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
                        ╰── <41>  [+]
                            ├── <37> Var [a]
                            ╰── <40> Var [b]
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
                    │       ╰── <9> Constant Int [1]
                    ├── Label [a]
                    │   ╰── <21> Assign [=]
                    │       ├── <14> Var [x]
                    │       ╰── <20>  [+]
                    │           ├── <17> Var [x]
                    │           ╰── <19> Constant Int [1]
                    ├── <25> FunctionCall [a]
                    ╰── Return
                        ╰── <28> Var [x]
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
                    ├── <16> Assign [+=]
                    │   ├── <13> FunctionCall [x]
                    │   ╰── <15> Constant Int [1]
                    ╰── Return
                        ╰── <18> Constant Int [0]
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
                    ╰── <15> Postfix [--]
                        ╰── <13> FunctionCall [x]
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
                    ╰── <15> Unary [++]
                        ╰── <14> FunctionCall [x]
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
                    │           ╰── <11> Constant Int [1]
                    ╰── Return
                        ╰── <16> FunctionCall [foo]
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
            │           ╰── <13> Var [a]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <25> FunctionCall [foo]
                            ├── <23> Constant Int [1]
                            ╰── <24> Constant Int [2]
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
            │       │       ╰── <15> Constant Int [1]
            │       ╰── Return
            │           ╰── <19> Var [foo]
            ╰── Function [foo]
                ╰── Body
                    ╰── Return
                        ╰── <28> Constant Int [1]
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
            │       │       ╰── <12> Constant Int [5]
            │       ╰── Return
            │           ╰── <16> Var [a]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <27> FunctionCall [foo]
                            ╰── <26> Constant Int [3]
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
            │       │       ╰── <9> Constant Int [1]
            │       ├── Function [foo]
            │       ╰── Return
            │           ╰── <19> Var [foo]
            ╰── Function [foo]
                ╰── Body
                    ╰── Return
                        ╰── <28> Constant Int [1]
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
            │           ╰── <8> FunctionCall [foo]
            │               ╰── <7> Constant Int [3]
            ╰── Function [foo]
                ├── Parameters
                │   ╰── Param
                │       ├── Name
                │       │   ╰── a
                │       ╰── Type
                │           ╰── Int
                ╰── Body
                    ╰── Return
                        ╰── <20> Constant Int [1]
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
            │           ╰── <17> FunctionCall [foo]
            │               ╰── <16> Constant Int [3]
            ╰── Function [foo]
                ├── Parameters
                │   ╰── Param
                │       ├── Name
                │       │   ╰── x
                │       ╰── Type
                │           ╰── Int
                ╰── Body
                    ╰── Return
                        ╰── <30> Var [a]
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
            │               ╰── <7> Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ├── Goto [label]
                    ╰── Return
                        ╰── <19> Constant Int [1]
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
            │           ╰── <6> Constant Int [3]
            ╰── Function [main]
                ╰── Body
                    ├── Goto [foo]
                    ╰── Return
                        ╰── <17> Constant Int [3]
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
                    │       ╰── <15> Constant Int [10]
                    ├── <23> Assign [=]
                    │   ├── <19> Var [a]
                    │   ╰── <22> Var [x]
                    ╰── Return
                        ╰── <25> Constant Int [0]
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
                    ├── <16> Assign [=]
                    │   ├── <13> Var [x]
                    │   ╰── <15> Constant Int [3]
                    ╰── Return
                        ╰── <18> Constant Int [0]
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
                    │       ╰── <15> Constant Int [0]
                    ╰── Return
                        ╰── <19> FunctionCall [x]
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
            │           ╰── <15> Constant Int [5]
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
                        ╰── <30> Constant Int [4]
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
            │           ╰── <27>  [+]
            │               ├── <22> FunctionCall [bar]
            │               ╰── <26> FunctionCall [foo]
            │                   ╰── <25> Constant Int [1]
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
                        ╰── <51> FunctionCall [foo]
                            ├── <49> Constant Int [1]
                            ╰── <50> Constant Int [2]
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
                    │       ╰── <19>  [/]
                    │           ├── <15> Constant Int [10]
                    │           ╰── <18> Var [x]
                    ╰── Return
                        ╰── <22> Constant Int [0]
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
                    ├── <16>  [>>]
                    │   ├── <13> Var [x]
                    │   ╰── <15> Constant Int [2]
                    ╰── Return
                        ╰── <18> Constant Int [0]
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
                    ├── <16> Assign [+=]
                    │   ├── <13> Var [x]
                    │   ╰── <15> Constant Int [3]
                    ╰── Return
                        ╰── <18> Constant Int [0]
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
                    │       ╰── <15> Constant Int [3]
                    ├── <23> Assign [+=]
                    │   ├── <19> Var [a]
                    │   ╰── <22> Var [x]
                    ╰── Return
                        ╰── <25> Constant Int [0]
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
                    ├── <15> Postfix [++]
                    │   ╰── <13> Var [x]
                    ╰── Return
                        ╰── <17> Constant Int [0]
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
                    ├── <15> Unary [--]
                    │   ╰── <14> Var [x]
                    ╰── Return
                        ╰── <17> Constant Int [0]
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
                        │   ╰── <13> Var [f]
                        ╰── Return
                            ╰── <14> Constant Int [0]
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
            │           ╰── <6> Constant Int [3]
            ├── Function [main]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <16> FunctionCall [foo]
            ╰── Function [foo]
                ╰── Body
                    ╰── Return
                        ╰── <25> Constant Int [4]
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
            │           ╰── <6> Constant Int [3]
            ├── Function [main]
            │   ╰── Body
            │       ├── Function [foo]
            │       ╰── Return
            │           ╰── <22> FunctionCall [foo]
            ╰── Function [foo]
                ╰── Body
                    ╰── Return
                        ╰── <31> Constant Int [4]
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
            │           ╰── <16>  [+]
            │               ├── <13> Var [a]
            │               ╰── <15> Constant Int [1]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <27> FunctionCall [foo]
                            ╰── <26> Constant Int [1]
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
            │           ╰── <13>  [+]
            │               ├── <10> Var [a]
            │               ╰── <12> Constant Int [1]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <25> FunctionCall [foo]
                            ├── <23> Constant Int [1]
                            ╰── <24> Constant Int [2]
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
            │           ╰── <63>  [&&]
            │               ├── <56>  [&&]
            │               │   ├── <49>  [&&]
            │               │   │   ├── <42>  [&&]
            │               │   │   │   ├── <35>  [&&]
            │               │   │   │   │   ├── <28>  [==]
            │               │   │   │   │   │   ├── <25> Var [a]
            │               │   │   │   │   │   ╰── <27> Constant Int [1]
            │               │   │   │   │   ╰── <34>  [==]
            │               │   │   │   │       ├── <31> Var [b]
            │               │   │   │   │       ╰── <33> Constant Int [2]
            │               │   │   │   ╰── <41>  [==]
            │               │   │   │       ├── <38> Var [c]
            │               │   │   │       ╰── <40> Constant Int [3]
            │               │   │   ╰── <48>  [==]
            │               │   │       ├── <45> Var [d]
            │               │   │       ╰── <47> Constant Int [4]
            │               │   ╰── <55>  [==]
            │               │       ├── <52> Var [e]
            │               │       ╰── <54> Constant Int [5]
            │               ╰── <62>  [==]
            │                   ├── <59> Var [f]
            │                   ╰── <61> Constant Int [6]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <75> Constant Int [4]
                    ╰── Return
                        ╰── <89> FunctionCall [x]
                            ├── <79> Constant Int [1]
                            ├── <80> Constant Int [2]
                            ├── <81> Constant Int [3]
                            ├── <82> Constant Int [4]
                            ├── <83> Constant Int [5]
                            ╰── <88>  [/]
                                ├── <84> Constant Int [24]
                                ╰── <87> Var [a]
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
            │           ╰── <17>  [-]
            │               ├── <13> Var [a]
            │               ╰── <16> Var [b]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── sum
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <35> FunctionCall [sub]
                    │           ├── <33>  [+]
                    │           │   ├── <30> Constant Int [1]
                    │           │   ╰── <32> Constant Int [2]
                    │           ╰── <34> Constant Int [1]
                    ╰── Return
                        ╰── <39> Var [sum]
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
            │           │   ╰── <20>  [||]
            │           │       ├── <13>  [==]
            │           │       │   ├── <10> Var [n]
            │           │       │   ╰── <12> Constant Int [0]
            │           │       ╰── <19>  [==]
            │           │           ├── <16> Var [n]
            │           │           ╰── <18> Constant Int [1]
            │           ├── Then
            │           │   ╰── Block
            │           │       ╰── Return
            │           │           ╰── <22> Var [n]
            │           ╰── Else
            │               ╰── Block
            │                   ╰── Return
            │                       ╰── <41>  [+]
            │                           ├── <32> FunctionCall [fib]
            │                           │   ╰── <31>  [-]
            │                           │       ├── <28> Var [n]
            │                           │       ╰── <30> Constant Int [1]
            │                           ╰── <40> FunctionCall [fib]
            │                               ╰── <39>  [-]
            │                                   ├── <36> Var [n]
            │                                   ╰── <38> Constant Int [2]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── n
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <56> Constant Int [6]
                    ╰── Return
                        ╰── <62> FunctionCall [fib]
                            ╰── <61> Var [n]
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
            │           ╰── <21> FunctionCall [foo]
            │               ├── <19> Constant Int [2]
            │               ╰── <20> Constant Int [1]
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
                        ╰── <41>  [-]
                            ├── <37> Var [x]
                            ╰── <40> Var [y]
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
                    ├── <17> FunctionCall [putchar]
                    │   ╰── <16> Constant Int [72]
                    ├── <21> FunctionCall [putchar]
                    │   ╰── <20> Constant Int [101]
                    ├── <25> FunctionCall [putchar]
                    │   ╰── <24> Constant Int [108]
                    ├── <29> FunctionCall [putchar]
                    │   ╰── <28> Constant Int [108]
                    ├── <33> FunctionCall [putchar]
                    │   ╰── <32> Constant Int [111]
                    ├── <37> FunctionCall [putchar]
                    │   ╰── <36> Constant Int [44]
                    ├── <41> FunctionCall [putchar]
                    │   ╰── <40> Constant Int [32]
                    ├── <45> FunctionCall [putchar]
                    │   ╰── <44> Constant Int [87]
                    ├── <49> FunctionCall [putchar]
                    │   ╰── <48> Constant Int [111]
                    ├── <53> FunctionCall [putchar]
                    │   ╰── <52> Constant Int [114]
                    ├── <57> FunctionCall [putchar]
                    │   ╰── <56> Constant Int [108]
                    ├── <61> FunctionCall [putchar]
                    │   ╰── <60> Constant Int [100]
                    ├── <65> FunctionCall [putchar]
                    │   ╰── <64> Constant Int [33]
                    ╰── <69> FunctionCall [putchar]
                        ╰── <68> Constant Int [10]
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
            │       │       ╰── <9> Constant Int [10]
            │       ├── Function [f]
            │       │   ╰── Parameters
            │       │       ╰── Param
            │       │           ├── Name
            │       │           │   ╰── a
            │       │           ╰── Type
            │       │               ╰── Int
            │       ╰── Return
            │           ╰── <24> FunctionCall [f]
            │               ╰── <23> Var [a]
            ╰── Function [f]
                ├── Parameters
                │   ╰── Param
                │       ├── Name
                │       │   ╰── a
                │       ╰── Type
                │           ╰── Int
                ╰── Body
                    ╰── Return
                        ╰── <40>  [*]
                            ├── <37> Var [a]
                            ╰── <39> Constant Int [2]
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
            │           ╰── <6> Constant Int [1]
            ├── Function [b]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── a
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <19> Var [a]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <34>  [+]
                            ├── <29> FunctionCall [a]
                            ╰── <33> FunctionCall [b]
                                ╰── <32> Constant Int [2]
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
            │           ╰── <13>  [*]
            │               ├── <10> Var [a]
            │               ╰── <12> Constant Int [2]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <24> FunctionCall [a]
                            ╰── <23> Constant Int [1]
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
            │       │   │   ╰── <43>  [&&]
            │       │   │       ├── <36>  [&&]
            │       │   │       │   ├── <29>  [&&]
            │       │   │       │   │   ├── <22>  [==]
            │       │   │       │   │   │   ├── <19> Var [w]
            │       │   │       │   │   │   ╰── <21> Constant Int [2]
            │       │   │       │   │   ╰── <28>  [==]
            │       │   │       │   │       ├── <25> Var [x]
            │       │   │       │   │       ╰── <27> Constant Int [4]
            │       │   │       │   ╰── <35>  [==]
            │       │   │       │       ├── <32> Var [y]
            │       │   │       │       ╰── <34> Constant Int [6]
            │       │   │       ╰── <42>  [==]
            │       │   │           ├── <39> Var [z]
            │       │   │           ╰── <41> Constant Int [8]
            │       │   ╰── Then
            │       │       ╰── Return
            │       │           ╰── <44> Constant Int [1]
            │       ╰── Return
            │           ╰── <47> Constant Int [0]
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
            │       │       ╰── <92> FunctionCall [g]
            │       │           ├── <76>  [*]
            │       │           │   ├── <73> Var [a]
            │       │           │   ╰── <75> Constant Int [2]
            │       │           ├── <81>  [*]
            │       │           │   ├── <78> Var [b]
            │       │           │   ╰── <80> Constant Int [2]
            │       │           ├── <86>  [*]
            │       │           │   ├── <83> Var [c]
            │       │           │   ╰── <85> Constant Int [2]
            │       │           ╰── <91>  [*]
            │       │               ├── <88> Var [d]
            │       │               ╰── <90> Constant Int [2]
            │       ╰── Return
            │           ╰── <128>  [&&]
            │               ├── <120>  [&&]
            │               │   ├── <113>  [&&]
            │               │   │   ├── <106>  [&&]
            │               │   │   │   ├── <99>  [==]
            │               │   │   │   │   ├── <96> Var [result]
            │               │   │   │   │   ╰── <98> Constant Int [1]
            │               │   │   │   ╰── <105>  [==]
            │               │   │   │       ├── <102> Var [a]
            │               │   │   │       ╰── <104> Constant Int [1]
            │               │   │   ╰── <112>  [==]
            │               │   │       ├── <109> Var [b]
            │               │   │       ╰── <111> Constant Int [2]
            │               │   ╰── <119>  [==]
            │               │       ├── <116> Var [c]
            │               │       ╰── <118> Constant Int [3]
            │               ╰── <126>  [==]
            │                   ├── <123> Var [d]
            │                   ╰── <125> Constant Int [4]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <142> FunctionCall [f]
                            ├── <138> Constant Int [1]
                            ├── <139> Constant Int [2]
                            ├── <140> Constant Int [3]
                            ╰── <141> Constant Int [4]
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
            │           ╰── <13>  [*]
            │               ├── <9> Constant Int [2]
            │               ╰── <12> Var [x]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <24> FunctionCall [twice]
                            ╰── <23> Constant Int [3]
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
            │           ╰── <6> Constant Int [2]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <18> Constant Int [3]
                    ├── <26> Assign [-=]
                    │   ├── <22> Var [x]
                    │   ╰── <25> FunctionCall [foo]
                    ╰── Return
                        ╰── <29> Var [x]
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
            │           ╰── <63>  [&&]
            │               ├── <56>  [&&]
            │               │   ├── <49>  [&&]
            │               │   │   ├── <42>  [&&]
            │               │   │   │   ├── <35>  [&&]
            │               │   │   │   │   ├── <28>  [==]
            │               │   │   │   │   │   ├── <25> Var [a]
            │               │   │   │   │   │   ╰── <27> Constant Int [1]
            │               │   │   │   │   ╰── <34>  [==]
            │               │   │   │   │       ├── <31> Var [b]
            │               │   │   │   │       ╰── <33> Constant Int [2]
            │               │   │   │   ╰── <41>  [==]
            │               │   │   │       ├── <38> Var [c]
            │               │   │   │       ╰── <40> Constant Int [3]
            │               │   │   ╰── <48>  [==]
            │               │   │       ├── <45> Var [d]
            │               │   │       ╰── <47> Constant Int [4]
            │               │   ╰── <55>  [==]
            │               │       ├── <52> Var [e]
            │               │       ╰── <54> Constant Int [5]
            │               ╰── <62>  [==]
            │                   ├── <59> Var [f]
            │                   ╰── <61> Constant Int [6]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <75> Constant Int [4]
                    ╰── Return
                        ╰── <93> FunctionCall [x]
                            ├── <79> Constant Int [1]
                            ├── <80> Constant Int [2]
                            ├── <81> Constant Int [3]
                            ├── <82> Constant Int [4]
                            ├── <83> Constant Int [5]
                            ╰── <92>  [>>]
                                ├── <84> Constant Int [24]
                                ╰── <91>  [/]
                                    ├── <87> Var [a]
                                    ╰── <89> Constant Int [2]
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
            │       │   ╰── <8> Constant Int [0]
            │       ╰── Label [label]
            │           ╰── Return
            │               ╰── <11> Constant Int [5]
            ╰── Function [main]
                ╰── Body
                    ├── Goto [label]
                    ├── Return
                    │   ╰── <23> Constant Int [0]
                    ╰── Label [label]
                        ╰── Return
                            ╰── <27> FunctionCall [foo]
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
            │       │   ╰── <8> Constant Int [0]
            │       ╰── Label [foo]
            │           ╰── Return
            │               ╰── <11> Constant Int [1]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <22> FunctionCall [foo]
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
            │                   ╰── <8> Constant Int [0]
            ├── Function [main_]
            │   ╰── Body
            │       ╰── Label [label]
            │           ╰── Return
            │               ╰── <20> Constant Int [0]
            ╰── Function [_main]
                ╰── Body
                    ╰── Label [label]
                        ╰── Return
                            ╰── <31> Constant Int [0]
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
                        ╰── <17>  [+]
                            ├── <13> Var [x]
                            ╰── <16> Var [y]
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
                        ╰── <21> FunctionCall [add]
                            ├── <19> Constant Int [1]
                            ╰── <20> Constant Int [2]
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
            │           │   ╰── <20>  [||]
            │           │       ├── <13>  [==]
            │           │       │   ├── <10> Var [n]
            │           │       │   ╰── <12> Constant Int [0]
            │           │       ╰── <19>  [==]
            │           │           ├── <16> Var [n]
            │           │           ╰── <18> Constant Int [1]
            │           ├── Then
            │           │   ╰── Block
            │           │       ╰── Return
            │           │           ╰── <22> Var [n]
            │           ╰── Else
            │               ╰── Block
            │                   ╰── Return
            │                       ╰── <41>  [+]
            │                           ├── <32> FunctionCall [fib]
            │                           │   ╰── <31>  [-]
            │                           │       ├── <28> Var [n]
            │                           │       ╰── <30> Constant Int [1]
            │                           ╰── <40> FunctionCall [fib]
            │                               ╰── <39>  [-]
            │                                   ├── <36> Var [n]
            │                                   ╰── <38> Constant Int [2]
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
                        ╰── <110>  [*]
                            ├── <104>  [*]
                            │   ├── <98>  [*]
                            │   │   ├── <94>  [*]
                            │   │   │   ├── <90>  [*]
                            │   │   │   │   ├── <86>  [*]
                            │   │   │   │   │   ├── <82>  [*]
                            │   │   │   │   │   │   ├── <78> Var [a]
                            │   │   │   │   │   │   ╰── <81> Var [b]
                            │   │   │   │   │   ╰── <85> Var [c]
                            │   │   │   │   ╰── <89> Var [d]
                            │   │   │   ╰── <93> Var [e]
                            │   │   ╰── <97> Var [f]
                            │   ╰── <103> FunctionCall [fib]
                            │       ╰── <102> Var [g]
                            ╰── <109> FunctionCall [fib]
                                ╰── <108> Var [h]
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
                    │       ╰── <50> FunctionCall [fib]
                    │           ╰── <49> Constant Int [4]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── seven
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <56> Constant Int [7]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── eight
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <64> FunctionCall [fib]
                    │           ╰── <63> Constant Int [6]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── y
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <82> FunctionCall [multiply_many_args]
                    │           ├── <72> Var [x]
                    │           ├── <73> Constant Int [2]
                    │           ├── <74> Constant Int [3]
                    │           ├── <75> Constant Int [4]
                    │           ├── <76> Constant Int [5]
                    │           ├── <77> Constant Int [6]
                    │           ├── <79> Var [seven]
                    │           ╰── <81> Var [eight]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <89>  [!=]
                    │   │       ├── <86> Var [x]
                    │   │       ╰── <88> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <90> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <99>  [!=]
                    │   │       ├── <96> Var [y]
                    │   │       ╰── <98> Constant Int [589680]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <100> Constant Int [2]
                    ╰── Return
                        ╰── <114>  [+]
                            ├── <106> Var [x]
                            ╰── <113>  [%]
                                ├── <109> Var [y]
                                ╰── <111> Constant Int [256]
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
                    │       ╰── <26>  [/]
                    │           ├── <22> Var [a]
                    │           ╰── <25> Var [b]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <61>  [&&]
                    │   │       ├── <54>  [&&]
                    │   │       │   ├── <47>  [&&]
                    │   │       │   │   ├── <40>  [&&]
                    │   │       │   │   │   ├── <33>  [==]
                    │   │       │   │   │   │   ├── <30> Var [a]
                    │   │       │   │   │   │   ╰── <32> Constant Int [10]
                    │   │       │   │   │   ╰── <39>  [==]
                    │   │       │   │   │       ├── <36> Var [b]
                    │   │       │   │   │       ╰── <38> Constant Int [2]
                    │   │       │   │   ╰── <46>  [==]
                    │   │       │   │       ├── <43> Var [c]
                    │   │       │   │       ╰── <45> Constant Int [100]
                    │   │       │   ╰── <53>  [==]
                    │   │       │       ├── <50> Var [d]
                    │   │       │       ╰── <52> Constant Int [4]
                    │   │       ╰── <60>  [==]
                    │   │           ├── <57> Var [x]
                    │   │           ╰── <59> Constant Int [5]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <62> Constant Int [1]
                    ╰── Return
                        ╰── <65> Constant Int [0]
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
                        ╰── <29> FunctionCall [f]
                            ├── <25> Constant Int [10]
                            ├── <26> Constant Int [2]
                            ├── <27> Constant Int [100]
                            ╰── <28> Constant Int [4]
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
                    │       ╰── <36> Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <112>  [&&]
                    │   │       ├── <105>  [&&]
                    │   │       │   ├── <96>  [&&]
                    │   │       │   │   ├── <87>  [&&]
                    │   │       │   │   │   ├── <78>  [&&]
                    │   │       │   │   │   │   ├── <71>  [&&]
                    │   │       │   │   │   │   │   ├── <64>  [&&]
                    │   │       │   │   │   │   │   │   ├── <57>  [&&]
                    │   │       │   │   │   │   │   │   │   ├── <50>  [&&]
                    │   │       │   │   │   │   │   │   │   │   ├── <43>  [==]
                    │   │       │   │   │   │   │   │   │   │   │   ├── <40> Var [reg1]
                    │   │       │   │   │   │   │   │   │   │   │   ╰── <42> Constant Int [1]
                    │   │       │   │   │   │   │   │   │   │   ╰── <49>  [==]
                    │   │       │   │   │   │   │   │   │   │       ├── <46> Var [reg2]
                    │   │       │   │   │   │   │   │   │   │       ╰── <48> Constant Int [2]
                    │   │       │   │   │   │   │   │   │   ╰── <56>  [==]
                    │   │       │   │   │   │   │   │   │       ├── <53> Var [reg3]
                    │   │       │   │   │   │   │   │   │       ╰── <55> Constant Int [3]
                    │   │       │   │   │   │   │   │   ╰── <63>  [==]
                    │   │       │   │   │   │   │   │       ├── <60> Var [reg4]
                    │   │       │   │   │   │   │   │       ╰── <62> Constant Int [4]
                    │   │       │   │   │   │   │   ╰── <70>  [==]
                    │   │       │   │   │   │   │       ├── <67> Var [reg5]
                    │   │       │   │   │   │   │       ╰── <69> Constant Int [5]
                    │   │       │   │   │   │   ╰── <77>  [==]
                    │   │       │   │   │   │       ├── <74> Var [reg6]
                    │   │       │   │   │   │       ╰── <76> Constant Int [6]
                    │   │       │   │   │   ╰── <86>  [==]
                    │   │       │   │   │       ├── <81> Var [stack1]
                    │   │       │   │   │       ╰── <85> Unary [-]
                    │   │       │   │   │           ╰── <84> Constant Int [1]
                    │   │       │   │   ╰── <95>  [==]
                    │   │       │   │       ├── <90> Var [stack2]
                    │   │       │   │       ╰── <94> Unary [-]
                    │   │       │   │           ╰── <93> Constant Int [2]
                    │   │       │   ╰── <104>  [==]
                    │   │       │       ├── <99> Var [stack3]
                    │   │       │       ╰── <103> Unary [-]
                    │   │       │           ╰── <102> Constant Int [3]
                    │   │       ╰── <111>  [==]
                    │   │           ├── <108> Var [x]
                    │   │           ╰── <110> Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ├── <117> Assign [=]
                    │           │   ├── <114> Var [stack2]
                    │           │   ╰── <116> Constant Int [100]
                    │           ╰── Return
                    │               ╰── <120> Var [stack2]
                    ╰── Return
                        ╰── <125> Constant Int [0]
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
                        ╰── <55> FunctionCall [f]
                            ├── <40> Constant Int [1]
                            ├── <41> Constant Int [2]
                            ├── <42> Constant Int [3]
                            ├── <43> Constant Int [4]
                            ├── <44> Constant Int [5]
                            ├── <45> Constant Int [6]
                            ├── <48> Unary [-]
                            │   ╰── <47> Constant Int [1]
                            ├── <51> Unary [-]
                            │   ╰── <50> Constant Int [2]
                            ╰── <54> Unary [-]
                                ╰── <53> Constant Int [3]
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
                        ╰── <24> FunctionCall [putchar]
                            ╰── <23>  [+]
                                ├── <20> Var [b]
                                ╰── <22> Constant Int [2]
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
                    ├── <17> FunctionCall [incr_and_print]
                    │   ╰── <16> Constant Int [70]
                    ╰── Return
                        ╰── <19> Constant Int [0]
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
            │           ╰── <13> FunctionCall [foo]
            ╰── Function [foo]
                ╰── Body
                    ╰── Return
                        ╰── <22> Constant Int [3]
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
            │       │       ╰── <9> Constant Int [3]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── bar
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <15> Constant Int [4]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <26>  [>]
            │       │   │       ├── <23>  [+]
            │       │   │       │   ├── <19> Var [foo]
            │       │   │       │   ╰── <22> Var [bar]
            │       │   │       ╰── <25> Constant Int [0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ├── Function [foo]
            │       │           ╰── <38> Assign [=]
            │       │               ├── <34> Var [bar]
            │       │               ╰── <37> FunctionCall [foo]
            │       ╰── Return
            │           ╰── <48>  [+]
            │               ├── <44> Var [foo]
            │               ╰── <47> Var [bar]
            ╰── Function [foo]
                ╰── Body
                    ╰── Return
                        ╰── <57> Constant Int [8]
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
            │           ╰── <19> FunctionCall [f]
            ╰── Function [f]
                ╰── Body
                    ╰── Return
                        ╰── <28> Constant Int [3]
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
            │               ╰── <9> Constant Int [1]
            ╰── Function [main]
                ╰── Body
                    ├── <20> FunctionCall [foo]
                    ╰── Return
                        ╰── <22> Constant Int [3]
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
            │           ╰── <6> Constant Int [3]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <18> Unary [!]
                            ╰── <17> FunctionCall [three]
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
            │           ╰── <6> Constant Int [9]
            ├── Function [foo]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <19>  [*]
            │               ├── <15> Constant Int [2]
            │               ╰── <18> FunctionCall [bar]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <36>  [+]
                            ├── <29> FunctionCall [foo]
                            ╰── <35>  [/]
                                ├── <32> FunctionCall [bar]
                                ╰── <34> Constant Int [3]
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
            │       │       ╰── <16> FunctionCall [foo]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <23>  [>]
            │       │   │       ├── <20> Var [x]
            │       │   │       ╰── <22> Constant Int [0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ├── VarDeclaration
            │       │           │   ├── Name
            │       │           │   │   ╰── foo
            │       │           │   ├── Type
            │       │           │   │   ╰── Int
            │       │           │   ╰── Initializer
            │       │           │       ╰── <27> Constant Int [3]
            │       │           ╰── <39> Assign [=]
            │       │               ├── <31> Var [x]
            │       │               ╰── <38>  [+]
            │       │                   ├── <34> Var [x]
            │       │                   ╰── <37> Var [foo]
            │       ╰── Return
            │           ╰── <45> Var [x]
            ╰── Function [foo]
                ╰── Body
                    ╰── Return
                        ╰── <54> Constant Int [4]
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
            │       ├── <42> FunctionCall [putchar]
            │       │   ╰── <41> Var [h]
            │       ╰── Return
            │           ╰── <49>  [+]
            │               ├── <45> Var [a]
            │               ╰── <48> Var [g]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <67> FunctionCall [foo]
                            ├── <59> Constant Int [1]
                            ├── <60> Constant Int [2]
                            ├── <61> Constant Int [3]
                            ├── <62> Constant Int [4]
                            ├── <63> Constant Int [5]
                            ├── <64> Constant Int [6]
                            ├── <65> Constant Int [7]
                            ╰── <66> Constant Int [65]
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
            │           ╰── <84>  [&&]
            │               ├── <76>  [&&]
            │               │   ├── <69>  [&&]
            │               │   │   ├── <62>  [&&]
            │               │   │   │   ├── <55>  [&&]
            │               │   │   │   │   ├── <48>  [&&]
            │               │   │   │   │   │   ├── <41>  [&&]
            │               │   │   │   │   │   │   ├── <34>  [==]
            │               │   │   │   │   │   │   │   ├── <31> Var [a]
            │               │   │   │   │   │   │   │   ╰── <33> Constant Int [1]
            │               │   │   │   │   │   │   ╰── <40>  [==]
            │               │   │   │   │   │   │       ├── <37> Var [b]
            │               │   │   │   │   │   │       ╰── <39> Constant Int [2]
            │               │   │   │   │   │   ╰── <47>  [==]
            │               │   │   │   │   │       ├── <44> Var [c]
            │               │   │   │   │   │       ╰── <46> Constant Int [3]
            │               │   │   │   │   ╰── <54>  [==]
            │               │   │   │   │       ├── <51> Var [d]
            │               │   │   │   │       ╰── <53> Constant Int [4]
            │               │   │   │   ╰── <61>  [==]
            │               │   │   │       ├── <58> Var [e]
            │               │   │   │       ╰── <60> Constant Int [5]
            │               │   │   ╰── <68>  [==]
            │               │   │       ├── <65> Var [f]
            │               │   │       ╰── <67> Constant Int [6]
            │               │   ╰── <75>  [==]
            │               │       ├── <72> Var [g]
            │               │       ╰── <74> Constant Int [7]
            │               ╰── <82>  [==]
            │                   ├── <79> Var [h]
            │                   ╰── <81> Constant Int [8]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <102> FunctionCall [foo]
                            ├── <94> Constant Int [1]
                            ├── <95> Constant Int [2]
                            ├── <96> Constant Int [3]
                            ├── <97> Constant Int [4]
                            ├── <98> Constant Int [5]
                            ├── <99> Constant Int [6]
                            ├── <100> Constant Int [7]
                            ╰── <101> Constant Int [8]
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
                    │       ╰── <72> Constant Int [3]
                    ├── <84> FunctionCall [even_arguments]
                    │   ├── <76> Constant Int [1]
                    │   ├── <77> Constant Int [2]
                    │   ├── <78> Constant Int [3]
                    │   ├── <79> Constant Int [4]
                    │   ├── <80> Constant Int [5]
                    │   ├── <81> Constant Int [6]
                    │   ├── <82> Constant Int [7]
                    │   ╰── <83> Constant Int [8]
                    ├── <96> FunctionCall [odd_arguments]
                    │   ├── <87> Constant Int [1]
                    │   ├── <88> Constant Int [2]
                    │   ├── <89> Constant Int [3]
                    │   ├── <90> Constant Int [4]
                    │   ├── <91> Constant Int [5]
                    │   ├── <92> Constant Int [6]
                    │   ├── <93> Constant Int [7]
                    │   ├── <94> Constant Int [8]
                    │   ╰── <95> Constant Int [9]
                    ╰── Return
                        ╰── <99> Var [x]
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
            │           ╰── <56>  [+]
            │               ├── <52> Var [l]
            │               ╰── <55> Var [o]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ret
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <68> Constant Int [0]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <74> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <82>  [<]
                    │   │       ├── <79> Var [i]
                    │   │       ╰── <81> Constant Int [10000000]
                    │   ├── Condition
                    │   │   ╰── <91> Assign [=]
                    │   │       ├── <84> Var [i]
                    │   │       ╰── <90>  [+]
                    │   │           ├── <87> Var [i]
                    │   │           ╰── <89> Constant Int [1]
                    │   ╰── Block
                    │       ╰── <113> Assign [=]
                    │           ├── <93> Var [ret]
                    │           ╰── <112> FunctionCall [lots_of_args]
                    │               ├── <96> Constant Int [1]
                    │               ├── <97> Constant Int [2]
                    │               ├── <98> Constant Int [3]
                    │               ├── <99> Constant Int [4]
                    │               ├── <100> Constant Int [5]
                    │               ├── <101> Constant Int [6]
                    │               ├── <102> Constant Int [7]
                    │               ├── <103> Constant Int [8]
                    │               ├── <104> Constant Int [9]
                    │               ├── <105> Constant Int [10]
                    │               ├── <106> Constant Int [11]
                    │               ├── <108> Var [ret]
                    │               ├── <109> Constant Int [13]
                    │               ├── <110> Constant Int [14]
                    │               ╰── <111> Constant Int [15]
                    ╰── Return
                        ╰── <122>  [==]
                            ├── <119> Var [ret]
                            ╰── <121> Constant Int [150000000]
    "#;
    assert_parse(src, expected);
}
