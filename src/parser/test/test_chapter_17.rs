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
                    │       ╰── <17> Constant Int [10]
                    ├── <25>  [<<]
                    │   ├── <21> Var [x]
                    │   ╰── <24> FunctionCall [f]
                    ╰── Return
                        ╰── <27> Constant Int [0]
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
                    │       ╰── <9> Constant Int [10]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── y
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [11]
                    ├── <26>  [&]
                    │   ├── <19> Var [x]
                    │   ╰── <25> Cast
                    │       ├── Target
                    │       │   ╰── Void
                    │       ╰── Expression
                    │           ╰── <24> Var [y]
                    ╰── Return
                        ╰── <28> Constant Int [0]
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
                    │       ╰── <24> FunctionCall [malloc]
                    │           ╰── <23> Constant Int [100]
                    ├── <31> Assign [+=]
                    │   ├── <28> Var [buff]
                    │   ╰── <30> Constant Int [3]
                    ╰── Return
                        ╰── <33> Constant Int [0]
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
                    │       ╰── <24> FunctionCall [malloc]
                    │           ╰── <23> Constant Int [100]
                    ├── <31> Assign [-=]
                    │   ├── <28> Var [buff]
                    │   ╰── <30> Constant Int [0]
                    ╰── Return
                        ╰── <33> Constant Int [0]
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
                    │       ╰── <17> Constant Int [10]
                    ├── <25> Assign [*=]
                    │   ├── <21> Var [x]
                    │   ╰── <24> FunctionCall [f]
                    ╰── Return
                        ╰── <27> Constant Int [0]
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
                    │       ╰── <19> Constant Int [0]
                    ├── <27> Assign [+=]
                    │   ├── <23> Var [x]
                    │   ╰── <26> FunctionCall [f]
                    ╰── Return
                        ╰── <29> Constant Int [0]
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
                    │       ╰── <17> Constant Int [10]
                    ├── <25> Assign [>>=]
                    │   ├── <21> Var [x]
                    │   ╰── <24> FunctionCall [f]
                    ╰── Return
                        ╰── <27> Constant Int [0]
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
                    ├── <20> Unary [++]
                    │   ╰── <19> Postfix [--]
                    │       ╰── <17> Dereference
                    │           ╰── <15> Var [x]
                    ╰── Return
                        ╰── <22> Constant Int [0]
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
                    │       ╰── <24> FunctionCall [malloc]
                    │           ╰── <23> Constant Int [100]
                    ├── <30> Postfix [--]
                    │   ╰── <28> Var [buff]
                    ╰── Return
                        ╰── <32> Constant Int [0]
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
                    │       ╰── <24> FunctionCall [malloc]
                    │           ╰── <23> Constant Int [100]
                    ├── <30> Postfix [++]
                    │   ╰── <28> Var [buff]
                    ╰── Return
                        ╰── <32> Constant Int [0]
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
                    │       ╰── <24> FunctionCall [malloc]
                    │           ╰── <23> Constant Int [100]
                    ├── <30> Unary [--]
                    │   ╰── <29> Var [buff]
                    ╰── Return
                        ╰── <32> Constant Int [0]
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
                    ├── <18> Unary [++]
                    │   ╰── <17> Dereference
                    │       ╰── <15> Var [x]
                    ╰── Return
                        ╰── <20> Constant Int [0]
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
                    │       ╰── <24> FunctionCall [malloc]
                    │           ╰── <23> Constant Int [100]
                    ├── <30> Unary [++]
                    │   ╰── <29> Var [buff]
                    ╰── Return
                        ╰── <32> Constant Int [0]
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
                        │   ╰── <15> FunctionCall [f]
                        ╰── Block
                            ╰── Default
                                ╰── Return
                                    ╰── <16> Constant Int [0]
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
                    │       ╰── <24> FunctionCall [malloc]
                    │           ╰── <23> Constant Int [100]
                    ├── <35> Assign [=]
                    │   ├── <28> Var [x]
                    │   ╰── <34>  [+]
                    │       ├── <31> Var [x]
                    │       ╰── <33> Constant Int [1]
                    ╰── Return
                        ╰── <37> Constant Int [0]
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
            │           ╰── <6> Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <17> SizeOfExpr
                            ╰── <16> Var [x]
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
                        ╰── <8> SizeOfType
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
                        ╰── <11> SizeOfType
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
                        ╰── <16> SizeOfExpr
                            ╰── <15> Cast
                                ├── Target
                                │   ╰── Void
                                ╰── Expression
                                    ╰── <13> Var [x]
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
                    │       ╰── <17> AddressOf
                    │           ╰── <16> Var [y]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── null
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Void
                    │   ╰── Initializer
                    │       ╰── <25> Constant Int [0]
                    ╰── Return
                        ╰── <33>  [-]
                            ├── <29> Var [x]
                            ╰── <32> Var [null]
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
                    │       ╰── <9> Constant Int [10]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── v
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Void
                    │   ╰── Initializer
                    │       ╰── <19> AddressOf
                    │           ╰── <18> Var [x]
                    ├── <25> Subscript
                    │   ├── <23> Var [v]
                    │   ╰── <24> Constant Int [0]
                    ╰── Return
                        ╰── <27> Constant Int [0]
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
                    │           ├── <12> Constant Int [1]
                    │           ├── <14> Constant Int [2]
                    │           ╰── <16> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── void_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Void
                    │   ╰── Initializer
                    │       ╰── <26> Var [arr]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── int_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <38>  [+]
                    │           ├── <35> Var [arr]
                    │           ╰── <37> Constant Int [1]
                    ╰── Return
                        ╰── <49> Subscript
                            ├── <47> Conditional [?]
                            │   ├── <41> Constant Int [1]
                            │   ├── Then
                            │   │   ╰── <43> Var [int_ptr]
                            │   ╰── Else
                            │       ╰── <45> Var [void_ptr]
                            ╰── <48> Constant Int [1]
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
                        ╰── <13> Constant Int [0]
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
                    ├── <13> Cast
                    │   ├── Target
                    │   │   ╰── Pointer
                    │   │       ╰── Array
                    │   │           ├── 3
                    │   │           ╰── Void
                    │   ╰── Expression
                    │       ╰── <12> Constant Int [4]
                    ╰── Return
                        ╰── <15> Constant Int [0]
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
            │           ╰── <12> Constant Int [3]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <21> Constant Int [0]
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
                        ╰── <23> Var [ptr]
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
                    │       ╰── <28> FunctionCall [malloc]
                    │           ╰── <27> Constant Int [3]
                    ╰── Return
                        ╰── <35>  [==]
                            ├── <32> Var [ptr]
                            ╰── <34> Constant Int [0]
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
            │           ╰── <19>  [==]
            │               ├── <16> Var [bad_array]
            │               ╰── <18> Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <28> Constant Int [0]
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
                        ╰── <13>  [==]
                            ├── <10> Cast
                            │   ├── Target
                            │   │   ╰── Pointer
                            │   │       ╰── Void
                            │   ╰── Expression
                            │       ╰── <9> Constant Int [0]
                            ╰── <12> Constant ULong [20]
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
                    │           ├── <12> Constant Int [1]
                    │           ├── <14> Constant Int [2]
                    │           ╰── <16> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Void
                    │   ╰── Initializer
                    │       ╰── <30> Cast
                    │           ├── Target
                    │           │   ╰── Pointer
                    │           │       ╰── Void
                    │           ╰── Expression
                    │               ╰── <29> Var [arr]
                    ╰── Return
                        ╰── <41>  [<]
                            ├── <34> Var [ptr]
                            ╰── <40>  [+]
                                ├── <37> Var [arr]
                                ╰── <39> Constant Int [1]
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
                    │       ╰── <9> Constant Int [0]
                    ╰── VarDeclaration
                        ├── Name
                        │   ╰── v
                        ├── Type
                        │   ╰── Pointer
                        │       ╰── Void
                        ╰── Initializer
                            ╰── <18> Var [x]
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
                    │       ╰── <11> Constant Int [0]
                    ╰── Return
                        ╰── <15> Var [x]
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
                            ╰── <16>  [*]
                                ├── <9> Constant Int [10]
                                ╰── <15> Cast
                                    ├── Target
                                    │   ╰── Pointer
                                    │       ╰── Void
                                    ╰── Expression
                                        ╰── <14> Constant Int [0]
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
                        ╰── <12>  [&&]
                            ├── <9> Cast
                            │   ├── Target
                            │   │   ╰── Void
                            │   ╰── Expression
                            │       ╰── <8> Constant Int [1]
                            ╰── <11> Constant Int [2]
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
                    │       ╰── <15> Cast
                    │           ├── Target
                    │           │   ╰── Int
                    │           ╰── Expression
                    │               ╰── <14> Cast
                    │                   ├── Target
                    │                   │   ╰── Void
                    │                   ╰── Expression
                    │                       ╰── <13> Constant Int [3]
                    ╰── Return
                        ╰── <19> Var [y]
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
                        ╰── <26> Unary [!]
                            ╰── <25> Conditional [?]
                                ├── <19> Constant Int [1]
                                ├── Then
                                │   ╰── <21> FunctionCall [f]
                                ╰── Else
                                    ╰── <23> FunctionCall [g]
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
                        ╰── <12>  [||]
                            ├── <6> Constant Int [1]
                            ╰── <11> Cast
                                ├── Target
                                │   ╰── Void
                                ╰── Expression
                                    ╰── <10> Constant Int [2]
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
                    │       ╰── <17> Constant Int [0]
                    ├── DoWhile
                    │   ├── Body
                    │   │   ╰── Block
                    │   │       ╰── <28> Assign [=]
                    │   │           ├── <21> Var [i]
                    │   │           ╰── <27>  [+]
                    │   │               ├── <24> Var [i]
                    │   │               ╰── <26> Constant Int [1]
                    │   ╰── Condition
                    │       ╰── <33> FunctionCall [f]
                    ╰── Return
                        ╰── <35> Constant Int [0]
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
                    │   │           ╰── <17> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <22> FunctionCall [foo]
                    │   ╰── Empty
                    ╰── Return
                        ╰── <25> Constant Int [0]
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
                    │       ╰── <17> Constant Int [0]
                    ├── While
                    │   ├── Condition
                    │   │   ╰── <23> Cast
                    │   │       ├── Target
                    │   │       │   ╰── Void
                    │   │       ╰── Expression
                    │   │           ╰── <22> Constant Int [10]
                    │   ╰── Body
                    │       ╰── Block
                    │           ╰── <32> Assign [=]
                    │               ├── <25> Var [i]
                    │               ╰── <31>  [+]
                    │                   ├── <28> Var [i]
                    │                   ╰── <30> Constant Int [1]
                    ╰── Return
                        ╰── <37> Constant Int [0]
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
                    │       ╰── <9> Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <16> Cast
                    │   │       ├── Target
                    │   │       │   ╰── Void
                    │   │       ╰── Expression
                    │   │           ╰── <15> Var [x]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <17> Constant Int [0]
                    ╰── Return
                        ╰── <20> Constant Int [1]
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
                        ╰── <16> Conditional [?]
                            ├── <13> FunctionCall [f]
                            ├── Then
                            │   ╰── <14> Constant Int [1]
                            ╰── Else
                                ╰── <15> Constant Int [2]
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
                    ├── <27> Assign [=]
                    │   ├── <23> Dereference
                    │   │   ╰── <22> Var [x]
                    │   ╰── <26> FunctionCall [foo]
                    ╰── Return
                        ╰── <29> Constant Int [0]
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
                    ├── <18> Assign [=]
                    │   ├── <12> Var [v1]
                    │   ╰── <17> Cast
                    │       ├── Target
                    │       │   ╰── Void
                    │       ╰── Expression
                    │           ╰── <16> Constant Int [0]
                    ╰── Return
                        ╰── <20> Constant Int [0]
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
                    │       ╰── <9> Constant Int [10]
                    ├── <19> Assign [=]
                    │   ├── <13> Var [a]
                    │   ╰── <18> Cast
                    │       ├── Target
                    │       │   ╰── Void
                    │       ╰── Expression
                    │           ╰── <17> Constant Int [20]
                    ╰── Return
                        ╰── <21> Constant Int [0]
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
                        ╰── <10> Constant Int [0]
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
                        ╰── <13> Constant Int [0]
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
                    │       ╰── <17> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── flag
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <23> Constant Int [4]
                    ├── <36> Conditional [?]
                    │   ├── <27> Var [flag]
                    │   ├── Then
                    │   │   ╰── <29> FunctionCall [foo]
                    │   ╰── Else
                    │       ╰── <35> Assign [=]
                    │           ├── <31> Var [a]
                    │           ╰── <33> Constant Int [3]
                    ╰── Return
                        ╰── <38> Constant Int [0]
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
                    ├── <11> Unary [-]
                    │   ╰── <10> Cast
                    │       ├── Target
                    │       │   ╰── Void
                    │       ╰── Expression
                    │           ╰── <9> Constant Int [10]
                    ╰── Return
                        ╰── <13> Constant Int [0]
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
                    ├── <15> FunctionCall [foo]
                    ╰── Return
                        ╰── <17> Constant Int [0]
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
            │           ╰── <6> Constant Int [1]
            ╰── Function [main]
                ╰── Body
                    ├── <16> FunctionCall [x]
                    ╰── Return
                        ╰── <18> Constant Int [0]
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
                        ╰── <11> Cast
                            ├── Target
                            │   ╰── Void
                            ╰── Expression
                                ╰── <10> Constant Int [0]
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
                        ╰── <19> Subscript
                            ├── <14> Var [arr]
                            ╰── <18> Cast
                                ├── Target
                                │   ╰── Void
                                ╰── Expression
                                    ╰── <17> Constant Int [1]
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
                    │   │   ╰── <15>  [<]
                    │   │       ├── <9> Cast
                    │   │       │   ├── Target
                    │   │       │   │   ╰── Void
                    │   │       │   ╰── Expression
                    │   │       │       ╰── <8> Constant Int [1]
                    │   │       ╰── <14> Cast
                    │   │           ├── Target
                    │   │           │   ╰── Void
                    │   │           ╰── Expression
                    │   │               ╰── <13> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <16> Constant Int [1]
                    ╰── Return
                        ╰── <19> Constant Int [0]
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
                        ╰── <19>  [==]
                            ├── <13> FunctionCall [x]
                            ╰── <18> Cast
                                ├── Target
                                │   ╰── Void
                                ╰── Expression
                                    ╰── <17> Constant Int [10]
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
                        ╰── <15> Constant Int [0]
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
                    │   │   ╰── <10> Constant Int [0]
                    │   ╰── Static
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <16> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Char
                    │   ├── Initializer
                    │   │   ╰── <23> Constant Int [0]
                    │   ╰── Static
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <36>  [!=]
                    │   │       ├── <33> SizeOfExpr
                    │   │       │   ╰── <32>  [&]
                    │   │       │       ├── <27> Var [c]
                    │   │       │       ╰── <30> Var [i]
                    │   │       ╰── <35> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <37> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <52>  [!=]
                    │   │       ├── <49> SizeOfExpr
                    │   │       │   ╰── <48>  [|]
                    │   │       │       ├── <43> Var [i]
                    │   │       │       ╰── <46> Var [l]
                    │   │       ╰── <51> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <53> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <68>  [!=]
                    │   │       ├── <65> SizeOfExpr
                    │   │       │   ╰── <64>  [^]
                    │   │       │       ├── <59> Var [c]
                    │   │       │       ╰── <62> Var [c]
                    │   │       ╰── <67> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <69> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <84>  [!=]
                    │   │       ├── <81> SizeOfExpr
                    │   │       │   ╰── <80>  [<<]
                    │   │       │       ├── <75> Var [i]
                    │   │       │       ╰── <78> Var [l]
                    │   │       ╰── <83> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <85> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <100>  [!=]
                    │   │       ├── <97> SizeOfExpr
                    │   │       │   ╰── <96>  [<<]
                    │   │       │       ├── <91> Var [c]
                    │   │       │       ╰── <94> Var [i]
                    │   │       ╰── <99> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <101> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <116>  [!=]
                    │   │       ├── <113> SizeOfExpr
                    │   │       │   ╰── <112>  [>>]
                    │   │       │       ├── <107> Var [l]
                    │   │       │       ╰── <110> Var [c]
                    │   │       ╰── <115> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <117> Constant Int [6]
                    ╰── Return
                        ╰── <122> Constant Int [0]
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
                    │           ├── <12> Constant Int [1]
                    │           ╰── <14> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ├── Initializer
                    │   │   ╰── <22> Constant Int [3]
                    │   ╰── Static
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── uc
                    │   ├── Type
                    │   │   ╰── Unsigned Char
                    │   ├── Initializer
                    │   │   ╰── <29> Constant Int [4]
                    │   ╰── Static
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <35> Constant Double [+5e0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Long
                    │   ╰── Initializer
                    │       ╰── <44> Var [long_arr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <58>  [!=]
                    │   │       ├── <55> SizeOfExpr
                    │   │       │   ╰── <54> Assign [*=]
                    │   │       │       ├── <50> Subscript
                    │   │       │       │   ├── <48> Var [long_arr]
                    │   │       │       │   ╰── <49> Constant Int [1]
                    │   │       │       ╰── <52> Constant Int [10]
                    │   │       ╰── <57> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <59> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <73>  [!=]
                    │   │       ├── <70> SizeOfExpr
                    │   │       │   ╰── <69> Assign [/=]
                    │   │       │       ├── <65> Var [i]
                    │   │       │       ╰── <67> Constant ULong [10]
                    │   │       ╰── <72> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <74> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <88>  [!=]
                    │   │       ├── <85> SizeOfExpr
                    │   │       │   ╰── <84> Assign [&=]
                    │   │       │       ├── <80> Var [uc]
                    │   │       │       ╰── <82> Constant Int [2]
                    │   │       ╰── <87> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <89> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <103>  [!=]
                    │   │       ├── <100> SizeOfExpr
                    │   │       │   ╰── <99> Assign [-=]
                    │   │       │       ├── <95> Var [d]
                    │   │       │       ╰── <97> Constant Int [11]
                    │   │       ╰── <102> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <104> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <118>  [!=]
                    │   │       ├── <115> SizeOfExpr
                    │   │       │   ╰── <114> Assign [+=]
                    │   │       │       ├── <110> Var [ptr]
                    │   │       │       ╰── <112> Constant Int [1]
                    │   │       ╰── <117> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <119> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <130>  [!=]
                    │   │       ├── <127> Subscript
                    │   │       │   ├── <125> Var [long_arr]
                    │   │       │   ╰── <126> Constant Int [0]
                    │   │       ╰── <129> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <131> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <142>  [!=]
                    │   │       ├── <139> Subscript
                    │   │       │   ├── <137> Var [long_arr]
                    │   │       │   ╰── <138> Constant Int [1]
                    │   │       ╰── <141> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <143> Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <152>  [!=]
                    │   │       ├── <149> Var [i]
                    │   │       ╰── <151> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <153> Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <162>  [!=]
                    │   │       ├── <159> Var [uc]
                    │   │       ╰── <161> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <163> Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <172>  [!=]
                    │   │       ├── <169> Var [d]
                    │   │       ╰── <171> Constant Double [+5e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <173> Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <183>  [!=]
                    │   │       ├── <179> Var [ptr]
                    │   │       ╰── <182> Var [long_arr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <184> Constant Int [11]
                    ╰── Return
                        ╰── <189> Constant Int [0]
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
                    │   │   ╰── <10> Constant Int [10]
                    │   ╰── Static
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── u
                    │   ├── Type
                    │   │   ╰── Unsigned Int
                    │   ╰── Initializer
                    │       ╰── <16> Constant UInt [10000]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <24> Unary [-]
                    │           ╰── <23> Constant Int [99999]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <37>  [!=]
                    │   │       ├── <34> SizeOfExpr
                    │   │       │   ╰── <33> Assign [&=]
                    │   │       │       ├── <28> Var [sc]
                    │   │       │       ╰── <31> Var [l]
                    │   │       ╰── <36> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <38> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <53>  [!=]
                    │   │       ├── <50> SizeOfExpr
                    │   │       │   ╰── <49> Assign [|=]
                    │   │       │       ├── <44> Var [l]
                    │   │       │       ╰── <47> Var [u]
                    │   │       ╰── <52> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <54> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <69>  [!=]
                    │   │       ├── <66> SizeOfExpr
                    │   │       │   ╰── <65> Assign [^=]
                    │   │       │       ├── <60> Var [u]
                    │   │       │       ╰── <63> Var [l]
                    │   │       ╰── <68> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <70> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <85>  [!=]
                    │   │       ├── <82> SizeOfExpr
                    │   │       │   ╰── <81> Assign [>>=]
                    │   │       │       ├── <76> Var [l]
                    │   │       │       ╰── <79> Var [sc]
                    │   │       ╰── <84> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <86> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <101>  [!=]
                    │   │       ├── <98> SizeOfExpr
                    │   │       │   ╰── <97> Assign [<<=]
                    │   │       │       ├── <92> Var [sc]
                    │   │       │       ╰── <95> Var [sc]
                    │   │       ╰── <100> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <102> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <111>  [!=]
                    │   │       ├── <108> Var [sc]
                    │   │       ╰── <110> Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <112> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <121>  [!=]
                    │   │       ├── <118> Var [u]
                    │   │       ╰── <120> Constant UInt [10000]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <122> Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <133>  [!=]
                    │   │       ├── <128> Var [l]
                    │   │       ╰── <132> Unary [-]
                    │   │           ╰── <131> Constant Int [99999]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <134> Constant Int [8]
                    ╰── Return
                        ╰── <139> Constant Int [0]
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
                    │       ╰── <9> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── arr
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 3
                    │   │       ╰── Char
                    │   ├── Initializer
                    │   │   ╰── Compound
                    │   │       ├── <25> Constant Int [0]
                    │   │       ├── <27> Constant Int [0]
                    │   │       ╰── <29> Constant Int [0]
                    │   ╰── Static
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── <39> Var [arr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <50>  [!=]
                    │   │       ├── <47> SizeOfExpr
                    │   │       │   ╰── <46> Postfix [++]
                    │   │       │       ╰── <43> Var [i]
                    │   │       ╰── <49> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <51> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <66>  [!=]
                    │   │       ├── <63> SizeOfExpr
                    │   │       │   ╰── <62> Postfix [--]
                    │   │       │       ╰── <59> Subscript
                    │   │       │           ├── <57> Var [arr]
                    │   │       │           ╰── <58> Constant Int [0]
                    │   │       ╰── <65> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <67> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <80>  [!=]
                    │   │       ├── <77> SizeOfExpr
                    │   │       │   ╰── <76> Unary [++]
                    │   │       │       ╰── <74> Var [l]
                    │   │       ╰── <79> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <81> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <96>  [!=]
                    │   │       ├── <93> SizeOfExpr
                    │   │       │   ╰── <92> Unary [--]
                    │   │       │       ╰── <90> Subscript
                    │   │       │           ├── <88> Var [arr]
                    │   │       │           ╰── <89> Constant Int [1]
                    │   │       ╰── <95> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <97> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <110>  [!=]
                    │   │       ├── <107> SizeOfExpr
                    │   │       │   ╰── <106> Postfix [--]
                    │   │       │       ╰── <103> Var [ptr]
                    │   │       ╰── <109> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <111> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <117> Var [i]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <118> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <124> Var [l]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <125> Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <145>  [||]
                    │   │       ├── <139>  [||]
                    │   │       │   ├── <133> Subscript
                    │   │       │   │   ├── <131> Var [arr]
                    │   │       │   │   ╰── <132> Constant Int [0]
                    │   │       │   ╰── <138> Subscript
                    │   │       │       ├── <136> Var [arr]
                    │   │       │       ╰── <137> Constant Int [1]
                    │   │       ╰── <144> Subscript
                    │   │           ├── <142> Var [arr]
                    │   │           ╰── <143> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <146> Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <156>  [!=]
                    │   │       ├── <152> Var [ptr]
                    │   │       ╰── <155> Var [arr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <157> Constant Int [9]
                    ╰── Return
                        ╰── <162> Constant Int [0]
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
            │           ╰── <55> FunctionCall [calloc]
            │               ├── <53> Constant Int [100]
            │               ╰── <54> Constant Int [1]
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
            │       ╰── <78> FunctionCall [memset]
            │           ├── <74> Var [pointer]
            │           ├── <76> Var [byte]
            │           ╰── <77> Constant Int [100]
            ╰── Function [free_bytes]
                ├── Parameters
                │   ╰── Param
                │       ├── Name
                │       │   ╰── ptr
                │       ╰── Type
                │           ╰── Pointer
                │               ╰── Void
                ╰── Body
                    ╰── <95> FunctionCall [free]
                        ╰── <94> Var [ptr]
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
                    │       ╰── <45> FunctionCall [get_100_zeroed_bytes]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <51> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <59>  [<]
                    │   │       ├── <56> Var [i]
                    │   │       ╰── <58> Constant Int [100]
                    │   ├── Condition
                    │   │   ╰── <68> Assign [=]
                    │   │       ├── <61> Var [i]
                    │   │       ╰── <67>  [+]
                    │   │           ├── <64> Var [i]
                    │   │           ╰── <66> Constant Int [1]
                    │   ╰── Block
                    │       ╰── If
                    │           ├── Condition
                    │           │   ╰── <81> Subscript
                    │           │       ├── <79>  [+]
                    │           │       │   ├── <74> Cast
                    │           │       │   │   ├── Target
                    │           │       │   │   │   ╰── Pointer
                    │           │       │   │   │       ╰── Char
                    │           │       │   │   ╰── Expression
                    │           │       │   │       ╰── <73> Var [mem]
                    │           │       │   ╰── <77> Var [i]
                    │           │       ╰── <80> Constant Int [0]
                    │           ╰── Then
                    │               ╰── Block
                    │                   ╰── Return
                    │                       ╰── <82> Constant Int [1]
                    ├── <94> FunctionCall [fill_100_bytes]
                    │   ├── <92> Var [mem]
                    │   ╰── <93> Constant Int [99]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <99> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <107>  [<]
                    │   │       ├── <104> Var [i]
                    │   │       ╰── <106> Constant Int [100]
                    │   ├── Condition
                    │   │   ╰── <116> Assign [=]
                    │   │       ├── <109> Var [i]
                    │   │       ╰── <115>  [+]
                    │   │           ├── <112> Var [i]
                    │   │           ╰── <114> Constant Int [1]
                    │   ╰── Block
                    │       ╰── If
                    │           ├── Condition
                    │           │   ╰── <132>  [!=]
                    │           │       ├── <129> Subscript
                    │           │       │   ├── <127>  [+]
                    │           │       │   │   ├── <122> Cast
                    │           │       │   │   │   ├── Target
                    │           │       │   │   │   │   ╰── Pointer
                    │           │       │   │   │   │       ╰── Char
                    │           │       │   │   │   ╰── Expression
                    │           │       │   │   │       ╰── <121> Var [mem]
                    │           │       │   │   ╰── <125> Var [i]
                    │           │       │   ╰── <128> Constant Int [0]
                    │           │       ╰── <131> Constant Int [99]
                    │           ╰── Then
                    │               ╰── Block
                    │                   ╰── Return
                    │                       ╰── <133> Constant Int [2]
                    ├── <144> FunctionCall [free_bytes]
                    │   ╰── <143> Var [mem]
                    ╰── Return
                        ╰── <146> Constant Int [0]
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
                        ╰── <22>  [==]
                            ├── <19> SizeOfExpr
                            │   ╰── <18> Var [large_array]
                            ╰── <21> Constant Int [16000000]
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
            │       ╰── <13> Constant Int [0]
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
                    │   │   ╰── <70>  [!=]
                    │   │       ├── <67> Var [a]
                    │   │       ╰── <69> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── <73> FunctionCall [exit]
                    │               ╰── <72> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <82>  [!=]
                    │   │       ├── <79> Var [b]
                    │   │       ╰── <81> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── <85> FunctionCall [exit]
                    │               ╰── <84> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <94>  [!=]
                    │   │       ├── <91> Var [c]
                    │   │       ╰── <93> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── <97> FunctionCall [exit]
                    │               ╰── <96> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <106>  [!=]
                    │   │       ├── <103> Var [d]
                    │   │       ╰── <105> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── <109> FunctionCall [exit]
                    │               ╰── <108> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <118>  [!=]
                    │   │       ├── <115> Var [e]
                    │   │       ╰── <117> Constant Int [5]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── <121> FunctionCall [exit]
                    │               ╰── <120> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <130>  [!=]
                    │   │       ├── <127> Var [f]
                    │   │       ╰── <129> Constant Int [6]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── <133> FunctionCall [exit]
                    │               ╰── <132> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <142>  [!=]
                    │   │       ├── <139> Var [g]
                    │   │       ╰── <141> Constant Int [7]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── <145> FunctionCall [exit]
                    │               ╰── <144> Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <154>  [!=]
                    │   │       ├── <151> Var [h]
                    │   │       ╰── <153> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── <157> FunctionCall [exit]
                    │               ╰── <156> Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <166>  [!=]
                    │   │       ├── <163> Var [i]
                    │   │       ╰── <165> Constant Int [9]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── <169> FunctionCall [exit]
                    │               ╰── <168> Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <178>  [!=]
                    │   │       ├── <175> Var [j]
                    │   │       ╰── <177> Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── <181> FunctionCall [exit]
                    │               ╰── <180> Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <190>  [!=]
                    │   │       ├── <187> Var [k]
                    │   │       ╰── <189> Constant Int [11]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── <193> FunctionCall [exit]
                    │               ╰── <192> Constant Int [11]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <202>  [!=]
                    │   │       ├── <199> Var [l]
                    │   │       ╰── <201> Constant Int [12]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── <205> FunctionCall [exit]
                    │               ╰── <204> Constant Int [12]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <214>  [!=]
                    │   │       ├── <211> Var [m]
                    │   │       ╰── <213> Constant Int [13]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── <217> FunctionCall [exit]
                    │               ╰── <216> Constant Int [13]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <226>  [!=]
                    │   │       ├── <223> Var [n]
                    │   │       ╰── <225> Constant Int [14]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── <229> FunctionCall [exit]
                    │               ╰── <228> Constant Int [14]
                    ├── <243> Assign [=]
                    │   ├── <235> Var [sum]
                    │   ╰── <242>  [+]
                    │       ├── <238> Var [sum]
                    │       ╰── <241> Var [o]
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
                    │   │           ╰── <65> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <73>  [<]
                    │   │       ├── <70> Var [i]
                    │   │       ╰── <72> Constant Int [10000000]
                    │   ├── Condition
                    │   │   ╰── <82> Assign [=]
                    │   │       ├── <75> Var [i]
                    │   │       ╰── <81>  [+]
                    │   │           ├── <78> Var [i]
                    │   │           ╰── <80> Constant Int [1]
                    │   ╰── Block
                    │       ╰── <100> FunctionCall [lots_of_args]
                    │           ├── <84> Constant Int [1]
                    │           ├── <85> Constant Int [2]
                    │           ├── <86> Constant Int [3]
                    │           ├── <87> Constant Int [4]
                    │           ├── <88> Constant Int [5]
                    │           ├── <89> Constant Int [6]
                    │           ├── <90> Constant Int [7]
                    │           ├── <91> Constant Int [8]
                    │           ├── <92> Constant Int [9]
                    │           ├── <93> Constant Int [10]
                    │           ├── <94> Constant Int [11]
                    │           ├── <95> Constant Int [12]
                    │           ├── <96> Constant Int [13]
                    │           ├── <97> Constant Int [14]
                    │           ╰── <99> Var [i]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <109>  [!=]
                    │   │       ├── <106> Var [sum]
                    │   │       ╰── <108> Constant Long [49999995000000]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <110> Constant Int [15]
                    ╰── Return
                        ╰── <115> Constant Int [0]
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
                    │   │   ╰── <11>  [!=]
                    │   │       ├── <8> SizeOfType
                    │   │       │   ╰── Int
                    │   │       ╰── <10> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <12> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <21>  [!=]
                    │   │       ├── <18> SizeOfExpr
                    │   │       │   ╰── <17> Constant Double [+3e0]
                    │   │       ╰── <20> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <22> Constant Int [2]
                    ╰── Return
                        ╰── <27> Constant Int [0]
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
            │           ╰── <14> SizeOfExpr
            │               ╰── <13> Var [arr]
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
                    │   │   ╰── <35>  [!=]
                    │   │       ├── <32> SizeOfExpr
                    │   │       │   ╰── <31> Var [arr]
                    │   │       ╰── <34> Constant Int [12]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <36> Constant Int [1]
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
                    │   │   ╰── <59>  [!=]
                    │   │       ├── <56> SizeOfExpr
                    │   │       │   ╰── <55> Subscript
                    │   │       │       ├── <53> Var [nested_arr]
                    │   │       │       ╰── <54> Constant Int [2]
                    │   │       ╰── <58> Constant Int [40]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <60> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <69>  [!=]
                    │   │       ├── <66> SizeOfExpr
                    │   │       │   ╰── <65> "Hello, World!"
                    │   │       ╰── <68> Constant Int [14]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <70> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <81>  [!=]
                    │   │       ├── <78> FunctionCall [sizeof_adjusted_param]
                    │   │       │   ╰── <77> Var [arr]
                    │   │       ╰── <80> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <82> Constant Int [4]
                    ╰── Return
                        ╰── <87> Constant Int [0]
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
                    │   │   ╰── <11>  [!=]
                    │   │       ├── <8> SizeOfType
                    │   │       │   ╰── Char
                    │   │       ╰── <10> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <12> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <22>  [!=]
                    │   │       ├── <19> SizeOfType
                    │   │       │   ╰── Signed Char
                    │   │       ╰── <21> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <23> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <33>  [!=]
                    │   │       ├── <30> SizeOfType
                    │   │       │   ╰── Unsigned Char
                    │   │       ╰── <32> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <34> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <44>  [!=]
                    │   │       ├── <41> SizeOfType
                    │   │       │   ╰── Int
                    │   │       ╰── <43> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <45> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <55>  [!=]
                    │   │       ├── <52> SizeOfType
                    │   │       │   ╰── Unsigned Int
                    │   │       ╰── <54> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <56> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <66>  [!=]
                    │   │       ├── <63> SizeOfType
                    │   │       │   ╰── Long
                    │   │       ╰── <65> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <67> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <77>  [!=]
                    │   │       ├── <74> SizeOfType
                    │   │       │   ╰── Unsigned Long
                    │   │       ╰── <76> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <78> Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <88>  [!=]
                    │   │       ├── <85> SizeOfType
                    │   │       │   ╰── Double
                    │   │       ╰── <87> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <89> Constant Int [8]
                    ╰── Return
                        ╰── <94> Constant Int [0]
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
                    │   │   ╰── <10>  [!=]
                    │   │       ├── <7> SizeOfExpr
                    │   │       │   ╰── <6> Constant Int [97]
                    │   │       ╰── <9> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <11> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <20>  [!=]
                    │   │       ├── <17> SizeOfExpr
                    │   │       │   ╰── <16> Constant Int [2147483647]
                    │   │       ╰── <19> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <21> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <30>  [!=]
                    │   │       ├── <27> SizeOfExpr
                    │   │       │   ╰── <26> Constant UInt [4294967295]
                    │   │       ╰── <29> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <31> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <40>  [!=]
                    │   │       ├── <37> SizeOfExpr
                    │   │       │   ╰── <36> Constant Long [2]
                    │   │       ╰── <39> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <41> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <50>  [!=]
                    │   │       ├── <47> SizeOfExpr
                    │   │       │   ╰── <46> Constant ULong [0]
                    │   │       ╰── <49> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <51> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <60>  [!=]
                    │   │       ├── <57> SizeOfExpr
                    │   │       │   ╰── <56> Constant Double [+1e0]
                    │   │       ╰── <59> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <61> Constant Int [6]
                    ╰── Return
                        ╰── <66> Constant Int [0]
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
                    │   │   ╰── <25>  [!=]
                    │   │       ├── <22> SizeOfType
                    │   │       │   ╰── Array
                    │   │       │       ├── 2
                    │   │       │       ╰── Int
                    │   │       ╰── <24> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <26> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <45>  [!=]
                    │   │       ├── <42> SizeOfType
                    │   │       │   ╰── Array
                    │   │       │       ├── 3
                    │   │       │       ╰── Array
                    │   │       │           ├── 6
                    │   │       │           ╰── Array
                    │   │       │               ├── 17
                    │   │       │               ╰── Array
                    │   │       │                   ├── 9
                    │   │       │                   ╰── Char
                    │   │       ╰── <44> Constant Int [2754]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <46> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <61>  [!=]
                    │   │       ├── <58> SizeOfType
                    │   │       │   ╰── Array
                    │   │       │       ├── 4294967297
                    │   │       │       ╰── Array
                    │   │       │           ├── 100000000
                    │   │       │           ╰── Int
                    │   │       ╰── <60> Constant Long [1717986918800000000]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <62> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <73>  [!=]
                    │   │       ├── <70> SizeOfType
                    │   │       │   ╰── Pointer
                    │   │       │       ╰── Int
                    │   │       ╰── <72> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <74> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <92>  [!=]
                    │   │       ├── <89> SizeOfType
                    │   │       │   ╰── Pointer
                    │   │       │       ╰── Array
                    │   │       │           ├── 2
                    │   │       │           ╰── Array
                    │   │       │               ├── 4
                    │   │       │               ╰── Array
                    │   │       │                   ├── 6
                    │   │       │                   ╰── Int
                    │   │       ╰── <91> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <93> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <104>  [!=]
                    │   │       ├── <101> SizeOfType
                    │   │       │   ╰── Pointer
                    │   │       │       ╰── Char
                    │   │       ╰── <103> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <105> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <125>  [!=]
                    │   │       ├── <122> SizeOfType
                    │   │       │   ╰── Array
                    │   │       │       ├── 3
                    │   │       │       ╰── Array
                    │   │       │           ├── 4
                    │   │       │           ╰── Pointer
                    │   │       │               ╰── Array
                    │   │       │                   ├── 2
                    │   │       │                   ╰── Double
                    │   │       ╰── <124> Constant Int [96]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <126> Constant Int [7]
                    ╰── Return
                        ╰── <131> Constant Int [0]
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
                    │   │   ╰── <37>  [!=]
                    │   │       ├── <34> SizeOfExpr
                    │   │       │   ╰── <33> Var [d]
                    │   │       ╰── <36> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <38> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ╰── Type
                    │       ╰── Unsigned Char
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <52>  [!=]
                    │   │       ├── <49> SizeOfExpr
                    │   │       │   ╰── <48> Var [c]
                    │   │       ╰── <51> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <53> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── buffer
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Void
                    │   ╰── Initializer
                    │       ╰── <65> FunctionCall [malloc]
                    │           ╰── <64> Constant Int [100]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <74>  [!=]
                    │   │       ├── <71> SizeOfExpr
                    │   │       │   ╰── <70> Var [buffer]
                    │   │       ╰── <73> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <75> Constant Int [4]
                    ├── <83> FunctionCall [free]
                    │   ╰── <82> Var [buffer]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <94>  [!=]
                    │   │       ├── <91> SizeOfExpr
                    │   │       │   ╰── <90> Cast
                    │   │       │       ├── Target
                    │   │       │       │   ╰── Int
                    │   │       │       ╰── Expression
                    │   │       │           ╰── <88> Var [d]
                    │   │       ╰── <93> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <95> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <110>  [!=]
                    │   │       ├── <107> SizeOfExpr
                    │   │       │   ╰── <106> Conditional [?]
                    │   │       │       ├── <101> Var [d]
                    │   │       │       ├── Then
                    │   │       │       │   ╰── <103> Var [c]
                    │   │       │       ╰── Else
                    │   │       │           ╰── <104> Constant Long [10]
                    │   │       ╰── <109> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <111> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <125>  [!=]
                    │   │       ├── <122> SizeOfExpr
                    │   │       │   ╰── <121> Assign [=]
                    │   │       │       ├── <117> Var [c]
                    │   │       │       ╰── <119> Constant Double [+1e1]
                    │   │       ╰── <124> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <126> Constant Int [7]
                    ╰── Return
                        ╰── <131> Constant Int [0]
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
            │       ╰── <17> FunctionCall [exit]
            │           ╰── <16> Constant Int [10]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <29> SizeOfExpr
                            ╰── <28> FunctionCall [foo]
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
                    │   │   ╰── <12>  [!=]
                    │   │       ├── <9> SizeOfExpr
                    │   │       │   ╰── <8> SizeOfType
                    │   │       │       ╰── Char
                    │   │       ╰── <11> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <13> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <29>  [<]
                    │   │       ├── <26>  [-]
                    │   │       │   ├── <23>  [-]
                    │   │       │   │   ├── <19> SizeOfExpr
                    │   │       │   │   │   ╰── <18> Constant Int [4]
                    │   │       │   │   ╰── <22> SizeOfExpr
                    │   │       │   │       ╰── <21> Constant Int [4]
                    │   │       │   ╰── <25> Constant Int [1]
                    │   │       ╰── <28> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <30> Constant Int [2]
                    ╰── Return
                        ╰── <35> Constant Int [0]
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
            │       ├── <18> Assign [=]
            │       │   ├── <14> Var [x]
            │       │   ╰── <17> Var [i]
            │       ╰── Return
            │           ╰── <20> Constant Int [0]
            ├── Function [do_nothing]
            │   ╰── Body
            │       ╰── Empty
            ╰── Function [main]
                ╰── Body
                    ├── <41> Cast
                    │   ├── Target
                    │   │   ╰── Void
                    │   ╰── Expression
                    │       ╰── <40> Var [x]
                    ├── <48> Cast
                    │   ├── Target
                    │   │   ╰── Void
                    │   ╰── Expression
                    │       ╰── <47> FunctionCall [set_x]
                    │           ╰── <46> Constant Int [12]
                    ├── <54> Cast
                    │   ├── Target
                    │   │   ╰── Void
                    │   ╰── Expression
                    │       ╰── <53> FunctionCall [do_nothing]
                    ╰── Return
                        ╰── <57> Var [x]
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
            │       ╰── <38> Assign [=]
            │           ├── <31> Var [i]
            │           ╰── <37>  [+]
            │               ├── <34> Var [i]
            │               ╰── <36> Constant Int [1]
            ├── Function [incr_j]
            │   ╰── Body
            │       ╰── <55> Assign [=]
            │           ├── <48> Var [j]
            │           ╰── <54>  [+]
            │               ├── <51> Var [j]
            │               ╰── <53> Constant Int [1]
            ╰── Function [main]
                ╰── Body
                    ├── <70> Conditional [?]
                    │   ├── <65> Var [flag_1]
                    │   ├── Then
                    │   │   ╰── <67> FunctionCall [incr_i]
                    │   ╰── Else
                    │       ╰── <69> FunctionCall [incr_j]
                    ├── <78> Conditional [?]
                    │   ├── <73> Var [flag_0]
                    │   ├── Then
                    │   │   ╰── <75> FunctionCall [incr_i]
                    │   ╰── Else
                    │       ╰── <77> FunctionCall [incr_j]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <84>  [!=]
                    │   │       ├── <81> Var [i]
                    │   │       ╰── <83> Constant Int [5]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <85> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <94>  [!=]
                    │   │       ├── <91> Var [j]
                    │   │       ╰── <93> Constant Int [6]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <95> Constant Int [2]
                    ├── <111> Conditional [?]
                    │   ├── <101> Var [flag_0]
                    │   ├── Then
                    │   │   ╰── <103> FunctionCall [incr_j]
                    │   ╰── Else
                    │       ╰── <110> Conditional [?]
                    │           ├── <105> Var [flag_1]
                    │           ├── Then
                    │           │   ╰── <107> FunctionCall [incr_i]
                    │           ╰── Else
                    │               ╰── <109> FunctionCall [incr_j]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <117>  [!=]
                    │   │       ├── <114> Var [i]
                    │   │       ╰── <116> Constant Int [6]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <118> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <127>  [!=]
                    │   │       ├── <124> Var [j]
                    │   │       ╰── <126> Constant Int [6]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <128> Constant Int [4]
                    ╰── Return
                        ╰── <133> Constant Int [0]
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
            │       ╰── <23> Assign [=]
            │           ├── <20> Var [letter]
            │           ╰── <22> Constant Int [90]
            ├── Function [decrement_letter]
            │   ╰── Body
            │       ╰── <40> Assign [=]
            │           ├── <33> Var [letter]
            │           ╰── <39>  [-]
            │               ├── <36> Var [letter]
            │               ╰── <38> Constant Int [1]
            ╰── Function [main]
                ╰── Body
                    ├── For
                    │   ├── Init
                    │   │   ╰── <50> FunctionCall [initialize_letter]
                    │   ├── Condition
                    │   │   ╰── <55>  [>=]
                    │   │       ├── <52> Var [letter]
                    │   │       ╰── <54> Constant Int [65]
                    │   ├── Condition
                    │   │   ╰── <64> Assign [=]
                    │   │       ├── <57> Var [letter]
                    │   │       ╰── <63>  [-]
                    │   │           ├── <60> Var [letter]
                    │   │           ╰── <62> Constant Int [1]
                    │   ╰── Block
                    │       ╰── <68> FunctionCall [putchar]
                    │           ╰── <67> Var [letter]
                    ├── For
                    │   ├── Init
                    │   │   ╰── <77> Assign [=]
                    │   │       ├── <74> Var [letter]
                    │   │       ╰── <76> Constant Int [65]
                    │   ├── Condition
                    │   │   ╰── <82>  [<=]
                    │   │       ├── <79> Var [letter]
                    │   │       ╰── <81> Constant Int [90]
                    │   ├── Condition
                    │   │   ╰── <95> Cast
                    │   │       ├── Target
                    │   │       │   ╰── Void
                    │   │       ╰── Expression
                    │   │           ╰── <94> Assign [=]
                    │   │               ├── <86> Var [letter]
                    │   │               ╰── <92>  [+]
                    │   │                   ├── <89> Var [letter]
                    │   │                   ╰── <91> Constant Int [1]
                    │   ╰── Block
                    │       ╰── <99> FunctionCall [putchar]
                    │           ╰── <98> Var [letter]
                    ├── For
                    │   ├── Init
                    │   │   ╰── <105> FunctionCall [initialize_letter]
                    │   ├── Condition
                    │   │   ╰── <110>  [>=]
                    │   │       ├── <107> Var [letter]
                    │   │       ╰── <109> Constant Int [65]
                    │   ├── Condition
                    │   │   ╰── <112> FunctionCall [decrement_letter]
                    │   ╰── Block
                    │       ╰── <116> FunctionCall [putchar]
                    │           ╰── <115> Var [letter]
                    ╰── Return
                        ╰── <121> Constant Int [0]
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
            │       │   │   ╰── <19>  [<]
            │       │   │       ├── <16> Var [a]
            │       │   │       ╰── <18> Constant Int [0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       ├── <29> Assign [=]
            │       │   ├── <25> Var [foo]
            │       │   ╰── <28> Var [a]
            │       ╰── Return
            ├── Function [do_nothing]
            │   ╰── Body
            ╰── Function [main]
                ╰── Body
                    ├── <50> FunctionCall [set_foo_to_positive_num]
                    │   ╰── <49> Unary [-]
                    │       ╰── <48> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <53> Var [foo]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <54> Constant Int [1]
                    ├── <61> FunctionCall [set_foo_to_positive_num]
                    │   ╰── <60> Constant Int [12]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <67>  [!=]
                    │   │       ├── <64> Var [foo]
                    │   │       ╰── <66> Constant Int [12]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <68> Constant Int [2]
                    ├── <74> FunctionCall [do_nothing]
                    ╰── Return
                        ╰── <76> Constant Int [0]
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
                    │       ╰── <34> Constant Int [10]
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
                    │           ├── <50> FunctionCall [calloc]
                    │           │   ├── <46> Constant Int [2]
                    │           │   ╰── <49> SizeOfType
                    │           │       ╰── Int
                    │           ├── <54> AddressOf
                    │           │   ╰── <53> Var [i]
                    │           ├── <56> Constant Int [0]
                    │           ╰── <59> Var [arr]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Long
                    │   ╰── Initializer
                    │       ╰── <71> Subscript
                    │           ├── <69> Var [arr]
                    │           ╰── <70> Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <76> Dereference
                    │   │       ╰── <75> Var [l]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <77> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── elem_1_val
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <91> Dereference
                    │           ╰── <90> Cast
                    │               ├── Target
                    │               │   ╰── Pointer
                    │               │       ╰── Int
                    │               ╰── Expression
                    │                   ╰── <89> Subscript
                    │                       ├── <87> Var [arr]
                    │                       ╰── <88> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <98>  [!=]
                    │   │       ├── <95> Var [elem_1_val]
                    │   │       ╰── <97> Constant Int [10]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <99> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <105> Subscript
                    │   │       ├── <103> Var [arr]
                    │   │       ╰── <104> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <106> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <116>  [!=]
                    │   │       ├── <112> Subscript
                    │   │       │   ├── <110> Var [arr]
                    │   │       │   ╰── <111> Constant Int [3]
                    │   │       ╰── <115> Var [arr]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <117> Constant Int [4]
                    ├── <125> FunctionCall [free]
                    │   ╰── <124> Subscript
                    │       ├── <122> Var [arr]
                    │       ╰── <123> Constant Int [0]
                    ╰── Return
                        ╰── <127> Constant Int [0]
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
                    │       ╰── <41> FunctionCall [calloc]
                    │           ├── <37> Constant Int [3]
                    │           ╰── <40> SizeOfType
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
                    │           ├── <50> Constant Int [1]
                    │           ├── <52> Constant Int [2]
                    │           ╰── <54> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <62>  [==]
                    │   │       ├── <59> Var [void_ptr]
                    │   │       ╰── <61> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <63> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <71>  [==]
                    │   │       ├── <67> Var [void_ptr]
                    │   │       ╰── <70> Var [array]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <72> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <83> Unary [!]
                    │   │       ╰── <82>  [!=]
                    │   │           ├── <77> Var [void_ptr]
                    │   │           ╰── <80> Var [array]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <84> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── null_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Void
                    │   ├── Initializer
                    │   │   ╰── <93> Constant Int [0]
                    │   ╰── Static
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── my_array
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <107> Conditional [?]
                    │           ├── <102> Var [null_ptr]
                    │           ├── Then
                    │           │   ╰── <104> Var [void_ptr]
                    │           ╰── Else
                    │               ╰── <106> Var [array]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── array_element
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <116> Subscript
                    │           ├── <114> Var [my_array]
                    │           ╰── <115> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <123>  [!=]
                    │   │       ├── <120> Var [array_element]
                    │   │       ╰── <122> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <124> Constant Int [4]
                    ├── <132> FunctionCall [free]
                    │   ╰── <131> Var [void_ptr]
                    ╰── Return
                        ╰── <134> Constant Int [0]
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
            │           ╰── <58>  [+]
            │               ├── <55> Var [i]
            │               ╰── <57> Constant Int [3]
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
            │           ╰── <81>  [==]
            │               ├── <77> Dereference
            │               │   ╰── <76> Var [pointer]
            │               ╰── <80> Var [expected_val]
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
            │           ╰── <98> Var [pointer]
            ├── Function [get_dbl_array]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── n
            │   │       ╰── Type
            │   │           ╰── Unsigned Long
            │   ╰── Body
            │       ╰── Return
            │           ╰── <124> Cast
            │               ├── Target
            │               │   ╰── Pointer
            │               │       ╰── Double
            │               ╰── Expression
            │                   ╰── <123> FunctionCall [malloc]
            │                       ╰── <122>  [*]
            │                           ├── <117> Var [n]
            │                           ╰── <121> SizeOfType
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
            │       │   │           ╰── <147> Constant Int [0]
            │       │   ├── Condition
            │       │   │   ╰── <156>  [<]
            │       │   │       ├── <152> Var [i]
            │       │   │       ╰── <155> Var [n]
            │       │   ├── Condition
            │       │   │   ╰── <165> Assign [=]
            │       │   │       ├── <158> Var [i]
            │       │   │       ╰── <164>  [+]
            │       │   │           ├── <161> Var [i]
            │       │   │           ╰── <163> Constant Int [1]
            │       │   ╰── Block
            │       │       ╰── <174> Assign [=]
            │       │           ├── <170> Subscript
            │       │           │   ├── <167> Var [array]
            │       │           │   ╰── <169> Var [i]
            │       │           ╰── <173> Var [d]
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
            │           ╰── <195> Var [ptr]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── four_bytes
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Void
                    │   ╰── Initializer
                    │       ╰── <211> FunctionCall [malloc]
                    │           ╰── <210> Constant Int [4]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── int_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <220> Var [four_bytes]
                    ├── <230> Assign [=]
                    │   ├── <225> Dereference
                    │   │   ╰── <224> Var [int_ptr]
                    │   ╰── <229> Unary [-]
                    │       ╰── <228> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <240> Unary [!]
                    │   │       ╰── <239> FunctionCall [check_char_ptr_argument]
                    │   │           ├── <235> Var [four_bytes]
                    │   │           ╰── <238> Unary [-]
                    │   │               ╰── <237> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <241> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <253>  [!=]
                    │   │       ├── <249> FunctionCall [return_void_ptr_as_int_ptr]
                    │   │       │   ╰── <248> Var [four_bytes]
                    │   │       ╰── <252> Var [int_ptr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <254> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── dbl_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Double
                    │   ╰── Initializer
                    │       ╰── <265> Var [four_bytes]
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
                    │       ╰── <284> Var [four_bytes]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── long_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Long
                    │   ╰── Initializer
                    │       ╰── <293> Var [four_bytes]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <317>  [||]
                    │   │       ├── <309>  [||]
                    │   │       │   ├── <301>  [!=]
                    │   │       │   │   ├── <297> Var [dbl_ptr]
                    │   │       │   │   ╰── <300> Var [four_bytes]
                    │   │       │   ╰── <308>  [!=]
                    │   │       │       ├── <304> Var [complicated_ptr]
                    │   │       │       ╰── <307> Var [four_bytes]
                    │   │       ╰── <316>  [!=]
                    │   │           ├── <312> Var [long_ptr]
                    │   │           ╰── <315> Var [four_bytes]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <318> Constant Int [3]
                    ├── <326> FunctionCall [free]
                    │   ╰── <325> Var [four_bytes]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── dbl_array
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Double
                    │   ╰── Initializer
                    │       ╰── <335> FunctionCall [get_dbl_array]
                    │           ╰── <334> Constant Int [5]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── void_array
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Void
                    │   ╰── Initializer
                    │       ╰── <344> Var [dbl_array]
                    ├── <352> FunctionCall [set_doubles]
                    │   ├── <349> Var [void_array]
                    │   ├── <350> Constant Int [5]
                    │   ╰── <351> Constant Double [+4e0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <360>  [!=]
                    │   │       ├── <357> Subscript
                    │   │       │   ├── <355> Var [dbl_array]
                    │   │       │   ╰── <356> Constant Int [3]
                    │   │       ╰── <359> Constant Double [+4e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <361> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <373>  [!=]
                    │   │       ├── <369> FunctionCall [return_dbl_ptr_as_void_ptr]
                    │   │       │   ╰── <368> Var [dbl_array]
                    │   │       ╰── <372> Var [void_array]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <374> Constant Int [5]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── some_other_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Void
                    │   ╰── Initializer
                    │       ╰── <384> Constant Int [0]
                    ├── <392> Assign [=]
                    │   ├── <388> Var [some_other_ptr]
                    │   ╰── <391> Var [dbl_array]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <399>  [!=]
                    │   │       ├── <395> Var [some_other_ptr]
                    │   │       ╰── <398> Var [void_array]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <400> Constant Int [6]
                    ├── <411> Assign [=]
                    │   ├── <406> Var [some_other_ptr]
                    │   ╰── <410> AddressOf
                    │       ╰── <409> Var [some_other_ptr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <418>  [==]
                    │   │       ├── <414> Var [some_other_ptr]
                    │   │       ╰── <417> Var [void_array]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <419> Constant Int [7]
                    ├── <428> Assign [=]
                    │   ├── <425> Var [complicated_ptr]
                    │   ╰── <427> Constant Int [0]
                    ├── <435> Assign [=]
                    │   ├── <431> Var [some_other_ptr]
                    │   ╰── <434> Var [complicated_ptr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <438> Var [some_other_ptr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <439> Constant Int [8]
                    ├── <447> FunctionCall [free]
                    │   ╰── <446> Var [dbl_array]
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
                    │           ├── <461> FunctionCall [malloc]
                    │           │   ╰── <460> SizeOfType
                    │           │       ╰── Long
                    │           ├── <467> FunctionCall [malloc]
                    │           │   ╰── <466> SizeOfType
                    │           │       ╰── Long
                    │           ╰── <473> FunctionCall [malloc]
                    │               ╰── <472> SizeOfType
                    │                   ╰── Long
                    ├── <484> Assign [=]
                    │   ├── <481> Dereference
                    │   │   ╰── <480> Subscript
                    │   │       ├── <478> Var [long_ptr_array]
                    │   │       ╰── <479> Constant Int [0]
                    │   ╰── <483> Constant Long [100]
                    ├── <493> Assign [=]
                    │   ├── <490> Dereference
                    │   │   ╰── <489> Subscript
                    │   │       ├── <487> Var [long_ptr_array]
                    │   │       ╰── <488> Constant Int [1]
                    │   ╰── <492> Constant Long [200]
                    ├── <502> Assign [=]
                    │   ├── <499> Dereference
                    │   │   ╰── <498> Subscript
                    │   │       ├── <496> Var [long_ptr_array]
                    │   │       ╰── <497> Constant Int [2]
                    │   ╰── <501> Constant Long [300]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── sum
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <526>  [+]
                    │           ├── <518>  [+]
                    │           │   ├── <511> Dereference
                    │           │   │   ╰── <510> Subscript
                    │           │   │       ├── <508> Var [long_ptr_array]
                    │           │   │       ╰── <509> Constant Int [0]
                    │           │   ╰── <517> Dereference
                    │           │       ╰── <516> Subscript
                    │           │           ├── <514> Var [long_ptr_array]
                    │           │           ╰── <515> Constant Int [1]
                    │           ╰── <524> Dereference
                    │               ╰── <523> Subscript
                    │                   ├── <521> Var [long_ptr_array]
                    │                   ╰── <522> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <533>  [!=]
                    │   │       ├── <530> Var [sum]
                    │   │       ╰── <532> Constant Long [600]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <534> Constant Int [9]
                    ├── <544> FunctionCall [free]
                    │   ╰── <543> Subscript
                    │       ├── <541> Var [long_ptr_array]
                    │       ╰── <542> Constant Int [0]
                    ├── <551> FunctionCall [free]
                    │   ╰── <550> Subscript
                    │       ├── <548> Var [long_ptr_array]
                    │       ╰── <549> Constant Int [1]
                    ├── <558> FunctionCall [free]
                    │   ╰── <557> Subscript
                    │       ├── <555> Var [long_ptr_array]
                    │       ╰── <556> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── arr1
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 3
                    │   │       ╰── Long
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <566> Constant Int [1]
                    │           ├── <568> Constant Int [2]
                    │           ╰── <570> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── arr2
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 3
                    │   │       ╰── Long
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <580> Constant Int [1]
                    │           ├── <582> Constant Int [2]
                    │           ╰── <584> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── arr3
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 3
                    │   │       ╰── Long
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <594> Constant Int [1]
                    │           ├── <596> Constant Int [2]
                    │           ╰── <598> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <613>  [!=]
                    │   │       ├── <610> FunctionCall [memcmp]
                    │   │       │   ├── <604> Var [arr1]
                    │   │       │   ├── <606> Var [arr2]
                    │   │       │   ╰── <609> SizeOfExpr
                    │   │       │       ╰── <608> Var [arr1]
                    │   │       ╰── <612> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <614> Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <632>  [!=]
                    │   │       ├── <627> FunctionCall [memcmp]
                    │   │       │   ├── <621> Var [arr1]
                    │   │       │   ├── <623> Var [arr3]
                    │   │       │   ╰── <626> SizeOfExpr
                    │   │       │       ╰── <625> Var [arr2]
                    │   │       ╰── <631> Unary [-]
                    │   │           ╰── <630> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <633> Constant Int [11]
                    ╰── Return
                        ╰── <638> Constant Int [0]
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
                    │       ╰── <61> FunctionCall [malloc]
                    │           ╰── <60>  [*]
                    │               ├── <55> Constant Int [4]
                    │               ╰── <59> SizeOfType
                    │                   ╰── Double
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── double_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Double
                    │   ╰── Initializer
                    │       ╰── <74> Cast
                    │           ├── Target
                    │           │   ╰── Pointer
                    │           │       ╰── Double
                    │           ╰── Expression
                    │               ╰── <73> Var [ptr]
                    ├── <83> Assign [=]
                    │   ├── <80> Subscript
                    │   │   ├── <78> Var [double_ptr]
                    │   │   ╰── <79> Constant Int [2]
                    │   ╰── <82> Constant Double [+1e1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <94>  [!=]
                    │   │       ├── <90> Cast
                    │   │       │   ├── Target
                    │   │       │   │   ╰── Pointer
                    │   │       │   │       ╰── Void
                    │   │       │   ╰── Expression
                    │   │       │       ╰── <89> Var [double_ptr]
                    │   │       ╰── <93> Var [ptr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <95> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── result
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <106> Subscript
                    │           ├── <104> Var [double_ptr]
                    │           ╰── <105> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <113>  [!=]
                    │   │       ├── <110> Var [result]
                    │   │       ╰── <112> Constant Double [+1e1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <114> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ul
                    │   ├── Type
                    │   │   ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <126> Cast
                    │           ├── Target
                    │           │   ╰── Unsigned Long
                    │           ╰── Expression
                    │               ╰── <125> Var [ptr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <133>  [%]
                    │   │       ├── <130> Var [ul]
                    │   │       ╰── <132> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <134> Constant Int [3]
                    ├── <142> FunctionCall [free]
                    │   ╰── <141> Var [ptr]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── zero
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <147> Constant Int [0]
                    ├── <159> Assign [=]
                    │   ├── <151> Var [ptr]
                    │   ╰── <158> Cast
                    │       ├── Target
                    │       │   ╰── Pointer
                    │       │       ╰── Void
                    │       ╰── Expression
                    │           ╰── <157> Var [zero]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <162> Var [ptr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <163> Constant Int [4]
                    ├── <176> Assign [=]
                    │   ├── <169> Var [zero]
                    │   ╰── <175> Cast
                    │       ├── Target
                    │       │   ╰── Long
                    │       ╰── Expression
                    │           ╰── <174> Var [ptr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <179> Var [zero]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <180> Constant Int [5]
                    ╰── Return
                        ╰── <185> Constant Int [0]
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
                    │       ╰── <79> FunctionCall [malloc]
                    │           ╰── <78> Constant Int [50]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <85> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <93>  [<]
                    │   │       ├── <90> Var [i]
                    │   │       ╰── <92> Constant Int [50]
                    │   ├── Condition
                    │   │   ╰── <102> Assign [=]
                    │   │       ├── <95> Var [i]
                    │   │       ╰── <101>  [+]
                    │   │           ├── <98> Var [i]
                    │   │           ╰── <100> Constant Int [1]
                    │   ╰── Block
                    │       ╰── <111> Assign [=]
                    │           ├── <107> Subscript
                    │           │   ├── <104> Var [char_buffer]
                    │           │   ╰── <106> Var [i]
                    │           ╰── <110> Var [i]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── char_buffer2
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── <125> FunctionCall [realloc]
                    │           ├── <123> Var [char_buffer]
                    │           ╰── <124> Constant Int [100]
                    ├── <134> Assign [=]
                    │   ├── <131> Subscript
                    │   │   ├── <129> Var [char_buffer2]
                    │   │   ╰── <130> Constant Int [75]
                    │   ╰── <133> Constant Int [11]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <139> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <147>  [<]
                    │   │       ├── <144> Var [i]
                    │   │       ╰── <146> Constant Int [50]
                    │   ├── Condition
                    │   │   ╰── <156> Assign [=]
                    │   │       ├── <149> Var [i]
                    │   │       ╰── <155>  [+]
                    │   │           ├── <152> Var [i]
                    │   │           ╰── <154> Constant Int [1]
                    │   ╰── Block
                    │       ╰── If
                    │           ├── Condition
                    │           │   ╰── <165>  [!=]
                    │           │       ├── <161> Subscript
                    │           │       │   ├── <158> Var [char_buffer2]
                    │           │       │   ╰── <160> Var [i]
                    │           │       ╰── <164> Var [i]
                    │           ╰── Then
                    │               ╰── Block
                    │                   ╰── Return
                    │                       ╰── <166> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <180>  [!=]
                    │   │       ├── <177> Subscript
                    │   │       │   ├── <175> Var [char_buffer2]
                    │   │       │   ╰── <176> Constant Int [75]
                    │   │       ╰── <179> Constant Int [11]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <181> Constant Int [2]
                    ├── <189> FunctionCall [free]
                    │   ╰── <188> Var [char_buffer2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── double_buffer
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Double
                    │   ╰── Initializer
                    │       ╰── <201> FunctionCall [calloc]
                    │           ├── <197> Constant Int [10]
                    │           ╰── <200> SizeOfType
                    │               ╰── Double
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <207> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <215>  [<]
                    │   │       ├── <212> Var [i]
                    │   │       ╰── <214> Constant Int [10]
                    │   ├── Condition
                    │   │   ╰── <224> Assign [=]
                    │   │       ├── <217> Var [i]
                    │   │       ╰── <223>  [+]
                    │   │           ├── <220> Var [i]
                    │   │           ╰── <222> Constant Int [1]
                    │   ╰── Block
                    │       ╰── If
                    │           ├── Condition
                    │           │   ╰── <229> Subscript
                    │           │       ├── <226> Var [double_buffer]
                    │           │       ╰── <228> Var [i]
                    │           ╰── Then
                    │               ╰── Block
                    │                   ╰── Return
                    │                       ╰── <230> Constant Int [3]
                    ├── <241> FunctionCall [free]
                    │   ╰── <240> Var [double_buffer]
                    ├── <250> Assign [=]
                    │   ├── <244> Var [char_buffer]
                    │   ╰── <249> FunctionCall [aligned_alloc]
                    │       ├── <247> Constant Int [256]
                    │       ╰── <248> Constant Int [256]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <259>  [%]
                    │   │       ├── <256> Cast
                    │   │       │   ├── Target
                    │   │       │   │   ╰── Unsigned Long
                    │   │       │   ╰── Expression
                    │   │       │       ╰── <255> Var [char_buffer]
                    │   │       ╰── <258> Constant Int [256]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <260> Constant Int [4]
                    ├── <268> FunctionCall [free]
                    │   ╰── <267> Var [char_buffer]
                    ╰── Return
                        ╰── <270> Constant Int [0]
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
                    │       ╰── <40> FunctionCall [malloc]
                    │           ╰── <39>  [*]
                    │               ├── <34> Constant Int [10]
                    │               ╰── <38> SizeOfType
                    │                   ╰── Int
                    ├── <49> Assign [=]
                    │   ├── <46> Subscript
                    │   │   ├── <44> Var [array]
                    │   │   ╰── <45> Constant Int [2]
                    │   ╰── <48> Constant Int [100]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── result
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <57> Subscript
                    │           ├── <55> Var [array]
                    │           ╰── <56> Constant Int [2]
                    ├── <63> FunctionCall [free]
                    │   ╰── <62> Var [array]
                    ╰── Return
                        ╰── <66> Var [result]
    "#;
    assert_parse(src, expected);
}
