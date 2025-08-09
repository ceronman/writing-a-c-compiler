use super::assert_error;

#[test]
fn test_invalid_labels_extra_credit_duplicate_case_char_const() {
    assert_error(
        r#"
        int main(void) {
            static int i = 120;
            switch (i) {
                case 'x':
                    return 1;
                case 120:
                   //^^^ duplicate case value
                    return 2;
                default:
                    return 3;
            }
        }
    "#,
    );
}

#[test]
fn test_invalid_types_assign_to_string_literal() {
    assert_error(
        r#"
        int main(void) {
            "foo" = "bar";
          //^^^^^ Type is not assignable
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_char_and_schar_conflict() {
    assert_error(
        r#"
        char c = 10;
        int main(void)
        {
            extern signed char c;
                             //^ Name 'c' was already declared
            return c;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_char_and_uchar_conflict() {
    assert_error(
        r#"
        int foo(unsigned char c) {
            return c;
        }
        int main(void) {
            return foo(0);
        }
        int foo(char c);
          //^^^ Conflicting declaration types for 'foo'
    "#,
    );
}

#[test]
fn test_invalid_types_compound_initializer_for_pointer() {
    assert_error(
        r#"
        
        int main(void) {
            char *ptr = {'a', 'b', 'c'};
                      //^^^^^^^^^^^^^^^ Cannot initialize a scalar value with a compound initializer
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_bit_shift_string() {
    assert_error(
        r#"
        
        int main(void) {
            "foo" << 3;
          //^^^^^ Operator requires integer operands
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_bitwise_operation_on_string() {
    assert_error(
        r#"
        
        int main(void) {
            "My string" & 100;
          //^^^^^^^^^^^ Operator requires integer operands
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_case_statement_string() {
    assert_error(
        r#"
        
        int main(void) {
            switch (0) {
                case "foo":
                   //^^^^^ case label does not reduce to an integer constant
                    return 1;
                default:
                    return 0;
            }
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_compound_assign_from_string() {
    assert_error(
        r#"
        
        int main(void) {
            char * s = "some string ";
            s += "another str";
               //^^^^^^^^^^^^^ Assign compound operation on a pointer requires integer operand
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_compound_assign_to_string() {
    assert_error(
        r#"
        
        int main(void) {
            "My string" += 1;
          //^^^^^^^^^^^ Type is not assignable
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_postfix_incr_string() {
    assert_error(
        r#"
        
        int main(void) {
            "foo"++;
          //^^^^^ Type is not assignable
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_prefix_incr_string() {
    assert_error(
        r#"
        
        int main(void) {
            ++"foo";
            //^^^^^ Type is not assignable
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_switch_on_string() {
    assert_error(
        r#"
        
        int main(void) {
            switch ("foo") {
                  //^^^^^ Switch statement requires an integer expression
                default:
                return 0;
            }
        }
    "#,
    );
}

#[test]
fn test_invalid_types_implicit_conversion_between_char_pointers() {
    assert_error(
        r#"
        
        int main(void) {
            char *c = 0;
            signed char *s = c;
                           //^ Cannot convert type for assignment!
            return (int) s;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_implicit_conversion_pointers_to_different_size_arrays() {
    assert_error(
        r#"
        int main(void) {
            char(*string_pointer)[10] = &"x";
                                      //^^^^ Cannot convert type for assignment!
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_negate_char_pointer() {
    assert_error(
        r#"
        
        int main(void) {
            char *x = "foo";
            return -x;
                  //^ Unary operator requires an arithmetic operator
        }
    "#,
    );
}

#[test]
fn test_invalid_types_string_initializer_for_multidim_array() {
    assert_error(
        r#"
        char arr[3][3] = "hello";
                       //^^^^^^^ Can't initialize a non-character type with a string literal
        int main(void)
        {
            return arr[0][2];
        }
    "#,
    );
}

#[test]
fn test_invalid_types_string_initializer_too_long() {
    assert_error(
        r#"
        int main(void) {
            char too_long[3] = "abcd";
                             //^^^^^^ Initializer string has more characters than destination array
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_string_initializer_too_long_nested() {
    assert_error(
        r#"
        int main(void) {
            char array[3][3] = {"a", "bcde"};
                                   //^^^^^^ Initializer string has more characters than destination array
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_string_initializer_too_long_nested_static() {
    assert_error(
        r#"
        char array[3][3] = {"a", "bcde"};
                               //^^^^^^ Initializer string has more characters than destination array
        int main(void)
        {
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_string_initializer_too_long_static() {
    assert_error(
        r#"
        int main(void) {
            static char too_long[3] = "abcd";
                                    //^^^^^^ Initializer string has more characters than destination array
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_string_initializer_wrong_type() {
    assert_error(
        r#"
        int main(void) {
            long ints[4] = "abc";
                         //^^^^^ Can't initialize a non-character type with a string literal
            return ints[1];
        }
    "#,
    );
}

#[test]
fn test_invalid_types_string_initializer_wrong_type_nested() {
    assert_error(
        r#"
        int main(void)
        {
            unsigned int nested[1][2] = {"a"};
                                       //^^^ Can't initialize a non-character type with a string literal
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_string_initializer_wrong_type_nested_static() {
    assert_error(
        r#"
        int main(void)
        {
            static long int nested[1][2] = {"a"};
                                          //^^^ Can't initialize a non-character type with a string literal
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_string_literal_is_plain_char_pointer() {
    assert_error(
        r#"
        int main(void) {
            signed char *ptr = "foo";
                             //^^^^^ Cannot convert type for assignment!
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_string_literal_is_plain_char_pointer_static() {
    assert_error(
        r#"
        int main(void) {
            static signed char *ptr = "foo";
                                    //^^^^^ Can't initialize a non-character pointer to a string literal
            return 0;
        }
    "#,
    );
}
