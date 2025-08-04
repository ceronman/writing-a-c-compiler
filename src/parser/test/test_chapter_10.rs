use super::{assert_error, assert_parse};

#[test]
fn test_invalid_declarations_conflicting_local_declarations() {
    let src = r#"
        int main(void) {
            int x = 1;
            static int x;
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
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Static
                    ╰── Return
                        ╰── <18> Var [x]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_declarations_extern_follows_local_var() {
    let src = r#"
        int main(void) {
            int x = 3;
            extern int x;
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
                    │       ╰── <9> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Extern
                    ╰── Return
                        ╰── <18> Var [x]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_declarations_extern_follows_static_local_var() {
    let src = r#"
        int main(void) {
            static int x = 0;
            extern int x;
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
                    │   ├── Initializer
                    │   │   ╰── <10> Constant Int [0]
                    │   ╰── Static
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Extern
                    ╰── Return
                        ╰── <19> Var [x]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_declarations_local_var_follows_extern() {
    let src = r#"
        int i = 10;
        int main(void) {
            extern int i;
            int i;
            return i;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── i
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Initializer
            │       ╰── <4> Constant Int [10]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Extern
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ╰── Type
                    │       ╰── Int
                    ╰── Return
                        ╰── <22> Var [i]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_declarations_out_of_scope_extern_var() {
    let src = r#"
        int main(void) {
            {
                extern int a;
            }
            return a;
        }
        int a = 1;
    "#;
    let expected = r#"
        Program
            ├── Function [main]
            │   ╰── Body
            │       ├── Block
            │       │   ╰── VarDeclaration
            │       │       ├── Name
            │       │       │   ╰── a
            │       │       ├── Type
            │       │       │   ╰── Int
            │       │       ╰── Extern
            │       ╰── Return
            │           ╰── <14> Var [a]
            ╰── VarDeclaration
                ├── Name
                │   ╰── a
                ├── Type
                │   ╰── Int
                ╰── Initializer
                    ╰── <21> Constant Int [1]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_declarations_redefine_param_as_identifier_with_linkage() {
    let src = r#"
        int f(int i) {
            extern int i;
            return i;
        }
        int main(void) {
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [f]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── i
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── i
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Extern
            │       ╰── Return
            │           ╰── <15> Var [i]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <24> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_declarations_undeclared_global_variable() {
    let src = r#"
        int main(void) {
            return x;
        }
        int x = 0;
    "#;
    let expected = r#"
        Program
            ├── Function [main]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <7> Var [x]
            ╰── VarDeclaration
                ├── Name
                │   ╰── x
                ├── Type
                │   ╰── Int
                ╰── Initializer
                    ╰── <14> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_labels_extra_credit_goto_global_var() {
    let src = r#"
        int x = 10;
        int main(void) {
            goto x;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── x
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Initializer
            │       ╰── <4> Constant Int [10]
            ╰── Function [main]
                ╰── Body
                    ├── Goto [x]
                    ╰── Return
                        ╰── <14> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_parse_extern_param() {
    assert_error(
        r#"
        
        int f(extern int i) {
            //^^^^^^ Expected type specifier
            return i;
        }
        int main(void) {
            return f(1);
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_extra_credit_extern_label() {
    assert_error(
        r#"
        int main(void) {
            extern a:
          //^^^^^^ Expected type specifier
            return 1;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_extra_credit_file_scope_label() {
    assert_error(
        r#"
        
        x:
      //^ Expected type specifier
        int foo = 0;
        int main(void) {
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_extra_credit_static_label() {
    assert_error(
        r#"
        int main(void) {
            static a:
          //^^^^^^ Expected type specifier
            return 1;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_missing_parameter_list() {
    assert_error(
        r#"
        
        int f {
            //^ Expected ';', but found '{'
            return 0
        };
        int main(void) {
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_missing_type_specifier() {
    assert_error(
        r#"
        static var = 0;
      //^^^^^^ Expected type specifier
        int main(void) {
            return var;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_multi_storage_class_fun() {
    assert_error(
        r#"
        
        static int extern foo(void) {
                 //^^^^^^ Duplicated storage class in declaration
            return 0;
        }
        int main(void) {
            return foo();
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_multi_storage_class_var() {
    assert_error(
        r#"
        int main(void) {
            static extern int foo = 0;
                 //^^^^^^ Duplicated storage class in declaration
            return foo;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_static_and_extern() {
    assert_error(
        r#"
        
        static extern int a;
             //^^^^^^ Duplicated storage class in declaration
        int main(void) {
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_static_param() {
    assert_error(
        r#"
        
        int f(static int i) {
            //^^^^^^ Expected type specifier
            return i;
        }
        int main(void) {
            return f(1);
        }
    "#,
    );
}

#[test]
fn test_invalid_types_conflicting_function_linkage() {
    let src = r#"
        int foo(void);
        int main(void) {
            return foo();
        }
        static int foo(void) {
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [foo]
            ├── Function [main]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <13> FunctionCall [foo]
            ╰── Function [static foo]
                ╰── Body
                    ╰── Return
                        ╰── <23> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_conflicting_function_linkage_2() {
    let src = r#"
        int main(void) {
            int foo(void);
            return foo();
        }
        static int foo(void) {
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [main]
            │   ╰── Body
            │       ├── Function [foo]
            │       ╰── Return
            │           ╰── <13> FunctionCall [foo]
            ╰── Function [static foo]
                ╰── Body
                    ╰── Return
                        ╰── <23> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_conflicting_global_definitions() {
    let src = r#"
        int foo = 3;
        int main(void) {
            return 0;
        }
        int foo = 4;
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── foo
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Initializer
            │       ╰── <4> Constant Int [3]
            ├── Function [main]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <12> Constant Int [0]
            ╰── VarDeclaration
                ├── Name
                │   ╰── foo
                ├── Type
                │   ╰── Int
                ╰── Initializer
                    ╰── <19> Constant Int [4]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_conflicting_variable_linkage() {
    let src = r#"
        
        static int foo;
        int main(void) {
            return foo;
        }
        int foo = 3;
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── foo
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Static
            ├── Function [main]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <12> Var [foo]
            ╰── VarDeclaration
                ├── Name
                │   ╰── foo
                ├── Type
                │   ╰── Int
                ╰── Initializer
                    ╰── <19> Constant Int [3]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_conflicting_variable_linkage_2() {
    let src = r#"
        int main(void) {
            int x = 3;
            {
                extern int x;
            }
            return x;
        }
        static int x = 10;
    "#;
    let expected = r#"
        Program
            ├── Function [main]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── x
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <9> Constant Int [3]
            │       ├── Block
            │       │   ╰── VarDeclaration
            │       │       ├── Name
            │       │       │   ╰── x
            │       │       ├── Type
            │       │       │   ╰── Int
            │       │       ╰── Extern
            │       ╰── Return
            │           ╰── <20> Var [x]
            ╰── VarDeclaration
                ├── Name
                │   ╰── x
                ├── Type
                │   ╰── Int
                ├── Initializer
                │   ╰── <28> Constant Int [10]
                ╰── Static
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extern_for_loop_counter() {
    let src = r#"
        int main(void) {
            int x = 0;
            for (extern int i = 0; i < 10; i = i + 1) {
                x = x + 1;
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
                    │       ╰── <9> Constant Int [0]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ├── Initializer
                    │   │       │   ╰── <16> Constant Int [0]
                    │   │       ╰── Extern
                    │   ├── Condition
                    │   │   ╰── <24>  [<]
                    │   │       ├── <21> Var [i]
                    │   │       ╰── <23> Constant Int [10]
                    │   ├── Condition
                    │   │   ╰── <33> Assign [=]
                    │   │       ├── <26> Var [i]
                    │   │       ╰── <32>  [+]
                    │   │           ├── <29> Var [i]
                    │   │           ╰── <31> Constant Int [1]
                    │   ╰── Block
                    │       ╰── <42> Assign [=]
                    │           ├── <35> Var [x]
                    │           ╰── <41>  [+]
                    │               ├── <38> Var [x]
                    │               ╰── <40> Constant Int [1]
                    ╰── Return
                        ╰── <48> Var [x]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extern_variable_initializer() {
    let src = r#"
        int main(void) {
            extern int i = 0;
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
                    │   ├── Initializer
                    │   │   ╰── <10> Constant Int [0]
                    │   ╰── Extern
                    ╰── Return
                        ╰── <14> Var [i]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_static_var_case() {
    let src = r#"
        int main(void) {
            static int i = 0;
            switch(0) {
                case i: return 0;
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
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ├── Initializer
                    │   │   ╰── <10> Constant Int [0]
                    │   ╰── Static
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <13> Constant Int [0]
                    │   ╰── Block
                    │       ╰── Case [invalid]
                    │           ├── Value
                    │           │   ╰── <15> Var [i]
                    │           ╰── Return
                    │               ╰── <16> Constant Int [0]
                    ╰── Return
                        ╰── <22> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_non_constant_static_initializer() {
    let src = r#"
        int a = 10;
        int b = 1 + a;
        int main(void) {
            return b;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── a
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Initializer
            │       ╰── <4> Constant Int [10]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── b
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Initializer
            │       ╰── <14>  [+]
            │           ├── <10> Constant Int [1]
            │           ╰── <13> Var [a]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <23> Var [b]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_non_constant_static_local_initializer() {
    let src = r#"
        int main(void) {
            int a = 1;
            static int b = a * 2;
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
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <9> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ├── Initializer
                    │   │   ╰── <20>  [*]
                    │   │       ├── <17> Var [a]
                    │   │       ╰── <19> Constant Int [2]
                    │   ╰── Static
                    ╰── Return
                        ╰── <24> Var [b]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_redeclare_file_scope_var_as_fun() {
    let src = r#"
        int foo = 10;
        int main(void) {
            int foo(void);
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── foo
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Initializer
            │       ╰── <4> Constant Int [10]
            ╰── Function [main]
                ╰── Body
                    ├── Function [foo]
                    ╰── Return
                        ╰── <18> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_redeclare_fun_as_file_scope_var() {
    let src = r#"
        int foo(void);
        int foo;
        int main(void) {
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [foo]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── foo
            │   ╰── Type
            │       ╰── Int
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <16> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_redeclare_fun_as_var() {
    let src = r#"
        int foo(void) {
            return 0;
        }
        int main(void) {
            extern int foo;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [foo]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <6> Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── foo
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Extern
                    ╰── Return
                        ╰── <20> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_static_block_scope_function_declaration() {
    let src = r#"
        int main(void) {
            static int foo(void);
            return foo();
        }
        static int foo(void) {
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [main]
            │   ╰── Body
            │       ├── Function [static foo]
            │       ╰── Return
            │           ╰── <14> FunctionCall [foo]
            ╰── Function [static foo]
                ╰── Body
                    ╰── Return
                        ╰── <24> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_static_for_loop_counter() {
    let src = r#"
        int main(void) {
            int x = 0;
            for (static int i = 0; i < 10; i = i + 1) {
                x = x + 1;
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
                    │       ╰── <9> Constant Int [0]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ├── Initializer
                    │   │       │   ╰── <16> Constant Int [0]
                    │   │       ╰── Static
                    │   ├── Condition
                    │   │   ╰── <24>  [<]
                    │   │       ├── <21> Var [i]
                    │   │       ╰── <23> Constant Int [10]
                    │   ├── Condition
                    │   │   ╰── <33> Assign [=]
                    │   │       ├── <26> Var [i]
                    │   │       ╰── <32>  [+]
                    │   │           ├── <29> Var [i]
                    │   │           ╰── <31> Constant Int [1]
                    │   ╰── Block
                    │       ╰── <42> Assign [=]
                    │           ├── <35> Var [x]
                    │           ╰── <41>  [+]
                    │               ├── <38> Var [x]
                    │               ╰── <40> Constant Int [1]
                    ╰── Return
                        ╰── <48> Var [x]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_use_file_scope_variable_as_fun() {
    let src = r#"
        
        extern int foo;
        int main(void) {
            return foo();
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── foo
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Extern
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <12> FunctionCall [foo]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_distinct_local_and_extern() {
    let src = r#"
        int a = 5;
        int return_a(void) {
            return a;
        }
        int main(void) {
            int a = 3;
            {
                extern int a;
                if (a != 5)
                    return 1;
                a = 4;
            }
            return a + return_a();
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── a
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Initializer
            │       ╰── <4> Constant Int [5]
            ├── Function [return_a]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <13> Var [a]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <25> Constant Int [3]
                    ├── Block
                    │   ├── VarDeclaration
                    │   │   ├── Name
                    │   │   │   ╰── a
                    │   │   ├── Type
                    │   │   │   ╰── Int
                    │   │   ╰── Extern
                    │   ├── If
                    │   │   ├── Condition
                    │   │   │   ╰── <37>  [!=]
                    │   │   │       ├── <34> Var [a]
                    │   │   │       ╰── <36> Constant Int [5]
                    │   │   ╰── Then
                    │   │       ╰── Return
                    │   │           ╰── <38> Constant Int [1]
                    │   ╰── <45> Assign [=]
                    │       ├── <42> Var [a]
                    │       ╰── <44> Constant Int [4]
                    ╰── Return
                        ╰── <54>  [+]
                            ├── <50> Var [a]
                            ╰── <53> FunctionCall [return_a]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extern_block_scope_variable() {
    let src = r#"
        int main(void) {
            int outer = 1;
            int foo = 0;
            if (outer) {
                extern int foo;
                extern int foo;
                return foo;
            }
            return 0;
        }
        int foo = 3;
    "#;
    let expected = r#"
        Program
            ├── Function [main]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── outer
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <9> Constant Int [1]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── foo
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <15> Constant Int [0]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <19> Var [outer]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ├── VarDeclaration
            │       │           │   ├── Name
            │       │           │   │   ╰── foo
            │       │           │   ├── Type
            │       │           │   │   ╰── Int
            │       │           │   ╰── Extern
            │       │           ├── VarDeclaration
            │       │           │   ├── Name
            │       │           │   │   ╰── foo
            │       │           │   ├── Type
            │       │           │   │   ╰── Int
            │       │           │   ╰── Extern
            │       │           ╰── Return
            │       │               ╰── <31> Var [foo]
            │       ╰── Return
            │           ╰── <36> Constant Int [0]
            ╰── VarDeclaration
                ├── Name
                │   ╰── foo
                ├── Type
                │   ╰── Int
                ╰── Initializer
                    ╰── <43> Constant Int [3]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_bitwise_ops_file_scope_vars() {
    let src = r#"
        int x = 1;
        int y = 0;
        int main(void) {
            y = -1;
            x = (x << 1) | 1;
            if (x != 3) {
                return 1;
            }
            y = ((y & -5) ^ 12) >> 2;
            if (y != -3) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── x
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Initializer
            │       ╰── <4> Constant Int [1]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── y
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Initializer
            │       ╰── <10> Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ├── <24> Assign [=]
                    │   ├── <19> Var [y]
                    │   ╰── <23> Unary [-]
                    │       ╰── <22> Constant Int [1]
                    ├── <38> Assign [=]
                    │   ├── <27> Var [x]
                    │   ╰── <37>  [|]
                    │       ├── <34>  [<<]
                    │       │   ├── <30> Var [x]
                    │       │   ╰── <32> Constant Int [1]
                    │       ╰── <36> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <44>  [!=]
                    │   │       ├── <41> Var [x]
                    │   │       ╰── <43> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <45> Constant Int [1]
                    ├── <68> Assign [=]
                    │   ├── <51> Var [y]
                    │   ╰── <67>  [>>]
                    │       ├── <64>  [^]
                    │       │   ├── <60>  [&]
                    │       │   │   ├── <54> Var [y]
                    │       │   │   ╰── <58> Unary [-]
                    │       │   │       ╰── <57> Constant Int [5]
                    │       │   ╰── <62> Constant Int [12]
                    │       ╰── <66> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <76>  [!=]
                    │   │       ├── <71> Var [y]
                    │   │       ╰── <75> Unary [-]
                    │   │           ╰── <74> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <77> Constant Int [2]
                    ╰── Return
                        ╰── <82> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_compound_assignment_static_var() {
    let src = r#"
        int f(void) {
            static int i = 0;
            static int j = 0;
            static int k = 1;
            static int l = 48;
            i += 1;
            j -= i;
            k *= j;
            l /= 2;
            if (i != 3) {
                return 1;
            }
            if (j != -6) {
                return 2;
            }
            if (k != -18) {
                return 3;
            }
            if (l != 6) {
                return 4;
            }
            return 0;
        }
        int main(void) {
            f();
            f();
            return f();
        }
    "#;
    let expected = r#"
        Program
            ├── Function [f]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── i
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ├── Initializer
            │       │   │   ╰── <10> Constant Int [0]
            │       │   ╰── Static
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── j
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ├── Initializer
            │       │   │   ╰── <17> Constant Int [0]
            │       │   ╰── Static
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── k
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ├── Initializer
            │       │   │   ╰── <24> Constant Int [1]
            │       │   ╰── Static
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── l
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ├── Initializer
            │       │   │   ╰── <31> Constant Int [48]
            │       │   ╰── Static
            │       ├── <38> Assign [+=]
            │       │   ├── <35> Var [i]
            │       │   ╰── <37> Constant Int [1]
            │       ├── <45> Assign [-=]
            │       │   ├── <41> Var [j]
            │       │   ╰── <44> Var [i]
            │       ├── <52> Assign [*=]
            │       │   ├── <48> Var [k]
            │       │   ╰── <51> Var [j]
            │       ├── <58> Assign [/=]
            │       │   ├── <55> Var [l]
            │       │   ╰── <57> Constant Int [2]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <64>  [!=]
            │       │   │       ├── <61> Var [i]
            │       │   │       ╰── <63> Constant Int [3]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <65> Constant Int [1]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <76>  [!=]
            │       │   │       ├── <71> Var [j]
            │       │   │       ╰── <75> Unary [-]
            │       │   │           ╰── <74> Constant Int [6]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <77> Constant Int [2]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <88>  [!=]
            │       │   │       ├── <83> Var [k]
            │       │   │       ╰── <87> Unary [-]
            │       │   │           ╰── <86> Constant Int [18]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <89> Constant Int [3]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <98>  [!=]
            │       │   │       ├── <95> Var [l]
            │       │   │       ╰── <97> Constant Int [6]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <99> Constant Int [4]
            │       ╰── Return
            │           ╰── <104> Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ├── <114> FunctionCall [f]
                    ├── <117> FunctionCall [f]
                    ╰── Return
                        ╰── <120> FunctionCall [f]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_goto_skip_static_initializer() {
    let src = r#"
        int main(void) {
            goto end;
            static int x = 10;
            end:
                return x;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── Goto [end]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ├── Initializer
                    │   │   ╰── <12> Constant Int [10]
                    │   ╰── Static
                    ╰── Label [end]
                        ╰── Return
                            ╰── <17> Var [x]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_increment_global_vars() {
    let src = r#"
        
        int i = 0;
        int j = 0;
        int incr_i(void){
            if (i == 1) {
                i++;
                ++i;
            }
            return 0;
        }
        int decr_j(void) {
            if (j == -1) {
                j--;
            }
            return 0;
        }
        int main(void) {
            i++ ? 0 : incr_i();
            if (i != 3) {
                return 1;
            }
            --j? decr_j(): 0;
            if (j != -2) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── i
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Initializer
            │       ╰── <4> Constant Int [0]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── j
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Initializer
            │       ╰── <10> Constant Int [0]
            ├── Function [incr_i]
            │   ╰── Body
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <22>  [==]
            │       │   │       ├── <19> Var [i]
            │       │   │       ╰── <21> Constant Int [1]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ├── <26> Postfix [++]
            │       │           │   ╰── <24> Var [i]
            │       │           ╰── <31> Unary [++]
            │       │               ╰── <30> Var [i]
            │       ╰── Return
            │           ╰── <36> Constant Int [0]
            ├── Function [decr_j]
            │   ╰── Body
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <51>  [==]
            │       │   │       ├── <46> Var [j]
            │       │   │       ╰── <50> Unary [-]
            │       │   │           ╰── <49> Constant Int [1]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── <55> Postfix [--]
            │       │               ╰── <53> Var [j]
            │       ╰── Return
            │           ╰── <60> Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ├── <76> Conditional [?]
                    │   ├── <72> Postfix [++]
                    │   │   ╰── <70> Var [i]
                    │   ├── Then
                    │   │   ╰── <73> Constant Int [0]
                    │   ╰── Else
                    │       ╰── <75> FunctionCall [incr_i]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <82>  [!=]
                    │   │       ├── <79> Var [i]
                    │   │       ╰── <81> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <83> Constant Int [1]
                    ├── <95> Conditional [?]
                    │   ├── <91> Unary [--]
                    │   │   ╰── <90> Var [j]
                    │   ├── Then
                    │   │   ╰── <93> FunctionCall [decr_j]
                    │   ╰── Else
                    │       ╰── <94> Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <103>  [!=]
                    │   │       ├── <98> Var [j]
                    │   │       ╰── <102> Unary [-]
                    │   │           ╰── <101> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <104> Constant Int [2]
                    ╰── Return
                        ╰── <109> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_label_file_scope_var_same_name() {
    let src = r#"
        int x;
        int main(void) {
            int x = 10;
            goto x;
            return x;
            {
                extern int x;
            x:
                return x;
            }
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── x
            │   ╰── Type
            │       ╰── Int
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <13> Constant Int [10]
                    ├── Goto [x]
                    ├── Return
                    │   ╰── <19> Var [x]
                    ╰── Block
                        ├── VarDeclaration
                        │   ├── Name
                        │   │   ╰── x
                        │   ├── Type
                        │   │   ╰── Int
                        │   ╰── Extern
                        ╰── Label [x]
                            ╰── Return
                                ╰── <28> Var [x]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_label_static_var_same_name() {
    let src = r#"
        int main(void) {
            static int x = 5;
            goto x;
            x = 0;
        x:
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
                    │   ├── Initializer
                    │   │   ╰── <10> Constant Int [5]
                    │   ╰── Static
                    ├── Goto [x]
                    ├── <19> Assign [=]
                    │   ├── <16> Var [x]
                    │   ╰── <18> Constant Int [0]
                    ╰── Label [x]
                        ╰── Return
                            ╰── <23> Var [x]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_libraries_same_label_same_fun() {
    let src = r#"
        static int f(void) {
            goto x;
            return 0;
            x:
            return 2;
        }
        int f_caller(void) {
            return f();
        }
    "#;
    let expected = r#"
        Program
            ├── Function [static f]
            │   ╰── Body
            │       ├── Goto [x]
            │       ├── Return
            │       │   ╰── <9> Constant Int [0]
            │       ╰── Label [x]
            │           ╰── Return
            │               ╰── <12> Constant Int [2]
            ╰── Function [f_caller]
                ╰── Body
                    ╰── Return
                        ╰── <23> FunctionCall [f]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_libraries_same_label_same_fun_client() {
    let src = r#"
        int f(void) {
            goto x;
            return 0;
        x:
            return 1;
        }
        int f_caller(void);
        int main(void) {
            if (f() != 1) {
                return 1;
            }
            if (f_caller() !=
                2) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [f]
            │   ╰── Body
            │       ├── Goto [x]
            │       ├── Return
            │       │   ╰── <8> Constant Int [0]
            │       ╰── Label [x]
            │           ╰── Return
            │               ╰── <11> Constant Int [1]
            ├── Function [f_caller]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <31>  [!=]
                    │   │       ├── <28> FunctionCall [f]
                    │   │       ╰── <30> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <32> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <41>  [!=]
                    │   │       ├── <38> FunctionCall [f_caller]
                    │   │       ╰── <40> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <42> Constant Int [2]
                    ╰── Return
                        ╰── <47> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_switch_on_extern() {
    let src = r#"
        int update_x(void);
        int main(void) {
            update_x();
            extern int x;
            switch(x) {
                case 0: return 1;
                case 1: return 2;
                case 4: return 0;
                default: return 4;
            }
        }
        int x;
        int update_x(void) {
            x = 4;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [update_x]
            ├── Function [main]
            │   ╰── Body
            │       ├── <13> FunctionCall [update_x]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── x
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Extern
            │       ╰── Switch
            │           ├── Expression
            │           │   ╰── <21> Var [x]
            │           ╰── Block
            │               ├── Case [0]
            │               │   ╰── Return
            │               │       ╰── <23> Constant Int [1]
            │               ├── Case [1]
            │               │   ╰── Return
            │               │       ╰── <27> Constant Int [2]
            │               ├── Case [4]
            │               │   ╰── Return
            │               │       ╰── <31> Constant Int [0]
            │               ╰── Default
            │                   ╰── Return
            │                       ╰── <34> Constant Int [4]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── x
            │   ╰── Type
            │       ╰── Int
            ╰── Function [update_x]
                ╰── Body
                    ├── <55> Assign [=]
                    │   ├── <52> Var [x]
                    │   ╰── <54> Constant Int [4]
                    ╰── Return
                        ╰── <57> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_switch_skip_extern_decl() {
    let src = r#"
        int main(void) {
            int a = 10;
            switch(a) {
                case 1: return 1;
                extern int x;
                case 2: return 2;
                case 10:
                if (x * 2 == 30) {
                    return 0;
                }
                default: return 5;
            }
            return 6;
        }
        int x = 15;
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
            │       ├── Switch
            │       │   ├── Expression
            │       │   │   ╰── <13> Var [a]
            │       │   ╰── Block
            │       │       ├── Case [1]
            │       │       │   ╰── Return
            │       │       │       ╰── <15> Constant Int [1]
            │       │       ├── VarDeclaration
            │       │       │   ├── Name
            │       │       │   │   ╰── x
            │       │       │   ├── Type
            │       │       │   │   ╰── Int
            │       │       │   ╰── Extern
            │       │       ├── Case [2]
            │       │       │   ╰── Return
            │       │       │       ╰── <24> Constant Int [2]
            │       │       ├── Case [10]
            │       │       │   ╰── If
            │       │       │       ├── Condition
            │       │       │       │   ╰── <35>  [==]
            │       │       │       │       ├── <32>  [*]
            │       │       │       │       │   ├── <29> Var [x]
            │       │       │       │       │   ╰── <31> Constant Int [2]
            │       │       │       │       ╰── <34> Constant Int [30]
            │       │       │       ╰── Then
            │       │       │           ╰── Block
            │       │       │               ╰── Return
            │       │       │                   ╰── <36> Constant Int [0]
            │       │       ╰── Default
            │       │           ╰── Return
            │       │               ╰── <42> Constant Int [5]
            │       ╰── Return
            │           ╰── <48> Constant Int [6]
            ╰── VarDeclaration
                ├── Name
                │   ╰── x
                ├── Type
                │   ╰── Int
                ╰── Initializer
                    ╰── <55> Constant Int [15]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_switch_skip_static_initializer() {
    let src = r#"
        int a = 3;
        int main(void) {
            switch (a) {
                case 1:;
                    static int x = 10;
                    x = 0;
                case 3:
                    return x;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── a
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Initializer
            │       ╰── <4> Constant Int [3]
            ╰── Function [main]
                ╰── Body
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <13> Var [a]
                    │   ╰── Block
                    │       ├── Case [1]
                    │       │   ╰── Empty
                    │       ├── VarDeclaration
                    │       │   ├── Name
                    │       │   │   ╰── x
                    │       │   ├── Type
                    │       │   │   ╰── Int
                    │       │   ├── Initializer
                    │       │   │   ╰── <21> Constant Int [10]
                    │       │   ╰── Static
                    │       ├── <28> Assign [=]
                    │       │   ├── <25> Var [x]
                    │       │   ╰── <27> Constant Int [0]
                    │       ╰── Case [3]
                    │           ╰── Return
                    │               ╰── <32> Var [x]
                    ╰── Return
                        ╰── <38> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_external_linkage_function() {
    let src = r#"
        extern int sum(int a, int b);
        int sum(int i, int j) {
            return i + j;
        }
        int sum(int x, int y);
    "#;
    let expected = r#"
        Program
            ├── Function [extern sum]
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
            ├── Function [sum]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── i
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── j
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <30>  [+]
            │               ├── <26> Var [i]
            │               ╰── <29> Var [j]
            ╰── Function [sum]
                ╰── Parameters
                    ├── Param
                    │   ├── Name
                    │   │   ╰── x
                    │   ╰── Type
                    │       ╰── Int
                    ╰── Param
                        ├── Name
                        │   ╰── y
                        ╰── Type
                            ╰── Int
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_external_linkage_function_client() {
    let src = r#"
        int add_one_and_two(void) {
            extern int sum(int a, int b);
            int sum(int a, int b);
            return sum(1, 2);
        }
        extern int sum(int x, int y);
        int sum(int x, int y);
        int add_three_and_four(void) {
            int sum = 3;
            if (sum > 2) {
                extern int sum(int one, int two);
                return sum(3, 4);
            }
            return 1;
        }
        int main(void) {
            if (add_three_and_four() != 7)
                return 1;
            if (add_one_and_two() != 3)
                return 1;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [add_one_and_two]
            │   ╰── Body
            │       ├── Function [extern sum]
            │       │   ╰── Parameters
            │       │       ├── Param
            │       │       │   ├── Name
            │       │       │   │   ╰── a
            │       │       │   ╰── Type
            │       │       │       ╰── Int
            │       │       ╰── Param
            │       │           ├── Name
            │       │           │   ╰── b
            │       │           ╰── Type
            │       │               ╰── Int
            │       ├── Function [sum]
            │       │   ╰── Parameters
            │       │       ├── Param
            │       │       │   ├── Name
            │       │       │   │   ╰── a
            │       │       │   ╰── Type
            │       │       │       ╰── Int
            │       │       ╰── Param
            │       │           ├── Name
            │       │           │   ╰── b
            │       │           ╰── Type
            │       │               ╰── Int
            │       ╰── Return
            │           ╰── <34> FunctionCall [sum]
            │               ├── <32> Constant Int [1]
            │               ╰── <33> Constant Int [2]
            ├── Function [extern sum]
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
            ├── Function [sum]
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
            ├── Function [add_three_and_four]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── sum
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <71> Constant Int [3]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <78>  [>]
            │       │   │       ├── <75> Var [sum]
            │       │   │       ╰── <77> Constant Int [2]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ├── Function [extern sum]
            │       │           │   ╰── Parameters
            │       │           │       ├── Param
            │       │           │       │   ├── Name
            │       │           │       │   │   ╰── one
            │       │           │       │   ╰── Type
            │       │           │       │       ╰── Int
            │       │           │       ╰── Param
            │       │           │           ├── Name
            │       │           │           │   ╰── two
            │       │           │           ╰── Type
            │       │           │               ╰── Int
            │       │           ╰── Return
            │       │               ╰── <95> FunctionCall [sum]
            │       │                   ├── <93> Constant Int [3]
            │       │                   ╰── <94> Constant Int [4]
            │       ╰── Return
            │           ╰── <100> Constant Int [1]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <113>  [!=]
                    │   │       ├── <110> FunctionCall [add_three_and_four]
                    │   │       ╰── <112> Constant Int [7]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <114> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <121>  [!=]
                    │   │       ├── <118> FunctionCall [add_one_and_two]
                    │   │       ╰── <120> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <122> Constant Int [1]
                    ╰── Return
                        ╰── <125> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_external_tentative_var() {
    let src = r#"
        
        int x;
        int read_x(void) {
            return x;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── x
            │   ╰── Type
            │       ╰── Int
            ╰── Function [read_x]
                ╰── Body
                    ╰── Return
                        ╰── <11> Var [x]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_external_tentative_var_client() {
    let src = r#"
        int read_x(void);
        int main(void) {
            extern int x;
            if (x != 0)
                return 1;
            x = 3;
            if (read_x() != 3)
                return 1;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [read_x]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Extern
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <21>  [!=]
                    │   │       ├── <18> Var [x]
                    │   │       ╰── <20> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <22> Constant Int [1]
                    ├── <29> Assign [=]
                    │   ├── <26> Var [x]
                    │   ╰── <28> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <35>  [!=]
                    │   │       ├── <32> FunctionCall [read_x]
                    │   │       ╰── <34> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <36> Constant Int [1]
                    ╰── Return
                        ╰── <39> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_external_var_scoping() {
    let src = r#"
        int read_x(void) {
            int x = 4;
            if (x == 4) {
                extern int x;
                return x;
            } else {
                return -1;
            }
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [read_x]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <9> Constant Int [4]
                    ╰── If
                        ├── Condition
                        │   ╰── <16>  [==]
                        │       ├── <13> Var [x]
                        │       ╰── <15> Constant Int [4]
                        ├── Then
                        │   ╰── Block
                        │       ├── VarDeclaration
                        │       │   ├── Name
                        │       │   │   ╰── x
                        │       │   ├── Type
                        │       │   │   ╰── Int
                        │       │   ╰── Extern
                        │       ╰── Return
                        │           ╰── <23> Var [x]
                        ╰── Else
                            ╰── Block
                                ╰── Return
                                    ╰── <29> Unary [-]
                                        ╰── <28> Constant Int [1]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_external_var_scoping_client() {
    let src = r#"
        int x = 10;
        int read_x(void);
        int main(void) {
            int x = 0;
            if (x == 0) {
                if (read_x() != 10)
                    return 1;
                extern int x;
                if (x != 10)
                    return 1;
                return 0;
            }
            return 1;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── x
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Initializer
            │       ╰── <4> Constant Int [10]
            ├── Function [read_x]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <21> Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <28>  [==]
                    │   │       ├── <25> Var [x]
                    │   │       ╰── <27> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ├── If
                    │           │   ├── Condition
                    │           │   │   ╰── <33>  [!=]
                    │           │   │       ├── <30> FunctionCall [read_x]
                    │           │   │       ╰── <32> Constant Int [10]
                    │           │   ╰── Then
                    │           │       ╰── Return
                    │           │           ╰── <34> Constant Int [1]
                    │           ├── VarDeclaration
                    │           │   ├── Name
                    │           │   │   ╰── x
                    │           │   ├── Type
                    │           │   │   ╰── Int
                    │           │   ╰── Extern
                    │           ├── If
                    │           │   ├── Condition
                    │           │   │   ╰── <46>  [!=]
                    │           │   │       ├── <43> Var [x]
                    │           │   │       ╰── <45> Constant Int [10]
                    │           │   ╰── Then
                    │           │       ╰── Return
                    │           │           ╰── <47> Constant Int [1]
                    │           ╰── Return
                    │               ╰── <50> Constant Int [0]
                    ╰── Return
                        ╰── <55> Constant Int [1]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_external_variable() {
    let src = r#"
        int x;
        extern int x;
        int x;
        int update_x(int new_val) {
            x = new_val;
            return 0;
        }
        int read_x(void) {
            return x;
        }
        int x = 3;
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── x
            │   ╰── Type
            │       ╰── Int
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── x
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Extern
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── x
            │   ╰── Type
            │       ╰── Int
            ├── Function [update_x]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── new_val
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ├── <27> Assign [=]
            │       │   ├── <23> Var [x]
            │       │   ╰── <26> Var [new_val]
            │       ╰── Return
            │           ╰── <29> Constant Int [0]
            ├── Function [read_x]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <39> Var [x]
            ╰── VarDeclaration
                ├── Name
                │   ╰── x
                ├── Type
                │   ╰── Int
                ╰── Initializer
                    ╰── <46> Constant Int [3]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_external_variable_client() {
    let src = r#"
        int update_x(int new_val);
        int read_x(void);
        extern int x;
        int main(void) {
            if (x != 3)
                return 1;
            if (read_x() != 3)
                return 1;
            x = 4;
            if (x != 4)
                return 1;
            if (read_x() != 4)
                return 1;
            update_x(5);
            if (x != 5)
                return 1;
            if (read_x() != 5)
                return 1;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [update_x]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── new_val
            │           ╰── Type
            │               ╰── Int
            ├── Function [read_x]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── x
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Extern
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <30>  [!=]
                    │   │       ├── <27> Var [x]
                    │   │       ╰── <29> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <31> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <38>  [!=]
                    │   │       ├── <35> FunctionCall [read_x]
                    │   │       ╰── <37> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <39> Constant Int [1]
                    ├── <46> Assign [=]
                    │   ├── <43> Var [x]
                    │   ╰── <45> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <52>  [!=]
                    │   │       ├── <49> Var [x]
                    │   │       ╰── <51> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <53> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <60>  [!=]
                    │   │       ├── <57> FunctionCall [read_x]
                    │   │       ╰── <59> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <61> Constant Int [1]
                    ├── <66> FunctionCall [update_x]
                    │   ╰── <65> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <72>  [!=]
                    │   │       ├── <69> Var [x]
                    │   │       ╰── <71> Constant Int [5]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <73> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <80>  [!=]
                    │   │       ├── <77> FunctionCall [read_x]
                    │   │       ╰── <79> Constant Int [5]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <81> Constant Int [1]
                    ╰── Return
                        ╰── <84> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_internal_hides_external_linkage() {
    let src = r#"
        int x = 10;
        int read_x(void){
            return x;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── x
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Initializer
            │       ╰── <4> Constant Int [10]
            ╰── Function [read_x]
                ╰── Body
                    ╰── Return
                        ╰── <13> Var [x]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_internal_hides_external_linkage_client() {
    let src = r#"
        static int x = 1;
        int read_internal_x(void);
        int read_x(void);
        int main(void) {
            extern int x;
            if (x != 1)
                return 1;
            x = 2;
            if (read_internal_x() != 2)
                return 1;
            if (read_x() != 10)
                return 1;
            return 0;
        }
        extern int x;
        int read_internal_x(void) {
            return x;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── x
            │   ├── Type
            │   │   ╰── Int
            │   ├── Initializer
            │   │   ╰── <5> Constant Int [1]
            │   ╰── Static
            ├── Function [read_internal_x]
            ├── Function [read_x]
            ├── Function [main]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── x
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Extern
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <34>  [!=]
            │       │   │       ├── <31> Var [x]
            │       │   │       ╰── <33> Constant Int [1]
            │       │   ╰── Then
            │       │       ╰── Return
            │       │           ╰── <35> Constant Int [1]
            │       ├── <42> Assign [=]
            │       │   ├── <39> Var [x]
            │       │   ╰── <41> Constant Int [2]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <48>  [!=]
            │       │   │       ├── <45> FunctionCall [read_internal_x]
            │       │   │       ╰── <47> Constant Int [2]
            │       │   ╰── Then
            │       │       ╰── Return
            │       │           ╰── <49> Constant Int [1]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <56>  [!=]
            │       │   │       ├── <53> FunctionCall [read_x]
            │       │   │       ╰── <55> Constant Int [10]
            │       │   ╰── Then
            │       │       ╰── Return
            │       │           ╰── <57> Constant Int [1]
            │       ╰── Return
            │           ╰── <60> Constant Int [0]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── x
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Extern
            ╰── Function [read_internal_x]
                ╰── Body
                    ╰── Return
                        ╰── <75> Var [x]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_internal_linkage_function() {
    let src = r#"
        
        static int my_fun(void);
        int call_static_my_fun(void) {
            return my_fun();
        }
        int call_static_my_fun_2(void) {
            int my_fun(void);
            return my_fun();
        }
        extern int my_fun(void);
        static int my_fun(void);
        int my_fun(void) {
            static int i = 0;
            i = i + 1;
            return i;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [static my_fun]
            ├── Function [call_static_my_fun]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <14> FunctionCall [my_fun]
            ├── Function [call_static_my_fun_2]
            │   ╰── Body
            │       ├── Function [my_fun]
            │       ╰── Return
            │           ╰── <30> FunctionCall [my_fun]
            ├── Function [extern my_fun]
            ├── Function [static my_fun]
            ╰── Function [my_fun]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ├── Initializer
                    │   │   ╰── <57> Constant Int [0]
                    │   ╰── Static
                    ├── <68> Assign [=]
                    │   ├── <61> Var [i]
                    │   ╰── <67>  [+]
                    │       ├── <64> Var [i]
                    │       ╰── <66> Constant Int [1]
                    ╰── Return
                        ╰── <71> Var [i]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_internal_linkage_function_client() {
    let src = r#"
        extern int my_fun(void);
        int call_static_my_fun(void);
        int call_static_my_fun_2(void);
        int main(void) {
            if (call_static_my_fun() != 1)
                return 1;
            if (my_fun() != 100)
                return 1;
            if (call_static_my_fun_2() != 2)
                return 1;
            return 0;
        }
        int my_fun(void) {
            return 100;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [extern my_fun]
            ├── Function [call_static_my_fun]
            ├── Function [call_static_my_fun_2]
            ├── Function [main]
            │   ╰── Body
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <29>  [!=]
            │       │   │       ├── <26> FunctionCall [call_static_my_fun]
            │       │   │       ╰── <28> Constant Int [1]
            │       │   ╰── Then
            │       │       ╰── Return
            │       │           ╰── <30> Constant Int [1]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <37>  [!=]
            │       │   │       ├── <34> FunctionCall [my_fun]
            │       │   │       ╰── <36> Constant Int [100]
            │       │   ╰── Then
            │       │       ╰── Return
            │       │           ╰── <38> Constant Int [1]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <45>  [!=]
            │       │   │       ├── <42> FunctionCall [call_static_my_fun_2]
            │       │   │       ╰── <44> Constant Int [2]
            │       │   ╰── Then
            │       │       ╰── Return
            │       │           ╰── <46> Constant Int [1]
            │       ╰── Return
            │           ╰── <49> Constant Int [0]
            ╰── Function [my_fun]
                ╰── Body
                    ╰── Return
                        ╰── <58> Constant Int [100]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_internal_linkage_var() {
    let src = r#"
        static int x;
        int read_x(void) {
            return x;
        }
        int update_x(int new_val) {
            extern int x;
            x = new_val;
            return 0;
        }
        extern int x;
        static int x = 5;
        static int x;
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── x
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Static
            ├── Function [read_x]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <12> Var [x]
            ├── Function [update_x]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── new_val
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── x
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Extern
            │       ├── <34> Assign [=]
            │       │   ├── <30> Var [x]
            │       │   ╰── <33> Var [new_val]
            │       ╰── Return
            │           ╰── <36> Constant Int [0]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── x
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Extern
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── x
            │   ├── Type
            │   │   ╰── Int
            │   ├── Initializer
            │   │   ╰── <49> Constant Int [5]
            │   ╰── Static
            ╰── VarDeclaration
                ├── Name
                │   ╰── x
                ├── Type
                │   ╰── Int
                ╰── Static
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_internal_linkage_var_client() {
    let src = r#"
        static int x;
        static int x;
        int read_x(void);
        int update_x(int x);
        int main(void) {
            if (x != 0)
                return 1;
            if (read_x() != 5)
                return 1;
            extern int x;
            update_x(10);
            if (read_x() != 10)
                return 1;
            if (x != 0)
                return 1;
            x = 20;
            if (x != 20)
                return 1;
            if (read_x() != 10)
                return 1;
            return 0;
        }
        static int x;
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── x
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Static
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── x
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Static
            ├── Function [read_x]
            ├── Function [update_x]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── x
            │           ╰── Type
            │               ╰── Int
            ├── Function [main]
            │   ╰── Body
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <35>  [!=]
            │       │   │       ├── <32> Var [x]
            │       │   │       ╰── <34> Constant Int [0]
            │       │   ╰── Then
            │       │       ╰── Return
            │       │           ╰── <36> Constant Int [1]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <43>  [!=]
            │       │   │       ├── <40> FunctionCall [read_x]
            │       │   │       ╰── <42> Constant Int [5]
            │       │   ╰── Then
            │       │       ╰── Return
            │       │           ╰── <44> Constant Int [1]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── x
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Extern
            │       ├── <54> FunctionCall [update_x]
            │       │   ╰── <53> Constant Int [10]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <60>  [!=]
            │       │   │       ├── <57> FunctionCall [read_x]
            │       │   │       ╰── <59> Constant Int [10]
            │       │   ╰── Then
            │       │       ╰── Return
            │       │           ╰── <61> Constant Int [1]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <68>  [!=]
            │       │   │       ├── <65> Var [x]
            │       │   │       ╰── <67> Constant Int [0]
            │       │   ╰── Then
            │       │       ╰── Return
            │       │           ╰── <69> Constant Int [1]
            │       ├── <76> Assign [=]
            │       │   ├── <73> Var [x]
            │       │   ╰── <75> Constant Int [20]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <82>  [!=]
            │       │   │       ├── <79> Var [x]
            │       │   │       ╰── <81> Constant Int [20]
            │       │   ╰── Then
            │       │       ╰── Return
            │       │           ╰── <83> Constant Int [1]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <90>  [!=]
            │       │   │       ├── <87> FunctionCall [read_x]
            │       │   │       ╰── <89> Constant Int [10]
            │       │   ╰── Then
            │       │       ╰── Return
            │       │           ╰── <91> Constant Int [1]
            │       ╰── Return
            │           ╰── <94> Constant Int [0]
            ╰── VarDeclaration
                ├── Name
                │   ╰── x
                ├── Type
                │   ╰── Int
                ╰── Static
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_multiple_static_file_scope_vars() {
    let src = r#"
        static int foo;
        int main(void) {
            return foo;
        }
        extern int foo;
        static int foo = 4;
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── foo
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Static
            ├── Function [main]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <12> Var [foo]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── foo
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Extern
            ╰── VarDeclaration
                ├── Name
                │   ╰── foo
                ├── Type
                │   ╰── Int
                ├── Initializer
                │   ╰── <25> Constant Int [4]
                ╰── Static
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_multiple_static_local() {
    let src = r#"
        int foo(void) {
            static int a = 3;
            a = a * 2;
            return a;
        }
        int bar(void) {
            static int a = 4;
            a = a + 1;
            return a;
        }
        int main(void) {
            return foo() + bar() + foo() + bar();
        }
    "#;
    let expected = r#"
        Program
            ├── Function [foo]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── a
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ├── Initializer
            │       │   │   ╰── <10> Constant Int [3]
            │       │   ╰── Static
            │       ├── <21> Assign [=]
            │       │   ├── <14> Var [a]
            │       │   ╰── <20>  [*]
            │       │       ├── <17> Var [a]
            │       │       ╰── <19> Constant Int [2]
            │       ╰── Return
            │           ╰── <24> Var [a]
            ├── Function [bar]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── a
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ├── Initializer
            │       │   │   ╰── <37> Constant Int [4]
            │       │   ╰── Static
            │       ├── <48> Assign [=]
            │       │   ├── <41> Var [a]
            │       │   ╰── <47>  [+]
            │       │       ├── <44> Var [a]
            │       │       ╰── <46> Constant Int [1]
            │       ╰── Return
            │           ╰── <51> Var [a]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <73>  [+]
                            ├── <69>  [+]
                            │   ├── <65>  [+]
                            │   │   ├── <61> FunctionCall [foo]
                            │   │   ╰── <64> FunctionCall [bar]
                            │   ╰── <68> FunctionCall [foo]
                            ╰── <72> FunctionCall [bar]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_push_arg_on_page_boundary() {
    let src = r#"
        extern int zed;
        int foo(int a, int b, int c, int d, int e, int f, int g) {
            return g + 1;
        }
        int main(void) {
            return foo(0, 0, 0, 0, 0, 0, zed);
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── zed
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Extern
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
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── g
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <36>  [+]
            │               ├── <33> Var [g]
            │               ╰── <35> Constant Int [1]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <54> FunctionCall [foo]
                            ├── <46> Constant Int [0]
                            ├── <47> Constant Int [0]
                            ├── <48> Constant Int [0]
                            ├── <49> Constant Int [0]
                            ├── <50> Constant Int [0]
                            ├── <51> Constant Int [0]
                            ╰── <53> Var [zed]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_shadow_static_local_var() {
    let src = r#"
        int i;
        int update_static_or_global(int update_global, int new_val)
        {
            static int i;
            if (update_global)
            {
                extern int i;
                i = new_val;
            }
            else
                i = new_val;
            return i;
        }
        int main(void)
        {
            if (i != 0)
                return 1;
            int result = update_static_or_global(1, 10);
            if (result != 0)
                return 1;
            if (i != 10)
                return 1;
            result = update_static_or_global(0, 9);
            if (result != 9)
                return 1;
            if (i != 10)
                return 1;
            result = update_static_or_global(1, 11);
            if (result != 9)
                return 1;
            if (i != 11)
                return 1;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── i
            │   ╰── Type
            │       ╰── Int
            ├── Function [update_static_or_global]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── update_global
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── new_val
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── i
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Static
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <22> Var [update_global]
            │       │   ├── Then
            │       │   │   ╰── Block
            │       │   │       ├── VarDeclaration
            │       │   │       │   ├── Name
            │       │   │       │   │   ╰── i
            │       │   │       │   ├── Type
            │       │   │       │   │   ╰── Int
            │       │   │       │   ╰── Extern
            │       │   │       ╰── <33> Assign [=]
            │       │   │           ├── <29> Var [i]
            │       │   │           ╰── <32> Var [new_val]
            │       │   ╰── Else
            │       │       ╰── <42> Assign [=]
            │       │           ├── <38> Var [i]
            │       │           ╰── <41> Var [new_val]
            │       ╰── Return
            │           ╰── <46> Var [i]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <59>  [!=]
                    │   │       ├── <56> Var [i]
                    │   │       ╰── <58> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <60> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── result
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <69> FunctionCall [update_static_or_global]
                    │           ├── <67> Constant Int [1]
                    │           ╰── <68> Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <76>  [!=]
                    │   │       ├── <73> Var [result]
                    │   │       ╰── <75> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <77> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <84>  [!=]
                    │   │       ├── <81> Var [i]
                    │   │       ╰── <83> Constant Int [10]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <85> Constant Int [1]
                    ├── <95> Assign [=]
                    │   ├── <89> Var [result]
                    │   ╰── <94> FunctionCall [update_static_or_global]
                    │       ├── <92> Constant Int [0]
                    │       ╰── <93> Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <101>  [!=]
                    │   │       ├── <98> Var [result]
                    │   │       ╰── <100> Constant Int [9]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <102> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <109>  [!=]
                    │   │       ├── <106> Var [i]
                    │   │       ╰── <108> Constant Int [10]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <110> Constant Int [1]
                    ├── <120> Assign [=]
                    │   ├── <114> Var [result]
                    │   ╰── <119> FunctionCall [update_static_or_global]
                    │       ├── <117> Constant Int [1]
                    │       ╰── <118> Constant Int [11]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <126>  [!=]
                    │   │       ├── <123> Var [result]
                    │   │       ╰── <125> Constant Int [9]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <127> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <134>  [!=]
                    │   │       ├── <131> Var [i]
                    │   │       ╰── <133> Constant Int [11]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <135> Constant Int [1]
                    ╰── Return
                        ╰── <138> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_static_local_multiple_scopes() {
    let src = r#"
        int putchar (int ch);
        int print_letters(void) {
            static int i = 65;
            putchar(i);
            {
                i = i + 1;
                static int i = 97;
                putchar(i);
                i = i + 1;
            }
            putchar(10);
            return 0;
        }
        int main(void) {
            for (int i = 0; i < 26; i = i + 1)
                print_letters();
        }
    "#;
    let expected = r#"
        Program
            ├── Function [putchar]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── ch
            │           ╰── Type
            │               ╰── Int
            ├── Function [print_letters]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── i
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ├── Initializer
            │       │   │   ╰── <19> Constant Int [65]
            │       │   ╰── Static
            │       ├── <25> FunctionCall [putchar]
            │       │   ╰── <24> Var [i]
            │       ├── Block
            │       │   ├── <35> Assign [=]
            │       │   │   ├── <28> Var [i]
            │       │   │   ╰── <34>  [+]
            │       │   │       ├── <31> Var [i]
            │       │   │       ╰── <33> Constant Int [1]
            │       │   ├── VarDeclaration
            │       │   │   ├── Name
            │       │   │   │   ╰── i
            │       │   │   ├── Type
            │       │   │   │   ╰── Int
            │       │   │   ├── Initializer
            │       │   │   │   ╰── <41> Constant Int [97]
            │       │   │   ╰── Static
            │       │   ├── <47> FunctionCall [putchar]
            │       │   │   ╰── <46> Var [i]
            │       │   ╰── <57> Assign [=]
            │       │       ├── <50> Var [i]
            │       │       ╰── <56>  [+]
            │       │           ├── <53> Var [i]
            │       │           ╰── <55> Constant Int [1]
            │       ├── <63> FunctionCall [putchar]
            │       │   ╰── <62> Constant Int [10]
            │       ╰── Return
            │           ╰── <65> Constant Int [0]
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
                        │           ╰── <77> Constant Int [0]
                        ├── Condition
                        │   ╰── <85>  [<]
                        │       ├── <82> Var [i]
                        │       ╰── <84> Constant Int [26]
                        ├── Condition
                        │   ╰── <94> Assign [=]
                        │       ├── <87> Var [i]
                        │       ╰── <93>  [+]
                        │           ├── <90> Var [i]
                        │           ╰── <92> Constant Int [1]
                        ╰── <96> FunctionCall [print_letters]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_static_local_uninitialized() {
    let src = r#"
        int foo(void) {
            static int x;
            x = x + 1;
            return x;
        }
        int main(void) {
            int ret;
            for (int i = 0; i < 4; i = i + 1)
                ret = foo();
            return ret;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [foo]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── x
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Static
            │       ├── <19> Assign [=]
            │       │   ├── <12> Var [x]
            │       │   ╰── <18>  [+]
            │       │       ├── <15> Var [x]
            │       │       ╰── <17> Constant Int [1]
            │       ╰── Return
            │           ╰── <22> Var [x]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ret
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
                    │   │           ╰── <38> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <46>  [<]
                    │   │       ├── <43> Var [i]
                    │   │       ╰── <45> Constant Int [4]
                    │   ├── Condition
                    │   │   ╰── <55> Assign [=]
                    │   │       ├── <48> Var [i]
                    │   │       ╰── <54>  [+]
                    │   │           ├── <51> Var [i]
                    │   │           ╰── <53> Constant Int [1]
                    │   ╰── <61> Assign [=]
                    │       ├── <57> Var [ret]
                    │       ╰── <60> FunctionCall [foo]
                    ╰── Return
                        ╰── <65> Var [ret]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_static_recursive_call() {
    let src = r#"
        int putchar (int ch);
        int print_alphabet(void) {
            static int count = 0;
            putchar(count + 65);
            count = count + 1;
            if (count < 26) {
                print_alphabet();
            }
            return count;
        }
        int main(void) {
            print_alphabet();
        }
    "#;
    let expected = r#"
        Program
            ├── Function [putchar]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── ch
            │           ╰── Type
            │               ╰── Int
            ├── Function [print_alphabet]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── count
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ├── Initializer
            │       │   │   ╰── <19> Constant Int [0]
            │       │   ╰── Static
            │       ├── <28> FunctionCall [putchar]
            │       │   ╰── <27>  [+]
            │       │       ├── <24> Var [count]
            │       │       ╰── <26> Constant Int [65]
            │       ├── <38> Assign [=]
            │       │   ├── <31> Var [count]
            │       │   ╰── <37>  [+]
            │       │       ├── <34> Var [count]
            │       │       ╰── <36> Constant Int [1]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <44>  [<]
            │       │   │       ├── <41> Var [count]
            │       │   │       ╰── <43> Constant Int [26]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── <46> FunctionCall [print_alphabet]
            │       ╰── Return
            │           ╰── <52> Var [count]
            ╰── Function [main]
                ╰── Body
                    ╰── <62> FunctionCall [print_alphabet]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_static_then_extern() {
    let src = r#"
        static int foo = 3;
        int main(void) {
            return foo;
        }
        extern int foo;
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── foo
            │   ├── Type
            │   │   ╰── Int
            │   ├── Initializer
            │   │   ╰── <5> Constant Int [3]
            │   ╰── Static
            ├── Function [main]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <14> Var [foo]
            ╰── VarDeclaration
                ├── Name
                │   ╰── foo
                ├── Type
                │   ╰── Int
                ╰── Extern
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_static_variables_in_expressions() {
    let src = r#"
        int main(void) {
            static int i = 2;
            static int j = 3;
            int cmp = i < j;
            if (!cmp)
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
                    │   ├── Initializer
                    │   │   ╰── <10> Constant Int [2]
                    │   ╰── Static
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── j
                    │   ├── Type
                    │   │   ╰── Int
                    │   ├── Initializer
                    │   │   ╰── <17> Constant Int [3]
                    │   ╰── Static
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── cmp
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <28>  [<]
                    │           ├── <24> Var [i]
                    │           ╰── <27> Var [j]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <34> Unary [!]
                    │   │       ╰── <33> Var [cmp]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <35> Constant Int [1]
                    ╰── Return
                        ╰── <38> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_tentative_definition() {
    let src = r#"
        extern int foo;
        int foo;
        int foo;
        int main(void) {
            for (int i = 0; i < 5; i = i + 1)
                foo = foo + 1;
            return foo;
        }
        int foo;
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── foo
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Extern
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── foo
            │   ╰── Type
            │       ╰── Int
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── foo
            │   ╰── Type
            │       ╰── Int
            ├── Function [main]
            │   ╰── Body
            │       ├── For
            │       │   ├── Init
            │       │   │   ╰── VarDeclaration
            │       │   │       ├── Name
            │       │   │       │   ╰── i
            │       │   │       ├── Type
            │       │   │       │   ╰── Int
            │       │   │       ╰── Initializer
            │       │   │           ╰── <22> Constant Int [0]
            │       │   ├── Condition
            │       │   │   ╰── <30>  [<]
            │       │   │       ├── <27> Var [i]
            │       │   │       ╰── <29> Constant Int [5]
            │       │   ├── Condition
            │       │   │   ╰── <39> Assign [=]
            │       │   │       ├── <32> Var [i]
            │       │   │       ╰── <38>  [+]
            │       │   │           ├── <35> Var [i]
            │       │   │           ╰── <37> Constant Int [1]
            │       │   ╰── <48> Assign [=]
            │       │       ├── <41> Var [foo]
            │       │       ╰── <47>  [+]
            │       │           ├── <44> Var [foo]
            │       │           ╰── <46> Constant Int [1]
            │       ╰── Return
            │           ╰── <52> Var [foo]
            ╰── VarDeclaration
                ├── Name
                │   ╰── foo
                ╰── Type
                    ╰── Int
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_type_before_storage_class() {
    let src = r#"
        int static foo(void) {
            return 3;
        }
        int static bar = 4;
        int main(void) {
            int extern foo(void);
            int extern bar;
            return foo() + bar;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [static foo]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <7> Constant Int [3]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── bar
            │   ├── Type
            │   │   ╰── Int
            │   ├── Initializer
            │   │   ╰── <15> Constant Int [4]
            │   ╰── Static
            ╰── Function [main]
                ╰── Body
                    ├── Function [extern foo]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── bar
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Extern
                    ╰── Return
                        ╰── <40>  [+]
                            ├── <36> FunctionCall [foo]
                            ╰── <39> Var [bar]
    "#;
    assert_parse(src, expected);
}
