use super::{assert_error, assert_parse};

#[test]
fn test_invalid_labels_extra_credit_duplicate_case_char_const() {
    let src = r#"
        int main(void) {
            static int i = 120;
            switch (i) {
                case 'x':
                    return 1;
                case 120:
                    return 2;
                default:
                    return 3;
            }
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
                    │   │   ╰── Constant Int [120]
                    │   ╰── Static
                    ╰── Switch
                        ├── Expression
                        │   ╰── <13> Var [i]
                        ╰── Block
                            ├── Case [120]
                            │   ╰── Return
                            │       ╰── Constant Int [1]
                            ├── Case [120]
                            │   ╰── Return
                            │       ╰── Constant Int [2]
                            ╰── Default
                                ╰── Return
                                    ╰── Constant Int [3]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_parse_extra_credit_character_const_goto() {
    assert_error(
        r#"
        
        int main(void) {
            goto 'x';
               //^^^ Expected identifier, but found ''x''
            'x';
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_extra_credit_character_const_label() {
    assert_error(
        r#"
        
        int main(void) {
            'x': return 0;
             //^ Expected ';', but found ':'
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_extra_credit_string_literal_goto() {
    assert_error(
        r#"
        
        int main(void) {
            goto "foo";
               //^^^^^ Expected identifier, but found '"foo"'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_extra_credit_string_literal_label() {
    assert_error(
        r#"
        
        int main(void) {
            "foo": return 0;
               //^ Expected ';', but found ':'
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_invalid_type_specifier() {
    assert_error(
        r#"
        int main(void)
        {
            int char x = 10;
          //^^^^^^^^ Invalid type specifier
            return x;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_invalid_type_specifier_2() {
    assert_error(
        r#"
        int main(void) {
            char static long x = 0;
          //^^^^^^^^^^^^^^^^ Invalid type specifier
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_misplaced_char_literal() {
    assert_error(
        r#"
        int main(void) {
            int a = 3;
            return a'1';
                  //^^^ Expected ';', but found ''1''
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_string_literal_varname() {
    assert_error(
        r#"
        int main(void) {
            int "x" = 0;
              //^^^ Expected identifier, but found '"x"'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_assign_to_string_literal() {
    let src = r#"
        int main(void) {
            "foo" = "bar";
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── <8> Assign [=]
                    │   ├── <5> "foo"
                    │   ╰── <7> "bar"
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_char_and_schar_conflict() {
    let src = r#"
        char c = 10;
        int main(void)
        {
            extern signed char c;
            return c;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── c
            │   ├── Type
            │   │   ╰── Char
            │   ╰── Initializer
            │       ╰── Constant Int [10]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Signed Char
                    │   ╰── Extern
                    ╰── Return
                        ╰── <17> Var [c]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_char_and_uchar_conflict() {
    let src = r#"
        int foo(unsigned char c) {
            return c;
        }
        int main(void) {
            return foo(0);
        }
        int foo(char c);
    "#;
    let expected = r#"
        Program
            ├── Function [foo]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── c
            │   │       ╰── Type
            │   │           ╰── Unsigned Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <9> Var [c]
            ├── Function [main]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <19> FunctionCall [foo]
            │               ╰── Constant Int [0]
            ╰── Function [foo]
                ╰── Parameters
                    ╰── Param
                        ├── Name
                        │   ╰── c
                        ╰── Type
                            ╰── Char
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_compound_initializer_for_pointer() {
    let src = r#"
        
        int main(void) {
            char *ptr = {'a', 'b', 'c'};
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
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── Constant Int [97]
                    │           ├── Constant Int [98]
                    │           ╰── Constant Int [99]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_bit_shift_string() {
    let src = r#"
        
        int main(void) {
            "foo" << 3;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── <8>  [<<]
                    │   ├── <5> "foo"
                    │   ╰── Constant Int [3]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_bitwise_operation_on_string() {
    let src = r#"
        
        int main(void) {
            "My string" & 100;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── <8>  [&]
                    │   ├── <5> "My string"
                    │   ╰── Constant Int [100]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_case_statement_string() {
    let src = r#"
        
        int main(void) {
            switch (0) {
                case "foo":
                    return 1;
                default:
                    return 0;
            }
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Switch
                        ├── Expression
                        │   ╰── Constant Int [0]
                        ╰── Block
                            ├── Case [invalid]
                            │   ├── Value
                            │   │   ╰── <6> "foo"
                            │   ╰── Return
                            │       ╰── Constant Int [1]
                            ╰── Default
                                ╰── Return
                                    ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_compound_assign_from_string() {
    let src = r#"
        
        int main(void) {
            char * s = "some string ";
            s += "another str";
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── s
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── <9> "some string "
                    ├── <16> Assign [+=]
                    │   ├── <13> Var [s]
                    │   ╰── <15> "another str"
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_compound_assign_to_string() {
    let src = r#"
        
        int main(void) {
            "My string" += 1;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── <8> Assign [+=]
                    │   ├── <5> "My string"
                    │   ╰── Constant Int [1]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_postfix_incr_string() {
    let src = r#"
        
        int main(void) {
            "foo"++;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── <7> Postfix [++]
                    │   ╰── <5> "foo"
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_prefix_incr_string() {
    let src = r#"
        
        int main(void) {
            ++"foo";
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── <7> Unary [++]
                    │   ╰── <6> "foo"
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_switch_on_string() {
    let src = r#"
        
        int main(void) {
            switch ("foo") {
                default:
                return 0;
            }
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Switch
                        ├── Expression
                        │   ╰── <5> "foo"
                        ╰── Block
                            ╰── Default
                                ╰── Return
                                    ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_implicit_conversion_between_char_pointers() {
    let src = r#"
        
        int main(void) {
            char *c = 0;
            signed char *s = c;
            return (int) s;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── s
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Signed Char
                    │   ╰── Initializer
                    │       ╰── <17> Var [c]
                    ╰── Return
                        ╰── <24> Cast
                            ├── Target
                            │   ╰── Int
                            ╰── Expression
                                ╰── <23> Var [s]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_implicit_conversion_pointers_to_different_size_arrays() {
    let src = r#"
        int main(void) {
            char(*string_pointer)[10] = &"x";
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── string_pointer
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Array
                    │   │           ├── 10
                    │   │           ╰── Char
                    │   ╰── Initializer
                    │       ╰── <13> AddressOf
                    │           ╰── <12> "x"
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_negate_char_pointer() {
    let src = r#"
        
        int main(void) {
            char *x = "foo";
            return -x;
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
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── <9> "foo"
                    ╰── Return
                        ╰── <15> Unary [-]
                            ╰── <14> Var [x]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_string_initializer_for_multidim_array() {
    let src = r#"
        char arr[3][3] = "hello";
        int main(void)
        {
            return arr[0][2];
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── arr
            │   ├── Type
            │   │   ╰── Array
            │   │       ├── 3
            │   │       ╰── Array
            │   │           ├── 3
            │   │           ╰── Char
            │   ╰── Initializer
            │       ╰── <8> "hello"
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <20> Subscript
                            ├── <18> Subscript
                            │   ├── <16> Var [arr]
                            │   ╰── Constant Int [0]
                            ╰── Constant Int [2]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_string_initializer_too_long() {
    let src = r#"
        int main(void) {
            char too_long[3] = "abcd";
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── too_long
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 3
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── <10> "abcd"
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_string_initializer_too_long_nested() {
    let src = r#"
        int main(void) {
            char array[3][3] = {"a", "bcde"};
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── array
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 3
                    │   │       ╰── Array
                    │   │           ├── 3
                    │   │           ╰── Char
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <12> "a"
                    │           ╰── <14> "bcde"
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_string_initializer_too_long_nested_static() {
    let src = r#"
        char array[3][3] = {"a", "bcde"};
        int main(void)
        {
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── array
            │   ├── Type
            │   │   ╰── Array
            │   │       ├── 3
            │   │       ╰── Array
            │   │           ├── 3
            │   │           ╰── Char
            │   ╰── Initializer
            │       ╰── Compound
            │           ├── <8> "a"
            │           ╰── <10> "bcde"
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_string_initializer_too_long_static() {
    let src = r#"
        int main(void) {
            static char too_long[3] = "abcd";
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── too_long
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 3
                    │   │       ╰── Char
                    │   ├── Initializer
                    │   │   ╰── <11> "abcd"
                    │   ╰── Static
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_string_initializer_wrong_type() {
    let src = r#"
        int main(void) {
            long ints[4] = "abc";
            return ints[1];
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ints
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 4
                    │   │       ╰── Long
                    │   ╰── Initializer
                    │       ╰── <10> "abc"
                    ╰── Return
                        ╰── <16> Subscript
                            ├── <14> Var [ints]
                            ╰── Constant Int [1]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_string_initializer_wrong_type_nested() {
    let src = r#"
        int main(void)
        {
            unsigned int nested[1][2] = {"a"};
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── nested
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 1
                    │   │       ╰── Array
                    │   │           ├── 2
                    │   │           ╰── Unsigned Int
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ╰── <12> "a"
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_string_initializer_wrong_type_nested_static() {
    let src = r#"
        int main(void)
        {
            static long int nested[1][2] = {"a"};
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── nested
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 1
                    │   │       ╰── Array
                    │   │           ├── 2
                    │   │           ╰── Long
                    │   ├── Initializer
                    │   │   ╰── Compound
                    │   │       ╰── <13> "a"
                    │   ╰── Static
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_string_literal_is_plain_char_pointer() {
    let src = r#"
        int main(void) {
            signed char *ptr = "foo";
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
                    │   │       ╰── Signed Char
                    │   ╰── Initializer
                    │       ╰── <9> "foo"
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_string_literal_is_plain_char_pointer_static() {
    let src = r#"
        int main(void) {
            static signed char *ptr = "foo";
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
                    │   │       ╰── Signed Char
                    │   ├── Initializer
                    │   │   ╰── <10> "foo"
                    │   ╰── Static
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_char_constants_char_constant_operations() {
    let src = r#"
        double d = '\\';
        int main(void) {
            if (d != 92.0) {
                return 1;
            }
            unsigned long array['\n'] = {1, 2, 'a', '\b', 3, 4, 5, '!', '%', '~'};
            if (array[2] != 97) {
                return 2;
            }
            if (array[3] != 8) {
                return 3;
            }
            if (array[7] != 33) {
                return 4;
            }
            if (array[8] != 37) {
                return 5;
            }
            if (array[9] != 126) {
                return 6;
            }
            unsigned long (*array_ptr)[10] = &array;
            if (array_ptr[0][9] != '~') {
                return 7;
            }
            int i = array['\a'];
            if (i != 33) {
                return 8;
            }
            double d = 10 % '\a' + 4.0 * '_' - ~'@';
            if (d != 448.0) {
                return 9;
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
            │   │   ╰── Double
            │   ╰── Initializer
            │       ╰── Constant Int [92]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <15>  [!=]
                    │   │       ├── <12> Var [d]
                    │   │       ╰── Constant Double [+9.2e1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── array
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 10
                    │   │       ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── Constant Int [1]
                    │           ├── Constant Int [2]
                    │           ├── Constant Int [97]
                    │           ├── Constant Int [8]
                    │           ├── Constant Int [3]
                    │           ├── Constant Int [4]
                    │           ├── Constant Int [5]
                    │           ├── Constant Int [33]
                    │           ├── Constant Int [37]
                    │           ╰── Constant Int [126]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <54>  [!=]
                    │   │       ├── <51> Subscript
                    │   │       │   ├── <49> Var [array]
                    │   │       │   ╰── Constant Int [2]
                    │   │       ╰── Constant Int [97]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <66>  [!=]
                    │   │       ├── <63> Subscript
                    │   │       │   ├── <61> Var [array]
                    │   │       │   ╰── Constant Int [3]
                    │   │       ╰── Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <78>  [!=]
                    │   │       ├── <75> Subscript
                    │   │       │   ├── <73> Var [array]
                    │   │       │   ╰── Constant Int [7]
                    │   │       ╰── Constant Int [33]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <90>  [!=]
                    │   │       ├── <87> Subscript
                    │   │       │   ├── <85> Var [array]
                    │   │       │   ╰── Constant Int [8]
                    │   │       ╰── Constant Int [37]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <102>  [!=]
                    │   │       ├── <99> Subscript
                    │   │       │   ├── <97> Var [array]
                    │   │       │   ╰── Constant Int [9]
                    │   │       ╰── Constant Int [126]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── array_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Array
                    │   │           ├── 10
                    │   │           ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <117> AddressOf
                    │           ╰── <116> Var [array]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <128>  [!=]
                    │   │       ├── <125> Subscript
                    │   │       │   ├── <123> Subscript
                    │   │       │   │   ├── <121> Var [array_ptr]
                    │   │       │   │   ╰── Constant Int [0]
                    │   │       │   ╰── Constant Int [9]
                    │   │       ╰── Constant Int [126]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [7]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <140> Subscript
                    │           ├── <138> Var [array]
                    │           ╰── Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <147>  [!=]
                    │   │       ├── <144> Var [i]
                    │   │       ╰── Constant Int [33]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [8]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <170>  [-]
                    │           ├── <165>  [+]
                    │           │   ├── <159>  [%]
                    │           │   │   ├── Constant Int [10]
                    │           │   │   ╰── Constant Int [7]
                    │           │   ╰── <164>  [*]
                    │           │       ├── Constant Double [+4e0]
                    │           │       ╰── Constant Int [95]
                    │           ╰── <169> Unary [~]
                    │               ╰── Constant Int [64]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <177>  [!=]
                    │   │       ├── <174> Var [d]
                    │   │       ╰── Constant Double [+4.48e2]
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
fn test_valid_char_constants_control_characters() {
    let src = r#"
        int main(void)
        {
            int tab = '	';
            int vertical_tab = '';
            int form_feed = '';
            if (tab != '\t') {
                return 1;
            }
            if (vertical_tab != '\v') {
                return 2;
            }
            if (form_feed != '\f') {
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
                    │   │   ╰── tab
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [9]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── vertical_tab
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [11]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── form_feed
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [12]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <27>  [!=]
                    │   │       ├── <24> Var [tab]
                    │   │       ╰── Constant Int [9]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <37>  [!=]
                    │   │       ├── <34> Var [vertical_tab]
                    │   │       ╰── Constant Int [11]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <47>  [!=]
                    │   │       ├── <44> Var [form_feed]
                    │   │       ╰── Constant Int [12]
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
fn test_valid_char_constants_escape_sequences() {
    let src = r#"
        
        int main(void) {
            if ('\?' != 63) {
                return 1;
            }
            if ('\"' != 34) {
                return 2;
            }
            if ('\'' != 39) {
                return 3;
            }
            if ('\\' != 92) {
                return 4;
            }
            if ('\a' != 7) {
                return 5;
            }
            if ('\b' != 8) {
                return 6;
            }
            if ('\f' != 12) {
                return 7;
            }
            if ('\n' != 10) {
                return 8;
            }
            if ('\r' != 13) {
                return 9;
            }
            if ('\t' != 9) {
                return 10;
            }
            if ('\v' != 11) {
                return 11;
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
                    │   │   ╰── <8>  [!=]
                    │   │       ├── Constant Int [63]
                    │   │       ╰── Constant Int [63]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <17>  [!=]
                    │   │       ├── Constant Int [34]
                    │   │       ╰── Constant Int [34]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <26>  [!=]
                    │   │       ├── Constant Int [39]
                    │   │       ╰── Constant Int [39]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <35>  [!=]
                    │   │       ├── Constant Int [92]
                    │   │       ╰── Constant Int [92]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <44>  [!=]
                    │   │       ├── Constant Int [7]
                    │   │       ╰── Constant Int [7]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <53>  [!=]
                    │   │       ├── Constant Int [8]
                    │   │       ╰── Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <62>  [!=]
                    │   │       ├── Constant Int [12]
                    │   │       ╰── Constant Int [12]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <71>  [!=]
                    │   │       ├── Constant Int [10]
                    │   │       ╰── Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <80>  [!=]
                    │   │       ├── Constant Int [13]
                    │   │       ╰── Constant Int [13]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <89>  [!=]
                    │   │       ├── Constant Int [9]
                    │   │       ╰── Constant Int [9]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <98>  [!=]
                    │   │       ├── Constant Int [11]
                    │   │       ╰── Constant Int [11]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [11]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_char_constants_return_char_constant() {
    let src = r#"
        
        int main(void) {
            return 'c';
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── Constant Int [99]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_chars_access_through_char_pointer() {
    let src = r#"
        int main(void) {
            int x = 100;
            char *byte_ptr = (char *) &x;
            if (byte_ptr[0] != 100) {
                return 1;
            }
            if (byte_ptr[1] || byte_ptr[2] || byte_ptr[3]) {
                return 2;
            }
            double d = -0.0;
            byte_ptr = (char *) &d;
            if (byte_ptr[7] != -128) {
                return 3;
            }
            for (int i = 0; i < 7; i = i + 1) {
                if (byte_ptr[i]) {
                    return 4;
                }
            }
            unsigned int array[3][2][1] = {
                {{-1}, {-1}},
                {{-1}, {-1}},
                {{4294901760u}}
            };
            byte_ptr = (char *) array;
            byte_ptr = byte_ptr + 16;
            if (byte_ptr[0] || byte_ptr[1]) {
                return 5;
            }
            if (byte_ptr[2] != -1) {
                return 6;
            }
            if (byte_ptr[3] != -1) {
                return 7;
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
                    │       ╰── Constant Int [100]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── byte_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── <21> Cast
                    │           ├── Target
                    │           │   ╰── Pointer
                    │           │       ╰── Char
                    │           ╰── Expression
                    │               ╰── <20> AddressOf
                    │                   ╰── <19> Var [x]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <30>  [!=]
                    │   │       ├── <27> Subscript
                    │   │       │   ├── <25> Var [byte_ptr]
                    │   │       │   ╰── Constant Int [0]
                    │   │       ╰── Constant Int [100]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <51>  [||]
                    │   │       ├── <45>  [||]
                    │   │       │   ├── <39> Subscript
                    │   │       │   │   ├── <37> Var [byte_ptr]
                    │   │       │   │   ╰── Constant Int [1]
                    │   │       │   ╰── <44> Subscript
                    │   │       │       ├── <42> Var [byte_ptr]
                    │   │       │       ╰── Constant Int [2]
                    │   │       ╰── <50> Subscript
                    │   │           ├── <48> Var [byte_ptr]
                    │   │           ╰── Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <62> Unary [-]
                    │           ╰── Constant Double [+0e0]
                    ├── <75> Assign [=]
                    │   ├── <66> Var [byte_ptr]
                    │   ╰── <74> Cast
                    │       ├── Target
                    │       │   ╰── Pointer
                    │       │       ╰── Char
                    │       ╰── Expression
                    │           ╰── <73> AddressOf
                    │               ╰── <72> Var [d]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <85>  [!=]
                    │   │       ├── <80> Subscript
                    │   │       │   ├── <78> Var [byte_ptr]
                    │   │       │   ╰── Constant Int [7]
                    │   │       ╰── <84> Unary [-]
                    │   │           ╰── Constant Int [128]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <102>  [<]
                    │   │       ├── <99> Var [i]
                    │   │       ╰── Constant Int [7]
                    │   ├── Condition
                    │   │   ╰── <111> Assign [=]
                    │   │       ├── <104> Var [i]
                    │   │       ╰── <110>  [+]
                    │   │           ├── <107> Var [i]
                    │   │           ╰── Constant Int [1]
                    │   ╰── Block
                    │       ╰── If
                    │           ├── Condition
                    │           │   ╰── <116> Subscript
                    │           │       ├── <113> Var [byte_ptr]
                    │           │       ╰── <115> Var [i]
                    │           ╰── Then
                    │               ╰── Block
                    │                   ╰── Return
                    │                       ╰── Constant Int [4]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── array
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 3
                    │   │       ╰── Array
                    │   │           ├── 2
                    │   │           ╰── Array
                    │   │               ├── 1
                    │   │               ╰── Unsigned Int
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── Compound
                    │           │   ├── Compound
                    │           │   │   ╰── <136> Unary [-]
                    │           │   │       ╰── Constant Int [1]
                    │           │   ╰── Compound
                    │           │       ╰── <141> Unary [-]
                    │           │           ╰── Constant Int [1]
                    │           ├── Compound
                    │           │   ├── Compound
                    │           │   │   ╰── <147> Unary [-]
                    │           │   │       ╰── Constant Int [1]
                    │           │   ╰── Compound
                    │           │       ╰── <152> Unary [-]
                    │           │           ╰── Constant Int [1]
                    │           ╰── Compound
                    │               ╰── Compound
                    │                   ╰── Constant UInt [4294901760]
                    ├── <171> Assign [=]
                    │   ├── <163> Var [byte_ptr]
                    │   ╰── <170> Cast
                    │       ├── Target
                    │       │   ╰── Pointer
                    │       │       ╰── Char
                    │       ╰── Expression
                    │           ╰── <169> Var [array]
                    ├── <181> Assign [=]
                    │   ├── <174> Var [byte_ptr]
                    │   ╰── <180>  [+]
                    │       ├── <177> Var [byte_ptr]
                    │       ╰── Constant Int [16]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <192>  [||]
                    │   │       ├── <186> Subscript
                    │   │       │   ├── <184> Var [byte_ptr]
                    │   │       │   ╰── Constant Int [0]
                    │   │       ╰── <191> Subscript
                    │   │           ├── <189> Var [byte_ptr]
                    │   │           ╰── Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <206>  [!=]
                    │   │       ├── <201> Subscript
                    │   │       │   ├── <199> Var [byte_ptr]
                    │   │       │   ╰── Constant Int [2]
                    │   │       ╰── <205> Unary [-]
                    │   │           ╰── Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <220>  [!=]
                    │   │       ├── <215> Subscript
                    │   │       │   ├── <213> Var [byte_ptr]
                    │   │       │   ╰── Constant Int [3]
                    │   │       ╰── <219> Unary [-]
                    │   │           ╰── Constant Int [1]
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
fn test_valid_chars_chained_casts() {
    let src = r#"
        unsigned int ui = 4294967200u;
        int main(void) {
            ui = (unsigned int)(unsigned char)ui;
            if (ui != 160) {
                return 1;
            }
            int i = (int)(signed char)ui;
            if (i != -96) {
                return 2;
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
            │       ╰── Constant UInt [4294967200]
            ╰── Function [main]
                ╰── Body
                    ├── <22> Assign [=]
                    │   ├── <12> Var [ui]
                    │   ╰── <21> Cast
                    │       ├── Target
                    │       │   ╰── Unsigned Int
                    │       ╰── Expression
                    │           ╰── <20> Cast
                    │               ├── Target
                    │               │   ╰── Unsigned Char
                    │               ╰── Expression
                    │                   ╰── <19> Var [ui]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <28>  [!=]
                    │   │       ├── <25> Var [ui]
                    │   │       ╰── Constant Int [160]
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
                    │       ╰── <44> Cast
                    │           ├── Target
                    │           │   ╰── Int
                    │           ╰── Expression
                    │               ╰── <43> Cast
                    │                   ├── Target
                    │                   │   ╰── Signed Char
                    │                   ╰── Expression
                    │                       ╰── <42> Var [ui]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <53>  [!=]
                    │   │       ├── <48> Var [i]
                    │   │       ╰── <52> Unary [-]
                    │   │           ╰── Constant Int [96]
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
fn test_valid_chars_char_arguments() {
    let src = r#"
        
        int check_args(char a, signed char b, char c, unsigned char d, char e, char f, signed char g, char h) {
            char expected_a = 5;
            signed char expected_b = -12;
            char expected_c = 117;
            unsigned char expected_d = 254;
            char expected_e = 1;
            char expected_f = -20;
            signed char expected_g = 60;
            char expected_h = 100;
            if (expected_a != a) {
             return 1;
            }
            if (expected_b != b) {
             return 2;
            }
            if (expected_c != c) {
             return 3;
            }
            if (expected_d != d) {
             return 4;
            }
            if (expected_e != e) {
             return 5;
            }
            if (expected_f != f) {
             return 6;
            }
            if (expected_g != g) {
             return 7;
            }
            if (expected_h != h) {
             return 8;
            }
            return 0;
        }
        int main(void) {
            char a = 5;
            signed char b = -12;
            char c = 117;
            unsigned char d = 254;
            char e = 1;
            char f = -20;
            signed char g = 60;
            char h = 100;
            return check_args(a, b, c, d, e, f, g, h);
        }
    "#;
    let expected = r#"
        Program
            ├── Function [check_args]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── a
            │   │   │   ╰── Type
            │   │   │       ╰── Char
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── b
            │   │   │   ╰── Type
            │   │   │       ╰── Signed Char
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── c
            │   │   │   ╰── Type
            │   │   │       ╰── Char
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── d
            │   │   │   ╰── Type
            │   │   │       ╰── Unsigned Char
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── e
            │   │   │   ╰── Type
            │   │   │       ╰── Char
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── f
            │   │   │   ╰── Type
            │   │   │       ╰── Char
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── g
            │   │   │   ╰── Type
            │   │   │       ╰── Signed Char
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── h
            │   │       ╰── Type
            │   │           ╰── Char
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── expected_a
            │       │   ├── Type
            │       │   │   ╰── Char
            │       │   ╰── Initializer
            │       │       ╰── Constant Int [5]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── expected_b
            │       │   ├── Type
            │       │   │   ╰── Signed Char
            │       │   ╰── Initializer
            │       │       ╰── <40> Unary [-]
            │       │           ╰── Constant Int [12]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── expected_c
            │       │   ├── Type
            │       │   │   ╰── Char
            │       │   ╰── Initializer
            │       │       ╰── Constant Int [117]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── expected_d
            │       │   ├── Type
            │       │   │   ╰── Unsigned Char
            │       │   ╰── Initializer
            │       │       ╰── Constant Int [254]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── expected_e
            │       │   ├── Type
            │       │   │   ╰── Char
            │       │   ╰── Initializer
            │       │       ╰── Constant Int [1]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── expected_f
            │       │   ├── Type
            │       │   │   ╰── Char
            │       │   ╰── Initializer
            │       │       ╰── <66> Unary [-]
            │       │           ╰── Constant Int [20]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── expected_g
            │       │   ├── Type
            │       │   │   ╰── Signed Char
            │       │   ╰── Initializer
            │       │       ╰── Constant Int [60]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── expected_h
            │       │   ├── Type
            │       │   │   ╰── Char
            │       │   ╰── Initializer
            │       │       ╰── Constant Int [100]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <86>  [!=]
            │       │   │       ├── <82> Var [expected_a]
            │       │   │       ╰── <85> Var [a]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [1]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <97>  [!=]
            │       │   │       ├── <93> Var [expected_b]
            │       │   │       ╰── <96> Var [b]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [2]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <108>  [!=]
            │       │   │       ├── <104> Var [expected_c]
            │       │   │       ╰── <107> Var [c]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [3]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <119>  [!=]
            │       │   │       ├── <115> Var [expected_d]
            │       │   │       ╰── <118> Var [d]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [4]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <130>  [!=]
            │       │   │       ├── <126> Var [expected_e]
            │       │   │       ╰── <129> Var [e]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [5]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <141>  [!=]
            │       │   │       ├── <137> Var [expected_f]
            │       │   │       ╰── <140> Var [f]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [6]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <152>  [!=]
            │       │   │       ├── <148> Var [expected_g]
            │       │   │       ╰── <151> Var [g]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [7]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <163>  [!=]
            │       │   │       ├── <159> Var [expected_h]
            │       │   │       ╰── <162> Var [h]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [8]
            │       ╰── Return
            │           ╰── Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── Constant Int [5]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Signed Char
                    │   ╰── Initializer
                    │       ╰── <188> Unary [-]
                    │           ╰── Constant Int [12]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── Constant Int [117]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Unsigned Char
                    │   ╰── Initializer
                    │       ╰── Constant Int [254]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── e
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── f
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── <214> Unary [-]
                    │           ╰── Constant Int [20]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── g
                    │   ├── Type
                    │   │   ╰── Signed Char
                    │   ╰── Initializer
                    │       ╰── Constant Int [60]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── h
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── Constant Int [100]
                    ╰── Return
                        ╰── <246> FunctionCall [check_args]
                            ├── <231> Var [a]
                            ├── <233> Var [b]
                            ├── <235> Var [c]
                            ├── <237> Var [d]
                            ├── <239> Var [e]
                            ├── <241> Var [f]
                            ├── <243> Var [g]
                            ╰── <245> Var [h]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_chars_char_expressions() {
    let src = r#"
        int add_chars(char c1, char c2) {
            return c1 + c2;
        }
        int divide_chars(unsigned char c1, unsigned char c2) {
            return c1 / c2;
        }
        int le(char c1, char c2) {
            return c1 <= c2;
        }
        int subscript_char(int *ptr, char idx){
            return ptr[idx];
        }
        int *sub_char_from_pointer(int *ptr, signed char idx) {
            return ptr - idx;
        }
        int and_char(signed char c1, int i) {
            return c1 && i;
        }
        int or_char(signed char c1, unsigned char c2) {
            return c1 || c2;
        }
        int test_for_loop_char(void) {
            int counter = 0;
            for (signed char s = 127; s > 0; s = s - 1) {
                counter = counter + 1;
            }
            return (counter == 127);
        }
        int main(void) {
            char c1 = 8;
            char c2 = 4;
            if (add_chars(c1, c2) != 12) {
                return 1;
            }
            unsigned char uc1 = 250;
            unsigned char uc2 = 25;
            if (divide_chars(uc1, uc2) != 10) {
                return 2;
            }
            if (le(c1, c2)) {
                return 3;
            }
            if (!le(c2, c2)) {
                return 4;
            }
            int arr[4] = {11, 12, 13, 14};
            char idx = 2;
            if (subscript_char(arr, idx) != 13) {
                return 5;
            }
            signed char offset = 1;
            if (sub_char_from_pointer(arr + 1, offset) != arr) {
                return 6;
            }
            signed char zero = 0;
            if (zero) {
                return 7;
            }
            if (and_char(zero, 12)) {
                return 8;
            }
            uc2 = 0;
            if (or_char(zero, uc2)) {
                return 9;
            }
            if (!test_for_loop_char()) {
                return 10;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [add_chars]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── c1
            │   │   │   ╰── Type
            │   │   │       ╰── Char
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── c2
            │   │       ╰── Type
            │   │           ╰── Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <16>  [+]
            │               ├── <12> Var [c1]
            │               ╰── <15> Var [c2]
            ├── Function [divide_chars]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── c1
            │   │   │   ╰── Type
            │   │   │       ╰── Unsigned Char
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── c2
            │   │       ╰── Type
            │   │           ╰── Unsigned Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <35>  [/]
            │               ├── <31> Var [c1]
            │               ╰── <34> Var [c2]
            ├── Function [le]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── c1
            │   │   │   ╰── Type
            │   │   │       ╰── Char
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── c2
            │   │       ╰── Type
            │   │           ╰── Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <54>  [<=]
            │               ├── <50> Var [c1]
            │               ╰── <53> Var [c2]
            ├── Function [subscript_char]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── ptr
            │   │   │   ╰── Type
            │   │   │       ╰── Pointer
            │   │   │           ╰── Int
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── idx
            │   │       ╰── Type
            │   │           ╰── Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <73> Subscript
            │               ├── <70> Var [ptr]
            │               ╰── <72> Var [idx]
            ├── Function [sub_char_from_pointer]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── ptr
            │   │   │   ╰── Type
            │   │   │       ╰── Pointer
            │   │   │           ╰── Int
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── idx
            │   │       ╰── Type
            │   │           ╰── Signed Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <94>  [-]
            │               ├── <90> Var [ptr]
            │               ╰── <93> Var [idx]
            ├── Function [and_char]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── c1
            │   │   │   ╰── Type
            │   │   │       ╰── Signed Char
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── i
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <113>  [&&]
            │               ├── <109> Var [c1]
            │               ╰── <112> Var [i]
            ├── Function [or_char]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── c1
            │   │   │   ╰── Type
            │   │   │       ╰── Signed Char
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── c2
            │   │       ╰── Type
            │   │           ╰── Unsigned Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <132>  [||]
            │               ├── <128> Var [c1]
            │               ╰── <131> Var [c2]
            ├── Function [test_for_loop_char]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── counter
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── Constant Int [0]
            │       ├── For
            │       │   ├── Init
            │       │   │   ╰── VarDeclaration
            │       │   │       ├── Name
            │       │   │       │   ╰── s
            │       │   │       ├── Type
            │       │   │       │   ╰── Signed Char
            │       │   │       ╰── Initializer
            │       │   │           ╰── Constant Int [127]
            │       │   ├── Condition
            │       │   │   ╰── <157>  [>]
            │       │   │       ├── <154> Var [s]
            │       │   │       ╰── Constant Int [0]
            │       │   ├── Condition
            │       │   │   ╰── <166> Assign [=]
            │       │   │       ├── <159> Var [s]
            │       │   │       ╰── <165>  [-]
            │       │   │           ├── <162> Var [s]
            │       │   │           ╰── Constant Int [1]
            │       │   ╰── Block
            │       │       ╰── <175> Assign [=]
            │       │           ├── <168> Var [counter]
            │       │           ╰── <174>  [+]
            │       │               ├── <171> Var [counter]
            │       │               ╰── Constant Int [1]
            │       ╰── Return
            │           ╰── <185>  [==]
            │               ├── <181> Var [counter]
            │               ╰── Constant Int [127]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c1
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── Constant Int [8]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c2
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <213>  [!=]
                    │   │       ├── <210> FunctionCall [add_chars]
                    │   │       │   ├── <207> Var [c1]
                    │   │       │   ╰── <209> Var [c2]
                    │   │       ╰── Constant Int [12]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── uc1
                    │   ├── Type
                    │   │   ╰── Unsigned Char
                    │   ╰── Initializer
                    │       ╰── Constant Int [250]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── uc2
                    │   ├── Type
                    │   │   ╰── Unsigned Char
                    │   ╰── Initializer
                    │       ╰── Constant Int [25]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <239>  [!=]
                    │   │       ├── <236> FunctionCall [divide_chars]
                    │   │       │   ├── <233> Var [uc1]
                    │   │       │   ╰── <235> Var [uc2]
                    │   │       ╰── Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <250> FunctionCall [le]
                    │   │       ├── <247> Var [c1]
                    │   │       ╰── <249> Var [c2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <263> Unary [!]
                    │   │       ╰── <262> FunctionCall [le]
                    │   │           ├── <259> Var [c2]
                    │   │           ╰── <261> Var [c2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── arr
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 4
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── Constant Int [11]
                    │           ├── Constant Int [12]
                    │           ├── Constant Int [13]
                    │           ╰── Constant Int [14]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── idx
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <298>  [!=]
                    │   │       ├── <295> FunctionCall [subscript_char]
                    │   │       │   ├── <292> Var [arr]
                    │   │       │   ╰── <294> Var [idx]
                    │   │       ╰── Constant Int [13]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── offset
                    │   ├── Type
                    │   │   ╰── Signed Char
                    │   ╰── Initializer
                    │       ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <322>  [!=]
                    │   │       ├── <318> FunctionCall [sub_char_from_pointer]
                    │   │       │   ├── <315>  [+]
                    │   │       │   │   ├── <312> Var [arr]
                    │   │       │   │   ╰── Constant Int [1]
                    │   │       │   ╰── <317> Var [offset]
                    │   │       ╰── <321> Var [arr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── zero
                    │   ├── Type
                    │   │   ╰── Signed Char
                    │   ╰── Initializer
                    │       ╰── Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <335> Var [zero]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <345> FunctionCall [and_char]
                    │   │       ├── <343> Var [zero]
                    │   │       ╰── Constant Int [12]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [8]
                    ├── <355> Assign [=]
                    │   ├── <352> Var [uc2]
                    │   ╰── Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <362> FunctionCall [or_char]
                    │   │       ├── <359> Var [zero]
                    │   │       ╰── <361> Var [uc2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <371> Unary [!]
                    │   │       ╰── <370> FunctionCall [test_for_loop_char]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [10]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_chars_common_type() {
    let src = r#"
        long ternary(int flag, char c) {
            return flag ? c : 1u;
        }
        int char_lt_int(char c, int i) {
            return c < i;
        }
        int uchar_gt_long(unsigned char uc, long l) {
            return uc > l;
        }
        int char_lt_uchar(char c, unsigned char u) {
            return c < u;
        }
        int signed_char_le_char(signed char s, char c) {
            return s <= c;
        }
        char ten = 10;
        int multiply(void) {
            char i = 10.75 * ten;
            return i == 107;
        }
        int main(void) {
            if (ternary(1, -10) != 4294967286l) {
                return 1;
            }
            if (!char_lt_int((char)1, 256)) {
                return 2;
            }
            if (!uchar_gt_long((unsigned char)100, -2)) {
                return 3;
            }
            char c = -1;
            unsigned char u = 2;
            if (!char_lt_uchar(c, u)) {
                return 4;
            }
            signed char s = -1;
            if (!signed_char_le_char(s, c)) {
                return 5;
            }
            if (!multiply()) {
                return 6;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [ternary]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── flag
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── c
            │   │       ╰── Type
            │   │           ╰── Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <{node_id}> Conditional [?]
            │               ├── <12> Var [flag]
            │               ├── Then
            │               │   ╰── <14> Var [c]
            │               ╰── Else
            │                   ╰── Constant UInt [1]
            ├── Function [char_lt_int]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── c
            │   │   │   ╰── Type
            │   │   │       ╰── Char
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── i
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <35>  [<]
            │               ├── <31> Var [c]
            │               ╰── <34> Var [i]
            ├── Function [uchar_gt_long]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── uc
            │   │   │   ╰── Type
            │   │   │       ╰── Unsigned Char
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── l
            │   │       ╰── Type
            │   │           ╰── Long
            │   ╰── Body
            │       ╰── Return
            │           ╰── <54>  [>]
            │               ├── <50> Var [uc]
            │               ╰── <53> Var [l]
            ├── Function [char_lt_uchar]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── c
            │   │   │   ╰── Type
            │   │   │       ╰── Char
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── u
            │   │       ╰── Type
            │   │           ╰── Unsigned Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <73>  [<]
            │               ├── <69> Var [c]
            │               ╰── <72> Var [u]
            ├── Function [signed_char_le_char]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── s
            │   │   │   ╰── Type
            │   │   │       ╰── Signed Char
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── c
            │   │       ╰── Type
            │   │           ╰── Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <92>  [<=]
            │               ├── <88> Var [s]
            │               ╰── <91> Var [c]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── ten
            │   ├── Type
            │   │   ╰── Char
            │   ╰── Initializer
            │       ╰── Constant Int [10]
            ├── Function [multiply]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── i
            │       │   ├── Type
            │       │   │   ╰── Char
            │       │   ╰── Initializer
            │       │       ╰── <113>  [*]
            │       │           ├── Constant Double [+1.075e1]
            │       │           ╰── <112> Var [ten]
            │       ╰── Return
            │           ╰── <120>  [==]
            │               ├── <117> Var [i]
            │               ╰── Constant Int [107]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <136>  [!=]
                    │   │       ├── <133> FunctionCall [ternary]
                    │   │       │   ├── Constant Int [1]
                    │   │       │   ╰── <132> Unary [-]
                    │   │       │       ╰── Constant Int [10]
                    │   │       ╰── Constant Long [4294967286]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <150> Unary [!]
                    │   │       ╰── <149> FunctionCall [char_lt_int]
                    │   │           ├── <147> Cast
                    │   │           │   ├── Target
                    │   │           │   │   ╰── Char
                    │   │           │   ╰── Expression
                    │   │           │       ╰── Constant Int [1]
                    │   │           ╰── Constant Int [256]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <166> Unary [!]
                    │   │       ╰── <165> FunctionCall [uchar_gt_long]
                    │   │           ├── <161> Cast
                    │   │           │   ├── Target
                    │   │           │   │   ╰── Unsigned Char
                    │   │           │   ╰── Expression
                    │   │           │       ╰── Constant Int [100]
                    │   │           ╰── <164> Unary [-]
                    │   │               ╰── Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── <177> Unary [-]
                    │           ╰── Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── u
                    │   ├── Type
                    │   │   ╰── Unsigned Char
                    │   ╰── Initializer
                    │       ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <193> Unary [!]
                    │   │       ╰── <192> FunctionCall [char_lt_uchar]
                    │   │           ├── <189> Var [c]
                    │   │           ╰── <191> Var [u]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── s
                    │   ├── Type
                    │   │   ╰── Signed Char
                    │   ╰── Initializer
                    │       ╰── <204> Unary [-]
                    │           ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <214> Unary [!]
                    │   │       ╰── <213> FunctionCall [signed_char_le_char]
                    │   │           ├── <210> Var [s]
                    │   │           ╰── <212> Var [c]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <223> Unary [!]
                    │   │       ╰── <222> FunctionCall [multiply]
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
fn test_valid_chars_convert_by_assignment() {
    let src = r#"
        int check_int(int converted, int expected) {
            return (converted == expected);
        }
        int check_uint(unsigned int converted, unsigned int expected) {
            return (converted == expected);
        }
        int check_long(long converted, long expected) {
            return (converted == expected);
        }
        int check_ulong(unsigned long converted, unsigned long expected) {
            return (converted == expected);
        }
        int check_double(double converted, double expected) {
            return (converted == expected);
        }
        int check_char(char converted, char expected) {
            return (converted == expected);
        }
        int check_uchar(unsigned char converted, unsigned char expected) {
            return (converted == expected);
        }
        int check_char_on_stack(signed char expected, int dummy1, int dummy2,
                                int dummy3, int dummy4, int dummy5, int dummy6,
                                signed char converted) {
            return converted == expected;
        }
        int return_extended_uchar(unsigned char c) {
            return c;
        }
        unsigned long return_extended_schar(signed char sc) {
            return sc;
        }
        unsigned char return_truncated_long(long l) {
            return l;
        }
        int main(void) {
            signed char sc = -10;
            if (!check_long(sc, -10l)) {
                return 1;
            }
            if (!check_uint(sc, 4294967286u)) {
                return 2;
            }
            if (!check_double(sc, -10.0)) {
                return 3;
            }
            unsigned char uc = 246;
            if (!check_uchar(sc, uc)) {
                return 4;
            }
            char c = -10;
            if (!check_char(-10, c)) {
                return 5;
            }
            if (!check_char(4294967286u, c)) {
                return 6;
            }
            if (!check_char(-10.0, c)) {
                return 7;
            }
            if (!check_char_on_stack(c, 0, 0, 0, 0, 0, 0, -10.0)) {
                return 8;
            }
            if (!check_int(uc, 246)) {
                return 9;
            }
            if (!check_ulong(uc, 246ul)) {
                return 10;
            }
            char expected_char = -10;
            if (!check_char(uc, expected_char)) {
                return 11;
            }
            if (!check_uchar(18446744073709551606ul, uc)) {
                return 12;
            }
            if (return_extended_uchar(uc) != 246) {
                return 13;
            }
            if (return_extended_schar(sc) != 18446744073709551606ul) {
                return 14;
            }
            if (return_truncated_long(5369233654l) != uc) {
                return 15;
            }
            char array[3] = {0, 0, 0};
            array[1] = 128;
            if (array[0] || array[2] || array[1] != -128) {
                return 16;
            }
            array[1] = 9224497936761618562ul;
            if (array[0] || array[2] || array[1] != -126) {
                return 17;
            }
            array[1] = -2.6;
            if (array[0] || array[2] || array[1] != -2) {
                return 18;
            }
            unsigned char uchar_array[3] = {0, 0, 0};
            uchar_array[1] = 17592186044416l;
            if (uchar_array[0] || uchar_array[2] || uchar_array[1] != 0) {
                return 19;
            }
            uchar_array[1] = 2147483898u;
            if (uchar_array[0] || uchar_array[2] || uchar_array[1] != 250) {
                return 20;
            }
            unsigned int ui = 4294967295U;
            static unsigned char
                uc_static;
            ui = uc_static;
            if (ui) {
                return 21;
            }
            signed long l = -1;
            static signed s_static =
                0;
            l = s_static;
            if (l) {
                return 22;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [check_int]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── converted
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── expected
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <17>  [==]
            │               ├── <12> Var [converted]
            │               ╰── <15> Var [expected]
            ├── Function [check_uint]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── converted
            │   │   │   ╰── Type
            │   │   │       ╰── Unsigned Int
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── expected
            │   │       ╰── Type
            │   │           ╰── Unsigned Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <37>  [==]
            │               ├── <32> Var [converted]
            │               ╰── <35> Var [expected]
            ├── Function [check_long]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── converted
            │   │   │   ╰── Type
            │   │   │       ╰── Long
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── expected
            │   │       ╰── Type
            │   │           ╰── Long
            │   ╰── Body
            │       ╰── Return
            │           ╰── <57>  [==]
            │               ├── <52> Var [converted]
            │               ╰── <55> Var [expected]
            ├── Function [check_ulong]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── converted
            │   │   │   ╰── Type
            │   │   │       ╰── Unsigned Long
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── expected
            │   │       ╰── Type
            │   │           ╰── Unsigned Long
            │   ╰── Body
            │       ╰── Return
            │           ╰── <77>  [==]
            │               ├── <72> Var [converted]
            │               ╰── <75> Var [expected]
            ├── Function [check_double]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── converted
            │   │   │   ╰── Type
            │   │   │       ╰── Double
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── expected
            │   │       ╰── Type
            │   │           ╰── Double
            │   ╰── Body
            │       ╰── Return
            │           ╰── <97>  [==]
            │               ├── <92> Var [converted]
            │               ╰── <95> Var [expected]
            ├── Function [check_char]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── converted
            │   │   │   ╰── Type
            │   │   │       ╰── Char
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── expected
            │   │       ╰── Type
            │   │           ╰── Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <117>  [==]
            │               ├── <112> Var [converted]
            │               ╰── <115> Var [expected]
            ├── Function [check_uchar]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── converted
            │   │   │   ╰── Type
            │   │   │       ╰── Unsigned Char
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── expected
            │   │       ╰── Type
            │   │           ╰── Unsigned Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <137>  [==]
            │               ├── <132> Var [converted]
            │               ╰── <135> Var [expected]
            ├── Function [check_char_on_stack]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── expected
            │   │   │   ╰── Type
            │   │   │       ╰── Signed Char
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── dummy1
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── dummy2
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── dummy3
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── dummy4
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── dummy5
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── dummy6
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── converted
            │   │       ╰── Type
            │   │           ╰── Signed Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <174>  [==]
            │               ├── <170> Var [converted]
            │               ╰── <173> Var [expected]
            ├── Function [return_extended_uchar]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── c
            │   │       ╰── Type
            │   │           ╰── Unsigned Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <186> Var [c]
            ├── Function [return_extended_schar]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── sc
            │   │       ╰── Type
            │   │           ╰── Signed Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <198> Var [sc]
            ├── Function [return_truncated_long]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── l
            │   │       ╰── Type
            │   │           ╰── Long
            │   ╰── Body
            │       ╰── Return
            │           ╰── <210> Var [l]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── sc
                    │   ├── Type
                    │   │   ╰── Signed Char
                    │   ╰── Initializer
                    │       ╰── <223> Unary [-]
                    │           ╰── Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <234> Unary [!]
                    │   │       ╰── <233> FunctionCall [check_long]
                    │   │           ├── <229> Var [sc]
                    │   │           ╰── <232> Unary [-]
                    │   │               ╰── Constant Long [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <246> Unary [!]
                    │   │       ╰── <245> FunctionCall [check_uint]
                    │   │           ├── <243> Var [sc]
                    │   │           ╰── Constant UInt [4294967286]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <260> Unary [!]
                    │   │       ╰── <259> FunctionCall [check_double]
                    │   │           ├── <255> Var [sc]
                    │   │           ╰── <258> Unary [-]
                    │   │               ╰── Constant Double [+1e1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── uc
                    │   ├── Type
                    │   │   ╰── Unsigned Char
                    │   ╰── Initializer
                    │       ╰── Constant Int [246]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <279> Unary [!]
                    │   │       ╰── <278> FunctionCall [check_uchar]
                    │   │           ├── <275> Var [sc]
                    │   │           ╰── <277> Var [uc]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── <290> Unary [-]
                    │           ╰── Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <301> Unary [!]
                    │   │       ╰── <300> FunctionCall [check_char]
                    │   │           ├── <297> Unary [-]
                    │   │           │   ╰── Constant Int [10]
                    │   │           ╰── <299> Var [c]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <313> Unary [!]
                    │   │       ╰── <312> FunctionCall [check_char]
                    │   │           ├── Constant UInt [4294967286]
                    │   │           ╰── <311> Var [c]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <327> Unary [!]
                    │   │       ╰── <326> FunctionCall [check_char]
                    │   │           ├── <323> Unary [-]
                    │   │           │   ╰── Constant Double [+1e1]
                    │   │           ╰── <325> Var [c]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <347> Unary [!]
                    │   │       ╰── <346> FunctionCall [check_char_on_stack]
                    │   │           ├── <336> Var [c]
                    │   │           ├── Constant Int [0]
                    │   │           ├── Constant Int [0]
                    │   │           ├── Constant Int [0]
                    │   │           ├── Constant Int [0]
                    │   │           ├── Constant Int [0]
                    │   │           ├── Constant Int [0]
                    │   │           ╰── <345> Unary [-]
                    │   │               ╰── Constant Double [+1e1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <359> Unary [!]
                    │   │       ╰── <358> FunctionCall [check_int]
                    │   │           ├── <356> Var [uc]
                    │   │           ╰── Constant Int [246]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <371> Unary [!]
                    │   │       ╰── <370> FunctionCall [check_ulong]
                    │   │           ├── <368> Var [uc]
                    │   │           ╰── Constant ULong [246]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [10]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── expected_char
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── <382> Unary [-]
                    │           ╰── Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <392> Unary [!]
                    │   │       ╰── <391> FunctionCall [check_char]
                    │   │           ├── <388> Var [uc]
                    │   │           ╰── <390> Var [expected_char]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [11]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <404> Unary [!]
                    │   │       ╰── <403> FunctionCall [check_uchar]
                    │   │           ├── Constant ULong [18446744073709551606]
                    │   │           ╰── <402> Var [uc]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [12]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <416>  [!=]
                    │   │       ├── <413> FunctionCall [return_extended_uchar]
                    │   │       │   ╰── <412> Var [uc]
                    │   │       ╰── Constant Int [246]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [13]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <428>  [!=]
                    │   │       ├── <425> FunctionCall [return_extended_schar]
                    │   │       │   ╰── <424> Var [sc]
                    │   │       ╰── Constant ULong [18446744073709551606]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [14]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <440>  [!=]
                    │   │       ├── <436> FunctionCall [return_truncated_long]
                    │   │       │   ╰── Constant Long [5369233654]
                    │   │       ╰── <439> Var [uc]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [15]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── array
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 3
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── Constant Int [0]
                    │           ├── Constant Int [0]
                    │           ╰── Constant Int [0]
                    ├── <465> Assign [=]
                    │   ├── <462> Subscript
                    │   │   ├── <460> Var [array]
                    │   │   ╰── Constant Int [1]
                    │   ╰── Constant Int [128]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <487>  [||]
                    │   │       ├── <476>  [||]
                    │   │       │   ├── <470> Subscript
                    │   │       │   │   ├── <468> Var [array]
                    │   │       │   │   ╰── Constant Int [0]
                    │   │       │   ╰── <475> Subscript
                    │   │       │       ├── <473> Var [array]
                    │   │       │       ╰── Constant Int [2]
                    │   │       ╰── <486>  [!=]
                    │   │           ├── <481> Subscript
                    │   │           │   ├── <479> Var [array]
                    │   │           │   ╰── Constant Int [1]
                    │   │           ╰── <485> Unary [-]
                    │   │               ╰── Constant Int [128]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [16]
                    ├── <499> Assign [=]
                    │   ├── <496> Subscript
                    │   │   ├── <494> Var [array]
                    │   │   ╰── Constant Int [1]
                    │   ╰── Constant ULong [9224497936761618562]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <521>  [||]
                    │   │       ├── <510>  [||]
                    │   │       │   ├── <504> Subscript
                    │   │       │   │   ├── <502> Var [array]
                    │   │       │   │   ╰── Constant Int [0]
                    │   │       │   ╰── <509> Subscript
                    │   │       │       ├── <507> Var [array]
                    │   │       │       ╰── Constant Int [2]
                    │   │       ╰── <520>  [!=]
                    │   │           ├── <515> Subscript
                    │   │           │   ├── <513> Var [array]
                    │   │           │   ╰── Constant Int [1]
                    │   │           ╰── <519> Unary [-]
                    │   │               ╰── Constant Int [126]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [17]
                    ├── <535> Assign [=]
                    │   ├── <530> Subscript
                    │   │   ├── <528> Var [array]
                    │   │   ╰── Constant Int [1]
                    │   ╰── <534> Unary [-]
                    │       ╰── Constant Double [+2.6e0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <557>  [||]
                    │   │       ├── <546>  [||]
                    │   │       │   ├── <540> Subscript
                    │   │       │   │   ├── <538> Var [array]
                    │   │       │   │   ╰── Constant Int [0]
                    │   │       │   ╰── <545> Subscript
                    │   │       │       ├── <543> Var [array]
                    │   │       │       ╰── Constant Int [2]
                    │   │       ╰── <556>  [!=]
                    │   │           ├── <551> Subscript
                    │   │           │   ├── <549> Var [array]
                    │   │           │   ╰── Constant Int [1]
                    │   │           ╰── <555> Unary [-]
                    │   │               ╰── Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [18]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── uchar_array
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 3
                    │   │       ╰── Unsigned Char
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── Constant Int [0]
                    │           ├── Constant Int [0]
                    │           ╰── Constant Int [0]
                    ├── <582> Assign [=]
                    │   ├── <579> Subscript
                    │   │   ├── <577> Var [uchar_array]
                    │   │   ╰── Constant Int [1]
                    │   ╰── Constant Long [17592186044416]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <602>  [||]
                    │   │       ├── <593>  [||]
                    │   │       │   ├── <587> Subscript
                    │   │       │   │   ├── <585> Var [uchar_array]
                    │   │       │   │   ╰── Constant Int [0]
                    │   │       │   ╰── <592> Subscript
                    │   │       │       ├── <590> Var [uchar_array]
                    │   │       │       ╰── Constant Int [2]
                    │   │       ╰── <601>  [!=]
                    │   │           ├── <598> Subscript
                    │   │           │   ├── <596> Var [uchar_array]
                    │   │           │   ╰── Constant Int [1]
                    │   │           ╰── Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [19]
                    ├── <614> Assign [=]
                    │   ├── <611> Subscript
                    │   │   ├── <609> Var [uchar_array]
                    │   │   ╰── Constant Int [1]
                    │   ╰── Constant UInt [2147483898]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <634>  [||]
                    │   │       ├── <625>  [||]
                    │   │       │   ├── <619> Subscript
                    │   │       │   │   ├── <617> Var [uchar_array]
                    │   │       │   │   ╰── Constant Int [0]
                    │   │       │   ╰── <624> Subscript
                    │   │       │       ├── <622> Var [uchar_array]
                    │   │       │       ╰── Constant Int [2]
                    │   │       ╰── <633>  [!=]
                    │   │           ├── <630> Subscript
                    │   │           │   ├── <628> Var [uchar_array]
                    │   │           │   ╰── Constant Int [1]
                    │   │           ╰── Constant Int [250]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [20]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ui
                    │   ├── Type
                    │   │   ╰── Unsigned Int
                    │   ╰── Initializer
                    │       ╰── Constant UInt [4294967295]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── uc_static
                    │   ├── Type
                    │   │   ╰── Unsigned Char
                    │   ╰── Static
                    ├── <656> Assign [=]
                    │   ├── <652> Var [ui]
                    │   ╰── <655> Var [uc_static]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <659> Var [ui]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [21]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <670> Unary [-]
                    │           ╰── Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── s_static
                    │   ├── Type
                    │   │   ╰── Int
                    │   ├── Initializer
                    │   │   ╰── Constant Int [0]
                    │   ╰── Static
                    ├── <685> Assign [=]
                    │   ├── <681> Var [l]
                    │   ╰── <684> Var [s_static]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <688> Var [l]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [22]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_chars_explicit_casts() {
    let src = r#"
        unsigned char char_to_uchar(char c) {
            return (unsigned char)c;
        }
        signed char char_to_schar(char c) {
            return (signed char)c;
        }
        char uchar_to_char(unsigned char u) {
            return (char)u;
        }
        char schar_to_char(signed char u) {
            return (char)u;
        }
        signed char uchar_to_schar(unsigned char u) {
            return (signed char)u;
        }
        unsigned char schar_to_uchar(signed char u) {
            return (unsigned char)u;
        }
        int char_to_int(char c) {
            return (int)c;
        }
        unsigned int char_to_uint(char c) {
            return (unsigned int)c;
        }
        long char_to_long(char c) {
            return (long)c;
        }
        unsigned long char_to_ulong(char c) {
            return (unsigned long)c;
        }
        double char_to_double(char c) {
            return (double)c;
        }
        int schar_to_int(signed char s) {
            return (int)s;
        }
        unsigned int schar_to_uint(signed char s) {
            return (unsigned int)s;
        }
        long schar_to_long(signed char s) {
            return (long)s;
        }
        unsigned long schar_to_ulong(signed char s) {
            return (unsigned long)s;
        }
        double schar_to_double(signed char s) {
            return (double)s;
        }
        int uchar_to_int(unsigned char u) {
            return (int)u;
        }
        unsigned int uchar_to_uint(unsigned char u) {
            return (unsigned int)u;
        }
        long uchar_to_long(unsigned char u) {
            return (long)u;
        }
        unsigned long uchar_to_ulong(unsigned char u) {
            return (unsigned long)u;
        }
        double uchar_to_double(unsigned char u) {
            return (double)u;
        }
        char int_to_char(int i) {
            return (char)i;
        }
        char uint_to_char(unsigned int u) {
            return (char)u;
        }
        char double_to_char(double d) {
            return (char)d;
        }
        signed char long_to_schar(long l) {
            return (signed char)l;
        }
        signed char ulong_to_schar(unsigned long l) {
            return (signed char)l;
        }
        unsigned char int_to_uchar(int i) {
            return (unsigned char)i;
        }
        unsigned char uint_to_uchar(unsigned int ui) {
            return (unsigned char)ui;
        }
        unsigned char long_to_uchar(long l) {
            return (unsigned char)l;
        }
        unsigned char ulong_to_uchar(unsigned long ul) {
            return (unsigned char)ul;
        }
        unsigned char double_to_uchar(double d) {
            return (unsigned char)d;
        }
        int main(void) {
            char c = 127;
            if (char_to_uchar(c) != 127) {
                return 1;
            }
            if (char_to_int(c) != 127) {
                return 2;
            }
            if (char_to_ulong(c) != 127) {
                return 3;
            }
            signed char sc = -10;
            if (schar_to_uchar(sc) != 246) {
                return 4;
            }
            if (schar_to_long(sc) != -10) {
                return 5;
            }
            if (schar_to_uint(sc) != 4294967286u) {
                return 6;
            }
            if (schar_to_double(sc) != -10.0) {
                return 7;
            }
            unsigned char uc = 250;
            if (uchar_to_int(uc) != 250) {
                return 8;
            }
            if (uchar_to_long(uc) != 250) {
                return 9;
            }
            if (uchar_to_uint(uc) != 250) {
                return 10;
            }
            if (uchar_to_ulong(uc) != 250) {
                return 11;
            }
            if (uchar_to_double(uc) != 250.0) {
                return 12;
            }
            if (uchar_to_schar(uc) != -6) {
                return 13;
            }
            if (uchar_to_char(uc) != -6) {
                return 14;
            }
            c = (char)-128;
            if (int_to_char(128) != c) {
                return 15;
            }
            c = (char)-6;
            if (uint_to_char(2147483898u) != c) {
                return 16;
            }
            c = (char)-2;
            if (double_to_char(-2.6) != c) {
                return 17;
            }
            if (long_to_schar(17592186044416l)) {
                return 18;
            }
            sc = (signed char)-126;
            if (ulong_to_schar(9224497936761618562ul) != sc) {
                return 19;
            }
            uc = (unsigned char)200;
            if (int_to_uchar(-1234488) != uc) {
                return 20;
            }
            if (uint_to_uchar(4293732808) != uc) {
                return 21;
            }
            if (long_to_uchar(-36283884951096l) != uc) {
                return 22;
            }
            if (ulong_to_uchar(9224497936761618632ul) != uc) {
                return 23;
            }
            if (double_to_uchar(200.99) != uc) {
                return 24;
            }
            static long *null_ptr;
            char zero = (char)null_ptr;
            if (zero) {
                return 25;
            }
            c = 32;
            int *i = (int *)c;
            if ((char)i != c) {
                return 26;
            }
            if ((char)300 != (char)44) {
                return 27;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [char_to_uchar]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── c
            │   │       ╰── Type
            │   │           ╰── Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <12> Cast
            │               ├── Target
            │               │   ╰── Unsigned Char
            │               ╰── Expression
            │                   ╰── <11> Var [c]
            ├── Function [char_to_schar]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── c
            │   │       ╰── Type
            │   │           ╰── Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <27> Cast
            │               ├── Target
            │               │   ╰── Signed Char
            │               ╰── Expression
            │                   ╰── <26> Var [c]
            ├── Function [uchar_to_char]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── u
            │   │       ╰── Type
            │   │           ╰── Unsigned Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <42> Cast
            │               ├── Target
            │               │   ╰── Char
            │               ╰── Expression
            │                   ╰── <41> Var [u]
            ├── Function [schar_to_char]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── u
            │   │       ╰── Type
            │   │           ╰── Signed Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <57> Cast
            │               ├── Target
            │               │   ╰── Char
            │               ╰── Expression
            │                   ╰── <56> Var [u]
            ├── Function [uchar_to_schar]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── u
            │   │       ╰── Type
            │   │           ╰── Unsigned Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <72> Cast
            │               ├── Target
            │               │   ╰── Signed Char
            │               ╰── Expression
            │                   ╰── <71> Var [u]
            ├── Function [schar_to_uchar]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── u
            │   │       ╰── Type
            │   │           ╰── Signed Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <87> Cast
            │               ├── Target
            │               │   ╰── Unsigned Char
            │               ╰── Expression
            │                   ╰── <86> Var [u]
            ├── Function [char_to_int]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── c
            │   │       ╰── Type
            │   │           ╰── Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <102> Cast
            │               ├── Target
            │               │   ╰── Int
            │               ╰── Expression
            │                   ╰── <101> Var [c]
            ├── Function [char_to_uint]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── c
            │   │       ╰── Type
            │   │           ╰── Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <117> Cast
            │               ├── Target
            │               │   ╰── Unsigned Int
            │               ╰── Expression
            │                   ╰── <116> Var [c]
            ├── Function [char_to_long]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── c
            │   │       ╰── Type
            │   │           ╰── Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <132> Cast
            │               ├── Target
            │               │   ╰── Long
            │               ╰── Expression
            │                   ╰── <131> Var [c]
            ├── Function [char_to_ulong]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── c
            │   │       ╰── Type
            │   │           ╰── Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <147> Cast
            │               ├── Target
            │               │   ╰── Unsigned Long
            │               ╰── Expression
            │                   ╰── <146> Var [c]
            ├── Function [char_to_double]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── c
            │   │       ╰── Type
            │   │           ╰── Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <162> Cast
            │               ├── Target
            │               │   ╰── Double
            │               ╰── Expression
            │                   ╰── <161> Var [c]
            ├── Function [schar_to_int]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── s
            │   │       ╰── Type
            │   │           ╰── Signed Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <177> Cast
            │               ├── Target
            │               │   ╰── Int
            │               ╰── Expression
            │                   ╰── <176> Var [s]
            ├── Function [schar_to_uint]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── s
            │   │       ╰── Type
            │   │           ╰── Signed Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <192> Cast
            │               ├── Target
            │               │   ╰── Unsigned Int
            │               ╰── Expression
            │                   ╰── <191> Var [s]
            ├── Function [schar_to_long]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── s
            │   │       ╰── Type
            │   │           ╰── Signed Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <207> Cast
            │               ├── Target
            │               │   ╰── Long
            │               ╰── Expression
            │                   ╰── <206> Var [s]
            ├── Function [schar_to_ulong]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── s
            │   │       ╰── Type
            │   │           ╰── Signed Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <222> Cast
            │               ├── Target
            │               │   ╰── Unsigned Long
            │               ╰── Expression
            │                   ╰── <221> Var [s]
            ├── Function [schar_to_double]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── s
            │   │       ╰── Type
            │   │           ╰── Signed Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <237> Cast
            │               ├── Target
            │               │   ╰── Double
            │               ╰── Expression
            │                   ╰── <236> Var [s]
            ├── Function [uchar_to_int]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── u
            │   │       ╰── Type
            │   │           ╰── Unsigned Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <252> Cast
            │               ├── Target
            │               │   ╰── Int
            │               ╰── Expression
            │                   ╰── <251> Var [u]
            ├── Function [uchar_to_uint]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── u
            │   │       ╰── Type
            │   │           ╰── Unsigned Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <267> Cast
            │               ├── Target
            │               │   ╰── Unsigned Int
            │               ╰── Expression
            │                   ╰── <266> Var [u]
            ├── Function [uchar_to_long]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── u
            │   │       ╰── Type
            │   │           ╰── Unsigned Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <282> Cast
            │               ├── Target
            │               │   ╰── Long
            │               ╰── Expression
            │                   ╰── <281> Var [u]
            ├── Function [uchar_to_ulong]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── u
            │   │       ╰── Type
            │   │           ╰── Unsigned Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <297> Cast
            │               ├── Target
            │               │   ╰── Unsigned Long
            │               ╰── Expression
            │                   ╰── <296> Var [u]
            ├── Function [uchar_to_double]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── u
            │   │       ╰── Type
            │   │           ╰── Unsigned Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <312> Cast
            │               ├── Target
            │               │   ╰── Double
            │               ╰── Expression
            │                   ╰── <311> Var [u]
            ├── Function [int_to_char]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── i
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <327> Cast
            │               ├── Target
            │               │   ╰── Char
            │               ╰── Expression
            │                   ╰── <326> Var [i]
            ├── Function [uint_to_char]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── u
            │   │       ╰── Type
            │   │           ╰── Unsigned Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <342> Cast
            │               ├── Target
            │               │   ╰── Char
            │               ╰── Expression
            │                   ╰── <341> Var [u]
            ├── Function [double_to_char]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── d
            │   │       ╰── Type
            │   │           ╰── Double
            │   ╰── Body
            │       ╰── Return
            │           ╰── <357> Cast
            │               ├── Target
            │               │   ╰── Char
            │               ╰── Expression
            │                   ╰── <356> Var [d]
            ├── Function [long_to_schar]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── l
            │   │       ╰── Type
            │   │           ╰── Long
            │   ╰── Body
            │       ╰── Return
            │           ╰── <372> Cast
            │               ├── Target
            │               │   ╰── Signed Char
            │               ╰── Expression
            │                   ╰── <371> Var [l]
            ├── Function [ulong_to_schar]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── l
            │   │       ╰── Type
            │   │           ╰── Unsigned Long
            │   ╰── Body
            │       ╰── Return
            │           ╰── <387> Cast
            │               ├── Target
            │               │   ╰── Signed Char
            │               ╰── Expression
            │                   ╰── <386> Var [l]
            ├── Function [int_to_uchar]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── i
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <402> Cast
            │               ├── Target
            │               │   ╰── Unsigned Char
            │               ╰── Expression
            │                   ╰── <401> Var [i]
            ├── Function [uint_to_uchar]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── ui
            │   │       ╰── Type
            │   │           ╰── Unsigned Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <417> Cast
            │               ├── Target
            │               │   ╰── Unsigned Char
            │               ╰── Expression
            │                   ╰── <416> Var [ui]
            ├── Function [long_to_uchar]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── l
            │   │       ╰── Type
            │   │           ╰── Long
            │   ╰── Body
            │       ╰── Return
            │           ╰── <432> Cast
            │               ├── Target
            │               │   ╰── Unsigned Char
            │               ╰── Expression
            │                   ╰── <431> Var [l]
            ├── Function [ulong_to_uchar]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── ul
            │   │       ╰── Type
            │   │           ╰── Unsigned Long
            │   ╰── Body
            │       ╰── Return
            │           ╰── <447> Cast
            │               ├── Target
            │               │   ╰── Unsigned Char
            │               ╰── Expression
            │                   ╰── <446> Var [ul]
            ├── Function [double_to_uchar]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── d
            │   │       ╰── Type
            │   │           ╰── Double
            │   ╰── Body
            │       ╰── Return
            │           ╰── <462> Cast
            │               ├── Target
            │               │   ╰── Unsigned Char
            │               ╰── Expression
            │                   ╰── <461> Var [d]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── Constant Int [127]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <482>  [!=]
                    │   │       ├── <479> FunctionCall [char_to_uchar]
                    │   │       │   ╰── <478> Var [c]
                    │   │       ╰── Constant Int [127]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <494>  [!=]
                    │   │       ├── <491> FunctionCall [char_to_int]
                    │   │       │   ╰── <490> Var [c]
                    │   │       ╰── Constant Int [127]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <506>  [!=]
                    │   │       ├── <503> FunctionCall [char_to_ulong]
                    │   │       │   ╰── <502> Var [c]
                    │   │       ╰── Constant Int [127]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── sc
                    │   ├── Type
                    │   │   ╰── Signed Char
                    │   ╰── Initializer
                    │       ╰── <517> Unary [-]
                    │           ╰── Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <526>  [!=]
                    │   │       ├── <523> FunctionCall [schar_to_uchar]
                    │   │       │   ╰── <522> Var [sc]
                    │   │       ╰── Constant Int [246]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <540>  [!=]
                    │   │       ├── <535> FunctionCall [schar_to_long]
                    │   │       │   ╰── <534> Var [sc]
                    │   │       ╰── <539> Unary [-]
                    │   │           ╰── Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <552>  [!=]
                    │   │       ├── <549> FunctionCall [schar_to_uint]
                    │   │       │   ╰── <548> Var [sc]
                    │   │       ╰── Constant UInt [4294967286]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <566>  [!=]
                    │   │       ├── <561> FunctionCall [schar_to_double]
                    │   │       │   ╰── <560> Var [sc]
                    │   │       ╰── <565> Unary [-]
                    │   │           ╰── Constant Double [+1e1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [7]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── uc
                    │   ├── Type
                    │   │   ╰── Unsigned Char
                    │   ╰── Initializer
                    │       ╰── Constant Int [250]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <584>  [!=]
                    │   │       ├── <581> FunctionCall [uchar_to_int]
                    │   │       │   ╰── <580> Var [uc]
                    │   │       ╰── Constant Int [250]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <596>  [!=]
                    │   │       ├── <593> FunctionCall [uchar_to_long]
                    │   │       │   ╰── <592> Var [uc]
                    │   │       ╰── Constant Int [250]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <608>  [!=]
                    │   │       ├── <605> FunctionCall [uchar_to_uint]
                    │   │       │   ╰── <604> Var [uc]
                    │   │       ╰── Constant Int [250]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <620>  [!=]
                    │   │       ├── <617> FunctionCall [uchar_to_ulong]
                    │   │       │   ╰── <616> Var [uc]
                    │   │       ╰── Constant Int [250]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [11]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <632>  [!=]
                    │   │       ├── <629> FunctionCall [uchar_to_double]
                    │   │       │   ╰── <628> Var [uc]
                    │   │       ╰── Constant Double [+2.5e2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [12]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <646>  [!=]
                    │   │       ├── <641> FunctionCall [uchar_to_schar]
                    │   │       │   ╰── <640> Var [uc]
                    │   │       ╰── <645> Unary [-]
                    │   │           ╰── Constant Int [6]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [13]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <660>  [!=]
                    │   │       ├── <655> FunctionCall [uchar_to_char]
                    │   │       │   ╰── <654> Var [uc]
                    │   │       ╰── <659> Unary [-]
                    │   │           ╰── Constant Int [6]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [14]
                    ├── <675> Assign [=]
                    │   ├── <667> Var [c]
                    │   ╰── <674> Cast
                    │       ├── Target
                    │       │   ╰── Char
                    │       ╰── Expression
                    │           ╰── <673> Unary [-]
                    │               ╰── Constant Int [128]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <683>  [!=]
                    │   │       ├── <679> FunctionCall [int_to_char]
                    │   │       │   ╰── Constant Int [128]
                    │   │       ╰── <682> Var [c]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [15]
                    ├── <698> Assign [=]
                    │   ├── <690> Var [c]
                    │   ╰── <697> Cast
                    │       ├── Target
                    │       │   ╰── Char
                    │       ╰── Expression
                    │           ╰── <696> Unary [-]
                    │               ╰── Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <706>  [!=]
                    │   │       ├── <702> FunctionCall [uint_to_char]
                    │   │       │   ╰── Constant UInt [2147483898]
                    │   │       ╰── <705> Var [c]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [16]
                    ├── <721> Assign [=]
                    │   ├── <713> Var [c]
                    │   ╰── <720> Cast
                    │       ├── Target
                    │       │   ╰── Char
                    │       ╰── Expression
                    │           ╰── <719> Unary [-]
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <731>  [!=]
                    │   │       ├── <727> FunctionCall [double_to_char]
                    │   │       │   ╰── <726> Unary [-]
                    │   │       │       ╰── Constant Double [+2.6e0]
                    │   │       ╰── <730> Var [c]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [17]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <739> FunctionCall [long_to_schar]
                    │   │       ╰── Constant Long [17592186044416]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [18]
                    ├── <754> Assign [=]
                    │   ├── <746> Var [sc]
                    │   ╰── <753> Cast
                    │       ├── Target
                    │       │   ╰── Signed Char
                    │       ╰── Expression
                    │           ╰── <752> Unary [-]
                    │               ╰── Constant Int [126]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <762>  [!=]
                    │   │       ├── <758> FunctionCall [ulong_to_schar]
                    │   │       │   ╰── Constant ULong [9224497936761618562]
                    │   │       ╰── <761> Var [sc]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [19]
                    ├── <775> Assign [=]
                    │   ├── <769> Var [uc]
                    │   ╰── <774> Cast
                    │       ├── Target
                    │       │   ╰── Unsigned Char
                    │       ╰── Expression
                    │           ╰── Constant Int [200]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <785>  [!=]
                    │   │       ├── <781> FunctionCall [int_to_uchar]
                    │   │       │   ╰── <780> Unary [-]
                    │   │       │       ╰── Constant Int [1234488]
                    │   │       ╰── <784> Var [uc]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [20]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <797>  [!=]
                    │   │       ├── <793> FunctionCall [uint_to_uchar]
                    │   │       │   ╰── Constant Long [4293732808]
                    │   │       ╰── <796> Var [uc]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [21]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <811>  [!=]
                    │   │       ├── <807> FunctionCall [long_to_uchar]
                    │   │       │   ╰── <806> Unary [-]
                    │   │       │       ╰── Constant Long [36283884951096]
                    │   │       ╰── <810> Var [uc]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [22]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <823>  [!=]
                    │   │       ├── <819> FunctionCall [ulong_to_uchar]
                    │   │       │   ╰── Constant ULong [9224497936761618632]
                    │   │       ╰── <822> Var [uc]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [23]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <835>  [!=]
                    │   │       ├── <831> FunctionCall [double_to_uchar]
                    │   │       │   ╰── Constant Double [+2.0099e2]
                    │   │       ╰── <834> Var [uc]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [24]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── null_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Long
                    │   ╰── Static
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── zero
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── <854> Cast
                    │           ├── Target
                    │           │   ╰── Char
                    │           ╰── Expression
                    │               ╰── <853> Var [null_ptr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <858> Var [zero]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [25]
                    ├── <868> Assign [=]
                    │   ├── <865> Var [c]
                    │   ╰── Constant Int [32]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <879> Cast
                    │           ├── Target
                    │           │   ╰── Pointer
                    │           │       ╰── Int
                    │           ╰── Expression
                    │               ╰── <878> Var [c]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <890>  [!=]
                    │   │       ├── <886> Cast
                    │   │       │   ├── Target
                    │   │       │   │   ╰── Char
                    │   │       │   ╰── Expression
                    │   │       │       ╰── <885> Var [i]
                    │   │       ╰── <889> Var [c]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [26]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <905>  [!=]
                    │   │       ├── <899> Cast
                    │   │       │   ├── Target
                    │   │       │   │   ╰── Char
                    │   │       │   ╰── Expression
                    │   │       │       ╰── Constant Int [300]
                    │   │       ╰── <904> Cast
                    │   │           ├── Target
                    │   │           │   ╰── Char
                    │   │           ╰── Expression
                    │   │               ╰── Constant Int [44]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [27]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_chars_integer_promotion() {
    let src = r#"
        int add_chars(char c1, char c2, char c3) {
            return c1 + c2 + c3;
        }
        int negate(unsigned char uc) {
            return -uc;
        }
        int complement(unsigned char uc) {
            return ~uc;
        }
        int add_then_div(signed char a, signed char b, signed char c) {
            return (a + b) / c;
        }
        int mixed_multiply(signed char s, unsigned char u) {
            return s * u;
        }
        signed char decrement(signed char s) {
            s = s - 1;
            return s;
        }
        int main(void) {
            char a = 100;
            char b = 109;
            if (add_chars(a, a, b) != 309) {
                return 1;
            }
            unsigned char one = 1;
            if (negate(one) != -1) {
                return 2;
            }
            if (complement(one) != -2) {
                return 3;
            }
            signed char w = 127;
            signed char x = 3;
            signed char y = 2;
            if (add_then_div(w, x, y) != 65)
                return 4;
            signed char sc = -3;
            unsigned char uc = 250;
            if (mixed_multiply(sc, uc) != -750)
                return 5;
            sc = -128;
            if (sc != -128) {
                return 6;
            }
            if (decrement(sc) != 127) {
                return 7;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [add_chars]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── c1
            │   │   │   ╰── Type
            │   │   │       ╰── Char
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── c2
            │   │   │   ╰── Type
            │   │   │       ╰── Char
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── c3
            │   │       ╰── Type
            │   │           ╰── Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <23>  [+]
            │               ├── <19>  [+]
            │               │   ├── <15> Var [c1]
            │               │   ╰── <18> Var [c2]
            │               ╰── <22> Var [c3]
            ├── Function [negate]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── uc
            │   │       ╰── Type
            │   │           ╰── Unsigned Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <37> Unary [-]
            │               ╰── <36> Var [uc]
            ├── Function [complement]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── uc
            │   │       ╰── Type
            │   │           ╰── Unsigned Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <51> Unary [~]
            │               ╰── <50> Var [uc]
            ├── Function [add_then_div]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── a
            │   │   │   ╰── Type
            │   │   │       ╰── Signed Char
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── b
            │   │   │   ╰── Type
            │   │   │       ╰── Signed Char
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── c
            │   │       ╰── Type
            │   │           ╰── Signed Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <78>  [/]
            │               ├── <74>  [+]
            │               │   ├── <69> Var [a]
            │               │   ╰── <72> Var [b]
            │               ╰── <77> Var [c]
            ├── Function [mixed_multiply]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── s
            │   │   │   ╰── Type
            │   │   │       ╰── Signed Char
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── u
            │   │       ╰── Type
            │   │           ╰── Unsigned Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <97>  [*]
            │               ├── <93> Var [s]
            │               ╰── <96> Var [u]
            ├── Function [decrement]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── s
            │   │       ╰── Type
            │   │           ╰── Signed Char
            │   ╰── Body
            │       ├── <116> Assign [=]
            │       │   ├── <109> Var [s]
            │       │   ╰── <115>  [-]
            │       │       ├── <112> Var [s]
            │       │       ╰── Constant Int [1]
            │       ╰── Return
            │           ╰── <119> Var [s]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── Constant Int [100]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── Constant Int [109]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <149>  [!=]
                    │   │       ├── <146> FunctionCall [add_chars]
                    │   │       │   ├── <141> Var [a]
                    │   │       │   ├── <143> Var [a]
                    │   │       │   ╰── <145> Var [b]
                    │   │       ╰── Constant Int [309]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── one
                    │   ├── Type
                    │   │   ╰── Unsigned Char
                    │   ╰── Initializer
                    │       ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <169>  [!=]
                    │   │       ├── <164> FunctionCall [negate]
                    │   │       │   ╰── <163> Var [one]
                    │   │       ╰── <168> Unary [-]
                    │   │           ╰── Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <183>  [!=]
                    │   │       ├── <178> FunctionCall [complement]
                    │   │       │   ╰── <177> Var [one]
                    │   │       ╰── <182> Unary [-]
                    │   │           ╰── Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── w
                    │   ├── Type
                    │   │   ╰── Signed Char
                    │   ╰── Initializer
                    │       ╰── Constant Int [127]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Signed Char
                    │   ╰── Initializer
                    │       ╰── Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── y
                    │   ├── Type
                    │   │   ╰── Signed Char
                    │   ╰── Initializer
                    │       ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <217>  [!=]
                    │   │       ├── <214> FunctionCall [add_then_div]
                    │   │       │   ├── <209> Var [w]
                    │   │       │   ├── <211> Var [x]
                    │   │       │   ╰── <213> Var [y]
                    │   │       ╰── Constant Int [65]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [4]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── sc
                    │   ├── Type
                    │   │   ╰── Signed Char
                    │   ╰── Initializer
                    │       ╰── <226> Unary [-]
                    │           ╰── Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── uc
                    │   ├── Type
                    │   │   ╰── Unsigned Char
                    │   ╰── Initializer
                    │       ╰── Constant Int [250]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <245>  [!=]
                    │   │       ├── <240> FunctionCall [mixed_multiply]
                    │   │       │   ├── <237> Var [sc]
                    │   │       │   ╰── <239> Var [uc]
                    │   │       ╰── <244> Unary [-]
                    │   │           ╰── Constant Int [750]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [5]
                    ├── <255> Assign [=]
                    │   ├── <250> Var [sc]
                    │   ╰── <254> Unary [-]
                    │       ╰── Constant Int [128]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <263>  [!=]
                    │   │       ├── <258> Var [sc]
                    │   │       ╰── <262> Unary [-]
                    │   │           ╰── Constant Int [128]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <275>  [!=]
                    │   │       ├── <272> FunctionCall [decrement]
                    │   │       │   ╰── <271> Var [sc]
                    │   │       ╰── Constant Int [127]
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
fn test_valid_chars_partial_initialization() {
    let src = r#"
        char static1[4] = {1, 2};
        signed char static2[4] = {3, 4};
        unsigned char static3[3] = {5};
        int main(void)
        {
            if (static1[0] != 1 || static1[1] != 2 || static1[2] || static1[3])
                return 1;
            if (static2[0] != 3 || static2[1] != 4 || static2[2] || static2[3])
                return 2;
            if (static3[0] != 5 || static3[1] || static3[2])
                return 3;
            char auto1[5] = {-4, 66, 4.0};
            signed char auto2[3] = {static1[2], -static1[0]};
            unsigned char auto3[2] = {'a'};
            if (auto1[0] != -4 || auto1[1] != 66 || auto1[2] != 4 || auto1[3] || auto1[4])
                return 4;
            if (auto2[0] || auto2[1] != -1 || auto2[2])
                return 5;
            if (auto3[0] != 'a' || auto3[1])
                return 6;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── static1
            │   ├── Type
            │   │   ╰── Array
            │   │       ├── 4
            │   │       ╰── Char
            │   ╰── Initializer
            │       ╰── Compound
            │           ├── Constant Int [1]
            │           ╰── Constant Int [2]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── static2
            │   ├── Type
            │   │   ╰── Array
            │   │       ├── 4
            │   │       ╰── Signed Char
            │   ╰── Initializer
            │       ╰── Compound
            │           ├── Constant Int [3]
            │           ╰── Constant Int [4]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── static3
            │   ├── Type
            │   │   ╰── Array
            │   │       ├── 3
            │   │       ╰── Unsigned Char
            │   ╰── Initializer
            │       ╰── Compound
            │           ╰── Constant Int [5]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <63>  [||]
                    │   │       ├── <57>  [||]
                    │   │       │   ├── <51>  [||]
                    │   │       │   │   ├── <42>  [!=]
                    │   │       │   │   │   ├── <39> Subscript
                    │   │       │   │   │   │   ├── <37> Var [static1]
                    │   │       │   │   │   │   ╰── Constant Int [0]
                    │   │       │   │   │   ╰── Constant Int [1]
                    │   │       │   │   ╰── <50>  [!=]
                    │   │       │   │       ├── <47> Subscript
                    │   │       │   │       │   ├── <45> Var [static1]
                    │   │       │   │       │   ╰── Constant Int [1]
                    │   │       │   │       ╰── Constant Int [2]
                    │   │       │   ╰── <56> Subscript
                    │   │       │       ├── <54> Var [static1]
                    │   │       │       ╰── Constant Int [2]
                    │   │       ╰── <62> Subscript
                    │   │           ├── <60> Var [static1]
                    │   │           ╰── Constant Int [3]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <94>  [||]
                    │   │       ├── <88>  [||]
                    │   │       │   ├── <82>  [||]
                    │   │       │   │   ├── <73>  [!=]
                    │   │       │   │   │   ├── <70> Subscript
                    │   │       │   │   │   │   ├── <68> Var [static2]
                    │   │       │   │   │   │   ╰── Constant Int [0]
                    │   │       │   │   │   ╰── Constant Int [3]
                    │   │       │   │   ╰── <81>  [!=]
                    │   │       │   │       ├── <78> Subscript
                    │   │       │   │       │   ├── <76> Var [static2]
                    │   │       │   │       │   ╰── Constant Int [1]
                    │   │       │   │       ╰── Constant Int [4]
                    │   │       │   ╰── <87> Subscript
                    │   │       │       ├── <85> Var [static2]
                    │   │       │       ╰── Constant Int [2]
                    │   │       ╰── <93> Subscript
                    │   │           ├── <91> Var [static2]
                    │   │           ╰── Constant Int [3]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <116>  [||]
                    │   │       ├── <110>  [||]
                    │   │       │   ├── <104>  [!=]
                    │   │       │   │   ├── <101> Subscript
                    │   │       │   │   │   ├── <99> Var [static3]
                    │   │       │   │   │   ╰── Constant Int [0]
                    │   │       │   │   ╰── Constant Int [5]
                    │   │       │   ╰── <109> Subscript
                    │   │       │       ├── <107> Var [static3]
                    │   │       │       ╰── Constant Int [1]
                    │   │       ╰── <115> Subscript
                    │   │           ├── <113> Var [static3]
                    │   │           ╰── Constant Int [2]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── auto1
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 5
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <127> Unary [-]
                    │           │   ╰── Constant Int [4]
                    │           ├── Constant Int [66]
                    │           ╰── Constant Double [+4e0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── auto2
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 3
                    │   │       ╰── Signed Char
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <143> Subscript
                    │           │   ├── <141> Var [static1]
                    │           │   ╰── Constant Int [2]
                    │           ╰── <150> Unary [-]
                    │               ╰── <149> Subscript
                    │                   ├── <147> Var [static1]
                    │                   ╰── Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── auto3
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 2
                    │   │       ╰── Unsigned Char
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ╰── Constant Int [97]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <201>  [||]
                    │   │       ├── <195>  [||]
                    │   │       │   ├── <189>  [||]
                    │   │       │   │   ├── <180>  [||]
                    │   │       │   │   │   ├── <171>  [!=]
                    │   │       │   │   │   │   ├── <166> Subscript
                    │   │       │   │   │   │   │   ├── <164> Var [auto1]
                    │   │       │   │   │   │   │   ╰── Constant Int [0]
                    │   │       │   │   │   │   ╰── <170> Unary [-]
                    │   │       │   │   │   │       ╰── Constant Int [4]
                    │   │       │   │   │   ╰── <179>  [!=]
                    │   │       │   │   │       ├── <176> Subscript
                    │   │       │   │   │       │   ├── <174> Var [auto1]
                    │   │       │   │   │       │   ╰── Constant Int [1]
                    │   │       │   │   │       ╰── Constant Int [66]
                    │   │       │   │   ╰── <188>  [!=]
                    │   │       │   │       ├── <185> Subscript
                    │   │       │   │       │   ├── <183> Var [auto1]
                    │   │       │   │       │   ╰── Constant Int [2]
                    │   │       │   │       ╰── Constant Int [4]
                    │   │       │   ╰── <194> Subscript
                    │   │       │       ├── <192> Var [auto1]
                    │   │       │       ╰── Constant Int [3]
                    │   │       ╰── <200> Subscript
                    │   │           ├── <198> Var [auto1]
                    │   │           ╰── Constant Int [4]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <225>  [||]
                    │   │       ├── <219>  [||]
                    │   │       │   ├── <208> Subscript
                    │   │       │   │   ├── <206> Var [auto2]
                    │   │       │   │   ╰── Constant Int [0]
                    │   │       │   ╰── <218>  [!=]
                    │   │       │       ├── <213> Subscript
                    │   │       │       │   ├── <211> Var [auto2]
                    │   │       │       │   ╰── Constant Int [1]
                    │   │       │       ╰── <217> Unary [-]
                    │   │       │           ╰── Constant Int [1]
                    │   │       ╰── <224> Subscript
                    │   │           ├── <222> Var [auto2]
                    │   │           ╰── Constant Int [2]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <241>  [||]
                    │   │       ├── <235>  [!=]
                    │   │       │   ├── <232> Subscript
                    │   │       │   │   ├── <230> Var [auto3]
                    │   │       │   │   ╰── Constant Int [0]
                    │   │       │   ╰── Constant Int [97]
                    │   │       ╰── <240> Subscript
                    │   │           ├── <238> Var [auto3]
                    │   │           ╰── Constant Int [1]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [6]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_chars_push_arg_on_page_boundary() {
    let src = r#"
        extern char zed;
        int foo(int a, int b, int c, int d, int e, int f, char g) {
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
            │   │   ╰── Char
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
            │   │           ╰── Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <35>  [+]
            │               ├── <32> Var [g]
            │               ╰── Constant Int [1]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <52> FunctionCall [foo]
                            ├── Constant Int [0]
                            ├── Constant Int [0]
                            ├── Constant Int [0]
                            ├── Constant Int [0]
                            ├── Constant Int [0]
                            ├── Constant Int [0]
                            ╰── <51> Var [zed]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_chars_return_char() {
    let src = r#"
        char return_char(void) {
            return 5369233654l;
        }
        signed char return_schar(void) {
            return 5369233654l;
        }
        unsigned char return_uchar(void) {
            return 5369233654l;
        }
        int main(void) {
            char char_array[3] = {121, -122, -3};
            char retval_c = return_char();
            char char_array2[3] = {-5, 88, -100};
            signed char retval_sc = return_schar();
            char char_array3[3] = {10, 11, 12};
            unsigned char retval_uc = return_uchar();
            char char_array4[2] = {-5, -6};
            if (char_array[0] != 121 || char_array[1] != -122 || char_array[2] != -3) {
                return 1;
            }
            if (retval_c != -10) {
                return 2;
            }
            if (char_array2[0] != -5 || char_array2[1] != 88 ||
                char_array2[2] != -100) {
                return 3;
            }
            if (retval_sc != -10) {
                return 4;
            }
            if (char_array3[0] != 10 || char_array3[1] != 11 || char_array3[2] != 12) {
                return 5;
            }
            if (retval_uc != 246) {
                return 6;
            }
            if (char_array4[0] != -5 || char_array4[1] != -6) {
                return 7;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [return_char]
            │   ╰── Body
            │       ╰── Return
            │           ╰── Constant Long [5369233654]
            ├── Function [return_schar]
            │   ╰── Body
            │       ╰── Return
            │           ╰── Constant Long [5369233654]
            ├── Function [return_uchar]
            │   ╰── Body
            │       ╰── Return
            │           ╰── Constant Long [5369233654]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── char_array
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 3
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── Constant Int [121]
                    │           ├── <38> Unary [-]
                    │           │   ╰── Constant Int [122]
                    │           ╰── <42> Unary [-]
                    │               ╰── Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── retval_c
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── <50> FunctionCall [return_char]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── char_array2
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 3
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <60> Unary [-]
                    │           │   ╰── Constant Int [5]
                    │           ├── Constant Int [88]
                    │           ╰── <66> Unary [-]
                    │               ╰── Constant Int [100]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── retval_sc
                    │   ├── Type
                    │   │   ╰── Signed Char
                    │   ╰── Initializer
                    │       ╰── <74> FunctionCall [return_schar]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── char_array3
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 3
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── Constant Int [10]
                    │           ├── Constant Int [11]
                    │           ╰── Constant Int [12]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── retval_uc
                    │   ├── Type
                    │   │   ╰── Unsigned Char
                    │   ╰── Initializer
                    │       ╰── <94> FunctionCall [return_uchar]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── char_array4
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 2
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <104> Unary [-]
                    │           │   ╰── Constant Int [5]
                    │           ╰── <108> Unary [-]
                    │               ╰── Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <140>  [||]
                    │   │       ├── <129>  [||]
                    │   │       │   ├── <118>  [!=]
                    │   │       │   │   ├── <115> Subscript
                    │   │       │   │   │   ├── <113> Var [char_array]
                    │   │       │   │   │   ╰── Constant Int [0]
                    │   │       │   │   ╰── Constant Int [121]
                    │   │       │   ╰── <128>  [!=]
                    │   │       │       ├── <123> Subscript
                    │   │       │       │   ├── <121> Var [char_array]
                    │   │       │       │   ╰── Constant Int [1]
                    │   │       │       ╰── <127> Unary [-]
                    │   │       │           ╰── Constant Int [122]
                    │   │       ╰── <139>  [!=]
                    │   │           ├── <134> Subscript
                    │   │           │   ├── <132> Var [char_array]
                    │   │           │   ╰── Constant Int [2]
                    │   │           ╰── <138> Unary [-]
                    │   │               ╰── Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <152>  [!=]
                    │   │       ├── <147> Var [retval_c]
                    │   │       ╰── <151> Unary [-]
                    │   │           ╰── Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <186>  [||]
                    │   │       ├── <175>  [||]
                    │   │       │   ├── <166>  [!=]
                    │   │       │   │   ├── <161> Subscript
                    │   │       │   │   │   ├── <159> Var [char_array2]
                    │   │       │   │   │   ╰── Constant Int [0]
                    │   │       │   │   ╰── <165> Unary [-]
                    │   │       │   │       ╰── Constant Int [5]
                    │   │       │   ╰── <174>  [!=]
                    │   │       │       ├── <171> Subscript
                    │   │       │       │   ├── <169> Var [char_array2]
                    │   │       │       │   ╰── Constant Int [1]
                    │   │       │       ╰── Constant Int [88]
                    │   │       ╰── <185>  [!=]
                    │   │           ├── <180> Subscript
                    │   │           │   ├── <178> Var [char_array2]
                    │   │           │   ╰── Constant Int [2]
                    │   │           ╰── <184> Unary [-]
                    │   │               ╰── Constant Int [100]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <198>  [!=]
                    │   │       ├── <193> Var [retval_sc]
                    │   │       ╰── <197> Unary [-]
                    │   │           ╰── Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <228>  [||]
                    │   │       ├── <219>  [||]
                    │   │       │   ├── <210>  [!=]
                    │   │       │   │   ├── <207> Subscript
                    │   │       │   │   │   ├── <205> Var [char_array3]
                    │   │       │   │   │   ╰── Constant Int [0]
                    │   │       │   │   ╰── Constant Int [10]
                    │   │       │   ╰── <218>  [!=]
                    │   │       │       ├── <215> Subscript
                    │   │       │       │   ├── <213> Var [char_array3]
                    │   │       │       │   ╰── Constant Int [1]
                    │   │       │       ╰── Constant Int [11]
                    │   │       ╰── <227>  [!=]
                    │   │           ├── <224> Subscript
                    │   │           │   ├── <222> Var [char_array3]
                    │   │           │   ╰── Constant Int [2]
                    │   │           ╰── Constant Int [12]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <238>  [!=]
                    │   │       ├── <235> Var [retval_uc]
                    │   │       ╰── Constant Int [246]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <263>  [||]
                    │   │       ├── <252>  [!=]
                    │   │       │   ├── <247> Subscript
                    │   │       │   │   ├── <245> Var [char_array4]
                    │   │       │   │   ╰── Constant Int [0]
                    │   │       │   ╰── <251> Unary [-]
                    │   │       │       ╰── Constant Int [5]
                    │   │       ╰── <262>  [!=]
                    │   │           ├── <257> Subscript
                    │   │           │   ├── <255> Var [char_array4]
                    │   │           │   ╰── Constant Int [1]
                    │   │           ╰── <261> Unary [-]
                    │   │               ╰── Constant Int [6]
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
fn test_valid_chars_rewrite_movz_regression() {
    let src = r#"
        int check_12_ints(int start, int a, int b, int c, int d, int e, int f, int g,
                          int h, int i, int j, int k, int l);
        unsigned char glob = 5;
        int main(void) {
            int should_spill = (int)glob;
            int one = glob - 4;
            int two = one + one;
            int three = 2 + one;
            int four = two * two;
            int five = 6 - one;
            int six = two * three;
            int seven = one + 6;
            int eight = two * 4;
            int nine = three * three;
            int ten = four + six;
            int eleven = 16 - five;
            int twelve = six + six;
            check_12_ints(one, two, three, four, five, six, seven, eight, nine, ten,
                          eleven, twelve, 1);
            int thirteen = 8 + glob;
            int fourteen = thirteen + 1;
            int fifteen = 28 - thirteen;
            int sixteen = fourteen + 2;
            int seventeen = 4 + thirteen;
            int eighteen = 32 - fourteen;
            int nineteen = 35 - sixteen;
            int twenty = fifteen + 5;
            int twenty_one = thirteen * 2 - 5;
            int twenty_two = fifteen + 7;
            int twenty_three = 6 + seventeen;
            int twenty_four = thirteen + 11;
            check_12_ints(thirteen, fourteen, fifteen, sixteen, seventeen, eighteen,
                          nineteen, twenty, twenty_one, twenty_two, twenty_three,
                          twenty_four, 13);
            if (should_spill != 5) {
                return -1;
            }
            return 0;
        }
        int check_12_ints(int a, int b, int c, int d, int e, int f, int g, int h, int i,
                          int j, int k, int l, int start) {
            int expected = 0;
            expected = start + 0;
            if (a != expected) {
                return expected;
            }
            expected = start + 1;
            if (b != expected) {
                return expected;
            }
            expected = start + 2;
            if (c != expected) {
                return expected;
            }
            expected = start + 3;
            if (d != expected) {
                return expected;
            }
            expected = start + 4;
            if (e != expected) {
                return expected;
            }
            expected = start + 5;
            if (f != expected) {
                return expected;
            }
            expected = start + 6;
            if (g != expected) {
                return expected;
            }
            expected = start + 7;
            if (h != expected) {
                return expected;
            }
            expected = start + 8;
            if (i != expected) {
                return expected;
            }
            expected = start + 9;
            if (j != expected) {
                return expected;
            }
            expected = start + 10;
            if (k != expected) {
                return expected;
            }
            expected = start + 11;
            if (l != expected) {
                return expected;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [check_12_ints]
            │   ╰── Parameters
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── start
            │       │   ╰── Type
            │       │       ╰── Int
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
            │       ╰── Param
            │           ├── Name
            │           │   ╰── l
            │           ╰── Type
            │               ╰── Int
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── glob
            │   ├── Type
            │   │   ╰── Unsigned Char
            │   ╰── Initializer
            │       ╰── Constant Int [5]
            ├── Function [main]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── should_spill
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <62> Cast
            │       │           ├── Target
            │       │           │   ╰── Int
            │       │           ╰── Expression
            │       │               ╰── <61> Var [glob]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── one
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <72>  [-]
            │       │           ├── <69> Var [glob]
            │       │           ╰── Constant Int [4]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── two
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <83>  [+]
            │       │           ├── <79> Var [one]
            │       │           ╰── <82> Var [one]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── three
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <93>  [+]
            │       │           ├── Constant Int [2]
            │       │           ╰── <92> Var [one]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── four
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <104>  [*]
            │       │           ├── <100> Var [two]
            │       │           ╰── <103> Var [two]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── five
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <114>  [-]
            │       │           ├── Constant Int [6]
            │       │           ╰── <113> Var [one]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── six
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <125>  [*]
            │       │           ├── <121> Var [two]
            │       │           ╰── <124> Var [three]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── seven
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <135>  [+]
            │       │           ├── <132> Var [one]
            │       │           ╰── Constant Int [6]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── eight
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <145>  [*]
            │       │           ├── <142> Var [two]
            │       │           ╰── Constant Int [4]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── nine
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <156>  [*]
            │       │           ├── <152> Var [three]
            │       │           ╰── <155> Var [three]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ten
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <167>  [+]
            │       │           ├── <163> Var [four]
            │       │           ╰── <166> Var [six]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── eleven
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <177>  [-]
            │       │           ├── Constant Int [16]
            │       │           ╰── <176> Var [five]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── twelve
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <188>  [+]
            │       │           ├── <184> Var [six]
            │       │           ╰── <187> Var [six]
            │       ├── <217> FunctionCall [check_12_ints]
            │       │   ├── <193> Var [one]
            │       │   ├── <195> Var [two]
            │       │   ├── <197> Var [three]
            │       │   ├── <199> Var [four]
            │       │   ├── <201> Var [five]
            │       │   ├── <203> Var [six]
            │       │   ├── <205> Var [seven]
            │       │   ├── <207> Var [eight]
            │       │   ├── <209> Var [nine]
            │       │   ├── <211> Var [ten]
            │       │   ├── <213> Var [eleven]
            │       │   ├── <215> Var [twelve]
            │       │   ╰── Constant Int [1]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── thirteen
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <226>  [+]
            │       │           ├── Constant Int [8]
            │       │           ╰── <225> Var [glob]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── fourteen
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <236>  [+]
            │       │           ├── <233> Var [thirteen]
            │       │           ╰── Constant Int [1]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── fifteen
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <246>  [-]
            │       │           ├── Constant Int [28]
            │       │           ╰── <245> Var [thirteen]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── sixteen
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <256>  [+]
            │       │           ├── <253> Var [fourteen]
            │       │           ╰── Constant Int [2]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── seventeen
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <266>  [+]
            │       │           ├── Constant Int [4]
            │       │           ╰── <265> Var [thirteen]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── eighteen
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <276>  [-]
            │       │           ├── Constant Int [32]
            │       │           ╰── <275> Var [fourteen]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── nineteen
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <286>  [-]
            │       │           ├── Constant Int [35]
            │       │           ╰── <285> Var [sixteen]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── twenty
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <296>  [+]
            │       │           ├── <293> Var [fifteen]
            │       │           ╰── Constant Int [5]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── twenty_one
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <309>  [-]
            │       │           ├── <306>  [*]
            │       │           │   ├── <303> Var [thirteen]
            │       │           │   ╰── Constant Int [2]
            │       │           ╰── Constant Int [5]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── twenty_two
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <319>  [+]
            │       │           ├── <316> Var [fifteen]
            │       │           ╰── Constant Int [7]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── twenty_three
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <329>  [+]
            │       │           ├── Constant Int [6]
            │       │           ╰── <328> Var [seventeen]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── twenty_four
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <339>  [+]
            │       │           ├── <336> Var [thirteen]
            │       │           ╰── Constant Int [11]
            │       ├── <368> FunctionCall [check_12_ints]
            │       │   ├── <344> Var [thirteen]
            │       │   ├── <346> Var [fourteen]
            │       │   ├── <348> Var [fifteen]
            │       │   ├── <350> Var [sixteen]
            │       │   ├── <352> Var [seventeen]
            │       │   ├── <354> Var [eighteen]
            │       │   ├── <356> Var [nineteen]
            │       │   ├── <358> Var [twenty]
            │       │   ├── <360> Var [twenty_one]
            │       │   ├── <362> Var [twenty_two]
            │       │   ├── <364> Var [twenty_three]
            │       │   ├── <366> Var [twenty_four]
            │       │   ╰── Constant Int [13]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <374>  [!=]
            │       │   │       ├── <371> Var [should_spill]
            │       │   │       ╰── Constant Int [5]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <377> Unary [-]
            │       │                   ╰── Constant Int [1]
            │       ╰── Return
            │           ╰── Constant Int [0]
            ╰── Function [check_12_ints]
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
                │   ╰── Param
                │       ├── Name
                │       │   ╰── start
                │       ╰── Type
                │           ╰── Int
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── expected
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [0]
                    ├── <443> Assign [=]
                    │   ├── <436> Var [expected]
                    │   ╰── <442>  [+]
                    │       ├── <439> Var [start]
                    │       ╰── Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <450>  [!=]
                    │   │       ├── <446> Var [a]
                    │   │       ╰── <449> Var [expected]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <452> Var [expected]
                    ├── <465> Assign [=]
                    │   ├── <458> Var [expected]
                    │   ╰── <464>  [+]
                    │       ├── <461> Var [start]
                    │       ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <472>  [!=]
                    │   │       ├── <468> Var [b]
                    │   │       ╰── <471> Var [expected]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <474> Var [expected]
                    ├── <487> Assign [=]
                    │   ├── <480> Var [expected]
                    │   ╰── <486>  [+]
                    │       ├── <483> Var [start]
                    │       ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <494>  [!=]
                    │   │       ├── <490> Var [c]
                    │   │       ╰── <493> Var [expected]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <496> Var [expected]
                    ├── <509> Assign [=]
                    │   ├── <502> Var [expected]
                    │   ╰── <508>  [+]
                    │       ├── <505> Var [start]
                    │       ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <516>  [!=]
                    │   │       ├── <512> Var [d]
                    │   │       ╰── <515> Var [expected]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <518> Var [expected]
                    ├── <531> Assign [=]
                    │   ├── <524> Var [expected]
                    │   ╰── <530>  [+]
                    │       ├── <527> Var [start]
                    │       ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <538>  [!=]
                    │   │       ├── <534> Var [e]
                    │   │       ╰── <537> Var [expected]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <540> Var [expected]
                    ├── <553> Assign [=]
                    │   ├── <546> Var [expected]
                    │   ╰── <552>  [+]
                    │       ├── <549> Var [start]
                    │       ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <560>  [!=]
                    │   │       ├── <556> Var [f]
                    │   │       ╰── <559> Var [expected]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <562> Var [expected]
                    ├── <575> Assign [=]
                    │   ├── <568> Var [expected]
                    │   ╰── <574>  [+]
                    │       ├── <571> Var [start]
                    │       ╰── Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <582>  [!=]
                    │   │       ├── <578> Var [g]
                    │   │       ╰── <581> Var [expected]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <584> Var [expected]
                    ├── <597> Assign [=]
                    │   ├── <590> Var [expected]
                    │   ╰── <596>  [+]
                    │       ├── <593> Var [start]
                    │       ╰── Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <604>  [!=]
                    │   │       ├── <600> Var [h]
                    │   │       ╰── <603> Var [expected]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <606> Var [expected]
                    ├── <619> Assign [=]
                    │   ├── <612> Var [expected]
                    │   ╰── <618>  [+]
                    │       ├── <615> Var [start]
                    │       ╰── Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <626>  [!=]
                    │   │       ├── <622> Var [i]
                    │   │       ╰── <625> Var [expected]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <628> Var [expected]
                    ├── <641> Assign [=]
                    │   ├── <634> Var [expected]
                    │   ╰── <640>  [+]
                    │       ├── <637> Var [start]
                    │       ╰── Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <648>  [!=]
                    │   │       ├── <644> Var [j]
                    │   │       ╰── <647> Var [expected]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <650> Var [expected]
                    ├── <663> Assign [=]
                    │   ├── <656> Var [expected]
                    │   ╰── <662>  [+]
                    │       ├── <659> Var [start]
                    │       ╰── Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <670>  [!=]
                    │   │       ├── <666> Var [k]
                    │   │       ╰── <669> Var [expected]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <672> Var [expected]
                    ├── <685> Assign [=]
                    │   ├── <678> Var [expected]
                    │   ╰── <684>  [+]
                    │       ├── <681> Var [start]
                    │       ╰── Constant Int [11]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <692>  [!=]
                    │   │       ├── <688> Var [l]
                    │   │       ╰── <691> Var [expected]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <694> Var [expected]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_chars_static_initializers() {
    let src = r#"
        char from_long = 17592186044416l;
        char from_double = 15.6;
        char from_uint = 2147483777u;
        char from_ulong = 9223372037928517642ul;
        signed char schar_from_long = 17592186044419l;
        signed char schar_from_uint = 2147483898u;
        signed char schar_from_ulong = 9223372037928517642ul;
        signed char schar_from_double = 1e-10;
        unsigned char uchar_from_int = 13526;
        unsigned char uchar_from_uint = 2147483898u;
        unsigned char uchar_from_long = 1101659111674l;
        unsigned char uchar_from_ulong = 9223372037928517642ul;
        unsigned char uchar_from_double = 77.7;
        int main(void) {
            if (from_long != 0) {
                return 1;
            }
            if (from_double != 15) {
                return 2;
            }
            if (from_uint != -127) {
                return 3;
            }
            if (from_ulong != 10) {
                return 4;
            }
            if (schar_from_uint != -6) {
                return 5;
            }
            if (schar_from_ulong != 10) {
                return 6;
            }
            if (schar_from_double != 0) {
                return 7;
            }
            if (uchar_from_int != 214) {
                return 8;
            }
            if (uchar_from_uint != 250) {
                return 9;
            }
            if (uchar_from_ulong != 10) {
                return 10;
            }
            if (uchar_from_double != 77) {
                return 11;
            }
            if (schar_from_long != 3) {
                return 12;
            }
            if (uchar_from_long != 250) {
                return 13;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── from_long
            │   ├── Type
            │   │   ╰── Char
            │   ╰── Initializer
            │       ╰── Constant Long [17592186044416]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── from_double
            │   ├── Type
            │   │   ╰── Char
            │   ╰── Initializer
            │       ╰── Constant Double [+1.56e1]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── from_uint
            │   ├── Type
            │   │   ╰── Char
            │   ╰── Initializer
            │       ╰── Constant UInt [2147483777]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── from_ulong
            │   ├── Type
            │   │   ╰── Char
            │   ╰── Initializer
            │       ╰── Constant ULong [9223372037928517642]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── schar_from_long
            │   ├── Type
            │   │   ╰── Signed Char
            │   ╰── Initializer
            │       ╰── Constant Long [17592186044419]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── schar_from_uint
            │   ├── Type
            │   │   ╰── Signed Char
            │   ╰── Initializer
            │       ╰── Constant UInt [2147483898]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── schar_from_ulong
            │   ├── Type
            │   │   ╰── Signed Char
            │   ╰── Initializer
            │       ╰── Constant ULong [9223372037928517642]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── schar_from_double
            │   ├── Type
            │   │   ╰── Signed Char
            │   ╰── Initializer
            │       ╰── Constant Double [+1e-10]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── uchar_from_int
            │   ├── Type
            │   │   ╰── Unsigned Char
            │   ╰── Initializer
            │       ╰── Constant Int [13526]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── uchar_from_uint
            │   ├── Type
            │   │   ╰── Unsigned Char
            │   ╰── Initializer
            │       ╰── Constant UInt [2147483898]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── uchar_from_long
            │   ├── Type
            │   │   ╰── Unsigned Char
            │   ╰── Initializer
            │       ╰── Constant Long [1101659111674]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── uchar_from_ulong
            │   ├── Type
            │   │   ╰── Unsigned Char
            │   ╰── Initializer
            │       ╰── Constant ULong [9223372037928517642]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── uchar_from_double
            │   ├── Type
            │   │   ╰── Unsigned Char
            │   ╰── Initializer
            │       ╰── Constant Double [+7.77e1]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <87>  [!=]
                    │   │       ├── <84> Var [from_long]
                    │   │       ╰── Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <97>  [!=]
                    │   │       ├── <94> Var [from_double]
                    │   │       ╰── Constant Int [15]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <109>  [!=]
                    │   │       ├── <104> Var [from_uint]
                    │   │       ╰── <108> Unary [-]
                    │   │           ╰── Constant Int [127]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <119>  [!=]
                    │   │       ├── <116> Var [from_ulong]
                    │   │       ╰── Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <131>  [!=]
                    │   │       ├── <126> Var [schar_from_uint]
                    │   │       ╰── <130> Unary [-]
                    │   │           ╰── Constant Int [6]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <141>  [!=]
                    │   │       ├── <138> Var [schar_from_ulong]
                    │   │       ╰── Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <151>  [!=]
                    │   │       ├── <148> Var [schar_from_double]
                    │   │       ╰── Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <161>  [!=]
                    │   │       ├── <158> Var [uchar_from_int]
                    │   │       ╰── Constant Int [214]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <171>  [!=]
                    │   │       ├── <168> Var [uchar_from_uint]
                    │   │       ╰── Constant Int [250]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <181>  [!=]
                    │   │       ├── <178> Var [uchar_from_ulong]
                    │   │       ╰── Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <191>  [!=]
                    │   │       ├── <188> Var [uchar_from_double]
                    │   │       ╰── Constant Int [77]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [11]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <201>  [!=]
                    │   │       ├── <198> Var [schar_from_long]
                    │   │       ╰── Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [12]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <211>  [!=]
                    │   │       ├── <208> Var [uchar_from_long]
                    │   │       ╰── Constant Int [250]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [13]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_chars_type_specifiers() {
    let src = r#"
        char signed static a = 10;
        unsigned static char b = 20;
        char c = 30;
        int main(void)
        {
            extern signed char a;
            char unsigned extern b;
            extern char c;
            if (a != 10) {
                return 1;
            }
            if (b != 20) {
                return 2;
            }
            if (c != 30) {
                return 3;
            }
            int loop_counter = 0;
            for (unsigned char d = 0; d < 100; d = d + 1) {
                loop_counter = loop_counter + 1;
            }
            if (loop_counter != 100) {
                return 4;
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
            │   │   ╰── Signed Char
            │   ├── Initializer
            │   │   ╰── Constant Int [10]
            │   ╰── Static
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── b
            │   ├── Type
            │   │   ╰── Unsigned Char
            │   ├── Initializer
            │   │   ╰── Constant Int [20]
            │   ╰── Static
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── c
            │   ├── Type
            │   │   ╰── Char
            │   ╰── Initializer
            │       ╰── Constant Int [30]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Signed Char
                    │   ╰── Extern
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Unsigned Char
                    │   ╰── Extern
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Extern
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <44>  [!=]
                    │   │       ├── <41> Var [a]
                    │   │       ╰── Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <54>  [!=]
                    │   │       ├── <51> Var [b]
                    │   │       ╰── Constant Int [20]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <64>  [!=]
                    │   │       ├── <61> Var [c]
                    │   │       ╰── Constant Int [30]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── loop_counter
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [0]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── d
                    │   │       ├── Type
                    │   │       │   ╰── Unsigned Char
                    │   │       ╰── Initializer
                    │   │           ╰── Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <87>  [<]
                    │   │       ├── <84> Var [d]
                    │   │       ╰── Constant Int [100]
                    │   ├── Condition
                    │   │   ╰── <96> Assign [=]
                    │   │       ├── <89> Var [d]
                    │   │       ╰── <95>  [+]
                    │   │           ├── <92> Var [d]
                    │   │           ╰── Constant Int [1]
                    │   ╰── Block
                    │       ╰── <105> Assign [=]
                    │           ├── <98> Var [loop_counter]
                    │           ╰── <104>  [+]
                    │               ├── <101> Var [loop_counter]
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <114>  [!=]
                    │   │       ├── <111> Var [loop_counter]
                    │   │       ╰── Constant Int [100]
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
fn test_valid_extra_credit_bitshift_chars() {
    let src = r#"
        int main(void) {
            unsigned char uc = 255;
            if ((uc >> 3) != 31) {
                return 2;
            }
            signed char sc = -127;
            char c = 5;
            if ((sc >> c) != -4) {
                return 3;
            }
            if (((-(c << 3ul)) >> 3) != -5) {
                return 4;
            }
            if ((-(uc << 5u) >> 5u) != -255l) {
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
                    │   │   ╰── uc
                    │   ├── Type
                    │   │   ╰── Unsigned Char
                    │   ╰── Initializer
                    │       ╰── Constant Int [255]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <19>  [!=]
                    │   │       ├── <16>  [>>]
                    │   │       │   ├── <12> Var [uc]
                    │   │       │   ╰── Constant Int [3]
                    │   │       ╰── Constant Int [31]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── sc
                    │   ├── Type
                    │   │   ╰── Signed Char
                    │   ╰── Initializer
                    │       ╰── <30> Unary [-]
                    │           ╰── Constant Int [127]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <50>  [!=]
                    │   │       ├── <45>  [>>]
                    │   │       │   ├── <40> Var [sc]
                    │   │       │   ╰── <43> Var [c]
                    │   │       ╰── <49> Unary [-]
                    │   │           ╰── Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <73>  [!=]
                    │   │       ├── <68>  [>>]
                    │   │       │   ├── <64> Unary [-]
                    │   │       │   │   ╰── <62>  [<<]
                    │   │       │   │       ├── <58> Var [c]
                    │   │       │   │       ╰── Constant ULong [3]
                    │   │       │   ╰── Constant Int [3]
                    │   │       ╰── <72> Unary [-]
                    │   │           ╰── Constant Int [5]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <95>  [!=]
                    │   │       ├── <90>  [>>]
                    │   │       │   ├── <86> Unary [-]
                    │   │       │   │   ╰── <85>  [<<]
                    │   │       │   │       ├── <81> Var [uc]
                    │   │       │   │       ╰── Constant UInt [5]
                    │   │       │   ╰── Constant UInt [5]
                    │   │       ╰── <94> Unary [-]
                    │   │           ╰── Constant Long [255]
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
fn test_valid_extra_credit_bitwise_ops_character_constants() {
    let src = r#"
        int main(void) {
            int x = 10;
            if ((x ^ 'A') != 75) {
                return 1;
            }
            static char c = 132;
            if (('!' | c) != -91) {
                return 2;
            }
            static unsigned long ul = 9259400834947493926ul;
            if ((ul & '~') != 38) {
                return 3;
            }
            if ((ul << ' ') != 4611738958194278400ul) {
                return 4;
            }
            if (('{' >> 3) != 15) {
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
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <19>  [!=]
                    │   │       ├── <16>  [^]
                    │   │       │   ├── <12> Var [x]
                    │   │       │   ╰── Constant Int [65]
                    │   │       ╰── Constant Int [75]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Char
                    │   ├── Initializer
                    │   │   ╰── Constant Int [132]
                    │   ╰── Static
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <42>  [!=]
                    │   │       ├── <37>  [|]
                    │   │       │   ├── Constant Int [33]
                    │   │       │   ╰── <35> Var [c]
                    │   │       ╰── <41> Unary [-]
                    │   │           ╰── Constant Int [91]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ul
                    │   ├── Type
                    │   │   ╰── Unsigned Long
                    │   ├── Initializer
                    │   │   ╰── Constant ULong [9259400834947493926]
                    │   ╰── Static
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <63>  [!=]
                    │   │       ├── <60>  [&]
                    │   │       │   ├── <56> Var [ul]
                    │   │       │   ╰── Constant Int [126]
                    │   │       ╰── Constant Int [38]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <77>  [!=]
                    │   │       ├── <74>  [<<]
                    │   │       │   ├── <70> Var [ul]
                    │   │       │   ╰── Constant Int [32]
                    │   │       ╰── Constant ULong [4611738958194278400]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <90>  [!=]
                    │   │       ├── <87>  [>>]
                    │   │       │   ├── Constant Int [123]
                    │   │       │   ╰── Constant Int [3]
                    │   │       ╰── Constant Int [15]
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
fn test_valid_extra_credit_bitwise_ops_chars() {
    let src = r#"
        int main(void) {
            unsigned char uc = 135;
            char c = -116;
            if ((uc & c) != 132) {
                return 1;
            }
            if ((uc | c) != -113) {
                return 2;
            }
            if (((c ^ 1001u) | 360l) != 4294966637) {
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
                    │   │   ╰── uc
                    │   ├── Type
                    │   │   ╰── Unsigned Char
                    │   ╰── Initializer
                    │       ╰── Constant Int [135]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── <16> Unary [-]
                    │           ╰── Constant Int [116]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <28>  [!=]
                    │   │       ├── <25>  [&]
                    │   │       │   ├── <20> Var [uc]
                    │   │       │   ╰── <23> Var [c]
                    │   │       ╰── Constant Int [132]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <45>  [!=]
                    │   │       ├── <40>  [|]
                    │   │       │   ├── <35> Var [uc]
                    │   │       │   ╰── <38> Var [c]
                    │   │       ╰── <44> Unary [-]
                    │   │           ╰── Constant Int [113]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <63>  [!=]
                    │   │       ├── <60>  [|]
                    │   │       │   ├── <56>  [^]
                    │   │       │   │   ├── <52> Var [c]
                    │   │       │   │   ╰── Constant UInt [1001]
                    │   │       │   ╰── Constant Long [360]
                    │   │       ╰── Constant Long [4294966637]
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
fn test_valid_extra_credit_char_consts_as_cases() {
    let src = r#"
        
        int main(void) {
            static int i = 65;
            switch (i) {
                case 100l:
                    return 1;
                case 'A':
                    return 0;
                case 'B':
                    return 2;
                case 2000u:
                    return 3;
                default:
                    return -1;
            }
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
                    │   │   ╰── Constant Int [65]
                    │   ╰── Static
                    ╰── Switch
                        ├── Expression
                        │   ╰── <13> Var [i]
                        ╰── Block
                            ├── Case [100]
                            │   ╰── Return
                            │       ╰── Constant Int [1]
                            ├── Case [65]
                            │   ╰── Return
                            │       ╰── Constant Int [0]
                            ├── Case [66]
                            │   ╰── Return
                            │       ╰── Constant Int [2]
                            ├── Case [2000]
                            │   ╰── Return
                            │       ╰── Constant Int [3]
                            ╰── Default
                                ╰── Return
                                    ╰── <32> Unary [-]
                                        ╰── Constant Int [1]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_compound_assign_chars() {
    let src = r#"
        int main(void) {
            static char c = 100;
            char c2 = 100;
            c += c2;
            if (c != -56) {
                return 1;
            }
            static unsigned char uc = 200;
            c2 = -100;
            uc /= c2;
            if (uc != 254) {
                return 2;
            }
            uc -= 250.0;
            if (uc != 4) {
                 return 3;
            }
            static signed char sc = 70;
            sc = -sc;
            sc *= c;
            if (sc != 80) {
                return 4;
            }
            if ((sc %= c) != 24) {
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
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Char
                    │   ├── Initializer
                    │   │   ╰── Constant Int [100]
                    │   ╰── Static
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c2
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── Constant Int [100]
                    ├── <23> Assign [+=]
                    │   ├── <19> Var [c]
                    │   ╰── <22> Var [c2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <31>  [!=]
                    │   │       ├── <26> Var [c]
                    │   │       ╰── <30> Unary [-]
                    │   │           ╰── Constant Int [56]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── uc
                    │   ├── Type
                    │   │   ╰── Unsigned Char
                    │   ├── Initializer
                    │   │   ╰── Constant Int [200]
                    │   ╰── Static
                    ├── <50> Assign [=]
                    │   ├── <45> Var [c2]
                    │   ╰── <49> Unary [-]
                    │       ╰── Constant Int [100]
                    ├── <57> Assign [/=]
                    │   ├── <53> Var [uc]
                    │   ╰── <56> Var [c2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <63>  [!=]
                    │   │       ├── <60> Var [uc]
                    │   │       ╰── Constant Int [254]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── <73> Assign [-=]
                    │   ├── <70> Var [uc]
                    │   ╰── Constant Double [+2.5e2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <79>  [!=]
                    │   │       ├── <76> Var [uc]
                    │   │       ╰── Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── sc
                    │   ├── Type
                    │   │   ╰── Signed Char
                    │   ├── Initializer
                    │   │   ╰── Constant Int [70]
                    │   ╰── Static
                    ├── <99> Assign [=]
                    │   ├── <93> Var [sc]
                    │   ╰── <98> Unary [-]
                    │       ╰── <97> Var [sc]
                    ├── <106> Assign [*=]
                    │   ├── <102> Var [sc]
                    │   ╰── <105> Var [c]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <112>  [!=]
                    │   │       ├── <109> Var [sc]
                    │   │       ╰── Constant Int [80]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <127>  [!=]
                    │   │       ├── <124> Assign [&=]
                    │   │       │   ├── <119> Var [sc]
                    │   │       │   ╰── <122> Var [c]
                    │   │       ╰── Constant Int [24]
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
fn test_valid_extra_credit_compound_bitwise_ops_chars() {
    let src = r#"
        int main(void) {
            signed char arr[5] = {-128, -120, -2, 1, 120};
            unsigned char u_arr[4] = {0, 170, 250, 255};
            arr[0] ^= 12345;
            arr[1] |= u_arr[3];
            arr[2] &= u_arr[1] - (unsigned char) 185;
            arr[3] <<= 7u;
            static long x = 32;
            arr[4] >>= 31;
            u_arr[3] <<= 12;
            u_arr[2] >>= (x - 1);
            u_arr[1] |= -399;
            x = -4296140120l;
            u_arr[0] ^= x;
            if (arr[0] != -71) {
                return 1;
            }
            if (arr[1] != -1) {
                return 2;
            }
            if (arr[2] != -16) {
                return 3;
            }
            if (arr[3] != -128) {
                return 4;
            }
            if (arr[4] != 0) {
                return 5;
            }
            if (u_arr[0] != 168) {
                return 6;
            }
            if (u_arr[1] != 251) {
                return 7;
            }
            if (u_arr[2] != 0) {
                return 8;
            }
            if (u_arr[3] != 0) {
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
                    │   │   ╰── arr
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 5
                    │   │       ╰── Signed Char
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <12> Unary [-]
                    │           │   ╰── Constant Int [128]
                    │           ├── <16> Unary [-]
                    │           │   ╰── Constant Int [120]
                    │           ├── <20> Unary [-]
                    │           │   ╰── Constant Int [2]
                    │           ├── Constant Int [1]
                    │           ╰── Constant Int [120]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── u_arr
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 4
                    │   │       ╰── Unsigned Char
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── Constant Int [0]
                    │           ├── Constant Int [170]
                    │           ├── Constant Int [250]
                    │           ╰── Constant Int [255]
                    ├── <49> Assign [^=]
                    │   ├── <46> Subscript
                    │   │   ├── <44> Var [arr]
                    │   │   ╰── Constant Int [0]
                    │   ╰── Constant Int [12345]
                    ├── <60> Assign [|=]
                    │   ├── <54> Subscript
                    │   │   ├── <52> Var [arr]
                    │   │   ╰── Constant Int [1]
                    │   ╰── <59> Subscript
                    │       ├── <57> Var [u_arr]
                    │       ╰── Constant Int [3]
                    ├── <77> Assign [&=]
                    │   ├── <65> Subscript
                    │   │   ├── <63> Var [arr]
                    │   │   ╰── Constant Int [2]
                    │   ╰── <76>  [-]
                    │       ├── <70> Subscript
                    │       │   ├── <68> Var [u_arr]
                    │       │   ╰── Constant Int [1]
                    │       ╰── <75> Cast
                    │           ├── Target
                    │           │   ╰── Unsigned Char
                    │           ╰── Expression
                    │               ╰── Constant Int [185]
                    ├── <85> Assign [<<=]
                    │   ├── <82> Subscript
                    │   │   ├── <80> Var [arr]
                    │   │   ╰── Constant Int [3]
                    │   ╰── Constant UInt [7]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Long
                    │   ├── Initializer
                    │   │   ╰── Constant Int [32]
                    │   ╰── Static
                    ├── <100> Assign [>>=]
                    │   ├── <97> Subscript
                    │   │   ├── <95> Var [arr]
                    │   │   ╰── Constant Int [4]
                    │   ╰── Constant Int [31]
                    ├── <108> Assign [<<=]
                    │   ├── <105> Subscript
                    │   │   ├── <103> Var [u_arr]
                    │   │   ╰── Constant Int [3]
                    │   ╰── Constant Int [12]
                    ├── <121> Assign [>>=]
                    │   ├── <113> Subscript
                    │   │   ├── <111> Var [u_arr]
                    │   │   ╰── Constant Int [2]
                    │   ╰── <120>  [-]
                    │       ├── <116> Var [x]
                    │       ╰── Constant Int [1]
                    ├── <131> Assign [|=]
                    │   ├── <126> Subscript
                    │   │   ├── <124> Var [u_arr]
                    │   │   ╰── Constant Int [1]
                    │   ╰── <130> Unary [-]
                    │       ╰── Constant Int [399]
                    ├── <139> Assign [=]
                    │   ├── <134> Var [x]
                    │   ╰── <138> Unary [-]
                    │       ╰── Constant Long [4296140120]
                    ├── <148> Assign [^=]
                    │   ├── <144> Subscript
                    │   │   ├── <142> Var [u_arr]
                    │   │   ╰── Constant Int [0]
                    │   ╰── <147> Var [x]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <158>  [!=]
                    │   │       ├── <153> Subscript
                    │   │       │   ├── <151> Var [arr]
                    │   │       │   ╰── Constant Int [0]
                    │   │       ╰── <157> Unary [-]
                    │   │           ╰── Constant Int [71]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <172>  [!=]
                    │   │       ├── <167> Subscript
                    │   │       │   ├── <165> Var [arr]
                    │   │       │   ╰── Constant Int [1]
                    │   │       ╰── <171> Unary [-]
                    │   │           ╰── Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <186>  [!=]
                    │   │       ├── <181> Subscript
                    │   │       │   ├── <179> Var [arr]
                    │   │       │   ╰── Constant Int [2]
                    │   │       ╰── <185> Unary [-]
                    │   │           ╰── Constant Int [16]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <200>  [!=]
                    │   │       ├── <195> Subscript
                    │   │       │   ├── <193> Var [arr]
                    │   │       │   ╰── Constant Int [3]
                    │   │       ╰── <199> Unary [-]
                    │   │           ╰── Constant Int [128]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <212>  [!=]
                    │   │       ├── <209> Subscript
                    │   │       │   ├── <207> Var [arr]
                    │   │       │   ╰── Constant Int [4]
                    │   │       ╰── Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <224>  [!=]
                    │   │       ├── <221> Subscript
                    │   │       │   ├── <219> Var [u_arr]
                    │   │       │   ╰── Constant Int [0]
                    │   │       ╰── Constant Int [168]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <236>  [!=]
                    │   │       ├── <233> Subscript
                    │   │       │   ├── <231> Var [u_arr]
                    │   │       │   ╰── Constant Int [1]
                    │   │       ╰── Constant Int [251]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <248>  [!=]
                    │   │       ├── <245> Subscript
                    │   │       │   ├── <243> Var [u_arr]
                    │   │       │   ╰── Constant Int [2]
                    │   │       ╰── Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <260>  [!=]
                    │   │       ├── <257> Subscript
                    │   │       │   ├── <255> Var [u_arr]
                    │   │       │   ╰── Constant Int [3]
                    │   │       ╰── Constant Int [0]
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
fn test_valid_extra_credit_incr_decr_chars() {
    let src = r#"
        
        int main(void) {
            static char chars[5] = {123, 124, 125, 126, 127};
            if (chars[0]++ != 123) {
                return 1;
            }
            if (chars[1]-- != 124) {
                return 2;
            }
            if (++chars[2] != 126) {
                return 3;
            }
            if (--chars[3] != 125) {
                return 4;
            }
            if (++chars[4] != -128) {
                return 5;
            }
            if (chars[0] != 124) {
                return 6;
            }
            if (chars[1] != 123) {
                return 7;
            }
            if (chars[2] != 126) {
                return 8;
            }
            if (chars[3] != 125) {
                return 9;
            }
            if (chars[4] != -128) {
                return 10;
            }
            signed char c = -128;
            c--;
            if (c != 127) {
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
                    │   │   ╰── chars
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 5
                    │   │       ╰── Char
                    │   ├── Initializer
                    │   │   ╰── Compound
                    │   │       ├── Constant Int [123]
                    │   │       ├── Constant Int [124]
                    │   │       ├── Constant Int [125]
                    │   │       ├── Constant Int [126]
                    │   │       ╰── Constant Int [127]
                    │   ╰── Static
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <31>  [!=]
                    │   │       ├── <28> Postfix [++]
                    │   │       │   ╰── <26> Subscript
                    │   │       │       ├── <24> Var [chars]
                    │   │       │       ╰── Constant Int [0]
                    │   │       ╰── Constant Int [123]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <45>  [!=]
                    │   │       ├── <42> Postfix [--]
                    │   │       │   ╰── <40> Subscript
                    │   │       │       ├── <38> Var [chars]
                    │   │       │       ╰── Constant Int [1]
                    │   │       ╰── Constant Int [124]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <59>  [!=]
                    │   │       ├── <56> Unary [++]
                    │   │       │   ╰── <55> Subscript
                    │   │       │       ├── <53> Var [chars]
                    │   │       │       ╰── Constant Int [2]
                    │   │       ╰── Constant Int [126]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <73>  [!=]
                    │   │       ├── <70> Unary [--]
                    │   │       │   ╰── <69> Subscript
                    │   │       │       ├── <67> Var [chars]
                    │   │       │       ╰── Constant Int [3]
                    │   │       ╰── Constant Int [125]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <89>  [!=]
                    │   │       ├── <84> Unary [++]
                    │   │       │   ╰── <83> Subscript
                    │   │       │       ├── <81> Var [chars]
                    │   │       │       ╰── Constant Int [4]
                    │   │       ╰── <88> Unary [-]
                    │   │           ╰── Constant Int [128]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <101>  [!=]
                    │   │       ├── <98> Subscript
                    │   │       │   ├── <96> Var [chars]
                    │   │       │   ╰── Constant Int [0]
                    │   │       ╰── Constant Int [124]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <113>  [!=]
                    │   │       ├── <110> Subscript
                    │   │       │   ├── <108> Var [chars]
                    │   │       │   ╰── Constant Int [1]
                    │   │       ╰── Constant Int [123]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <125>  [!=]
                    │   │       ├── <122> Subscript
                    │   │       │   ├── <120> Var [chars]
                    │   │       │   ╰── Constant Int [2]
                    │   │       ╰── Constant Int [126]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <137>  [!=]
                    │   │       ├── <134> Subscript
                    │   │       │   ├── <132> Var [chars]
                    │   │       │   ╰── Constant Int [3]
                    │   │       ╰── Constant Int [125]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <151>  [!=]
                    │   │       ├── <146> Subscript
                    │   │       │   ├── <144> Var [chars]
                    │   │       │   ╰── Constant Int [4]
                    │   │       ╰── <150> Unary [-]
                    │   │           ╰── Constant Int [128]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [10]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Signed Char
                    │   ╰── Initializer
                    │       ╰── <162> Unary [-]
                    │           ╰── Constant Int [128]
                    ├── <168> Postfix [--]
                    │   ╰── <166> Var [c]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <174>  [!=]
                    │   │       ├── <171> Var [c]
                    │   │       ╰── Constant Int [127]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [11]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_incr_decr_unsigned_chars() {
    let src = r#"
        
        int main(void) {
            unsigned char chars[5] = {0, 2, 4, 253, 255};
            if (chars[0]--) {
                return 1;
            }
            if (chars[1]++ != 2) {
                return 2;
            }
            if (--chars[3] != 252) {
                return 3;
            }
            if (++chars[4] != 0) {
                return 4;
            }
            if (chars[0] != 255) {
                return 5;
            }
            if (chars[1] != 3) {
                return 6;
            }
            if (chars[2] != 4) {
                return 7;
            }
            if (chars[3] != 252) {
                return 8;
            }
            if (chars[4]) {
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
                    │   │   ╰── chars
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 5
                    │   │       ╰── Unsigned Char
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── Constant Int [0]
                    │           ├── Constant Int [2]
                    │           ├── Constant Int [4]
                    │           ├── Constant Int [253]
                    │           ╰── Constant Int [255]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <27> Postfix [--]
                    │   │       ╰── <25> Subscript
                    │   │           ├── <23> Var [chars]
                    │   │           ╰── Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <41>  [!=]
                    │   │       ├── <38> Postfix [++]
                    │   │       │   ╰── <36> Subscript
                    │   │       │       ├── <34> Var [chars]
                    │   │       │       ╰── Constant Int [1]
                    │   │       ╰── Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <55>  [!=]
                    │   │       ├── <52> Unary [--]
                    │   │       │   ╰── <51> Subscript
                    │   │       │       ├── <49> Var [chars]
                    │   │       │       ╰── Constant Int [3]
                    │   │       ╰── Constant Int [252]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <69>  [!=]
                    │   │       ├── <66> Unary [++]
                    │   │       │   ╰── <65> Subscript
                    │   │       │       ├── <63> Var [chars]
                    │   │       │       ╰── Constant Int [4]
                    │   │       ╰── Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <81>  [!=]
                    │   │       ├── <78> Subscript
                    │   │       │   ├── <76> Var [chars]
                    │   │       │   ╰── Constant Int [0]
                    │   │       ╰── Constant Int [255]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <93>  [!=]
                    │   │       ├── <90> Subscript
                    │   │       │   ├── <88> Var [chars]
                    │   │       │   ╰── Constant Int [1]
                    │   │       ╰── Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <105>  [!=]
                    │   │       ├── <102> Subscript
                    │   │       │   ├── <100> Var [chars]
                    │   │       │   ╰── Constant Int [2]
                    │   │       ╰── Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <117>  [!=]
                    │   │       ├── <114> Subscript
                    │   │       │   ├── <112> Var [chars]
                    │   │       │   ╰── Constant Int [3]
                    │   │       ╰── Constant Int [252]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <126> Subscript
                    │   │       ├── <124> Var [chars]
                    │   │       ╰── Constant Int [4]
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
fn test_valid_extra_credit_promote_switch_cond() {
    let src = r#"
        int main(void) {
            char c = 100;
            switch (c) {
                case 0:
                    return 1;
                case 100:
                    return 0;
                case 356:
                    return 2;
                default:
                    return 3;
            }
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── Constant Int [100]
                    ╰── Switch
                        ├── Expression
                        │   ╰── <12> Var [c]
                        ╰── Block
                            ├── Case [0]
                            │   ╰── Return
                            │       ╰── Constant Int [1]
                            ├── Case [100]
                            │   ╰── Return
                            │       ╰── Constant Int [0]
                            ├── Case [356]
                            │   ╰── Return
                            │       ╰── Constant Int [2]
                            ╰── Default
                                ╰── Return
                                    ╰── Constant Int [3]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_promote_switch_cond_2() {
    let src = r#"
        int main(void) {
            char c = -56;
            switch (c) {
                case 33554632:
                    return 1;
                default:
                    return 0;
            }
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── <10> Unary [-]
                    │           ╰── Constant Int [56]
                    ╰── Switch
                        ├── Expression
                        │   ╰── <14> Var [c]
                        ╰── Block
                            ├── Case [33554632]
                            │   ╰── Return
                            │       ╰── Constant Int [1]
                            ╰── Default
                                ╰── Return
                                    ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_switch_on_char_const() {
    let src = r#"
        
        int main(void) {
            switch ('x') {
                case 1:
                    return 1;
                case 2:
                    return 2;
                case 120:
                    return 0;
                default:
                    return -1;
            }
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Switch
                        ├── Expression
                        │   ╰── Constant Int [120]
                        ╰── Block
                            ├── Case [1]
                            │   ╰── Return
                            │       ╰── Constant Int [1]
                            ├── Case [2]
                            │   ╰── Return
                            │       ╰── Constant Int [2]
                            ├── Case [120]
                            │   ╰── Return
                            │       ╰── Constant Int [0]
                            ╰── Default
                                ╰── Return
                                    ╰── <20> Unary [-]
                                        ╰── Constant Int [1]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_char_arguments() {
    let src = r#"
        int check_args(char a, signed char b, char c, unsigned char d, char e, char f, signed char g, char h) {
            char expected_a = 5;
            signed char expected_b = -12;
            char expected_c = 117;
            unsigned char expected_d = 254;
            char expected_e = 1;
            char expected_f = -20;
            signed char expected_g = 60;
            char expected_h = 100;
            if (expected_a != a) {
             return 1;
            }
            if (expected_b != b) {
             return 2;
            }
            if (expected_c != c) {
             return 3;
            }
            if (expected_d != d) {
             return 4;
            }
            if (expected_e != e) {
             return 5;
            }
            if (expected_f != f) {
             return 6;
            }
            if (expected_g != g) {
             return 7;
            }
            if (expected_h != h) {
             return 8;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [check_args]
                ├── Parameters
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── a
                │   │   ╰── Type
                │   │       ╰── Char
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── b
                │   │   ╰── Type
                │   │       ╰── Signed Char
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── c
                │   │   ╰── Type
                │   │       ╰── Char
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── d
                │   │   ╰── Type
                │   │       ╰── Unsigned Char
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── e
                │   │   ╰── Type
                │   │       ╰── Char
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── f
                │   │   ╰── Type
                │   │       ╰── Char
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── g
                │   │   ╰── Type
                │   │       ╰── Signed Char
                │   ╰── Param
                │       ├── Name
                │       │   ╰── h
                │       ╰── Type
                │           ╰── Char
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── expected_a
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── Constant Int [5]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── expected_b
                    │   ├── Type
                    │   │   ╰── Signed Char
                    │   ╰── Initializer
                    │       ╰── <40> Unary [-]
                    │           ╰── Constant Int [12]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── expected_c
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── Constant Int [117]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── expected_d
                    │   ├── Type
                    │   │   ╰── Unsigned Char
                    │   ╰── Initializer
                    │       ╰── Constant Int [254]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── expected_e
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── expected_f
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── <66> Unary [-]
                    │           ╰── Constant Int [20]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── expected_g
                    │   ├── Type
                    │   │   ╰── Signed Char
                    │   ╰── Initializer
                    │       ╰── Constant Int [60]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── expected_h
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── Constant Int [100]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <86>  [!=]
                    │   │       ├── <82> Var [expected_a]
                    │   │       ╰── <85> Var [a]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <97>  [!=]
                    │   │       ├── <93> Var [expected_b]
                    │   │       ╰── <96> Var [b]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <108>  [!=]
                    │   │       ├── <104> Var [expected_c]
                    │   │       ╰── <107> Var [c]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <119>  [!=]
                    │   │       ├── <115> Var [expected_d]
                    │   │       ╰── <118> Var [d]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <130>  [!=]
                    │   │       ├── <126> Var [expected_e]
                    │   │       ╰── <129> Var [e]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <141>  [!=]
                    │   │       ├── <137> Var [expected_f]
                    │   │       ╰── <140> Var [f]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <152>  [!=]
                    │   │       ├── <148> Var [expected_g]
                    │   │       ╰── <151> Var [g]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <163>  [!=]
                    │   │       ├── <159> Var [expected_h]
                    │   │       ╰── <162> Var [h]
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
fn test_valid_libraries_char_arguments_client() {
    let src = r#"
        
        int check_args(char a, signed char b, char c, unsigned char d, char e, char f, signed char g, char h);
        int main(void) {
            char a = 5;
            signed char b = -12;
            char c = 117;
            unsigned char d = 254;
            char e = 1;
            char f = -20;
            signed char g = 60;
            char h = 100;
            return check_args(a, b, c, d, e, f, g, h);
        }
    "#;
    let expected = r#"
        Program
            ├── Function [check_args]
            │   ╰── Parameters
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── a
            │       │   ╰── Type
            │       │       ╰── Char
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── b
            │       │   ╰── Type
            │       │       ╰── Signed Char
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── c
            │       │   ╰── Type
            │       │       ╰── Char
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── d
            │       │   ╰── Type
            │       │       ╰── Unsigned Char
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── e
            │       │   ╰── Type
            │       │       ╰── Char
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── f
            │       │   ╰── Type
            │       │       ╰── Char
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── g
            │       │   ╰── Type
            │       │       ╰── Signed Char
            │       ╰── Param
            │           ├── Name
            │           │   ╰── h
            │           ╰── Type
            │               ╰── Char
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── Constant Int [5]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Signed Char
                    │   ╰── Initializer
                    │       ╰── <45> Unary [-]
                    │           ╰── Constant Int [12]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── Constant Int [117]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Unsigned Char
                    │   ╰── Initializer
                    │       ╰── Constant Int [254]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── e
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── f
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── <71> Unary [-]
                    │           ╰── Constant Int [20]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── g
                    │   ├── Type
                    │   │   ╰── Signed Char
                    │   ╰── Initializer
                    │       ╰── Constant Int [60]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── h
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── Constant Int [100]
                    ╰── Return
                        ╰── <103> FunctionCall [check_args]
                            ├── <88> Var [a]
                            ├── <90> Var [b]
                            ├── <92> Var [c]
                            ├── <94> Var [d]
                            ├── <96> Var [e]
                            ├── <98> Var [f]
                            ├── <100> Var [g]
                            ╰── <102> Var [h]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_global_char() {
    let src = r#"
        char c = 100;
        unsigned char uc = 250;
        signed char sc = 0;
        int update_global_chars(void) {
            c = c + 10;
            uc = uc + 10;
            sc = sc - 10;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── c
            │   ├── Type
            │   │   ╰── Char
            │   ╰── Initializer
            │       ╰── Constant Int [100]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── uc
            │   ├── Type
            │   │   ╰── Unsigned Char
            │   ╰── Initializer
            │       ╰── Constant Int [250]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── sc
            │   ├── Type
            │   │   ╰── Signed Char
            │   ╰── Initializer
            │       ╰── Constant Int [0]
            ╰── Function [update_global_chars]
                ╰── Body
                    ├── <31> Assign [=]
                    │   ├── <24> Var [c]
                    │   ╰── <30>  [+]
                    │       ├── <27> Var [c]
                    │       ╰── Constant Int [10]
                    ├── <41> Assign [=]
                    │   ├── <34> Var [uc]
                    │   ╰── <40>  [+]
                    │       ├── <37> Var [uc]
                    │       ╰── Constant Int [10]
                    ├── <51> Assign [=]
                    │   ├── <44> Var [sc]
                    │   ╰── <50>  [-]
                    │       ├── <47> Var [sc]
                    │       ╰── Constant Int [10]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_global_char_client() {
    let src = r#"
        extern char c;
        extern unsigned char uc;
        extern signed char sc;
        int update_global_chars(void);
        int main(void) {
            if (c != 100) {
                return 1;
            }
            if (uc != 250) {
                return 2;
            }
            if (sc != 0) {
                return 3;
            }
            update_global_chars();
            if (c != 110) {
                return 4;
            }
            if (uc != 4) {
                return 5;
            }
            if (sc != -10) {
                return 6;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── c
            │   ├── Type
            │   │   ╰── Char
            │   ╰── Extern
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── uc
            │   ├── Type
            │   │   ╰── Unsigned Char
            │   ╰── Extern
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── sc
            │   ├── Type
            │   │   ╰── Signed Char
            │   ╰── Extern
            ├── Function [update_global_chars]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <29>  [!=]
                    │   │       ├── <26> Var [c]
                    │   │       ╰── Constant Int [100]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <39>  [!=]
                    │   │       ├── <36> Var [uc]
                    │   │       ╰── Constant Int [250]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <49>  [!=]
                    │   │       ├── <46> Var [sc]
                    │   │       ╰── Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── <56> FunctionCall [update_global_chars]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <62>  [!=]
                    │   │       ├── <59> Var [c]
                    │   │       ╰── Constant Int [110]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <72>  [!=]
                    │   │       ├── <69> Var [uc]
                    │   │       ╰── Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <84>  [!=]
                    │   │       ├── <79> Var [sc]
                    │   │       ╰── <83> Unary [-]
                    │   │           ╰── Constant Int [10]
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
fn test_valid_libraries_return_char() {
    let src = r#"
        char return_char(void) {
            return 5369233654l;
        }
        signed char return_schar(void) {
            return 5369233654l;
        }
        unsigned char return_uchar(void) {
            return 5369233654l;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [return_char]
            │   ╰── Body
            │       ╰── Return
            │           ╰── Constant Long [5369233654]
            ├── Function [return_schar]
            │   ╰── Body
            │       ╰── Return
            │           ╰── Constant Long [5369233654]
            ╰── Function [return_uchar]
                ╰── Body
                    ╰── Return
                        ╰── Constant Long [5369233654]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_return_char_client() {
    let src = r#"
        char return_char(void);
        signed char return_schar(void);
        unsigned char return_uchar(void);
        int main(void) {
            char char_array[3] = {121, -122, -3};
            char retval_c = return_char();
            char char_array2[3] = {-5, 88, -100};
            signed char retval_sc = return_schar();
            char char_array3[3] = {10, 11, 12};
            unsigned char retval_uc = return_uchar();
            char char_array4[2] = {-5, -6};
            if (char_array[0] != 121 || char_array[1] != -122 || char_array[2] != -3) {
                return 1;
            }
            if (retval_c != -10) {
                return 2;
            }
            if (char_array2[0] != -5 || char_array2[1] != 88 ||
                char_array2[2] != -100) {
                return 3;
            }
            if (retval_sc != -10) {
                return 4;
            }
            if (char_array3[0] != 10 || char_array3[1] != 11 || char_array3[2] != 12) {
                return 5;
            }
            if (retval_uc != 246) {
                return 6;
            }
            if (char_array4[0] != -5 || char_array4[1] != -6) {
                return 7;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [return_char]
            ├── Function [return_schar]
            ├── Function [return_uchar]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── char_array
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 3
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── Constant Int [121]
                    │           ├── <29> Unary [-]
                    │           │   ╰── Constant Int [122]
                    │           ╰── <33> Unary [-]
                    │               ╰── Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── retval_c
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── <41> FunctionCall [return_char]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── char_array2
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 3
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <51> Unary [-]
                    │           │   ╰── Constant Int [5]
                    │           ├── Constant Int [88]
                    │           ╰── <57> Unary [-]
                    │               ╰── Constant Int [100]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── retval_sc
                    │   ├── Type
                    │   │   ╰── Signed Char
                    │   ╰── Initializer
                    │       ╰── <65> FunctionCall [return_schar]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── char_array3
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 3
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── Constant Int [10]
                    │           ├── Constant Int [11]
                    │           ╰── Constant Int [12]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── retval_uc
                    │   ├── Type
                    │   │   ╰── Unsigned Char
                    │   ╰── Initializer
                    │       ╰── <85> FunctionCall [return_uchar]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── char_array4
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 2
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <95> Unary [-]
                    │           │   ╰── Constant Int [5]
                    │           ╰── <99> Unary [-]
                    │               ╰── Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <131>  [||]
                    │   │       ├── <120>  [||]
                    │   │       │   ├── <109>  [!=]
                    │   │       │   │   ├── <106> Subscript
                    │   │       │   │   │   ├── <104> Var [char_array]
                    │   │       │   │   │   ╰── Constant Int [0]
                    │   │       │   │   ╰── Constant Int [121]
                    │   │       │   ╰── <119>  [!=]
                    │   │       │       ├── <114> Subscript
                    │   │       │       │   ├── <112> Var [char_array]
                    │   │       │       │   ╰── Constant Int [1]
                    │   │       │       ╰── <118> Unary [-]
                    │   │       │           ╰── Constant Int [122]
                    │   │       ╰── <130>  [!=]
                    │   │           ├── <125> Subscript
                    │   │           │   ├── <123> Var [char_array]
                    │   │           │   ╰── Constant Int [2]
                    │   │           ╰── <129> Unary [-]
                    │   │               ╰── Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <143>  [!=]
                    │   │       ├── <138> Var [retval_c]
                    │   │       ╰── <142> Unary [-]
                    │   │           ╰── Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <177>  [||]
                    │   │       ├── <166>  [||]
                    │   │       │   ├── <157>  [!=]
                    │   │       │   │   ├── <152> Subscript
                    │   │       │   │   │   ├── <150> Var [char_array2]
                    │   │       │   │   │   ╰── Constant Int [0]
                    │   │       │   │   ╰── <156> Unary [-]
                    │   │       │   │       ╰── Constant Int [5]
                    │   │       │   ╰── <165>  [!=]
                    │   │       │       ├── <162> Subscript
                    │   │       │       │   ├── <160> Var [char_array2]
                    │   │       │       │   ╰── Constant Int [1]
                    │   │       │       ╰── Constant Int [88]
                    │   │       ╰── <176>  [!=]
                    │   │           ├── <171> Subscript
                    │   │           │   ├── <169> Var [char_array2]
                    │   │           │   ╰── Constant Int [2]
                    │   │           ╰── <175> Unary [-]
                    │   │               ╰── Constant Int [100]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <189>  [!=]
                    │   │       ├── <184> Var [retval_sc]
                    │   │       ╰── <188> Unary [-]
                    │   │           ╰── Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <219>  [||]
                    │   │       ├── <210>  [||]
                    │   │       │   ├── <201>  [!=]
                    │   │       │   │   ├── <198> Subscript
                    │   │       │   │   │   ├── <196> Var [char_array3]
                    │   │       │   │   │   ╰── Constant Int [0]
                    │   │       │   │   ╰── Constant Int [10]
                    │   │       │   ╰── <209>  [!=]
                    │   │       │       ├── <206> Subscript
                    │   │       │       │   ├── <204> Var [char_array3]
                    │   │       │       │   ╰── Constant Int [1]
                    │   │       │       ╰── Constant Int [11]
                    │   │       ╰── <218>  [!=]
                    │   │           ├── <215> Subscript
                    │   │           │   ├── <213> Var [char_array3]
                    │   │           │   ╰── Constant Int [2]
                    │   │           ╰── Constant Int [12]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <229>  [!=]
                    │   │       ├── <226> Var [retval_uc]
                    │   │       ╰── Constant Int [246]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <254>  [||]
                    │   │       ├── <243>  [!=]
                    │   │       │   ├── <238> Subscript
                    │   │       │   │   ├── <236> Var [char_array4]
                    │   │       │   │   ╰── Constant Int [0]
                    │   │       │   ╰── <242> Unary [-]
                    │   │       │       ╰── Constant Int [5]
                    │   │       ╰── <253>  [!=]
                    │   │           ├── <248> Subscript
                    │   │           │   ├── <246> Var [char_array4]
                    │   │           │   ╰── Constant Int [1]
                    │   │           ╰── <252> Unary [-]
                    │   │               ╰── Constant Int [6]
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
fn test_valid_strings_as_initializers_adjacent_strings_in_initializer() {
    let src = r#"
        int strcmp(char *s1, char *s2);
        int main(void) {
            char multi_string[6] =
                "yes"
                "no";
            char nested_multi_string[2][3] = {
                "a"
                "b",
                "c"
                "d"};
            if (strcmp(multi_string, "yesno"))
                return 1;
            if (strcmp(nested_multi_string[0], "ab"))
                return 2;
            if (strcmp(nested_multi_string[1], "cd"))
                return 3;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [strcmp]
            │   ╰── Parameters
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── s1
            │       │   ╰── Type
            │       │       ╰── Pointer
            │       │           ╰── Char
            │       ╰── Param
            │           ├── Name
            │           │   ╰── s2
            │           ╰── Type
            │               ╰── Pointer
            │                   ╰── Char
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── multi_string
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 6
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── <23> "yesno"
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── nested_multi_string
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 2
                    │   │       ╰── Array
                    │   │           ├── 3
                    │   │           ╰── Char
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <33> "ab"
                    │           ╰── <35> "cd"
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <43> FunctionCall [strcmp]
                    │   │       ├── <41> Var [multi_string]
                    │   │       ╰── <42> "yesno"
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <53> FunctionCall [strcmp]
                    │   │       ├── <51> Subscript
                    │   │       │   ├── <49> Var [nested_multi_string]
                    │   │       │   ╰── Constant Int [0]
                    │   │       ╰── <52> "ab"
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <63> FunctionCall [strcmp]
                    │   │       ├── <61> Subscript
                    │   │       │   ├── <59> Var [nested_multi_string]
                    │   │       │   ╰── Constant Int [1]
                    │   │       ╰── <62> "cd"
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [3]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_strings_as_initializers_array_init_special_chars() {
    let src = r#"
        
        int main(void) {
            char special[6] = "\a\b\n	";
            if (special[0] != '\a') {
                return 1;
            }
            if (special[1] != '\b') {
                return 2;
            }
            if (special[2] != '\n') {
                return 3;
            }
            if (special[3] != '\v') {
                return 4;
            }
            if (special[4] != '\f') {
                return 5;
            }
            if (special[5] != '\t') {
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
                    │   │   ╰── special
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 6
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── <10> "
        	"
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <19>  [!=]
                    │   │       ├── <16> Subscript
                    │   │       │   ├── <14> Var [special]
                    │   │       │   ╰── Constant Int [0]
                    │   │       ╰── Constant Int [7]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <31>  [!=]
                    │   │       ├── <28> Subscript
                    │   │       │   ├── <26> Var [special]
                    │   │       │   ╰── Constant Int [1]
                    │   │       ╰── Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <43>  [!=]
                    │   │       ├── <40> Subscript
                    │   │       │   ├── <38> Var [special]
                    │   │       │   ╰── Constant Int [2]
                    │   │       ╰── Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <55>  [!=]
                    │   │       ├── <52> Subscript
                    │   │       │   ├── <50> Var [special]
                    │   │       │   ╰── Constant Int [3]
                    │   │       ╰── Constant Int [11]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <67>  [!=]
                    │   │       ├── <64> Subscript
                    │   │       │   ├── <62> Var [special]
                    │   │       │   ╰── Constant Int [4]
                    │   │       ╰── Constant Int [12]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <79>  [!=]
                    │   │       ├── <76> Subscript
                    │   │       │   ├── <74> Var [special]
                    │   │       │   ╰── Constant Int [5]
                    │   │       ╰── Constant Int [9]
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
fn test_valid_strings_as_initializers_literals_and_compound_initializers() {
    let src = r#"
        signed char static_array[3][4] = {{'a', 'b', 'c', 'd'}, "efgh", "ijk"};
        int main(void) {
            unsigned char auto_array[2][3] = {"lmn", {'o', 'p'}};
            for (int i = 0; i < 3; i = i + 1)
                for (int j = 0; j < 4; j = j + 1)
                    if (static_array[i][j] != "abcdefghijk"[i * 4 + j])
                        return 1;
            for (int i = 0; i < 2; i = i + 1)
                for (int j = 0; j < 3; j = j + 1)
                    if (auto_array[i][j] != "lmnop"[i * 3 + j])
                        return 2;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── static_array
            │   ├── Type
            │   │   ╰── Array
            │   │       ├── 3
            │   │       ╰── Array
            │   │           ├── 4
            │   │           ╰── Signed Char
            │   ╰── Initializer
            │       ╰── Compound
            │           ├── Compound
            │           │   ├── Constant Int [97]
            │           │   ├── Constant Int [98]
            │           │   ├── Constant Int [99]
            │           │   ╰── Constant Int [100]
            │           ├── <17> "efgh"
            │           ╰── <19> "ijk"
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── auto_array
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 2
                    │   │       ╰── Array
                    │   │           ├── 3
                    │   │           ╰── Unsigned Char
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <34> "lmn"
                    │           ╰── Compound
                    │               ├── Constant Int [111]
                    │               ╰── Constant Int [112]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <54>  [<]
                    │   │       ├── <51> Var [i]
                    │   │       ╰── Constant Int [3]
                    │   ├── Condition
                    │   │   ╰── <63> Assign [=]
                    │   │       ├── <56> Var [i]
                    │   │       ╰── <62>  [+]
                    │   │           ├── <59> Var [i]
                    │   │           ╰── Constant Int [1]
                    │   ╰── For
                    │       ├── Init
                    │       │   ╰── VarDeclaration
                    │       │       ├── Name
                    │       │       │   ╰── j
                    │       │       ├── Type
                    │       │       │   ╰── Int
                    │       │       ╰── Initializer
                    │       │           ╰── Constant Int [0]
                    │       ├── Condition
                    │       │   ╰── <75>  [<]
                    │       │       ├── <72> Var [j]
                    │       │       ╰── Constant Int [4]
                    │       ├── Condition
                    │       │   ╰── <84> Assign [=]
                    │       │       ├── <77> Var [j]
                    │       │       ╰── <83>  [+]
                    │       │           ├── <80> Var [j]
                    │       │           ╰── Constant Int [1]
                    │       ╰── If
                    │           ├── Condition
                    │           │   ╰── <105>  [!=]
                    │           │       ├── <92> Subscript
                    │           │       │   ├── <89> Subscript
                    │           │       │   │   ├── <86> Var [static_array]
                    │           │       │   │   ╰── <88> Var [i]
                    │           │       │   ╰── <91> Var [j]
                    │           │       ╰── <104> Subscript
                    │           │           ├── <94> "abcdefghijk"
                    │           │           ╰── <103>  [+]
                    │           │               ├── <99>  [*]
                    │           │               │   ├── <96> Var [i]
                    │           │               │   ╰── Constant Int [4]
                    │           │               ╰── <102> Var [j]
                    │           ╰── Then
                    │               ╰── Return
                    │                   ╰── Constant Int [1]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <122>  [<]
                    │   │       ├── <119> Var [i]
                    │   │       ╰── Constant Int [2]
                    │   ├── Condition
                    │   │   ╰── <131> Assign [=]
                    │   │       ├── <124> Var [i]
                    │   │       ╰── <130>  [+]
                    │   │           ├── <127> Var [i]
                    │   │           ╰── Constant Int [1]
                    │   ╰── For
                    │       ├── Init
                    │       │   ╰── VarDeclaration
                    │       │       ├── Name
                    │       │       │   ╰── j
                    │       │       ├── Type
                    │       │       │   ╰── Int
                    │       │       ╰── Initializer
                    │       │           ╰── Constant Int [0]
                    │       ├── Condition
                    │       │   ╰── <143>  [<]
                    │       │       ├── <140> Var [j]
                    │       │       ╰── Constant Int [3]
                    │       ├── Condition
                    │       │   ╰── <152> Assign [=]
                    │       │       ├── <145> Var [j]
                    │       │       ╰── <151>  [+]
                    │       │           ├── <148> Var [j]
                    │       │           ╰── Constant Int [1]
                    │       ╰── If
                    │           ├── Condition
                    │           │   ╰── <173>  [!=]
                    │           │       ├── <160> Subscript
                    │           │       │   ├── <157> Subscript
                    │           │       │   │   ├── <154> Var [auto_array]
                    │           │       │   │   ╰── <156> Var [i]
                    │           │       │   ╰── <159> Var [j]
                    │           │       ╰── <172> Subscript
                    │           │           ├── <162> "lmnop"
                    │           │           ╰── <171>  [+]
                    │           │               ├── <167>  [*]
                    │           │               │   ├── <164> Var [i]
                    │           │               │   ╰── Constant Int [3]
                    │           │               ╰── <170> Var [j]
                    │           ╰── Then
                    │               ╰── Return
                    │                   ╰── Constant Int [2]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_strings_as_initializers_partial_initialize_via_string() {
    let src = r#"
        static char static_arr[5] = "hi";
        int test_static(void) {
            return (static_arr[0] == 'h' && static_arr[1] == 'i' &&
                    !(static_arr[2] || static_arr[3] || static_arr[4]));
        }
        static signed char nested_static_arr[3][4] = {
            "", "bc"};
        int test_static_nested(void) {
            for (int i = 0; i < 3; i = i + 1)
                for (int j = 0; j < 4; j = j + 1) {
                    signed char c = nested_static_arr[i][j];
                    signed char expected = 0;
                    if (i == 1 && j == 0) {
                        expected = 'b';
                    } else if (i == 1 && j == 1) {
                        expected = 'c';
                    }
                    if (c != expected) {
                        return 0;
                    }
                }
            return 1;
        }
        int test_automatic(void) {
            unsigned char aut[4] = "ab";
            return (aut[0] == 'a' && aut[1] == 'b' && !(aut[2] || aut[3]));
        }
        int test_automatic_nested(void) {
            signed char nested_auto[2][2][4] = {{"foo"}, {"x", "yz"}};
            for (int i = 0; i < 2; i = i + 1) {
                for (int j = 0; j < 2; j = j + 1) {
                    for (int k = 0; k < 4; k = k + 1) {
                        signed char c = nested_auto[i][j][k];
                        signed char expected = 0;
                        if (i == 0 && j == 0) {
                            if (k == 0) {
                                expected = 'f';
                            } else if (k == 1 || k == 2) {
                                expected = 'o';
                            }
                        } else if (i == 1 && j == 0 && k == 0) {
                            expected = 'x';
                        } else if (i == 1 && j == 1 && k == 0) {
                            expected = 'y';
                        } else if (i == 1 && j == 1 && k == 1) {
                            expected = 'z';
                        }
                        if (c != expected) {
                            return 0;
                        }
                    }
                }
            }
            return 1;
        }
        int main(void) {
            if (!test_static()) {
                return 1;
            }
            if (!test_static_nested()) {
                return 2;
            }
            if (!test_automatic()) {
                return 3;
            }
            if (!test_automatic_nested()) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── static_arr
            │   ├── Type
            │   │   ╰── Array
            │   │       ├── 5
            │   │       ╰── Char
            │   ├── Initializer
            │   │   ╰── <7> "hi"
            │   ╰── Static
            ├── Function [test_static]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <51>  [&&]
            │               ├── <29>  [&&]
            │               │   ├── <20>  [==]
            │               │   │   ├── <17> Subscript
            │               │   │   │   ├── <15> Var [static_arr]
            │               │   │   │   ╰── Constant Int [0]
            │               │   │   ╰── Constant Int [104]
            │               │   ╰── <28>  [==]
            │               │       ├── <25> Subscript
            │               │       │   ├── <23> Var [static_arr]
            │               │       │   ╰── Constant Int [1]
            │               │       ╰── Constant Int [105]
            │               ╰── <49> Unary [!]
            │                   ╰── <48>  [||]
            │                       ├── <41>  [||]
            │                       │   ├── <35> Subscript
            │                       │   │   ├── <33> Var [static_arr]
            │                       │   │   ╰── Constant Int [2]
            │                       │   ╰── <40> Subscript
            │                       │       ├── <38> Var [static_arr]
            │                       │       ╰── Constant Int [3]
            │                       ╰── <46> Subscript
            │                           ├── <44> Var [static_arr]
            │                           ╰── Constant Int [4]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── nested_static_arr
            │   ├── Type
            │   │   ╰── Array
            │   │       ├── 3
            │   │       ╰── Array
            │   │           ├── 4
            │   │           ╰── Signed Char
            │   ├── Initializer
            │   │   ╰── Compound
            │   │       ├── <63> ""
            │   │       ╰── <65> "bc"
            │   ╰── Static
            ├── Function [test_static_nested]
            │   ╰── Body
            │       ├── For
            │       │   ├── Init
            │       │   │   ╰── VarDeclaration
            │       │   │       ├── Name
            │       │   │       │   ╰── i
            │       │   │       ├── Type
            │       │   │       │   ╰── Int
            │       │   │       ╰── Initializer
            │       │   │           ╰── Constant Int [0]
            │       │   ├── Condition
            │       │   │   ╰── <84>  [<]
            │       │   │       ├── <81> Var [i]
            │       │   │       ╰── Constant Int [3]
            │       │   ├── Condition
            │       │   │   ╰── <93> Assign [=]
            │       │   │       ├── <86> Var [i]
            │       │   │       ╰── <92>  [+]
            │       │   │           ├── <89> Var [i]
            │       │   │           ╰── Constant Int [1]
            │       │   ╰── For
            │       │       ├── Init
            │       │       │   ╰── VarDeclaration
            │       │       │       ├── Name
            │       │       │       │   ╰── j
            │       │       │       ├── Type
            │       │       │       │   ╰── Int
            │       │       │       ╰── Initializer
            │       │       │           ╰── Constant Int [0]
            │       │       ├── Condition
            │       │       │   ╰── <105>  [<]
            │       │       │       ├── <102> Var [j]
            │       │       │       ╰── Constant Int [4]
            │       │       ├── Condition
            │       │       │   ╰── <114> Assign [=]
            │       │       │       ├── <107> Var [j]
            │       │       │       ╰── <113>  [+]
            │       │       │           ├── <110> Var [j]
            │       │       │           ╰── Constant Int [1]
            │       │       ╰── Block
            │       │           ├── VarDeclaration
            │       │           │   ├── Name
            │       │           │   │   ╰── c
            │       │           │   ├── Type
            │       │           │   │   ╰── Signed Char
            │       │           │   ╰── Initializer
            │       │           │       ╰── <125> Subscript
            │       │           │           ├── <122> Subscript
            │       │           │           │   ├── <119> Var [nested_static_arr]
            │       │           │           │   ╰── <121> Var [i]
            │       │           │           ╰── <124> Var [j]
            │       │           ├── VarDeclaration
            │       │           │   ├── Name
            │       │           │   │   ╰── expected
            │       │           │   ├── Type
            │       │           │   │   ╰── Signed Char
            │       │           │   ╰── Initializer
            │       │           │       ╰── Constant Int [0]
            │       │           ├── If
            │       │           │   ├── Condition
            │       │           │   │   ╰── <145>  [&&]
            │       │           │   │       ├── <138>  [==]
            │       │           │   │       │   ├── <135> Var [i]
            │       │           │   │       │   ╰── Constant Int [1]
            │       │           │   │       ╰── <144>  [==]
            │       │           │   │           ├── <141> Var [j]
            │       │           │   │           ╰── Constant Int [0]
            │       │           │   ├── Then
            │       │           │   │   ╰── Block
            │       │           │   │       ╰── <150> Assign [=]
            │       │           │   │           ├── <147> Var [expected]
            │       │           │   │           ╰── Constant Int [98]
            │       │           │   ╰── Else
            │       │           │       ╰── If
            │       │           │           ├── Condition
            │       │           │           │   ╰── <165>  [&&]
            │       │           │           │       ├── <158>  [==]
            │       │           │           │       │   ├── <155> Var [i]
            │       │           │           │       │   ╰── Constant Int [1]
            │       │           │           │       ╰── <164>  [==]
            │       │           │           │           ├── <161> Var [j]
            │       │           │           │           ╰── Constant Int [1]
            │       │           │           ╰── Then
            │       │           │               ╰── Block
            │       │           │                   ╰── <170> Assign [=]
            │       │           │                       ├── <167> Var [expected]
            │       │           │                       ╰── Constant Int [99]
            │       │           ╰── If
            │       │               ├── Condition
            │       │               │   ╰── <181>  [!=]
            │       │               │       ├── <177> Var [c]
            │       │               │       ╰── <180> Var [expected]
            │       │               ╰── Then
            │       │                   ╰── Block
            │       │                       ╰── Return
            │       │                           ╰── Constant Int [0]
            │       ╰── Return
            │           ╰── Constant Int [1]
            ├── Function [test_automatic]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── aut
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 4
            │       │   │       ╰── Unsigned Char
            │       │   ╰── Initializer
            │       │       ╰── <204> "ab"
            │       ╰── Return
            │           ╰── <238>  [&&]
            │               ├── <222>  [&&]
            │               │   ├── <213>  [==]
            │               │   │   ├── <210> Subscript
            │               │   │   │   ├── <208> Var [aut]
            │               │   │   │   ╰── Constant Int [0]
            │               │   │   ╰── Constant Int [97]
            │               │   ╰── <221>  [==]
            │               │       ├── <218> Subscript
            │               │       │   ├── <216> Var [aut]
            │               │       │   ╰── Constant Int [1]
            │               │       ╰── Constant Int [98]
            │               ╰── <236> Unary [!]
            │                   ╰── <235>  [||]
            │                       ├── <228> Subscript
            │                       │   ├── <226> Var [aut]
            │                       │   ╰── Constant Int [2]
            │                       ╰── <233> Subscript
            │                           ├── <231> Var [aut]
            │                           ╰── Constant Int [3]
            ├── Function [test_automatic_nested]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── nested_auto
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 2
            │       │   │       ╰── Array
            │       │   │           ├── 2
            │       │   │           ╰── Array
            │       │   │               ├── 4
            │       │   │               ╰── Signed Char
            │       │   ╰── Initializer
            │       │       ╰── Compound
            │       │           ├── Compound
            │       │           │   ╰── <255> "foo"
            │       │           ╰── Compound
            │       │               ├── <258> "x"
            │       │               ╰── <260> "yz"
            │       ├── For
            │       │   ├── Init
            │       │   │   ╰── VarDeclaration
            │       │   │       ├── Name
            │       │   │       │   ╰── i
            │       │   │       ├── Type
            │       │   │       │   ╰── Int
            │       │   │       ╰── Initializer
            │       │   │           ╰── Constant Int [0]
            │       │   ├── Condition
            │       │   │   ╰── <276>  [<]
            │       │   │       ├── <273> Var [i]
            │       │   │       ╰── Constant Int [2]
            │       │   ├── Condition
            │       │   │   ╰── <285> Assign [=]
            │       │   │       ├── <278> Var [i]
            │       │   │       ╰── <284>  [+]
            │       │   │           ├── <281> Var [i]
            │       │   │           ╰── Constant Int [1]
            │       │   ╰── Block
            │       │       ╰── For
            │       │           ├── Init
            │       │           │   ╰── VarDeclaration
            │       │           │       ├── Name
            │       │           │       │   ╰── j
            │       │           │       ├── Type
            │       │           │       │   ╰── Int
            │       │           │       ╰── Initializer
            │       │           │           ╰── Constant Int [0]
            │       │           ├── Condition
            │       │           │   ╰── <297>  [<]
            │       │           │       ├── <294> Var [j]
            │       │           │       ╰── Constant Int [2]
            │       │           ├── Condition
            │       │           │   ╰── <306> Assign [=]
            │       │           │       ├── <299> Var [j]
            │       │           │       ╰── <305>  [+]
            │       │           │           ├── <302> Var [j]
            │       │           │           ╰── Constant Int [1]
            │       │           ╰── Block
            │       │               ╰── For
            │       │                   ├── Init
            │       │                   │   ╰── VarDeclaration
            │       │                   │       ├── Name
            │       │                   │       │   ╰── k
            │       │                   │       ├── Type
            │       │                   │       │   ╰── Int
            │       │                   │       ╰── Initializer
            │       │                   │           ╰── Constant Int [0]
            │       │                   ├── Condition
            │       │                   │   ╰── <318>  [<]
            │       │                   │       ├── <315> Var [k]
            │       │                   │       ╰── Constant Int [4]
            │       │                   ├── Condition
            │       │                   │   ╰── <327> Assign [=]
            │       │                   │       ├── <320> Var [k]
            │       │                   │       ╰── <326>  [+]
            │       │                   │           ├── <323> Var [k]
            │       │                   │           ╰── Constant Int [1]
            │       │                   ╰── Block
            │       │                       ├── VarDeclaration
            │       │                       │   ├── Name
            │       │                       │   │   ╰── c
            │       │                       │   ├── Type
            │       │                       │   │   ╰── Signed Char
            │       │                       │   ╰── Initializer
            │       │                       │       ╰── <341> Subscript
            │       │                       │           ├── <338> Subscript
            │       │                       │           │   ├── <335> Subscript
            │       │                       │           │   │   ├── <332> Var [nested_auto]
            │       │                       │           │   │   ╰── <334> Var [i]
            │       │                       │           │   ╰── <337> Var [j]
            │       │                       │           ╰── <340> Var [k]
            │       │                       ├── VarDeclaration
            │       │                       │   ├── Name
            │       │                       │   │   ╰── expected
            │       │                       │   ├── Type
            │       │                       │   │   ╰── Signed Char
            │       │                       │   ╰── Initializer
            │       │                       │       ╰── Constant Int [0]
            │       │                       ├── If
            │       │                       │   ├── Condition
            │       │                       │   │   ╰── <361>  [&&]
            │       │                       │   │       ├── <354>  [==]
            │       │                       │   │       │   ├── <351> Var [i]
            │       │                       │   │       │   ╰── Constant Int [0]
            │       │                       │   │       ╰── <360>  [==]
            │       │                       │   │           ├── <357> Var [j]
            │       │                       │   │           ╰── Constant Int [0]
            │       │                       │   ├── Then
            │       │                       │   │   ╰── Block
            │       │                       │   │       ╰── If
            │       │                       │   │           ├── Condition
            │       │                       │   │           │   ╰── <366>  [==]
            │       │                       │   │           │       ├── <363> Var [k]
            │       │                       │   │           │       ╰── Constant Int [0]
            │       │                       │   │           ├── Then
            │       │                       │   │           │   ╰── Block
            │       │                       │   │           │       ╰── <371> Assign [=]
            │       │                       │   │           │           ├── <368> Var [expected]
            │       │                       │   │           │           ╰── Constant Int [102]
            │       │                       │   │           ╰── Else
            │       │                       │   │               ╰── If
            │       │                       │   │                   ├── Condition
            │       │                       │   │                   │   ╰── <386>  [||]
            │       │                       │   │                   │       ├── <379>  [==]
            │       │                       │   │                   │       │   ├── <376> Var [k]
            │       │                       │   │                   │       │   ╰── Constant Int [1]
            │       │                       │   │                   │       ╰── <385>  [==]
            │       │                       │   │                   │           ├── <382> Var [k]
            │       │                       │   │                   │           ╰── Constant Int [2]
            │       │                       │   │                   ╰── Then
            │       │                       │   │                       ╰── Block
            │       │                       │   │                           ╰── <391> Assign [=]
            │       │                       │   │                               ├── <388> Var [expected]
            │       │                       │   │                               ╰── Constant Int [111]
            │       │                       │   ╰── Else
            │       │                       │       ╰── If
            │       │                       │           ├── Condition
            │       │                       │           │   ╰── <417>  [&&]
            │       │                       │           │       ├── <410>  [&&]
            │       │                       │           │       │   ├── <403>  [==]
            │       │                       │           │       │   │   ├── <400> Var [i]
            │       │                       │           │       │   │   ╰── Constant Int [1]
            │       │                       │           │       │   ╰── <409>  [==]
            │       │                       │           │       │       ├── <406> Var [j]
            │       │                       │           │       │       ╰── Constant Int [0]
            │       │                       │           │       ╰── <416>  [==]
            │       │                       │           │           ├── <413> Var [k]
            │       │                       │           │           ╰── Constant Int [0]
            │       │                       │           ├── Then
            │       │                       │           │   ╰── Block
            │       │                       │           │       ╰── <422> Assign [=]
            │       │                       │           │           ├── <419> Var [expected]
            │       │                       │           │           ╰── Constant Int [120]
            │       │                       │           ╰── Else
            │       │                       │               ╰── If
            │       │                       │                   ├── Condition
            │       │                       │                   │   ╰── <444>  [&&]
            │       │                       │                   │       ├── <437>  [&&]
            │       │                       │                   │       │   ├── <430>  [==]
            │       │                       │                   │       │   │   ├── <427> Var [i]
            │       │                       │                   │       │   │   ╰── Constant Int [1]
            │       │                       │                   │       │   ╰── <436>  [==]
            │       │                       │                   │       │       ├── <433> Var [j]
            │       │                       │                   │       │       ╰── Constant Int [1]
            │       │                       │                   │       ╰── <443>  [==]
            │       │                       │                   │           ├── <440> Var [k]
            │       │                       │                   │           ╰── Constant Int [0]
            │       │                       │                   ├── Then
            │       │                       │                   │   ╰── Block
            │       │                       │                   │       ╰── <449> Assign [=]
            │       │                       │                   │           ├── <446> Var [expected]
            │       │                       │                   │           ╰── Constant Int [121]
            │       │                       │                   ╰── Else
            │       │                       │                       ╰── If
            │       │                       │                           ├── Condition
            │       │                       │                           │   ╰── <471>  [&&]
            │       │                       │                           │       ├── <464>  [&&]
            │       │                       │                           │       │   ├── <457>  [==]
            │       │                       │                           │       │   │   ├── <454> Var [i]
            │       │                       │                           │       │   │   ╰── Constant Int [1]
            │       │                       │                           │       │   ╰── <463>  [==]
            │       │                       │                           │       │       ├── <460> Var [j]
            │       │                       │                           │       │       ╰── Constant Int [1]
            │       │                       │                           │       ╰── <470>  [==]
            │       │                       │                           │           ├── <467> Var [k]
            │       │                       │                           │           ╰── Constant Int [1]
            │       │                       │                           ╰── Then
            │       │                       │                               ╰── Block
            │       │                       │                                   ╰── <476> Assign [=]
            │       │                       │                                       ├── <473> Var [expected]
            │       │                       │                                       ╰── Constant Int [122]
            │       │                       ╰── If
            │       │                           ├── Condition
            │       │                           │   ╰── <489>  [!=]
            │       │                           │       ├── <485> Var [c]
            │       │                           │       ╰── <488> Var [expected]
            │       │                           ╰── Then
            │       │                               ╰── Block
            │       │                                   ╰── Return
            │       │                                       ╰── Constant Int [0]
            │       ╰── Return
            │           ╰── Constant Int [1]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <515> Unary [!]
                    │   │       ╰── <514> FunctionCall [test_static]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <524> Unary [!]
                    │   │       ╰── <523> FunctionCall [test_static_nested]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <533> Unary [!]
                    │   │       ╰── <532> FunctionCall [test_automatic]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <542> Unary [!]
                    │   │       ╰── <541> FunctionCall [test_automatic_nested]
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
fn test_valid_strings_as_initializers_simple() {
    let src = r#"
        int main(void) {
            unsigned char chars[4] = "abc";
            return chars[2];
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── chars
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 4
                    │   │       ╰── Unsigned Char
                    │   ╰── Initializer
                    │       ╰── <10> "abc"
                    ╰── Return
                        ╰── <16> Subscript
                            ├── <14> Var [chars]
                            ╰── Constant Int [2]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_strings_as_initializers_terminating_null_bytes() {
    let src = r#"
        int strcmp(char *s1, char *s2);
        int test_flat_static_with_null_byte(void) {
            static unsigned char flat[4] = "dog";
            return (flat[0] == 'd' && flat[1] == 'o' && flat[2] == 'g' && flat[3] == 0);
        }
        int test_nested_static_with_null_byte(void) {
            static char nested[2][4] = {"yes", "yup"};
            return (nested[0][0] == 'y' && nested[0][1] == 'e' && nested[0][2] == 's' &&
                    nested[0][3] == 0 && nested[1][0] == 'y' && nested[1][1] == 'u' &&
                    nested[1][2] == 'p' && nested[1][3] == 0);
        }
        int test_flat_auto_with_null_byte(void) {
            char flat_auto[2] = "x";
            return (flat_auto[0] == 'x' && flat_auto[1] == 0);
        }
        int test_nested_auto_with_null_byte(void) {
            char nested_auto[2][2][2] = {{"a", "b"}, {"c", "d"}};
            return (nested_auto[0][0][0] == 'a' && nested_auto[0][0][1] == 0 &&
                    nested_auto[0][1][0] == 'b' && nested_auto[0][1][1] == 0 &&
                    nested_auto[1][0][0] == 'c' && nested_auto[1][0][1] == 0 &&
                    nested_auto[1][1][0] == 'd' && nested_auto[1][1][1] == 0);
        }
        int test_flat_static_without_null_byte(void) {
            static char letters[4] = "abcd";
            return letters[0] == 'a' && letters[1] == 'b' && letters[2] == 'c' &&
                   letters[3] == 'd';
        }
        char nested[3][3] = {"yes", "no", "ok"};
        int test_nested_static_without_null_byte(void) {
            char *whole_array = (char *)nested;
            char *word1 = (char *)nested[0];
            char *word2 = (char *)nested[1];
            char *word3 = (char *)nested[2];
            return !(strcmp(whole_array, "yesno") || strcmp(word1, "yesno") ||
                     strcmp(word2, "no") || strcmp(word3, "ok"));
        }
        int test_flat_auto_without_null_byte(void) {
            int x = -1;
            char letters[4] = "abcd";
            int y = -1;
            return (x == -1 && y == -1 && letters[0] == 'a' && letters[1] == 'b' &&
                    letters[2] == 'c' && letters[3] == 'd');
        }
        int test_nested_auto_without_null_byte(void) {
            char nested[3][3] = {"yes", "no", "ok"};
            char *whole_array = (char *)nested;
            char *word1 = (char *)nested[0];
            char *word2 = (char *)nested[1];
            char *word3 = (char *)nested[2];
            return !(strcmp(whole_array, "yesno") || strcmp(word1, "yesno") ||
                     strcmp(word2, "no") || strcmp(word3, "ok"));
        }
        int main(void) {
            if (!test_flat_static_with_null_byte()) {
                return 1;
            }
            if (!test_nested_static_with_null_byte()) {
                return 2;
            }
            if (!test_flat_auto_with_null_byte()) {
                return 3;
            }
            if (!test_nested_auto_with_null_byte()) {
                return 4;
            }
            if (!test_flat_static_without_null_byte()) {
                return 5;
            }
            if (!test_nested_static_without_null_byte()) {
                return 6;
            }
            if (!test_flat_auto_without_null_byte()) {
                return 7;
            }
            if (!test_nested_auto_without_null_byte()) {
                return 8;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [strcmp]
            │   ╰── Parameters
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── s1
            │       │   ╰── Type
            │       │       ╰── Pointer
            │       │           ╰── Char
            │       ╰── Param
            │           ├── Name
            │           │   ╰── s2
            │           ╰── Type
            │               ╰── Pointer
            │                   ╰── Char
            ├── Function [test_flat_static_with_null_byte]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── flat
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 4
            │       │   │       ╰── Unsigned Char
            │       │   ├── Initializer
            │       │   │   ╰── <24> "dog"
            │       │   ╰── Static
            │       ╰── Return
            │           ╰── <61>  [&&]
            │               ├── <51>  [&&]
            │               │   ├── <42>  [&&]
            │               │   │   ├── <33>  [==]
            │               │   │   │   ├── <30> Subscript
            │               │   │   │   │   ├── <28> Var [flat]
            │               │   │   │   │   ╰── Constant Int [0]
            │               │   │   │   ╰── Constant Int [100]
            │               │   │   ╰── <41>  [==]
            │               │   │       ├── <38> Subscript
            │               │   │       │   ├── <36> Var [flat]
            │               │   │       │   ╰── Constant Int [1]
            │               │   │       ╰── Constant Int [111]
            │               │   ╰── <50>  [==]
            │               │       ├── <47> Subscript
            │               │       │   ├── <45> Var [flat]
            │               │       │   ╰── Constant Int [2]
            │               │       ╰── Constant Int [103]
            │               ╰── <59>  [==]
            │                   ├── <56> Subscript
            │                   │   ├── <54> Var [flat]
            │                   │   ╰── Constant Int [3]
            │                   ╰── Constant Int [0]
            ├── Function [test_nested_static_with_null_byte]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── nested
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 2
            │       │   │       ╰── Array
            │       │   │           ├── 4
            │       │   │           ╰── Char
            │       │   ├── Initializer
            │       │   │   ╰── Compound
            │       │   │       ├── <77> "yes"
            │       │   │       ╰── <79> "yup"
            │       │   ╰── Static
            │       ╰── Return
            │           ╰── <169>  [&&]
            │               ├── <157>  [&&]
            │               │   ├── <146>  [&&]
            │               │   │   ├── <135>  [&&]
            │               │   │   │   ├── <124>  [&&]
            │               │   │   │   │   ├── <113>  [&&]
            │               │   │   │   │   │   ├── <102>  [&&]
            │               │   │   │   │   │   │   ├── <91>  [==]
            │               │   │   │   │   │   │   │   ├── <88> Subscript
            │               │   │   │   │   │   │   │   │   ├── <86> Subscript
            │               │   │   │   │   │   │   │   │   │   ├── <84> Var [nested]
            │               │   │   │   │   │   │   │   │   │   ╰── Constant Int [0]
            │               │   │   │   │   │   │   │   │   ╰── Constant Int [0]
            │               │   │   │   │   │   │   │   ╰── Constant Int [121]
            │               │   │   │   │   │   │   ╰── <101>  [==]
            │               │   │   │   │   │   │       ├── <98> Subscript
            │               │   │   │   │   │   │       │   ├── <96> Subscript
            │               │   │   │   │   │   │       │   │   ├── <94> Var [nested]
            │               │   │   │   │   │   │       │   │   ╰── Constant Int [0]
            │               │   │   │   │   │   │       │   ╰── Constant Int [1]
            │               │   │   │   │   │   │       ╰── Constant Int [101]
            │               │   │   │   │   │   ╰── <112>  [==]
            │               │   │   │   │   │       ├── <109> Subscript
            │               │   │   │   │   │       │   ├── <107> Subscript
            │               │   │   │   │   │       │   │   ├── <105> Var [nested]
            │               │   │   │   │   │       │   │   ╰── Constant Int [0]
            │               │   │   │   │   │       │   ╰── Constant Int [2]
            │               │   │   │   │   │       ╰── Constant Int [115]
            │               │   │   │   │   ╰── <123>  [==]
            │               │   │   │   │       ├── <120> Subscript
            │               │   │   │   │       │   ├── <118> Subscript
            │               │   │   │   │       │   │   ├── <116> Var [nested]
            │               │   │   │   │       │   │   ╰── Constant Int [0]
            │               │   │   │   │       │   ╰── Constant Int [3]
            │               │   │   │   │       ╰── Constant Int [0]
            │               │   │   │   ╰── <134>  [==]
            │               │   │   │       ├── <131> Subscript
            │               │   │   │       │   ├── <129> Subscript
            │               │   │   │       │   │   ├── <127> Var [nested]
            │               │   │   │       │   │   ╰── Constant Int [1]
            │               │   │   │       │   ╰── Constant Int [0]
            │               │   │   │       ╰── Constant Int [121]
            │               │   │   ╰── <145>  [==]
            │               │   │       ├── <142> Subscript
            │               │   │       │   ├── <140> Subscript
            │               │   │       │   │   ├── <138> Var [nested]
            │               │   │       │   │   ╰── Constant Int [1]
            │               │   │       │   ╰── Constant Int [1]
            │               │   │       ╰── Constant Int [117]
            │               │   ╰── <156>  [==]
            │               │       ├── <153> Subscript
            │               │       │   ├── <151> Subscript
            │               │       │   │   ├── <149> Var [nested]
            │               │       │   │   ╰── Constant Int [1]
            │               │       │   ╰── Constant Int [2]
            │               │       ╰── Constant Int [112]
            │               ╰── <167>  [==]
            │                   ├── <164> Subscript
            │                   │   ├── <162> Subscript
            │                   │   │   ├── <160> Var [nested]
            │                   │   │   ╰── Constant Int [1]
            │                   │   ╰── Constant Int [3]
            │                   ╰── Constant Int [0]
            ├── Function [test_flat_auto_with_null_byte]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── flat_auto
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 2
            │       │   │       ╰── Char
            │       │   ╰── Initializer
            │       │       ╰── <182> "x"
            │       ╰── Return
            │           ╰── <201>  [&&]
            │               ├── <191>  [==]
            │               │   ├── <188> Subscript
            │               │   │   ├── <186> Var [flat_auto]
            │               │   │   ╰── Constant Int [0]
            │               │   ╰── Constant Int [120]
            │               ╰── <199>  [==]
            │                   ├── <196> Subscript
            │                   │   ├── <194> Var [flat_auto]
            │                   │   ╰── Constant Int [1]
            │                   ╰── Constant Int [0]
            ├── Function [test_nested_auto_with_null_byte]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── nested_auto
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 2
            │       │   │       ╰── Array
            │       │   │           ├── 2
            │       │   │           ╰── Array
            │       │   │               ├── 2
            │       │   │               ╰── Char
            │       │   ╰── Initializer
            │       │       ╰── Compound
            │       │           ├── Compound
            │       │           │   ├── <218> "a"
            │       │           │   ╰── <220> "b"
            │       │           ╰── Compound
            │       │               ├── <223> "c"
            │       │               ╰── <225> "d"
            │       ╰── Return
            │           ╰── <332>  [&&]
            │               ├── <318>  [&&]
            │               │   ├── <305>  [&&]
            │               │   │   ├── <292>  [&&]
            │               │   │   │   ├── <279>  [&&]
            │               │   │   │   │   ├── <266>  [&&]
            │               │   │   │   │   │   ├── <253>  [&&]
            │               │   │   │   │   │   │   ├── <240>  [==]
            │               │   │   │   │   │   │   │   ├── <237> Subscript
            │               │   │   │   │   │   │   │   │   ├── <235> Subscript
            │               │   │   │   │   │   │   │   │   │   ├── <233> Subscript
            │               │   │   │   │   │   │   │   │   │   │   ├── <231> Var [nested_auto]
            │               │   │   │   │   │   │   │   │   │   │   ╰── Constant Int [0]
            │               │   │   │   │   │   │   │   │   │   ╰── Constant Int [0]
            │               │   │   │   │   │   │   │   │   ╰── Constant Int [0]
            │               │   │   │   │   │   │   │   ╰── Constant Int [97]
            │               │   │   │   │   │   │   ╰── <252>  [==]
            │               │   │   │   │   │   │       ├── <249> Subscript
            │               │   │   │   │   │   │       │   ├── <247> Subscript
            │               │   │   │   │   │   │       │   │   ├── <245> Subscript
            │               │   │   │   │   │   │       │   │   │   ├── <243> Var [nested_auto]
            │               │   │   │   │   │   │       │   │   │   ╰── Constant Int [0]
            │               │   │   │   │   │   │       │   │   ╰── Constant Int [0]
            │               │   │   │   │   │   │       │   ╰── Constant Int [1]
            │               │   │   │   │   │   │       ╰── Constant Int [0]
            │               │   │   │   │   │   ╰── <265>  [==]
            │               │   │   │   │   │       ├── <262> Subscript
            │               │   │   │   │   │       │   ├── <260> Subscript
            │               │   │   │   │   │       │   │   ├── <258> Subscript
            │               │   │   │   │   │       │   │   │   ├── <256> Var [nested_auto]
            │               │   │   │   │   │       │   │   │   ╰── Constant Int [0]
            │               │   │   │   │   │       │   │   ╰── Constant Int [1]
            │               │   │   │   │   │       │   ╰── Constant Int [0]
            │               │   │   │   │   │       ╰── Constant Int [98]
            │               │   │   │   │   ╰── <278>  [==]
            │               │   │   │   │       ├── <275> Subscript
            │               │   │   │   │       │   ├── <273> Subscript
            │               │   │   │   │       │   │   ├── <271> Subscript
            │               │   │   │   │       │   │   │   ├── <269> Var [nested_auto]
            │               │   │   │   │       │   │   │   ╰── Constant Int [0]
            │               │   │   │   │       │   │   ╰── Constant Int [1]
            │               │   │   │   │       │   ╰── Constant Int [1]
            │               │   │   │   │       ╰── Constant Int [0]
            │               │   │   │   ╰── <291>  [==]
            │               │   │   │       ├── <288> Subscript
            │               │   │   │       │   ├── <286> Subscript
            │               │   │   │       │   │   ├── <284> Subscript
            │               │   │   │       │   │   │   ├── <282> Var [nested_auto]
            │               │   │   │       │   │   │   ╰── Constant Int [1]
            │               │   │   │       │   │   ╰── Constant Int [0]
            │               │   │   │       │   ╰── Constant Int [0]
            │               │   │   │       ╰── Constant Int [99]
            │               │   │   ╰── <304>  [==]
            │               │   │       ├── <301> Subscript
            │               │   │       │   ├── <299> Subscript
            │               │   │       │   │   ├── <297> Subscript
            │               │   │       │   │   │   ├── <295> Var [nested_auto]
            │               │   │       │   │   │   ╰── Constant Int [1]
            │               │   │       │   │   ╰── Constant Int [0]
            │               │   │       │   ╰── Constant Int [1]
            │               │   │       ╰── Constant Int [0]
            │               │   ╰── <317>  [==]
            │               │       ├── <314> Subscript
            │               │       │   ├── <312> Subscript
            │               │       │   │   ├── <310> Subscript
            │               │       │   │   │   ├── <308> Var [nested_auto]
            │               │       │   │   │   ╰── Constant Int [1]
            │               │       │   │   ╰── Constant Int [1]
            │               │       │   ╰── Constant Int [0]
            │               │       ╰── Constant Int [100]
            │               ╰── <330>  [==]
            │                   ├── <327> Subscript
            │                   │   ├── <325> Subscript
            │                   │   │   ├── <323> Subscript
            │                   │   │   │   ├── <321> Var [nested_auto]
            │                   │   │   │   ╰── Constant Int [1]
            │                   │   │   ╰── Constant Int [1]
            │                   │   ╰── Constant Int [1]
            │                   ╰── Constant Int [0]
            ├── Function [test_flat_static_without_null_byte]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── letters
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 4
            │       │   │       ╰── Char
            │       │   ├── Initializer
            │       │   │   ╰── <346> "abcd"
            │       │   ╰── Static
            │       ╰── Return
            │           ╰── <382>  [&&]
            │               ├── <373>  [&&]
            │               │   ├── <364>  [&&]
            │               │   │   ├── <355>  [==]
            │               │   │   │   ├── <352> Subscript
            │               │   │   │   │   ├── <350> Var [letters]
            │               │   │   │   │   ╰── Constant Int [0]
            │               │   │   │   ╰── Constant Int [97]
            │               │   │   ╰── <363>  [==]
            │               │   │       ├── <360> Subscript
            │               │   │       │   ├── <358> Var [letters]
            │               │   │       │   ╰── Constant Int [1]
            │               │   │       ╰── Constant Int [98]
            │               │   ╰── <372>  [==]
            │               │       ├── <369> Subscript
            │               │       │   ├── <367> Var [letters]
            │               │       │   ╰── Constant Int [2]
            │               │       ╰── Constant Int [99]
            │               ╰── <381>  [==]
            │                   ├── <378> Subscript
            │                   │   ├── <376> Var [letters]
            │                   │   ╰── Constant Int [3]
            │                   ╰── Constant Int [100]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── nested
            │   ├── Type
            │   │   ╰── Array
            │   │       ├── 3
            │   │       ╰── Array
            │   │           ├── 3
            │   │           ╰── Char
            │   ╰── Initializer
            │       ╰── Compound
            │           ├── <393> "yes"
            │           ├── <395> "no"
            │           ╰── <397> "ok"
            ├── Function [test_nested_static_without_null_byte]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── whole_array
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Char
            │       │   ╰── Initializer
            │       │       ╰── <414> Cast
            │       │           ├── Target
            │       │           │   ╰── Pointer
            │       │           │       ╰── Char
            │       │           ╰── Expression
            │       │               ╰── <413> Var [nested]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── word1
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Char
            │       │   ╰── Initializer
            │       │       ╰── <428> Cast
            │       │           ├── Target
            │       │           │   ╰── Pointer
            │       │           │       ╰── Char
            │       │           ╰── Expression
            │       │               ╰── <427> Subscript
            │       │                   ├── <425> Var [nested]
            │       │                   ╰── Constant Int [0]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── word2
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Char
            │       │   ╰── Initializer
            │       │       ╰── <442> Cast
            │       │           ├── Target
            │       │           │   ╰── Pointer
            │       │           │       ╰── Char
            │       │           ╰── Expression
            │       │               ╰── <441> Subscript
            │       │                   ├── <439> Var [nested]
            │       │                   ╰── Constant Int [1]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── word3
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Char
            │       │   ╰── Initializer
            │       │       ╰── <456> Cast
            │       │           ├── Target
            │       │           │   ╰── Pointer
            │       │           │       ╰── Char
            │       │           ╰── Expression
            │       │               ╰── <455> Subscript
            │       │                   ├── <453> Var [nested]
            │       │                   ╰── Constant Int [2]
            │       ╰── Return
            │           ╰── <487> Unary [!]
            │               ╰── <486>  [||]
            │                   ├── <478>  [||]
            │                   │   ├── <471>  [||]
            │                   │   │   ├── <464> FunctionCall [strcmp]
            │                   │   │   │   ├── <462> Var [whole_array]
            │                   │   │   │   ╰── <463> "yesno"
            │                   │   │   ╰── <470> FunctionCall [strcmp]
            │                   │   │       ├── <468> Var [word1]
            │                   │   │       ╰── <469> "yesno"
            │                   │   ╰── <477> FunctionCall [strcmp]
            │                   │       ├── <475> Var [word2]
            │                   │       ╰── <476> "no"
            │                   ╰── <484> FunctionCall [strcmp]
            │                       ├── <482> Var [word3]
            │                       ╰── <483> "ok"
            ├── Function [test_flat_auto_without_null_byte]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── x
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <500> Unary [-]
            │       │           ╰── Constant Int [1]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── letters
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 4
            │       │   │       ╰── Char
            │       │   ╰── Initializer
            │       │       ╰── <508> "abcd"
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── y
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <516> Unary [-]
            │       │           ╰── Constant Int [1]
            │       ╰── Return
            │           ╰── <571>  [&&]
            │               ├── <561>  [&&]
            │               │   ├── <552>  [&&]
            │               │   │   ├── <543>  [&&]
            │               │   │   │   ├── <534>  [&&]
            │               │   │   │   │   ├── <525>  [==]
            │               │   │   │   │   │   ├── <520> Var [x]
            │               │   │   │   │   │   ╰── <524> Unary [-]
            │               │   │   │   │   │       ╰── Constant Int [1]
            │               │   │   │   │   ╰── <533>  [==]
            │               │   │   │   │       ├── <528> Var [y]
            │               │   │   │   │       ╰── <532> Unary [-]
            │               │   │   │   │           ╰── Constant Int [1]
            │               │   │   │   ╰── <542>  [==]
            │               │   │   │       ├── <539> Subscript
            │               │   │   │       │   ├── <537> Var [letters]
            │               │   │   │       │   ╰── Constant Int [0]
            │               │   │   │       ╰── Constant Int [97]
            │               │   │   ╰── <551>  [==]
            │               │   │       ├── <548> Subscript
            │               │   │       │   ├── <546> Var [letters]
            │               │   │       │   ╰── Constant Int [1]
            │               │   │       ╰── Constant Int [98]
            │               │   ╰── <560>  [==]
            │               │       ├── <557> Subscript
            │               │       │   ├── <555> Var [letters]
            │               │       │   ╰── Constant Int [2]
            │               │       ╰── Constant Int [99]
            │               ╰── <569>  [==]
            │                   ├── <566> Subscript
            │                   │   ├── <564> Var [letters]
            │                   │   ╰── Constant Int [3]
            │                   ╰── Constant Int [100]
            ├── Function [test_nested_auto_without_null_byte]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── nested
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 3
            │       │   │       ╰── Array
            │       │   │           ├── 3
            │       │   │           ╰── Char
            │       │   ╰── Initializer
            │       │       ╰── Compound
            │       │           ├── <586> "yes"
            │       │           ├── <588> "no"
            │       │           ╰── <590> "ok"
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── whole_array
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Char
            │       │   ╰── Initializer
            │       │       ╰── <603> Cast
            │       │           ├── Target
            │       │           │   ╰── Pointer
            │       │           │       ╰── Char
            │       │           ╰── Expression
            │       │               ╰── <602> Var [nested]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── word1
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Char
            │       │   ╰── Initializer
            │       │       ╰── <617> Cast
            │       │           ├── Target
            │       │           │   ╰── Pointer
            │       │           │       ╰── Char
            │       │           ╰── Expression
            │       │               ╰── <616> Subscript
            │       │                   ├── <614> Var [nested]
            │       │                   ╰── Constant Int [0]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── word2
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Char
            │       │   ╰── Initializer
            │       │       ╰── <631> Cast
            │       │           ├── Target
            │       │           │   ╰── Pointer
            │       │           │       ╰── Char
            │       │           ╰── Expression
            │       │               ╰── <630> Subscript
            │       │                   ├── <628> Var [nested]
            │       │                   ╰── Constant Int [1]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── word3
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Char
            │       │   ╰── Initializer
            │       │       ╰── <645> Cast
            │       │           ├── Target
            │       │           │   ╰── Pointer
            │       │           │       ╰── Char
            │       │           ╰── Expression
            │       │               ╰── <644> Subscript
            │       │                   ├── <642> Var [nested]
            │       │                   ╰── Constant Int [2]
            │       ╰── Return
            │           ╰── <676> Unary [!]
            │               ╰── <675>  [||]
            │                   ├── <667>  [||]
            │                   │   ├── <660>  [||]
            │                   │   │   ├── <653> FunctionCall [strcmp]
            │                   │   │   │   ├── <651> Var [whole_array]
            │                   │   │   │   ╰── <652> "yesno"
            │                   │   │   ╰── <659> FunctionCall [strcmp]
            │                   │   │       ├── <657> Var [word1]
            │                   │   │       ╰── <658> "yesno"
            │                   │   ╰── <666> FunctionCall [strcmp]
            │                   │       ├── <664> Var [word2]
            │                   │       ╰── <665> "no"
            │                   ╰── <673> FunctionCall [strcmp]
            │                       ├── <671> Var [word3]
            │                       ╰── <672> "ok"
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <687> Unary [!]
                    │   │       ╰── <686> FunctionCall [test_flat_static_with_null_byte]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <696> Unary [!]
                    │   │       ╰── <695> FunctionCall [test_nested_static_with_null_byte]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <705> Unary [!]
                    │   │       ╰── <704> FunctionCall [test_flat_auto_with_null_byte]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <714> Unary [!]
                    │   │       ╰── <713> FunctionCall [test_nested_auto_with_null_byte]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <723> Unary [!]
                    │   │       ╰── <722> FunctionCall [test_flat_static_without_null_byte]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <732> Unary [!]
                    │   │       ╰── <731> FunctionCall [test_nested_static_without_null_byte]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <741> Unary [!]
                    │   │       ╰── <740> FunctionCall [test_flat_auto_without_null_byte]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <750> Unary [!]
                    │   │       ╰── <749> FunctionCall [test_nested_auto_without_null_byte]
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
fn test_valid_strings_as_initializers_test_alignment() {
    let src = r#"
        int check_aligment(char *c) {
            unsigned long l = (unsigned long)c;
            return (l % 16 == 0);
        }
        static signed char flat_static[16] = "x";
        static unsigned char nested_static[3][4][2] = {{"a"}, {"b"}};
        int main(void) {
            char flat_auto[22];
            char nested_auto[10][3];
            if (!check_aligment((char *)flat_static)) {
                return 1;
            }
            if (!check_aligment((char *)nested_static)) {
                return 2;
            }
            if (!check_aligment((char *)flat_auto)) {
                return 3;
            }
            if (!check_aligment((char *)nested_auto)) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [check_aligment]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── c
            │   │       ╰── Type
            │   │           ╰── Pointer
            │   │               ╰── Char
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── l
            │       │   ├── Type
            │       │   │   ╰── Unsigned Long
            │       │   ╰── Initializer
            │       │       ╰── <16> Cast
            │       │           ├── Target
            │       │           │   ╰── Unsigned Long
            │       │           ╰── Expression
            │       │               ╰── <15> Var [c]
            │       ╰── Return
            │           ╰── <27>  [==]
            │               ├── <23>  [%]
            │               │   ├── <20> Var [l]
            │               │   ╰── Constant Int [16]
            │               ╰── Constant Int [0]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── flat_static
            │   ├── Type
            │   │   ╰── Array
            │   │       ├── 16
            │   │       ╰── Signed Char
            │   ├── Initializer
            │   │   ╰── <37> "x"
            │   ╰── Static
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── nested_static
            │   ├── Type
            │   │   ╰── Array
            │   │       ├── 3
            │   │       ╰── Array
            │   │           ├── 4
            │   │           ╰── Array
            │   │               ├── 2
            │   │               ╰── Unsigned Char
            │   ├── Initializer
            │   │   ╰── Compound
            │   │       ├── Compound
            │   │       │   ╰── <50> "a"
            │   │       ╰── Compound
            │   │           ╰── <53> "b"
            │   ╰── Static
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── flat_auto
                    │   ╰── Type
                    │       ╰── Array
                    │           ├── 22
                    │           ╰── Char
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── nested_auto
                    │   ╰── Type
                    │       ╰── Array
                    │           ├── 10
                    │           ╰── Array
                    │               ├── 3
                    │               ╰── Char
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <85> Unary [!]
                    │   │       ╰── <84> FunctionCall [check_aligment]
                    │   │           ╰── <83> Cast
                    │   │               ├── Target
                    │   │               │   ╰── Pointer
                    │   │               │       ╰── Char
                    │   │               ╰── Expression
                    │   │                   ╰── <82> Var [flat_static]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <100> Unary [!]
                    │   │       ╰── <99> FunctionCall [check_aligment]
                    │   │           ╰── <98> Cast
                    │   │               ├── Target
                    │   │               │   ╰── Pointer
                    │   │               │       ╰── Char
                    │   │               ╰── Expression
                    │   │                   ╰── <97> Var [nested_static]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <115> Unary [!]
                    │   │       ╰── <114> FunctionCall [check_aligment]
                    │   │           ╰── <113> Cast
                    │   │               ├── Target
                    │   │               │   ╰── Pointer
                    │   │               │       ╰── Char
                    │   │               ╰── Expression
                    │   │                   ╰── <112> Var [flat_auto]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <130> Unary [!]
                    │   │       ╰── <129> FunctionCall [check_aligment]
                    │   │           ╰── <128> Cast
                    │   │               ├── Target
                    │   │               │   ╰── Pointer
                    │   │               │       ╰── Char
                    │   │               ╰── Expression
                    │   │                   ╰── <127> Var [nested_auto]
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
fn test_valid_strings_as_initializers_transfer_by_eightbyte() {
    let src = r#"
        int strcmp(char *s1, char *s2);
        int main(void) {
            char strings[2][13] = {"abcdefghijkl", "z"};
            if (strcmp(strings[0], "abcdefghijkl"))
                return 1;
            if (strings[1][0] != 'z')
                return 2;
            for (int i = 1; i < 13; i = i + 1) {
                if (strings[1][i])
                    return 3;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [strcmp]
            │   ╰── Parameters
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── s1
            │       │   ╰── Type
            │       │       ╰── Pointer
            │       │           ╰── Char
            │       ╰── Param
            │           ├── Name
            │           │   ╰── s2
            │           ╰── Type
            │               ╰── Pointer
            │                   ╰── Char
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── strings
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 2
                    │   │       ╰── Array
                    │   │           ├── 13
                    │   │           ╰── Char
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <25> "abcdefghijkl"
                    │           ╰── <27> "z"
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <37> FunctionCall [strcmp]
                    │   │       ├── <35> Subscript
                    │   │       │   ├── <33> Var [strings]
                    │   │       │   ╰── Constant Int [0]
                    │   │       ╰── <36> "abcdefghijkl"
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <49>  [!=]
                    │   │       ├── <46> Subscript
                    │   │       │   ├── <44> Subscript
                    │   │       │   │   ├── <42> Var [strings]
                    │   │       │   │   ╰── Constant Int [1]
                    │   │       │   ╰── Constant Int [0]
                    │   │       ╰── Constant Int [122]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [2]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── Constant Int [1]
                    │   ├── Condition
                    │   │   ╰── <64>  [<]
                    │   │       ├── <61> Var [i]
                    │   │       ╰── Constant Int [13]
                    │   ├── Condition
                    │   │   ╰── <73> Assign [=]
                    │   │       ├── <66> Var [i]
                    │   │       ╰── <72>  [+]
                    │   │           ├── <69> Var [i]
                    │   │           ╰── Constant Int [1]
                    │   ╰── Block
                    │       ╰── If
                    │           ├── Condition
                    │           │   ╰── <80> Subscript
                    │           │       ├── <77> Subscript
                    │           │       │   ├── <75> Var [strings]
                    │           │       │   ╰── Constant Int [1]
                    │           │       ╰── <79> Var [i]
                    │           ╰── Then
                    │               ╰── Return
                    │                   ╰── Constant Int [3]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_strings_as_initializers_write_to_array() {
    let src = r#"
        int puts(char *s);
        int main(void) {
            char flat_arr[4] = "abc";
            puts(flat_arr);
            flat_arr[2] = 'x';
            puts(flat_arr);
            char nested_array[2][6] = {"Hello", "World"};
            puts(nested_array[0]);
            puts(nested_array[1]);
            nested_array[0][0] = 'J';
            puts(nested_array[0]);
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [puts]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── s
            │           ╰── Type
            │               ╰── Pointer
            │                   ╰── Char
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── flat_arr
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 4
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── <19> "abc"
                    ├── <25> FunctionCall [puts]
                    │   ╰── <24> Var [flat_arr]
                    ├── <33> Assign [=]
                    │   ├── <30> Subscript
                    │   │   ├── <28> Var [flat_arr]
                    │   │   ╰── Constant Int [2]
                    │   ╰── Constant Int [120]
                    ├── <38> FunctionCall [puts]
                    │   ╰── <37> Var [flat_arr]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── nested_array
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 2
                    │   │       ╰── Array
                    │   │           ├── 6
                    │   │           ╰── Char
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <47> "Hello"
                    │           ╰── <49> "World"
                    ├── <58> FunctionCall [puts]
                    │   ╰── <57> Subscript
                    │       ├── <55> Var [nested_array]
                    │       ╰── Constant Int [0]
                    ├── <65> FunctionCall [puts]
                    │   ╰── <64> Subscript
                    │       ├── <62> Var [nested_array]
                    │       ╰── Constant Int [1]
                    ├── <75> Assign [=]
                    │   ├── <72> Subscript
                    │   │   ├── <70> Subscript
                    │   │   │   ├── <68> Var [nested_array]
                    │   │   │   ╰── Constant Int [0]
                    │   │   ╰── Constant Int [0]
                    │   ╰── Constant Int [74]
                    ├── <82> FunctionCall [puts]
                    │   ╰── <81> Subscript
                    │       ├── <79> Var [nested_array]
                    │       ╰── Constant Int [0]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_strings_as_lvalues_addr_of_string() {
    let src = r#"
        int puts(char *s);
        int main(void) {
            char(*str)[16] = &"Sample\tstring!\n";
            puts(*str);
            char (*one_past_the_end)[16] = str + 1;
            char *last_byte_pointer = (char *)one_past_the_end - 1;
            if (*last_byte_pointer != 0) {
                return 1;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [puts]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── s
            │           ╰── Type
            │               ╰── Pointer
            │                   ╰── Char
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── str
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Array
                    │   │           ├── 16
                    │   │           ╰── Char
                    │   ╰── Initializer
                    │       ╰── <22> AddressOf
                    │           ╰── <21> "Sample	string!
        "
                    ├── <29> FunctionCall [puts]
                    │   ╰── <28> Dereference
                    │       ╰── <27> Var [str]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── one_past_the_end
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Array
                    │   │           ├── 16
                    │   │           ╰── Char
                    │   ╰── Initializer
                    │       ╰── <42>  [+]
                    │           ├── <39> Var [str]
                    │           ╰── Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── last_byte_pointer
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── <57>  [-]
                    │           ├── <54> Cast
                    │           │   ├── Target
                    │           │   │   ╰── Pointer
                    │           │   │       ╰── Char
                    │           │   ╰── Expression
                    │           │       ╰── <53> Var [one_past_the_end]
                    │           ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <65>  [!=]
                    │   │       ├── <62> Dereference
                    │   │       │   ╰── <61> Var [last_byte_pointer]
                    │   │       ╰── Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_strings_as_lvalues_adjacent_strings() {
    let src = r#"
        int puts(char *s);
        int main(void) {
            char *strings = "Hello," " World";
            puts(strings);
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [puts]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── s
            │           ╰── Type
            │               ╰── Pointer
            │                   ╰── Char
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── strings
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── <18> "Hello, World"
                    ├── <24> FunctionCall [puts]
                    │   ╰── <23> Var [strings]
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_strings_as_lvalues_array_of_strings() {
    let src = r#"
        int strcmp(char *s1, char *s2);
        int main(void) {
            char *strings[4] = {"yes", "no", "maybe"};
            if (strcmp(strings[0], "yes")) {
                return 1;
            }
            if (strcmp(strings[1], "no")) {
                return 2;
            }
            if (strcmp(strings[2], "maybe")) {
                return 3;
            }
            if (strings[3]) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [strcmp]
            │   ╰── Parameters
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── s1
            │       │   ╰── Type
            │       │       ╰── Pointer
            │       │           ╰── Char
            │       ╰── Param
            │           ├── Name
            │           │   ╰── s2
            │           ╰── Type
            │               ╰── Pointer
            │                   ╰── Char
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── strings
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 4
                    │   │       ╰── Pointer
                    │   │           ╰── Char
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <24> "yes"
                    │           ├── <26> "no"
                    │           ╰── <28> "maybe"
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <38> FunctionCall [strcmp]
                    │   │       ├── <36> Subscript
                    │   │       │   ├── <34> Var [strings]
                    │   │       │   ╰── Constant Int [0]
                    │   │       ╰── <37> "yes"
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <50> FunctionCall [strcmp]
                    │   │       ├── <48> Subscript
                    │   │       │   ├── <46> Var [strings]
                    │   │       │   ╰── Constant Int [1]
                    │   │       ╰── <49> "no"
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <62> FunctionCall [strcmp]
                    │   │       ├── <60> Subscript
                    │   │       │   ├── <58> Var [strings]
                    │   │       │   ╰── Constant Int [2]
                    │   │       ╰── <61> "maybe"
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <71> Subscript
                    │   │       ├── <69> Var [strings]
                    │   │       ╰── Constant Int [3]
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
fn test_valid_strings_as_lvalues_cast_string_pointer() {
    let src = r#"
        int main(void) {
            char *c = "This is a string!";
            unsigned char *uc = (unsigned char *)c;
            if (uc[3] != 's') {
                return 1;
            }
            signed char *sc = (signed char *)c;
            if (sc[3] != 's'){
                    return 2;
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
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── <9> "This is a string!"
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── uc
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Unsigned Char
                    │   ╰── Initializer
                    │       ╰── <21> Cast
                    │           ├── Target
                    │           │   ╰── Pointer
                    │           │       ╰── Unsigned Char
                    │           ╰── Expression
                    │               ╰── <20> Var [c]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <30>  [!=]
                    │   │       ├── <27> Subscript
                    │   │       │   ├── <25> Var [uc]
                    │   │       │   ╰── Constant Int [3]
                    │   │       ╰── Constant Int [115]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── sc
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Signed Char
                    │   ╰── Initializer
                    │       ╰── <45> Cast
                    │           ├── Target
                    │           │   ╰── Pointer
                    │           │       ╰── Signed Char
                    │           ╰── Expression
                    │               ╰── <44> Var [c]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <54>  [!=]
                    │   │       ├── <51> Subscript
                    │   │       │   ├── <49> Var [sc]
                    │   │       │   ╰── Constant Int [3]
                    │   │       ╰── Constant Int [115]
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
fn test_valid_strings_as_lvalues_empty_string() {
    let src = r#"
        
        int main(void) {
            char *empty = "";
            return empty[0];
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── empty
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── <9> ""
                    ╰── Return
                        ╰── <15> Subscript
                            ├── <13> Var [empty]
                            ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_strings_as_lvalues_pointer_operations() {
    let src = r#"
        int main(void) {
            if ("abcdefg"[2] != 'c') {
                return 1;
            }
            char *ptr = "This is a string!" + 10;
            if (*ptr != 's') {
                return 2;
            }
            if (ptr[6] != '!') {
                return 3;
            }
            if (ptr[7]) {
                return 4;
            }
            if (!"Not a null pointer!") {
                return 5;
            }
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <10>  [!=]
                    │   │       ├── <7> Subscript
                    │   │       │   ├── <5> "abcdefg"
                    │   │       │   ╰── Constant Int [2]
                    │   │       ╰── Constant Int [99]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── <23>  [+]
                    │           ├── <20> "This is a string!"
                    │           ╰── Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <31>  [!=]
                    │   │       ├── <28> Dereference
                    │   │       │   ╰── <27> Var [ptr]
                    │   │       ╰── Constant Int [115]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <43>  [!=]
                    │   │       ├── <40> Subscript
                    │   │       │   ├── <38> Var [ptr]
                    │   │       │   ╰── Constant Int [6]
                    │   │       ╰── Constant Int [33]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <52> Subscript
                    │   │       ├── <50> Var [ptr]
                    │   │       ╰── Constant Int [7]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ╰── If
                        ├── Condition
                        │   ╰── <60> Unary [!]
                        │       ╰── <59> "Not a null pointer!"
                        ╰── Then
                            ╰── Block
                                ╰── Return
                                    ╰── Constant Int [5]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_strings_as_lvalues_simple() {
    let src = r#"
        int main(void) {
            char *x = "Hello, World!";
            return x[2];
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
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── <9> "Hello, World!"
                    ╰── Return
                        ╰── <15> Subscript
                            ├── <13> Var [x]
                            ╰── Constant Int [2]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_strings_as_lvalues_standard_library_calls() {
    let src = r#"
        int strcmp(char *s1, char *s2);
        int puts(char *s);
        unsigned long strlen(char *s);
        int atoi(char *s);
        int main(void) {
            if (strcmp("abc", "abc")) {
                return 1;
            }
            if (strcmp("ab", "xy") >= 0) {
                return 2;
            }
            puts("Hello, World!");
            if (strlen("")) {
                return 3;
            }
            int i = atoi("10");
            if (i != 10) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [strcmp]
            │   ╰── Parameters
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── s1
            │       │   ╰── Type
            │       │       ╰── Pointer
            │       │           ╰── Char
            │       ╰── Param
            │           ├── Name
            │           │   ╰── s2
            │           ╰── Type
            │               ╰── Pointer
            │                   ╰── Char
            ├── Function [puts]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── s
            │           ╰── Type
            │               ╰── Pointer
            │                   ╰── Char
            ├── Function [strlen]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── s
            │           ╰── Type
            │               ╰── Pointer
            │                   ╰── Char
            ├── Function [atoi]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── s
            │           ╰── Type
            │               ╰── Pointer
            │                   ╰── Char
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <48> FunctionCall [strcmp]
                    │   │       ├── <46> "abc"
                    │   │       ╰── <47> "abc"
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <60>  [>=]
                    │   │       ├── <57> FunctionCall [strcmp]
                    │   │       │   ├── <55> "ab"
                    │   │       │   ╰── <56> "xy"
                    │   │       ╰── Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── <68> FunctionCall [puts]
                    │   ╰── <67> "Hello, World!"
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <72> FunctionCall [strlen]
                    │   │       ╰── <71> ""
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <83> FunctionCall [atoi]
                    │           ╰── <82> "10"
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <90>  [!=]
                    │   │       ├── <87> Var [i]
                    │   │       ╰── Constant Int [10]
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
fn test_valid_strings_as_lvalues_string_special_characters() {
    let src = r#"
        int puts(char *s);
        int strcmp(char *s1, char *s2);
        int main(void) {
            char *escape_sequence = "\a\b";
            if (escape_sequence[0] != 7) {
                return 1;
            }
            if (escape_sequence[1] != 8) {
                return 2;
            }
            if (escape_sequence[2]) {
                return 3;
            }
            char *with_double_quote = "Hello\"world";
            if (with_double_quote[5] != '"') {
                return 4;
            }
            puts(with_double_quote);
            char *with_backslash = "Hello\\World";
            if (with_backslash[5] != '\\') {
                return 5;
            }
            puts(with_backslash);
            char *with_newline = "Line\nbreak!";
            if (with_newline[4] != 10) {
                return 6;
            }
            puts(with_newline);
            char *tab = "	";
            if (strcmp(tab, "\t")) {
                return 7;
            }
           puts("Testing, 123.");
            puts("^@1 _\\]");
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [puts]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── s
            │           ╰── Type
            │               ╰── Pointer
            │                   ╰── Char
            ├── Function [strcmp]
            │   ╰── Parameters
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── s1
            │       │   ╰── Type
            │       │       ╰── Pointer
            │       │           ╰── Char
            │       ╰── Param
            │           ├── Name
            │           │   ╰── s2
            │           ╰── Type
            │               ╰── Pointer
            │                   ╰── Char
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── escape_sequence
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── <31> ""
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <40>  [!=]
                    │   │       ├── <37> Subscript
                    │   │       │   ├── <35> Var [escape_sequence]
                    │   │       │   ╰── Constant Int [0]
                    │   │       ╰── Constant Int [7]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <52>  [!=]
                    │   │       ├── <49> Subscript
                    │   │       │   ├── <47> Var [escape_sequence]
                    │   │       │   ╰── Constant Int [1]
                    │   │       ╰── Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <61> Subscript
                    │   │       ├── <59> Var [escape_sequence]
                    │   │       ╰── Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── with_double_quote
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── <71> "Hello"world"
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <80>  [!=]
                    │   │       ├── <77> Subscript
                    │   │       │   ├── <75> Var [with_double_quote]
                    │   │       │   ╰── Constant Int [5]
                    │   │       ╰── Constant Int [34]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [4]
                    ├── <89> FunctionCall [puts]
                    │   ╰── <88> Var [with_double_quote]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── with_backslash
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── <95> "Hello\World"
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <104>  [!=]
                    │   │       ├── <101> Subscript
                    │   │       │   ├── <99> Var [with_backslash]
                    │   │       │   ╰── Constant Int [5]
                    │   │       ╰── Constant Int [92]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [5]
                    ├── <113> FunctionCall [puts]
                    │   ╰── <112> Var [with_backslash]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── with_newline
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── <119> "Line
        break!"
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <128>  [!=]
                    │   │       ├── <125> Subscript
                    │   │       │   ├── <123> Var [with_newline]
                    │   │       │   ╰── Constant Int [4]
                    │   │       ╰── Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [6]
                    ├── <137> FunctionCall [puts]
                    │   ╰── <136> Var [with_newline]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── tab
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── <143> "	"
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <150> FunctionCall [strcmp]
                    │   │       ├── <148> Var [tab]
                    │   │       ╰── <149> "	"
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [7]
                    ├── <158> FunctionCall [puts]
                    │   ╰── <157> "Testing, 123."
                    ├── <162> FunctionCall [puts]
                    │   ╰── <161> "^@1 _\]"
                    ╰── Return
                        ╰── Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_strings_as_lvalues_strings_in_function_calls() {
    let src = r#"
        unsigned long strlen(char *s);
        char *return_string(void) {
            return "I'm a string!";
        }
        int pass_string_args(char *s1, char *s2) {
            if (s1 == 0 || s2 == 0) {
                return 0;
            }
            if (strlen(s1) != 45) {
                return 0;
            }
            if (s1[41] != 'd' || s1[42] != 'o' || s1[43] != 'g') {
                return 0;
            }
            if (s2[0]) {
                return 0;
            }
            return 1;
        }
        int main(void) {
            char *ptr = 0;
            ptr = return_string();
            if (!ptr)
                return 1;
            if (ptr[0] != 'I' || ptr[1] != '\'' || ptr[13]) {
                return 2;
            }
            if (!pass_string_args("The quick brown fox jumped over the lazy dog.",
                                  "")) {
                return 3;
            }
            return 0;
            char *ptr2;
            ptr2 = 1 ? ptr + 2 : ptr + 4;
            return *ptr2 == 'm';
        }
    "#;
    let expected = r#"
        Program
            ├── Function [strlen]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── s
            │           ╰── Type
            │               ╰── Pointer
            │                   ╰── Char
            ├── Function [return_string]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <15> "I'm a string!"
            ├── Function [pass_string_args]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── s1
            │   │   │   ╰── Type
            │   │   │       ╰── Pointer
            │   │   │           ╰── Char
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── s2
            │   │       ╰── Type
            │   │           ╰── Pointer
            │   │               ╰── Char
            │   ╰── Body
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <42>  [||]
            │       │   │       ├── <35>  [==]
            │       │   │       │   ├── <32> Var [s1]
            │       │   │       │   ╰── Constant Int [0]
            │       │   │       ╰── <41>  [==]
            │       │   │           ├── <38> Var [s2]
            │       │   │           ╰── Constant Int [0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [0]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <54>  [!=]
            │       │   │       ├── <51> FunctionCall [strlen]
            │       │   │       │   ╰── <50> Var [s1]
            │       │   │       ╰── Constant Int [45]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [0]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <84>  [||]
            │       │   │       ├── <75>  [||]
            │       │   │       │   ├── <66>  [!=]
            │       │   │       │   │   ├── <63> Subscript
            │       │   │       │   │   │   ├── <61> Var [s1]
            │       │   │       │   │   │   ╰── Constant Int [41]
            │       │   │       │   │   ╰── Constant Int [100]
            │       │   │       │   ╰── <74>  [!=]
            │       │   │       │       ├── <71> Subscript
            │       │   │       │       │   ├── <69> Var [s1]
            │       │   │       │       │   ╰── Constant Int [42]
            │       │   │       │       ╰── Constant Int [111]
            │       │   │       ╰── <83>  [!=]
            │       │   │           ├── <80> Subscript
            │       │   │           │   ├── <78> Var [s1]
            │       │   │           │   ╰── Constant Int [43]
            │       │   │           ╰── Constant Int [103]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [0]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <93> Subscript
            │       │   │       ├── <91> Var [s2]
            │       │   │       ╰── Constant Int [0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── Constant Int [0]
            │       ╰── Return
            │           ╰── Constant Int [1]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── Constant Int [0]
                    ├── <119> Assign [=]
                    │   ├── <115> Var [ptr]
                    │   ╰── <118> FunctionCall [return_string]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <124> Unary [!]
                    │   │       ╰── <123> Var [ptr]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <149>  [||]
                    │   │       ├── <143>  [||]
                    │   │       │   ├── <134>  [!=]
                    │   │       │   │   ├── <131> Subscript
                    │   │       │   │   │   ├── <129> Var [ptr]
                    │   │       │   │   │   ╰── Constant Int [0]
                    │   │       │   │   ╰── Constant Int [73]
                    │   │       │   ╰── <142>  [!=]
                    │   │       │       ├── <139> Subscript
                    │   │       │       │   ├── <137> Var [ptr]
                    │   │       │       │   ╰── Constant Int [1]
                    │   │       │       ╰── Constant Int [39]
                    │   │       ╰── <148> Subscript
                    │   │           ├── <146> Var [ptr]
                    │   │           ╰── Constant Int [13]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <160> Unary [!]
                    │   │       ╰── <159> FunctionCall [pass_string_args]
                    │   │           ├── <157> "The quick brown fox jumped over the lazy dog."
                    │   │           ╰── <158> ""
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── Constant Int [3]
                    ├── Return
                    │   ╰── Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr2
                    │   ╰── Type
                    │       ╰── Pointer
                    │           ╰── Char
                    ├── <188> Assign [=]
                    │   ├── <174> Var [ptr2]
                    │   ╰── <{node_id}> Conditional [?]
                    │       ├── Constant Int [1]
                    │       ├── Then
                    │       │   ╰── <181>  [+]
                    │       │       ├── <178> Var [ptr]
                    │       │       ╰── Constant Int [2]
                    │       ╰── Else
                    │           ╰── <186>  [+]
                    │               ├── <183> Var [ptr]
                    │               ╰── Constant Int [4]
                    ╰── Return
                        ╰── <195>  [==]
                            ├── <192> Dereference
                            │   ╰── <191> Var [ptr2]
                            ╰── Constant Int [109]
    "#;
    assert_parse(src, expected);
}
