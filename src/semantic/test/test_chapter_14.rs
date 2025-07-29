use super::assert_error;

#[test]
fn test_invalid_declarations_extra_credit_addr_of_label() {
    assert_error(
        r#"
        
        int main(void) {
            int x = 0;
            lbl:
            x = 1;
            if (&lbl == 0) {
               //^^^ Undeclared variable 'lbl'
                return 1;
            }
            goto lbl;
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_declarations_extra_credit_deref_label() {
    assert_error(
        r#"
        
        int main(void) {
            lbl:
            *lbl;
           //^^^ Undeclared variable 'lbl'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_address_of_address() {
    assert_error(
        r#"
        int main(void) {
            int x = 0;
            int *y = &x;
            int **z = &(&x);
                    //^^^^^ Can't take address of non-lvalue!
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_address_of_assignment() {
    assert_error(
        r#"
        int main(void) {
            int x = 0;
            int y = 0;
            int *ptr = &(x = y);
                     //^^^^^^^^ Can't take address of non-lvalue!
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_address_of_constant() {
    assert_error(
        r#"
        
        int main(void) {
            int *ptr = &10;
                     //^^^ Can't take address of non-lvalue!
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_address_of_ternary() {
    assert_error(
        r#"
        int main(void) {
            int x = 1;
            int y = 2;
            int z = 3;
            int *ptr = &(x ? y : z);
                     //^^^^^^^^^^^^ Can't take address of non-lvalue!
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_assign_int_to_pointer() {
    assert_error(
        r#"
        int main(void) {
            int *x;
            x = 10;
              //^^ Cannot convert type for assignment!
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_assign_int_var_to_pointer() {
    assert_error(
        r#"
        int main(void)
        {
            int x = 0;
            int *ptr = x;
                     //^ Cannot convert type for assignment!
        }
    "#,
    );
}

#[test]
fn test_invalid_types_assign_to_address() {
    assert_error(
        r#"
        int main(void)
        {
            int x = 0;
            &x = 10;
          //^^ Expression is not assignable
        }
    "#,
    );
}

#[test]
fn test_invalid_types_assign_wrong_pointer_type() {
    assert_error(
        r#"
        int main(void)
        {
            double *d = 0;
            long *l = 0;
            l = d;
              //^ Cannot convert type for assignment!
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_bad_null_pointer_constant() {
    assert_error(
        r#"
        int main(void)
        {
            int *x = 0.0;
                   //^^^ Cannot convert type for assignment!
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_cast_double_to_pointer() {
    assert_error(
        r#"
        int main(void) {
            double d = 0.0;
            int *x = (int *) d;
                           //^ Cannot cast a double to a pointer
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_cast_pointer_to_double() {
    assert_error(
        r#"
        int main(void) {
            int *x;
            double d = (double) x;
                              //^ Cannot cast a pointer to a double
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_compare_mixed_pointer_types() {
    assert_error(
        r#"
        
        int main(void) {
            int *x = 0ul;
            unsigned *y = 0ul;
            return x == y;
                 //^^^^^^ Expressions have incompatible types
        }
    "#,
    );
}

#[test]
fn test_invalid_types_compare_pointer_to_ulong() {
    assert_error(
        r#"
        
        int main(void) {
            int *ptr = 0ul;
            unsigned long ul = 0ul;
            return ptr == ul;
                 //^^^^^^^^^ Expressions have incompatible types
        }
    "#,
    );
}

#[test]
fn test_invalid_types_complement_pointer() {
    assert_error(
        r#"
        
        int main(void) {
            int *x = 0;
            return (int) ~x;
                        //^ Unary operator requires an integer type
        }
    "#,
    );
}

#[test]
fn test_invalid_types_dereference_non_pointer() {
    assert_error(
        r#"
        
        int main(void) {
            unsigned long l = 100ul;
            return *l;
                 //^^ Cannot dereference a non-pointer
        }
    "#,
    );
}

#[test]
fn test_invalid_types_divide_pointer() {
    assert_error(
        r#"
        
        int main(void)
        {
            int x = 10;
            int *y = &x;
            (y / 8);
           //^ Operator is invalid
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_bitwise_and_pointer() {
    assert_error(
        r#"
        
        int main(void) {
            long *ptr = 0;
            10 & ptr;
               //^^^ Operator requires integer operands
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_bitwise_compound_assign_to_pointer() {
    assert_error(
        r#"
        
        int main(void) {
            int x = 0;
            int *ptr = &x;
            ptr &= 0;
          //^^^ Assign compound operation requires integer operands
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_bitwise_compound_assign_with_pointer() {
    assert_error(
        r#"
        int main(void) {
            int *null = 0;
            int x = 100;
            x |= null;
               //^^^^ Assign compound operation requires integer operands
            return 1;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_bitwise_lshift_pointer() {
    assert_error(
        r#"
        
        int main(void) {
            int *ptr = 0;
            int i = 1000;
            i >> ptr;
               //^^^ Operator requires integer operands
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_bitwise_or_pointer() {
    assert_error(
        r#"
        
        int main(void) {
            int *x = 0;
            int *y = 0;
            x | y;
          //^ Operator requires integer operands
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_bitwise_rshift_pointer() {
    assert_error(
        r#"
        
        int main(void) {
            int *x = 0;
            return (int) (x >> 10);
                        //^ Operator requires integer operands
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_bitwise_xor_pointer() {
    assert_error(
        r#"
        
        int main(void) {
            unsigned long *ptr = 0;
            long l = 100;
            ptr ^ l;
          //^^^ Operator requires integer operands
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_compound_assign_thru_ptr_not_lval() {
    assert_error(
        r#"
        int main(void) {
            int i = 100;
            int *ptr = &i;
            int *ptr2 = &(*ptr -= 10);
                      //^^^^^^^^^^^^^ Can't take address of non-lvalue!
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_compound_assignment_not_lval() {
    assert_error(
        r#"
        int main(void) {
            int i = 100;
            int *ptr = &(i += 200);
                     //^^^^^^^^^^^ Can't take address of non-lvalue!
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_compound_divide_pointer() {
    assert_error(
        r#"
        int main(void) {
            int *x = 0;
            int *y = 0;
            x /= y;
          //^ Assign compound operation cannot be used on pointer type
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_compound_mod_pointer() {
    assert_error(
        r#"
        int main(void) {
            int i = 10;
            int *ptr = &i;
            i %= ptr;
               //^^^ Assign compound operation requires integer operands
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_compound_multiply_pointer() {
    assert_error(
        r#"
        int main(void) {
            int *x = 0;
            x *= 2;
          //^ Assign compound operation cannot be used on pointer type
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_postfix_decr_not_lvalue() {
    assert_error(
        r#"
        int main(void) {
            int i = 10;
            int *ptr = &i--;
                     //^^^^ Can't take address of non-lvalue!
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_prefix_incr_not_lvalue() {
    assert_error(
        r#"
        int main(void) {
            int i = 10;
            int *ptr = &++i;
                     //^^^^ Can't take address of non-lvalue!
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_switch_on_pointer() {
    assert_error(
        r#"
        int main(void) {
            int *x = 0;
            switch(x) {
                 //^ Switch statement requires an integer expression
                case 0: return 0;
                default: return 1;
            }
        }
    "#,
    );
}

#[test]
fn test_invalid_types_invalid_pointer_initializer() {
    assert_error(
        r#"
        int main(void)
        {
            int *ptr = 140732898195768ul;
                     //^^^^^^^^^^^^^^^^^ Cannot convert type for assignment!
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_invalid_static_initializer() {
    assert_error(
        r#"
        
        static int *x = 10;
                      //^^ Invalid type of static declaration
        int main(void) {
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_multiply_pointers() {
    assert_error(
        r#"
        
        int main(void) {
            int *x = 0;
            int *y = x;
            (x * y);
           //^ Operator is invalid
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_multiply_pointers_2() {
    assert_error(
        r#"
        
        int main(void)
        {
            int *x = 0;
            (0 * x);
               //^ Operator is invalid
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_negate_pointer() {
    assert_error(
        r#"
        
        int main(void) {
            int *x = 0;
            -x;
           //^ Unary operator requires an arithmetic operator
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_pass_pointer_as_int() {
    assert_error(
        r#"
        int f(int i) {
            return i;
        }
        int main(void) {
            int x;
            return f(&x);
                   //^^ Cannot convert type for assignment!
        }
    "#,
    );
}

#[test]
fn test_invalid_types_return_wrong_pointer_type() {
    assert_error(
        r#"
        int i;
        long *return_long_pointer(void) {
            return &i;
                 //^^ Cannot convert type for assignment!
        }
        int main(void) {
            long *l = return_long_pointer();
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_ternary_mixed_pointer_types() {
    assert_error(
        r#"
        int main(void) {
            long *x = 0;
            int *y = 0;
            int *result = 1 ? x : y;
                            //^^^^^ Expressions have incompatible types
            return 0;
        }
    "#,
    );
}
