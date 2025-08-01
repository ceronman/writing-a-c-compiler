use super::{assert_error, assert_parse};

#[test]
fn test_invalid_parse_bad_specifier() {
    assert_error(
        r#"
        int main(void) {
          unsigned void *v;
        //^^^^^^^^^^^^^ Invalid type specifier
          return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_bad_specifier_2() {
    assert_error(
        r#"
        void char *x;
      //^^^^^^^^^ Invalid type specifier
        int main(void) { return 0; }
    "#,
    );
}

#[test]
fn test_invalid_parse_sizeof_cast() {
    assert_error(
        r#"
        int main(void) {
            return sizeof(char) 1;
                              //^ Expected ';', but found '1'
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_sizeof_type_no_parens() {
    assert_error(
        r#"
        
        int main(void) {
            return sizeof int;
                        //^^^ Expected expression, but found 'int'
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_bitshift_void() {
    let src = r#"
        
        void f(void){
            return;
        }
        int main(void) {
            int x = 10;
            x << f();
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [f]
            │   ╰── Body
            │       ╰── Return
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [10]
                    ├── <23>  [<<]
                    │   ├── <19> Var [x]
                    │   ╰── <22> FunctionCall [f]
                    ╰── Return
                        ╰── <25> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_bitwise_void() {
    let src = r#"
        
        int main(void) {
            int x = 10;
            int y = 11;
            x & (void) y;
            return 0;
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
                    │       ╰── <8> Constant Int [10]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── y
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <14> Constant Int [11]
                    ├── <25>  [&]
                    │   ├── <18> Var [x]
                    │   ╰── <24> Cast
                    │       ├── Target
                    │       │   ╰── Void
                    │       ╰── Expression
                    │           ╰── <23> Var [y]
                    ╰── Return
                        ╰── <27> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_compound_add_void_pointer() {
    let src = r#"
        void *malloc(unsigned long size);
        int main(void) {
            void *buff = malloc(100);
            buff += 3;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [malloc]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── size
            │           ╰── Type
            │               ╰── Unsigned Long
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── buff
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Void
                    │   ╰── Initializer
                    │       ╰── <20> FunctionCall [malloc]
                    │           ╰── <19> Constant Int [100]
                    ├── <27> Assign [+=]
                    │   ├── <24> Var [buff]
                    │   ╰── <26> Constant Int [3]
                    ╰── Return
                        ╰── <29> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_compound_sub_void_pointer() {
    let src = r#"
        void *malloc(unsigned long size);
        int main(void) {
            void *buff = malloc(100);
            buff -= 0;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [malloc]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── size
            │           ╰── Type
            │               ╰── Unsigned Long
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── buff
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Void
                    │   ╰── Initializer
                    │       ╰── <20> FunctionCall [malloc]
                    │           ╰── <19> Constant Int [100]
                    ├── <27> Assign [-=]
                    │   ├── <24> Var [buff]
                    │   ╰── <26> Constant Int [0]
                    ╰── Return
                        ╰── <29> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_compound_void_rval() {
    let src = r#"
        
        void f(void) {
            return;
        }
        int main(void) {
            int x = 10;
            x *= f();
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [f]
            │   ╰── Body
            │       ╰── Return
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [10]
                    ├── <23> Assign [*=]
                    │   ├── <19> Var [x]
                    │   ╰── <22> FunctionCall [f]
                    ╰── Return
                        ╰── <25> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_compound_void_rval_add() {
    let src = r#"
        
        void f(void) {
            return;
        }
        int main(void) {
            int *x = 0;
            x += f();
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [f]
            │   ╰── Body
            │       ╰── Return
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <16> Constant Int [0]
                    ├── <24> Assign [+=]
                    │   ├── <20> Var [x]
                    │   ╰── <23> FunctionCall [f]
                    ╰── Return
                        ╰── <26> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_compound_void_rval_bitshift() {
    let src = r#"
        
        void f(void) {
            return;
        }
        int main(void) {
            int x = 10;
            x >>= f();
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [f]
            │   ╰── Body
            │       ╰── Return
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [10]
                    ├── <23> Assign [>>=]
                    │   ├── <19> Var [x]
                    │   ╰── <22> FunctionCall [f]
                    ╰── Return
                        ╰── <25> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_postfix_decr_void() {
    let src = r#"
        
        extern void *x;
        int main(void) {
            ++(*x)--;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── x
            │   ├── Type
            │   │   ╰── Pointer
            │   │       ╰── Void
            │   ╰── Extern
            ╰── Function [main]
                ╰── Body
                    ├── <18> Unary [++]
                    │   ╰── <17> Postfix [--]
                    │       ╰── <15> Dereference
                    │           ╰── <13> Var [x]
                    ╰── Return
                        ╰── <20> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_postfix_decr_void_pointer() {
    let src = r#"
        
        void *malloc(unsigned long size);
        int main(void) {
            void *buff = malloc(100);
            buff--;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [malloc]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── size
            │           ╰── Type
            │               ╰── Unsigned Long
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── buff
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Void
                    │   ╰── Initializer
                    │       ╰── <20> FunctionCall [malloc]
                    │           ╰── <19> Constant Int [100]
                    ├── <26> Postfix [--]
                    │   ╰── <24> Var [buff]
                    ╰── Return
                        ╰── <28> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_postfix_incr_void_pointer() {
    let src = r#"
        
        void *malloc(unsigned long size);
        int main(void) {
            void *buff = malloc(100);
            buff++;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [malloc]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── size
            │           ╰── Type
            │               ╰── Unsigned Long
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── buff
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Void
                    │   ╰── Initializer
                    │       ╰── <20> FunctionCall [malloc]
                    │           ╰── <19> Constant Int [100]
                    ├── <26> Postfix [++]
                    │   ╰── <24> Var [buff]
                    ╰── Return
                        ╰── <28> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_prefix_decr_void_pointer() {
    let src = r#"
        
        void *malloc(unsigned long size);
        int main(void) {
            void *buff = malloc(100);
            --buff;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [malloc]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── size
            │           ╰── Type
            │               ╰── Unsigned Long
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── buff
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Void
                    │   ╰── Initializer
                    │       ╰── <20> FunctionCall [malloc]
                    │           ╰── <19> Constant Int [100]
                    ├── <26> Unary [--]
                    │   ╰── <25> Var [buff]
                    ╰── Return
                        ╰── <28> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_prefix_incr_void() {
    let src = r#"
        
        extern void *x;
        int main(void) {
            ++(*x);
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── x
            │   ├── Type
            │   │   ╰── Pointer
            │   │       ╰── Void
            │   ╰── Extern
            ╰── Function [main]
                ╰── Body
                    ├── <16> Unary [++]
                    │   ╰── <15> Dereference
                    │       ╰── <13> Var [x]
                    ╰── Return
                        ╰── <18> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_prefix_incr_void_pointer() {
    let src = r#"
        
        void *malloc(unsigned long size);
        int main(void) {
            void *buff = malloc(100);
            ++buff;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [malloc]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── size
            │           ╰── Type
            │               ╰── Unsigned Long
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── buff
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Void
                    │   ╰── Initializer
                    │       ╰── <20> FunctionCall [malloc]
                    │           ╰── <19> Constant Int [100]
                    ├── <26> Unary [++]
                    │   ╰── <25> Var [buff]
                    ╰── Return
                        ╰── <28> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_switch_void() {
    let src = r#"
        
        void f(void) {
            return;
        }
        int main(void) {
            switch(f()) {
                default: return 0;
            }
        }
    "#;
    let expected = r#"
        Program
            ├── Function [f]
            │   ╰── Body
            │       ╰── Return
            ╰── Function [main]
                ╰── Body
                    ╰── Switch
                        ├── Expression
                        │   ╰── <13> FunctionCall [f]
                        ╰── Block
                            ╰── Default
                                ╰── Return
                                    ╰── <14> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_incomplete_types_add_void_pointer() {
    let src = r#"
        void *malloc(unsigned long size);
        int main(void) {
          void *x = malloc(100);
          x = x + 1;
          return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [malloc]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── size
            │           ╰── Type
            │               ╰── Unsigned Long
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Void
                    │   ╰── Initializer
                    │       ╰── <20> FunctionCall [malloc]
                    │           ╰── <19> Constant Int [100]
                    ├── <31> Assign [=]
                    │   ├── <24> Var [x]
                    │   ╰── <30>  [+]
                    │       ├── <27> Var [x]
                    │       ╰── <29> Constant Int [1]
                    ╰── Return
                        ╰── <33> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_incomplete_types_sizeof_function() {
    let src = r#"
        int x(void) { return 0; }
        int main(void) { return sizeof x; }
    "#;
    let expected = r#"
        Program
            ├── Function [x]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <5> Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <15> SizeOfExpr
                            ╰── <14> Var [x]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_incomplete_types_sizeof_void() {
    let src = r#"
        int main(void) {
            return sizeof (void);
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <7> SizeOfType
                            ╰── Void
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_incomplete_types_sizeof_void_array() {
    let src = r#"
        int main(void) {
            return sizeof(void[3]);
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <10> SizeOfType
                            ╰── Array
                                ├── 3
                                ╰── Void
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_incomplete_types_sizeof_void_expression() {
    let src = r#"
        int main(void) {
          int x;
          return sizeof((void)x);
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ╰── Type
                    │       ╰── Int
                    ╰── Return
                        ╰── <15> SizeOfExpr
                            ╰── <14> Cast
                                ├── Target
                                │   ╰── Void
                                ╰── Expression
                                    ╰── <12> Var [x]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_incomplete_types_sub_void_pointer() {
    let src = r#"
        int main(void) {
          int y;
          void *x = &y;
          void *null = 0;
          return x - null;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── y
                    │   ╰── Type
                    │       ╰── Int
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Void
                    │   ╰── Initializer
                    │       ╰── <15> AddressOf
                    │           ╰── <14> Var [y]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── null
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Void
                    │   ╰── Initializer
                    │       ╰── <22> Constant Int [0]
                    ╰── Return
                        ╰── <30>  [-]
                            ├── <26> Var [x]
                            ╰── <29> Var [null]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_incomplete_types_subscript_void() {
    let src = r#"
        int main(void) {
          int x = 10;
          void *v = &x;
          v[0];
          return 0;
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
                    │       ╰── <8> Constant Int [10]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── v
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Void
                    │   ╰── Initializer
                    │       ╰── <17> AddressOf
                    │           ╰── <16> Var [x]
                    ├── <23> Subscript
                    │   ├── <21> Var [v]
                    │   ╰── <22> Constant Int [0]
                    ╰── Return
                        ╰── <25> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_incomplete_types_subscript_void_pointer_conditional() {
    let src = r#"
        int main(void) {
          int arr[3] = {1, 2, 3};
          void *void_ptr = arr;
          int *int_ptr = arr + 1;
          return (1 ? int_ptr : void_ptr)[1];
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── arr
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 3
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <10> Constant Int [1]
                    │           ├── <12> Constant Int [2]
                    │           ╰── <14> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── void_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Void
                    │   ╰── Initializer
                    │       ╰── <23> Var [arr]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── int_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <34>  [+]
                    │           ├── <31> Var [arr]
                    │           ╰── <33> Constant Int [1]
                    ╰── Return
                        ╰── <45> Subscript
                            ├── <43> Conditional [?]
                            │   ├── <37> Constant Int [1]
                            │   ├── Then
                            │   │   ╰── <39> Var [int_ptr]
                            │   ╰── Else
                            │       ╰── <41> Var [void_ptr]
                            ╰── <44> Constant Int [1]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_incomplete_types_void_array() {
    let src = r#"
        int main(void) {
            void arr[3];
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── arr
                    │   ╰── Type
                    │       ╰── Array
                    │           ├── 3
                    │           ╰── Void
                    ╰── Return
                        ╰── <11> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_incomplete_types_void_array_in_cast() {
    let src = r#"
        int main(void) {
            (void(*)[3]) 4;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── <12> Cast
                    │   ├── Target
                    │   │   ╰── Pointer
                    │   │       ╰── Array
                    │   │           ├── 3
                    │   │           ╰── Void
                    │   ╰── Expression
                    │       ╰── <11> Constant Int [4]
                    ╰── Return
                        ╰── <14> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_incomplete_types_void_array_in_param_type() {
    let src = r#"
        int arr(void foo[3]) { return 3; }
        int main(void) { return 0; }
    "#;
    let expected = r#"
        Program
            ├── Function [arr]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── foo
            │   │       ╰── Type
            │   │           ╰── Array
            │   │               ├── 3
            │   │               ╰── Void
            │   ╰── Body
            │       ╰── Return
            │           ╰── <10> Constant Int [3]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <18> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_incomplete_types_void_array_nested_in_declaration() {
    let src = r#"
        extern void (*ptr)[3][4];
        void *foo(void) {
            return ptr;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── ptr
            │   ├── Type
            │   │   ╰── Pointer
            │   │       ╰── Array
            │   │           ├── 3
            │   │           ╰── Array
            │   │               ├── 4
            │   │               ╰── Void
            │   ╰── Extern
            ╰── Function [foo]
                ╰── Body
                    ╰── Return
                        ╰── <18> Var [ptr]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_incomplete_types_void_array_pointer_in_declaration() {
    let src = r#"
        void *malloc(unsigned long size);
        int main(void) {
            void (*ptr)[3] = malloc(3);
            return ptr == 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [malloc]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── size
            │           ╰── Type
            │               ╰── Unsigned Long
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Array
                    │   │           ├── 3
                    │   │           ╰── Void
                    │   ╰── Initializer
                    │       ╰── <23> FunctionCall [malloc]
                    │           ╰── <22> Constant Int [3]
                    ╰── Return
                        ╰── <30>  [==]
                            ├── <27> Var [ptr]
                            ╰── <29> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_incomplete_types_void_array_pointer_in_param_type() {
    let src = r#"
        
        int foo(void (*bad_array)[3]) {
            return bad_array == 0;
        }
        int main(void) {
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [foo]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── bad_array
            │   │       ╰── Type
            │   │           ╰── Pointer
            │   │               ╰── Array
            │   │                   ├── 3
            │   │                   ╰── Void
            │   ╰── Body
            │       ╰── Return
            │           ╰── <16>  [==]
            │               ├── <13> Var [bad_array]
            │               ╰── <15> Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <24> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_pointer_conversions_compare_void_ptr_to_int() {
    let src = r#"
        int main(void) {
            return (void *)0 == 20ul;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <12>  [==]
                            ├── <9> Cast
                            │   ├── Target
                            │   │   ╰── Pointer
                            │   │       ╰── Void
                            │   ╰── Expression
                            │       ╰── <8> Constant Int [0]
                            ╰── <11> Constant ULong [20]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_pointer_conversions_compare_void_to_other_pointer() {
    let src = r#"
        int main(void) {
          int arr[3] = {1, 2, 3};
          void *ptr = (void *)arr;
          return ptr < arr + 1;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── arr
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 3
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <10> Constant Int [1]
                    │           ├── <12> Constant Int [2]
                    │           ╰── <14> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Void
                    │   ╰── Initializer
                    │       ╰── <27> Cast
                    │           ├── Target
                    │           │   ╰── Pointer
                    │           │       ╰── Void
                    │           ╰── Expression
                    │               ╰── <26> Var [arr]
                    ╰── Return
                        ╰── <38>  [<]
                            ├── <31> Var [ptr]
                            ╰── <37>  [+]
                                ├── <34> Var [arr]
                                ╰── <36> Constant Int [1]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_pointer_conversions_convert_ulong_to_void_ptr() {
    let src = r#"
        int main(void) {
          unsigned long x = 0;
          void *v = x;
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
                    │   │   ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <8> Constant Int [0]
                    ╰── VarDeclaration
                        ├── Name
                        │   ╰── v
                        ├── Type
                        │   ╰── Pointer
                        │       ╰── Void
                        ╰── Initializer
                            ╰── <16> Var [x]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_pointer_conversions_convert_void_ptr_to_int() {
    let src = r#"
        int main(void) {
          void *x = 0;
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
                    │   │   ╰── Pointer
                    │   │       ╰── Void
                    │   ╰── Initializer
                    │       ╰── <9> Constant Int [0]
                    ╰── Return
                        ╰── <13> Var [x]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_pointer_conversions_usual_arithmetic_conversions_ptr() {
    let src = r#"
        int main(void) {
          int i = 10 * (void *)0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── VarDeclaration
                        ├── Name
                        │   ╰── i
                        ├── Type
                        │   ╰── Int
                        ╰── Initializer
                            ╰── <15>  [*]
                                ├── <8> Constant Int [10]
                                ╰── <14> Cast
                                    ├── Target
                                    │   ╰── Pointer
                                    │       ╰── Void
                                    ╰── Expression
                                        ╰── <13> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_scalar_expressions_and_void() {
    let src = r#"
        int main(void) {
            return (void)1 && 2;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <11>  [&&]
                            ├── <8> Cast
                            │   ├── Target
                            │   │   ╰── Void
                            │   ╰── Expression
                            │       ╰── <7> Constant Int [1]
                            ╰── <10> Constant Int [2]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_scalar_expressions_cast_void() {
    let src = r#"
        int main(void) {
            int y = (int) (void) 3;
            return y;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── y
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <14> Cast
                    │           ├── Target
                    │           │   ╰── Int
                    │           ╰── Expression
                    │               ╰── <13> Cast
                    │                   ├── Target
                    │                   │   ╰── Void
                    │                   ╰── Expression
                    │                       ╰── <12> Constant Int [3]
                    ╰── Return
                        ╰── <18> Var [y]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_scalar_expressions_not_void() {
    let src = r#"
        void f(void);
        void g(void);
        int main(void) { return !(1 ? f() : g()); }
    "#;
    let expected = r#"
        Program
            ├── Function [f]
            ├── Function [g]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <23> Unary [!]
                            ╰── <22> Conditional [?]
                                ├── <16> Constant Int [1]
                                ├── Then
                                │   ╰── <18> FunctionCall [f]
                                ╰── Else
                                    ╰── <20> FunctionCall [g]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_scalar_expressions_or_void() {
    let src = r#"
        int main(void) { return 1 || (void)2; }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <11>  [||]
                            ├── <5> Constant Int [1]
                            ╰── <10> Cast
                                ├── Target
                                │   ╰── Void
                                ╰── Expression
                                    ╰── <9> Constant Int [2]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_scalar_expressions_void_condition_do_loop() {
    let src = r#"
        void f(void) { return; }
        int main(void) {
          int i = 0;
          do {
            i = i + 1;
          } while (f());
          return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [f]
            │   ╰── Body
            │       ╰── Return
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [0]
                    ├── DoWhile
                    │   ├── Body
                    │   │   ╰── Block
                    │   │       ╰── <26> Assign [=]
                    │   │           ├── <19> Var [i]
                    │   │           ╰── <25>  [+]
                    │   │               ├── <22> Var [i]
                    │   │               ╰── <24> Constant Int [1]
                    │   ╰── Condition
                    │       ╰── <31> FunctionCall [f]
                    ╰── Return
                        ╰── <33> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_scalar_expressions_void_condition_for_loop() {
    let src = r#"
        void foo(void) {
            return;
        }
        int main(void) {
            for (int i = 0; foo(); )
                ;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [foo]
            │   ╰── Body
            │       ╰── Return
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
                    │   │           ╰── <15> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <20> FunctionCall [foo]
                    │   ╰── Empty
                    ╰── Return
                        ╰── <23> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_scalar_expressions_void_condition_while_loop() {
    let src = r#"
        void f(void) { return; }
        int main(void) {
          int i = 0;
          while ((void)10) {
            i = i + 1;
          }
          return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [f]
            │   ╰── Body
            │       ╰── Return
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [0]
                    ├── While
                    │   ├── Condition
                    │   │   ╰── <21> Cast
                    │   │       ├── Target
                    │   │       │   ╰── Void
                    │   │       ╰── Expression
                    │   │           ╰── <20> Constant Int [10]
                    │   ╰── Body
                    │       ╰── Block
                    │           ╰── <30> Assign [=]
                    │               ├── <23> Var [i]
                    │               ╰── <29>  [+]
                    │                   ├── <26> Var [i]
                    │                   ╰── <28> Constant Int [1]
                    ╰── Return
                        ╰── <35> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_scalar_expressions_void_if_condition() {
    let src = r#"
        int main(void) {
          int x = 10;
          if ((void)x)
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
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <8> Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <15> Cast
                    │   │       ├── Target
                    │   │       │   ╰── Void
                    │   │       ╰── Expression
                    │   │           ╰── <14> Var [x]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <16> Constant Int [0]
                    ╰── Return
                        ╰── <19> Constant Int [1]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_scalar_expressions_void_ternary_condition() {
    let src = r#"
        void f(void);
        int main(void) {
            return f() ? 1 : 2;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [f]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <14> Conditional [?]
                            ├── <11> FunctionCall [f]
                            ├── Then
                            │   ╰── <12> Constant Int [1]
                            ╰── Else
                                ╰── <13> Constant Int [2]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_void_assign_to_void_lvalue() {
    let src = r#"
        extern void *x;
        void foo(void) { return; }
        int main(void) {
          *x = foo();
          return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── x
            │   ├── Type
            │   │   ╰── Pointer
            │   │       ╰── Void
            │   ╰── Extern
            ├── Function [foo]
            │   ╰── Body
            │       ╰── Return
            ╰── Function [main]
                ╰── Body
                    ├── <24> Assign [=]
                    │   ├── <20> Dereference
                    │   │   ╰── <19> Var [x]
                    │   ╰── <23> FunctionCall [foo]
                    ╰── Return
                        ╰── <26> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_void_assign_to_void_var() {
    let src = r#"
        extern void v1;
        int main(void) {
          v1 = (void)0;
          return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── v1
            │   ├── Type
            │   │   ╰── Void
            │   ╰── Extern
            ╰── Function [main]
                ╰── Body
                    ├── <17> Assign [=]
                    │   ├── <11> Var [v1]
                    │   ╰── <16> Cast
                    │       ├── Target
                    │       │   ╰── Void
                    │       ╰── Expression
                    │           ╰── <15> Constant Int [0]
                    ╰── Return
                        ╰── <19> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_void_assign_void_rval() {
    let src = r#"
        
        int main(void) {
          int a = 10;
          a = (void)20;
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
                    │       ╰── <8> Constant Int [10]
                    ├── <18> Assign [=]
                    │   ├── <12> Var [a]
                    │   ╰── <17> Cast
                    │       ├── Target
                    │       │   ╰── Void
                    │       ╰── Expression
                    │           ╰── <16> Constant Int [20]
                    ╰── Return
                        ╰── <20> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_void_define_void() {
    let src = r#"
        int main(void) {
            void x;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ╰── Type
                    │       ╰── Void
                    ╰── Return
                        ╰── <9> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_void_initialized_void() {
    let src = r#"
        extern void v = 0;
        int main(void) { return 0; }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── v
            │   ├── Type
            │   │   ╰── Void
            │   ├── Initializer
            │   │   ╰── <5> Constant Int [0]
            │   ╰── Extern
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <12> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_void_mismatched_conditional() {
    let src = r#"
        void foo(void) {
            return;
        }
        int main(void) {
            int a = 3;
            int flag = 4;
            flag ? foo() : (a = 3);
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [foo]
            │   ╰── Body
            │       ╰── Return
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── flag
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <21> Constant Int [4]
                    ├── <34> Conditional [?]
                    │   ├── <25> Var [flag]
                    │   ├── Then
                    │   │   ╰── <27> FunctionCall [foo]
                    │   ╰── Else
                    │       ╰── <33> Assign [=]
                    │           ├── <29> Var [a]
                    │           ╰── <31> Constant Int [3]
                    ╰── Return
                        ╰── <36> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_void_negate_void() {
    let src = r#"
        int main(void) {
          -(void)10;
          return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── <10> Unary [-]
                    │   ╰── <9> Cast
                    │       ├── Target
                    │       │   ╰── Void
                    │       ╰── Expression
                    │           ╰── <8> Constant Int [10]
                    ╰── Return
                        ╰── <12> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_void_no_return_value() {
    let src = r#"
        int foo(void) {
          return;
        }
        int main(void) {
          foo();
          return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [foo]
            │   ╰── Body
            │       ╰── Return
            ╰── Function [main]
                ╰── Body
                    ├── <13> FunctionCall [foo]
                    ╰── Return
                        ╰── <15> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_void_non_void_return() {
    let src = r#"
        void x(void) {
          return 1;
        }
        int main(void) {
          x();
          return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [x]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <5> Constant Int [1]
            ╰── Function [main]
                ╰── Body
                    ├── <14> FunctionCall [x]
                    ╰── Return
                        ╰── <16> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_void_return_void_as_pointer() {
    let src = r#"
        void *x(void) {
          return (void)0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [x]
                ╰── Body
                    ╰── Return
                        ╰── <9> Cast
                            ├── Target
                            │   ╰── Void
                            ╰── Expression
                                ╰── <8> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_void_subscript_void() {
    let src = r#"
        int main(void) {
          char arr[3];
          return arr[(void)1];
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── arr
                    │   ╰── Type
                    │       ╰── Array
                    │           ├── 3
                    │           ╰── Char
                    ╰── Return
                        ╰── <17> Subscript
                            ├── <12> Var [arr]
                            ╰── <16> Cast
                                ├── Target
                                │   ╰── Void
                                ╰── Expression
                                    ╰── <15> Constant Int [1]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_void_void_compare() {
    let src = r#"
        int main(void) {
          if ((void)1 < (void)2)
            return 1;
          return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <14>  [<]
                    │   │       ├── <8> Cast
                    │   │       │   ├── Target
                    │   │       │   │   ╰── Void
                    │   │       │   ╰── Expression
                    │   │       │       ╰── <7> Constant Int [1]
                    │   │       ╰── <13> Cast
                    │   │           ├── Target
                    │   │           │   ╰── Void
                    │   │           ╰── Expression
                    │   │               ╰── <12> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <15> Constant Int [1]
                    ╰── Return
                        ╰── <18> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_void_void_equality() {
    let src = r#"
        void x(void);
        int main(void) {
            return x() == (void)10;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [x]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <17>  [==]
                            ├── <11> FunctionCall [x]
                            ╰── <16> Cast
                                ├── Target
                                │   ╰── Void
                                ╰── Expression
                                    ╰── <15> Constant Int [10]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_void_void_fun_params() {
    let src = r#"
        void foo(void x);
        int main(void) {
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [foo]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── x
            │           ╰── Type
            │               ╰── Void
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <13> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_sizeof_bitwise() {
    let src = r#"
        
        int main(void) {
            static long l = 0;
            int i = 0;
            static char c = 0;
            if (sizeof (c & i) != 4) {
                return 1;
            }
            if (sizeof (i | l) != 8) {
                return 2;
            }
            if (sizeof (c ^ c) != 4) {
                return 3;
            }
            if (sizeof (i << l) != 4) {
                return 4;
            }
            if (sizeof (c << i) != 4) {
                return 5;
            }
            if (sizeof (l >> c) != 8) {
                return 6;
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
                    │   │   ╰── l
                    │   ├── Type
                    │   │   ╰── Long
                    │   ├── Initializer
                    │   │   ╰── <9> Constant Int [0]
                    │   ╰── Static
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Char
                    │   ├── Initializer
                    │   │   ╰── <22> Constant Int [0]
                    │   ╰── Static
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <35>  [!=]
                    │   │       ├── <32> SizeOfExpr
                    │   │       │   ╰── <31>  [&]
                    │   │       │       ├── <26> Var [c]
                    │   │       │       ╰── <29> Var [i]
                    │   │       ╰── <34> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <36> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <51>  [!=]
                    │   │       ├── <48> SizeOfExpr
                    │   │       │   ╰── <47>  [|]
                    │   │       │       ├── <42> Var [i]
                    │   │       │       ╰── <45> Var [l]
                    │   │       ╰── <50> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <52> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <67>  [!=]
                    │   │       ├── <64> SizeOfExpr
                    │   │       │   ╰── <63>  [^]
                    │   │       │       ├── <58> Var [c]
                    │   │       │       ╰── <61> Var [c]
                    │   │       ╰── <66> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <68> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <83>  [!=]
                    │   │       ├── <80> SizeOfExpr
                    │   │       │   ╰── <79>  [<<]
                    │   │       │       ├── <74> Var [i]
                    │   │       │       ╰── <77> Var [l]
                    │   │       ╰── <82> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <84> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <99>  [!=]
                    │   │       ├── <96> SizeOfExpr
                    │   │       │   ╰── <95>  [<<]
                    │   │       │       ├── <90> Var [c]
                    │   │       │       ╰── <93> Var [i]
                    │   │       ╰── <98> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <100> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <115>  [!=]
                    │   │       ├── <112> SizeOfExpr
                    │   │       │   ╰── <111>  [>>]
                    │   │       │       ├── <106> Var [l]
                    │   │       │       ╰── <109> Var [c]
                    │   │       ╰── <114> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <116> Constant Int [6]
                    ╰── Return
                        ╰── <121> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_sizeof_compound() {
    let src = r#"
        int main(void) {
            long long_arr[2] = {1, 2};
            static int i = 3;
            static unsigned char uc = 4;
            double d = 5.0;
            long *ptr = long_arr;
            if (sizeof(long_arr[1] *= 10) != 8) {
                return 1;
            }
            if (sizeof(i /= 10ul) != 4) {
                return 2;
            }
            if (sizeof(uc %= 2) != 1) {
                return 3;
            }
            if (sizeof(d -= 11) != 8) {
                return 4;
            }
            if (sizeof(ptr += 1) != 8) {
                return 5;
            }
            if (long_arr[0] != 1) {
                return 6;
            }
            if (long_arr[1] != 2) {
                return 7;
            }
            if (i != 3) {
                return 8;
            }
            if (uc != 4) {
                return 9;
            }
            if (d != 5.0) {
                return 10;
            }
            if (ptr != long_arr) {
                return 11;
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
                    │   │   ╰── long_arr
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 2
                    │   │       ╰── Long
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <10> Constant Int [1]
                    │           ╰── <12> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ├── Initializer
                    │   │   ╰── <20> Constant Int [3]
                    │   ╰── Static
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── uc
                    │   ├── Type
                    │   │   ╰── Unsigned Char
                    │   ├── Initializer
                    │   │   ╰── <27> Constant Int [4]
                    │   ╰── Static
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <33> Constant Double [+5e0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Long
                    │   ╰── Initializer
                    │       ╰── <41> Var [long_arr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <55>  [!=]
                    │   │       ├── <52> SizeOfExpr
                    │   │       │   ╰── <51> Assign [*=]
                    │   │       │       ├── <47> Subscript
                    │   │       │       │   ├── <45> Var [long_arr]
                    │   │       │       │   ╰── <46> Constant Int [1]
                    │   │       │       ╰── <49> Constant Int [10]
                    │   │       ╰── <54> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <56> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <70>  [!=]
                    │   │       ├── <67> SizeOfExpr
                    │   │       │   ╰── <66> Assign [/=]
                    │   │       │       ├── <62> Var [i]
                    │   │       │       ╰── <64> Constant ULong [10]
                    │   │       ╰── <69> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <71> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <85>  [!=]
                    │   │       ├── <82> SizeOfExpr
                    │   │       │   ╰── <81> Assign [&=]
                    │   │       │       ├── <77> Var [uc]
                    │   │       │       ╰── <79> Constant Int [2]
                    │   │       ╰── <84> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <86> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <100>  [!=]
                    │   │       ├── <97> SizeOfExpr
                    │   │       │   ╰── <96> Assign [-=]
                    │   │       │       ├── <92> Var [d]
                    │   │       │       ╰── <94> Constant Int [11]
                    │   │       ╰── <99> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <101> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <115>  [!=]
                    │   │       ├── <112> SizeOfExpr
                    │   │       │   ╰── <111> Assign [+=]
                    │   │       │       ├── <107> Var [ptr]
                    │   │       │       ╰── <109> Constant Int [1]
                    │   │       ╰── <114> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <116> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <127>  [!=]
                    │   │       ├── <124> Subscript
                    │   │       │   ├── <122> Var [long_arr]
                    │   │       │   ╰── <123> Constant Int [0]
                    │   │       ╰── <126> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <128> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <139>  [!=]
                    │   │       ├── <136> Subscript
                    │   │       │   ├── <134> Var [long_arr]
                    │   │       │   ╰── <135> Constant Int [1]
                    │   │       ╰── <138> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <140> Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <149>  [!=]
                    │   │       ├── <146> Var [i]
                    │   │       ╰── <148> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <150> Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <159>  [!=]
                    │   │       ├── <156> Var [uc]
                    │   │       ╰── <158> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <160> Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <169>  [!=]
                    │   │       ├── <166> Var [d]
                    │   │       ╰── <168> Constant Double [+5e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <170> Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <180>  [!=]
                    │   │       ├── <176> Var [ptr]
                    │   │       ╰── <179> Var [long_arr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <181> Constant Int [11]
                    ╰── Return
                        ╰── <186> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_sizeof_compound_bitwise() {
    let src = r#"
        int main(void) {
            static signed char sc = 10;
            unsigned int u = 10000u;
            long l = -99999;
            if (sizeof(sc &= l) != 1) {
                return 1;
            }
            if (sizeof(l |= u) != 8) {
                return 2;
            }
            if (sizeof(u ^= l) != 4) {
                return 3;
            }
            if (sizeof(l >>= sc) != 8) {
                return 4;
            }
            if (sizeof(sc <<= sc) != 1) {
                return 5;
            }
            if (sc != 10) {
                return 6;
            }
            if (u != 10000u) {
                return 7;
            }
            if (l != -99999) {
                return 8;
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
                    │   │   ╰── sc
                    │   ├── Type
                    │   │   ╰── Signed Char
                    │   ├── Initializer
                    │   │   ╰── <9> Constant Int [10]
                    │   ╰── Static
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── u
                    │   ├── Type
                    │   │   ╰── Unsigned Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant UInt [10000]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <23> Unary [-]
                    │           ╰── <22> Constant Int [99999]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <36>  [!=]
                    │   │       ├── <33> SizeOfExpr
                    │   │       │   ╰── <32> Assign [&=]
                    │   │       │       ├── <27> Var [sc]
                    │   │       │       ╰── <30> Var [l]
                    │   │       ╰── <35> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <37> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <52>  [!=]
                    │   │       ├── <49> SizeOfExpr
                    │   │       │   ╰── <48> Assign [|=]
                    │   │       │       ├── <43> Var [l]
                    │   │       │       ╰── <46> Var [u]
                    │   │       ╰── <51> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <53> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <68>  [!=]
                    │   │       ├── <65> SizeOfExpr
                    │   │       │   ╰── <64> Assign [^=]
                    │   │       │       ├── <59> Var [u]
                    │   │       │       ╰── <62> Var [l]
                    │   │       ╰── <67> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <69> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <84>  [!=]
                    │   │       ├── <81> SizeOfExpr
                    │   │       │   ╰── <80> Assign [>>=]
                    │   │       │       ├── <75> Var [l]
                    │   │       │       ╰── <78> Var [sc]
                    │   │       ╰── <83> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <85> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <100>  [!=]
                    │   │       ├── <97> SizeOfExpr
                    │   │       │   ╰── <96> Assign [<<=]
                    │   │       │       ├── <91> Var [sc]
                    │   │       │       ╰── <94> Var [sc]
                    │   │       ╰── <99> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <101> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <110>  [!=]
                    │   │       ├── <107> Var [sc]
                    │   │       ╰── <109> Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <111> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <120>  [!=]
                    │   │       ├── <117> Var [u]
                    │   │       ╰── <119> Constant UInt [10000]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <121> Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <132>  [!=]
                    │   │       ├── <127> Var [l]
                    │   │       ╰── <131> Unary [-]
                    │   │           ╰── <130> Constant Int [99999]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <133> Constant Int [8]
                    ╰── Return
                        ╰── <138> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_sizeof_incr() {
    let src = r#"
        int main(void) {
            int i = 0;
            long l = 0;
            static char arr[3] = {0, 0, 0};
            char *ptr = arr;
            if (sizeof (i++) != 4) {
                return 1;
            }
            if (sizeof (arr[0]--) != 1) {
                return 2;
            }
            if (sizeof (++l) != 8) {
                return 3;
            }
            if (sizeof (--arr[1]) != 1) {
                return 4;
            }
            if (sizeof (ptr--) != 8) {
                return 5;
            }
            if (i) {
                return 6;
            }
            if (l) {
                return 7;
            }
            if (arr[0] || arr[1] || arr[2]) {
                return 8;
            }
            if (ptr != arr) {
                return 9;
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
                    │   ╰── Initializer
                    │       ╰── <8> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <14> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── arr
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 3
                    │   │       ╰── Char
                    │   ├── Initializer
                    │   │   ╰── Compound
                    │   │       ├── <23> Constant Int [0]
                    │   │       ├── <25> Constant Int [0]
                    │   │       ╰── <27> Constant Int [0]
                    │   ╰── Static
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── <36> Var [arr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <47>  [!=]
                    │   │       ├── <44> SizeOfExpr
                    │   │       │   ╰── <43> Postfix [++]
                    │   │       │       ╰── <40> Var [i]
                    │   │       ╰── <46> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <48> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <63>  [!=]
                    │   │       ├── <60> SizeOfExpr
                    │   │       │   ╰── <59> Postfix [--]
                    │   │       │       ╰── <56> Subscript
                    │   │       │           ├── <54> Var [arr]
                    │   │       │           ╰── <55> Constant Int [0]
                    │   │       ╰── <62> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <64> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <77>  [!=]
                    │   │       ├── <74> SizeOfExpr
                    │   │       │   ╰── <73> Unary [++]
                    │   │       │       ╰── <71> Var [l]
                    │   │       ╰── <76> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <78> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <93>  [!=]
                    │   │       ├── <90> SizeOfExpr
                    │   │       │   ╰── <89> Unary [--]
                    │   │       │       ╰── <87> Subscript
                    │   │       │           ├── <85> Var [arr]
                    │   │       │           ╰── <86> Constant Int [1]
                    │   │       ╰── <92> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <94> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <107>  [!=]
                    │   │       ├── <104> SizeOfExpr
                    │   │       │   ╰── <103> Postfix [--]
                    │   │       │       ╰── <100> Var [ptr]
                    │   │       ╰── <106> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <108> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <114> Var [i]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <115> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <121> Var [l]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <122> Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <142>  [||]
                    │   │       ├── <136>  [||]
                    │   │       │   ├── <130> Subscript
                    │   │       │   │   ├── <128> Var [arr]
                    │   │       │   │   ╰── <129> Constant Int [0]
                    │   │       │   ╰── <135> Subscript
                    │   │       │       ├── <133> Var [arr]
                    │   │       │       ╰── <134> Constant Int [1]
                    │   │       ╰── <141> Subscript
                    │   │           ├── <139> Var [arr]
                    │   │           ╰── <140> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <143> Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <153>  [!=]
                    │   │       ├── <149> Var [ptr]
                    │   │       ╰── <152> Var [arr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <154> Constant Int [9]
                    ╰── Return
                        ╰── <159> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_pass_alloced_memory() {
    let src = r#"
        void *calloc(unsigned long nmemb, unsigned long size);
        void *memset(void *s, int c, unsigned long n);
        void free(void *ptr);
        void *get_100_zeroed_bytes(void) {
            return calloc(100, 1);
        }
        void fill_100_bytes(void *pointer, int byte) {
            memset(pointer, byte, 100);
        }
        void free_bytes(void *ptr) {
            free(ptr);
        }
    "#;
    let expected = r#"
        Program
            ├── Function [calloc]
            │   ╰── Parameters
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── nmemb
            │       │   ╰── Type
            │       │       ╰── Unsigned Long
            │       ╰── Param
            │           ├── Name
            │           │   ╰── size
            │           ╰── Type
            │               ╰── Unsigned Long
            ├── Function [memset]
            │   ╰── Parameters
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── s
            │       │   ╰── Type
            │       │       ╰── Pointer
            │       │           ╰── Void
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── c
            │       │   ╰── Type
            │       │       ╰── Int
            │       ╰── Param
            │           ├── Name
            │           │   ╰── n
            │           ╰── Type
            │               ╰── Unsigned Long
            ├── Function [free]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── ptr
            │           ╰── Type
            │               ╰── Pointer
            │                   ╰── Void
            ├── Function [get_100_zeroed_bytes]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <46> FunctionCall [calloc]
            │               ├── <44> Constant Int [100]
            │               ╰── <45> Constant Int [1]
            ├── Function [fill_100_bytes]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── pointer
            │   │   │   ╰── Type
            │   │   │       ╰── Pointer
            │   │   │           ╰── Void
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── byte
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ╰── <67> FunctionCall [memset]
            │           ├── <63> Var [pointer]
            │           ├── <65> Var [byte]
            │           ╰── <66> Constant Int [100]
            ╰── Function [free_bytes]
                ├── Parameters
                │   ╰── Param
                │       ├── Name
                │       │   ╰── ptr
                │       ╰── Type
                │           ╰── Pointer
                │               ╰── Void
                ╰── Body
                    ╰── <82> FunctionCall [free]
                        ╰── <81> Var [ptr]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_pass_alloced_memory_client() {
    let src = r#"
        void *get_100_zeroed_bytes(void);
        void fill_100_bytes(void *pointer, int byte);
        void free_bytes(void *ptr);
        int main(void) {
            void *mem = get_100_zeroed_bytes();
            for (int i = 0; i < 100; i = i + 1) {
                if (((char *) mem + i)[0]) {
                    return 1;
                }
            }
            fill_100_bytes(mem, 99);
            for (int i = 0; i < 100; i = i + 1) {
                if (((char *) mem + i)[0] != 99) {
                    return 2;
                }
            }
            free_bytes(mem);
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [get_100_zeroed_bytes]
            ├── Function [fill_100_bytes]
            │   ╰── Parameters
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── pointer
            │       │   ╰── Type
            │       │       ╰── Pointer
            │       │           ╰── Void
            │       ╰── Param
            │           ├── Name
            │           │   ╰── byte
            │           ╰── Type
            │               ╰── Int
            ├── Function [free_bytes]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── ptr
            │           ╰── Type
            │               ╰── Pointer
            │                   ╰── Void
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── mem
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Void
                    │   ╰── Initializer
                    │       ╰── <37> FunctionCall [get_100_zeroed_bytes]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <43> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <51>  [<]
                    │   │       ├── <48> Var [i]
                    │   │       ╰── <50> Constant Int [100]
                    │   ├── Condition
                    │   │   ╰── <60> Assign [=]
                    │   │       ├── <53> Var [i]
                    │   │       ╰── <59>  [+]
                    │   │           ├── <56> Var [i]
                    │   │           ╰── <58> Constant Int [1]
                    │   ╰── Block
                    │       ╰── If
                    │           ├── Condition
                    │           │   ╰── <73> Subscript
                    │           │       ├── <71>  [+]
                    │           │       │   ├── <66> Cast
                    │           │       │   │   ├── Target
                    │           │       │   │   │   ╰── Pointer
                    │           │       │   │   │       ╰── Char
                    │           │       │   │   ╰── Expression
                    │           │       │   │       ╰── <65> Var [mem]
                    │           │       │   ╰── <69> Var [i]
                    │           │       ╰── <72> Constant Int [0]
                    │           ╰── Then
                    │               ╰── Block
                    │                   ╰── Return
                    │                       ╰── <74> Constant Int [1]
                    ├── <86> FunctionCall [fill_100_bytes]
                    │   ├── <84> Var [mem]
                    │   ╰── <85> Constant Int [99]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <91> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <99>  [<]
                    │   │       ├── <96> Var [i]
                    │   │       ╰── <98> Constant Int [100]
                    │   ├── Condition
                    │   │   ╰── <108> Assign [=]
                    │   │       ├── <101> Var [i]
                    │   │       ╰── <107>  [+]
                    │   │           ├── <104> Var [i]
                    │   │           ╰── <106> Constant Int [1]
                    │   ╰── Block
                    │       ╰── If
                    │           ├── Condition
                    │           │   ╰── <124>  [!=]
                    │           │       ├── <121> Subscript
                    │           │       │   ├── <119>  [+]
                    │           │       │   │   ├── <114> Cast
                    │           │       │   │   │   ├── Target
                    │           │       │   │   │   │   ╰── Pointer
                    │           │       │   │   │   │       ╰── Char
                    │           │       │   │   │   ╰── Expression
                    │           │       │   │   │       ╰── <113> Var [mem]
                    │           │       │   │   ╰── <117> Var [i]
                    │           │       │   ╰── <120> Constant Int [0]
                    │           │       ╰── <123> Constant Int [99]
                    │           ╰── Then
                    │               ╰── Block
                    │                   ╰── Return
                    │                       ╰── <125> Constant Int [2]
                    ├── <136> FunctionCall [free_bytes]
                    │   ╰── <135> Var [mem]
                    ╰── Return
                        ╰── <138> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_sizeof_extern() {
    let src = r#"
        double large_array[1000][2000];
    "#;
    let expected = r#"
        Program
            ╰── VarDeclaration
                ├── Name
                │   ╰── large_array
                ╰── Type
                    ╰── Array
                        ├── 1000
                        ╰── Array
                            ├── 2000
                            ╰── Double
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_sizeof_extern_client() {
    let src = r#"
        
        extern double large_array[1000][2000];
        int main(void) {
            return sizeof large_array == 16000000;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── large_array
            │   ├── Type
            │   │   ╰── Array
            │   │       ├── 1000
            │   │       ╰── Array
            │   │           ├── 2000
            │   │           ╰── Double
            │   ╰── Extern
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <19>  [==]
                            ├── <16> SizeOfExpr
                            │   ╰── <15> Var [large_array]
                            ╰── <18> Constant Int [16000000]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_test_for_memory_leaks() {
    let src = r#"
        void exit(int status);
        long sum = 0;
        void lots_of_args(int a, int b, int c, int d, int e, int f, int g, int h, int i,
                          int j, int k, int l, int m, int n, int o) {
            if (a != 1) {
                exit(1);
            }
            if (b != 2) {
                exit(2);
            }
            if (c != 3) {
                exit(3);
            }
            if (d != 4) {
                exit(4);
            }
            if (e != 5) {
                exit(5);
            }
            if (f != 6) {
                exit(6);
            }
            if (g != 7) {
                exit(7);
            }
            if (h != 8) {
                exit(8);
            }
            if (i != 9) {
                exit(9);
            }
            if (j != 10) {
                exit(10);
            }
            if (k != 11) {
                exit(11);
            }
            if (l != 12) {
                exit(12);
            }
            if (m != 13) {
                exit(13);
            }
            if (n != 14) {
             exit(14);
            }
            sum = sum + o;
            return;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [exit]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── status
            │           ╰── Type
            │               ╰── Int
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── sum
            │   ├── Type
            │   │   ╰── Long
            │   ╰── Initializer
            │       ╰── <12> Constant Int [0]
            ╰── Function [lots_of_args]
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
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── h
                │   │   ╰── Type
                │   │       ╰── Int
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── i
                │   │   ╰── Type
                │   │       ╰── Int
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── j
                │   │   ╰── Type
                │   │       ╰── Int
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── k
                │   │   ╰── Type
                │   │       ╰── Int
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── l
                │   │   ╰── Type
                │   │       ╰── Int
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── m
                │   │   ╰── Type
                │   │       ╰── Int
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── n
                │   │   ╰── Type
                │   │       ╰── Int
                │   ╰── Param
                │       ├── Name
                │       │   ╰── o
                │       ╰── Type
                │           ╰── Int
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <68>  [!=]
                    │   │       ├── <65> Var [a]
                    │   │       ╰── <67> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── <71> FunctionCall [exit]
                    │               ╰── <70> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <80>  [!=]
                    │   │       ├── <77> Var [b]
                    │   │       ╰── <79> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── <83> FunctionCall [exit]
                    │               ╰── <82> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <92>  [!=]
                    │   │       ├── <89> Var [c]
                    │   │       ╰── <91> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── <95> FunctionCall [exit]
                    │               ╰── <94> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <104>  [!=]
                    │   │       ├── <101> Var [d]
                    │   │       ╰── <103> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── <107> FunctionCall [exit]
                    │               ╰── <106> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <116>  [!=]
                    │   │       ├── <113> Var [e]
                    │   │       ╰── <115> Constant Int [5]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── <119> FunctionCall [exit]
                    │               ╰── <118> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <128>  [!=]
                    │   │       ├── <125> Var [f]
                    │   │       ╰── <127> Constant Int [6]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── <131> FunctionCall [exit]
                    │               ╰── <130> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <140>  [!=]
                    │   │       ├── <137> Var [g]
                    │   │       ╰── <139> Constant Int [7]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── <143> FunctionCall [exit]
                    │               ╰── <142> Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <152>  [!=]
                    │   │       ├── <149> Var [h]
                    │   │       ╰── <151> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── <155> FunctionCall [exit]
                    │               ╰── <154> Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <164>  [!=]
                    │   │       ├── <161> Var [i]
                    │   │       ╰── <163> Constant Int [9]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── <167> FunctionCall [exit]
                    │               ╰── <166> Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <176>  [!=]
                    │   │       ├── <173> Var [j]
                    │   │       ╰── <175> Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── <179> FunctionCall [exit]
                    │               ╰── <178> Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <188>  [!=]
                    │   │       ├── <185> Var [k]
                    │   │       ╰── <187> Constant Int [11]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── <191> FunctionCall [exit]
                    │               ╰── <190> Constant Int [11]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <200>  [!=]
                    │   │       ├── <197> Var [l]
                    │   │       ╰── <199> Constant Int [12]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── <203> FunctionCall [exit]
                    │               ╰── <202> Constant Int [12]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <212>  [!=]
                    │   │       ├── <209> Var [m]
                    │   │       ╰── <211> Constant Int [13]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── <215> FunctionCall [exit]
                    │               ╰── <214> Constant Int [13]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <224>  [!=]
                    │   │       ├── <221> Var [n]
                    │   │       ╰── <223> Constant Int [14]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── <227> FunctionCall [exit]
                    │               ╰── <226> Constant Int [14]
                    ├── <241> Assign [=]
                    │   ├── <233> Var [sum]
                    │   ╰── <240>  [+]
                    │       ├── <236> Var [sum]
                    │       ╰── <239> Var [o]
                    ╰── Return
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_test_for_memory_leaks_client() {
    let src = r#"
        extern long sum;
        void lots_of_args(int a, int b, int c, int d, int e, int f, int g, int h, int i, int j, int k, int l, int m, int n, int o);
        int main(void) {
            for (int i = 0; i < 10000000; i = i + 1) {
                lots_of_args(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, i);
            }
            if (sum != 49999995000000) {
                return 15;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── sum
            │   ├── Type
            │   │   ╰── Long
            │   ╰── Extern
            ├── Function [lots_of_args]
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
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── i
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── j
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── k
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── l
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── m
            │       │   ╰── Type
            │       │       ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── n
            │       │   ╰── Type
            │       │       ╰── Int
            │       ╰── Param
            │           ├── Name
            │           │   ╰── o
            │           ╰── Type
            │               ╰── Int
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
                    │   │           ╰── <63> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <71>  [<]
                    │   │       ├── <68> Var [i]
                    │   │       ╰── <70> Constant Int [10000000]
                    │   ├── Condition
                    │   │   ╰── <80> Assign [=]
                    │   │       ├── <73> Var [i]
                    │   │       ╰── <79>  [+]
                    │   │           ├── <76> Var [i]
                    │   │           ╰── <78> Constant Int [1]
                    │   ╰── Block
                    │       ╰── <98> FunctionCall [lots_of_args]
                    │           ├── <82> Constant Int [1]
                    │           ├── <83> Constant Int [2]
                    │           ├── <84> Constant Int [3]
                    │           ├── <85> Constant Int [4]
                    │           ├── <86> Constant Int [5]
                    │           ├── <87> Constant Int [6]
                    │           ├── <88> Constant Int [7]
                    │           ├── <89> Constant Int [8]
                    │           ├── <90> Constant Int [9]
                    │           ├── <91> Constant Int [10]
                    │           ├── <92> Constant Int [11]
                    │           ├── <93> Constant Int [12]
                    │           ├── <94> Constant Int [13]
                    │           ├── <95> Constant Int [14]
                    │           ╰── <97> Var [i]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <107>  [!=]
                    │   │       ├── <104> Var [sum]
                    │   │       ╰── <106> Constant Long [49999995000000]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <108> Constant Int [15]
                    ╰── Return
                        ╰── <113> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_sizeof_simple() {
    let src = r#"
        int main(void) {
            if (sizeof (int) != 4) {
                return 1;
            }
            if (sizeof 3.0 != 8) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <10>  [!=]
                    │   │       ├── <7> SizeOfType
                    │   │       │   ╰── Int
                    │   │       ╰── <9> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <11> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <20>  [!=]
                    │   │       ├── <17> SizeOfExpr
                    │   │       │   ╰── <16> Constant Double [+3e0]
                    │   │       ╰── <19> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <21> Constant Int [2]
                    ╰── Return
                        ╰── <26> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_sizeof_sizeof_array() {
    let src = r#"
        unsigned long sizeof_adjusted_param(int arr[3]) {
            return sizeof arr;
        }
        int main(void) {
            int arr[3];
            if (sizeof arr != 12) {
                return 1;
            }
            static long nested_arr[4][5];
            if (sizeof nested_arr[2] != 40) {
                return 2;
            }
            if (sizeof "Hello, World!" != 14) {
                return 3;
            }
            if (sizeof_adjusted_param(arr) != 8) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [sizeof_adjusted_param]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── arr
            │   │       ╰── Type
            │   │           ╰── Array
            │   │               ├── 3
            │   │               ╰── Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <12> SizeOfExpr
            │               ╰── <11> Var [arr]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── arr
                    │   ╰── Type
                    │       ╰── Array
                    │           ├── 3
                    │           ╰── Int
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <31>  [!=]
                    │   │       ├── <28> SizeOfExpr
                    │   │       │   ╰── <27> Var [arr]
                    │   │       ╰── <30> Constant Int [12]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <32> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── nested_arr
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 4
                    │   │       ╰── Array
                    │   │           ├── 5
                    │   │           ╰── Long
                    │   ╰── Static
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <53>  [!=]
                    │   │       ├── <50> SizeOfExpr
                    │   │       │   ╰── <49> Subscript
                    │   │       │       ├── <47> Var [nested_arr]
                    │   │       │       ╰── <48> Constant Int [2]
                    │   │       ╰── <52> Constant Int [40]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <54> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <63>  [!=]
                    │   │       ├── <60> SizeOfExpr
                    │   │       │   ╰── <59> "Hello, World!"
                    │   │       ╰── <62> Constant Int [14]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <64> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <75>  [!=]
                    │   │       ├── <72> FunctionCall [sizeof_adjusted_param]
                    │   │       │   ╰── <71> Var [arr]
                    │   │       ╰── <74> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <76> Constant Int [4]
                    ╰── Return
                        ╰── <81> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_sizeof_sizeof_basic_types() {
    let src = r#"
        int main(void) {
            if (sizeof(char) != 1) {
                return 1;
            }
            if (sizeof(signed char) != 1) {
                return 2;
            }
            if (sizeof(unsigned char) != 1) {
                return 3;
            }
            if (sizeof(int) != 4) {
                return 4;
            }
            if (sizeof(unsigned int) != 4) {
                return 5;
            }
            if (sizeof(long) != 8) {
                return 6;
            }
            if (sizeof(unsigned long) != 8) {
                return 7;
            }
            if (sizeof(double) != 8) {
                return 8;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <10>  [!=]
                    │   │       ├── <7> SizeOfType
                    │   │       │   ╰── Char
                    │   │       ╰── <9> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <11> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <21>  [!=]
                    │   │       ├── <18> SizeOfType
                    │   │       │   ╰── Signed Char
                    │   │       ╰── <20> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <22> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <32>  [!=]
                    │   │       ├── <29> SizeOfType
                    │   │       │   ╰── Unsigned Char
                    │   │       ╰── <31> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <33> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <43>  [!=]
                    │   │       ├── <40> SizeOfType
                    │   │       │   ╰── Int
                    │   │       ╰── <42> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <44> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <54>  [!=]
                    │   │       ├── <51> SizeOfType
                    │   │       │   ╰── Unsigned Int
                    │   │       ╰── <53> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <55> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <65>  [!=]
                    │   │       ├── <62> SizeOfType
                    │   │       │   ╰── Long
                    │   │       ╰── <64> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <66> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <76>  [!=]
                    │   │       ├── <73> SizeOfType
                    │   │       │   ╰── Unsigned Long
                    │   │       ╰── <75> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <77> Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <87>  [!=]
                    │   │       ├── <84> SizeOfType
                    │   │       │   ╰── Double
                    │   │       ╰── <86> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <88> Constant Int [8]
                    ╰── Return
                        ╰── <93> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_sizeof_sizeof_consts() {
    let src = r#"
        int main(void) {
            if (sizeof 'a' != 4) {
                return 1;
            }
            if (sizeof 2147483647 != 4) {
                return 2;
            }
            if (sizeof 4294967295U != 4) {
                return 3;
            }
            if (sizeof 2l != 8) {
                return 4;
            }
            if (sizeof 0ul != 8) {
                return 5;
            }
            if (sizeof 1.0 != 8) {
                return 6;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <9>  [!=]
                    │   │       ├── <6> SizeOfExpr
                    │   │       │   ╰── <5> Constant Int [97]
                    │   │       ╰── <8> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <10> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <19>  [!=]
                    │   │       ├── <16> SizeOfExpr
                    │   │       │   ╰── <15> Constant Int [2147483647]
                    │   │       ╰── <18> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <20> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <29>  [!=]
                    │   │       ├── <26> SizeOfExpr
                    │   │       │   ╰── <25> Constant UInt [4294967295]
                    │   │       ╰── <28> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <30> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <39>  [!=]
                    │   │       ├── <36> SizeOfExpr
                    │   │       │   ╰── <35> Constant Long [2]
                    │   │       ╰── <38> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <40> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <49>  [!=]
                    │   │       ├── <46> SizeOfExpr
                    │   │       │   ╰── <45> Constant ULong [0]
                    │   │       ╰── <48> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <50> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <59>  [!=]
                    │   │       ├── <56> SizeOfExpr
                    │   │       │   ╰── <55> Constant Double [+1e0]
                    │   │       ╰── <58> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <60> Constant Int [6]
                    ╰── Return
                        ╰── <65> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_sizeof_sizeof_derived_types() {
    let src = r#"
        void *malloc(unsigned long size);
        int main(void) {
            if (sizeof(int[2]) != 8) {
                return 1;
            }
            if (sizeof(char[3][6][17][9]) != 2754) {
                return 2;
            }
            if (sizeof(int[4294967297L][100000000]) != 1717986918800000000l) {
                return 3;
            }
            if (sizeof(int *) != 8) {
                return 4;
            }
            if (sizeof(int(*)[2][4][6]) !=
                8) {
                return 5;
            }
            if (sizeof(char *) != 8) {
                return 6;
            }
            if (sizeof(double(*([3][4]))[2]) != 96) {
                return 7;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [malloc]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── size
            │           ╰── Type
            │               ╰── Unsigned Long
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <22>  [!=]
                    │   │       ├── <19> SizeOfType
                    │   │       │   ╰── Array
                    │   │       │       ├── 2
                    │   │       │       ╰── Int
                    │   │       ╰── <21> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <23> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <42>  [!=]
                    │   │       ├── <39> SizeOfType
                    │   │       │   ╰── Array
                    │   │       │       ├── 3
                    │   │       │       ╰── Array
                    │   │       │           ├── 6
                    │   │       │           ╰── Array
                    │   │       │               ├── 17
                    │   │       │               ╰── Array
                    │   │       │                   ├── 9
                    │   │       │                   ╰── Char
                    │   │       ╰── <41> Constant Int [2754]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <43> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <58>  [!=]
                    │   │       ├── <55> SizeOfType
                    │   │       │   ╰── Array
                    │   │       │       ├── 4294967297
                    │   │       │       ╰── Array
                    │   │       │           ├── 100000000
                    │   │       │           ╰── Int
                    │   │       ╰── <57> Constant Long [1717986918800000000]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <59> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <70>  [!=]
                    │   │       ├── <67> SizeOfType
                    │   │       │   ╰── Pointer
                    │   │       │       ╰── Int
                    │   │       ╰── <69> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <71> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <89>  [!=]
                    │   │       ├── <86> SizeOfType
                    │   │       │   ╰── Pointer
                    │   │       │       ╰── Array
                    │   │       │           ├── 2
                    │   │       │           ╰── Array
                    │   │       │               ├── 4
                    │   │       │               ╰── Array
                    │   │       │                   ├── 6
                    │   │       │                   ╰── Int
                    │   │       ╰── <88> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <90> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <101>  [!=]
                    │   │       ├── <98> SizeOfType
                    │   │       │   ╰── Pointer
                    │   │       │       ╰── Char
                    │   │       ╰── <100> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <102> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <122>  [!=]
                    │   │       ├── <119> SizeOfType
                    │   │       │   ╰── Array
                    │   │       │       ├── 3
                    │   │       │       ╰── Array
                    │   │       │           ├── 4
                    │   │       │           ╰── Pointer
                    │   │       │               ╰── Array
                    │   │       │                   ├── 2
                    │   │       │                   ╰── Double
                    │   │       ╰── <121> Constant Int [96]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <123> Constant Int [7]
                    ╰── Return
                        ╰── <128> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_sizeof_sizeof_expressions() {
    let src = r#"
        void *malloc(unsigned long size);
        void free(void *ptr);
        int main(void) {
            double d;
            if (sizeof d != 8) {
                return 2;
            }
            unsigned char c;
            if (sizeof c != 1) {
                return 3;
            }
            void *buffer = malloc(100);
            if (sizeof(buffer) != 8) {
                return 4;
            }
            free(buffer);
            if (sizeof ((int)d) != 4) {
                return 5;
            }
            if (sizeof (d ? c : 10l) != 8) {
                return 6;
            }
            if (sizeof (c = 10.0) != 1) {
                return 7;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [malloc]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── size
            │           ╰── Type
            │               ╰── Unsigned Long
            ├── Function [free]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── ptr
            │           ╰── Type
            │               ╰── Pointer
            │                   ╰── Void
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ╰── Type
                    │       ╰── Double
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <32>  [!=]
                    │   │       ├── <29> SizeOfExpr
                    │   │       │   ╰── <28> Var [d]
                    │   │       ╰── <31> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <33> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ╰── Type
                    │       ╰── Unsigned Char
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <47>  [!=]
                    │   │       ├── <44> SizeOfExpr
                    │   │       │   ╰── <43> Var [c]
                    │   │       ╰── <46> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <48> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── buffer
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Void
                    │   ╰── Initializer
                    │       ╰── <59> FunctionCall [malloc]
                    │           ╰── <58> Constant Int [100]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <68>  [!=]
                    │   │       ├── <65> SizeOfExpr
                    │   │       │   ╰── <64> Var [buffer]
                    │   │       ╰── <67> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <69> Constant Int [4]
                    ├── <77> FunctionCall [free]
                    │   ╰── <76> Var [buffer]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <88>  [!=]
                    │   │       ├── <85> SizeOfExpr
                    │   │       │   ╰── <84> Cast
                    │   │       │       ├── Target
                    │   │       │       │   ╰── Int
                    │   │       │       ╰── Expression
                    │   │       │           ╰── <82> Var [d]
                    │   │       ╰── <87> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <89> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <104>  [!=]
                    │   │       ├── <101> SizeOfExpr
                    │   │       │   ╰── <100> Conditional [?]
                    │   │       │       ├── <95> Var [d]
                    │   │       │       ├── Then
                    │   │       │       │   ╰── <97> Var [c]
                    │   │       │       ╰── Else
                    │   │       │           ╰── <98> Constant Long [10]
                    │   │       ╰── <103> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <105> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <119>  [!=]
                    │   │       ├── <116> SizeOfExpr
                    │   │       │   ╰── <115> Assign [=]
                    │   │       │       ├── <111> Var [c]
                    │   │       │       ╰── <113> Constant Double [+1e1]
                    │   │       ╰── <118> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <120> Constant Int [7]
                    ╰── Return
                        ╰── <125> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_sizeof_sizeof_not_evaluated() {
    let src = r#"
        void exit(int status);
        int foo(void) { exit(10); }
        int main(void) {
          return sizeof(foo());
        }
    "#;
    let expected = r#"
        Program
            ├── Function [exit]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── status
            │           ╰── Type
            │               ╰── Int
            ├── Function [foo]
            │   ╰── Body
            │       ╰── <15> FunctionCall [exit]
            │           ╰── <14> Constant Int [10]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <26> SizeOfExpr
                            ╰── <25> FunctionCall [foo]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_sizeof_sizeof_result_is_ulong() {
    let src = r#"
        int main(void) {
            if (sizeof sizeof (char) != 8) {
                return 1;
            }
            if (sizeof 4 - sizeof 4 - 1 < 0) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <11>  [!=]
                    │   │       ├── <8> SizeOfExpr
                    │   │       │   ╰── <7> SizeOfType
                    │   │       │       ╰── Char
                    │   │       ╰── <10> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <12> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <28>  [<]
                    │   │       ├── <25>  [-]
                    │   │       │   ├── <22>  [-]
                    │   │       │   │   ├── <18> SizeOfExpr
                    │   │       │   │   │   ╰── <17> Constant Int [4]
                    │   │       │   │   ╰── <21> SizeOfExpr
                    │   │       │   │       ╰── <20> Constant Int [4]
                    │   │       │   ╰── <24> Constant Int [1]
                    │   │       ╰── <27> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <29> Constant Int [2]
                    ╰── Return
                        ╰── <34> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_void_cast_to_void() {
    let src = r#"
        int x;
        int set_x(int i) {
            x = i;
            return 0;
        }
        void do_nothing(void) {
            ;
        }
        int main(void) {
            (void) x;
            (void) set_x(12);
            (void) do_nothing();
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
            ├── Function [set_x]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── i
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ├── <17> Assign [=]
            │       │   ├── <13> Var [x]
            │       │   ╰── <16> Var [i]
            │       ╰── Return
            │           ╰── <19> Constant Int [0]
            ├── Function [do_nothing]
            │   ╰── Body
            │       ╰── Empty
            ╰── Function [main]
                ╰── Body
                    ├── <38> Cast
                    │   ├── Target
                    │   │   ╰── Void
                    │   ╰── Expression
                    │       ╰── <37> Var [x]
                    ├── <45> Cast
                    │   ├── Target
                    │   │   ╰── Void
                    │   ╰── Expression
                    │       ╰── <44> FunctionCall [set_x]
                    │           ╰── <43> Constant Int [12]
                    ├── <51> Cast
                    │   ├── Target
                    │   │   ╰── Void
                    │   ╰── Expression
                    │       ╰── <50> FunctionCall [do_nothing]
                    ╰── Return
                        ╰── <54> Var [x]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_void_ternary() {
    let src = r#"
        int i = 4;
        int j = 5;
        int flag_1 = 1;
        int flag_0 = 0;
        void incr_i(void) {
            i = i + 1;
        }
        void incr_j(void) {
            j = j + 1;
        }
        int main(void) {
            flag_1 ? incr_i() : incr_j();
            flag_0 ? incr_i() : incr_j();
            if (i != 5) {
                return 1;
            }
            if (j != 6) {
                return 2;
            }
            flag_0 ? incr_j() : flag_1 ? incr_i() : incr_j();
            if (i != 6) {
                return 3;
            }
            if (j != 6) {
                return 4;
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
            │       ╰── <4> Constant Int [4]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── j
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Initializer
            │       ╰── <10> Constant Int [5]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── flag_1
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Initializer
            │       ╰── <16> Constant Int [1]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── flag_0
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Initializer
            │       ╰── <22> Constant Int [0]
            ├── Function [incr_i]
            │   ╰── Body
            │       ╰── <37> Assign [=]
            │           ├── <30> Var [i]
            │           ╰── <36>  [+]
            │               ├── <33> Var [i]
            │               ╰── <35> Constant Int [1]
            ├── Function [incr_j]
            │   ╰── Body
            │       ╰── <53> Assign [=]
            │           ├── <46> Var [j]
            │           ╰── <52>  [+]
            │               ├── <49> Var [j]
            │               ╰── <51> Constant Int [1]
            ╰── Function [main]
                ╰── Body
                    ├── <67> Conditional [?]
                    │   ├── <62> Var [flag_1]
                    │   ├── Then
                    │   │   ╰── <64> FunctionCall [incr_i]
                    │   ╰── Else
                    │       ╰── <66> FunctionCall [incr_j]
                    ├── <75> Conditional [?]
                    │   ├── <70> Var [flag_0]
                    │   ├── Then
                    │   │   ╰── <72> FunctionCall [incr_i]
                    │   ╰── Else
                    │       ╰── <74> FunctionCall [incr_j]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <81>  [!=]
                    │   │       ├── <78> Var [i]
                    │   │       ╰── <80> Constant Int [5]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <82> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <91>  [!=]
                    │   │       ├── <88> Var [j]
                    │   │       ╰── <90> Constant Int [6]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <92> Constant Int [2]
                    ├── <108> Conditional [?]
                    │   ├── <98> Var [flag_0]
                    │   ├── Then
                    │   │   ╰── <100> FunctionCall [incr_j]
                    │   ╰── Else
                    │       ╰── <107> Conditional [?]
                    │           ├── <102> Var [flag_1]
                    │           ├── Then
                    │           │   ╰── <104> FunctionCall [incr_i]
                    │           ╰── Else
                    │               ╰── <106> FunctionCall [incr_j]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <114>  [!=]
                    │   │       ├── <111> Var [i]
                    │   │       ╰── <113> Constant Int [6]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <115> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <124>  [!=]
                    │   │       ├── <121> Var [j]
                    │   │       ╰── <123> Constant Int [6]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <125> Constant Int [4]
                    ╰── Return
                        ╰── <130> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_void_void_for_loop() {
    let src = r#"
        int putchar(int c);
        int letter;
        void initialize_letter(void) {
            letter = 'Z';
        }
        void decrement_letter(void) {
            letter = letter - 1;
        }
        int main(void) {
            for (initialize_letter(); letter >= 'A';
                 letter = letter - 1) {
                putchar(letter);
            }
            for (letter = 'A'; letter <= 90; (void)(letter = letter + 1)) {
                putchar(letter);
            }
            for (initialize_letter(); letter >= 65; decrement_letter()) {
                putchar(letter);
            }
            return 0;
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
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── letter
            │   ╰── Type
            │       ╰── Int
            ├── Function [initialize_letter]
            │   ╰── Body
            │       ╰── <21> Assign [=]
            │           ├── <18> Var [letter]
            │           ╰── <20> Constant Int [90]
            ├── Function [decrement_letter]
            │   ╰── Body
            │       ╰── <37> Assign [=]
            │           ├── <30> Var [letter]
            │           ╰── <36>  [-]
            │               ├── <33> Var [letter]
            │               ╰── <35> Constant Int [1]
            ╰── Function [main]
                ╰── Body
                    ├── For
                    │   ├── Init
                    │   │   ╰── <46> FunctionCall [initialize_letter]
                    │   ├── Condition
                    │   │   ╰── <51>  [>=]
                    │   │       ├── <48> Var [letter]
                    │   │       ╰── <50> Constant Int [65]
                    │   ├── Condition
                    │   │   ╰── <60> Assign [=]
                    │   │       ├── <53> Var [letter]
                    │   │       ╰── <59>  [-]
                    │   │           ├── <56> Var [letter]
                    │   │           ╰── <58> Constant Int [1]
                    │   ╰── Block
                    │       ╰── <64> FunctionCall [putchar]
                    │           ╰── <63> Var [letter]
                    ├── For
                    │   ├── Init
                    │   │   ╰── <73> Assign [=]
                    │   │       ├── <70> Var [letter]
                    │   │       ╰── <72> Constant Int [65]
                    │   ├── Condition
                    │   │   ╰── <78>  [<=]
                    │   │       ├── <75> Var [letter]
                    │   │       ╰── <77> Constant Int [90]
                    │   ├── Condition
                    │   │   ╰── <91> Cast
                    │   │       ├── Target
                    │   │       │   ╰── Void
                    │   │       ╰── Expression
                    │   │           ╰── <90> Assign [=]
                    │   │               ├── <82> Var [letter]
                    │   │               ╰── <88>  [+]
                    │   │                   ├── <85> Var [letter]
                    │   │                   ╰── <87> Constant Int [1]
                    │   ╰── Block
                    │       ╰── <95> FunctionCall [putchar]
                    │           ╰── <94> Var [letter]
                    ├── For
                    │   ├── Init
                    │   │   ╰── <101> FunctionCall [initialize_letter]
                    │   ├── Condition
                    │   │   ╰── <106>  [>=]
                    │   │       ├── <103> Var [letter]
                    │   │       ╰── <105> Constant Int [65]
                    │   ├── Condition
                    │   │   ╰── <108> FunctionCall [decrement_letter]
                    │   ╰── Block
                    │       ╰── <112> FunctionCall [putchar]
                    │           ╰── <111> Var [letter]
                    ╰── Return
                        ╰── <117> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_void_void_function() {
    let src = r#"
        
        int foo = 0;
        void set_foo_to_positive_num(int a) {
            if (a < 0) {
                return;
            }
            foo = a;
            return;
        }
        void do_nothing(void) {
        }
        int main(void) {
            set_foo_to_positive_num(-2);
            if (foo) {
                return 1;
            }
            set_foo_to_positive_num(12);
            if (foo != 12) {
                return 2;
            }
            do_nothing();
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
            │       ╰── <4> Constant Int [0]
            ├── Function [set_foo_to_positive_num]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── a
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <18>  [<]
            │       │   │       ├── <15> Var [a]
            │       │   │       ╰── <17> Constant Int [0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       ├── <28> Assign [=]
            │       │   ├── <24> Var [foo]
            │       │   ╰── <27> Var [a]
            │       ╰── Return
            ├── Function [do_nothing]
            │   ╰── Body
            ╰── Function [main]
                ╰── Body
                    ├── <47> FunctionCall [set_foo_to_positive_num]
                    │   ╰── <46> Unary [-]
                    │       ╰── <45> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <50> Var [foo]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <51> Constant Int [1]
                    ├── <58> FunctionCall [set_foo_to_positive_num]
                    │   ╰── <57> Constant Int [12]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <64>  [!=]
                    │   │       ├── <61> Var [foo]
                    │   │       ╰── <63> Constant Int [12]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <65> Constant Int [2]
                    ├── <71> FunctionCall [do_nothing]
                    ╰── Return
                        ╰── <73> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_void_pointer_array_of_pointers_to_void() {
    let src = r#"
        void *calloc(unsigned long nmemb, unsigned long size);
        void free(void *ptr);
        int main(void) {
            int i = 10;
            void *arr[4] = {
                calloc(2, sizeof(int)),
                &i,
                0,
                arr
            };
            long *l = arr[0];
            if (*l)
                return 1;
            int elem_1_val = *(int *)arr[1];
            if (elem_1_val != 10)
                return 2;
            if (arr[2])
                return 3;
            if (arr[3] != arr)
                return 4;
            free(arr[0]);
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [calloc]
            │   ╰── Parameters
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── nmemb
            │       │   ╰── Type
            │       │       ╰── Unsigned Long
            │       ╰── Param
            │           ├── Name
            │           │   ╰── size
            │           ╰── Type
            │               ╰── Unsigned Long
            ├── Function [free]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── ptr
            │           ╰── Type
            │               ╰── Pointer
            │                   ╰── Void
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <29> Constant Int [10]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── arr
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 4
                    │   │       ╰── Pointer
                    │   │           ╰── Void
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <43> FunctionCall [calloc]
                    │           │   ├── <39> Constant Int [2]
                    │           │   ╰── <42> SizeOfType
                    │           │       ╰── Int
                    │           ├── <47> AddressOf
                    │           │   ╰── <46> Var [i]
                    │           ├── <49> Constant Int [0]
                    │           ╰── <52> Var [arr]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Long
                    │   ╰── Initializer
                    │       ╰── <63> Subscript
                    │           ├── <61> Var [arr]
                    │           ╰── <62> Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <68> Dereference
                    │   │       ╰── <67> Var [l]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <69> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── elem_1_val
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <83> Dereference
                    │           ╰── <82> Cast
                    │               ├── Target
                    │               │   ╰── Pointer
                    │               │       ╰── Int
                    │               ╰── Expression
                    │                   ╰── <81> Subscript
                    │                       ├── <79> Var [arr]
                    │                       ╰── <80> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <90>  [!=]
                    │   │       ├── <87> Var [elem_1_val]
                    │   │       ╰── <89> Constant Int [10]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <91> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <97> Subscript
                    │   │       ├── <95> Var [arr]
                    │   │       ╰── <96> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <98> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <108>  [!=]
                    │   │       ├── <104> Subscript
                    │   │       │   ├── <102> Var [arr]
                    │   │       │   ╰── <103> Constant Int [3]
                    │   │       ╰── <107> Var [arr]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <109> Constant Int [4]
                    ├── <117> FunctionCall [free]
                    │   ╰── <116> Subscript
                    │       ├── <114> Var [arr]
                    │       ╰── <115> Constant Int [0]
                    ╰── Return
                        ╰── <119> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_void_pointer_common_pointer_type() {
    let src = r#"
        void *calloc(unsigned long nmemb, unsigned long size);
        void free(void *ptr);
        int main(void) {
            void *void_ptr = calloc(3, sizeof(unsigned int));
            unsigned int array[3] = {1, 2, 3};
            if (void_ptr == 0)
                return 1;
            if (void_ptr == array)
                return 2;
            if (!(void_ptr != array))
                return 3;
            static void *null_ptr = 0;
            int *my_array = null_ptr ? void_ptr : array;
            int array_element = my_array[1];
            if (array_element != 2) {
                return 4;
            }
            free(void_ptr);
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [calloc]
            │   ╰── Parameters
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── nmemb
            │       │   ╰── Type
            │       │       ╰── Unsigned Long
            │       ╰── Param
            │           ├── Name
            │           │   ╰── size
            │           ╰── Type
            │               ╰── Unsigned Long
            ├── Function [free]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── ptr
            │           ╰── Type
            │               ╰── Pointer
            │                   ╰── Void
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── void_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Void
                    │   ╰── Initializer
                    │       ╰── <35> FunctionCall [calloc]
                    │           ├── <31> Constant Int [3]
                    │           ╰── <34> SizeOfType
                    │               ╰── Unsigned Int
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── array
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 3
                    │   │       ╰── Unsigned Int
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <43> Constant Int [1]
                    │           ├── <45> Constant Int [2]
                    │           ╰── <47> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <55>  [==]
                    │   │       ├── <52> Var [void_ptr]
                    │   │       ╰── <54> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <56> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <64>  [==]
                    │   │       ├── <60> Var [void_ptr]
                    │   │       ╰── <63> Var [array]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <65> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <76> Unary [!]
                    │   │       ╰── <75>  [!=]
                    │   │           ├── <70> Var [void_ptr]
                    │   │           ╰── <73> Var [array]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <77> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── null_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Void
                    │   ├── Initializer
                    │   │   ╰── <85> Constant Int [0]
                    │   ╰── Static
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── my_array
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <98> Conditional [?]
                    │           ├── <93> Var [null_ptr]
                    │           ├── Then
                    │           │   ╰── <95> Var [void_ptr]
                    │           ╰── Else
                    │               ╰── <97> Var [array]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── array_element
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <107> Subscript
                    │           ├── <105> Var [my_array]
                    │           ╰── <106> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <114>  [!=]
                    │   │       ├── <111> Var [array_element]
                    │   │       ╰── <113> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <115> Constant Int [4]
                    ├── <123> FunctionCall [free]
                    │   ╰── <122> Var [void_ptr]
                    ╰── Return
                        ╰── <125> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_void_pointer_conversion_by_assignment() {
    let src = r#"
        void *malloc(unsigned long size);
        void free(void *ptr);
        int memcmp(void *s1, void *s2, unsigned long n);
        void *return_ptr(char *i) {
            return i + 3;
        }
        int check_char_ptr_argument(char *pointer, char expected_val) {
            return *pointer == expected_val;
        }
        int *return_void_ptr_as_int_ptr(void *pointer) {
            return pointer;
        }
        double *get_dbl_array(unsigned long n) {
            return (double *) malloc(n * sizeof (double));
        }
        void set_doubles(double *array, unsigned long n, double d) {
            for (unsigned long i = 0; i < n; i = i + 1) {
                array[i] = d;
            }
            return;
        }
        void *return_dbl_ptr_as_void_ptr(double *ptr) {
            return ptr;
        }
        int main(void) {
            void *four_bytes = malloc(4);
            int *int_ptr = four_bytes;
            *int_ptr = -1;
            if (!check_char_ptr_argument(four_bytes, -1)) {
                return 1;
            }
            if (return_void_ptr_as_int_ptr(four_bytes) != int_ptr) {
                return 2;
            }
            double *dbl_ptr = four_bytes;
            int (*complicated_ptr)[3][2][5] = four_bytes;
            long *long_ptr = four_bytes;
            if (dbl_ptr != four_bytes || complicated_ptr != four_bytes || long_ptr != four_bytes) {
                return 3;
            }
            free(four_bytes);
            double *dbl_array = get_dbl_array(5);
            void *void_array = dbl_array;
            set_doubles(void_array, 5, 4.0);
            if (dbl_array[3] != 4.0) {
                return 4;
            }
            if (return_dbl_ptr_as_void_ptr(dbl_array) != void_array) {
                return 5;
            }
            void *some_other_ptr = 0;
            some_other_ptr = dbl_array;
            if (some_other_ptr != void_array) {
                return 6;
            }
            some_other_ptr = &some_other_ptr;
            if (some_other_ptr == void_array) {
                return 7;
            }
            complicated_ptr = 0;
            some_other_ptr = complicated_ptr;
            if (some_other_ptr) {
                return 8;
            }
            free(dbl_array);
            long *long_ptr_array[3] = {
                malloc(sizeof(long)), malloc(sizeof(long)), malloc(sizeof(long))
            };
            *long_ptr_array[0] = 100l;
            *long_ptr_array[1] = 200l;
            *long_ptr_array[2] = 300l;
            long sum = (*long_ptr_array[0] + *long_ptr_array[1] + *long_ptr_array[2]);
            if (sum != 600l) {
                return 9;
            }
            free(long_ptr_array[0]);
            free(long_ptr_array[1]);
            free(long_ptr_array[2]);
            long arr1[3] = {1, 2, 3};
            long arr2[3] = {1, 2, 3};
            long arr3[3] = {1, 2, 4};
            if (memcmp(arr1, arr2, sizeof arr1) != 0) {
                return 10;
            }
            if (memcmp(arr1, arr3, sizeof arr2) != -1) {
                return 11;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [malloc]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── size
            │           ╰── Type
            │               ╰── Unsigned Long
            ├── Function [free]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── ptr
            │           ╰── Type
            │               ╰── Pointer
            │                   ╰── Void
            ├── Function [memcmp]
            │   ╰── Parameters
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── s1
            │       │   ╰── Type
            │       │       ╰── Pointer
            │       │           ╰── Void
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── s2
            │       │   ╰── Type
            │       │       ╰── Pointer
            │       │           ╰── Void
            │       ╰── Param
            │           ├── Name
            │           │   ╰── n
            │           ╰── Type
            │               ╰── Unsigned Long
            ├── Function [return_ptr]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── i
            │   │       ╰── Type
            │   │           ╰── Pointer
            │   │               ╰── Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <48>  [+]
            │               ├── <45> Var [i]
            │               ╰── <47> Constant Int [3]
            ├── Function [check_char_ptr_argument]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── pointer
            │   │   │   ╰── Type
            │   │   │       ╰── Pointer
            │   │   │           ╰── Char
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── expected_val
            │   │       ╰── Type
            │   │           ╰── Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <69>  [==]
            │               ├── <65> Dereference
            │               │   ╰── <64> Var [pointer]
            │               ╰── <68> Var [expected_val]
            ├── Function [return_void_ptr_as_int_ptr]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── pointer
            │   │       ╰── Type
            │   │           ╰── Pointer
            │   │               ╰── Void
            │   ╰── Body
            │       ╰── Return
            │           ╰── <83> Var [pointer]
            ├── Function [get_dbl_array]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── n
            │   │       ╰── Type
            │   │           ╰── Unsigned Long
            │   ╰── Body
            │       ╰── Return
            │           ╰── <107> Cast
            │               ├── Target
            │               │   ╰── Pointer
            │               │       ╰── Double
            │               ╰── Expression
            │                   ╰── <106> FunctionCall [malloc]
            │                       ╰── <105>  [*]
            │                           ├── <100> Var [n]
            │                           ╰── <104> SizeOfType
            │                               ╰── Double
            ├── Function [set_doubles]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── array
            │   │   │   ╰── Type
            │   │   │       ╰── Pointer
            │   │   │           ╰── Double
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── n
            │   │   │   ╰── Type
            │   │   │       ╰── Unsigned Long
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── d
            │   │       ╰── Type
            │   │           ╰── Double
            │   ╰── Body
            │       ├── For
            │       │   ├── Init
            │       │   │   ╰── VarDeclaration
            │       │   │       ├── Name
            │       │   │       │   ╰── i
            │       │   │       ├── Type
            │       │   │       │   ╰── Unsigned Long
            │       │   │       ╰── Initializer
            │       │   │           ╰── <128> Constant Int [0]
            │       │   ├── Condition
            │       │   │   ╰── <137>  [<]
            │       │   │       ├── <133> Var [i]
            │       │   │       ╰── <136> Var [n]
            │       │   ├── Condition
            │       │   │   ╰── <146> Assign [=]
            │       │   │       ├── <139> Var [i]
            │       │   │       ╰── <145>  [+]
            │       │   │           ├── <142> Var [i]
            │       │   │           ╰── <144> Constant Int [1]
            │       │   ╰── Block
            │       │       ╰── <155> Assign [=]
            │       │           ├── <151> Subscript
            │       │           │   ├── <148> Var [array]
            │       │           │   ╰── <150> Var [i]
            │       │           ╰── <154> Var [d]
            │       ╰── Return
            ├── Function [return_dbl_ptr_as_void_ptr]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── ptr
            │   │       ╰── Type
            │   │           ╰── Pointer
            │   │               ╰── Double
            │   ╰── Body
            │       ╰── Return
            │           ╰── <173> Var [ptr]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── four_bytes
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Void
                    │   ╰── Initializer
                    │       ╰── <187> FunctionCall [malloc]
                    │           ╰── <186> Constant Int [4]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── int_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <195> Var [four_bytes]
                    ├── <205> Assign [=]
                    │   ├── <200> Dereference
                    │   │   ╰── <199> Var [int_ptr]
                    │   ╰── <204> Unary [-]
                    │       ╰── <203> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <215> Unary [!]
                    │   │       ╰── <214> FunctionCall [check_char_ptr_argument]
                    │   │           ├── <210> Var [four_bytes]
                    │   │           ╰── <213> Unary [-]
                    │   │               ╰── <212> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <216> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <228>  [!=]
                    │   │       ├── <224> FunctionCall [return_void_ptr_as_int_ptr]
                    │   │       │   ╰── <223> Var [four_bytes]
                    │   │       ╰── <227> Var [int_ptr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <229> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── dbl_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Double
                    │   ╰── Initializer
                    │       ╰── <239> Var [four_bytes]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── complicated_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Array
                    │   │           ├── 3
                    │   │           ╰── Array
                    │   │               ├── 2
                    │   │               ╰── Array
                    │   │                   ├── 5
                    │   │                   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <254> Var [four_bytes]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── long_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Long
                    │   ╰── Initializer
                    │       ╰── <262> Var [four_bytes]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <286>  [||]
                    │   │       ├── <278>  [||]
                    │   │       │   ├── <270>  [!=]
                    │   │       │   │   ├── <266> Var [dbl_ptr]
                    │   │       │   │   ╰── <269> Var [four_bytes]
                    │   │       │   ╰── <277>  [!=]
                    │   │       │       ├── <273> Var [complicated_ptr]
                    │   │       │       ╰── <276> Var [four_bytes]
                    │   │       ╰── <285>  [!=]
                    │   │           ├── <281> Var [long_ptr]
                    │   │           ╰── <284> Var [four_bytes]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <287> Constant Int [3]
                    ├── <295> FunctionCall [free]
                    │   ╰── <294> Var [four_bytes]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── dbl_array
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Double
                    │   ╰── Initializer
                    │       ╰── <303> FunctionCall [get_dbl_array]
                    │           ╰── <302> Constant Int [5]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── void_array
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Void
                    │   ╰── Initializer
                    │       ╰── <311> Var [dbl_array]
                    ├── <319> FunctionCall [set_doubles]
                    │   ├── <316> Var [void_array]
                    │   ├── <317> Constant Int [5]
                    │   ╰── <318> Constant Double [+4e0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <327>  [!=]
                    │   │       ├── <324> Subscript
                    │   │       │   ├── <322> Var [dbl_array]
                    │   │       │   ╰── <323> Constant Int [3]
                    │   │       ╰── <326> Constant Double [+4e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <328> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <340>  [!=]
                    │   │       ├── <336> FunctionCall [return_dbl_ptr_as_void_ptr]
                    │   │       │   ╰── <335> Var [dbl_array]
                    │   │       ╰── <339> Var [void_array]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <341> Constant Int [5]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── some_other_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Void
                    │   ╰── Initializer
                    │       ╰── <350> Constant Int [0]
                    ├── <358> Assign [=]
                    │   ├── <354> Var [some_other_ptr]
                    │   ╰── <357> Var [dbl_array]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <365>  [!=]
                    │   │       ├── <361> Var [some_other_ptr]
                    │   │       ╰── <364> Var [void_array]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <366> Constant Int [6]
                    ├── <377> Assign [=]
                    │   ├── <372> Var [some_other_ptr]
                    │   ╰── <376> AddressOf
                    │       ╰── <375> Var [some_other_ptr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <384>  [==]
                    │   │       ├── <380> Var [some_other_ptr]
                    │   │       ╰── <383> Var [void_array]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <385> Constant Int [7]
                    ├── <394> Assign [=]
                    │   ├── <391> Var [complicated_ptr]
                    │   ╰── <393> Constant Int [0]
                    ├── <401> Assign [=]
                    │   ├── <397> Var [some_other_ptr]
                    │   ╰── <400> Var [complicated_ptr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <404> Var [some_other_ptr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <405> Constant Int [8]
                    ├── <413> FunctionCall [free]
                    │   ╰── <412> Var [dbl_array]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── long_ptr_array
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 3
                    │   │       ╰── Pointer
                    │   │           ╰── Long
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <425> FunctionCall [malloc]
                    │           │   ╰── <424> SizeOfType
                    │           │       ╰── Long
                    │           ├── <431> FunctionCall [malloc]
                    │           │   ╰── <430> SizeOfType
                    │           │       ╰── Long
                    │           ╰── <437> FunctionCall [malloc]
                    │               ╰── <436> SizeOfType
                    │                   ╰── Long
                    ├── <448> Assign [=]
                    │   ├── <445> Dereference
                    │   │   ╰── <444> Subscript
                    │   │       ├── <442> Var [long_ptr_array]
                    │   │       ╰── <443> Constant Int [0]
                    │   ╰── <447> Constant Long [100]
                    ├── <457> Assign [=]
                    │   ├── <454> Dereference
                    │   │   ╰── <453> Subscript
                    │   │       ├── <451> Var [long_ptr_array]
                    │   │       ╰── <452> Constant Int [1]
                    │   ╰── <456> Constant Long [200]
                    ├── <466> Assign [=]
                    │   ├── <463> Dereference
                    │   │   ╰── <462> Subscript
                    │   │       ├── <460> Var [long_ptr_array]
                    │   │       ╰── <461> Constant Int [2]
                    │   ╰── <465> Constant Long [300]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── sum
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <490>  [+]
                    │           ├── <482>  [+]
                    │           │   ├── <475> Dereference
                    │           │   │   ╰── <474> Subscript
                    │           │   │       ├── <472> Var [long_ptr_array]
                    │           │   │       ╰── <473> Constant Int [0]
                    │           │   ╰── <481> Dereference
                    │           │       ╰── <480> Subscript
                    │           │           ├── <478> Var [long_ptr_array]
                    │           │           ╰── <479> Constant Int [1]
                    │           ╰── <488> Dereference
                    │               ╰── <487> Subscript
                    │                   ├── <485> Var [long_ptr_array]
                    │                   ╰── <486> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <497>  [!=]
                    │   │       ├── <494> Var [sum]
                    │   │       ╰── <496> Constant Long [600]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <498> Constant Int [9]
                    ├── <508> FunctionCall [free]
                    │   ╰── <507> Subscript
                    │       ├── <505> Var [long_ptr_array]
                    │       ╰── <506> Constant Int [0]
                    ├── <515> FunctionCall [free]
                    │   ╰── <514> Subscript
                    │       ├── <512> Var [long_ptr_array]
                    │       ╰── <513> Constant Int [1]
                    ├── <522> FunctionCall [free]
                    │   ╰── <521> Subscript
                    │       ├── <519> Var [long_ptr_array]
                    │       ╰── <520> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── arr1
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 3
                    │   │       ╰── Long
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <529> Constant Int [1]
                    │           ├── <531> Constant Int [2]
                    │           ╰── <533> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── arr2
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 3
                    │   │       ╰── Long
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <542> Constant Int [1]
                    │           ├── <544> Constant Int [2]
                    │           ╰── <546> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── arr3
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 3
                    │   │       ╰── Long
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <555> Constant Int [1]
                    │           ├── <557> Constant Int [2]
                    │           ╰── <559> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <574>  [!=]
                    │   │       ├── <571> FunctionCall [memcmp]
                    │   │       │   ├── <565> Var [arr1]
                    │   │       │   ├── <567> Var [arr2]
                    │   │       │   ╰── <570> SizeOfExpr
                    │   │       │       ╰── <569> Var [arr1]
                    │   │       ╰── <573> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <575> Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <593>  [!=]
                    │   │       ├── <588> FunctionCall [memcmp]
                    │   │       │   ├── <582> Var [arr1]
                    │   │       │   ├── <584> Var [arr3]
                    │   │       │   ╰── <587> SizeOfExpr
                    │   │       │       ╰── <586> Var [arr2]
                    │   │       ╰── <592> Unary [-]
                    │   │           ╰── <591> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <594> Constant Int [11]
                    ╰── Return
                        ╰── <599> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_void_pointer_explicit_cast() {
    let src = r#"
        void *malloc(unsigned long size);
        void free(void *ptr);
        void *memcpy(void *s1, void *s2, unsigned long n);
        int main(void) {
            void *ptr = malloc(4 * sizeof(double));
            double *double_ptr = (double *)ptr;
            double_ptr[2] = 10.0;
            if ((void *)double_ptr != ptr) {
                return 1;
            }
            double result = double_ptr[2];
            if (result != 10.0) {
                return 2;
            }
            unsigned long ul = (unsigned long)ptr;
            if (ul % 8) {
                return 3;
            }
            free(ptr);
            long zero = 0;
            ptr = (void *) zero;
            if (ptr) {
                return 4;
            }
            zero = (long) ptr;
            if (zero) {
                return 5;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [malloc]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── size
            │           ╰── Type
            │               ╰── Unsigned Long
            ├── Function [free]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── ptr
            │           ╰── Type
            │               ╰── Pointer
            │                   ╰── Void
            ├── Function [memcpy]
            │   ╰── Parameters
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── s1
            │       │   ╰── Type
            │       │       ╰── Pointer
            │       │           ╰── Void
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── s2
            │       │   ╰── Type
            │       │       ╰── Pointer
            │       │           ╰── Void
            │       ╰── Param
            │           ├── Name
            │           │   ╰── n
            │           ╰── Type
            │               ╰── Unsigned Long
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Void
                    │   ╰── Initializer
                    │       ╰── <51> FunctionCall [malloc]
                    │           ╰── <50>  [*]
                    │               ├── <45> Constant Int [4]
                    │               ╰── <49> SizeOfType
                    │                   ╰── Double
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── double_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Double
                    │   ╰── Initializer
                    │       ╰── <63> Cast
                    │           ├── Target
                    │           │   ╰── Pointer
                    │           │       ╰── Double
                    │           ╰── Expression
                    │               ╰── <62> Var [ptr]
                    ├── <72> Assign [=]
                    │   ├── <69> Subscript
                    │   │   ├── <67> Var [double_ptr]
                    │   │   ╰── <68> Constant Int [2]
                    │   ╰── <71> Constant Double [+1e1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <83>  [!=]
                    │   │       ├── <79> Cast
                    │   │       │   ├── Target
                    │   │       │   │   ╰── Pointer
                    │   │       │   │       ╰── Void
                    │   │       │   ╰── Expression
                    │   │       │       ╰── <78> Var [double_ptr]
                    │   │       ╰── <82> Var [ptr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <84> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── result
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <95> Subscript
                    │           ├── <93> Var [double_ptr]
                    │           ╰── <94> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <102>  [!=]
                    │   │       ├── <99> Var [result]
                    │   │       ╰── <101> Constant Double [+1e1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <103> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ul
                    │   ├── Type
                    │   │   ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <115> Cast
                    │           ├── Target
                    │           │   ╰── Unsigned Long
                    │           ╰── Expression
                    │               ╰── <114> Var [ptr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <122>  [%]
                    │   │       ├── <119> Var [ul]
                    │   │       ╰── <121> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <123> Constant Int [3]
                    ├── <131> FunctionCall [free]
                    │   ╰── <130> Var [ptr]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── zero
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <136> Constant Int [0]
                    ├── <148> Assign [=]
                    │   ├── <140> Var [ptr]
                    │   ╰── <147> Cast
                    │       ├── Target
                    │       │   ╰── Pointer
                    │       │       ╰── Void
                    │       ╰── Expression
                    │           ╰── <146> Var [zero]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <151> Var [ptr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <152> Constant Int [4]
                    ├── <165> Assign [=]
                    │   ├── <158> Var [zero]
                    │   ╰── <164> Cast
                    │       ├── Target
                    │       │   ╰── Long
                    │       ╰── Expression
                    │           ╰── <163> Var [ptr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <168> Var [zero]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <169> Constant Int [5]
                    ╰── Return
                        ╰── <174> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_void_pointer_memory_management_functions() {
    let src = r#"
        void *malloc(unsigned long size);
        void *realloc(void *ptr, unsigned long size);
        void *calloc(unsigned long nmemb, unsigned long size);
        void *aligned_alloc(unsigned long alignment, unsigned long size);
        void free(void *ptr);
        int main(void) {
            char *char_buffer = malloc(50);
            for (int i = 0; i < 50; i = i + 1) {
                char_buffer[i] = i;
            }
            char *char_buffer2 = realloc(char_buffer, 100);
            char_buffer2[75] = 11;
            for (int i = 0; i < 50; i = i + 1) {
                if ( char_buffer2[i] != i) {
                    return 1;
                }
            }
            if (char_buffer2[75] != 11) {
                return 2;
            }
            free(char_buffer2);
            double *double_buffer = calloc(10, sizeof(double));
            for (int i = 0; i < 10; i = i + 1) {
                if (double_buffer[i]) {
                    return 3;
                }
            }
            free(double_buffer);
            char_buffer = aligned_alloc(256, 256);
            if ((unsigned long) char_buffer % 256) {
                return 4;
            }
            free(char_buffer);
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [malloc]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── size
            │           ╰── Type
            │               ╰── Unsigned Long
            ├── Function [realloc]
            │   ╰── Parameters
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── ptr
            │       │   ╰── Type
            │       │       ╰── Pointer
            │       │           ╰── Void
            │       ╰── Param
            │           ├── Name
            │           │   ╰── size
            │           ╰── Type
            │               ╰── Unsigned Long
            ├── Function [calloc]
            │   ╰── Parameters
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── nmemb
            │       │   ╰── Type
            │       │       ╰── Unsigned Long
            │       ╰── Param
            │           ├── Name
            │           │   ╰── size
            │           ╰── Type
            │               ╰── Unsigned Long
            ├── Function [aligned_alloc]
            │   ╰── Parameters
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── alignment
            │       │   ╰── Type
            │       │       ╰── Unsigned Long
            │       ╰── Param
            │           ├── Name
            │           │   ╰── size
            │           ╰── Type
            │               ╰── Unsigned Long
            ├── Function [free]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── ptr
            │           ╰── Type
            │               ╰── Pointer
            │                   ╰── Void
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── char_buffer
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── <66> FunctionCall [malloc]
                    │           ╰── <65> Constant Int [50]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <72> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <80>  [<]
                    │   │       ├── <77> Var [i]
                    │   │       ╰── <79> Constant Int [50]
                    │   ├── Condition
                    │   │   ╰── <89> Assign [=]
                    │   │       ├── <82> Var [i]
                    │   │       ╰── <88>  [+]
                    │   │           ├── <85> Var [i]
                    │   │           ╰── <87> Constant Int [1]
                    │   ╰── Block
                    │       ╰── <98> Assign [=]
                    │           ├── <94> Subscript
                    │           │   ├── <91> Var [char_buffer]
                    │           │   ╰── <93> Var [i]
                    │           ╰── <97> Var [i]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── char_buffer2
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── <111> FunctionCall [realloc]
                    │           ├── <109> Var [char_buffer]
                    │           ╰── <110> Constant Int [100]
                    ├── <120> Assign [=]
                    │   ├── <117> Subscript
                    │   │   ├── <115> Var [char_buffer2]
                    │   │   ╰── <116> Constant Int [75]
                    │   ╰── <119> Constant Int [11]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <125> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <133>  [<]
                    │   │       ├── <130> Var [i]
                    │   │       ╰── <132> Constant Int [50]
                    │   ├── Condition
                    │   │   ╰── <142> Assign [=]
                    │   │       ├── <135> Var [i]
                    │   │       ╰── <141>  [+]
                    │   │           ├── <138> Var [i]
                    │   │           ╰── <140> Constant Int [1]
                    │   ╰── Block
                    │       ╰── If
                    │           ├── Condition
                    │           │   ╰── <151>  [!=]
                    │           │       ├── <147> Subscript
                    │           │       │   ├── <144> Var [char_buffer2]
                    │           │       │   ╰── <146> Var [i]
                    │           │       ╰── <150> Var [i]
                    │           ╰── Then
                    │               ╰── Block
                    │                   ╰── Return
                    │                       ╰── <152> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <166>  [!=]
                    │   │       ├── <163> Subscript
                    │   │       │   ├── <161> Var [char_buffer2]
                    │   │       │   ╰── <162> Constant Int [75]
                    │   │       ╰── <165> Constant Int [11]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <167> Constant Int [2]
                    ├── <175> FunctionCall [free]
                    │   ╰── <174> Var [char_buffer2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── double_buffer
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Double
                    │   ╰── Initializer
                    │       ╰── <186> FunctionCall [calloc]
                    │           ├── <182> Constant Int [10]
                    │           ╰── <185> SizeOfType
                    │               ╰── Double
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <192> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <200>  [<]
                    │   │       ├── <197> Var [i]
                    │   │       ╰── <199> Constant Int [10]
                    │   ├── Condition
                    │   │   ╰── <209> Assign [=]
                    │   │       ├── <202> Var [i]
                    │   │       ╰── <208>  [+]
                    │   │           ├── <205> Var [i]
                    │   │           ╰── <207> Constant Int [1]
                    │   ╰── Block
                    │       ╰── If
                    │           ├── Condition
                    │           │   ╰── <214> Subscript
                    │           │       ├── <211> Var [double_buffer]
                    │           │       ╰── <213> Var [i]
                    │           ╰── Then
                    │               ╰── Block
                    │                   ╰── Return
                    │                       ╰── <215> Constant Int [3]
                    ├── <226> FunctionCall [free]
                    │   ╰── <225> Var [double_buffer]
                    ├── <235> Assign [=]
                    │   ├── <229> Var [char_buffer]
                    │   ╰── <234> FunctionCall [aligned_alloc]
                    │       ├── <232> Constant Int [256]
                    │       ╰── <233> Constant Int [256]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <244>  [%]
                    │   │       ├── <241> Cast
                    │   │       │   ├── Target
                    │   │       │   │   ╰── Unsigned Long
                    │   │       │   ╰── Expression
                    │   │       │       ╰── <240> Var [char_buffer]
                    │   │       ╰── <243> Constant Int [256]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <245> Constant Int [4]
                    ├── <253> FunctionCall [free]
                    │   ╰── <252> Var [char_buffer]
                    ╰── Return
                        ╰── <255> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_void_pointer_simple() {
    let src = r#"
        void *malloc(unsigned long size);
        void free(void *ptr);
        int main(void) {
            int *array = malloc(10 * sizeof (int));
            array[2] = 100;
            int result = array[2];
            free(array);
            return result;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [malloc]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── size
            │           ╰── Type
            │               ╰── Unsigned Long
            ├── Function [free]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── ptr
            │           ╰── Type
            │               ╰── Pointer
            │                   ╰── Void
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── array
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <34> FunctionCall [malloc]
                    │           ╰── <33>  [*]
                    │               ├── <28> Constant Int [10]
                    │               ╰── <32> SizeOfType
                    │                   ╰── Int
                    ├── <43> Assign [=]
                    │   ├── <40> Subscript
                    │   │   ├── <38> Var [array]
                    │   │   ╰── <39> Constant Int [2]
                    │   ╰── <42> Constant Int [100]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── result
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <51> Subscript
                    │           ├── <49> Var [array]
                    │           ╰── <50> Constant Int [2]
                    ├── <57> FunctionCall [free]
                    │   ╰── <56> Var [array]
                    ╰── Return
                        ╰── <60> Var [result]
    "#;
    assert_parse(src, expected);
}
