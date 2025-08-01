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
                    │       ╰── <8> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Static
                    ╰── Return
                        ╰── <17> Var [x]
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
                    │       ╰── <8> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Extern
                    ╰── Return
                        ╰── <17> Var [x]
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
                    │   │   ╰── <9> Constant Int [0]
                    │   ╰── Static
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
                        ╰── <21> Var [i]
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
            │           ╰── <13> Var [a]
            ╰── VarDeclaration
                ├── Name
                │   ╰── a
                ├── Type
                │   ╰── Int
                ╰── Initializer
                    ╰── <20> Constant Int [1]
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
            │           ╰── <14> Var [i]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <22> Constant Int [0]
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
            │           ╰── <6> Var [x]
            ╰── VarDeclaration
                ├── Name
                │   ╰── x
                ├── Type
                │   ╰── Int
                ╰── Initializer
                    ╰── <13> Constant Int [0]
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
                        ╰── <13> Constant Int [0]
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
            │           ╰── <11> FunctionCall [foo]
            ╰── Function [static foo]
                ╰── Body
                    ╰── Return
                        ╰── <20> Constant Int [0]
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
            │           ╰── <11> FunctionCall [foo]
            ╰── Function [static foo]
                ╰── Body
                    ╰── Return
                        ╰── <20> Constant Int [0]
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
            │           ╰── <11> Constant Int [0]
            ╰── VarDeclaration
                ├── Name
                │   ╰── foo
                ├── Type
                │   ╰── Int
                ╰── Initializer
                    ╰── <18> Constant Int [4]
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
            │           ╰── <11> Var [foo]
            ╰── VarDeclaration
                ├── Name
                │   ╰── foo
                ├── Type
                │   ╰── Int
                ╰── Initializer
                    ╰── <18> Constant Int [3]
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
            │       │       ╰── <8> Constant Int [3]
            │       ├── Block
            │       │   ╰── VarDeclaration
            │       │       ├── Name
            │       │       │   ╰── x
            │       │       ├── Type
            │       │       │   ╰── Int
            │       │       ╰── Extern
            │       ╰── Return
            │           ╰── <19> Var [x]
            ╰── VarDeclaration
                ├── Name
                │   ╰── x
                ├── Type
                │   ╰── Int
                ├── Initializer
                │   ╰── <27> Constant Int [10]
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
                    │       ╰── <8> Constant Int [0]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ├── Initializer
                    │   │       │   ╰── <15> Constant Int [0]
                    │   │       ╰── Extern
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
                    │       ╰── <41> Assign [=]
                    │           ├── <34> Var [x]
                    │           ╰── <40>  [+]
                    │               ├── <37> Var [x]
                    │               ╰── <39> Constant Int [1]
                    ╰── Return
                        ╰── <47> Var [x]
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
                    │   │   ╰── <9> Constant Int [0]
                    │   ╰── Extern
                    ╰── Return
                        ╰── <13> Var [i]
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
                    │   │   ╰── <9> Constant Int [0]
                    │   ╰── Static
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <12> Constant Int [0]
                    │   ╰── Block
                    │       ╰── Case [invalid]
                    │           ├── Value
                    │           │   ╰── <14> Var [i]
                    │           ╰── Return
                    │               ╰── <15> Constant Int [0]
                    ╰── Return
                        ╰── <21> Constant Int [0]
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
                        ╰── <22> Var [b]
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
                    │       ╰── <8> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ├── Initializer
                    │   │   ╰── <19>  [*]
                    │   │       ├── <16> Var [a]
                    │   │       ╰── <18> Constant Int [2]
                    │   ╰── Static
                    ╰── Return
                        ╰── <23> Var [b]
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
                        ╰── <16> Constant Int [0]
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
                        ╰── <14> Constant Int [0]
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
            │           ╰── <5> Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── foo
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Extern
                    ╰── Return
                        ╰── <18> Constant Int [0]
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
            │           ╰── <12> FunctionCall [foo]
            ╰── Function [static foo]
                ╰── Body
                    ╰── Return
                        ╰── <21> Constant Int [0]
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
                    │       ╰── <8> Constant Int [0]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ├── Initializer
                    │   │       │   ╰── <15> Constant Int [0]
                    │   │       ╰── Static
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
                    │       ╰── <41> Assign [=]
                    │           ├── <34> Var [x]
                    │           ╰── <40>  [+]
                    │               ├── <37> Var [x]
                    │               ╰── <39> Constant Int [1]
                    ╰── Return
                        ╰── <47> Var [x]
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
                        ╰── <11> FunctionCall [foo]
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
            │           ╰── <12> Var [a]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <23> Constant Int [3]
                    ├── Block
                    │   ├── VarDeclaration
                    │   │   ├── Name
                    │   │   │   ╰── a
                    │   │   ├── Type
                    │   │   │   ╰── Int
                    │   │   ╰── Extern
                    │   ├── If
                    │   │   ├── Condition
                    │   │   │   ╰── <35>  [!=]
                    │   │   │       ├── <32> Var [a]
                    │   │   │       ╰── <34> Constant Int [5]
                    │   │   ╰── Then
                    │   │       ╰── Return
                    │   │           ╰── <36> Constant Int [1]
                    │   ╰── <43> Assign [=]
                    │       ├── <40> Var [a]
                    │       ╰── <42> Constant Int [4]
                    ╰── Return
                        ╰── <52>  [+]
                            ├── <48> Var [a]
                            ╰── <51> FunctionCall [return_a]
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
            │       │       ╰── <8> Constant Int [1]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── foo
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <14> Constant Int [0]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <18> Var [outer]
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
            │       │               ╰── <30> Var [foo]
            │       ╰── Return
            │           ╰── <35> Constant Int [0]
            ╰── VarDeclaration
                ├── Name
                │   ╰── foo
                ├── Type
                │   ╰── Int
                ╰── Initializer
                    ╰── <42> Constant Int [3]
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
                    ├── <23> Assign [=]
                    │   ├── <18> Var [y]
                    │   ╰── <22> Unary [-]
                    │       ╰── <21> Constant Int [1]
                    ├── <37> Assign [=]
                    │   ├── <26> Var [x]
                    │   ╰── <36>  [|]
                    │       ├── <33>  [<<]
                    │       │   ├── <29> Var [x]
                    │       │   ╰── <31> Constant Int [1]
                    │       ╰── <35> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <43>  [!=]
                    │   │       ├── <40> Var [x]
                    │   │       ╰── <42> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <44> Constant Int [1]
                    ├── <67> Assign [=]
                    │   ├── <50> Var [y]
                    │   ╰── <66>  [>>]
                    │       ├── <63>  [^]
                    │       │   ├── <59>  [&]
                    │       │   │   ├── <53> Var [y]
                    │       │   │   ╰── <57> Unary [-]
                    │       │   │       ╰── <56> Constant Int [5]
                    │       │   ╰── <61> Constant Int [12]
                    │       ╰── <65> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <75>  [!=]
                    │   │       ├── <70> Var [y]
                    │   │       ╰── <74> Unary [-]
                    │   │           ╰── <73> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <76> Constant Int [2]
                    ╰── Return
                        ╰── <81> Constant Int [0]
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
            │       │   │   ╰── <9> Constant Int [0]
            │       │   ╰── Static
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── j
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ├── Initializer
            │       │   │   ╰── <16> Constant Int [0]
            │       │   ╰── Static
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── k
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ├── Initializer
            │       │   │   ╰── <23> Constant Int [1]
            │       │   ╰── Static
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── l
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ├── Initializer
            │       │   │   ╰── <30> Constant Int [48]
            │       │   ╰── Static
            │       ├── <37> Assign [+=]
            │       │   ├── <34> Var [i]
            │       │   ╰── <36> Constant Int [1]
            │       ├── <44> Assign [-=]
            │       │   ├── <40> Var [j]
            │       │   ╰── <43> Var [i]
            │       ├── <51> Assign [*=]
            │       │   ├── <47> Var [k]
            │       │   ╰── <50> Var [j]
            │       ├── <57> Assign [/=]
            │       │   ├── <54> Var [l]
            │       │   ╰── <56> Constant Int [2]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <63>  [!=]
            │       │   │       ├── <60> Var [i]
            │       │   │       ╰── <62> Constant Int [3]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <64> Constant Int [1]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <75>  [!=]
            │       │   │       ├── <70> Var [j]
            │       │   │       ╰── <74> Unary [-]
            │       │   │           ╰── <73> Constant Int [6]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <76> Constant Int [2]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <87>  [!=]
            │       │   │       ├── <82> Var [k]
            │       │   │       ╰── <86> Unary [-]
            │       │   │           ╰── <85> Constant Int [18]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <88> Constant Int [3]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <97>  [!=]
            │       │   │       ├── <94> Var [l]
            │       │   │       ╰── <96> Constant Int [6]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <98> Constant Int [4]
            │       ╰── Return
            │           ╰── <103> Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ├── <112> FunctionCall [f]
                    ├── <115> FunctionCall [f]
                    ╰── Return
                        ╰── <118> FunctionCall [f]
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
                    │   │   ╰── <11> Constant Int [10]
                    │   ╰── Static
                    ╰── Label [end]
                        ╰── Return
                            ╰── <16> Var [x]
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
            │       │   │   ╰── <21>  [==]
            │       │   │       ├── <18> Var [i]
            │       │   │       ╰── <20> Constant Int [1]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ├── <25> Postfix [++]
            │       │           │   ╰── <23> Var [i]
            │       │           ╰── <30> Unary [++]
            │       │               ╰── <29> Var [i]
            │       ╰── Return
            │           ╰── <35> Constant Int [0]
            ├── Function [decr_j]
            │   ╰── Body
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <49>  [==]
            │       │   │       ├── <44> Var [j]
            │       │   │       ╰── <48> Unary [-]
            │       │   │           ╰── <47> Constant Int [1]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── <53> Postfix [--]
            │       │               ╰── <51> Var [j]
            │       ╰── Return
            │           ╰── <58> Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ├── <73> Conditional [?]
                    │   ├── <69> Postfix [++]
                    │   │   ╰── <67> Var [i]
                    │   ├── Then
                    │   │   ╰── <70> Constant Int [0]
                    │   ╰── Else
                    │       ╰── <72> FunctionCall [incr_i]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <79>  [!=]
                    │   │       ├── <76> Var [i]
                    │   │       ╰── <78> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <80> Constant Int [1]
                    ├── <92> Conditional [?]
                    │   ├── <88> Unary [--]
                    │   │   ╰── <87> Var [j]
                    │   ├── Then
                    │   │   ╰── <90> FunctionCall [decr_j]
                    │   ╰── Else
                    │       ╰── <91> Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <100>  [!=]
                    │   │       ├── <95> Var [j]
                    │   │       ╰── <99> Unary [-]
                    │   │           ╰── <98> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <101> Constant Int [2]
                    ╰── Return
                        ╰── <106> Constant Int [0]
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
                    │       ╰── <12> Constant Int [10]
                    ├── Goto [x]
                    ├── Return
                    │   ╰── <18> Var [x]
                    ╰── Block
                        ├── VarDeclaration
                        │   ├── Name
                        │   │   ╰── x
                        │   ├── Type
                        │   │   ╰── Int
                        │   ╰── Extern
                        ╰── Label [x]
                            ╰── Return
                                ╰── <27> Var [x]
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
                    │   │   ╰── <9> Constant Int [5]
                    │   ╰── Static
                    ├── Goto [x]
                    ├── <18> Assign [=]
                    │   ├── <15> Var [x]
                    │   ╰── <17> Constant Int [0]
                    ╰── Label [x]
                        ╰── Return
                            ╰── <22> Var [x]
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
            │       │   ╰── <8> Constant Int [0]
            │       ╰── Label [x]
            │           ╰── Return
            │               ╰── <11> Constant Int [2]
            ╰── Function [f_caller]
                ╰── Body
                    ╰── Return
                        ╰── <21> FunctionCall [f]
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
            │       │   ╰── <7> Constant Int [0]
            │       ╰── Label [x]
            │           ╰── Return
            │               ╰── <10> Constant Int [1]
            ├── Function [f_caller]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <28>  [!=]
                    │   │       ├── <25> FunctionCall [f]
                    │   │       ╰── <27> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <29> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <38>  [!=]
                    │   │       ├── <35> FunctionCall [f_caller]
                    │   │       ╰── <37> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <39> Constant Int [2]
                    ╰── Return
                        ╰── <44> Constant Int [0]
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
            │       ├── <11> FunctionCall [update_x]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── x
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Extern
            │       ╰── Switch
            │           ├── Expression
            │           │   ╰── <19> Var [x]
            │           ╰── Block
            │               ├── Case [0]
            │               │   ╰── Return
            │               │       ╰── <21> Constant Int [1]
            │               ├── Case [1]
            │               │   ╰── Return
            │               │       ╰── <25> Constant Int [2]
            │               ├── Case [4]
            │               │   ╰── Return
            │               │       ╰── <29> Constant Int [0]
            │               ╰── Default
            │                   ╰── Return
            │                       ╰── <32> Constant Int [4]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── x
            │   ╰── Type
            │       ╰── Int
            ╰── Function [update_x]
                ╰── Body
                    ├── <52> Assign [=]
                    │   ├── <49> Var [x]
                    │   ╰── <51> Constant Int [4]
                    ╰── Return
                        ╰── <54> Constant Int [0]
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
            │       │       ╰── <8> Constant Int [10]
            │       ├── Switch
            │       │   ├── Expression
            │       │   │   ╰── <12> Var [a]
            │       │   ╰── Block
            │       │       ├── Case [1]
            │       │       │   ╰── Return
            │       │       │       ╰── <14> Constant Int [1]
            │       │       ├── VarDeclaration
            │       │       │   ├── Name
            │       │       │   │   ╰── x
            │       │       │   ├── Type
            │       │       │   │   ╰── Int
            │       │       │   ╰── Extern
            │       │       ├── Case [2]
            │       │       │   ╰── Return
            │       │       │       ╰── <23> Constant Int [2]
            │       │       ├── Case [10]
            │       │       │   ╰── If
            │       │       │       ├── Condition
            │       │       │       │   ╰── <34>  [==]
            │       │       │       │       ├── <31>  [*]
            │       │       │       │       │   ├── <28> Var [x]
            │       │       │       │       │   ╰── <30> Constant Int [2]
            │       │       │       │       ╰── <33> Constant Int [30]
            │       │       │       ╰── Then
            │       │       │           ╰── Block
            │       │       │               ╰── Return
            │       │       │                   ╰── <35> Constant Int [0]
            │       │       ╰── Default
            │       │           ╰── Return
            │       │               ╰── <41> Constant Int [5]
            │       ╰── Return
            │           ╰── <47> Constant Int [6]
            ╰── VarDeclaration
                ├── Name
                │   ╰── x
                ├── Type
                │   ╰── Int
                ╰── Initializer
                    ╰── <54> Constant Int [15]
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
                    │   │   ╰── <12> Var [a]
                    │   ╰── Block
                    │       ├── Case [1]
                    │       │   ╰── Empty
                    │       ├── VarDeclaration
                    │       │   ├── Name
                    │       │   │   ╰── x
                    │       │   ├── Type
                    │       │   │   ╰── Int
                    │       │   ├── Initializer
                    │       │   │   ╰── <20> Constant Int [10]
                    │       │   ╰── Static
                    │       ├── <27> Assign [=]
                    │       │   ├── <24> Var [x]
                    │       │   ╰── <26> Constant Int [0]
                    │       ╰── Case [3]
                    │           ╰── Return
                    │               ╰── <31> Var [x]
                    ╰── Return
                        ╰── <37> Constant Int [0]
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
            │           ╰── <28>  [+]
            │               ├── <24> Var [i]
            │               ╰── <27> Var [j]
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
            │           ╰── <31> FunctionCall [sum]
            │               ├── <29> Constant Int [1]
            │               ╰── <30> Constant Int [2]
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
            │       │       ╰── <65> Constant Int [3]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <72>  [>]
            │       │   │       ├── <69> Var [sum]
            │       │   │       ╰── <71> Constant Int [2]
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
            │       │               ╰── <88> FunctionCall [sum]
            │       │                   ├── <86> Constant Int [3]
            │       │                   ╰── <87> Constant Int [4]
            │       ╰── Return
            │           ╰── <93> Constant Int [1]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <105>  [!=]
                    │   │       ├── <102> FunctionCall [add_three_and_four]
                    │   │       ╰── <104> Constant Int [7]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <106> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <113>  [!=]
                    │   │       ├── <110> FunctionCall [add_one_and_two]
                    │   │       ╰── <112> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <114> Constant Int [1]
                    ╰── Return
                        ╰── <117> Constant Int [0]
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
                        ╰── <10> Var [x]
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
                    │   │   ╰── <19>  [!=]
                    │   │       ├── <16> Var [x]
                    │   │       ╰── <18> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <20> Constant Int [1]
                    ├── <27> Assign [=]
                    │   ├── <24> Var [x]
                    │   ╰── <26> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <33>  [!=]
                    │   │       ├── <30> FunctionCall [read_x]
                    │   │       ╰── <32> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <34> Constant Int [1]
                    ╰── Return
                        ╰── <37> Constant Int [0]
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
                    │       ╰── <8> Constant Int [4]
                    ╰── If
                        ├── Condition
                        │   ╰── <15>  [==]
                        │       ├── <12> Var [x]
                        │       ╰── <14> Constant Int [4]
                        ├── Then
                        │   ╰── Block
                        │       ├── VarDeclaration
                        │       │   ├── Name
                        │       │   │   ╰── x
                        │       │   ├── Type
                        │       │   │   ╰── Int
                        │       │   ╰── Extern
                        │       ╰── Return
                        │           ╰── <22> Var [x]
                        ╰── Else
                            ╰── Block
                                ╰── Return
                                    ╰── <28> Unary [-]
                                        ╰── <27> Constant Int [1]
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
                    │       ╰── <19> Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <26>  [==]
                    │   │       ├── <23> Var [x]
                    │   │       ╰── <25> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ├── If
                    │           │   ├── Condition
                    │           │   │   ╰── <31>  [!=]
                    │           │   │       ├── <28> FunctionCall [read_x]
                    │           │   │       ╰── <30> Constant Int [10]
                    │           │   ╰── Then
                    │           │       ╰── Return
                    │           │           ╰── <32> Constant Int [1]
                    │           ├── VarDeclaration
                    │           │   ├── Name
                    │           │   │   ╰── x
                    │           │   ├── Type
                    │           │   │   ╰── Int
                    │           │   ╰── Extern
                    │           ├── If
                    │           │   ├── Condition
                    │           │   │   ╰── <44>  [!=]
                    │           │   │       ├── <41> Var [x]
                    │           │   │       ╰── <43> Constant Int [10]
                    │           │   ╰── Then
                    │           │       ╰── Return
                    │           │           ╰── <45> Constant Int [1]
                    │           ╰── Return
                    │               ╰── <48> Constant Int [0]
                    ╰── Return
                        ╰── <53> Constant Int [1]
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
            │       ├── <26> Assign [=]
            │       │   ├── <22> Var [x]
            │       │   ╰── <25> Var [new_val]
            │       ╰── Return
            │           ╰── <28> Constant Int [0]
            ├── Function [read_x]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <37> Var [x]
            ╰── VarDeclaration
                ├── Name
                │   ╰── x
                ├── Type
                │   ╰── Int
                ╰── Initializer
                    ╰── <44> Constant Int [3]
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
                    │   │   ╰── <27>  [!=]
                    │   │       ├── <24> Var [x]
                    │   │       ╰── <26> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <28> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <35>  [!=]
                    │   │       ├── <32> FunctionCall [read_x]
                    │   │       ╰── <34> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <36> Constant Int [1]
                    ├── <43> Assign [=]
                    │   ├── <40> Var [x]
                    │   ╰── <42> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <49>  [!=]
                    │   │       ├── <46> Var [x]
                    │   │       ╰── <48> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <50> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <57>  [!=]
                    │   │       ├── <54> FunctionCall [read_x]
                    │   │       ╰── <56> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <58> Constant Int [1]
                    ├── <63> FunctionCall [update_x]
                    │   ╰── <62> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <69>  [!=]
                    │   │       ├── <66> Var [x]
                    │   │       ╰── <68> Constant Int [5]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <70> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <77>  [!=]
                    │   │       ├── <74> FunctionCall [read_x]
                    │   │       ╰── <76> Constant Int [5]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <78> Constant Int [1]
                    ╰── Return
                        ╰── <81> Constant Int [0]
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
                        ╰── <12> Var [x]
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
            │       │   │   ╰── <31>  [!=]
            │       │   │       ├── <28> Var [x]
            │       │   │       ╰── <30> Constant Int [1]
            │       │   ╰── Then
            │       │       ╰── Return
            │       │           ╰── <32> Constant Int [1]
            │       ├── <39> Assign [=]
            │       │   ├── <36> Var [x]
            │       │   ╰── <38> Constant Int [2]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <45>  [!=]
            │       │   │       ├── <42> FunctionCall [read_internal_x]
            │       │   │       ╰── <44> Constant Int [2]
            │       │   ╰── Then
            │       │       ╰── Return
            │       │           ╰── <46> Constant Int [1]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <53>  [!=]
            │       │   │       ├── <50> FunctionCall [read_x]
            │       │   │       ╰── <52> Constant Int [10]
            │       │   ╰── Then
            │       │       ╰── Return
            │       │           ╰── <54> Constant Int [1]
            │       ╰── Return
            │           ╰── <57> Constant Int [0]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── x
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Extern
            ╰── Function [read_internal_x]
                ╰── Body
                    ╰── Return
                        ╰── <71> Var [x]
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
            │           ╰── <12> FunctionCall [my_fun]
            ├── Function [call_static_my_fun_2]
            │   ╰── Body
            │       ├── Function [my_fun]
            │       ╰── Return
            │           ╰── <26> FunctionCall [my_fun]
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
                    │   │   ╰── <50> Constant Int [0]
                    │   ╰── Static
                    ├── <61> Assign [=]
                    │   ├── <54> Var [i]
                    │   ╰── <60>  [+]
                    │       ├── <57> Var [i]
                    │       ╰── <59> Constant Int [1]
                    ╰── Return
                        ╰── <64> Var [i]
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
            │       │   │   ╰── <25>  [!=]
            │       │   │       ├── <22> FunctionCall [call_static_my_fun]
            │       │   │       ╰── <24> Constant Int [1]
            │       │   ╰── Then
            │       │       ╰── Return
            │       │           ╰── <26> Constant Int [1]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <33>  [!=]
            │       │   │       ├── <30> FunctionCall [my_fun]
            │       │   │       ╰── <32> Constant Int [100]
            │       │   ╰── Then
            │       │       ╰── Return
            │       │           ╰── <34> Constant Int [1]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <41>  [!=]
            │       │   │       ├── <38> FunctionCall [call_static_my_fun_2]
            │       │   │       ╰── <40> Constant Int [2]
            │       │   ╰── Then
            │       │       ╰── Return
            │       │           ╰── <42> Constant Int [1]
            │       ╰── Return
            │           ╰── <45> Constant Int [0]
            ╰── Function [my_fun]
                ╰── Body
                    ╰── Return
                        ╰── <53> Constant Int [100]
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
            │           ╰── <11> Var [x]
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
            │       ├── <32> Assign [=]
            │       │   ├── <28> Var [x]
            │       │   ╰── <31> Var [new_val]
            │       ╰── Return
            │           ╰── <34> Constant Int [0]
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
            │   │   ╰── <47> Constant Int [5]
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
            │       │   │   ╰── <32>  [!=]
            │       │   │       ├── <29> Var [x]
            │       │   │       ╰── <31> Constant Int [0]
            │       │   ╰── Then
            │       │       ╰── Return
            │       │           ╰── <33> Constant Int [1]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <40>  [!=]
            │       │   │       ├── <37> FunctionCall [read_x]
            │       │   │       ╰── <39> Constant Int [5]
            │       │   ╰── Then
            │       │       ╰── Return
            │       │           ╰── <41> Constant Int [1]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── x
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Extern
            │       ├── <51> FunctionCall [update_x]
            │       │   ╰── <50> Constant Int [10]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <57>  [!=]
            │       │   │       ├── <54> FunctionCall [read_x]
            │       │   │       ╰── <56> Constant Int [10]
            │       │   ╰── Then
            │       │       ╰── Return
            │       │           ╰── <58> Constant Int [1]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <65>  [!=]
            │       │   │       ├── <62> Var [x]
            │       │   │       ╰── <64> Constant Int [0]
            │       │   ╰── Then
            │       │       ╰── Return
            │       │           ╰── <66> Constant Int [1]
            │       ├── <73> Assign [=]
            │       │   ├── <70> Var [x]
            │       │   ╰── <72> Constant Int [20]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <79>  [!=]
            │       │   │       ├── <76> Var [x]
            │       │   │       ╰── <78> Constant Int [20]
            │       │   ╰── Then
            │       │       ╰── Return
            │       │           ╰── <80> Constant Int [1]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <87>  [!=]
            │       │   │       ├── <84> FunctionCall [read_x]
            │       │   │       ╰── <86> Constant Int [10]
            │       │   ╰── Then
            │       │       ╰── Return
            │       │           ╰── <88> Constant Int [1]
            │       ╰── Return
            │           ╰── <91> Constant Int [0]
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
            │           ╰── <11> Var [foo]
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
                │   ╰── <24> Constant Int [4]
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
            │       │   │   ╰── <9> Constant Int [3]
            │       │   ╰── Static
            │       ├── <20> Assign [=]
            │       │   ├── <13> Var [a]
            │       │   ╰── <19>  [*]
            │       │       ├── <16> Var [a]
            │       │       ╰── <18> Constant Int [2]
            │       ╰── Return
            │           ╰── <23> Var [a]
            ├── Function [bar]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── a
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ├── Initializer
            │       │   │   ╰── <35> Constant Int [4]
            │       │   ╰── Static
            │       ├── <46> Assign [=]
            │       │   ├── <39> Var [a]
            │       │   ╰── <45>  [+]
            │       │       ├── <42> Var [a]
            │       │       ╰── <44> Constant Int [1]
            │       ╰── Return
            │           ╰── <49> Var [a]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <70>  [+]
                            ├── <66>  [+]
                            │   ├── <62>  [+]
                            │   │   ├── <58> FunctionCall [foo]
                            │   │   ╰── <61> FunctionCall [bar]
                            │   ╰── <65> FunctionCall [foo]
                            ╰── <69> FunctionCall [bar]
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
            │           ╰── <35>  [+]
            │               ├── <32> Var [g]
            │               ╰── <34> Constant Int [1]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <52> FunctionCall [foo]
                            ├── <44> Constant Int [0]
                            ├── <45> Constant Int [0]
                            ├── <46> Constant Int [0]
                            ├── <47> Constant Int [0]
                            ├── <48> Constant Int [0]
                            ├── <49> Constant Int [0]
                            ╰── <51> Var [zed]
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
            │       │   │   ╰── <21> Var [update_global]
            │       │   ├── Then
            │       │   │   ╰── Block
            │       │   │       ├── VarDeclaration
            │       │   │       │   ├── Name
            │       │   │       │   │   ╰── i
            │       │   │       │   ├── Type
            │       │   │       │   │   ╰── Int
            │       │   │       │   ╰── Extern
            │       │   │       ╰── <32> Assign [=]
            │       │   │           ├── <28> Var [i]
            │       │   │           ╰── <31> Var [new_val]
            │       │   ╰── Else
            │       │       ╰── <41> Assign [=]
            │       │           ├── <37> Var [i]
            │       │           ╰── <40> Var [new_val]
            │       ╰── Return
            │           ╰── <45> Var [i]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <57>  [!=]
                    │   │       ├── <54> Var [i]
                    │   │       ╰── <56> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <58> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── result
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <67> FunctionCall [update_static_or_global]
                    │           ├── <65> Constant Int [1]
                    │           ╰── <66> Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <74>  [!=]
                    │   │       ├── <71> Var [result]
                    │   │       ╰── <73> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <75> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <82>  [!=]
                    │   │       ├── <79> Var [i]
                    │   │       ╰── <81> Constant Int [10]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <83> Constant Int [1]
                    ├── <93> Assign [=]
                    │   ├── <87> Var [result]
                    │   ╰── <92> FunctionCall [update_static_or_global]
                    │       ├── <90> Constant Int [0]
                    │       ╰── <91> Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <99>  [!=]
                    │   │       ├── <96> Var [result]
                    │   │       ╰── <98> Constant Int [9]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <100> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <107>  [!=]
                    │   │       ├── <104> Var [i]
                    │   │       ╰── <106> Constant Int [10]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <108> Constant Int [1]
                    ├── <118> Assign [=]
                    │   ├── <112> Var [result]
                    │   ╰── <117> FunctionCall [update_static_or_global]
                    │       ├── <115> Constant Int [1]
                    │       ╰── <116> Constant Int [11]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <124>  [!=]
                    │   │       ├── <121> Var [result]
                    │   │       ╰── <123> Constant Int [9]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <125> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <132>  [!=]
                    │   │       ├── <129> Var [i]
                    │   │       ╰── <131> Constant Int [11]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <133> Constant Int [1]
                    ╰── Return
                        ╰── <136> Constant Int [0]
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
            │       │   │   ╰── <17> Constant Int [65]
            │       │   ╰── Static
            │       ├── <23> FunctionCall [putchar]
            │       │   ╰── <22> Var [i]
            │       ├── Block
            │       │   ├── <33> Assign [=]
            │       │   │   ├── <26> Var [i]
            │       │   │   ╰── <32>  [+]
            │       │   │       ├── <29> Var [i]
            │       │   │       ╰── <31> Constant Int [1]
            │       │   ├── VarDeclaration
            │       │   │   ├── Name
            │       │   │   │   ╰── i
            │       │   │   ├── Type
            │       │   │   │   ╰── Int
            │       │   │   ├── Initializer
            │       │   │   │   ╰── <39> Constant Int [97]
            │       │   │   ╰── Static
            │       │   ├── <45> FunctionCall [putchar]
            │       │   │   ╰── <44> Var [i]
            │       │   ╰── <55> Assign [=]
            │       │       ├── <48> Var [i]
            │       │       ╰── <54>  [+]
            │       │           ├── <51> Var [i]
            │       │           ╰── <53> Constant Int [1]
            │       ├── <61> FunctionCall [putchar]
            │       │   ╰── <60> Constant Int [10]
            │       ╰── Return
            │           ╰── <63> Constant Int [0]
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
                        │           ╰── <74> Constant Int [0]
                        ├── Condition
                        │   ╰── <82>  [<]
                        │       ├── <79> Var [i]
                        │       ╰── <81> Constant Int [26]
                        ├── Condition
                        │   ╰── <91> Assign [=]
                        │       ├── <84> Var [i]
                        │       ╰── <90>  [+]
                        │           ├── <87> Var [i]
                        │           ╰── <89> Constant Int [1]
                        ╰── <93> FunctionCall [print_letters]
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
            │       ├── <18> Assign [=]
            │       │   ├── <11> Var [x]
            │       │   ╰── <17>  [+]
            │       │       ├── <14> Var [x]
            │       │       ╰── <16> Constant Int [1]
            │       ╰── Return
            │           ╰── <21> Var [x]
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
                    │   │           ╰── <36> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <44>  [<]
                    │   │       ├── <41> Var [i]
                    │   │       ╰── <43> Constant Int [4]
                    │   ├── Condition
                    │   │   ╰── <53> Assign [=]
                    │   │       ├── <46> Var [i]
                    │   │       ╰── <52>  [+]
                    │   │           ├── <49> Var [i]
                    │   │           ╰── <51> Constant Int [1]
                    │   ╰── <59> Assign [=]
                    │       ├── <55> Var [ret]
                    │       ╰── <58> FunctionCall [foo]
                    ╰── Return
                        ╰── <63> Var [ret]
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
            │       │   │   ╰── <17> Constant Int [0]
            │       │   ╰── Static
            │       ├── <26> FunctionCall [putchar]
            │       │   ╰── <25>  [+]
            │       │       ├── <22> Var [count]
            │       │       ╰── <24> Constant Int [65]
            │       ├── <36> Assign [=]
            │       │   ├── <29> Var [count]
            │       │   ╰── <35>  [+]
            │       │       ├── <32> Var [count]
            │       │       ╰── <34> Constant Int [1]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <42>  [<]
            │       │   │       ├── <39> Var [count]
            │       │   │       ╰── <41> Constant Int [26]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── <44> FunctionCall [print_alphabet]
            │       ╰── Return
            │           ╰── <50> Var [count]
            ╰── Function [main]
                ╰── Body
                    ╰── <59> FunctionCall [print_alphabet]
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
            │           ╰── <13> Var [foo]
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
                    │   │   ╰── <9> Constant Int [2]
                    │   ╰── Static
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── j
                    │   ├── Type
                    │   │   ╰── Int
                    │   ├── Initializer
                    │   │   ╰── <16> Constant Int [3]
                    │   ╰── Static
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── cmp
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <27>  [<]
                    │           ├── <23> Var [i]
                    │           ╰── <26> Var [j]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <33> Unary [!]
                    │   │       ╰── <32> Var [cmp]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <34> Constant Int [1]
                    ╰── Return
                        ╰── <37> Constant Int [0]
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
            │       │   │           ╰── <21> Constant Int [0]
            │       │   ├── Condition
            │       │   │   ╰── <29>  [<]
            │       │   │       ├── <26> Var [i]
            │       │   │       ╰── <28> Constant Int [5]
            │       │   ├── Condition
            │       │   │   ╰── <38> Assign [=]
            │       │   │       ├── <31> Var [i]
            │       │   │       ╰── <37>  [+]
            │       │   │           ├── <34> Var [i]
            │       │   │           ╰── <36> Constant Int [1]
            │       │   ╰── <47> Assign [=]
            │       │       ├── <40> Var [foo]
            │       │       ╰── <46>  [+]
            │       │           ├── <43> Var [foo]
            │       │           ╰── <45> Constant Int [1]
            │       ╰── Return
            │           ╰── <51> Var [foo]
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
            │           ╰── <6> Constant Int [3]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── bar
            │   ├── Type
            │   │   ╰── Int
            │   ├── Initializer
            │   │   ╰── <14> Constant Int [4]
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
                        ╰── <37>  [+]
                            ├── <33> FunctionCall [foo]
                            ╰── <36> Var [bar]
    "#;
    assert_parse(src, expected);
}
