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
                    │       ╰── Constant Int [0]
                    ├── Label [lbl]
                    │   ╰── <16> Assign [=]
                    │       ├── <13> Var [x]
                    │       ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <24>  [==]
                    │   │       ├── <21> AddressOf
                    │   │       │   ╰── <20> Var [lbl]
                    │   │       ╰── Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── Goto [lbl]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │   ╰── <8> Dereference
                    │       ╰── <7> Var [lbl]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │       ╰── Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── y
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <17> AddressOf
                    │           ╰── <16> Var [x]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── z
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Pointer
                    │   │           ╰── Int
                    │   ╰── Initializer
                    │       ╰── <29> AddressOf
                    │           ╰── <28> AddressOf
                    │               ╰── <26> Var [x]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │       ╰── Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── y
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <28> AddressOf
                    │           ╰── <27> Assign [=]
                    │               ├── <22> Var [x]
                    │               ╰── <25> Var [y]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │       ╰── <10> AddressOf
                    │           ╰── Constant Int [10]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │       ╰── Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── y
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── z
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <35> AddressOf
                    │           ╰── <{node_id}> Conditional [?]
                    │               ├── <28> Var [x]
                    │               ├── Then
                    │               │   ╰── <30> Var [y]
                    │               ╰── Else
                    │                   ╰── <32> Var [z]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    ├── <14> Assign [=]
                    │   ├── <11> Var [x]
                    │   ╰── Constant Int [10]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │       ╰── Constant Int [0]
                    ╰── VarDeclaration
                        ├── Name
                        │   ╰── ptr
                        ├── Type
                        │   ╰── Pointer
                        │       ╰── Int
                        ╰── Initializer
                            ╰── <16> Var [x]
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
                    │       ╰── Constant Int [0]
                    ╰── <16> Assign [=]
                        ├── <13> AddressOf
                        │   ╰── <12> Var [x]
                        ╰── Constant Int [10]
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
                    │       ╰── Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Long
                    │   ╰── Initializer
                    │       ╰── Constant Int [0]
                    ├── <24> Assign [=]
                    │   ├── <20> Var [l]
                    │   ╰── <23> Var [d]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │       ╰── Constant Double [+0e0]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │       ╰── Constant Double [+0e0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <20> Cast
                    │           ├── Target
                    │           │   ╰── Pointer
                    │           │       ╰── Int
                    │           ╰── Expression
                    │               ╰── <19> Var [d]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │       ╰── <17> Cast
                    │           ├── Target
                    │           │   ╰── Double
                    │           ╰── Expression
                    │               ╰── <16> Var [x]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │       ╰── Constant ULong [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── y
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Unsigned Int
                    │   ╰── Initializer
                    │       ╰── Constant ULong [0]
                    ╰── Return
                        ╰── <24>  [==]
                            ├── <20> Var [x]
                            ╰── <23> Var [y]
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
                    │       ╰── Constant ULong [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ul
                    │   ├── Type
                    │   │   ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── Constant ULong [0]
                    ╰── Return
                        ╰── <23>  [==]
                            ├── <19> Var [ptr]
                            ╰── <22> Var [ul]
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
                    │       ╰── Constant Int [0]
                    ╰── Return
                        ╰── <18> Cast
                            ├── Target
                            │   ╰── Int
                            ╰── Expression
                                ╰── <17> Unary [~]
                                    ╰── <16> Var [x]
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
                    │       ╰── Constant ULong [100]
                    ╰── Return
                        ╰── <13> Dereference
                            ╰── <12> Var [l]
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
                    │       ╰── Constant Int [10]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── y
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <17> AddressOf
                    │           ╰── <16> Var [x]
                    ├── <25>  [/]
                    │   ├── <21> Var [y]
                    │   ╰── Constant Int [8]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │       ╰── Constant Int [0]
                    ├── <16>  [&]
                    │   ├── Constant Int [10]
                    │   ╰── <15> Var [ptr]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │       ╰── Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <17> AddressOf
                    │           ╰── <16> Var [x]
                    ├── <24> Assign [&=]
                    │   ├── <21> Var [ptr]
                    │   ╰── Constant Int [0]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │       ╰── Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [100]
                    ├── <23> Assign [|=]
                    │   ├── <19> Var [x]
                    │   ╰── <22> Var [null]
                    ╰── Return
                        ╰── Constant Int [1]
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
                    │       ╰── Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [1000]
                    ├── <23>  [>>]
                    │   ├── <19> Var [i]
                    │   ╰── <22> Var [ptr]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │       ╰── Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── y
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [0]
                    ├── <24>  [|]
                    │   ├── <20> Var [x]
                    │   ╰── <23> Var [y]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │       ╰── Constant Int [0]
                    ╰── Return
                        ╰── <20> Cast
                            ├── Target
                            │   ╰── Int
                            ╰── Expression
                                ╰── <19>  [>>]
                                    ├── <15> Var [x]
                                    ╰── Constant Int [10]
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
                    │       ╰── Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── Constant Int [100]
                    ├── <23>  [^]
                    │   ├── <19> Var [ptr]
                    │   ╰── <22> Var [l]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │       ╰── Constant Int [100]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <17> AddressOf
                    │           ╰── <16> Var [i]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr2
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <31> AddressOf
                    │           ╰── <30> Assign [-=]
                    │               ├── <26> Dereference
                    │               │   ╰── <25> Var [ptr]
                    │               ╰── Constant Int [10]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │       ╰── Constant Int [100]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <21> AddressOf
                    │           ╰── <20> Assign [+=]
                    │               ├── <16> Var [i]
                    │               ╰── Constant Int [200]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │       ╰── Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── y
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [0]
                    ├── <24> Assign [/=]
                    │   ├── <20> Var [x]
                    │   ╰── <23> Var [y]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │       ╰── Constant Int [10]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <17> AddressOf
                    │           ╰── <16> Var [i]
                    ├── <25> Assign [&=]
                    │   ├── <21> Var [i]
                    │   ╰── <24> Var [ptr]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │       ╰── Constant Int [0]
                    ├── <16> Assign [*=]
                    │   ├── <13> Var [x]
                    │   ╰── Constant Int [2]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │       ╰── Constant Int [10]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <19> AddressOf
                    │           ╰── <18> Postfix [--]
                    │               ╰── <16> Var [i]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │       ╰── Constant Int [10]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <19> AddressOf
                    │           ╰── <18> Unary [++]
                    │               ╰── <17> Var [i]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │       ╰── Constant Int [0]
                    ╰── Switch
                        ├── Expression
                        │   ╰── <13> Var [x]
                        ╰── Block
                            ├── Case [0]
                            │   ╰── Return
                            │       ╰── Constant Int [0]
                            ╰── Default
                                ╰── Return
                                    ╰── Constant Int [1]
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
                    │       ╰── Constant ULong [140732898195768]
                    ╰── Return
                        ╰── Constant Int [0]
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
            │   │   ╰── Constant Int [10]
            │   ╰── Static
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │       ╰── Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── y
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <17> Var [x]
                    ├── <26>  [*]
                    │   ├── <21> Var [x]
                    │   ╰── <24> Var [y]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │       ╰── Constant Int [0]
                    ├── <17>  [*]
                    │   ├── Constant Int [0]
                    │   ╰── <15> Var [x]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │       ╰── Constant Int [0]
                    ├── <15> Unary [-]
                    │   ╰── <14> Var [x]
                    ╰── Return
                        ╰── Constant Int [0]
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
            │           ╰── <9> Var [i]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ╰── Type
                    │       ╰── Int
                    ╰── Return
                        ╰── <25> FunctionCall [f]
                            ╰── <24> AddressOf
                                ╰── <23> Var [x]
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
            │           ╰── <12> AddressOf
            │               ╰── <11> Var [i]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Long
                    │   ╰── Initializer
                    │       ╰── <25> FunctionCall [return_long_pointer]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │       ╰── Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── y
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── result
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <{node_id}> Conditional [?]
                    │           ├── Constant Int [1]
                    │           ├── Then
                    │           │   ╰── <25> Var [x]
                    │           ╰── Else
                    │               ╰── <27> Var [y]
                    ╰── Return
                        ╰── Constant Int [0]
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
            │       │   │   ╰── Constant Int [0]
            │       │   ╰── Static
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── dbl_ptr
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Double
            │       │   ╰── Initializer
            │       │       ╰── <22> Cast
            │       │           ├── Target
            │       │           │   ╰── Pointer
            │       │           │       ╰── Double
            │       │           ╰── Expression
            │       │               ╰── <21> Var [long_ptr]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── int_ptr
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Unsigned Int
            │       │   ╰── Initializer
            │       │       ╰── <34> Cast
            │       │           ├── Target
            │       │           │   ╰── Pointer
            │       │           │       ╰── Unsigned Int
            │       │           ╰── Expression
            │       │               ╰── <33> Var [long_ptr]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ptr_ptr
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Pointer
            │       │   │           ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <48> Cast
            │       │           ├── Target
            │       │           │   ╰── Pointer
            │       │           │       ╰── Pointer
            │       │           │           ╰── Int
            │       │           ╰── Expression
            │       │               ╰── <47> Var [long_ptr]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <52> Var [long_ptr]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [1]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <59> Var [dbl_ptr]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [2]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <66> Var [int_ptr]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [3]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <73> Var [ptr_ptr]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [4]
            │       ╰── Return
            │           ╰── Constant Int [0]
            ├── Function [check_round_trip]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── l
            │       │   ├── Type
            │       │   │   ╰── Long
            │       │   ╰── Initializer
            │       │       ╰── <92> Unary [-]
            │       │           ╰── Constant Int [1]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── long_ptr
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Long
            │       │   ╰── Initializer
            │       │       ╰── <101> AddressOf
            │       │           ╰── <100> Var [l]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── dbl_ptr
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Double
            │       │   ╰── Initializer
            │       │       ╰── <113> Cast
            │       │           ├── Target
            │       │           │   ╰── Pointer
            │       │           │       ╰── Double
            │       │           ╰── Expression
            │       │               ╰── <112> Var [long_ptr]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── other_long_ptr
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Long
            │       │   ╰── Initializer
            │       │       ╰── <125> Cast
            │       │           ├── Target
            │       │           │   ╰── Pointer
            │       │           │       ╰── Long
            │       │           ╰── Expression
            │       │               ╰── <124> Var [dbl_ptr]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <135>  [!=]
            │       │   │       ├── <130> Dereference
            │       │   │       │   ╰── <129> Var [other_long_ptr]
            │       │   │       ╰── <134> Unary [-]
            │       │   │           ╰── Constant Int [1]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [5]
            │       ╰── Return
            │           ╰── Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── result
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <153> FunctionCall [check_null_ptr_cast]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <157> Var [result]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <159> Var [result]
                    ├── <169> Assign [=]
                    │   ├── <165> Var [result]
                    │   ╰── <168> FunctionCall [check_round_trip]
                    ╰── Return
                        ╰── <172> Var [result]
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
            │       ╰── Constant Long [0]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── i
            │   ├── Type
            │   │   ╰── Pointer
            │   │       ╰── Int
            │   ╰── Initializer
            │       ╰── Constant ULong [0]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── i2
            │   ├── Type
            │   │   ╰── Pointer
            │   │       ╰── Int
            │   ╰── Initializer
            │       ╰── Constant UInt [0]
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
            │           ╰── <35>  [==]
            │               ├── <31> Var [val]
            │               ╰── Constant ULong [0]
            ├── Function [return_null_ptr]
            │   ╰── Body
            │       ╰── Return
            │           ╰── Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [10]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <64> AddressOf
                    │           ╰── <63> Var [x]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <68> Var [d]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <75> Var [i]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <82> Var [i2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── <92> Assign [=]
                    │   ├── <89> Var [ptr]
                    │   ╰── Constant ULong [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <95> Var [ptr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── y
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <112>  [!=]
                    │   │       ├── <109> Var [y]
                    │   │       ╰── Constant Int [0]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <120> Unary [!]
                    │   │       ╰── <119> FunctionCall [expect_null_param]
                    │   │           ╰── Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── null_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Long
                    │   ╰── Initializer
                    │       ╰── <131> FunctionCall [return_null_ptr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <138>  [!=]
                    │   │       ├── <135> Var [null_ptr]
                    │   │       ╰── Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [7]
                    ├── <150> Assign [=]
                    │   ├── <145> Var [ptr]
                    │   ╰── <149> AddressOf
                    │       ╰── <148> Var [x]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ternary_result
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <{node_id}> Conditional [?]
                    │           ├── Constant Int [10]
                    │           ├── Then
                    │           │   ╰── Constant Int [0]
                    │           ╰── Else
                    │               ╰── <159> Var [ptr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <164> Var [ternary_result]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [8]
                    ╰── Return
                        ╰── Constant Int [0]
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
            │       ╰── Constant Int [128]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── l
            │   ├── Type
            │   │   ╰── Long
            │   ╰── Initializer
            │       ╰── Constant Long [128]
            ├── Function [int_to_pointer]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── a
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <26> Cast
            │       │           ├── Target
            │       │           │   ╰── Pointer
            │       │           │       ╰── Int
            │       │           ╰── Expression
            │       │               ╰── <25> Var [i]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── b
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <38> Cast
            │       │           ├── Target
            │       │           │   ╰── Pointer
            │       │           │       ╰── Int
            │       │           ╰── Expression
            │       │               ╰── <37> Var [l]
            │       ╰── Return
            │           ╰── <46>  [==]
            │               ├── <42> Var [a]
            │               ╰── <45> Var [b]
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
            │       │       ╰── <65> AddressOf
            │       │           ╰── <64> Var [l]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ptr_as_long
            │       │   ├── Type
            │       │   │   ╰── Unsigned Long
            │       │   ╰── Initializer
            │       │       ╰── <75> Cast
            │       │           ├── Target
            │       │           │   ╰── Unsigned Long
            │       │           ╰── Expression
            │       │               ╰── <74> Var [ptr]
            │       ╰── Return
            │           ╰── <86>  [==]
            │               ├── <82>  [%]
            │               │   ├── <79> Var [ptr_as_long]
            │               │   ╰── Constant Int [8]
            │               ╰── Constant Int [0]
            ├── Function [cast_long_round_trip]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ptr
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <103> Cast
            │       │           ├── Target
            │       │           │   ╰── Pointer
            │       │           │       ╰── Int
            │       │           ╰── Expression
            │       │               ╰── <102> Var [l]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── l2
            │       │   ├── Type
            │       │   │   ╰── Long
            │       │   ╰── Initializer
            │       │       ╰── <113> Cast
            │       │           ├── Target
            │       │           │   ╰── Long
            │       │           ╰── Expression
            │       │               ╰── <112> Var [ptr]
            │       ╰── Return
            │           ╰── <122>  [==]
            │               ├── <117> Var [l]
            │               ╰── <120> Var [l2]
            ├── Function [cast_ulong_round_trip]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ptr
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Long
            │       │   ╰── Initializer
            │       │       ╰── <136> AddressOf
            │       │           ╰── <135> Var [l]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ptr_as_ulong
            │       │   ├── Type
            │       │   │   ╰── Unsigned Long
            │       │   ╰── Initializer
            │       │       ╰── <146> Cast
            │       │           ├── Target
            │       │           │   ╰── Unsigned Long
            │       │           ╰── Expression
            │       │               ╰── <145> Var [ptr]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ptr2
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Long
            │       │   ╰── Initializer
            │       │       ╰── <158> Cast
            │       │           ├── Target
            │       │           │   ╰── Pointer
            │       │           │       ╰── Long
            │       │           ╰── Expression
            │       │               ╰── <157> Var [ptr_as_ulong]
            │       ╰── Return
            │           ╰── <167>  [==]
            │               ├── <162> Var [ptr]
            │               ╰── <165> Var [ptr2]
            ├── Function [cast_int_round_trip]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── a
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Double
            │       │   ╰── Initializer
            │       │       ╰── <184> Cast
            │       │           ├── Target
            │       │           │   ╰── Pointer
            │       │           │       ╰── Double
            │       │           ╰── Expression
            │       │               ╰── <183> Var [i]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── i2
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <194> Cast
            │       │           ├── Target
            │       │           │   ╰── Int
            │       │           ╰── Expression
            │       │               ╰── <193> Var [a]
            │       ╰── Return
            │           ╰── <202>  [==]
            │               ├── <198> Var [i2]
            │               ╰── Constant Int [128]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <213> Unary [!]
                    │   │       ╰── <212> FunctionCall [int_to_pointer]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <222> Unary [!]
                    │   │       ╰── <221> FunctionCall [pointer_to_int]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <231> Unary [!]
                    │   │       ╰── <230> FunctionCall [cast_long_round_trip]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <240> Unary [!]
                    │   │       ╰── <239> FunctionCall [cast_ulong_round_trip]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <249> Unary [!]
                    │   │       ╰── <248> FunctionCall [cast_int_round_trip]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │       ╰── Constant Int [0]
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
                    │       ╰── <21> AddressOf
                    │           ╰── <20> Var [a]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a_ptr2
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <30> AddressOf
                    │           ╰── <29> Var [a]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <39> AddressOf
                    │           ╰── <38> Var [b]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <47>  [==]
                    │   │       ├── <43> Var [a_ptr]
                    │   │       ╰── <46> Var [b_ptr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <58>  [!=]
                    │   │       ├── <54> Var [a_ptr]
                    │   │       ╰── <57> Var [a_ptr2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <72> Unary [!]
                    │   │       ╰── <71>  [==]
                    │   │           ├── <66> Var [a_ptr]
                    │   │           ╰── <69> Var [a_ptr2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <86> Unary [!]
                    │   │       ╰── <85>  [!=]
                    │   │           ├── <80> Var [a_ptr]
                    │   │           ╰── <83> Var [b_ptr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── <99> Assign [=]
                    │   ├── <94> Dereference
                    │   │   ╰── <93> Var [b_ptr]
                    │   ╰── <98> Dereference
                    │       ╰── <97> Var [a_ptr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <106>  [==]
                    │   │       ├── <102> Var [a_ptr]
                    │   │       ╰── <105> Var [b_ptr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── <117> Assign [=]
                    │   ├── <113> Var [b_ptr]
                    │   ╰── <116> Var [a_ptr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <124>  [!=]
                    │   │       ├── <120> Var [b_ptr]
                    │   │       ╰── <123> Var [a_ptr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ╰── Return
                        ╰── Constant Int [0]
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
            │           ╰── Constant Int [0]
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
                    │       ╰── <23> FunctionCall [get_null_pointer]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── non_null
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Double
                    │   ╰── Initializer
                    │       ╰── <32> AddressOf
                    │           ╰── <31> Var [x]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <39>  [==]
                    │   │       ├── <36> Var [non_null]
                    │   │       ╰── Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <52> Unary [!]
                    │   │       ╰── <51>  [==]
                    │   │           ├── <47> Var [null]
                    │   │           ╰── Constant Long [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <65> Unary [!]
                    │   │       ╰── <64>  [!=]
                    │   │           ├── <60> Var [non_null]
                    │   │           ╰── Constant UInt [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <75>  [!=]
                    │   │       ├── <72> Var [null]
                    │   │       ╰── Constant ULong [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ╰── Return
                        ╰── Constant Int [0]
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
            │           ╰── Constant Int [0]
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
                    │       ╰── <24> AddressOf
                    │           ╰── <23> Var [x]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── null_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Long
                    │   ╰── Initializer
                    │       ╰── <32> FunctionCall [get_null_pointer]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <39>  [&&]
                    │   │       ├── Constant Double [+5e0]
                    │   │       ╰── <38> Var [null_ptr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <63> Unary [!]
                    │   │       ╰── <62>  [||]
                    │   │           ├── <53> Var [ptr]
                    │   │           ╰── <60> Assign [=]
                    │   │               ├── <56> Var [a]
                    │   │               ╰── Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <73>  [!=]
                    │   │       ├── <70> Var [a]
                    │   │       ╰── Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <82> Unary [!]
                    │   │       ╰── <81> Var [ptr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── j
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <{node_id}> Conditional [?]
                    │           ├── <92> Var [ptr]
                    │           ├── Then
                    │           │   ╰── Constant Int [1]
                    │           ╰── Else
                    │               ╰── Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── k
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <{node_id}> Conditional [?]
                    │           ├── <102> Var [null_ptr]
                    │           ├── Then
                    │           │   ╰── Constant Int [3]
                    │           ╰── Else
                    │               ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <112>  [!=]
                    │   │       ├── <109> Var [j]
                    │   │       ╰── Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <122>  [!=]
                    │   │       ├── <119> Var [k]
                    │   │       ╰── Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [0]
                    ├── While
                    │   ├── Condition
                    │   │   ╰── <135> Var [ptr]
                    │   ╰── Body
                    │       ╰── Block
                    │           ├── If
                    │           │   ├── Condition
                    │           │   │   ╰── <140>  [>=]
                    │           │   │       ├── <137> Var [i]
                    │           │   │       ╰── Constant Int [10]
                    │           │   ╰── Then
                    │           │       ╰── Block
                    │           │           ├── <145> Assign [=]
                    │           │           │   ├── <142> Var [ptr]
                    │           │           │   ╰── Constant Int [0]
                    │           │           ╰── Continue
                    │           ╰── <159> Assign [=]
                    │               ├── <152> Var [i]
                    │               ╰── <158>  [+]
                    │                   ├── <155> Var [i]
                    │                   ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <168>  [!=]
                    │   │       ├── <165> Var [i]
                    │   │       ╰── Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [7]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │       ╰── Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <21>  [!=]
                    │   │       ├── <13> Var [x]
                    │   │       ╰── <20> Cast
                    │   │           ├── Target
                    │   │           │   ╰── Pointer
                    │   │           │       ╰── Unsigned Long
                    │   │           ╰── Expression
                    │   │               ╰── Constant Int [0]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <37>  [!=]
                    │   │       ├── <26> Var [x]
                    │   │       ╰── <36> Cast
                    │   │           ├── Target
                    │   │           │   ╰── Pointer
                    │   │           │       ╰── Unsigned Long
                    │   │           ╰── Expression
                    │   │               ╰── Constant Int [0]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── y
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Pointer
                    │   │           ╰── Pointer
                    │   │               ╰── Double
                    │   ╰── Initializer
                    │       ╰── Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <61>  [!=]
                    │   │       ├── <51> Var [y]
                    │   │       ╰── <60> Cast
                    │   │           ├── Target
                    │   │           │   ╰── Pointer
                    │   │           │       ╰── Pointer
                    │   │           │           ╰── Pointer
                    │   │           │               ╰── Double
                    │   │           ╰── Expression
                    │   │               ╰── Constant Int [0]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <76>  [!=]
                    │   │       ├── <66> Var [y]
                    │   │       ╰── <75> Cast
                    │   │           ├── Target
                    │   │           │   ╰── Pointer
                    │   │           │       ╰── Pointer
                    │   │           │           ╰── Pointer
                    │   │           │               ╰── Double
                    │   │           ╰── Expression
                    │   │               ╰── Constant Int [0]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <93>  [!=]
                    │   │       ├── <89> Cast
                    │   │       │   ├── Target
                    │   │       │   │   ╰── Pointer
                    │   │       │   │       ╰── Pointer
                    │   │       │   │           ╰── Pointer
                    │   │       │   │               ╰── Double
                    │   │       │   ╰── Expression
                    │   │       │       ╰── Constant Int [0]
                    │   │       ╰── <92> Var [y]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [5]
                    ╰── Return
                        ╰── Constant Int [0]
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
            │           ╰── Constant Int [3]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── l
            │   ├── Type
            │   │   ╰── Long
            │   ╰── Initializer
            │       ╰── Constant Int [100]
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
            │       ├── <52> Assign [=]
            │       │   ├── <48> Dereference
            │       │   │   ╰── <47> Var [ptr]
            │       │   ╰── <51> Var [val]
            │       ╰── Return
            │           ╰── <56> AddressOf
            │               ╰── <55> Var [l]
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
            │       ├── <134> Assign [=]
            │       │   ├── <129> Var [u_ptr]
            │       │   ╰── <133> AddressOf
            │       │       ╰── <132> Var [u]
            │       ├── <143> Assign [=]
            │       │   ├── <137> Var [u]
            │       │   ╰── <142> Dereference
            │       │       ╰── <141> Dereference
            │       │           ╰── <140> Var [p]
            │       ╰── Return
            │           ╰── <147> AddressOf
            │               ╰── <146> Var [u_ptr]
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
                    │       ╰── Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <216> AddressOf
                    │           ╰── <215> Var [i]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr_to_iptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Pointer
                    │   │           ╰── Int
                    │   ╰── Initializer
                    │       ╰── <227> AddressOf
                    │           ╰── <226> Var [i_ptr]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d1
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── Constant Double [+0e0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d2
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── Constant Double [+1e1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Double
                    │   ╰── Initializer
                    │       ╰── <250> AddressOf
                    │           ╰── <249> Var [d1]
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
                    ├── <273> Assign [=]
                    │   ├── <269> Var [i]
                    │   ╰── <272> FunctionCall [return_3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <279>  [!=]
                    │   │       ├── <276> Var [i]
                    │   │       ╰── Constant Int [3]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <288>  [!=]
                    │   │       ├── <285> Dereference
                    │   │       │   ╰── <284> Var [i_ptr]
                    │   │       ╰── Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── <303> Assign [=]
                    │   ├── <295> Var [l_ptr]
                    │   ╰── <302> FunctionCall [two_pointers]
                    │       ├── <299> Var [d2]
                    │       ╰── <301> Var [d_ptr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <311>  [!=]
                    │   │       ├── <306> Var [l_ptr]
                    │   │       ╰── <310> AddressOf
                    │   │           ╰── <309> Var [l]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <322>  [!=]
                    │   │       ├── <319> Dereference
                    │   │       │   ╰── <318> Var [l_ptr]
                    │   │       ╰── Constant Int [100]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <333>  [!=]
                    │   │       ├── <330> Dereference
                    │   │       │   ╰── <329> Var [d_ptr]
                    │   │       ╰── Constant Double [+1e1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <343>  [!=]
                    │   │       ├── <340> Var [d1]
                    │   │       ╰── Constant Double [+1e1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ├── <356> Assign [=]
                    │   ├── <350> Var [ptr_to_uptr]
                    │   ╰── <355> FunctionCall [pointers_to_pointers]
                    │       ╰── <354> Var [ptr_to_iptr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <364>  [!=]
                    │   │       ├── <361> Dereference
                    │   │       │   ╰── <360> Dereference
                    │   │       │       ╰── <359> Var [ptr_to_uptr]
                    │   │       ╰── Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [7]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │       ╰── Constant Int [10]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Pointer
                    │   │       │       ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <17> AddressOf
                    │   │               ╰── <16> Var [x]
                    │   ├── Condition
                    │   │   ╰── <25>  [!=]
                    │   │       ├── <22> Var [i]
                    │   │       ╰── Constant Int [0]
                    │   ╰── Block
                    │       ├── <31> Assign [=]
                    │       │   ├── <28> Dereference
                    │       │   │   ╰── <27> Var [i]
                    │       │   ╰── Constant Int [5]
                    │       ╰── <37> Assign [=]
                    │           ├── <34> Var [i]
                    │           ╰── Constant Int [0]
                    ╰── Return
                        ╰── <43> Var [x]
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
                    │       ╰── Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <18>  [!=]
                    │   │       ├── <15> AddressOf
                    │   │       │   ╰── <14> Dereference
                    │   │       │       ╰── <13> Var [null_ptr]
                    │   │       ╰── Constant Int [0]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr_to_null
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Pointer
                    │   │           ╰── Int
                    │   ╰── Initializer
                    │       ╰── <29> AddressOf
                    │           ╰── <28> Var [null_ptr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <36> AddressOf
                    │   │       ╰── <35> Dereference
                    │   │           ╰── <34> Dereference
                    │   │               ╰── <33> Var [ptr_to_null]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [2]
                    ╰── Return
                        ╰── Constant Int [0]
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
            │       │   │   ╰── Constant Int [10]
            │       │   ╰── Static
            │       ╰── Return
            │           ╰── <15> AddressOf
            │               ╰── <14> Var [var]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── one
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Initializer
            │       ╰── Constant Int [1]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── val
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [100]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr_var
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <41> AddressOf
                    │           ╰── <40> Var [val]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <49>  [!=]
                    │   │       ├── <46> Dereference
                    │   │       │   ╰── <45> FunctionCall [return_pointer]
                    │   │       ╰── Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <66>  [!=]
                    │   │       ├── <63> Dereference
                    │   │       │   ╰── <{node_id}> Conditional [?]
                    │   │       │       ├── <56> Var [one]
                    │   │       │       ├── Then
                    │   │       │       │   ╰── <58> FunctionCall [return_pointer]
                    │   │       │       ╰── Else
                    │   │       │           ╰── <60> Var [ptr_var]
                    │   │       ╰── Constant Int [10]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <84>  [!=]
                    │   │       ├── <81> Dereference
                    │   │       │   ╰── <{node_id}> Conditional [?]
                    │   │       │       ├── <74>  [-]
                    │   │       │       │   ├── <71> Var [one]
                    │   │       │       │   ╰── Constant Int [1]
                    │   │       │       ├── Then
                    │   │       │       │   ╰── <76> FunctionCall [return_pointer]
                    │   │       │       ╰── Else
                    │   │       │           ╰── <78> Var [ptr_var]
                    │   │       ╰── Constant Int [100]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr_to_one
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <96> AddressOf
                    │           ╰── <95> Var [one]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <109>  [!=]
                    │   │       ├── <106> Dereference
                    │   │       │   ╰── <105> Assign [=]
                    │   │       │       ├── <100> Var [ptr_var]
                    │   │       │       ╰── <103> Var [ptr_to_one]
                    │   │       ╰── Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── <120> Assign [=]
                    │   ├── <117> Dereference
                    │   │   ╰── <116> FunctionCall [return_pointer]
                    │   ╰── Constant Int [20]
                    ├── <133> Assign [=]
                    │   ├── <130> Dereference
                    │   │   ╰── <{node_id}> Conditional [?]
                    │   │       ├── <123> Var [one]
                    │   │       ├── Then
                    │   │       │   ╰── <125> Var [ptr_var]
                    │   │       ╰── Else
                    │   │           ╰── <127> FunctionCall [return_pointer]
                    │   ╰── Constant Int [30]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <140>  [!=]
                    │   │       ├── <137> Dereference
                    │   │       │   ╰── <136> FunctionCall [return_pointer]
                    │   │       ╰── Constant Int [20]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <151>  [!=]
                    │   │       ├── <148> Dereference
                    │   │       │   ╰── <147> Var [ptr_var]
                    │   │       ╰── Constant Int [30]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <161>  [!=]
                    │   │       ├── <158> Var [one]
                    │   │       ╰── Constant Int [30]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [7]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │       ╰── Constant Double [+1e1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Double
                    │   ╰── Initializer
                    │       ╰── <17> AddressOf
                    │           ╰── <16> Var [d]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d_ptr_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Pointer
                    │   │           ╰── Double
                    │   ╰── Initializer
                    │       ╰── <27> AddressOf
                    │           ╰── <26> Var [d_ptr]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d_ptr_ptr_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Pointer
                    │   │           ╰── Pointer
                    │   │               ╰── Double
                    │   ╰── Initializer
                    │       ╰── <38> AddressOf
                    │           ╰── <37> Var [d_ptr_ptr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <45>  [!=]
                    │   │       ├── <42> Var [d]
                    │   │       ╰── Constant Double [+1e1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <56>  [!=]
                    │   │       ├── <53> Dereference
                    │   │       │   ╰── <52> Var [d_ptr]
                    │   │       ╰── Constant Double [+1e1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <68>  [!=]
                    │   │       ├── <65> Dereference
                    │   │       │   ╰── <64> Dereference
                    │   │       │       ╰── <63> Var [d_ptr_ptr]
                    │   │       ╰── Constant Double [+1e1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <81>  [!=]
                    │   │       ├── <78> Dereference
                    │   │       │   ╰── <77> Dereference
                    │   │       │       ╰── <76> Dereference
                    │   │       │           ╰── <75> Var [d_ptr_ptr_ptr]
                    │   │       ╰── Constant Double [+1e1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <93>  [!=]
                    │   │       ├── <89> AddressOf
                    │   │       │   ╰── <88> Var [d]
                    │   │       ╰── <92> Var [d_ptr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <105>  [!=]
                    │   │       ├── <101> Dereference
                    │   │       │   ╰── <100> Var [d_ptr_ptr]
                    │   │       ╰── <104> Var [d_ptr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <118>  [!=]
                    │   │       ├── <114> Dereference
                    │   │       │   ╰── <113> Dereference
                    │   │       │       ╰── <112> Var [d_ptr_ptr_ptr]
                    │   │       ╰── <117> Var [d_ptr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [7]
                    ├── <131> Assign [=]
                    │   ├── <128> Dereference
                    │   │   ╰── <127> Dereference
                    │   │       ╰── <126> Dereference
                    │   │           ╰── <125> Var [d_ptr_ptr_ptr]
                    │   ╰── Constant Double [+5e0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <137>  [!=]
                    │   │       ├── <134> Var [d]
                    │   │       ╰── Constant Double [+5e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <148>  [!=]
                    │   │       ├── <145> Dereference
                    │   │       │   ╰── <144> Var [d_ptr]
                    │   │       ╰── Constant Double [+5e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <160>  [!=]
                    │   │       ├── <157> Dereference
                    │   │       │   ╰── <156> Dereference
                    │   │       │       ╰── <155> Var [d_ptr_ptr]
                    │   │       ╰── Constant Double [+5e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <173>  [!=]
                    │   │       ├── <170> Dereference
                    │   │       │   ╰── <169> Dereference
                    │   │       │       ╰── <168> Dereference
                    │   │       │           ╰── <167> Var [d_ptr_ptr_ptr]
                    │   │       ╰── Constant Double [+5e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [11]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d2
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── Constant Double [+1e0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d2_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Double
                    │   ╰── Initializer
                    │       ╰── <191> AddressOf
                    │           ╰── <190> Var [d2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d2_ptr2
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Double
                    │   ╰── Initializer
                    │       ╰── <199> Var [d2_ptr]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d2_ptr_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Pointer
                    │   │           ╰── Double
                    │   ╰── Initializer
                    │       ╰── <209> AddressOf
                    │           ╰── <208> Var [d2_ptr]
                    ├── <218> Assign [=]
                    │   ├── <214> Dereference
                    │   │   ╰── <213> Var [d_ptr_ptr_ptr]
                    │   ╰── <217> Var [d2_ptr_ptr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <227>  [!=]
                    │   │       ├── <223> Dereference
                    │   │       │   ╰── <222> Dereference
                    │   │       │       ╰── <221> Var [d_ptr_ptr_ptr]
                    │   │       ╰── <226> Var [d2_ptr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [12]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <240>  [!=]
                    │   │       ├── <237> Dereference
                    │   │       │   ╰── <236> Dereference
                    │   │       │       ╰── <235> Dereference
                    │   │       │           ╰── <234> Var [d_ptr_ptr_ptr]
                    │   │       ╰── Constant Double [+1e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [13]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <252>  [==]
                    │   │       ├── <247> Var [d2_ptr_ptr]
                    │   │       ╰── <251> AddressOf
                    │   │           ╰── <250> Var [d2_ptr2]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [14]
                    ├── <261> Assign [=]
                    │   ├── <257> Var [d2_ptr]
                    │   ╰── <260> Var [d_ptr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <270>  [!=]
                    │   │       ├── <266> Dereference
                    │   │       │   ╰── <265> Dereference
                    │   │       │       ╰── <264> Var [d_ptr_ptr_ptr]
                    │   │       ╰── <269> Var [d_ptr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [15]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <282>  [!=]
                    │   │       ├── <278> Dereference
                    │   │       │   ╰── <277> Var [d2_ptr_ptr]
                    │   │       ╰── <281> Var [d_ptr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [16]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <295>  [==]
                    │   │       ├── <291> Dereference
                    │   │       │   ╰── <290> Dereference
                    │   │       │       ╰── <289> Var [d_ptr_ptr_ptr]
                    │   │       ╰── <294> Var [d2_ptr2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [17]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <308>  [!=]
                    │   │       ├── <305> Dereference
                    │   │       │   ╰── <304> Dereference
                    │   │       │       ╰── <303> Dereference
                    │   │       │           ╰── <302> Var [d_ptr_ptr_ptr]
                    │   │       ╰── Constant Double [+5e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [18]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │       ╰── <10> Unary [-]
                    │           ╰── Constant Int [100]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ul
                    │   ├── Type
                    │   │   ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── Constant ULong [13835058055282163712]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── Constant Double [+3.5e0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <31> AddressOf
                    │           ╰── <30> Var [i]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ul_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <40> AddressOf
                    │           ╰── <39> Var [ul]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Double
                    │   ╰── Initializer
                    │       ╰── <49> AddressOf
                    │           ╰── <48> Var [d]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <59>  [!=]
                    │   │       ├── <54> Dereference
                    │   │       │   ╰── <53> Var [i_ptr]
                    │   │       ╰── <58> Unary [-]
                    │   │           ╰── Constant Int [100]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <70>  [!=]
                    │   │       ├── <67> Dereference
                    │   │       │   ╰── <66> Var [ul_ptr]
                    │   │       ╰── Constant ULong [13835058055282163712]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <81>  [!=]
                    │   │       ├── <78> Dereference
                    │   │       │   ╰── <77> Var [d_ptr]
                    │   │       ╰── Constant Double [+3.5e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── <91> Assign [=]
                    │   ├── <88> Var [i]
                    │   ╰── Constant Int [12]
                    ├── <97> Assign [=]
                    │   ├── <94> Var [ul]
                    │   ╰── Constant Int [1000]
                    ├── <105> Assign [=]
                    │   ├── <100> Var [d]
                    │   ╰── <104> Unary [-]
                    │       ╰── Constant Double [+1e-3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <112>  [!=]
                    │   │       ├── <109> Dereference
                    │   │       │   ╰── <108> Var [i_ptr]
                    │   │       ╰── Constant Int [12]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <123>  [!=]
                    │   │       ├── <120> Dereference
                    │   │       │   ╰── <119> Var [ul_ptr]
                    │   │       ╰── Constant Int [1000]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <136>  [!=]
                    │   │       ├── <131> Dereference
                    │   │       │   ╰── <130> Var [d_ptr]
                    │   │       ╰── <135> Unary [-]
                    │   │           ╰── Constant Double [+1e-3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i2
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ul2
                    │   ├── Type
                    │   │   ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── Constant ULong [144115196665790464]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d2
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <159> Unary [-]
                    │           ╰── Constant Double [+3.33e1]
                    ├── <168> Assign [=]
                    │   ├── <163> Var [i_ptr]
                    │   ╰── <167> AddressOf
                    │       ╰── <166> Var [i2]
                    ├── <176> Assign [=]
                    │   ├── <171> Var [ul_ptr]
                    │   ╰── <175> AddressOf
                    │       ╰── <174> Var [ul2]
                    ├── <184> Assign [=]
                    │   ├── <179> Var [d_ptr]
                    │   ╰── <183> AddressOf
                    │       ╰── <182> Var [d2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <191>  [!=]
                    │   │       ├── <188> Dereference
                    │   │       │   ╰── <187> Var [i_ptr]
                    │   │       ╰── Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <202>  [!=]
                    │   │       ├── <199> Dereference
                    │   │       │   ╰── <198> Var [ul_ptr]
                    │   │       ╰── Constant ULong [144115196665790464]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <215>  [!=]
                    │   │       ├── <210> Dereference
                    │   │       │   ╰── <209> Var [d_ptr]
                    │   │       ╰── <214> Unary [-]
                    │   │           ╰── Constant Double [+3.33e1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [9]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │       ╰── Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <17> AddressOf
                    │           ╰── <16> Var [x]
                    ╰── Return
                        ╰── <22> Dereference
                            ╰── <21> Var [ptr]
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
            │       ╰── Constant UInt [4294967295]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── x
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Initializer
            │       ╰── Constant Int [10]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── y
            │   ├── Type
            │   │   ╰── Unsigned Int
            │   ╰── Initializer
            │       ╰── Constant UInt [4294967295]
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
            │       │   │   ╰── <39> Var [new_ptr]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── <45> Assign [=]
            │       │               ├── <41> Var [p]
            │       │               ╰── <44> Var [new_ptr]
            │       ╰── Return
            │           ╰── <52> Dereference
            │               ╰── <51> Var [p]
            ├── Function [increment_ptr]
            │   ╰── Body
            │       ├── <70> Assign [=]
            │       │   ├── <62> Dereference
            │       │   │   ╰── <61> Var [dbl_ptr]
            │       │   ╰── <69>  [+]
            │       │       ├── <66> Dereference
            │       │       │   ╰── <65> Var [dbl_ptr]
            │       │       ╰── Constant Double [+5e0]
            │       ╰── Return
            │           ╰── Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── pointer_to_static
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <86> AddressOf
                    │           ╰── <85> Var [x]
                    ├── <93> Assign [=]
                    │   ├── <90> Var [x]
                    │   ╰── Constant Int [20]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <100>  [!=]
                    │   │       ├── <97> Dereference
                    │   │       │   ╰── <96> Var [pointer_to_static]
                    │   │       ╰── Constant Int [20]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── <111> Assign [=]
                    │   ├── <108> Dereference
                    │   │   ╰── <107> Var [pointer_to_static]
                    │   ╰── Constant Int [100]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <117>  [!=]
                    │   │       ├── <114> Var [x]
                    │   │       ╰── Constant Int [100]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <127>  [!=]
                    │   │       ├── <124> Var [w]
                    │   │       ╰── Constant UInt [4294967295]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <137>  [!=]
                    │   │       ├── <134> Var [y]
                    │   │       ╰── Constant UInt [4294967295]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <144> Var [dbl_ptr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── Constant Long [1000]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <163>  [!=]
                    │   │       ├── <160> FunctionCall [modify_ptr]
                    │   │       │   ╰── <159> AddressOf
                    │   │       │       ╰── <158> Var [l]
                    │   │       ╰── Constant Long [1000]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ├── <175> Assign [=]
                    │   ├── <170> Var [l]
                    │   ╰── <174> Unary [-]
                    │       ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <183>  [!=]
                    │   │       ├── <179> FunctionCall [modify_ptr]
                    │   │       │   ╰── Constant Int [0]
                    │   │       ╰── <182> Var [l]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [7]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── Constant Double [+1e1]
                    ├── <201> Assign [=]
                    │   ├── <196> Var [dbl_ptr]
                    │   ╰── <200> AddressOf
                    │       ╰── <199> Var [d]
                    ├── <204> FunctionCall [increment_ptr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <211>  [!=]
                    │   │       ├── <208> Dereference
                    │   │       │   ╰── <207> Var [dbl_ptr]
                    │   │       ╰── Constant Int [15]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [8]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │       ╰── Constant UInt [2185232384]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── Constant Long [144115196665790464]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── Constant Double [+1e50]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Unsigned Int
                    │   ╰── Initializer
                    │       ╰── <29> AddressOf
                    │           ╰── <28> Var [i]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Long
                    │   ╰── Initializer
                    │       ╰── <38> AddressOf
                    │           ╰── <37> Var [l]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Double
                    │   ╰── Initializer
                    │       ╰── <47> AddressOf
                    │           ╰── <46> Var [d]
                    ├── <55> Assign [=]
                    │   ├── <52> Dereference
                    │   │   ╰── <51> Var [i_ptr]
                    │   ╰── Constant Int [10]
                    ├── <64> Assign [=]
                    │   ├── <59> Dereference
                    │   │   ╰── <58> Var [l_ptr]
                    │   ╰── <63> Unary [-]
                    │       ╰── Constant Int [20]
                    ├── <71> Assign [=]
                    │   ├── <68> Dereference
                    │   │   ╰── <67> Var [d_ptr]
                    │   ╰── Constant Double [+3.01e1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <77>  [!=]
                    │   │       ├── <74> Var [i]
                    │   │       ╰── Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <89>  [!=]
                    │   │       ├── <84> Var [l]
                    │   │       ╰── <88> Unary [-]
                    │   │           ╰── Constant Int [20]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <99>  [!=]
                    │   │       ├── <96> Var [d]
                    │   │       ╰── Constant Double [+3.01e1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ╰── Return
                        ╰── Constant Int [0]
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
            │       ╰── Constant Long [4294967295]
            ├── Function [get_ui_ptr]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <14> AddressOf
            │               ╰── <13> Var [ui]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── shiftcount
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Initializer
            │       ╰── Constant Int [5]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <37>  [!=]
                    │   │       ├── <34>  [<<]
                    │   │       │   ├── <30> Dereference
                    │   │       │   │   ╰── <29> FunctionCall [get_ui_ptr]
                    │   │       │   ╰── Constant Long [2]
                    │   │       ╰── Constant Long [4294967292]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <52>  [!=]
                    │   │       ├── <49>  [>>]
                    │   │       │   ├── <45> Dereference
                    │   │       │   │   ╰── <44> FunctionCall [get_ui_ptr]
                    │   │       │   ╰── Constant Int [2]
                    │   │       ╰── Constant Int [1073741823]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── shiftcount_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <64> AddressOf
                    │           ╰── <63> Var [shiftcount]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <76>  [!=]
                    │   │       ├── <73>  [>>]
                    │   │       │   ├── Constant UInt [1000000]
                    │   │       │   ╰── <71> Dereference
                    │   │       │       ╰── <70> Var [shiftcount_ptr]
                    │   │       ╰── Constant Int [31250]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <91>  [!=]
                    │   │       ├── <88>  [<<]
                    │   │       │   ├── Constant UInt [1000000]
                    │   │       │   ╰── <86> Dereference
                    │   │       │       ╰── <85> Var [shiftcount_ptr]
                    │   │       ╰── Constant Int [32000000]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │       ╰── <10> Unary [-]
                    │           ╰── Constant UInt [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ul
                    │   ├── Type
                    │   │   ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── Constant ULong [9223372036854775808]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ui_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Unsigned Int
                    │   ╰── Initializer
                    │       ╰── <25> AddressOf
                    │           ╰── <24> Var [ui]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ul_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <34> AddressOf
                    │           ╰── <33> Var [ul]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <48>  [!=]
                    │   │       ├── <45>  [&]
                    │   │       │   ├── <39> Dereference
                    │   │       │   │   ╰── <38> Var [ui_ptr]
                    │   │       │   ╰── <43> Dereference
                    │   │       │       ╰── <42> Var [ul_ptr]
                    │   │       ╰── Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <65>  [!=]
                    │   │       ├── <62>  [|]
                    │   │       │   ├── <56> Dereference
                    │   │       │   │   ╰── <55> Var [ui_ptr]
                    │   │       │   ╰── <60> Dereference
                    │   │       │       ╰── <59> Var [ul_ptr]
                    │   │       ╰── Constant ULong [9223372041149743103]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <76> Unary [-]
                    │           ╰── Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <85> AddressOf
                    │           ╰── <84> Var [i]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <100>  [!=]
                    │   │       ├── <95>  [&]
                    │   │       │   ├── <90> Dereference
                    │   │       │   │   ╰── <89> Var [i_ptr]
                    │   │       │   ╰── <93> Var [ul]
                    │   │       ╰── <99> Dereference
                    │   │           ╰── <98> Var [ul_ptr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <118>  [!=]
                    │   │       ├── <114>  [|]
                    │   │       │   ├── <108> Dereference
                    │   │       │   │   ╰── <107> Var [i_ptr]
                    │   │       │   ╰── <112> Dereference
                    │   │       │       ╰── <111> Var [ul_ptr]
                    │   │       ╰── <117> Var [i]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │       ╰── Constant Double [+5e0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Double
                    │   ╰── Initializer
                    │       ╰── <17> AddressOf
                    │           ╰── <16> Var [d]
                    ├── <25> Assign [*=]
                    │   ├── <22> Dereference
                    │   │   ╰── <21> Var [d_ptr]
                    │   ╰── Constant UInt [1000]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <31>  [!=]
                    │   │       ├── <28> Var [d]
                    │   │       ╰── Constant Double [+5e3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <42> Unary [-]
                    │           ╰── Constant Int [50]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <51> AddressOf
                    │           ╰── <50> Var [i]
                    ├── <59> Assign [&=]
                    │   ├── <56> Dereference
                    │   │   ╰── <55> Var [i_ptr]
                    │   ╰── Constant UInt [4294967200]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <66>  [!=]
                    │   │       ├── <63> Dereference
                    │   │       │   ╰── <62> Var [i_ptr]
                    │   │       ╰── Constant Int [46]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ui
                    │   ├── Type
                    │   │   ╰── Unsigned Int
                    │   ╰── Initializer
                    │       ╰── Constant UInt [4294967295]
                    ├── <84> Assign [/=]
                    │   ├── <79> Var [ui]
                    │   ╰── <83> Dereference
                    │       ╰── <82> Var [d_ptr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <90>  [!=]
                    │   │       ├── <87> Var [ui]
                    │   │       ╰── Constant UInt [858993]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── <102> Assign [=]
                    │   ├── <97> Var [i]
                    │   ╰── <101> Unary [-]
                    │       ╰── Constant Int [10]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ul
                    │   ├── Type
                    │   │   ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── Constant ULong [9223372036854775807]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ul_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <116> AddressOf
                    │           ╰── <115> Var [ul]
                    ├── <126> Assign [-=]
                    │   ├── <121> Dereference
                    │   │   ╰── <120> Var [i_ptr]
                    │   ╰── <125> Dereference
                    │       ╰── <124> Var [ul_ptr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <134>  [!=]
                    │   │       ├── <129> Var [i]
                    │   │       ╰── <133> Unary [-]
                    │   │           ╰── Constant Int [9]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <144>  [!=]
                    │   │       ├── <141> Var [ul]
                    │   │       ╰── Constant ULong [9223372036854775807]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │       ╰── Constant Int [10]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <17> AddressOf
                    │           ╰── <16> Var [x]
                    ├── <25> Assign [+=]
                    │   ├── <22> Dereference
                    │   │   ╰── <21> Var [ptr]
                    │   ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <31>  [!=]
                    │   │       ├── <28> Var [x]
                    │   │       ╰── Constant Int [15]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <46>  [!=]
                    │   │       ├── <43> Assign [-=]
                    │   │       │   ├── <39> Dereference
                    │   │       │   │   ╰── <38> Var [ptr]
                    │   │       │   ╰── Constant Int [12]
                    │   │       ╰── Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <56>  [!=]
                    │   │       ├── <53> Var [x]
                    │   │       ╰── Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── <67> Assign [*=]
                    │   ├── <64> Dereference
                    │   │   ╰── <63> Var [ptr]
                    │   ╰── Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <73>  [!=]
                    │   │       ├── <70> Var [x]
                    │   │       ╰── Constant Int [18]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── <84> Assign [/=]
                    │   ├── <81> Dereference
                    │   │   ╰── <80> Var [ptr]
                    │   ╰── Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <90>  [!=]
                    │   │       ├── <87> Var [x]
                    │   │       ╰── Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── <101> Assign [&=]
                    │   ├── <98> Dereference
                    │   │   ╰── <97> Var [ptr]
                    │   ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <107>  [!=]
                    │   │       ├── <104> Var [x]
                    │   │       ╰── Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ╰── Return
                        ╰── Constant Int [0]
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
            │       ╰── Constant ULong [18446460386757245432]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ul_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <17> AddressOf
                    │           ╰── <16> Var [ul]
                    ├── <27> Assign [&=]
                    │   ├── <22> Dereference
                    │   │   ╰── <21> Var [ul_ptr]
                    │   ╰── <26> Unary [-]
                    │       ╰── Constant Int [1000]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <33>  [!=]
                    │   │       ├── <30> Var [ul]
                    │   │       ╰── Constant ULong [18446460386757244952]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── <44> Assign [|=]
                    │   ├── <41> Dereference
                    │   │   ╰── <40> Var [ul_ptr]
                    │   ╰── Constant UInt [4294967040]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <50>  [!=]
                    │   │       ├── <47> Var [ul]
                    │   │       ╰── Constant ULong [18446460386824683288]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [123456]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ui
                    │   ├── Type
                    │   │   ╰── Unsigned Int
                    │   ╰── Initializer
                    │       ╰── Constant UInt [4042322160]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <73> Unary [-]
                    │           ╰── Constant Int [252645136]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ui_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Unsigned Int
                    │   ╰── Initializer
                    │       ╰── <82> AddressOf
                    │           ╰── <81> Var [ui]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Long
                    │   ╰── Initializer
                    │       ╰── <91> AddressOf
                    │           ╰── <90> Var [l]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <101> Assign [^=]
                    │   │       ├── <96> Dereference
                    │   │       │   ╰── <95> Var [ui_ptr]
                    │   │       ╰── <100> Dereference
                    │   │           ╰── <99> Var [l_ptr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <108> Var [ui]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <118>  [!=]
                    │   │       ├── <115> Var [i]
                    │   │       ╰── Constant Int [123456]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <130>  [!=]
                    │   │       ├── <125> Var [l]
                    │   │       ╰── <129> Unary [-]
                    │   │           ╰── Constant Int [252645136]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ╰── Return
                        ╰── Constant Int [0]
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
            │       ╰── Constant Int [0]
            ├── Function [putchar]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── c
            │           ╰── Type
            │               ╰── Int
            ├── Function [print_A]
            │   ╰── Body
            │       ├── <22> FunctionCall [putchar]
            │       │   ╰── Constant Int [65]
            │       ╰── Return
            │           ╰── <26> AddressOf
            │               ╰── <25> Var [i]
            ├── Function [print_B]
            │   ╰── Body
            │       ├── <37> FunctionCall [putchar]
            │       │   ╰── Constant Int [66]
            │       ╰── Return
            │           ╰── <41> AddressOf
            │               ╰── <40> Var [i]
            ╰── Function [main]
                ╰── Body
                    ├── <54> Assign [+=]
                    │   ├── <51> Dereference
                    │   │   ╰── <50> FunctionCall [print_A]
                    │   ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <60>  [!=]
                    │   │       ├── <57> Var [i]
                    │   │       ╰── Constant Int [5]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── <71> Assign [+=]
                    │   ├── <68> Dereference
                    │   │   ╰── <67> FunctionCall [print_B]
                    │   ╰── Constant Long [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <77>  [!=]
                    │   │       ├── <74> Var [i]
                    │   │       ╰── Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │       ╰── Constant Int [10]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── y
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <17> AddressOf
                    │           ╰── <16> Var [x]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <27>  [!=]
                    │   │       ├── <24> Unary [++]
                    │   │       │   ╰── <23> Dereference
                    │   │       │       ╰── <22> Var [y]
                    │   │       ╰── Constant Int [11]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <37>  [!=]
                    │   │       ├── <34> Var [x]
                    │   │       ╰── Constant Int [11]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <50>  [!=]
                    │   │       ├── <47> Unary [--]
                    │   │       │   ╰── <46> Dereference
                    │   │       │       ╰── <45> Var [y]
                    │   │       ╰── Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <60>  [!=]
                    │   │       ├── <57> Var [x]
                    │   │       ╰── Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <74>  [!=]
                    │   │       ├── <71> Postfix [++]
                    │   │       │   ╰── <69> Dereference
                    │   │       │       ╰── <67> Var [y]
                    │   │       ╰── Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <84>  [!=]
                    │   │       ├── <81> Var [x]
                    │   │       ╰── Constant Int [11]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <98>  [!=]
                    │   │       ├── <95> Postfix [--]
                    │   │       │   ╰── <93> Dereference
                    │   │       │       ╰── <91> Var [y]
                    │   │       ╰── Constant Int [11]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <108>  [!=]
                    │   │       ├── <105> Var [x]
                    │   │       ╰── Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [8]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ul
                    │   ├── Type
                    │   │   ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ul_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <126> AddressOf
                    │           ╰── <125> Var [ul]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <134> Postfix [--]
                    │   │       ╰── <132> Dereference
                    │   │           ╰── <130> Var [ul_ptr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <144>  [!=]
                    │   │       ├── <141> Var [ul]
                    │   │       ╰── Constant ULong [18446744073709551615]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [10]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── Constant Double [+0e0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Double
                    │   ╰── Initializer
                    │       ╰── <162> AddressOf
                    │           ╰── <161> Var [d]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <173>  [!=]
                    │   │       ├── <170> Unary [++]
                    │   │       │   ╰── <169> Dereference
                    │   │       │       ╰── <167> Var [d_ptr]
                    │   │       ╰── Constant Double [+1e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [11]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <183>  [!=]
                    │   │       ├── <180> Var [d]
                    │   │       ╰── Constant Double [+1e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [12]
                    ╰── Return
                        ╰── Constant Int [0]
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
            │       ╰── Constant Long [4294967300]
            ├── Function [get_ptr]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <14> AddressOf
            │               ╰── <13> Var [l]
            ╰── Function [main]
                ╰── Body
                    ╰── Switch
                        ├── Expression
                        │   ╰── <24> Dereference
                        │       ╰── <23> FunctionCall [get_ptr]
                        ╰── Block
                            ├── Case [1]
                            │   ╰── Return
                            │       ╰── Constant Int [1]
                            ├── Case [4]
                            │   ╰── Return
                            │       ╰── Constant Int [2]
                            ├── Case [4294967300]
                            │   ╰── Return
                            │       ╰── Constant Int [0]
                            ├── Case [-16]
                            │   ╰── Return
                            │       ╰── Constant Int [3]
                            ╰── Default
                                ╰── Return
                                    ╰── Constant Int [4]
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
            │       │       ╰── <14> AddressOf
            │       │           ╰── <13> Var [a]
            │       ├── <22> Assign [=]
            │       │   ├── <19> Dereference
            │       │   │   ╰── <18> Var [ptr]
            │       │   ╰── Constant Int [10]
            │       ╰── Return
            │           ╰── <25> Var [a]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── result
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <40> FunctionCall [addr_of_arg]
                    │           ╰── <39> Unary [-]
                    │               ╰── Constant Int [20]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <47>  [!=]
                    │   │       ├── <44> Var [result]
                    │   │       ╰── Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── var
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [100]
                    ├── <66> Assign [=]
                    │   ├── <60> Var [result]
                    │   ╰── <65> FunctionCall [addr_of_arg]
                    │       ╰── <64> Var [var]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <72>  [!=]
                    │   │       ├── <69> Var [result]
                    │   │       ╰── Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <82>  [!=]
                    │   │       ├── <79> Var [var]
                    │   │       ╰── Constant Int [100]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ╰── Return
                        ╰── Constant Int [0]
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
            │           ╰── <11> Var [in]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [10]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <33> FunctionCall [return_pointer]
                    │           ╰── <32> AddressOf
                    │               ╰── <31> Var [x]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <41>  [!=]
                    │   │       ├── <38> Dereference
                    │   │       │   ╰── <37> Var [x_ptr]
                    │   │       ╰── Constant Int [10]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [1]
                    ├── <49> Assign [=]
                    │   ├── <46> Var [x]
                    │   ╰── Constant Int [100]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <56>  [!=]
                    │   │       ├── <53> Dereference
                    │   │       │   ╰── <52> Var [x_ptr]
                    │   │       ╰── Constant Int [100]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <66>  [!=]
                    │   │       ├── <61> Var [x_ptr]
                    │   │       ╰── <65> AddressOf
                    │   │           ╰── <64> Var [x]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [3]
                    ╰── Return
                        ╰── Constant Int [0]
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
            │       │       ╰── <14> Dereference
            │       │           ╰── <13> Var [ptr]
            │       ├── <22> Assign [=]
            │       │   ├── <19> Dereference
            │       │   │   ╰── <18> Var [ptr]
            │       │   ╰── Constant Int [10]
            │       ╰── Return
            │           ╰── <25> Var [old_val]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [20]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── result
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <46> FunctionCall [update_value]
                    │           ╰── <45> AddressOf
                    │               ╰── <44> Var [x]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <53>  [!=]
                    │   │       ├── <50> Var [result]
                    │   │       ╰── Constant Int [20]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <63>  [!=]
                    │   │       ├── <60> Var [x]
                    │   │       ╰── Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    ├── <19> Assign [=]
                    │   ├── <15> Dereference
                    │   │   ╰── <14> Var [d_ptr]
                    │   ╰── <18> Var [new_val]
                    ╰── Return
                        ╰── Constant Int [0]
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
                    │       ╰── Constant Double [+0e0]
                    ├── <31> Assign [=]
                    │   ├── <26> Var [d_ptr]
                    │   ╰── <30> AddressOf
                    │       ╰── <29> Var [d]
                    ├── <35> FunctionCall [update_thru_ptr]
                    │   ╰── Constant Double [+1e1]
                    ╰── Return
                        ╰── <42>  [==]
                            ├── <38> Var [d]
                            ╰── Constant Double [+1e1]
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
            │           ╰── <13> Var [long_ptr]
            ╰── Function [set_pointer]
                ├── Parameters
                │   ╰── Param
                │       ├── Name
                │       │   ╰── new_ptr
                │       ╰── Type
                │           ╰── Pointer
                │               ╰── Long
                ╰── Body
                    ├── <30> Assign [=]
                    │   ├── <26> Var [long_ptr]
                    │   ╰── <29> Var [new_ptr]
                    ╰── Return
                        ╰── Constant Int [0]
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
            │   │   ╰── Constant Long [100]
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
                    │       ╰── <32> FunctionCall [get_pointer]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <36> Var [initial_ptr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── <46> FunctionCall [set_pointer]
                    │   ╰── <45> AddressOf
                    │       ╰── <44> Var [private_long]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── new_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Long
                    │   ╰── Initializer
                    │       ╰── <53> FunctionCall [get_pointer]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <61>  [==]
                    │   │       ├── <57> Var [initial_ptr]
                    │   │       ╰── <60> Var [new_ptr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <72>  [!=]
                    │   │       ├── <69> Dereference
                    │   │       │   ╰── <68> Var [new_ptr]
                    │   │       ╰── Constant Long [100]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <84>  [!=]
                    │   │       ├── <79> Var [new_ptr]
                    │   │       ╰── <83> AddressOf
                    │   │           ╰── <82> Var [private_long]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── <92> FunctionCall [set_pointer]
                    │   ╰── Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <95> FunctionCall [get_pointer]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <107>  [!=]
                    │   │       ├── <102> Var [new_ptr]
                    │   │       ╰── <106> AddressOf
                    │   │           ╰── <105> Var [private_long]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}
