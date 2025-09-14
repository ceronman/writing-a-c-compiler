use super::{assert_error, assert_parse};

#[test]
fn test_invalid_declarations_extra_credit_addr_of_label() {
    let src = r#"
        
        int main(void) {
            int x = 0;
            lbl:
            x = 1;
            if (&lbl == 0) {
                return 1;
            }
            goto lbl;
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
                    │       ╰── <9> Constant Int [0]
                    ├── Label [lbl]
                    │   ╰── <17> Assign [=]
                    │       ├── <14> Var [x]
                    │       ╰── <16> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <25>  [==]
                    │   │       ├── <22> AddressOf
                    │   │       │   ╰── <21> Var [lbl]
                    │   │       ╰── <24> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <26> Constant Int [1]
                    ├── Goto [lbl]
                    ╰── Return
                        ╰── <33> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_declarations_extra_credit_deref_label() {
    let src = r#"
        
        int main(void) {
            lbl:
            *lbl;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── Label [lbl]
                    │   ╰── <9> Dereference
                    │       ╰── <8> Var [lbl]
                    ╰── Return
                        ╰── <12> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_parse_abstract_function_declarator() {
    assert_error(
        r#"
        int main(void) {
            (int (void)) 0;
                //^^^^ Expected ')', but found 'void'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_cast_to_declarator() {
    assert_error(
        r#"
        int main(void)
        {
            return (int **a)(10);
                        //^ Expected ')', but found 'a'
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_malformed_abstract_declarator() {
    assert_error(
        r#"
        int main(void) {
            (int (*)*) 10;
                  //^ Expected ')', but found '*'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_malformed_declarator() {
    assert_error(
        r#"
        int main(void) {
            int (*)* y;
                //^ Expected identifier, but found ')'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_malformed_function_declarator() {
    assert_error(
        r#"
        int (foo(void))(void);
          //^^^^^^^^^^^ Can't apply additional derivations to a function type
        int main(void) {
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_malformed_function_declarator_2() {
    assert_error(
        r#"
        int foo((void));
              //^ Expected type specifier
    "#,
    );
}

#[test]
fn test_invalid_types_address_of_address() {
    let src = r#"
        int main(void) {
            int x = 0;
            int *y = &x;
            int **z = &(&x);
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
                    │       ╰── <9> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── y
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <19> AddressOf
                    │           ╰── <18> Var [x]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── z
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Pointer
                    │   │           ╰── Int
                    │   ╰── Initializer
                    │       ╰── <33> AddressOf
                    │           ╰── <32> AddressOf
                    │               ╰── <30> Var [x]
                    ╰── Return
                        ╰── <36> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_address_of_assignment() {
    let src = r#"
        int main(void) {
            int x = 0;
            int y = 0;
            int *ptr = &(x = y);
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
                    │       ╰── <9> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── y
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <30> AddressOf
                    │           ╰── <29> Assign [=]
                    │               ├── <24> Var [x]
                    │               ╰── <27> Var [y]
                    ╰── Return
                        ╰── <33> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_address_of_constant() {
    let src = r#"
        
        int main(void) {
            int *ptr = &10;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <12> AddressOf
                    │           ╰── <11> Constant Int [10]
                    ╰── Return
                        ╰── <15> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_address_of_ternary() {
    let src = r#"
        int main(void) {
            int x = 1;
            int y = 2;
            int z = 3;
            int *ptr = &(x ? y : z);
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
                    │       ╰── <9> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── y
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── z
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <21> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <37> AddressOf
                    │           ╰── <36> Conditional [?]
                    │               ├── <30> Var [x]
                    │               ├── Then
                    │               │   ╰── <32> Var [y]
                    │               ╰── Else
                    │                   ╰── <34> Var [z]
                    ╰── Return
                        ╰── <40> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_assign_int_to_pointer() {
    let src = r#"
        int main(void) {
            int *x;
            x = 10;
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
                    │       ╰── Pointer
                    │           ╰── Int
                    ├── <16> Assign [=]
                    │   ├── <13> Var [x]
                    │   ╰── <15> Constant Int [10]
                    ╰── Return
                        ╰── <18> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_assign_int_var_to_pointer() {
    let src = r#"
        int main(void)
        {
            int x = 0;
            int *ptr = x;
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
                    ╰── VarDeclaration
                        ├── Name
                        │   ╰── ptr
                        ├── Type
                        │   ╰── Pointer
                        │       ╰── Int
                        ╰── Initializer
                            ╰── <18> Var [x]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_assign_to_address() {
    let src = r#"
        int main(void)
        {
            int x = 0;
            &x = 10;
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
                    ╰── <17> Assign [=]
                        ├── <14> AddressOf
                        │   ╰── <13> Var [x]
                        ╰── <16> Constant Int [10]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_assign_wrong_pointer_type() {
    let src = r#"
        int main(void)
        {
            double *d = 0;
            long *l = 0;
            l = d;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Double
                    │   ╰── Initializer
                    │       ╰── <11> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Long
                    │   ╰── Initializer
                    │       ╰── <19> Constant Int [0]
                    ├── <27> Assign [=]
                    │   ├── <23> Var [l]
                    │   ╰── <26> Var [d]
                    ╰── Return
                        ╰── <29> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_bad_null_pointer_constant() {
    let src = r#"
        int main(void)
        {
            int *x = 0.0;
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
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <11> Constant Double [+0e0]
                    ╰── Return
                        ╰── <14> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_cast_double_to_pointer() {
    let src = r#"
        int main(void) {
            double d = 0.0;
            int *x = (int *) d;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <9> Constant Double [+0e0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <22> Cast
                    │           ├── Target
                    │           │   ╰── Pointer
                    │           │       ╰── Int
                    │           ╰── Expression
                    │               ╰── <21> Var [d]
                    ╰── Return
                        ╰── <25> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_cast_pointer_to_double() {
    let src = r#"
        int main(void) {
            int *x;
            double d = (double) x;
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
                    │       ╰── Pointer
                    │           ╰── Int
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <19> Cast
                    │           ├── Target
                    │           │   ╰── Double
                    │           ╰── Expression
                    │               ╰── <18> Var [x]
                    ╰── Return
                        ╰── <22> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_compare_mixed_pointer_types() {
    let src = r#"
        
        int main(void) {
            int *x = 0ul;
            unsigned *y = 0ul;
            return x == y;
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
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <11> Constant ULong [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── y
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Unsigned Int
                    │   ╰── Initializer
                    │       ╰── <19> Constant ULong [0]
                    ╰── Return
                        ╰── <27>  [==]
                            ├── <23> Var [x]
                            ╰── <26> Var [y]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_compare_pointer_to_ulong() {
    let src = r#"
        
        int main(void) {
            int *ptr = 0ul;
            unsigned long ul = 0ul;
            return ptr == ul;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <11> Constant ULong [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ul
                    │   ├── Type
                    │   │   ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <17> Constant ULong [0]
                    ╰── Return
                        ╰── <25>  [==]
                            ├── <21> Var [ptr]
                            ╰── <24> Var [ul]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_complement_pointer() {
    let src = r#"
        
        int main(void) {
            int *x = 0;
            return (int) ~x;
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
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <11> Constant Int [0]
                    ╰── Return
                        ╰── <20> Cast
                            ├── Target
                            │   ╰── Int
                            ╰── Expression
                                ╰── <19> Unary [~]
                                    ╰── <18> Var [x]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_dereference_non_pointer() {
    let src = r#"
        
        int main(void) {
            unsigned long l = 100ul;
            return *l;
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
                    │   │   ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <9> Constant ULong [100]
                    ╰── Return
                        ╰── <14> Dereference
                            ╰── <13> Var [l]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_divide_pointer() {
    let src = r#"
        
        int main(void)
        {
            int x = 10;
            int *y = &x;
            (y / 8);
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
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <19> AddressOf
                    │           ╰── <18> Var [x]
                    ├── <27>  [/]
                    │   ├── <23> Var [y]
                    │   ╰── <25> Constant Int [8]
                    ╰── Return
                        ╰── <29> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_bitwise_and_pointer() {
    let src = r#"
        
        int main(void) {
            long *ptr = 0;
            10 & ptr;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Long
                    │   ╰── Initializer
                    │       ╰── <11> Constant Int [0]
                    ├── <18>  [&]
                    │   ├── <14> Constant Int [10]
                    │   ╰── <17> Var [ptr]
                    ╰── Return
                        ╰── <20> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_bitwise_compound_assign_to_pointer() {
    let src = r#"
        
        int main(void) {
            int x = 0;
            int *ptr = &x;
            ptr &= 0;
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
                    │       ╰── <9> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <19> AddressOf
                    │           ╰── <18> Var [x]
                    ├── <26> Assign [&=]
                    │   ├── <23> Var [ptr]
                    │   ╰── <25> Constant Int [0]
                    ╰── Return
                        ╰── <28> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_bitwise_compound_assign_with_pointer() {
    let src = r#"
        int main(void) {
            int *null = 0;
            int x = 100;
            x |= null;
            return 1;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── null
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <11> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <17> Constant Int [100]
                    ├── <25> Assign [|=]
                    │   ├── <21> Var [x]
                    │   ╰── <24> Var [null]
                    ╰── Return
                        ╰── <27> Constant Int [1]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_bitwise_lshift_pointer() {
    let src = r#"
        
        int main(void) {
            int *ptr = 0;
            int i = 1000;
            i >> ptr;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <11> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <17> Constant Int [1000]
                    ├── <25>  [>>]
                    │   ├── <21> Var [i]
                    │   ╰── <24> Var [ptr]
                    ╰── Return
                        ╰── <27> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_bitwise_or_pointer() {
    let src = r#"
        
        int main(void) {
            int *x = 0;
            int *y = 0;
            x | y;
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
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <11> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── y
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <19> Constant Int [0]
                    ├── <27>  [|]
                    │   ├── <23> Var [x]
                    │   ╰── <26> Var [y]
                    ╰── Return
                        ╰── <29> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_bitwise_rshift_pointer() {
    let src = r#"
        
        int main(void) {
            int *x = 0;
            return (int) (x >> 10);
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
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <11> Constant Int [0]
                    ╰── Return
                        ╰── <22> Cast
                            ├── Target
                            │   ╰── Int
                            ╰── Expression
                                ╰── <21>  [>>]
                                    ├── <17> Var [x]
                                    ╰── <19> Constant Int [10]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_bitwise_xor_pointer() {
    let src = r#"
        
        int main(void) {
            unsigned long *ptr = 0;
            long l = 100;
            ptr ^ l;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <11> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <17> Constant Int [100]
                    ├── <25>  [^]
                    │   ├── <21> Var [ptr]
                    │   ╰── <24> Var [l]
                    ╰── Return
                        ╰── <27> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_compound_assign_thru_ptr_not_lval() {
    let src = r#"
        int main(void) {
            int i = 100;
            int *ptr = &i;
            int *ptr2 = &(*ptr -= 10);
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
                    │       ╰── <9> Constant Int [100]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <19> AddressOf
                    │           ╰── <18> Var [i]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr2
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <34> AddressOf
                    │           ╰── <33> Assign [-=]
                    │               ├── <29> Dereference
                    │               │   ╰── <28> Var [ptr]
                    │               ╰── <31> Constant Int [10]
                    ╰── Return
                        ╰── <37> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_compound_assignment_not_lval() {
    let src = r#"
        int main(void) {
            int i = 100;
            int *ptr = &(i += 200);
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
                    │       ╰── <9> Constant Int [100]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <23> AddressOf
                    │           ╰── <22> Assign [+=]
                    │               ├── <18> Var [i]
                    │               ╰── <20> Constant Int [200]
                    ╰── Return
                        ╰── <26> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_compound_divide_pointer() {
    let src = r#"
        int main(void) {
            int *x = 0;
            int *y = 0;
            x /= y;
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
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <11> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── y
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <19> Constant Int [0]
                    ├── <27> Assign [/=]
                    │   ├── <23> Var [x]
                    │   ╰── <26> Var [y]
                    ╰── Return
                        ╰── <29> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_compound_mod_pointer() {
    let src = r#"
        int main(void) {
            int i = 10;
            int *ptr = &i;
            i %= ptr;
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
                    │       ╰── <9> Constant Int [10]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <19> AddressOf
                    │           ╰── <18> Var [i]
                    ├── <27> Assign [%=]
                    │   ├── <23> Var [i]
                    │   ╰── <26> Var [ptr]
                    ╰── Return
                        ╰── <29> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_compound_multiply_pointer() {
    let src = r#"
        int main(void) {
            int *x = 0;
            x *= 2;
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
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <11> Constant Int [0]
                    ├── <18> Assign [*=]
                    │   ├── <15> Var [x]
                    │   ╰── <17> Constant Int [2]
                    ╰── Return
                        ╰── <20> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_postfix_decr_not_lvalue() {
    let src = r#"
        int main(void) {
            int i = 10;
            int *ptr = &i--;
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
                    │       ╰── <9> Constant Int [10]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <21> AddressOf
                    │           ╰── <20> Postfix [--]
                    │               ╰── <18> Var [i]
                    ╰── Return
                        ╰── <24> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_prefix_incr_not_lvalue() {
    let src = r#"
        int main(void) {
            int i = 10;
            int *ptr = &++i;
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
                    │       ╰── <9> Constant Int [10]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <21> AddressOf
                    │           ╰── <20> Unary [++]
                    │               ╰── <19> Var [i]
                    ╰── Return
                        ╰── <24> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_switch_on_pointer() {
    let src = r#"
        int main(void) {
            int *x = 0;
            switch(x) {
                case 0: return 0;
                default: return 1;
            }
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
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <11> Constant Int [0]
                    ╰── Switch
                        ├── Expression
                        │   ╰── <15> Var [x]
                        ╰── Block
                            ├── Case [0]
                            │   ╰── Return
                            │       ╰── <17> Constant Int [0]
                            ╰── Default
                                ╰── Return
                                    ╰── <20> Constant Int [1]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_invalid_pointer_initializer() {
    let src = r#"
        int main(void)
        {
            int *ptr = 140732898195768ul;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <11> Constant ULong [140732898195768]
                    ╰── Return
                        ╰── <14> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_invalid_static_initializer() {
    let src = r#"
        
        static int *x = 10;
        int main(void) {
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
            │   │       ╰── Int
            │   ├── Initializer
            │   │   ╰── <7> Constant Int [10]
            │   ╰── Static
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <15> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_multiply_pointers() {
    let src = r#"
        
        int main(void) {
            int *x = 0;
            int *y = x;
            (x * y);
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
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <11> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── y
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <20> Var [x]
                    ├── <29>  [*]
                    │   ├── <24> Var [x]
                    │   ╰── <27> Var [y]
                    ╰── Return
                        ╰── <31> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_multiply_pointers_2() {
    let src = r#"
        
        int main(void)
        {
            int *x = 0;
            (0 * x);
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
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <11> Constant Int [0]
                    ├── <19>  [*]
                    │   ├── <14> Constant Int [0]
                    │   ╰── <17> Var [x]
                    ╰── Return
                        ╰── <21> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_negate_pointer() {
    let src = r#"
        
        int main(void) {
            int *x = 0;
            -x;
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
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <11> Constant Int [0]
                    ├── <17> Unary [-]
                    │   ╰── <16> Var [x]
                    ╰── Return
                        ╰── <19> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_pass_pointer_as_int() {
    let src = r#"
        int f(int i) {
            return i;
        }
        int main(void) {
            int x;
            return f(&x);
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
            │       ╰── Return
            │           ╰── <10> Var [i]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ╰── Type
                    │       ╰── Int
                    ╰── Return
                        ╰── <27> FunctionCall [f]
                            ╰── <26> AddressOf
                                ╰── <25> Var [x]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_return_wrong_pointer_type() {
    let src = r#"
        int i;
        long *return_long_pointer(void) {
            return &i;
        }
        int main(void) {
            long *l = return_long_pointer();
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
            ├── Function [return_long_pointer]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <14> AddressOf
            │               ╰── <13> Var [i]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Long
                    │   ╰── Initializer
                    │       ╰── <29> FunctionCall [return_long_pointer]
                    ╰── Return
                        ╰── <32> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_ternary_mixed_pointer_types() {
    let src = r#"
        int main(void) {
            long *x = 0;
            int *y = 0;
            int *result = 1 ? x : y;
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
                    │   │   ╰── Pointer
                    │   │       ╰── Long
                    │   ╰── Initializer
                    │       ╰── <11> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── y
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <19> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── result
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <32> Conditional [?]
                    │           ├── <27> Constant Int [1]
                    │           ├── Then
                    │           │   ╰── <29> Var [x]
                    │           ╰── Else
                    │               ╰── <31> Var [y]
                    ╰── Return
                        ╰── <35> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_casts_cast_between_pointer_types() {
    let src = r#"
        int check_null_ptr_cast(void) {
            static long *long_ptr = 0;
            double *dbl_ptr = (double *)long_ptr;
            unsigned int *int_ptr = (unsigned int *)long_ptr;
            int **ptr_ptr = (int **)long_ptr;
            if (long_ptr) {
                return 1;
            }
            if (dbl_ptr) {
                return 2;
            }
            if (int_ptr) {
                return 3;
            }
            if (ptr_ptr) {
                return 4;
            }
            return 0;
        }
        int check_round_trip(void) {
            long l = -1;
            long *long_ptr = &l;
            double *dbl_ptr = (double *)long_ptr;
            long *other_long_ptr = (long *)dbl_ptr;
            if (*other_long_ptr != -1) {
                return 5;
            }
            return 0;
        }
        int main(void)
        {
            int result = check_null_ptr_cast();
            if (result) {
                return result;
            }
            result = check_round_trip();
            return result;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [check_null_ptr_cast]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── long_ptr
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Long
            │       │   ├── Initializer
            │       │   │   ╰── <12> Constant Int [0]
            │       │   ╰── Static
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── dbl_ptr
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Double
            │       │   ╰── Initializer
            │       │       ╰── <25> Cast
            │       │           ├── Target
            │       │           │   ╰── Pointer
            │       │           │       ╰── Double
            │       │           ╰── Expression
            │       │               ╰── <24> Var [long_ptr]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── int_ptr
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Unsigned Int
            │       │   ╰── Initializer
            │       │       ╰── <38> Cast
            │       │           ├── Target
            │       │           │   ╰── Pointer
            │       │           │       ╰── Unsigned Int
            │       │           ╰── Expression
            │       │               ╰── <37> Var [long_ptr]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ptr_ptr
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Pointer
            │       │   │           ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <54> Cast
            │       │           ├── Target
            │       │           │   ╰── Pointer
            │       │           │       ╰── Pointer
            │       │           │           ╰── Int
            │       │           ╰── Expression
            │       │               ╰── <53> Var [long_ptr]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <58> Var [long_ptr]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <59> Constant Int [1]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <65> Var [dbl_ptr]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <66> Constant Int [2]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <72> Var [int_ptr]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <73> Constant Int [3]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <79> Var [ptr_ptr]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <80> Constant Int [4]
            │       ╰── Return
            │           ╰── <85> Constant Int [0]
            ├── Function [check_round_trip]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── l
            │       │   ├── Type
            │       │   │   ╰── Long
            │       │   ╰── Initializer
            │       │       ╰── <99> Unary [-]
            │       │           ╰── <98> Constant Int [1]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── long_ptr
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Long
            │       │   ╰── Initializer
            │       │       ╰── <109> AddressOf
            │       │           ╰── <108> Var [l]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── dbl_ptr
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Double
            │       │   ╰── Initializer
            │       │       ╰── <122> Cast
            │       │           ├── Target
            │       │           │   ╰── Pointer
            │       │           │       ╰── Double
            │       │           ╰── Expression
            │       │               ╰── <121> Var [long_ptr]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── other_long_ptr
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Long
            │       │   ╰── Initializer
            │       │       ╰── <135> Cast
            │       │           ├── Target
            │       │           │   ╰── Pointer
            │       │           │       ╰── Long
            │       │           ╰── Expression
            │       │               ╰── <134> Var [dbl_ptr]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <145>  [!=]
            │       │   │       ├── <140> Dereference
            │       │   │       │   ╰── <139> Var [other_long_ptr]
            │       │   │       ╰── <144> Unary [-]
            │       │   │           ╰── <143> Constant Int [1]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <146> Constant Int [5]
            │       ╰── Return
            │           ╰── <151> Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── result
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <164> FunctionCall [check_null_ptr_cast]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <168> Var [result]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <170> Var [result]
                    ├── <180> Assign [=]
                    │   ├── <176> Var [result]
                    │   ╰── <179> FunctionCall [check_round_trip]
                    ╰── Return
                        ╰── <183> Var [result]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_casts_null_pointer_conversion() {
    let src = r#"
        double *d = 0l;
        int *i = 0ul;
        int *i2 = 0u;
        int expect_null_param(int *val)
        {
            return (val == 0ul);
        }
        long *return_null_ptr(void)
        {
            return 0;
        }
        int main(void)
        {
            int x = 10;
            int *ptr = &x;
            if (d) {
                return 1;
            }
            if (i) {
                return 2;
            }
            if (i2) {
                return 3;
            }
            ptr = 0ul;
            if (ptr) {
                return 4;
            }
            int *y = 0;
            if (y != 0)
                return 5;
            if (!expect_null_param(0)) {
                return 6;
            }
            long *null_ptr = return_null_ptr();
            if (null_ptr != 0) {
                return 7;
            }
            ptr = &x;
            int *ternary_result = 10 ? 0 : ptr;
            if (ternary_result) {
                return 8;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── d
            │   ├── Type
            │   │   ╰── Pointer
            │   │       ╰── Double
            │   ╰── Initializer
            │       ╰── <6> Constant Long [0]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── i
            │   ├── Type
            │   │   ╰── Pointer
            │   │       ╰── Int
            │   ╰── Initializer
            │       ╰── <14> Constant ULong [0]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── i2
            │   ├── Type
            │   │   ╰── Pointer
            │   │       ╰── Int
            │   ╰── Initializer
            │       ╰── <22> Constant UInt [0]
            ├── Function [expect_null_param]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── val
            │   │       ╰── Type
            │   │           ╰── Pointer
            │   │               ╰── Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <40>  [==]
            │               ├── <36> Var [val]
            │               ╰── <38> Constant ULong [0]
            ├── Function [return_null_ptr]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <51> Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <63> Constant Int [10]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <73> AddressOf
                    │           ╰── <72> Var [x]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <77> Var [d]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <78> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <84> Var [i]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <85> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <91> Var [i2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <92> Constant Int [3]
                    ├── <101> Assign [=]
                    │   ├── <98> Var [ptr]
                    │   ╰── <100> Constant ULong [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <104> Var [ptr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <105> Constant Int [4]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── y
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <115> Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <122>  [!=]
                    │   │       ├── <119> Var [y]
                    │   │       ╰── <121> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <123> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <130> Unary [!]
                    │   │       ╰── <129> FunctionCall [expect_null_param]
                    │   │           ╰── <128> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <131> Constant Int [6]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── null_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Long
                    │   ╰── Initializer
                    │       ╰── <142> FunctionCall [return_null_ptr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <149>  [!=]
                    │   │       ├── <146> Var [null_ptr]
                    │   │       ╰── <148> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <150> Constant Int [7]
                    ├── <161> Assign [=]
                    │   ├── <156> Var [ptr]
                    │   ╰── <160> AddressOf
                    │       ╰── <159> Var [x]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ternary_result
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <172> Conditional [?]
                    │           ├── <168> Constant Int [10]
                    │           ├── Then
                    │           │   ╰── <169> Constant Int [0]
                    │           ╰── Else
                    │               ╰── <171> Var [ptr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <176> Var [ternary_result]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <177> Constant Int [8]
                    ╰── Return
                        ╰── <182> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_casts_pointer_int_casts() {
    let src = r#"
        int i = 128;
        long l = 128l;
        int int_to_pointer(void) {
            int *a = (int *) i;
            int *b = (int *) l;
            return a == b;
        }
        int pointer_to_int(void) {
            static long l;
            long *ptr = &l;
            unsigned long ptr_as_long = (unsigned long) ptr;
            return (ptr_as_long % 8 == 0);
        }
        int cast_long_round_trip(void) {
            int *ptr = (int *) l;
            long l2 = (long) ptr;
            return (l == l2);
        }
        int cast_ulong_round_trip(void) {
            long *ptr = &l;
            unsigned long ptr_as_ulong = (unsigned long) ptr;
            long *ptr2 = (long *) ptr_as_ulong;
            return (ptr == ptr2);
        }
        int cast_int_round_trip(void) {
            double *a = (double *)i;
            int i2 = (int) a;
            return (i2 == 128);
        }
        int main(void) {
            if (!int_to_pointer()) {
                return 1;
            }
            if (!pointer_to_int()) {
                return 2;
            }
            if (!cast_long_round_trip()) {
                return 3;
            }
            if (!cast_ulong_round_trip()) {
                return 4;
            }
            if (!cast_int_round_trip()) {
                return 5;
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
            │       ╰── <4> Constant Int [128]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── l
            │   ├── Type
            │   │   ╰── Long
            │   ╰── Initializer
            │       ╰── <10> Constant Long [128]
            ├── Function [int_to_pointer]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── a
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <28> Cast
            │       │           ├── Target
            │       │           │   ╰── Pointer
            │       │           │       ╰── Int
            │       │           ╰── Expression
            │       │               ╰── <27> Var [i]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── b
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <41> Cast
            │       │           ├── Target
            │       │           │   ╰── Pointer
            │       │           │       ╰── Int
            │       │           ╰── Expression
            │       │               ╰── <40> Var [l]
            │       ╰── Return
            │           ╰── <49>  [==]
            │               ├── <45> Var [a]
            │               ╰── <48> Var [b]
            ├── Function [pointer_to_int]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── l
            │       │   ├── Type
            │       │   │   ╰── Long
            │       │   ╰── Static
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ptr
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Long
            │       │   ╰── Initializer
            │       │       ╰── <70> AddressOf
            │       │           ╰── <69> Var [l]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ptr_as_long
            │       │   ├── Type
            │       │   │   ╰── Unsigned Long
            │       │   ╰── Initializer
            │       │       ╰── <80> Cast
            │       │           ├── Target
            │       │           │   ╰── Unsigned Long
            │       │           ╰── Expression
            │       │               ╰── <79> Var [ptr]
            │       ╰── Return
            │           ╰── <91>  [==]
            │               ├── <87>  [%]
            │               │   ├── <84> Var [ptr_as_long]
            │               │   ╰── <86> Constant Int [8]
            │               ╰── <89> Constant Int [0]
            ├── Function [cast_long_round_trip]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ptr
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <110> Cast
            │       │           ├── Target
            │       │           │   ╰── Pointer
            │       │           │       ╰── Int
            │       │           ╰── Expression
            │       │               ╰── <109> Var [l]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── l2
            │       │   ├── Type
            │       │   │   ╰── Long
            │       │   ╰── Initializer
            │       │       ╰── <120> Cast
            │       │           ├── Target
            │       │           │   ╰── Long
            │       │           ╰── Expression
            │       │               ╰── <119> Var [ptr]
            │       ╰── Return
            │           ╰── <129>  [==]
            │               ├── <124> Var [l]
            │               ╰── <127> Var [l2]
            ├── Function [cast_ulong_round_trip]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ptr
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Long
            │       │   ╰── Initializer
            │       │       ╰── <145> AddressOf
            │       │           ╰── <144> Var [l]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ptr_as_ulong
            │       │   ├── Type
            │       │   │   ╰── Unsigned Long
            │       │   ╰── Initializer
            │       │       ╰── <155> Cast
            │       │           ├── Target
            │       │           │   ╰── Unsigned Long
            │       │           ╰── Expression
            │       │               ╰── <154> Var [ptr]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ptr2
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Long
            │       │   ╰── Initializer
            │       │       ╰── <168> Cast
            │       │           ├── Target
            │       │           │   ╰── Pointer
            │       │           │       ╰── Long
            │       │           ╰── Expression
            │       │               ╰── <167> Var [ptr_as_ulong]
            │       ╰── Return
            │           ╰── <177>  [==]
            │               ├── <172> Var [ptr]
            │               ╰── <175> Var [ptr2]
            ├── Function [cast_int_round_trip]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── a
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Double
            │       │   ╰── Initializer
            │       │       ╰── <196> Cast
            │       │           ├── Target
            │       │           │   ╰── Pointer
            │       │           │       ╰── Double
            │       │           ╰── Expression
            │       │               ╰── <195> Var [i]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── i2
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <206> Cast
            │       │           ├── Target
            │       │           │   ╰── Int
            │       │           ╰── Expression
            │       │               ╰── <205> Var [a]
            │       ╰── Return
            │           ╰── <214>  [==]
            │               ├── <210> Var [i2]
            │               ╰── <212> Constant Int [128]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <226> Unary [!]
                    │   │       ╰── <225> FunctionCall [int_to_pointer]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <227> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <235> Unary [!]
                    │   │       ╰── <234> FunctionCall [pointer_to_int]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <236> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <244> Unary [!]
                    │   │       ╰── <243> FunctionCall [cast_long_round_trip]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <245> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <253> Unary [!]
                    │   │       ╰── <252> FunctionCall [cast_ulong_round_trip]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <254> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <262> Unary [!]
                    │   │       ╰── <261> FunctionCall [cast_int_round_trip]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <263> Constant Int [5]
                    ╰── Return
                        ╰── <268> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_comparisons_compare_pointers() {
    let src = r#"
        int main(void) {
            int a = 0;
            int b;
            int *a_ptr = &a;
            int *a_ptr2 = &a;
            int *b_ptr = &b;
            if (a_ptr == b_ptr) {
                return 1;
            }
            if (a_ptr != a_ptr2) {
                return 2;
            }
            if (!(a_ptr == a_ptr2)) {
                return 3;
            }
            if (!(a_ptr != b_ptr)) {
                return 4;
            }
            *b_ptr = *a_ptr;
            if (a_ptr == b_ptr) {
                return 5;
            }
            b_ptr = a_ptr;
            if (b_ptr != a_ptr) {
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
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <9> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ╰── Type
                    │       ╰── Int
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <23> AddressOf
                    │           ╰── <22> Var [a]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a_ptr2
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <33> AddressOf
                    │           ╰── <32> Var [a]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <43> AddressOf
                    │           ╰── <42> Var [b]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <51>  [==]
                    │   │       ├── <47> Var [a_ptr]
                    │   │       ╰── <50> Var [b_ptr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <52> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <62>  [!=]
                    │   │       ├── <58> Var [a_ptr]
                    │   │       ╰── <61> Var [a_ptr2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <63> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <76> Unary [!]
                    │   │       ╰── <75>  [==]
                    │   │           ├── <70> Var [a_ptr]
                    │   │           ╰── <73> Var [a_ptr2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <77> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <90> Unary [!]
                    │   │       ╰── <89>  [!=]
                    │   │           ├── <84> Var [a_ptr]
                    │   │           ╰── <87> Var [b_ptr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <91> Constant Int [4]
                    ├── <103> Assign [=]
                    │   ├── <98> Dereference
                    │   │   ╰── <97> Var [b_ptr]
                    │   ╰── <102> Dereference
                    │       ╰── <101> Var [a_ptr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <110>  [==]
                    │   │       ├── <106> Var [a_ptr]
                    │   │       ╰── <109> Var [b_ptr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <111> Constant Int [5]
                    ├── <121> Assign [=]
                    │   ├── <117> Var [b_ptr]
                    │   ╰── <120> Var [a_ptr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <128>  [!=]
                    │   │       ├── <124> Var [b_ptr]
                    │   │       ╰── <127> Var [a_ptr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <129> Constant Int [6]
                    ╰── Return
                        ╰── <134> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_comparisons_compare_to_null() {
    let src = r#"
        double *get_null_pointer(void) {
            return 0;
        }
        int main(void)
        {
            double x;
            double *null = get_null_pointer();
            double *non_null = &x;
            if (non_null == 0) {
                return 1;
            }
            if (!(null == 0l)) {
                return 2;
            }
            if (!(non_null != 0u)) {
                return 3;
            }
            if (null != 0ul) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [get_null_pointer]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <8> Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ╰── Type
                    │       ╰── Double
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── null
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Double
                    │   ╰── Initializer
                    │       ╰── <27> FunctionCall [get_null_pointer]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── non_null
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Double
                    │   ╰── Initializer
                    │       ╰── <37> AddressOf
                    │           ╰── <36> Var [x]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <44>  [==]
                    │   │       ├── <41> Var [non_null]
                    │   │       ╰── <43> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <45> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <57> Unary [!]
                    │   │       ╰── <56>  [==]
                    │   │           ├── <52> Var [null]
                    │   │           ╰── <54> Constant Long [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <58> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <70> Unary [!]
                    │   │       ╰── <69>  [!=]
                    │   │           ├── <65> Var [non_null]
                    │   │           ╰── <67> Constant UInt [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <71> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <80>  [!=]
                    │   │       ├── <77> Var [null]
                    │   │       ╰── <79> Constant ULong [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <81> Constant Int [4]
                    ╰── Return
                        ╰── <86> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_comparisons_pointers_as_conditions() {
    let src = r#"
        long *get_null_pointer(void) {
            return 0;
        }
        int main(void)
        {
            long x;
            long *ptr = &x;
            long *null_ptr = get_null_pointer();
            if (5.0 && null_ptr) {
                return 1;
            }
            int a = 0;
            if (!(ptr || (a = 10))) {
                return 2;
            }
            if (a != 0) {
                return 3;
            }
            if (!ptr) {
                return 4;
            }
            int j = ptr ? 1 : 2;
            int k = null_ptr ? 3 : 4;
            if (j != 1) {
                return 5;
            }
            if (k != 4) {
                return 6;
            }
            int i = 0;
            while (ptr)
            {
                if (i >= 10) {
                    ptr = 0;
                    continue;
                }
                i = i + 1;
            }
            if (i != 10) {
                return 7;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [get_null_pointer]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <8> Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ╰── Type
                    │       ╰── Long
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Long
                    │   ╰── Initializer
                    │       ╰── <28> AddressOf
                    │           ╰── <27> Var [x]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── null_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Long
                    │   ╰── Initializer
                    │       ╰── <37> FunctionCall [get_null_pointer]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <44>  [&&]
                    │   │       ├── <40> Constant Double [+5e0]
                    │   │       ╰── <43> Var [null_ptr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <45> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <53> Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <68> Unary [!]
                    │   │       ╰── <67>  [||]
                    │   │           ├── <58> Var [ptr]
                    │   │           ╰── <65> Assign [=]
                    │   │               ├── <61> Var [a]
                    │   │               ╰── <63> Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <69> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <78>  [!=]
                    │   │       ├── <75> Var [a]
                    │   │       ╰── <77> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <79> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <87> Unary [!]
                    │   │       ╰── <86> Var [ptr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <88> Constant Int [4]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── j
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <100> Conditional [?]
                    │           ├── <97> Var [ptr]
                    │           ├── Then
                    │           │   ╰── <98> Constant Int [1]
                    │           ╰── Else
                    │               ╰── <99> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── k
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <110> Conditional [?]
                    │           ├── <107> Var [null_ptr]
                    │           ├── Then
                    │           │   ╰── <108> Constant Int [3]
                    │           ╰── Else
                    │               ╰── <109> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <117>  [!=]
                    │   │       ├── <114> Var [j]
                    │   │       ╰── <116> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <118> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <127>  [!=]
                    │   │       ├── <124> Var [k]
                    │   │       ╰── <126> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <128> Constant Int [6]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <136> Constant Int [0]
                    ├── While
                    │   ├── Condition
                    │   │   ╰── <140> Var [ptr]
                    │   ╰── Body
                    │       ╰── Block
                    │           ├── If
                    │           │   ├── Condition
                    │           │   │   ╰── <145>  [>=]
                    │           │   │       ├── <142> Var [i]
                    │           │   │       ╰── <144> Constant Int [10]
                    │           │   ╰── Then
                    │           │       ╰── Block
                    │           │           ├── <150> Assign [=]
                    │           │           │   ├── <147> Var [ptr]
                    │           │           │   ╰── <149> Constant Int [0]
                    │           │           ╰── Continue
                    │           ╰── <164> Assign [=]
                    │               ├── <157> Var [i]
                    │               ╰── <163>  [+]
                    │                   ├── <160> Var [i]
                    │                   ╰── <162> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <173>  [!=]
                    │   │       ├── <170> Var [i]
                    │   │       ╰── <172> Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <174> Constant Int [7]
                    ╰── Return
                        ╰── <179> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_declarators_abstract_declarators() {
    let src = r#"
        
        int main(void) {
            long int unsigned *x = 0;
            if (x != (unsigned long (*)) 0)
                return 1;
            if (x != (long unsigned int ((((*))))) 0)
                return 2;
            double ***y = 0;
            if (y != (double *(**)) 0)
                return 3;
            if (y != (double (***)) 0)
                return 4;
            if ((double (*(*(*)))) 0 != y)
                return 5;
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
                    │   │   ╰── Pointer
                    │   │       ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <11> Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <23>  [!=]
                    │   │       ├── <15> Var [x]
                    │   │       ╰── <22> Cast
                    │   │           ├── Target
                    │   │           │   ╰── Pointer
                    │   │           │       ╰── Unsigned Long
                    │   │           ╰── Expression
                    │   │               ╰── <21> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <24> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <39>  [!=]
                    │   │       ├── <28> Var [x]
                    │   │       ╰── <38> Cast
                    │   │           ├── Target
                    │   │           │   ╰── Pointer
                    │   │           │       ╰── Unsigned Long
                    │   │           ╰── Expression
                    │   │               ╰── <37> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <40> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── y
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Pointer
                    │   │           ╰── Pointer
                    │   │               ╰── Double
                    │   ╰── Initializer
                    │       ╰── <52> Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <66>  [!=]
                    │   │       ├── <56> Var [y]
                    │   │       ╰── <65> Cast
                    │   │           ├── Target
                    │   │           │   ╰── Pointer
                    │   │           │       ╰── Pointer
                    │   │           │           ╰── Pointer
                    │   │           │               ╰── Double
                    │   │           ╰── Expression
                    │   │               ╰── <64> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <67> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <81>  [!=]
                    │   │       ├── <71> Var [y]
                    │   │       ╰── <80> Cast
                    │   │           ├── Target
                    │   │           │   ╰── Pointer
                    │   │           │       ╰── Pointer
                    │   │           │           ╰── Pointer
                    │   │           │               ╰── Double
                    │   │           ╰── Expression
                    │   │               ╰── <79> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <82> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <98>  [!=]
                    │   │       ├── <94> Cast
                    │   │       │   ├── Target
                    │   │       │   │   ╰── Pointer
                    │   │       │   │       ╰── Pointer
                    │   │       │   │           ╰── Pointer
                    │   │       │   │               ╰── Double
                    │   │       │   ╰── Expression
                    │   │       │       ╰── <93> Constant Int [0]
                    │   │       ╰── <97> Var [y]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <99> Constant Int [5]
                    ╰── Return
                        ╰── <102> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_declarators_declarators() {
    let src = r#"
        int return_3(void);
        int(return_3(void));
        int(return_3)(void);
        int((return_3))(void)
        {
            return 3;
        }
        long l = 100;
        long *two_pointers(double val, double *ptr)
        {
            *ptr = val;
            return &l;
        }
        long(*two_pointers(double val, double(*d)));
        long *(two_pointers)(double val, double *(d));
        long *(two_pointers)(double val, double(*(d)));
        unsigned **pointers_to_pointers(int **p)
        {
            static unsigned u;
            static unsigned *u_ptr;
            u_ptr = &u;
            u = **p;
            return &u_ptr;
        }
        unsigned(**(pointers_to_pointers(int *(*p))));
        unsigned *(*pointers_to_pointers(int(**p)));
        unsigned(*(*((pointers_to_pointers)(int(*(*(p)))))));
        int main(void)
        {
            int i = 0;
            int(*i_ptr) = &i;
            int(**ptr_to_iptr) = &i_ptr;
            double(d1) = 0.0;
            double d2 = 10.0;
            double *(d_ptr) = &d1;
            long(*(l_ptr));
            unsigned *(*(ptr_to_uptr));
            i = return_3();
            if (i != 3)
                return 1;
            if (*i_ptr != 3) {
                return 2;
            }
            l_ptr = two_pointers(d2, d_ptr);
            if (l_ptr != &l) {
                return 3;
            }
            if (*l_ptr != 100) {
                return 4;
            }
            if (*d_ptr != 10.0) {
                return 5;
            }
            if (d1 != 10.0) {
                return 6;
            }
            ptr_to_uptr = pointers_to_pointers(ptr_to_iptr);
            if (**ptr_to_uptr != 3) {
                return 7;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [return_3]
            ├── Function [return_3]
            ├── Function [return_3]
            ├── Function [return_3]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <28> Constant Int [3]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── l
            │   ├── Type
            │   │   ╰── Long
            │   ╰── Initializer
            │       ╰── <35> Constant Int [100]
            ├── Function [two_pointers]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── val
            │   │   │   ╰── Type
            │   │   │       ╰── Double
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── ptr
            │   │       ╰── Type
            │   │           ╰── Pointer
            │   │               ╰── Double
            │   ╰── Body
            │       ├── <59> Assign [=]
            │       │   ├── <55> Dereference
            │       │   │   ╰── <54> Var [ptr]
            │       │   ╰── <58> Var [val]
            │       ╰── Return
            │           ╰── <63> AddressOf
            │               ╰── <62> Var [l]
            ├── Function [two_pointers]
            │   ╰── Parameters
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── val
            │       │   ╰── Type
            │       │       ╰── Double
            │       ╰── Param
            │           ├── Name
            │           │   ╰── d
            │           ╰── Type
            │               ╰── Pointer
            │                   ╰── Double
            ├── Function [two_pointers]
            │   ╰── Parameters
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── val
            │       │   ╰── Type
            │       │       ╰── Double
            │       ╰── Param
            │           ├── Name
            │           │   ╰── d
            │           ╰── Type
            │               ╰── Pointer
            │                   ╰── Double
            ├── Function [two_pointers]
            │   ╰── Parameters
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── val
            │       │   ╰── Type
            │       │       ╰── Double
            │       ╰── Param
            │           ├── Name
            │           │   ╰── d
            │           ╰── Type
            │               ╰── Pointer
            │                   ╰── Double
            ├── Function [pointers_to_pointers]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── p
            │   │       ╰── Type
            │   │           ╰── Pointer
            │   │               ╰── Pointer
            │   │                   ╰── Int
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── u
            │       │   ├── Type
            │       │   │   ╰── Unsigned Int
            │       │   ╰── Static
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── u_ptr
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Unsigned Int
            │       │   ╰── Static
            │       ├── <156> Assign [=]
            │       │   ├── <151> Var [u_ptr]
            │       │   ╰── <155> AddressOf
            │       │       ╰── <154> Var [u]
            │       ├── <165> Assign [=]
            │       │   ├── <159> Var [u]
            │       │   ╰── <164> Dereference
            │       │       ╰── <163> Dereference
            │       │           ╰── <162> Var [p]
            │       ╰── Return
            │           ╰── <169> AddressOf
            │               ╰── <168> Var [u_ptr]
            ├── Function [pointers_to_pointers]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── p
            │           ╰── Type
            │               ╰── Pointer
            │                   ╰── Pointer
            │                       ╰── Int
            ├── Function [pointers_to_pointers]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── p
            │           ╰── Type
            │               ╰── Pointer
            │                   ╰── Pointer
            │                       ╰── Int
            ├── Function [pointers_to_pointers]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── p
            │           ╰── Type
            │               ╰── Pointer
            │                   ╰── Pointer
            │                       ╰── Int
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <244> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <255> AddressOf
                    │           ╰── <254> Var [i]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr_to_iptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Pointer
                    │   │           ╰── Int
                    │   ╰── Initializer
                    │       ╰── <268> AddressOf
                    │           ╰── <267> Var [i_ptr]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d1
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <275> Constant Double [+0e0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d2
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <281> Constant Double [+1e1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Double
                    │   ╰── Initializer
                    │       ╰── <292> AddressOf
                    │           ╰── <291> Var [d1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l_ptr
                    │   ╰── Type
                    │       ╰── Pointer
                    │           ╰── Long
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr_to_uptr
                    │   ╰── Type
                    │       ╰── Pointer
                    │           ╰── Pointer
                    │               ╰── Unsigned Int
                    ├── <318> Assign [=]
                    │   ├── <314> Var [i]
                    │   ╰── <317> FunctionCall [return_3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <324>  [!=]
                    │   │       ├── <321> Var [i]
                    │   │       ╰── <323> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <325> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <333>  [!=]
                    │   │       ├── <330> Dereference
                    │   │       │   ╰── <329> Var [i_ptr]
                    │   │       ╰── <332> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <334> Constant Int [2]
                    ├── <348> Assign [=]
                    │   ├── <340> Var [l_ptr]
                    │   ╰── <347> FunctionCall [two_pointers]
                    │       ├── <344> Var [d2]
                    │       ╰── <346> Var [d_ptr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <356>  [!=]
                    │   │       ├── <351> Var [l_ptr]
                    │   │       ╰── <355> AddressOf
                    │   │           ╰── <354> Var [l]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <357> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <367>  [!=]
                    │   │       ├── <364> Dereference
                    │   │       │   ╰── <363> Var [l_ptr]
                    │   │       ╰── <366> Constant Int [100]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <368> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <378>  [!=]
                    │   │       ├── <375> Dereference
                    │   │       │   ╰── <374> Var [d_ptr]
                    │   │       ╰── <377> Constant Double [+1e1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <379> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <388>  [!=]
                    │   │       ├── <385> Var [d1]
                    │   │       ╰── <387> Constant Double [+1e1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <389> Constant Int [6]
                    ├── <401> Assign [=]
                    │   ├── <395> Var [ptr_to_uptr]
                    │   ╰── <400> FunctionCall [pointers_to_pointers]
                    │       ╰── <399> Var [ptr_to_iptr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <409>  [!=]
                    │   │       ├── <406> Dereference
                    │   │       │   ╰── <405> Dereference
                    │   │       │       ╰── <404> Var [ptr_to_uptr]
                    │   │       ╰── <408> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <410> Constant Int [7]
                    ╰── Return
                        ╰── <415> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_declarators_declare_pointer_in_for_loop() {
    let src = r#"
        int main(void) {
            int x = 10;
            for (int *i = &x; i != 0; ) {
                *i = 5;
                i = 0;
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
                    │       ╰── <9> Constant Int [10]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Pointer
                    │   │       │       ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <19> AddressOf
                    │   │               ╰── <18> Var [x]
                    │   ├── Condition
                    │   │   ╰── <27>  [!=]
                    │   │       ├── <24> Var [i]
                    │   │       ╰── <26> Constant Int [0]
                    │   ╰── Block
                    │       ├── <33> Assign [=]
                    │       │   ├── <30> Dereference
                    │       │   │   ╰── <29> Var [i]
                    │       │   ╰── <32> Constant Int [5]
                    │       ╰── <39> Assign [=]
                    │           ├── <36> Var [i]
                    │           ╰── <38> Constant Int [0]
                    ╰── Return
                        ╰── <45> Var [x]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_dereference_address_of_dereference() {
    let src = r#"
        int main(void) {
            int *null_ptr = 0;
            if (&*null_ptr != 0)
                return 1;
            int **ptr_to_null = &null_ptr;
            if (&**ptr_to_null)
                return 2;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── null_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <11> Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <20>  [!=]
                    │   │       ├── <17> AddressOf
                    │   │       │   ╰── <16> Dereference
                    │   │       │       ╰── <15> Var [null_ptr]
                    │   │       ╰── <19> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <21> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr_to_null
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Pointer
                    │   │           ╰── Int
                    │   ╰── Initializer
                    │       ╰── <33> AddressOf
                    │           ╰── <32> Var [null_ptr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <40> AddressOf
                    │   │       ╰── <39> Dereference
                    │   │           ╰── <38> Dereference
                    │   │               ╰── <37> Var [ptr_to_null]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <41> Constant Int [2]
                    ╰── Return
                        ╰── <44> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_dereference_dereference_expression_result() {
    let src = r#"
        int *return_pointer(void) {
            static int var = 10;
            return &var;
        }
        int one = 1;
        int main(void) {
            int val = 100;
            int *ptr_var = &val;
            if (*return_pointer() != 10) {
                return 1;
            }
            if (*(one ? return_pointer() : ptr_var) != 10)
                return 2;
            if (*(one - 1 ? return_pointer() : ptr_var) != 100) {
                return 3;
            }
            int *ptr_to_one = &one;
            if (*(ptr_var = ptr_to_one) != 1) {
                return 4;
            }
            *return_pointer() = 20;
            *(one ? ptr_var : return_pointer()) = 30;
            if (*return_pointer() != 20) {
                return 5;
            }
            if (*ptr_var != 30) {
                return 6;
            }
            if (one != 30) {
                return 7;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [return_pointer]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── var
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ├── Initializer
            │       │   │   ╰── <12> Constant Int [10]
            │       │   ╰── Static
            │       ╰── Return
            │           ╰── <17> AddressOf
            │               ╰── <16> Var [var]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── one
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Initializer
            │       ╰── <24> Constant Int [1]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── val
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <35> Constant Int [100]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr_var
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <45> AddressOf
                    │           ╰── <44> Var [val]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <53>  [!=]
                    │   │       ├── <50> Dereference
                    │   │       │   ╰── <49> FunctionCall [return_pointer]
                    │   │       ╰── <52> Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <54> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <70>  [!=]
                    │   │       ├── <67> Dereference
                    │   │       │   ╰── <66> Conditional [?]
                    │   │       │       ├── <60> Var [one]
                    │   │       │       ├── Then
                    │   │       │       │   ╰── <62> FunctionCall [return_pointer]
                    │   │       │       ╰── Else
                    │   │       │           ╰── <64> Var [ptr_var]
                    │   │       ╰── <69> Constant Int [10]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <71> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <88>  [!=]
                    │   │       ├── <85> Dereference
                    │   │       │   ╰── <84> Conditional [?]
                    │   │       │       ├── <78>  [-]
                    │   │       │       │   ├── <75> Var [one]
                    │   │       │       │   ╰── <77> Constant Int [1]
                    │   │       │       ├── Then
                    │   │       │       │   ╰── <80> FunctionCall [return_pointer]
                    │   │       │       ╰── Else
                    │   │       │           ╰── <82> Var [ptr_var]
                    │   │       ╰── <87> Constant Int [100]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <89> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr_to_one
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <101> AddressOf
                    │           ╰── <100> Var [one]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <114>  [!=]
                    │   │       ├── <111> Dereference
                    │   │       │   ╰── <110> Assign [=]
                    │   │       │       ├── <105> Var [ptr_var]
                    │   │       │       ╰── <108> Var [ptr_to_one]
                    │   │       ╰── <113> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <115> Constant Int [4]
                    ├── <125> Assign [=]
                    │   ├── <122> Dereference
                    │   │   ╰── <121> FunctionCall [return_pointer]
                    │   ╰── <124> Constant Int [20]
                    ├── <138> Assign [=]
                    │   ├── <135> Dereference
                    │   │   ╰── <134> Conditional [?]
                    │   │       ├── <128> Var [one]
                    │   │       ├── Then
                    │   │       │   ╰── <130> Var [ptr_var]
                    │   │       ╰── Else
                    │   │           ╰── <132> FunctionCall [return_pointer]
                    │   ╰── <137> Constant Int [30]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <145>  [!=]
                    │   │       ├── <142> Dereference
                    │   │       │   ╰── <141> FunctionCall [return_pointer]
                    │   │       ╰── <144> Constant Int [20]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <146> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <156>  [!=]
                    │   │       ├── <153> Dereference
                    │   │       │   ╰── <152> Var [ptr_var]
                    │   │       ╰── <155> Constant Int [30]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <157> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <166>  [!=]
                    │   │       ├── <163> Var [one]
                    │   │       ╰── <165> Constant Int [30]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <167> Constant Int [7]
                    ╰── Return
                        ╰── <172> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_dereference_multilevel_indirection() {
    let src = r#"
        
        int main(void) {
            double d = 10.0;
            double *d_ptr = &d;
            double **d_ptr_ptr = &d_ptr;
            double ***d_ptr_ptr_ptr = &d_ptr_ptr;
            if (d != 10.0) {
                return 1;
            }
            if (*d_ptr != 10.0) {
                return 2;
            }
            if (**d_ptr_ptr != 10.0) {
                return 3;
            }
            if (***d_ptr_ptr_ptr != 10.0) {
                return 4;
            }
            if (&d != d_ptr) {
                return 5;
            }
            if (*d_ptr_ptr != d_ptr) {
                return 6;
            }
            if (**d_ptr_ptr_ptr != d_ptr) {
                return 7;
            }
            ***d_ptr_ptr_ptr = 5.0;
            if (d != 5.0) {
                return 8;
            }
            if (*d_ptr != 5.0) {
                return 9;
            }
            if (**d_ptr_ptr != 5.0) {
                return 10;
            }
            if (***d_ptr_ptr_ptr != 5.0) {
                return 11;
            }
            double d2 = 1.0;
            double *d2_ptr = &d2;
            double *d2_ptr2 = d2_ptr;
            double **d2_ptr_ptr = &d2_ptr;
            *d_ptr_ptr_ptr = d2_ptr_ptr;
            if (**d_ptr_ptr_ptr != d2_ptr) {
                return 12;
            }
            if (***d_ptr_ptr_ptr != 1.0) {
                return 13;
            }
            if (d2_ptr_ptr == &d2_ptr2)
                return 14;
            d2_ptr = d_ptr;
            if (**d_ptr_ptr_ptr != d_ptr) {
                return 15;
            }
            if (*d2_ptr_ptr != d_ptr) {
                return 16;
            }
            if (**d_ptr_ptr_ptr == d2_ptr2) {
                return 17;
            }
            if (***d_ptr_ptr_ptr != 5.0) {
                return 18;
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
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <9> Constant Double [+1e1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Double
                    │   ╰── Initializer
                    │       ╰── <19> AddressOf
                    │           ╰── <18> Var [d]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d_ptr_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Pointer
                    │   │           ╰── Double
                    │   ╰── Initializer
                    │       ╰── <31> AddressOf
                    │           ╰── <30> Var [d_ptr]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d_ptr_ptr_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Pointer
                    │   │           ╰── Pointer
                    │   │               ╰── Double
                    │   ╰── Initializer
                    │       ╰── <45> AddressOf
                    │           ╰── <44> Var [d_ptr_ptr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <52>  [!=]
                    │   │       ├── <49> Var [d]
                    │   │       ╰── <51> Constant Double [+1e1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <53> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <63>  [!=]
                    │   │       ├── <60> Dereference
                    │   │       │   ╰── <59> Var [d_ptr]
                    │   │       ╰── <62> Constant Double [+1e1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <64> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <75>  [!=]
                    │   │       ├── <72> Dereference
                    │   │       │   ╰── <71> Dereference
                    │   │       │       ╰── <70> Var [d_ptr_ptr]
                    │   │       ╰── <74> Constant Double [+1e1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <76> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <88>  [!=]
                    │   │       ├── <85> Dereference
                    │   │       │   ╰── <84> Dereference
                    │   │       │       ╰── <83> Dereference
                    │   │       │           ╰── <82> Var [d_ptr_ptr_ptr]
                    │   │       ╰── <87> Constant Double [+1e1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <89> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <100>  [!=]
                    │   │       ├── <96> AddressOf
                    │   │       │   ╰── <95> Var [d]
                    │   │       ╰── <99> Var [d_ptr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <101> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <112>  [!=]
                    │   │       ├── <108> Dereference
                    │   │       │   ╰── <107> Var [d_ptr_ptr]
                    │   │       ╰── <111> Var [d_ptr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <113> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <125>  [!=]
                    │   │       ├── <121> Dereference
                    │   │       │   ╰── <120> Dereference
                    │   │       │       ╰── <119> Var [d_ptr_ptr_ptr]
                    │   │       ╰── <124> Var [d_ptr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <126> Constant Int [7]
                    ├── <138> Assign [=]
                    │   ├── <135> Dereference
                    │   │   ╰── <134> Dereference
                    │   │       ╰── <133> Dereference
                    │   │           ╰── <132> Var [d_ptr_ptr_ptr]
                    │   ╰── <137> Constant Double [+5e0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <144>  [!=]
                    │   │       ├── <141> Var [d]
                    │   │       ╰── <143> Constant Double [+5e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <145> Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <155>  [!=]
                    │   │       ├── <152> Dereference
                    │   │       │   ╰── <151> Var [d_ptr]
                    │   │       ╰── <154> Constant Double [+5e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <156> Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <167>  [!=]
                    │   │       ├── <164> Dereference
                    │   │       │   ╰── <163> Dereference
                    │   │       │       ╰── <162> Var [d_ptr_ptr]
                    │   │       ╰── <166> Constant Double [+5e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <168> Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <180>  [!=]
                    │   │       ├── <177> Dereference
                    │   │       │   ╰── <176> Dereference
                    │   │       │       ╰── <175> Dereference
                    │   │       │           ╰── <174> Var [d_ptr_ptr_ptr]
                    │   │       ╰── <179> Constant Double [+5e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <181> Constant Int [11]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d2
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <189> Constant Double [+1e0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d2_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Double
                    │   ╰── Initializer
                    │       ╰── <199> AddressOf
                    │           ╰── <198> Var [d2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d2_ptr2
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Double
                    │   ╰── Initializer
                    │       ╰── <208> Var [d2_ptr]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d2_ptr_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Pointer
                    │   │           ╰── Double
                    │   ╰── Initializer
                    │       ╰── <220> AddressOf
                    │           ╰── <219> Var [d2_ptr]
                    ├── <229> Assign [=]
                    │   ├── <225> Dereference
                    │   │   ╰── <224> Var [d_ptr_ptr_ptr]
                    │   ╰── <228> Var [d2_ptr_ptr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <238>  [!=]
                    │   │       ├── <234> Dereference
                    │   │       │   ╰── <233> Dereference
                    │   │       │       ╰── <232> Var [d_ptr_ptr_ptr]
                    │   │       ╰── <237> Var [d2_ptr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <239> Constant Int [12]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <251>  [!=]
                    │   │       ├── <248> Dereference
                    │   │       │   ╰── <247> Dereference
                    │   │       │       ╰── <246> Dereference
                    │   │       │           ╰── <245> Var [d_ptr_ptr_ptr]
                    │   │       ╰── <250> Constant Double [+1e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <252> Constant Int [13]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <263>  [==]
                    │   │       ├── <258> Var [d2_ptr_ptr]
                    │   │       ╰── <262> AddressOf
                    │   │           ╰── <261> Var [d2_ptr2]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <264> Constant Int [14]
                    ├── <272> Assign [=]
                    │   ├── <268> Var [d2_ptr]
                    │   ╰── <271> Var [d_ptr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <281>  [!=]
                    │   │       ├── <277> Dereference
                    │   │       │   ╰── <276> Dereference
                    │   │       │       ╰── <275> Var [d_ptr_ptr_ptr]
                    │   │       ╰── <280> Var [d_ptr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <282> Constant Int [15]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <293>  [!=]
                    │   │       ├── <289> Dereference
                    │   │       │   ╰── <288> Var [d2_ptr_ptr]
                    │   │       ╰── <292> Var [d_ptr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <294> Constant Int [16]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <306>  [==]
                    │   │       ├── <302> Dereference
                    │   │       │   ╰── <301> Dereference
                    │   │       │       ╰── <300> Var [d_ptr_ptr_ptr]
                    │   │       ╰── <305> Var [d2_ptr2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <307> Constant Int [17]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <319>  [!=]
                    │   │       ├── <316> Dereference
                    │   │       │   ╰── <315> Dereference
                    │   │       │       ╰── <314> Dereference
                    │   │       │           ╰── <313> Var [d_ptr_ptr_ptr]
                    │   │       ╰── <318> Constant Double [+5e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <320> Constant Int [18]
                    ╰── Return
                        ╰── <325> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_dereference_read_through_pointers() {
    let src = r#"
        
        int main(void) {
            int i = -100;
            unsigned long ul = 13835058055282163712ul;
            double d = 3.5;
            int *i_ptr = &i;
            unsigned long *ul_ptr = &ul;
            double *d_ptr = &d;
            if (*i_ptr != -100) {
                return 1;
            }
            if (*ul_ptr != 13835058055282163712ul) {
                return 2;
            }
            if (*d_ptr != 3.5) {
                return 3;
            }
            i = 12;
            ul = 1000;
            d = -000.001;
            if (*i_ptr != 12) {
                return 4;
            }
            if (*ul_ptr != 1000) {
                return 5;
            }
            if (*d_ptr != -000.001) {
                return 6;
            }
            int i2 = 1;
            unsigned long ul2 = 144115196665790464ul;
            double d2 = -33.3;
            i_ptr = &i2;
            ul_ptr = &ul2;
            d_ptr = &d2;
            if (*i_ptr != 1) {
                return 7;
            }
            if (*ul_ptr != 144115196665790464ul) {
                return 8;
            }
            if (*d_ptr != -33.3) {
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
                    │       ╰── <11> Unary [-]
                    │           ╰── <10> Constant Int [100]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ul
                    │   ├── Type
                    │   │   ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <17> Constant ULong [13835058055282163712]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <23> Constant Double [+3.5e0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <33> AddressOf
                    │           ╰── <32> Var [i]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ul_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <43> AddressOf
                    │           ╰── <42> Var [ul]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Double
                    │   ╰── Initializer
                    │       ╰── <53> AddressOf
                    │           ╰── <52> Var [d]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <63>  [!=]
                    │   │       ├── <58> Dereference
                    │   │       │   ╰── <57> Var [i_ptr]
                    │   │       ╰── <62> Unary [-]
                    │   │           ╰── <61> Constant Int [100]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <64> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <74>  [!=]
                    │   │       ├── <71> Dereference
                    │   │       │   ╰── <70> Var [ul_ptr]
                    │   │       ╰── <73> Constant ULong [13835058055282163712]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <75> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <85>  [!=]
                    │   │       ├── <82> Dereference
                    │   │       │   ╰── <81> Var [d_ptr]
                    │   │       ╰── <84> Constant Double [+3.5e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <86> Constant Int [3]
                    ├── <95> Assign [=]
                    │   ├── <92> Var [i]
                    │   ╰── <94> Constant Int [12]
                    ├── <101> Assign [=]
                    │   ├── <98> Var [ul]
                    │   ╰── <100> Constant Int [1000]
                    ├── <109> Assign [=]
                    │   ├── <104> Var [d]
                    │   ╰── <108> Unary [-]
                    │       ╰── <107> Constant Double [+1e-3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <116>  [!=]
                    │   │       ├── <113> Dereference
                    │   │       │   ╰── <112> Var [i_ptr]
                    │   │       ╰── <115> Constant Int [12]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <117> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <127>  [!=]
                    │   │       ├── <124> Dereference
                    │   │       │   ╰── <123> Var [ul_ptr]
                    │   │       ╰── <126> Constant Int [1000]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <128> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <140>  [!=]
                    │   │       ├── <135> Dereference
                    │   │       │   ╰── <134> Var [d_ptr]
                    │   │       ╰── <139> Unary [-]
                    │   │           ╰── <138> Constant Double [+1e-3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <141> Constant Int [6]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i2
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <149> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ul2
                    │   ├── Type
                    │   │   ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <155> Constant ULong [144115196665790464]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d2
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <163> Unary [-]
                    │           ╰── <162> Constant Double [+3.33e1]
                    ├── <172> Assign [=]
                    │   ├── <167> Var [i_ptr]
                    │   ╰── <171> AddressOf
                    │       ╰── <170> Var [i2]
                    ├── <180> Assign [=]
                    │   ├── <175> Var [ul_ptr]
                    │   ╰── <179> AddressOf
                    │       ╰── <178> Var [ul2]
                    ├── <188> Assign [=]
                    │   ├── <183> Var [d_ptr]
                    │   ╰── <187> AddressOf
                    │       ╰── <186> Var [d2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <195>  [!=]
                    │   │       ├── <192> Dereference
                    │   │       │   ╰── <191> Var [i_ptr]
                    │   │       ╰── <194> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <196> Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <206>  [!=]
                    │   │       ├── <203> Dereference
                    │   │       │   ╰── <202> Var [ul_ptr]
                    │   │       ╰── <205> Constant ULong [144115196665790464]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <207> Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <219>  [!=]
                    │   │       ├── <214> Dereference
                    │   │       │   ╰── <213> Var [d_ptr]
                    │   │       ╰── <218> Unary [-]
                    │   │           ╰── <217> Constant Double [+3.33e1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <220> Constant Int [9]
                    ╰── Return
                        ╰── <225> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_dereference_simple() {
    let src = r#"
        int main(void) {
            int x = 3;
            int *ptr = &x;
            return *ptr;
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
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <19> AddressOf
                    │           ╰── <18> Var [x]
                    ╰── Return
                        ╰── <24> Dereference
                            ╰── <23> Var [ptr]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_dereference_static_var_indirection() {
    let src = r#"
        unsigned int w = 4294967295U;
        int x = 10;
        unsigned int y = 4294967295U;
        double *dbl_ptr;
        long modify_ptr(long *new_ptr) {
            static long *p;
            if (new_ptr)
            {
                p = new_ptr;
            }
            return *p;
        }
        int increment_ptr(void)
        {
            *dbl_ptr = *dbl_ptr + 5.0;
            return 0;
        }
        int main(void) {
            int *pointer_to_static = &x;
            x = 20;
            if (*pointer_to_static != 20) {
                return 1;
            }
            *pointer_to_static = 100;
            if (x != 100) {
                return 2;
            }
            if (w != 4294967295U) {
                return 3;
            }
            if (y != 4294967295U) {
                return 4;
            }
            if (dbl_ptr) {
                return 5;
            }
            long l = 1000l;
            if (modify_ptr(&l) != 1000l) {
                return 6;
            }
            l = -1;
            if (modify_ptr(0) != l) {
                return 7;
            }
            double d = 10.0;
            dbl_ptr = &d;
            increment_ptr();
            if (*dbl_ptr != 15) {
                return 8;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── w
            │   ├── Type
            │   │   ╰── Unsigned Int
            │   ╰── Initializer
            │       ╰── <4> Constant UInt [4294967295]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── x
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Initializer
            │       ╰── <10> Constant Int [10]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── y
            │   ├── Type
            │   │   ╰── Unsigned Int
            │   ╰── Initializer
            │       ╰── <16> Constant UInt [4294967295]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── dbl_ptr
            │   ╰── Type
            │       ╰── Pointer
            │           ╰── Double
            ├── Function [modify_ptr]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── new_ptr
            │   │       ╰── Type
            │   │           ╰── Pointer
            │   │               ╰── Long
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── p
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Long
            │       │   ╰── Static
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <43> Var [new_ptr]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── <49> Assign [=]
            │       │               ├── <45> Var [p]
            │       │               ╰── <48> Var [new_ptr]
            │       ╰── Return
            │           ╰── <56> Dereference
            │               ╰── <55> Var [p]
            ├── Function [increment_ptr]
            │   ╰── Body
            │       ├── <75> Assign [=]
            │       │   ├── <67> Dereference
            │       │   │   ╰── <66> Var [dbl_ptr]
            │       │   ╰── <74>  [+]
            │       │       ├── <71> Dereference
            │       │       │   ╰── <70> Var [dbl_ptr]
            │       │       ╰── <73> Constant Double [+5e0]
            │       ╰── Return
            │           ╰── <77> Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── pointer_to_static
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <93> AddressOf
                    │           ╰── <92> Var [x]
                    ├── <100> Assign [=]
                    │   ├── <97> Var [x]
                    │   ╰── <99> Constant Int [20]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <107>  [!=]
                    │   │       ├── <104> Dereference
                    │   │       │   ╰── <103> Var [pointer_to_static]
                    │   │       ╰── <106> Constant Int [20]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <108> Constant Int [1]
                    ├── <118> Assign [=]
                    │   ├── <115> Dereference
                    │   │   ╰── <114> Var [pointer_to_static]
                    │   ╰── <117> Constant Int [100]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <124>  [!=]
                    │   │       ├── <121> Var [x]
                    │   │       ╰── <123> Constant Int [100]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <125> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <134>  [!=]
                    │   │       ├── <131> Var [w]
                    │   │       ╰── <133> Constant UInt [4294967295]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <135> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <144>  [!=]
                    │   │       ├── <141> Var [y]
                    │   │       ╰── <143> Constant UInt [4294967295]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <145> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <151> Var [dbl_ptr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <152> Constant Int [5]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <160> Constant Long [1000]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <170>  [!=]
                    │   │       ├── <167> FunctionCall [modify_ptr]
                    │   │       │   ╰── <166> AddressOf
                    │   │       │       ╰── <165> Var [l]
                    │   │       ╰── <169> Constant Long [1000]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <171> Constant Int [6]
                    ├── <182> Assign [=]
                    │   ├── <177> Var [l]
                    │   ╰── <181> Unary [-]
                    │       ╰── <180> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <190>  [!=]
                    │   │       ├── <186> FunctionCall [modify_ptr]
                    │   │       │   ╰── <185> Constant Int [0]
                    │   │       ╰── <189> Var [l]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <191> Constant Int [7]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <199> Constant Double [+1e1]
                    ├── <208> Assign [=]
                    │   ├── <203> Var [dbl_ptr]
                    │   ╰── <207> AddressOf
                    │       ╰── <206> Var [d]
                    ├── <211> FunctionCall [increment_ptr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <218>  [!=]
                    │   │       ├── <215> Dereference
                    │   │       │   ╰── <214> Var [dbl_ptr]
                    │   │       ╰── <217> Constant Int [15]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <219> Constant Int [8]
                    ╰── Return
                        ╰── <224> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_dereference_update_through_pointers() {
    let src = r#"
        int main(void) {
            unsigned int i = 2185232384u;
            signed long l = 144115196665790464l;
            double d = 1e50;
            unsigned *i_ptr = &i;
            long *l_ptr = &l;
            double *d_ptr = &d;
            *i_ptr = 10;
            *l_ptr = -20;
            *d_ptr = 30.1;
            if (i != 10) {
                return 1;
            }
            if (l != -20) {
                return 2;
            }
            if (d != 30.1) {
                return 3;
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
                    │   │   ╰── Unsigned Int
                    │   ╰── Initializer
                    │       ╰── <9> Constant UInt [2185232384]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <15> Constant Long [144115196665790464]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <21> Constant Double [+1e50]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Unsigned Int
                    │   ╰── Initializer
                    │       ╰── <31> AddressOf
                    │           ╰── <30> Var [i]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Long
                    │   ╰── Initializer
                    │       ╰── <41> AddressOf
                    │           ╰── <40> Var [l]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Double
                    │   ╰── Initializer
                    │       ╰── <51> AddressOf
                    │           ╰── <50> Var [d]
                    ├── <59> Assign [=]
                    │   ├── <56> Dereference
                    │   │   ╰── <55> Var [i_ptr]
                    │   ╰── <58> Constant Int [10]
                    ├── <68> Assign [=]
                    │   ├── <63> Dereference
                    │   │   ╰── <62> Var [l_ptr]
                    │   ╰── <67> Unary [-]
                    │       ╰── <66> Constant Int [20]
                    ├── <75> Assign [=]
                    │   ├── <72> Dereference
                    │   │   ╰── <71> Var [d_ptr]
                    │   ╰── <74> Constant Double [+3.01e1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <81>  [!=]
                    │   │       ├── <78> Var [i]
                    │   │       ╰── <80> Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <82> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <93>  [!=]
                    │   │       ├── <88> Var [l]
                    │   │       ╰── <92> Unary [-]
                    │   │           ╰── <91> Constant Int [20]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <94> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <103>  [!=]
                    │   │       ├── <100> Var [d]
                    │   │       ╰── <102> Constant Double [+3.01e1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <104> Constant Int [3]
                    ╰── Return
                        ╰── <109> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_bitshift_dereferenced_ptrs() {
    let src = r#"
        unsigned int ui = 4294967295;
        unsigned int *get_ui_ptr(void){
            return &ui;
        }
        int shiftcount = 5;
        int main(void) {
            if ((*get_ui_ptr() << 2l) != 4294967292) {
                return 1;
            }
            if ((*get_ui_ptr() >> 2) != 1073741823) {
                return 2;
            }
            int *shiftcount_ptr = &shiftcount;
            if ((1000000u >> *shiftcount_ptr) != 31250) {
                return 3;
            }
            if ((1000000u << *shiftcount_ptr) != 32000000) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── ui
            │   ├── Type
            │   │   ╰── Unsigned Int
            │   ╰── Initializer
            │       ╰── <4> Constant Long [4294967295]
            ├── Function [get_ui_ptr]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <16> AddressOf
            │               ╰── <15> Var [ui]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── shiftcount
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Initializer
            │       ╰── <23> Constant Int [5]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <40>  [!=]
                    │   │       ├── <37>  [<<]
                    │   │       │   ├── <33> Dereference
                    │   │       │   │   ╰── <32> FunctionCall [get_ui_ptr]
                    │   │       │   ╰── <35> Constant Long [2]
                    │   │       ╰── <39> Constant Long [4294967292]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <41> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <55>  [!=]
                    │   │       ├── <52>  [>>]
                    │   │       │   ├── <48> Dereference
                    │   │       │   │   ╰── <47> FunctionCall [get_ui_ptr]
                    │   │       │   ╰── <50> Constant Int [2]
                    │   │       ╰── <54> Constant Int [1073741823]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <56> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── shiftcount_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <68> AddressOf
                    │           ╰── <67> Var [shiftcount]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <80>  [!=]
                    │   │       ├── <77>  [>>]
                    │   │       │   ├── <71> Constant UInt [1000000]
                    │   │       │   ╰── <75> Dereference
                    │   │       │       ╰── <74> Var [shiftcount_ptr]
                    │   │       ╰── <79> Constant Int [31250]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <81> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <95>  [!=]
                    │   │       ├── <92>  [<<]
                    │   │       │   ├── <86> Constant UInt [1000000]
                    │   │       │   ╰── <90> Dereference
                    │   │       │       ╰── <89> Var [shiftcount_ptr]
                    │   │       ╰── <94> Constant Int [32000000]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <96> Constant Int [4]
                    ╰── Return
                        ╰── <101> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_bitwise_ops_with_dereferenced_ptrs() {
    let src = r#"
        int main(void) {
            unsigned int ui = -1u;
            unsigned long ul = 9223372036854775808ul;
            unsigned int *ui_ptr = &ui;
            unsigned long *ul_ptr = &ul;
            if ((*ui_ptr & *ul_ptr) != 0) {
                return 1;
            }
            if ((*ui_ptr | *ul_ptr) != 9223372041149743103ul) {
                return 2;
            }
            int i = -1;
            signed int *i_ptr = &i;
            if ((*i_ptr & ul) != *ul_ptr) {
                return 3;
            }
            if ((*i_ptr | *ul_ptr) != i) {
                return 4;
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
                    │   │   ╰── ui
                    │   ├── Type
                    │   │   ╰── Unsigned Int
                    │   ╰── Initializer
                    │       ╰── <11> Unary [-]
                    │           ╰── <10> Constant UInt [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ul
                    │   ├── Type
                    │   │   ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <17> Constant ULong [9223372036854775808]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ui_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Unsigned Int
                    │   ╰── Initializer
                    │       ╰── <27> AddressOf
                    │           ╰── <26> Var [ui]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ul_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <37> AddressOf
                    │           ╰── <36> Var [ul]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <51>  [!=]
                    │   │       ├── <48>  [&]
                    │   │       │   ├── <42> Dereference
                    │   │       │   │   ╰── <41> Var [ui_ptr]
                    │   │       │   ╰── <46> Dereference
                    │   │       │       ╰── <45> Var [ul_ptr]
                    │   │       ╰── <50> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <52> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <68>  [!=]
                    │   │       ├── <65>  [|]
                    │   │       │   ├── <59> Dereference
                    │   │       │   │   ╰── <58> Var [ui_ptr]
                    │   │       │   ╰── <63> Dereference
                    │   │       │       ╰── <62> Var [ul_ptr]
                    │   │       ╰── <67> Constant ULong [9223372041149743103]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <69> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <79> Unary [-]
                    │           ╰── <78> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <89> AddressOf
                    │           ╰── <88> Var [i]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <104>  [!=]
                    │   │       ├── <99>  [&]
                    │   │       │   ├── <94> Dereference
                    │   │       │   │   ╰── <93> Var [i_ptr]
                    │   │       │   ╰── <97> Var [ul]
                    │   │       ╰── <103> Dereference
                    │   │           ╰── <102> Var [ul_ptr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <105> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <122>  [!=]
                    │   │       ├── <118>  [|]
                    │   │       │   ├── <112> Dereference
                    │   │       │   │   ╰── <111> Var [i_ptr]
                    │   │       │   ╰── <116> Dereference
                    │   │       │       ╰── <115> Var [ul_ptr]
                    │   │       ╰── <121> Var [i]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <123> Constant Int [4]
                    ╰── Return
                        ╰── <128> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_compound_assign_conversion() {
    let src = r#"
        int main(void) {
            double d = 5.0;
            double *d_ptr = &d;
            *d_ptr *= 1000u;
            if (d != 5000.0) {
                return 1;
            }
            int i = -50;
            int *i_ptr = &i;
            *i_ptr %= 4294967200U;
            if (*i_ptr != 46) {
                return 2;
            }
            unsigned int ui = 4294967295U;
            ui /= *d_ptr;
            if (ui != 858993u) {
                return 3;
            }
            i = -10;
            unsigned long ul = 9223372036854775807ul;
            unsigned long *ul_ptr = &ul;
            *i_ptr -= *ul_ptr;
            if (i != -9) {
                return 4;
            }
            if (ul != 9223372036854775807ul) {
                return 5;
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
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <9> Constant Double [+5e0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Double
                    │   ╰── Initializer
                    │       ╰── <19> AddressOf
                    │           ╰── <18> Var [d]
                    ├── <27> Assign [*=]
                    │   ├── <24> Dereference
                    │   │   ╰── <23> Var [d_ptr]
                    │   ╰── <26> Constant UInt [1000]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <33>  [!=]
                    │   │       ├── <30> Var [d]
                    │   │       ╰── <32> Constant Double [+5e3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <34> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <44> Unary [-]
                    │           ╰── <43> Constant Int [50]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <54> AddressOf
                    │           ╰── <53> Var [i]
                    ├── <62> Assign [%=]
                    │   ├── <59> Dereference
                    │   │   ╰── <58> Var [i_ptr]
                    │   ╰── <61> Constant UInt [4294967200]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <69>  [!=]
                    │   │       ├── <66> Dereference
                    │   │       │   ╰── <65> Var [i_ptr]
                    │   │       ╰── <68> Constant Int [46]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <70> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ui
                    │   ├── Type
                    │   │   ╰── Unsigned Int
                    │   ╰── Initializer
                    │       ╰── <78> Constant UInt [4294967295]
                    ├── <87> Assign [/=]
                    │   ├── <82> Var [ui]
                    │   ╰── <86> Dereference
                    │       ╰── <85> Var [d_ptr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <93>  [!=]
                    │   │       ├── <90> Var [ui]
                    │   │       ╰── <92> Constant UInt [858993]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <94> Constant Int [3]
                    ├── <105> Assign [=]
                    │   ├── <100> Var [i]
                    │   ╰── <104> Unary [-]
                    │       ╰── <103> Constant Int [10]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ul
                    │   ├── Type
                    │   │   ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <110> Constant ULong [9223372036854775807]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ul_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <120> AddressOf
                    │           ╰── <119> Var [ul]
                    ├── <130> Assign [-=]
                    │   ├── <125> Dereference
                    │   │   ╰── <124> Var [i_ptr]
                    │   ╰── <129> Dereference
                    │       ╰── <128> Var [ul_ptr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <138>  [!=]
                    │   │       ├── <133> Var [i]
                    │   │       ╰── <137> Unary [-]
                    │   │           ╰── <136> Constant Int [9]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <139> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <148>  [!=]
                    │   │       ├── <145> Var [ul]
                    │   │       ╰── <147> Constant ULong [9223372036854775807]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <149> Constant Int [5]
                    ╰── Return
                        ╰── <154> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_compound_assign_through_pointer() {
    let src = r#"
        int main(void) {
            int x = 10;
            int *ptr = &x;
            *ptr += 5;
            if (x != 15) {
                return 1;
            }
            if ((*ptr -= 12) != 3) {
                return 2;
            }
            if (x != 3) {
                return 3;
            }
            *ptr *= 6;
            if (x != 18) {
                return 4;
            }
            *ptr /= 9;
            if (x != 2) {
                return 5;
            }
            *ptr %= 3;
            if (x != 2) {
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
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <9> Constant Int [10]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <19> AddressOf
                    │           ╰── <18> Var [x]
                    ├── <27> Assign [+=]
                    │   ├── <24> Dereference
                    │   │   ╰── <23> Var [ptr]
                    │   ╰── <26> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <33>  [!=]
                    │   │       ├── <30> Var [x]
                    │   │       ╰── <32> Constant Int [15]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <34> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <48>  [!=]
                    │   │       ├── <45> Assign [-=]
                    │   │       │   ├── <41> Dereference
                    │   │       │   │   ╰── <40> Var [ptr]
                    │   │       │   ╰── <43> Constant Int [12]
                    │   │       ╰── <47> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <49> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <58>  [!=]
                    │   │       ├── <55> Var [x]
                    │   │       ╰── <57> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <59> Constant Int [3]
                    ├── <69> Assign [*=]
                    │   ├── <66> Dereference
                    │   │   ╰── <65> Var [ptr]
                    │   ╰── <68> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <75>  [!=]
                    │   │       ├── <72> Var [x]
                    │   │       ╰── <74> Constant Int [18]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <76> Constant Int [4]
                    ├── <86> Assign [/=]
                    │   ├── <83> Dereference
                    │   │   ╰── <82> Var [ptr]
                    │   ╰── <85> Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <92>  [!=]
                    │   │       ├── <89> Var [x]
                    │   │       ╰── <91> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <93> Constant Int [5]
                    ├── <103> Assign [%=]
                    │   ├── <100> Dereference
                    │   │   ╰── <99> Var [ptr]
                    │   ╰── <102> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <109>  [!=]
                    │   │       ├── <106> Var [x]
                    │   │       ╰── <108> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <110> Constant Int [6]
                    ╰── Return
                        ╰── <115> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_compound_bitwise_dereferenced_ptrs() {
    let src = r#"
        unsigned long ul = 18446460386757245432ul;
        int main(void) {
            unsigned long *ul_ptr = &ul;
            *ul_ptr &= -1000;
            if (ul != 18446460386757244952ul ) {
                return 1;
            }
            *ul_ptr |= 4294967040u;
            if (ul != 18446460386824683288ul ) {
                return 2;
            }
            int i = 123456;
            unsigned int ui = 4042322160u;
            long l = -252645136;
            unsigned int *ui_ptr = &ui;
            long *l_ptr = &l;
            if (*ui_ptr ^= *l_ptr) {
                return 3;
            }
            if (ui) {
                return 4;
            }
            if (i != 123456) {
                return 5;
            }
            if (l != -252645136) {
                return 6;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── ul
            │   ├── Type
            │   │   ╰── Unsigned Long
            │   ╰── Initializer
            │       ╰── <4> Constant ULong [18446460386757245432]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ul_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <19> AddressOf
                    │           ╰── <18> Var [ul]
                    ├── <29> Assign [&=]
                    │   ├── <24> Dereference
                    │   │   ╰── <23> Var [ul_ptr]
                    │   ╰── <28> Unary [-]
                    │       ╰── <27> Constant Int [1000]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <35>  [!=]
                    │   │       ├── <32> Var [ul]
                    │   │       ╰── <34> Constant ULong [18446460386757244952]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <36> Constant Int [1]
                    ├── <46> Assign [|=]
                    │   ├── <43> Dereference
                    │   │   ╰── <42> Var [ul_ptr]
                    │   ╰── <45> Constant UInt [4294967040]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <52>  [!=]
                    │   │       ├── <49> Var [ul]
                    │   │       ╰── <51> Constant ULong [18446460386824683288]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <53> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <61> Constant Int [123456]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ui
                    │   ├── Type
                    │   │   ╰── Unsigned Int
                    │   ╰── Initializer
                    │       ╰── <67> Constant UInt [4042322160]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <75> Unary [-]
                    │           ╰── <74> Constant Int [252645136]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ui_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Unsigned Int
                    │   ╰── Initializer
                    │       ╰── <85> AddressOf
                    │           ╰── <84> Var [ui]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Long
                    │   ╰── Initializer
                    │       ╰── <95> AddressOf
                    │           ╰── <94> Var [l]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <105> Assign [^=]
                    │   │       ├── <100> Dereference
                    │   │       │   ╰── <99> Var [ui_ptr]
                    │   │       ╰── <104> Dereference
                    │   │           ╰── <103> Var [l_ptr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <106> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <112> Var [ui]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <113> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <122>  [!=]
                    │   │       ├── <119> Var [i]
                    │   │       ╰── <121> Constant Int [123456]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <123> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <134>  [!=]
                    │   │       ├── <129> Var [l]
                    │   │       ╰── <133> Unary [-]
                    │   │           ╰── <132> Constant Int [252645136]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <135> Constant Int [6]
                    ╰── Return
                        ╰── <140> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_eval_compound_lhs_once() {
    let src = r#"
        int i = 0;
        int putchar(int c);
        int *print_A(void) {
            putchar(65);
            return &i;
        }
        int *print_B(void) {
            putchar(66);
            return &i;
        }
        int main(void) {
            *print_A() += 5;
            if (i != 5) {
                return 1;
            }
            *print_B() += 5l;
            if (i != 10) {
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
            ├── Function [putchar]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── c
            │           ╰── Type
            │               ╰── Int
            ├── Function [print_A]
            │   ╰── Body
            │       ├── <25> FunctionCall [putchar]
            │       │   ╰── <24> Constant Int [65]
            │       ╰── Return
            │           ╰── <29> AddressOf
            │               ╰── <28> Var [i]
            ├── Function [print_B]
            │   ╰── Body
            │       ├── <42> FunctionCall [putchar]
            │       │   ╰── <41> Constant Int [66]
            │       ╰── Return
            │           ╰── <46> AddressOf
            │               ╰── <45> Var [i]
            ╰── Function [main]
                ╰── Body
                    ├── <60> Assign [+=]
                    │   ├── <57> Dereference
                    │   │   ╰── <56> FunctionCall [print_A]
                    │   ╰── <59> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <66>  [!=]
                    │   │       ├── <63> Var [i]
                    │   │       ╰── <65> Constant Int [5]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <67> Constant Int [1]
                    ├── <77> Assign [+=]
                    │   ├── <74> Dereference
                    │   │   ╰── <73> FunctionCall [print_B]
                    │   ╰── <76> Constant Long [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <83>  [!=]
                    │   │       ├── <80> Var [i]
                    │   │       ╰── <82> Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <84> Constant Int [2]
                    ╰── Return
                        ╰── <89> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_incr_and_decr_through_pointer() {
    let src = r#"
        
        int main(void) {
            int x = 10;
            int *y = &x;
            if (++*y != 11) {
                return 1;
            }
            if (x != 11) {
                return 2;
            }
            if (--*y != 10) {
                return 3;
            }
            if (x != 10) {
                return 4;
            }
            if ((*y)++ != 10) {
                return 5;
            }
            if (x != 11) {
                return 6;
            }
            if ((*y)-- != 11) {
                return 7;
            }
            if (x != 10) {
                return 8;
            }
            unsigned long ul = 0;
            unsigned long *ul_ptr = &ul;
            if ((*ul_ptr)--) {
                return 9;
            }
            if (ul != 18446744073709551615UL) {
                return 10;
            }
            double d = 0.0;
            double *d_ptr = &d;
            if (++(*d_ptr) != 1.0) {
                return 11;
            }
            if (d != 1.0) {
                return 12;
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
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <9> Constant Int [10]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── y
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <19> AddressOf
                    │           ╰── <18> Var [x]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <29>  [!=]
                    │   │       ├── <26> Unary [++]
                    │   │       │   ╰── <25> Dereference
                    │   │       │       ╰── <24> Var [y]
                    │   │       ╰── <28> Constant Int [11]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <30> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <39>  [!=]
                    │   │       ├── <36> Var [x]
                    │   │       ╰── <38> Constant Int [11]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <40> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <52>  [!=]
                    │   │       ├── <49> Unary [--]
                    │   │       │   ╰── <48> Dereference
                    │   │       │       ╰── <47> Var [y]
                    │   │       ╰── <51> Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <53> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <62>  [!=]
                    │   │       ├── <59> Var [x]
                    │   │       ╰── <61> Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <63> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <76>  [!=]
                    │   │       ├── <73> Postfix [++]
                    │   │       │   ╰── <71> Dereference
                    │   │       │       ╰── <69> Var [y]
                    │   │       ╰── <75> Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <77> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <86>  [!=]
                    │   │       ├── <83> Var [x]
                    │   │       ╰── <85> Constant Int [11]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <87> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <100>  [!=]
                    │   │       ├── <97> Postfix [--]
                    │   │       │   ╰── <95> Dereference
                    │   │       │       ╰── <93> Var [y]
                    │   │       ╰── <99> Constant Int [11]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <101> Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <110>  [!=]
                    │   │       ├── <107> Var [x]
                    │   │       ╰── <109> Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <111> Constant Int [8]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ul
                    │   ├── Type
                    │   │   ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <119> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ul_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <129> AddressOf
                    │           ╰── <128> Var [ul]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <137> Postfix [--]
                    │   │       ╰── <135> Dereference
                    │   │           ╰── <133> Var [ul_ptr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <138> Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <147>  [!=]
                    │   │       ├── <144> Var [ul]
                    │   │       ╰── <146> Constant ULong [18446744073709551615]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <148> Constant Int [10]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <156> Constant Double [+0e0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Double
                    │   ╰── Initializer
                    │       ╰── <166> AddressOf
                    │           ╰── <165> Var [d]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <177>  [!=]
                    │   │       ├── <174> Unary [++]
                    │   │       │   ╰── <173> Dereference
                    │   │       │       ╰── <171> Var [d_ptr]
                    │   │       ╰── <176> Constant Double [+1e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <178> Constant Int [11]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <187>  [!=]
                    │   │       ├── <184> Var [d]
                    │   │       ╰── <186> Constant Double [+1e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <188> Constant Int [12]
                    ╰── Return
                        ╰── <193> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_switch_dereferenced_pointer() {
    let src = r#"
        long l = 4294967300l;
        long *get_ptr(void) {
            return &l;
        }
        int main(void) {
            switch (*get_ptr()) {
                case 1:
                    return 1;
                case 4:
                    return 2;
                case 4294967300l:
                    return 0;
                case 18446744073709551600UL:
                    return 3;
                default:
                    return 4;
            }
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── l
            │   ├── Type
            │   │   ╰── Long
            │   ╰── Initializer
            │       ╰── <4> Constant Long [4294967300]
            ├── Function [get_ptr]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <16> AddressOf
            │               ╰── <15> Var [l]
            ╰── Function [main]
                ╰── Body
                    ╰── Switch
                        ├── Expression
                        │   ╰── <27> Dereference
                        │       ╰── <26> FunctionCall [get_ptr]
                        ╰── Block
                            ├── Case [1]
                            │   ╰── Return
                            │       ╰── <29> Constant Int [1]
                            ├── Case [4]
                            │   ╰── Return
                            │       ╰── <33> Constant Int [2]
                            ├── Case [4294967300]
                            │   ╰── Return
                            │       ╰── <37> Constant Int [0]
                            ├── Case [-16]
                            │   ╰── Return
                            │       ╰── <41> Constant Int [3]
                            ╰── Default
                                ╰── Return
                                    ╰── <44> Constant Int [4]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_function_calls_address_of_argument() {
    let src = r#"
        int addr_of_arg(int a) {
            int *ptr = &a;
            *ptr = 10;
            return a;
        }
        int main(void) {
            int result = addr_of_arg(-20);
            if (result != 10) {
                return 1;
            }
            int var = 100;
            result = addr_of_arg(var);
            if (result != 10) {
                return 2;
            }
            if (var != 100) {
                return 3;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [addr_of_arg]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── a
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ptr
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <16> AddressOf
            │       │           ╰── <15> Var [a]
            │       ├── <24> Assign [=]
            │       │   ├── <21> Dereference
            │       │   │   ╰── <20> Var [ptr]
            │       │   ╰── <23> Constant Int [10]
            │       ╰── Return
            │           ╰── <27> Var [a]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── result
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <43> FunctionCall [addr_of_arg]
                    │           ╰── <42> Unary [-]
                    │               ╰── <41> Constant Int [20]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <50>  [!=]
                    │   │       ├── <47> Var [result]
                    │   │       ╰── <49> Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <51> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── var
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <59> Constant Int [100]
                    ├── <69> Assign [=]
                    │   ├── <63> Var [result]
                    │   ╰── <68> FunctionCall [addr_of_arg]
                    │       ╰── <67> Var [var]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <75>  [!=]
                    │   │       ├── <72> Var [result]
                    │   │       ╰── <74> Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <76> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <85>  [!=]
                    │   │       ├── <82> Var [var]
                    │   │       ╰── <84> Constant Int [100]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <86> Constant Int [3]
                    ╰── Return
                        ╰── <91> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_function_calls_return_pointer() {
    let src = r#"
        
        int *return_pointer(int *in) {
            return in;
        }
        int main(void) {
            int x = 10;
            int *x_ptr = return_pointer(&x);
            if (*x_ptr != 10)
                return 1;
            x = 100;
            if (*x_ptr != 100)
                return 2;
            if (x_ptr != &x)
                return 3;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [return_pointer]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── in
            │   │       ╰── Type
            │   │           ╰── Pointer
            │   │               ╰── Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <14> Var [in]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <26> Constant Int [10]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <38> FunctionCall [return_pointer]
                    │           ╰── <37> AddressOf
                    │               ╰── <36> Var [x]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <46>  [!=]
                    │   │       ├── <43> Dereference
                    │   │       │   ╰── <42> Var [x_ptr]
                    │   │       ╰── <45> Constant Int [10]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <47> Constant Int [1]
                    ├── <54> Assign [=]
                    │   ├── <51> Var [x]
                    │   ╰── <53> Constant Int [100]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <61>  [!=]
                    │   │       ├── <58> Dereference
                    │   │       │   ╰── <57> Var [x_ptr]
                    │   │       ╰── <60> Constant Int [100]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <62> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <71>  [!=]
                    │   │       ├── <66> Var [x_ptr]
                    │   │       ╰── <70> AddressOf
                    │   │           ╰── <69> Var [x]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <72> Constant Int [3]
                    ╰── Return
                        ╰── <75> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_function_calls_update_value_through_pointer_parameter() {
    let src = r#"
        
        int update_value(int *ptr) {
            int old_val = *ptr;
            *ptr = 10;
            return old_val;
        }
        int main(void) {
            int x = 20;
            int result = update_value(&x);
            if (result != 20) {
                return 1;
            }
            if (x != 10) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [update_value]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── ptr
            │   │       ╰── Type
            │   │           ╰── Pointer
            │   │               ╰── Int
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── old_val
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <16> Dereference
            │       │           ╰── <15> Var [ptr]
            │       ├── <24> Assign [=]
            │       │   ├── <21> Dereference
            │       │   │   ╰── <20> Var [ptr]
            │       │   ╰── <23> Constant Int [10]
            │       ╰── Return
            │           ╰── <27> Var [old_val]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <39> Constant Int [20]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── result
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <49> FunctionCall [update_value]
                    │           ╰── <48> AddressOf
                    │               ╰── <47> Var [x]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <56>  [!=]
                    │   │       ├── <53> Var [result]
                    │   │       ╰── <55> Constant Int [20]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <57> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <66>  [!=]
                    │   │       ├── <63> Var [x]
                    │   │       ╰── <65> Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <67> Constant Int [2]
                    ╰── Return
                        ╰── <72> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_global_pointer() {
    let src = r#"
        double *d_ptr;
        int update_thru_ptr(double new_val) {
            *d_ptr = new_val;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── d_ptr
            │   ╰── Type
            │       ╰── Pointer
            │           ╰── Double
            ╰── Function [update_thru_ptr]
                ├── Parameters
                │   ╰── Param
                │       ├── Name
                │       │   ╰── new_val
                │       ╰── Type
                │           ╰── Double
                ╰── Body
                    ├── <21> Assign [=]
                    │   ├── <17> Dereference
                    │   │   ╰── <16> Var [d_ptr]
                    │   ╰── <20> Var [new_val]
                    ╰── Return
                        ╰── <23> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_global_pointer_client() {
    let src = r#"
        extern double *d_ptr;
        int update_thru_ptr(double new_val);
        int main(void) {
            double d = 0.0;
            d_ptr = &d;
            update_thru_ptr(10.0);
            return (d == 10.0);
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── d_ptr
            │   ├── Type
            │   │   ╰── Pointer
            │   │       ╰── Double
            │   ╰── Extern
            ├── Function [update_thru_ptr]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── new_val
            │           ╰── Type
            │               ╰── Double
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <25> Constant Double [+0e0]
                    ├── <34> Assign [=]
                    │   ├── <29> Var [d_ptr]
                    │   ╰── <33> AddressOf
                    │       ╰── <32> Var [d]
                    ├── <38> FunctionCall [update_thru_ptr]
                    │   ╰── <37> Constant Double [+1e1]
                    ╰── Return
                        ╰── <45>  [==]
                            ├── <41> Var [d]
                            ╰── <43> Constant Double [+1e1]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_static_pointer() {
    let src = r#"
        static long *long_ptr;
        long *get_pointer(void) {
            return long_ptr;
        }
        int set_pointer(long *new_ptr) {
            long_ptr = new_ptr;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── long_ptr
            │   ├── Type
            │   │   ╰── Pointer
            │   │       ╰── Long
            │   ╰── Static
            ├── Function [get_pointer]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <16> Var [long_ptr]
            ╰── Function [set_pointer]
                ├── Parameters
                │   ╰── Param
                │       ├── Name
                │       │   ╰── new_ptr
                │       ╰── Type
                │           ╰── Pointer
                │               ╰── Long
                ╰── Body
                    ├── <35> Assign [=]
                    │   ├── <31> Var [long_ptr]
                    │   ╰── <34> Var [new_ptr]
                    ╰── Return
                        ╰── <37> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_static_pointer_client() {
    let src = r#"
        long *get_pointer(void);
        int set_pointer(long *new_ptr);
        static long private_long = 100l;
        int main(void) {
            long *initial_ptr = get_pointer();
            if (initial_ptr) {
                return 1;
            }
            set_pointer(&private_long);
            long *new_ptr = get_pointer();
            if (initial_ptr == new_ptr) {
                return 2;
            }
            if (*new_ptr != 100l) {
                return 3;
            }
            if (new_ptr != &private_long) {
                return 4;
            }
            set_pointer(0);
            if (get_pointer()) {
                return 5;
            }
            if (new_ptr != &private_long) {
                return 6;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [get_pointer]
            ├── Function [set_pointer]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── new_ptr
            │           ╰── Type
            │               ╰── Pointer
            │                   ╰── Long
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── private_long
            │   ├── Type
            │   │   ╰── Long
            │   ├── Initializer
            │   │   ╰── <24> Constant Long [100]
            │   ╰── Static
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── initial_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Long
                    │   ╰── Initializer
                    │       ╰── <38> FunctionCall [get_pointer]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <42> Var [initial_ptr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <43> Constant Int [1]
                    ├── <52> FunctionCall [set_pointer]
                    │   ╰── <51> AddressOf
                    │       ╰── <50> Var [private_long]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── new_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Long
                    │   ╰── Initializer
                    │       ╰── <60> FunctionCall [get_pointer]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <68>  [==]
                    │   │       ├── <64> Var [initial_ptr]
                    │   │       ╰── <67> Var [new_ptr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <69> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <79>  [!=]
                    │   │       ├── <76> Dereference
                    │   │       │   ╰── <75> Var [new_ptr]
                    │   │       ╰── <78> Constant Long [100]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <80> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <91>  [!=]
                    │   │       ├── <86> Var [new_ptr]
                    │   │       ╰── <90> AddressOf
                    │   │           ╰── <89> Var [private_long]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <92> Constant Int [4]
                    ├── <99> FunctionCall [set_pointer]
                    │   ╰── <98> Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <102> FunctionCall [get_pointer]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <103> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <114>  [!=]
                    │   │       ├── <109> Var [new_ptr]
                    │   │       ╰── <113> AddressOf
                    │   │           ╰── <112> Var [private_long]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <115> Constant Int [6]
                    ╰── Return
                        ╰── <120> Constant Int [0]
    "#;
    assert_parse(src, expected);
}
