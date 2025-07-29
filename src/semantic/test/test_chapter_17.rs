use super::assert_error;

#[test]
fn test_invalid_types_extra_credit_bitshift_void() {
    assert_error(
        r#"
        
        void f(void){
            return;
        }
        int main(void) {
            int x = 10;
            x << f();
               //^ Invalid operator type
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_bitwise_void() {
    assert_error(
        r#"
        
        int main(void) {
            int x = 10;
            int y = 11;
            x & (void) y;
              //^^^^^^^^ Invalid operator type
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_compound_add_void_pointer() {
    assert_error(
        r#"
        void *malloc(unsigned long size);
        int main(void) {
            void *buff = malloc(100);
            buff += 3;
          //^^^^ Cannot assign to a pointer of an incomplete type
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_compound_sub_void_pointer() {
    assert_error(
        r#"
        void *malloc(unsigned long size);
        int main(void) {
            void *buff = malloc(100);
            buff -= 0;
          //^^^^ Cannot assign to a pointer of an incomplete type
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_compound_void_rval() {
    assert_error(
        r#"
        
        void f(void) {
            return;
        }
        int main(void) {
            int x = 10;
            x *= f();
               //^ Invalid assign target
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_compound_void_rval_add() {
    assert_error(
        r#"
        
        void f(void) {
            return;
        }
        int main(void) {
            int *x = 0;
            x += f();
               //^ Invalid assign target
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_compound_void_rval_bitshift() {
    assert_error(
        r#"
        
        void f(void) {
            return;
        }
        int main(void) {
            int x = 10;
            x >>= f();
                //^ Invalid assign target
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_postfix_decr_void() {
    assert_error(
        r#"
        
        extern void *x;
        int main(void) {
            ++(*x)--;
              //^ Cannot dereference pointers to void
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_postfix_decr_void_pointer() {
    assert_error(
        r#"
        
        void *malloc(unsigned long size);
        int main(void) {
            void *buff = malloc(100);
            buff--;
          //^^^^ Illegal operation on pointer to void type
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_postfix_incr_void_pointer() {
    assert_error(
        r#"
        
        void *malloc(unsigned long size);
        int main(void) {
            void *buff = malloc(100);
            buff++;
          //^^^^ Illegal operation on pointer to void type
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_prefix_decr_void_pointer() {
    assert_error(
        r#"
        
        void *malloc(unsigned long size);
        int main(void) {
            void *buff = malloc(100);
            --buff;
            //^^^^ Illegal operation on pointer to void type
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_prefix_incr_void() {
    assert_error(
        r#"
        
        extern void *x;
        int main(void) {
            ++(*x);
              //^ Cannot dereference pointers to void
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_prefix_incr_void_pointer() {
    assert_error(
        r#"
        
        void *malloc(unsigned long size);
        int main(void) {
            void *buff = malloc(100);
            ++buff;
            //^^^^ Illegal operation on pointer to void type
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_switch_void() {
    assert_error(
        r#"
        
        void f(void) {
            return;
        }
        int main(void) {
            switch(f()) {
                 //^ Switch statement requires an integer expression
                default: return 0;
            }
        }
    "#,
    );
}

#[test]
fn test_invalid_types_incomplete_types_add_void_pointer() {
    assert_error(
        r#"
        void *malloc(unsigned long size);
        int main(void) {
          void *x = malloc(100);
          x = x + 1;
            //^ Cannot add pointers to incomplete types
          return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_incomplete_types_sizeof_function() {
    assert_error(
        r#"
        int x(void) { return 0; }
        int main(void) { return sizeof x; }
                                     //^ Function used as variable
    "#,
    );
}

#[test]
fn test_invalid_types_incomplete_types_sizeof_void() {
    assert_error(
        r#"
        int main(void) {
            return sizeof (void);
                         //^^^^ Cannot get size of an incomplete type
        }
    "#,
    );
}

#[test]
fn test_invalid_types_incomplete_types_sizeof_void_array() {
    assert_error(
        r#"
        int main(void) {
            return sizeof(void[3]);
                            //^^^ Illegal array of incomplete types
        }
    "#,
    );
}

#[test]
fn test_invalid_types_incomplete_types_sizeof_void_expression() {
    assert_error(
        r#"
        int main(void) {
          int x;
          return sizeof((void)x);
                     //^^^^^^^^^ Cannot get size of an incomplete type
        }
    "#,
    );
}

#[test]
fn test_invalid_types_incomplete_types_sub_void_pointer() {
    assert_error(
        r#"
        int main(void) {
          int y;
          void *x = &y;
          void *null = 0;
          return x - null;
               //^ Incomplete pointer type
        }
    "#,
    );
}

#[test]
fn test_invalid_types_incomplete_types_subscript_void() {
    assert_error(
        r#"
        int main(void) {
          int x = 10;
          void *v = &x;
          v[0];
        //^ Cannot subscript a pointer to void type
          return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_incomplete_types_subscript_void_pointer_conditional() {
    assert_error(
        r#"
        int main(void) {
          int arr[3] = {1, 2, 3};
          void *void_ptr = arr;
          int *int_ptr = arr + 1;
          return (1 ? int_ptr : void_ptr)[1];
               //^^^^^^^^^^^^^^^^^^^^^^^^ Cannot subscript a pointer to void type
        }
    "#,
    );
}

#[test]
fn test_invalid_types_incomplete_types_void_array() {
    assert_error(
        r#"
        int main(void) {
            void arr[3];
               //^^^ Illegal array of incomplete types
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_incomplete_types_void_array_in_cast() {
    assert_error(
        r#"
        int main(void) {
            (void(*)[3]) 4;
                //^^ Illegal array of incomplete types
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_incomplete_types_void_array_in_param_type() {
    assert_error(
        r#"
        int arr(void foo[3]) { return 3; }
          //^^^ Illegal array of incomplete types
        int main(void) { return 0; }
    "#,
    );
}

#[test]
fn test_invalid_types_incomplete_types_void_array_nested_in_declaration() {
    assert_error(
        r#"
        extern void (*ptr)[3][4];
                    //^^^ Illegal array of incomplete types
        void *foo(void) {
            return ptr;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_incomplete_types_void_array_pointer_in_declaration() {
    assert_error(
        r#"
        void *malloc(unsigned long size);
        int main(void) {
            void (*ptr)[3] = malloc(3);
                 //^^^ Illegal array of incomplete types
            return ptr == 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_incomplete_types_void_array_pointer_in_param_type() {
    assert_error(
        r#"
        
        int foo(void (*bad_array)[3]) {
          //^^^ Illegal array of incomplete types
            return bad_array == 0;
        }
        int main(void) {
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_pointer_conversions_compare_void_ptr_to_int() {
    assert_error(
        r#"
        int main(void) {
            return (void *)0 == 20ul;
                 //^^^^^^^^^^^^^^^^^ Expressions have incompatible types
        }
    "#,
    );
}

#[test]
fn test_invalid_types_pointer_conversions_compare_void_to_other_pointer() {
    assert_error(
        r#"
        int main(void) {
          int arr[3] = {1, 2, 3};
          void *ptr = (void *)arr;
          return ptr < arr + 1;
                     //^^^^^^^ Operator is invalid
        }
    "#,
    );
}

#[test]
fn test_invalid_types_pointer_conversions_convert_ulong_to_void_ptr() {
    assert_error(
        r#"
        int main(void) {
          unsigned long x = 0;
          void *v = x;
                  //^ Cannot convert type for assignment!
        }
    "#,
    );
}

#[test]
fn test_invalid_types_pointer_conversions_convert_void_ptr_to_int() {
    assert_error(
        r#"
        int main(void) {
          void *x = 0;
          return x;
               //^ Cannot convert type for assignment!
        }
    "#,
    );
}

#[test]
fn test_invalid_types_pointer_conversions_usual_arithmetic_conversions_ptr() {
    assert_error(
        r#"
        int main(void) {
          int i = 10 * (void *)0;
                     //^^^^^^^^^ Operator is invalid
        }
    "#,
    );
}

#[test]
fn test_invalid_types_scalar_expressions_and_void() {
    assert_error(
        r#"
        int main(void) {
            return (void)1 && 2;
                 //^^^^^^^ Invalid operator type
        }
    "#,
    );
}

#[test]
fn test_invalid_types_scalar_expressions_cast_void() {
    assert_error(
        r#"
        int main(void) {
            int y = (int) (void) 3;
                        //^^^^^^^^ Cannot cast non-scalar expression
            return y;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_scalar_expressions_not_void() {
    assert_error(
        r#"
        void f(void);
        void g(void);
        int main(void) { return !(1 ? f() : g()); }
                               //^^^^^^^^^^^^^^^ Unary operator requires a scalar operator
    "#,
    );
}

#[test]
fn test_invalid_types_scalar_expressions_or_void() {
    assert_error(
        r#"
        int main(void) { return 1 || (void)2; }
                                   //^^^^^^^ Invalid operator type
    "#,
    );
}

#[test]
fn test_invalid_types_scalar_expressions_void_condition_do_loop() {
    assert_error(
        r#"
        void f(void) { return; }
        int main(void) {
          int i = 0;
          do {
            i = i + 1;
          } while (f());
                 //^ Invalid condition type
          return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_scalar_expressions_void_condition_for_loop() {
    assert_error(
        r#"
        void foo(void) {
            return;
        }
        int main(void) {
            for (int i = 0; foo(); )
                          //^^^ Invalid condition type
                ;
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_scalar_expressions_void_condition_while_loop() {
    assert_error(
        r#"
        void f(void) { return; }
        int main(void) {
          int i = 0;
          while ((void)10) {
               //^^^^^^^^ Invalid condition type
            i = i + 1;
          }
          return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_scalar_expressions_void_if_condition() {
    assert_error(
        r#"
        int main(void) {
          int x = 10;
          if ((void)x)
            //^^^^^^^ Invalid condition type
            return 0;
          return 1;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_scalar_expressions_void_ternary_condition() {
    assert_error(
        r#"
        void f(void);
        int main(void) {
            return f() ? 1 : 2;
                 //^ Invalid condition type
        }
    "#,
    );
}

#[test]
fn test_invalid_types_void_assign_to_void_lvalue() {
    assert_error(
        r#"
        extern void *x;
        void foo(void) { return; }
        int main(void) {
          *x = foo();
         //^ Cannot dereference pointers to void
          return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_void_assign_to_void_var() {
    assert_error(
        r#"
        extern void v1;
                  //^^ Illegal void variable
        int main(void) {
          v1 = (void)0;
          return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_void_assign_void_rval() {
    assert_error(
        r#"
        
        int main(void) {
          int a = 10;
          a = (void)20;
            //^^^^^^^^ Invalid assign target
          return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_void_define_void() {
    assert_error(
        r#"
        int main(void) {
            void x;
               //^ Illegal void variable
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_void_initialized_void() {
    assert_error(
        r#"
        extern void v = 0;
                  //^ Illegal void variable
        int main(void) { return 0; }
    "#,
    );
}

#[test]
fn test_invalid_types_void_mismatched_conditional() {
    assert_error(
        r#"
        void foo(void) {
            return;
        }
        int main(void) {
            int a = 3;
            int flag = 4;
            flag ? foo() : (a = 3);
          //^^^^^^^^^^^^^^^^^^^^^^ Invalid types of the branches
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_void_negate_void() {
    assert_error(
        r#"
        int main(void) {
          -(void)10;
         //^^^^^^^^ Unary operator requires an arithmetic operator
          return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_void_no_return_value() {
    assert_error(
        r#"
        int foo(void) {
          return;
        //^^^^^^^^^^^^^^^^^ Return statement without an expression 
        }
        int main(void) {
          foo();
          return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_void_non_void_return() {
    assert_error(
        r#"
        void x(void) {
          return 1;
        //^^^^^^^^^ Return statement with an expression in a void function
        }
        int main(void) {
          x();
          return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_void_return_void_as_pointer() {
    assert_error(
        r#"
        void *x(void) {
          return (void)0;
               //^^^^^^^ Cannot convert type for assignment!
        }
    "#,
    );
}

#[test]
fn test_invalid_types_void_subscript_void() {
    assert_error(
        r#"
        int main(void) {
          char arr[3];
          return arr[(void)1];
               //^^^^^^^^^^^^ Subscript requires integer and pointer types
        }
    "#,
    );
}

#[test]
fn test_invalid_types_void_void_compare() {
    assert_error(
        r#"
        int main(void) {
          if ((void)1 < (void)2)
            //^^^^^^^ Invalid operator type
            return 1;
          return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_void_void_equality() {
    assert_error(
        r#"
        void x(void);
        int main(void) {
            return x() == (void)10;
                 //^ Invalid operator type
        }
    "#,
    );
}

#[test]
fn test_invalid_types_void_void_fun_params() {
    assert_error(
        r#"
        void foo(void x);
           //^^^ Illegal void parameter
        int main(void) {
            return 0;
        }
    "#,
    );
}
