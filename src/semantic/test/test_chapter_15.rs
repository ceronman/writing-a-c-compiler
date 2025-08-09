use super::assert_error;

#[test]
fn test_invalid_types_add_two_pointers() {
    assert_error(
        r#"
        int main(void)
        {
            int *x = 0;
            int *y = 0;
            return (x + y == 0);
                      //^ Operator is invalid
        }
    "#,
    );
}

#[test]
fn test_invalid_types_assign_incompatible_pointer_types() {
    assert_error(
        r#"
        int main(void) {
            int four_element_array[4] = {1, 2, 3, 4};
            int (*arr)[3] = &four_element_array;
                          //^^^^^^^^^^^^^^^^^^^ Cannot convert type for assignment!
        }
    "#,
    );
}

#[test]
fn test_invalid_types_assign_to_array() {
    assert_error(
        r#"
        int main(void)
        {
            int arr[3] = {1, 2, 3};
            int arr2[3] = {4, 5, 6};
            arr = arr2;
          //^^^ Type is not assignable
            return arr[0];
        }
    "#,
    );
}

#[test]
fn test_invalid_types_assign_to_array_2() {
    assert_error(
        r#"
        int main(void)
        {
            int dim2[1][2] = {{1, 2}};
            int dim[2] = {3, 4};
            dim2[0] = dim;
          //^^^^^^^ Type is not assignable
            return dim[0];
        }
    "#,
    );
}

#[test]
fn test_invalid_types_assign_to_array_3() {
    assert_error(
        r#"
        int main(void) {
            int arr[3] = { 1, 2, 3};
            int (*ptr_to_array)[3];
            *ptr_to_array = arr;
          //^^^^^^^^^^^^^ Type is not assignable
        }
    "#,
    );
}

#[test]
fn test_invalid_types_bad_arg_type() {
    assert_error(
        r#"
        int foo(int **x) {
            return x[0][0];
        }
        int main(void) {
            int arr[1] = {10};
            return foo(&arr);
                     //^^^^ Cannot convert type for assignment!
        }
    "#,
    );
}

#[test]
fn test_invalid_types_cast_to_array_type() {
    assert_error(
        r#"
        int main(void)
        {
            int arr[10];
            return (int[10])arr;
                          //^^^ Cannot cast to an array type
        }
    "#,
    );
}

#[test]
fn test_invalid_types_cast_to_array_type_2() {
    assert_error(
        r#"
        int main(void)
        {
            long arr[10];
            return (int *[10])arr;
                            //^^^ Cannot cast to an array type
        }
    "#,
    );
}

#[test]
fn test_invalid_types_cast_to_array_type_3() {
    assert_error(
        r#"
        int main(void)
        {
            long arr[6];
            return ((long(([2])[3]))arr);
                                  //^^^ Cannot cast to an array type
        }
    "#,
    );
}

#[test]
fn test_invalid_types_compare_different_pointer_types() {
    assert_error(
        r#"
        int main(void)
        {
            long x = 10;
            long *ptr = &x + 1;
            long(*array_ptr)[10] = (long (*)[10]) &x;
            return array_ptr < ptr;
                             //^^^ Operator is invalid
        }
    "#,
    );
}

#[test]
fn test_invalid_types_compare_explicit_and_implict_addr() {
    assert_error(
        r#"
        int main(void)
        {
            int arr[10];
            return arr == &arr;
                 //^^^^^^^^^^^ Expressions have incompatible types
        }
    "#,
    );
}

#[test]
fn test_invalid_types_compare_pointer_to_int() {
    assert_error(
        r#"
        int main(void)
        {
            long *l = 0;
            return l <= 100ul;
                      //^^^^^ Operator is invalid
        }
    "#,
    );
}

#[test]
fn test_invalid_types_compare_pointer_to_zero() {
    assert_error(
        r#"
        int main(void)
        {
            int *x = 0;
            return x > 0;
                     //^ Operator is invalid
        }
    "#,
    );
}

#[test]
fn test_invalid_types_compound_initializer_for_scalar() {
    assert_error(
        r#"
        int main(void)
        {
            int x = {1, 2, 3};
                  //^^^^^^^^^ Cannot initialize a scalar value with a compound initializer
            return x;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_compound_initializer_for_static_scalar() {
    assert_error(
        r#"
        int main(void)
        {
            static int x = {1, 2, 3};
                         //^^^^^^^^^ Cannot initialize a scalar value with a compound initializer
            return x;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_compound_initializer_too_long_static() {
    assert_error(
        r#"
        int main(void) {
            static int arr[3] = {1, 2, 3, 4};
                              //^^^^^^^^^^^^ Too many elements in the initializer
            return arr[2];
        }
    "#,
    );
}

#[test]
fn test_invalid_types_compound_inititializer_too_long() {
    assert_error(
        r#"
        int main(void) {
            int arr[3] = {1, 2, 3, 4};
                       //^^^^^^^^^^^^ Too many elements in the initializer
            return arr[2];
        }
    "#,
    );
}

#[test]
fn test_invalid_types_conflicting_array_declarations() {
    assert_error(
        r#"
        int arr[6];
        int main(void) {
            return arr[0];
        }
        int arr[5];
          //^^^ Variable 'arr' is already declared with a different type
    "#,
    );
}

#[test]
fn test_invalid_types_conflicting_function_declarations() {
    assert_error(
        r#"
        int f(int arr[2][3]);
        int f(int arr[2][4]);
          //^ Conflicting declaration types for 'f'
    "#,
    );
}

#[test]
fn test_invalid_types_double_subscript() {
    assert_error(
        r#"
        int main(void) {
            int arr[3] = {4, 5, 6};
            return arr[2.0];
                 //^^^^^^^^ Subscript requires integer and pointer types
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_compound_add_double_to_pointer() {
    assert_error(
        r#"
        
        int main(void) {
            int arr[3] = {1, 2, 3};
            int *elem = arr;
            elem += 1.0;
                  //^^^ Assign compound operation on a pointer requires integer operand
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_compound_add_two_pointers() {
    assert_error(
        r#"
        
        int main(void) {
            int arr[3] = {1, 2, 3};
            int *elem0 = arr;
            int *elem1 = arr + 1;
            elem0 += elem1;
                   //^^^^^ Assign compound operation on a pointer requires integer operand
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_compound_assign_to_array() {
    assert_error(
        r#"
        int main(void) {
            int arr[3] = {1, 2, 3};
            arr -= 1;
          //^^^ Type is not assignable
            0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_compound_assign_to_nested_array() {
    assert_error(
        r#"
        int main(void) {
            long arr[2][2] = {{1, 2}, {3, 4}};
            arr[1] += 1;
          //^^^^^^ Type is not assignable
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_compound_sub_pointer_from_int() {
    assert_error(
        r#"
        
        int main(void) {
            int arr[3] = {1, 2, 3};
            int *elem = arr + 1;
            int i = 0;
            i -= elem;
               //^^^^ Assign compound operator cannot be a pointer type
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_postfix_incr_array() {
    assert_error(
        r#"
        int main(void) {
            int arr[3] = {1, 2, 3};
            arr++;
          //^^^ Type is not assignable
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_postfix_incr_nested_array() {
    assert_error(
        r#"
        int main(void) {
            int arr[2][3] = {{1, 2, 3}, {4, 5, 6}};
            arr[2]++;
          //^^^^^^ Type is not assignable
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_prefix_decr_array() {
    assert_error(
        r#"
        int main(void) {
            int arr[3] = {1, 2, 3};
            --arr;
            //^^^ Type is not assignable
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_prefix_decr_nested_array() {
    assert_error(
        r#"
        int main(void) {
            int arr[2][3] = {{1, 2, 3}, {4, 5, 6}};
            --arr[2];
            //^^^^^^ Type is not assignable
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_switch_on_array() {
    assert_error(
        r#"
        
        int main(void) {
            int arr[3] = {1, 2, 3};
            switch (arr) {
                  //^^^ Switch statement requires an integer expression
                default:
                    return 0;
            }
            return 1;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_function_returns_array() {
    assert_error(
        r#"
        int(foo(void))[3][4];
      //^^^^^^^^^^^^^^^^^^^^ A function cannot return array
    "#,
    );
}

#[test]
fn test_invalid_types_incompatible_elem_type_compound_init() {
    assert_error(
        r#"
        int main(void)
        {
            int *arr[3] = {0, 0, 1.0};
                               //^^^ Cannot convert type for assignment!
        }
    "#,
    );
}

#[test]
fn test_invalid_types_incompatible_elem_type_static_compound_init() {
    assert_error(
        r#"
        
        int *arr[3] = {0, 0, 1.0};
                           //^^^ Invalid type of static declaration
        int main(void)
        {
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_null_ptr_array_initializer() {
    assert_error(
        r#"
        int main(void)
        {
            int arr[1] = 0;
                       //^ Cannot convert type for assignment!
            return arr[0];
        }
    "#,
    );
}

#[test]
fn test_invalid_types_null_ptr_static_array_initializer() {
    assert_error(
        r#"
        int main(void)
        {
            static int arr[1] = 0;
                              //^ Invalid type of static declaration
            return arr[0];
        }
    "#,
    );
}

#[test]
fn test_invalid_types_scalar_initializer_for_array() {
    assert_error(
        r#"
        int main(void)
        {
            int arr[1] = 4;
                       //^ Cannot convert type for assignment!
            return arr[0];
        }
    "#,
    );
}

#[test]
fn test_invalid_types_scalar_initializer_for_static_array() {
    assert_error(
        r#"
        
        double arr[3] = 1.0;
                      //^^^ Invalid type of static declaration
        int main(void)
        {
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_static_non_const_array() {
    assert_error(
        r#"
        int foo(int p) {
            static int arr[3] = { p, p + 1, 0};
                                //^ Non-constant initializer on local static variable
            return arr[2];
        }
        int main(void) {
            return foo(5);
        }
    "#,
    );
}

#[test]
fn test_invalid_types_sub_different_pointer_types() {
    assert_error(
        r#"
        int main(void)
        {
            long x[10];
            long *ptr = x;
            unsigned long *ptr2 = (unsigned long *)ptr;
            return ptr - ptr2;
                       //^^^^ Invalid pointer operator type
        }
    "#,
    );
}

#[test]
fn test_invalid_types_sub_double_from_ptr() {
    assert_error(
        r#"
        int main(void)
        {
            int *y = 0;
            return (y - 0.0 == 0.0);
                      //^^^ Operator is invalid
        }
    "#,
    );
}

#[test]
fn test_invalid_types_sub_ptr_from_int() {
    assert_error(
        r#"
        int main(void)
        {
            int *x = 0;
            return 0 - x == 0;
                     //^ Cannot substract a pointer from an int
        }
    "#,
    );
}

#[test]
fn test_invalid_types_subscript_both_pointers() {
    assert_error(
        r#"
        int main(void)
        {
            int x = 10;
            int *ptr = &x;
            int *subscript = 0;
            return ptr[subscript];
                 //^^^^^^^^^^^^^^ Subscript requires integer and pointer types
        }
    "#,
    );
}

#[test]
fn test_invalid_types_subscript_non_ptr() {
    assert_error(
        r#"
        int main(void) {
            int a = 3;
            return a[4];
                 //^^^^ Subscript requires integer and pointer types
        }
    "#,
    );
}
