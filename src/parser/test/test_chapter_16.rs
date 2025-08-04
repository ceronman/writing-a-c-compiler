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
                    │   │   ╰── <10> Constant Int [120]
                    │   ╰── Static
                    ╰── Switch
                        ├── Expression
                        │   ╰── <14> Var [i]
                        ╰── Block
                            ├── Case [120]
                            │   ╰── Return
                            │       ╰── <16> Constant Int [1]
                            ├── Case [120]
                            │   ╰── Return
                            │       ╰── <20> Constant Int [2]
                            ╰── Default
                                ╰── Return
                                    ╰── <23> Constant Int [3]
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
                    ├── <9> Assign [=]
                    │   ├── <6> "foo"
                    │   ╰── <8> "bar"
                    ╰── Return
                        ╰── <11> Constant Int [0]
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
            │       ╰── <4> Constant Int [10]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Signed Char
                    │   ╰── Extern
                    ╰── Return
                        ╰── <18> Var [c]
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
            │           ╰── <10> Var [c]
            ├── Function [main]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <21> FunctionCall [foo]
            │               ╰── <20> Constant Int [0]
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
                    │           ├── <11> Constant Int [97]
                    │           ├── <13> Constant Int [98]
                    │           ╰── <15> Constant Int [99]
                    ╰── Return
                        ╰── <19> Constant Int [0]
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
                    ├── <9>  [<<]
                    │   ├── <6> "foo"
                    │   ╰── <8> Constant Int [3]
                    ╰── Return
                        ╰── <11> Constant Int [0]
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
                    ├── <9>  [&]
                    │   ├── <6> "My string"
                    │   ╰── <8> Constant Int [100]
                    ╰── Return
                        ╰── <11> Constant Int [0]
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
                        │   ╰── <6> Constant Int [0]
                        ╰── Block
                            ├── Case [invalid]
                            │   ├── Value
                            │   │   ╰── <7> "foo"
                            │   ╰── Return
                            │       ╰── <8> Constant Int [1]
                            ╰── Default
                                ╰── Return
                                    ╰── <11> Constant Int [0]
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
                    │       ╰── <11> "some string "
                    ├── <18> Assign [+=]
                    │   ├── <15> Var [s]
                    │   ╰── <17> "another str"
                    ╰── Return
                        ╰── <20> Constant Int [0]
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
                    ├── <9> Assign [+=]
                    │   ├── <6> "My string"
                    │   ╰── <8> Constant Int [1]
                    ╰── Return
                        ╰── <11> Constant Int [0]
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
                    ├── <8> Postfix [++]
                    │   ╰── <6> "foo"
                    ╰── Return
                        ╰── <10> Constant Int [0]
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
                    ├── <8> Unary [++]
                    │   ╰── <7> "foo"
                    ╰── Return
                        ╰── <10> Constant Int [0]
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
                        │   ╰── <6> "foo"
                        ╰── Block
                            ╰── Default
                                ╰── Return
                                    ╰── <7> Constant Int [0]
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
                    │       ╰── <11> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── s
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Signed Char
                    │   ╰── Initializer
                    │       ╰── <20> Var [c]
                    ╰── Return
                        ╰── <27> Cast
                            ├── Target
                            │   ╰── Int
                            ╰── Expression
                                ╰── <26> Var [s]
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
                    │       ╰── <16> AddressOf
                    │           ╰── <15> "x"
                    ╰── Return
                        ╰── <19> Constant Int [0]
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
                    │       ╰── <11> "foo"
                    ╰── Return
                        ╰── <17> Unary [-]
                            ╰── <16> Var [x]
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
            │       ╰── <10> "hello"
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <23> Subscript
                            ├── <21> Subscript
                            │   ├── <19> Var [arr]
                            │   ╰── <20> Constant Int [0]
                            ╰── <22> Constant Int [2]
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
                    │       ╰── <12> "abcd"
                    ╰── Return
                        ╰── <15> Constant Int [0]
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
                    │           ├── <15> "a"
                    │           ╰── <17> "bcde"
                    ╰── Return
                        ╰── <21> Constant Int [0]
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
            │           ├── <10> "a"
            │           ╰── <12> "bcde"
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <21> Constant Int [0]
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
                    │   │   ╰── <13> "abcd"
                    │   ╰── Static
                    ╰── Return
                        ╰── <16> Constant Int [0]
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
                    │       ╰── <12> "abc"
                    ╰── Return
                        ╰── <18> Subscript
                            ├── <16> Var [ints]
                            ╰── <17> Constant Int [1]
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
                    │           ╰── <15> "a"
                    ╰── Return
                        ╰── <19> Constant Int [0]
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
                    │   │       ╰── <16> "a"
                    │   ╰── Static
                    ╰── Return
                        ╰── <20> Constant Int [0]
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
                    │       ╰── <11> "foo"
                    ╰── Return
                        ╰── <14> Constant Int [0]
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
                    │   │   ╰── <12> "foo"
                    │   ╰── Static
                    ╰── Return
                        ╰── <15> Constant Int [0]
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
            │       ╰── <4> Constant Int [92]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <16>  [!=]
                    │   │       ├── <13> Var [d]
                    │   │       ╰── <15> Constant Double [+9.2e1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <17> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── array
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 10
                    │   │       ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <28> Constant Int [1]
                    │           ├── <30> Constant Int [2]
                    │           ├── <32> Constant Int [97]
                    │           ├── <34> Constant Int [8]
                    │           ├── <36> Constant Int [3]
                    │           ├── <38> Constant Int [4]
                    │           ├── <40> Constant Int [5]
                    │           ├── <42> Constant Int [33]
                    │           ├── <44> Constant Int [37]
                    │           ╰── <46> Constant Int [126]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <56>  [!=]
                    │   │       ├── <53> Subscript
                    │   │       │   ├── <51> Var [array]
                    │   │       │   ╰── <52> Constant Int [2]
                    │   │       ╰── <55> Constant Int [97]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <57> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <68>  [!=]
                    │   │       ├── <65> Subscript
                    │   │       │   ├── <63> Var [array]
                    │   │       │   ╰── <64> Constant Int [3]
                    │   │       ╰── <67> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <69> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <80>  [!=]
                    │   │       ├── <77> Subscript
                    │   │       │   ├── <75> Var [array]
                    │   │       │   ╰── <76> Constant Int [7]
                    │   │       ╰── <79> Constant Int [33]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <81> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <92>  [!=]
                    │   │       ├── <89> Subscript
                    │   │       │   ├── <87> Var [array]
                    │   │       │   ╰── <88> Constant Int [8]
                    │   │       ╰── <91> Constant Int [37]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <93> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <104>  [!=]
                    │   │       ├── <101> Subscript
                    │   │       │   ├── <99> Var [array]
                    │   │       │   ╰── <100> Constant Int [9]
                    │   │       ╰── <103> Constant Int [126]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <105> Constant Int [6]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── array_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Array
                    │   │           ├── 10
                    │   │           ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <121> AddressOf
                    │           ╰── <120> Var [array]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <132>  [!=]
                    │   │       ├── <129> Subscript
                    │   │       │   ├── <127> Subscript
                    │   │       │   │   ├── <125> Var [array_ptr]
                    │   │       │   │   ╰── <126> Constant Int [0]
                    │   │       │   ╰── <128> Constant Int [9]
                    │   │       ╰── <131> Constant Int [126]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <133> Constant Int [7]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <144> Subscript
                    │           ├── <142> Var [array]
                    │           ╰── <143> Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <151>  [!=]
                    │   │       ├── <148> Var [i]
                    │   │       ╰── <150> Constant Int [33]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <152> Constant Int [8]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <174>  [-]
                    │           ├── <169>  [+]
                    │           │   ├── <163>  [%]
                    │           │   │   ├── <160> Constant Int [10]
                    │           │   │   ╰── <162> Constant Int [7]
                    │           │   ╰── <168>  [*]
                    │           │       ├── <165> Constant Double [+4e0]
                    │           │       ╰── <167> Constant Int [95]
                    │           ╰── <173> Unary [~]
                    │               ╰── <172> Constant Int [64]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <181>  [!=]
                    │   │       ├── <178> Var [d]
                    │   │       ╰── <180> Constant Double [+4.48e2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <182> Constant Int [9]
                    ╰── Return
                        ╰── <187> Constant Int [0]
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
                    │       ╰── <9> Constant Int [9]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── vertical_tab
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [11]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── form_feed
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <21> Constant Int [12]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <28>  [!=]
                    │   │       ├── <25> Var [tab]
                    │   │       ╰── <27> Constant Int [9]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <29> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <38>  [!=]
                    │   │       ├── <35> Var [vertical_tab]
                    │   │       ╰── <37> Constant Int [11]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <39> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <48>  [!=]
                    │   │       ├── <45> Var [form_feed]
                    │   │       ╰── <47> Constant Int [12]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <49> Constant Int [3]
                    ╰── Return
                        ╰── <54> Constant Int [0]
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
                    │   │   ╰── <9>  [!=]
                    │   │       ├── <6> Constant Int [63]
                    │   │       ╰── <8> Constant Int [63]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <10> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <18>  [!=]
                    │   │       ├── <15> Constant Int [34]
                    │   │       ╰── <17> Constant Int [34]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <19> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <27>  [!=]
                    │   │       ├── <24> Constant Int [39]
                    │   │       ╰── <26> Constant Int [39]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <28> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <36>  [!=]
                    │   │       ├── <33> Constant Int [92]
                    │   │       ╰── <35> Constant Int [92]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <37> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <45>  [!=]
                    │   │       ├── <42> Constant Int [7]
                    │   │       ╰── <44> Constant Int [7]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <46> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <54>  [!=]
                    │   │       ├── <51> Constant Int [8]
                    │   │       ╰── <53> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <55> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <63>  [!=]
                    │   │       ├── <60> Constant Int [12]
                    │   │       ╰── <62> Constant Int [12]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <64> Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <72>  [!=]
                    │   │       ├── <69> Constant Int [10]
                    │   │       ╰── <71> Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <73> Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <81>  [!=]
                    │   │       ├── <78> Constant Int [13]
                    │   │       ╰── <80> Constant Int [13]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <82> Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <90>  [!=]
                    │   │       ├── <87> Constant Int [9]
                    │   │       ╰── <89> Constant Int [9]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <91> Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <99>  [!=]
                    │   │       ├── <96> Constant Int [11]
                    │   │       ╰── <98> Constant Int [11]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <100> Constant Int [11]
                    ╰── Return
                        ╰── <105> Constant Int [0]
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
                        ╰── <6> Constant Int [99]
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
                    │       ╰── <9> Constant Int [100]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── byte_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── <23> Cast
                    │           ├── Target
                    │           │   ╰── Pointer
                    │           │       ╰── Char
                    │           ╰── Expression
                    │               ╰── <22> AddressOf
                    │                   ╰── <21> Var [x]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <32>  [!=]
                    │   │       ├── <29> Subscript
                    │   │       │   ├── <27> Var [byte_ptr]
                    │   │       │   ╰── <28> Constant Int [0]
                    │   │       ╰── <31> Constant Int [100]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <33> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <53>  [||]
                    │   │       ├── <47>  [||]
                    │   │       │   ├── <41> Subscript
                    │   │       │   │   ├── <39> Var [byte_ptr]
                    │   │       │   │   ╰── <40> Constant Int [1]
                    │   │       │   ╰── <46> Subscript
                    │   │       │       ├── <44> Var [byte_ptr]
                    │   │       │       ╰── <45> Constant Int [2]
                    │   │       ╰── <52> Subscript
                    │   │           ├── <50> Var [byte_ptr]
                    │   │           ╰── <51> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <54> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Double
                    │   ╰── Initializer
                    │       ╰── <64> Unary [-]
                    │           ╰── <63> Constant Double [+0e0]
                    ├── <77> Assign [=]
                    │   ├── <68> Var [byte_ptr]
                    │   ╰── <76> Cast
                    │       ├── Target
                    │       │   ╰── Pointer
                    │       │       ╰── Char
                    │       ╰── Expression
                    │           ╰── <75> AddressOf
                    │               ╰── <74> Var [d]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <87>  [!=]
                    │   │       ├── <82> Subscript
                    │   │       │   ├── <80> Var [byte_ptr]
                    │   │       │   ╰── <81> Constant Int [7]
                    │   │       ╰── <86> Unary [-]
                    │   │           ╰── <85> Constant Int [128]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <88> Constant Int [3]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <96> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <104>  [<]
                    │   │       ├── <101> Var [i]
                    │   │       ╰── <103> Constant Int [7]
                    │   ├── Condition
                    │   │   ╰── <113> Assign [=]
                    │   │       ├── <106> Var [i]
                    │   │       ╰── <112>  [+]
                    │   │           ├── <109> Var [i]
                    │   │           ╰── <111> Constant Int [1]
                    │   ╰── Block
                    │       ╰── If
                    │           ├── Condition
                    │           │   ╰── <118> Subscript
                    │           │       ├── <115> Var [byte_ptr]
                    │           │       ╰── <117> Var [i]
                    │           ╰── Then
                    │               ╰── Block
                    │                   ╰── Return
                    │                       ╰── <119> Constant Int [4]
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
                    │           │   │   ╰── <141> Unary [-]
                    │           │   │       ╰── <140> Constant Int [1]
                    │           │   ╰── Compound
                    │           │       ╰── <146> Unary [-]
                    │           │           ╰── <145> Constant Int [1]
                    │           ├── Compound
                    │           │   ├── Compound
                    │           │   │   ╰── <152> Unary [-]
                    │           │   │       ╰── <151> Constant Int [1]
                    │           │   ╰── Compound
                    │           │       ╰── <157> Unary [-]
                    │           │           ╰── <156> Constant Int [1]
                    │           ╰── Compound
                    │               ╰── Compound
                    │                   ╰── <161> Constant UInt [4294901760]
                    ├── <176> Assign [=]
                    │   ├── <168> Var [byte_ptr]
                    │   ╰── <175> Cast
                    │       ├── Target
                    │       │   ╰── Pointer
                    │       │       ╰── Char
                    │       ╰── Expression
                    │           ╰── <174> Var [array]
                    ├── <186> Assign [=]
                    │   ├── <179> Var [byte_ptr]
                    │   ╰── <185>  [+]
                    │       ├── <182> Var [byte_ptr]
                    │       ╰── <184> Constant Int [16]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <197>  [||]
                    │   │       ├── <191> Subscript
                    │   │       │   ├── <189> Var [byte_ptr]
                    │   │       │   ╰── <190> Constant Int [0]
                    │   │       ╰── <196> Subscript
                    │   │           ├── <194> Var [byte_ptr]
                    │   │           ╰── <195> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <198> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <211>  [!=]
                    │   │       ├── <206> Subscript
                    │   │       │   ├── <204> Var [byte_ptr]
                    │   │       │   ╰── <205> Constant Int [2]
                    │   │       ╰── <210> Unary [-]
                    │   │           ╰── <209> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <212> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <225>  [!=]
                    │   │       ├── <220> Subscript
                    │   │       │   ├── <218> Var [byte_ptr]
                    │   │       │   ╰── <219> Constant Int [3]
                    │   │       ╰── <224> Unary [-]
                    │   │           ╰── <223> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <226> Constant Int [7]
                    ╰── Return
                        ╰── <231> Constant Int [0]
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
            │       ╰── <4> Constant UInt [4294967200]
            ╰── Function [main]
                ╰── Body
                    ├── <23> Assign [=]
                    │   ├── <13> Var [ui]
                    │   ╰── <22> Cast
                    │       ├── Target
                    │       │   ╰── Unsigned Int
                    │       ╰── Expression
                    │           ╰── <21> Cast
                    │               ├── Target
                    │               │   ╰── Unsigned Char
                    │               ╰── Expression
                    │                   ╰── <20> Var [ui]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <29>  [!=]
                    │   │       ├── <26> Var [ui]
                    │   │       ╰── <28> Constant Int [160]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <30> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <45> Cast
                    │           ├── Target
                    │           │   ╰── Int
                    │           ╰── Expression
                    │               ╰── <44> Cast
                    │                   ├── Target
                    │                   │   ╰── Signed Char
                    │                   ╰── Expression
                    │                       ╰── <43> Var [ui]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <54>  [!=]
                    │   │       ├── <49> Var [i]
                    │   │       ╰── <53> Unary [-]
                    │   │           ╰── <52> Constant Int [96]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <55> Constant Int [2]
                    ╰── Return
                        ╰── <60> Constant Int [0]
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
            │       │       ╰── <33> Constant Int [5]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── expected_b
            │       │   ├── Type
            │       │   │   ╰── Signed Char
            │       │   ╰── Initializer
            │       │       ╰── <41> Unary [-]
            │       │           ╰── <40> Constant Int [12]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── expected_c
            │       │   ├── Type
            │       │   │   ╰── Char
            │       │   ╰── Initializer
            │       │       ╰── <47> Constant Int [117]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── expected_d
            │       │   ├── Type
            │       │   │   ╰── Unsigned Char
            │       │   ╰── Initializer
            │       │       ╰── <53> Constant Int [254]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── expected_e
            │       │   ├── Type
            │       │   │   ╰── Char
            │       │   ╰── Initializer
            │       │       ╰── <59> Constant Int [1]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── expected_f
            │       │   ├── Type
            │       │   │   ╰── Char
            │       │   ╰── Initializer
            │       │       ╰── <67> Unary [-]
            │       │           ╰── <66> Constant Int [20]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── expected_g
            │       │   ├── Type
            │       │   │   ╰── Signed Char
            │       │   ╰── Initializer
            │       │       ╰── <73> Constant Int [60]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── expected_h
            │       │   ├── Type
            │       │   │   ╰── Char
            │       │   ╰── Initializer
            │       │       ╰── <79> Constant Int [100]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <87>  [!=]
            │       │   │       ├── <83> Var [expected_a]
            │       │   │       ╰── <86> Var [a]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <88> Constant Int [1]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <98>  [!=]
            │       │   │       ├── <94> Var [expected_b]
            │       │   │       ╰── <97> Var [b]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <99> Constant Int [2]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <109>  [!=]
            │       │   │       ├── <105> Var [expected_c]
            │       │   │       ╰── <108> Var [c]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <110> Constant Int [3]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <120>  [!=]
            │       │   │       ├── <116> Var [expected_d]
            │       │   │       ╰── <119> Var [d]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <121> Constant Int [4]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <131>  [!=]
            │       │   │       ├── <127> Var [expected_e]
            │       │   │       ╰── <130> Var [e]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <132> Constant Int [5]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <142>  [!=]
            │       │   │       ├── <138> Var [expected_f]
            │       │   │       ╰── <141> Var [f]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <143> Constant Int [6]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <153>  [!=]
            │       │   │       ├── <149> Var [expected_g]
            │       │   │       ╰── <152> Var [g]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <154> Constant Int [7]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <164>  [!=]
            │       │   │       ├── <160> Var [expected_h]
            │       │   │       ╰── <163> Var [h]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <165> Constant Int [8]
            │       ╰── Return
            │           ╰── <170> Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── <182> Constant Int [5]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Signed Char
                    │   ╰── Initializer
                    │       ╰── <190> Unary [-]
                    │           ╰── <189> Constant Int [12]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── <196> Constant Int [117]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Unsigned Char
                    │   ╰── Initializer
                    │       ╰── <202> Constant Int [254]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── e
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── <208> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── f
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── <216> Unary [-]
                    │           ╰── <215> Constant Int [20]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── g
                    │   ├── Type
                    │   │   ╰── Signed Char
                    │   ╰── Initializer
                    │       ╰── <222> Constant Int [60]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── h
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── <228> Constant Int [100]
                    ╰── Return
                        ╰── <248> FunctionCall [check_args]
                            ├── <233> Var [a]
                            ├── <235> Var [b]
                            ├── <237> Var [c]
                            ├── <239> Var [d]
                            ├── <241> Var [e]
                            ├── <243> Var [f]
                            ├── <245> Var [g]
                            ╰── <247> Var [h]
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
            │           ╰── <17>  [+]
            │               ├── <13> Var [c1]
            │               ╰── <16> Var [c2]
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
            │           ╰── <37>  [/]
            │               ├── <33> Var [c1]
            │               ╰── <36> Var [c2]
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
            │           ╰── <57>  [<=]
            │               ├── <53> Var [c1]
            │               ╰── <56> Var [c2]
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
            │           ╰── <78> Subscript
            │               ├── <75> Var [ptr]
            │               ╰── <77> Var [idx]
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
            │           ╰── <102>  [-]
            │               ├── <98> Var [ptr]
            │               ╰── <101> Var [idx]
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
            │           ╰── <122>  [&&]
            │               ├── <118> Var [c1]
            │               ╰── <121> Var [i]
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
            │           ╰── <142>  [||]
            │               ├── <138> Var [c1]
            │               ╰── <141> Var [c2]
            ├── Function [test_for_loop_char]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── counter
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <154> Constant Int [0]
            │       ├── For
            │       │   ├── Init
            │       │   │   ╰── VarDeclaration
            │       │   │       ├── Name
            │       │   │       │   ╰── s
            │       │   │       ├── Type
            │       │   │       │   ╰── Signed Char
            │       │   │       ╰── Initializer
            │       │   │           ╰── <160> Constant Int [127]
            │       │   ├── Condition
            │       │   │   ╰── <168>  [>]
            │       │   │       ├── <165> Var [s]
            │       │   │       ╰── <167> Constant Int [0]
            │       │   ├── Condition
            │       │   │   ╰── <177> Assign [=]
            │       │   │       ├── <170> Var [s]
            │       │   │       ╰── <176>  [-]
            │       │   │           ├── <173> Var [s]
            │       │   │           ╰── <175> Constant Int [1]
            │       │   ╰── Block
            │       │       ╰── <186> Assign [=]
            │       │           ├── <179> Var [counter]
            │       │           ╰── <185>  [+]
            │       │               ├── <182> Var [counter]
            │       │               ╰── <184> Constant Int [1]
            │       ╰── Return
            │           ╰── <196>  [==]
            │               ├── <192> Var [counter]
            │               ╰── <194> Constant Int [127]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c1
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── <208> Constant Int [8]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c2
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── <214> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <225>  [!=]
                    │   │       ├── <222> FunctionCall [add_chars]
                    │   │       │   ├── <219> Var [c1]
                    │   │       │   ╰── <221> Var [c2]
                    │   │       ╰── <224> Constant Int [12]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <226> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── uc1
                    │   ├── Type
                    │   │   ╰── Unsigned Char
                    │   ╰── Initializer
                    │       ╰── <234> Constant Int [250]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── uc2
                    │   ├── Type
                    │   │   ╰── Unsigned Char
                    │   ╰── Initializer
                    │       ╰── <240> Constant Int [25]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <251>  [!=]
                    │   │       ├── <248> FunctionCall [divide_chars]
                    │   │       │   ├── <245> Var [uc1]
                    │   │       │   ╰── <247> Var [uc2]
                    │   │       ╰── <250> Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <252> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <262> FunctionCall [le]
                    │   │       ├── <259> Var [c1]
                    │   │       ╰── <261> Var [c2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <263> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <275> Unary [!]
                    │   │       ╰── <274> FunctionCall [le]
                    │   │           ├── <271> Var [c2]
                    │   │           ╰── <273> Var [c2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <276> Constant Int [4]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── arr
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 4
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <287> Constant Int [11]
                    │           ├── <289> Constant Int [12]
                    │           ├── <291> Constant Int [13]
                    │           ╰── <293> Constant Int [14]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── idx
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── <300> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <311>  [!=]
                    │   │       ├── <308> FunctionCall [subscript_char]
                    │   │       │   ├── <305> Var [arr]
                    │   │       │   ╰── <307> Var [idx]
                    │   │       ╰── <310> Constant Int [13]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <312> Constant Int [5]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── offset
                    │   ├── Type
                    │   │   ╰── Signed Char
                    │   ╰── Initializer
                    │       ╰── <320> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <335>  [!=]
                    │   │       ├── <331> FunctionCall [sub_char_from_pointer]
                    │   │       │   ├── <328>  [+]
                    │   │       │   │   ├── <325> Var [arr]
                    │   │       │   │   ╰── <327> Constant Int [1]
                    │   │       │   ╰── <330> Var [offset]
                    │   │       ╰── <334> Var [arr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <336> Constant Int [6]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── zero
                    │   ├── Type
                    │   │   ╰── Signed Char
                    │   ╰── Initializer
                    │       ╰── <344> Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <348> Var [zero]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <349> Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <358> FunctionCall [and_char]
                    │   │       ├── <356> Var [zero]
                    │   │       ╰── <357> Constant Int [12]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <359> Constant Int [8]
                    ├── <368> Assign [=]
                    │   ├── <365> Var [uc2]
                    │   ╰── <367> Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <375> FunctionCall [or_char]
                    │   │       ├── <372> Var [zero]
                    │   │       ╰── <374> Var [uc2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <376> Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <384> Unary [!]
                    │   │       ╰── <383> FunctionCall [test_for_loop_char]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <385> Constant Int [10]
                    ╰── Return
                        ╰── <390> Constant Int [0]
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
            │           ╰── <17> Conditional [?]
            │               ├── <13> Var [flag]
            │               ├── Then
            │               │   ╰── <15> Var [c]
            │               ╰── Else
            │                   ╰── <16> Constant UInt [1]
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
            │           ╰── <37>  [<]
            │               ├── <33> Var [c]
            │               ╰── <36> Var [i]
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
            │           ╰── <57>  [>]
            │               ├── <53> Var [uc]
            │               ╰── <56> Var [l]
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
            │           ╰── <77>  [<]
            │               ├── <73> Var [c]
            │               ╰── <76> Var [u]
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
            │           ╰── <97>  [<=]
            │               ├── <93> Var [s]
            │               ╰── <96> Var [c]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── ten
            │   ├── Type
            │   │   ╰── Char
            │   ╰── Initializer
            │       ╰── <104> Constant Int [10]
            ├── Function [multiply]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── i
            │       │   ├── Type
            │       │   │   ╰── Char
            │       │   ╰── Initializer
            │       │       ╰── <119>  [*]
            │       │           ├── <115> Constant Double [+1.075e1]
            │       │           ╰── <118> Var [ten]
            │       ╰── Return
            │           ╰── <126>  [==]
            │               ├── <123> Var [i]
            │               ╰── <125> Constant Int [107]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <143>  [!=]
                    │   │       ├── <140> FunctionCall [ternary]
                    │   │       │   ├── <136> Constant Int [1]
                    │   │       │   ╰── <139> Unary [-]
                    │   │       │       ╰── <138> Constant Int [10]
                    │   │       ╰── <142> Constant Long [4294967286]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <144> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <157> Unary [!]
                    │   │       ╰── <156> FunctionCall [char_lt_int]
                    │   │           ├── <154> Cast
                    │   │           │   ├── Target
                    │   │           │   │   ╰── Char
                    │   │           │   ╰── Expression
                    │   │           │       ╰── <153> Constant Int [1]
                    │   │           ╰── <155> Constant Int [256]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <158> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <173> Unary [!]
                    │   │       ╰── <172> FunctionCall [uchar_gt_long]
                    │   │           ├── <168> Cast
                    │   │           │   ├── Target
                    │   │           │   │   ╰── Unsigned Char
                    │   │           │   ╰── Expression
                    │   │           │       ╰── <167> Constant Int [100]
                    │   │           ╰── <171> Unary [-]
                    │   │               ╰── <170> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <174> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── <184> Unary [-]
                    │           ╰── <183> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── u
                    │   ├── Type
                    │   │   ╰── Unsigned Char
                    │   ╰── Initializer
                    │       ╰── <190> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <200> Unary [!]
                    │   │       ╰── <199> FunctionCall [char_lt_uchar]
                    │   │           ├── <196> Var [c]
                    │   │           ╰── <198> Var [u]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <201> Constant Int [4]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── s
                    │   ├── Type
                    │   │   ╰── Signed Char
                    │   ╰── Initializer
                    │       ╰── <211> Unary [-]
                    │           ╰── <210> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <221> Unary [!]
                    │   │       ╰── <220> FunctionCall [signed_char_le_char]
                    │   │           ├── <217> Var [s]
                    │   │           ╰── <219> Var [c]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <222> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <230> Unary [!]
                    │   │       ╰── <229> FunctionCall [multiply]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <231> Constant Int [6]
                    ╰── Return
                        ╰── <236> Constant Int [0]
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
            │           ╰── <18>  [==]
            │               ├── <13> Var [converted]
            │               ╰── <16> Var [expected]
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
            │           ╰── <39>  [==]
            │               ├── <34> Var [converted]
            │               ╰── <37> Var [expected]
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
            │           ╰── <60>  [==]
            │               ├── <55> Var [converted]
            │               ╰── <58> Var [expected]
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
            │           ╰── <81>  [==]
            │               ├── <76> Var [converted]
            │               ╰── <79> Var [expected]
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
            │           ╰── <102>  [==]
            │               ├── <97> Var [converted]
            │               ╰── <100> Var [expected]
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
            │           ╰── <123>  [==]
            │               ├── <118> Var [converted]
            │               ╰── <121> Var [expected]
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
            │           ╰── <144>  [==]
            │               ├── <139> Var [converted]
            │               ╰── <142> Var [expected]
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
            │           ╰── <182>  [==]
            │               ├── <178> Var [converted]
            │               ╰── <181> Var [expected]
            ├── Function [return_extended_uchar]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── c
            │   │       ╰── Type
            │   │           ╰── Unsigned Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <195> Var [c]
            ├── Function [return_extended_schar]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── sc
            │   │       ╰── Type
            │   │           ╰── Signed Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <208> Var [sc]
            ├── Function [return_truncated_long]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── l
            │   │       ╰── Type
            │   │           ╰── Long
            │   ╰── Body
            │       ╰── Return
            │           ╰── <221> Var [l]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── sc
                    │   ├── Type
                    │   │   ╰── Signed Char
                    │   ╰── Initializer
                    │       ╰── <235> Unary [-]
                    │           ╰── <234> Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <246> Unary [!]
                    │   │       ╰── <245> FunctionCall [check_long]
                    │   │           ├── <241> Var [sc]
                    │   │           ╰── <244> Unary [-]
                    │   │               ╰── <243> Constant Long [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <247> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <258> Unary [!]
                    │   │       ╰── <257> FunctionCall [check_uint]
                    │   │           ├── <255> Var [sc]
                    │   │           ╰── <256> Constant UInt [4294967286]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <259> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <272> Unary [!]
                    │   │       ╰── <271> FunctionCall [check_double]
                    │   │           ├── <267> Var [sc]
                    │   │           ╰── <270> Unary [-]
                    │   │               ╰── <269> Constant Double [+1e1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <273> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── uc
                    │   ├── Type
                    │   │   ╰── Unsigned Char
                    │   ╰── Initializer
                    │       ╰── <281> Constant Int [246]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <291> Unary [!]
                    │   │       ╰── <290> FunctionCall [check_uchar]
                    │   │           ├── <287> Var [sc]
                    │   │           ╰── <289> Var [uc]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <292> Constant Int [4]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── <302> Unary [-]
                    │           ╰── <301> Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <313> Unary [!]
                    │   │       ╰── <312> FunctionCall [check_char]
                    │   │           ├── <309> Unary [-]
                    │   │           │   ╰── <308> Constant Int [10]
                    │   │           ╰── <311> Var [c]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <314> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <325> Unary [!]
                    │   │       ╰── <324> FunctionCall [check_char]
                    │   │           ├── <321> Constant UInt [4294967286]
                    │   │           ╰── <323> Var [c]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <326> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <339> Unary [!]
                    │   │       ╰── <338> FunctionCall [check_char]
                    │   │           ├── <335> Unary [-]
                    │   │           │   ╰── <334> Constant Double [+1e1]
                    │   │           ╰── <337> Var [c]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <340> Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <359> Unary [!]
                    │   │       ╰── <358> FunctionCall [check_char_on_stack]
                    │   │           ├── <348> Var [c]
                    │   │           ├── <349> Constant Int [0]
                    │   │           ├── <350> Constant Int [0]
                    │   │           ├── <351> Constant Int [0]
                    │   │           ├── <352> Constant Int [0]
                    │   │           ├── <353> Constant Int [0]
                    │   │           ├── <354> Constant Int [0]
                    │   │           ╰── <357> Unary [-]
                    │   │               ╰── <356> Constant Double [+1e1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <360> Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <371> Unary [!]
                    │   │       ╰── <370> FunctionCall [check_int]
                    │   │           ├── <368> Var [uc]
                    │   │           ╰── <369> Constant Int [246]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <372> Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <383> Unary [!]
                    │   │       ╰── <382> FunctionCall [check_ulong]
                    │   │           ├── <380> Var [uc]
                    │   │           ╰── <381> Constant ULong [246]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <384> Constant Int [10]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── expected_char
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── <394> Unary [-]
                    │           ╰── <393> Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <404> Unary [!]
                    │   │       ╰── <403> FunctionCall [check_char]
                    │   │           ├── <400> Var [uc]
                    │   │           ╰── <402> Var [expected_char]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <405> Constant Int [11]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <416> Unary [!]
                    │   │       ╰── <415> FunctionCall [check_uchar]
                    │   │           ├── <412> Constant ULong [18446744073709551606]
                    │   │           ╰── <414> Var [uc]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <417> Constant Int [12]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <428>  [!=]
                    │   │       ├── <425> FunctionCall [return_extended_uchar]
                    │   │       │   ╰── <424> Var [uc]
                    │   │       ╰── <427> Constant Int [246]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <429> Constant Int [13]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <440>  [!=]
                    │   │       ├── <437> FunctionCall [return_extended_schar]
                    │   │       │   ╰── <436> Var [sc]
                    │   │       ╰── <439> Constant ULong [18446744073709551606]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <441> Constant Int [14]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <452>  [!=]
                    │   │       ├── <448> FunctionCall [return_truncated_long]
                    │   │       │   ╰── <447> Constant Long [5369233654]
                    │   │       ╰── <451> Var [uc]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <453> Constant Int [15]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── array
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 3
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <464> Constant Int [0]
                    │           ├── <466> Constant Int [0]
                    │           ╰── <468> Constant Int [0]
                    ├── <478> Assign [=]
                    │   ├── <475> Subscript
                    │   │   ├── <473> Var [array]
                    │   │   ╰── <474> Constant Int [1]
                    │   ╰── <477> Constant Int [128]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <500>  [||]
                    │   │       ├── <489>  [||]
                    │   │       │   ├── <483> Subscript
                    │   │       │   │   ├── <481> Var [array]
                    │   │       │   │   ╰── <482> Constant Int [0]
                    │   │       │   ╰── <488> Subscript
                    │   │       │       ├── <486> Var [array]
                    │   │       │       ╰── <487> Constant Int [2]
                    │   │       ╰── <499>  [!=]
                    │   │           ├── <494> Subscript
                    │   │           │   ├── <492> Var [array]
                    │   │           │   ╰── <493> Constant Int [1]
                    │   │           ╰── <498> Unary [-]
                    │   │               ╰── <497> Constant Int [128]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <501> Constant Int [16]
                    ├── <512> Assign [=]
                    │   ├── <509> Subscript
                    │   │   ├── <507> Var [array]
                    │   │   ╰── <508> Constant Int [1]
                    │   ╰── <511> Constant ULong [9224497936761618562]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <534>  [||]
                    │   │       ├── <523>  [||]
                    │   │       │   ├── <517> Subscript
                    │   │       │   │   ├── <515> Var [array]
                    │   │       │   │   ╰── <516> Constant Int [0]
                    │   │       │   ╰── <522> Subscript
                    │   │       │       ├── <520> Var [array]
                    │   │       │       ╰── <521> Constant Int [2]
                    │   │       ╰── <533>  [!=]
                    │   │           ├── <528> Subscript
                    │   │           │   ├── <526> Var [array]
                    │   │           │   ╰── <527> Constant Int [1]
                    │   │           ╰── <532> Unary [-]
                    │   │               ╰── <531> Constant Int [126]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <535> Constant Int [17]
                    ├── <548> Assign [=]
                    │   ├── <543> Subscript
                    │   │   ├── <541> Var [array]
                    │   │   ╰── <542> Constant Int [1]
                    │   ╰── <547> Unary [-]
                    │       ╰── <546> Constant Double [+2.6e0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <570>  [||]
                    │   │       ├── <559>  [||]
                    │   │       │   ├── <553> Subscript
                    │   │       │   │   ├── <551> Var [array]
                    │   │       │   │   ╰── <552> Constant Int [0]
                    │   │       │   ╰── <558> Subscript
                    │   │       │       ├── <556> Var [array]
                    │   │       │       ╰── <557> Constant Int [2]
                    │   │       ╰── <569>  [!=]
                    │   │           ├── <564> Subscript
                    │   │           │   ├── <562> Var [array]
                    │   │           │   ╰── <563> Constant Int [1]
                    │   │           ╰── <568> Unary [-]
                    │   │               ╰── <567> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <571> Constant Int [18]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── uchar_array
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 3
                    │   │       ╰── Unsigned Char
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <582> Constant Int [0]
                    │           ├── <584> Constant Int [0]
                    │           ╰── <586> Constant Int [0]
                    ├── <596> Assign [=]
                    │   ├── <593> Subscript
                    │   │   ├── <591> Var [uchar_array]
                    │   │   ╰── <592> Constant Int [1]
                    │   ╰── <595> Constant Long [17592186044416]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <616>  [||]
                    │   │       ├── <607>  [||]
                    │   │       │   ├── <601> Subscript
                    │   │       │   │   ├── <599> Var [uchar_array]
                    │   │       │   │   ╰── <600> Constant Int [0]
                    │   │       │   ╰── <606> Subscript
                    │   │       │       ├── <604> Var [uchar_array]
                    │   │       │       ╰── <605> Constant Int [2]
                    │   │       ╰── <615>  [!=]
                    │   │           ├── <612> Subscript
                    │   │           │   ├── <610> Var [uchar_array]
                    │   │           │   ╰── <611> Constant Int [1]
                    │   │           ╰── <614> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <617> Constant Int [19]
                    ├── <628> Assign [=]
                    │   ├── <625> Subscript
                    │   │   ├── <623> Var [uchar_array]
                    │   │   ╰── <624> Constant Int [1]
                    │   ╰── <627> Constant UInt [2147483898]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <648>  [||]
                    │   │       ├── <639>  [||]
                    │   │       │   ├── <633> Subscript
                    │   │       │   │   ├── <631> Var [uchar_array]
                    │   │       │   │   ╰── <632> Constant Int [0]
                    │   │       │   ╰── <638> Subscript
                    │   │       │       ├── <636> Var [uchar_array]
                    │   │       │       ╰── <637> Constant Int [2]
                    │   │       ╰── <647>  [!=]
                    │   │           ├── <644> Subscript
                    │   │           │   ├── <642> Var [uchar_array]
                    │   │           │   ╰── <643> Constant Int [1]
                    │   │           ╰── <646> Constant Int [250]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <649> Constant Int [20]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ui
                    │   ├── Type
                    │   │   ╰── Unsigned Int
                    │   ╰── Initializer
                    │       ╰── <657> Constant UInt [4294967295]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── uc_static
                    │   ├── Type
                    │   │   ╰── Unsigned Char
                    │   ╰── Static
                    ├── <670> Assign [=]
                    │   ├── <666> Var [ui]
                    │   ╰── <669> Var [uc_static]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <673> Var [ui]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <674> Constant Int [21]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── l
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <684> Unary [-]
                    │           ╰── <683> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── s_static
                    │   ├── Type
                    │   │   ╰── Int
                    │   ├── Initializer
                    │   │   ╰── <691> Constant Int [0]
                    │   ╰── Static
                    ├── <699> Assign [=]
                    │   ├── <695> Var [l]
                    │   ╰── <698> Var [s_static]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <702> Var [l]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <703> Constant Int [22]
                    ╰── Return
                        ╰── <708> Constant Int [0]
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
            │           ╰── <13> Cast
            │               ├── Target
            │               │   ╰── Unsigned Char
            │               ╰── Expression
            │                   ╰── <12> Var [c]
            ├── Function [char_to_schar]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── c
            │   │       ╰── Type
            │   │           ╰── Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <29> Cast
            │               ├── Target
            │               │   ╰── Signed Char
            │               ╰── Expression
            │                   ╰── <28> Var [c]
            ├── Function [uchar_to_char]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── u
            │   │       ╰── Type
            │   │           ╰── Unsigned Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <45> Cast
            │               ├── Target
            │               │   ╰── Char
            │               ╰── Expression
            │                   ╰── <44> Var [u]
            ├── Function [schar_to_char]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── u
            │   │       ╰── Type
            │   │           ╰── Signed Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <61> Cast
            │               ├── Target
            │               │   ╰── Char
            │               ╰── Expression
            │                   ╰── <60> Var [u]
            ├── Function [uchar_to_schar]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── u
            │   │       ╰── Type
            │   │           ╰── Unsigned Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <77> Cast
            │               ├── Target
            │               │   ╰── Signed Char
            │               ╰── Expression
            │                   ╰── <76> Var [u]
            ├── Function [schar_to_uchar]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── u
            │   │       ╰── Type
            │   │           ╰── Signed Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <93> Cast
            │               ├── Target
            │               │   ╰── Unsigned Char
            │               ╰── Expression
            │                   ╰── <92> Var [u]
            ├── Function [char_to_int]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── c
            │   │       ╰── Type
            │   │           ╰── Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <109> Cast
            │               ├── Target
            │               │   ╰── Int
            │               ╰── Expression
            │                   ╰── <108> Var [c]
            ├── Function [char_to_uint]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── c
            │   │       ╰── Type
            │   │           ╰── Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <125> Cast
            │               ├── Target
            │               │   ╰── Unsigned Int
            │               ╰── Expression
            │                   ╰── <124> Var [c]
            ├── Function [char_to_long]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── c
            │   │       ╰── Type
            │   │           ╰── Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <141> Cast
            │               ├── Target
            │               │   ╰── Long
            │               ╰── Expression
            │                   ╰── <140> Var [c]
            ├── Function [char_to_ulong]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── c
            │   │       ╰── Type
            │   │           ╰── Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <157> Cast
            │               ├── Target
            │               │   ╰── Unsigned Long
            │               ╰── Expression
            │                   ╰── <156> Var [c]
            ├── Function [char_to_double]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── c
            │   │       ╰── Type
            │   │           ╰── Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <173> Cast
            │               ├── Target
            │               │   ╰── Double
            │               ╰── Expression
            │                   ╰── <172> Var [c]
            ├── Function [schar_to_int]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── s
            │   │       ╰── Type
            │   │           ╰── Signed Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <189> Cast
            │               ├── Target
            │               │   ╰── Int
            │               ╰── Expression
            │                   ╰── <188> Var [s]
            ├── Function [schar_to_uint]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── s
            │   │       ╰── Type
            │   │           ╰── Signed Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <205> Cast
            │               ├── Target
            │               │   ╰── Unsigned Int
            │               ╰── Expression
            │                   ╰── <204> Var [s]
            ├── Function [schar_to_long]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── s
            │   │       ╰── Type
            │   │           ╰── Signed Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <221> Cast
            │               ├── Target
            │               │   ╰── Long
            │               ╰── Expression
            │                   ╰── <220> Var [s]
            ├── Function [schar_to_ulong]
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
            │               │   ╰── Unsigned Long
            │               ╰── Expression
            │                   ╰── <236> Var [s]
            ├── Function [schar_to_double]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── s
            │   │       ╰── Type
            │   │           ╰── Signed Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <253> Cast
            │               ├── Target
            │               │   ╰── Double
            │               ╰── Expression
            │                   ╰── <252> Var [s]
            ├── Function [uchar_to_int]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── u
            │   │       ╰── Type
            │   │           ╰── Unsigned Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <269> Cast
            │               ├── Target
            │               │   ╰── Int
            │               ╰── Expression
            │                   ╰── <268> Var [u]
            ├── Function [uchar_to_uint]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── u
            │   │       ╰── Type
            │   │           ╰── Unsigned Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <285> Cast
            │               ├── Target
            │               │   ╰── Unsigned Int
            │               ╰── Expression
            │                   ╰── <284> Var [u]
            ├── Function [uchar_to_long]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── u
            │   │       ╰── Type
            │   │           ╰── Unsigned Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <301> Cast
            │               ├── Target
            │               │   ╰── Long
            │               ╰── Expression
            │                   ╰── <300> Var [u]
            ├── Function [uchar_to_ulong]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── u
            │   │       ╰── Type
            │   │           ╰── Unsigned Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <317> Cast
            │               ├── Target
            │               │   ╰── Unsigned Long
            │               ╰── Expression
            │                   ╰── <316> Var [u]
            ├── Function [uchar_to_double]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── u
            │   │       ╰── Type
            │   │           ╰── Unsigned Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <333> Cast
            │               ├── Target
            │               │   ╰── Double
            │               ╰── Expression
            │                   ╰── <332> Var [u]
            ├── Function [int_to_char]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── i
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <349> Cast
            │               ├── Target
            │               │   ╰── Char
            │               ╰── Expression
            │                   ╰── <348> Var [i]
            ├── Function [uint_to_char]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── u
            │   │       ╰── Type
            │   │           ╰── Unsigned Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <365> Cast
            │               ├── Target
            │               │   ╰── Char
            │               ╰── Expression
            │                   ╰── <364> Var [u]
            ├── Function [double_to_char]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── d
            │   │       ╰── Type
            │   │           ╰── Double
            │   ╰── Body
            │       ╰── Return
            │           ╰── <381> Cast
            │               ├── Target
            │               │   ╰── Char
            │               ╰── Expression
            │                   ╰── <380> Var [d]
            ├── Function [long_to_schar]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── l
            │   │       ╰── Type
            │   │           ╰── Long
            │   ╰── Body
            │       ╰── Return
            │           ╰── <397> Cast
            │               ├── Target
            │               │   ╰── Signed Char
            │               ╰── Expression
            │                   ╰── <396> Var [l]
            ├── Function [ulong_to_schar]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── l
            │   │       ╰── Type
            │   │           ╰── Unsigned Long
            │   ╰── Body
            │       ╰── Return
            │           ╰── <413> Cast
            │               ├── Target
            │               │   ╰── Signed Char
            │               ╰── Expression
            │                   ╰── <412> Var [l]
            ├── Function [int_to_uchar]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── i
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <429> Cast
            │               ├── Target
            │               │   ╰── Unsigned Char
            │               ╰── Expression
            │                   ╰── <428> Var [i]
            ├── Function [uint_to_uchar]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── ui
            │   │       ╰── Type
            │   │           ╰── Unsigned Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <445> Cast
            │               ├── Target
            │               │   ╰── Unsigned Char
            │               ╰── Expression
            │                   ╰── <444> Var [ui]
            ├── Function [long_to_uchar]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── l
            │   │       ╰── Type
            │   │           ╰── Long
            │   ╰── Body
            │       ╰── Return
            │           ╰── <461> Cast
            │               ├── Target
            │               │   ╰── Unsigned Char
            │               ╰── Expression
            │                   ╰── <460> Var [l]
            ├── Function [ulong_to_uchar]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── ul
            │   │       ╰── Type
            │   │           ╰── Unsigned Long
            │   ╰── Body
            │       ╰── Return
            │           ╰── <477> Cast
            │               ├── Target
            │               │   ╰── Unsigned Char
            │               ╰── Expression
            │                   ╰── <476> Var [ul]
            ├── Function [double_to_uchar]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── d
            │   │       ╰── Type
            │   │           ╰── Double
            │   ╰── Body
            │       ╰── Return
            │           ╰── <493> Cast
            │               ├── Target
            │               │   ╰── Unsigned Char
            │               ╰── Expression
            │                   ╰── <492> Var [d]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── <505> Constant Int [127]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <514>  [!=]
                    │   │       ├── <511> FunctionCall [char_to_uchar]
                    │   │       │   ╰── <510> Var [c]
                    │   │       ╰── <513> Constant Int [127]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <515> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <526>  [!=]
                    │   │       ├── <523> FunctionCall [char_to_int]
                    │   │       │   ╰── <522> Var [c]
                    │   │       ╰── <525> Constant Int [127]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <527> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <538>  [!=]
                    │   │       ├── <535> FunctionCall [char_to_ulong]
                    │   │       │   ╰── <534> Var [c]
                    │   │       ╰── <537> Constant Int [127]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <539> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── sc
                    │   ├── Type
                    │   │   ╰── Signed Char
                    │   ╰── Initializer
                    │       ╰── <549> Unary [-]
                    │           ╰── <548> Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <558>  [!=]
                    │   │       ├── <555> FunctionCall [schar_to_uchar]
                    │   │       │   ╰── <554> Var [sc]
                    │   │       ╰── <557> Constant Int [246]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <559> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <572>  [!=]
                    │   │       ├── <567> FunctionCall [schar_to_long]
                    │   │       │   ╰── <566> Var [sc]
                    │   │       ╰── <571> Unary [-]
                    │   │           ╰── <570> Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <573> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <584>  [!=]
                    │   │       ├── <581> FunctionCall [schar_to_uint]
                    │   │       │   ╰── <580> Var [sc]
                    │   │       ╰── <583> Constant UInt [4294967286]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <585> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <598>  [!=]
                    │   │       ├── <593> FunctionCall [schar_to_double]
                    │   │       │   ╰── <592> Var [sc]
                    │   │       ╰── <597> Unary [-]
                    │   │           ╰── <596> Constant Double [+1e1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <599> Constant Int [7]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── uc
                    │   ├── Type
                    │   │   ╰── Unsigned Char
                    │   ╰── Initializer
                    │       ╰── <607> Constant Int [250]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <616>  [!=]
                    │   │       ├── <613> FunctionCall [uchar_to_int]
                    │   │       │   ╰── <612> Var [uc]
                    │   │       ╰── <615> Constant Int [250]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <617> Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <628>  [!=]
                    │   │       ├── <625> FunctionCall [uchar_to_long]
                    │   │       │   ╰── <624> Var [uc]
                    │   │       ╰── <627> Constant Int [250]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <629> Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <640>  [!=]
                    │   │       ├── <637> FunctionCall [uchar_to_uint]
                    │   │       │   ╰── <636> Var [uc]
                    │   │       ╰── <639> Constant Int [250]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <641> Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <652>  [!=]
                    │   │       ├── <649> FunctionCall [uchar_to_ulong]
                    │   │       │   ╰── <648> Var [uc]
                    │   │       ╰── <651> Constant Int [250]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <653> Constant Int [11]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <664>  [!=]
                    │   │       ├── <661> FunctionCall [uchar_to_double]
                    │   │       │   ╰── <660> Var [uc]
                    │   │       ╰── <663> Constant Double [+2.5e2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <665> Constant Int [12]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <678>  [!=]
                    │   │       ├── <673> FunctionCall [uchar_to_schar]
                    │   │       │   ╰── <672> Var [uc]
                    │   │       ╰── <677> Unary [-]
                    │   │           ╰── <676> Constant Int [6]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <679> Constant Int [13]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <692>  [!=]
                    │   │       ├── <687> FunctionCall [uchar_to_char]
                    │   │       │   ╰── <686> Var [uc]
                    │   │       ╰── <691> Unary [-]
                    │   │           ╰── <690> Constant Int [6]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <693> Constant Int [14]
                    ├── <707> Assign [=]
                    │   ├── <699> Var [c]
                    │   ╰── <706> Cast
                    │       ├── Target
                    │       │   ╰── Char
                    │       ╰── Expression
                    │           ╰── <705> Unary [-]
                    │               ╰── <704> Constant Int [128]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <715>  [!=]
                    │   │       ├── <711> FunctionCall [int_to_char]
                    │   │       │   ╰── <710> Constant Int [128]
                    │   │       ╰── <714> Var [c]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <716> Constant Int [15]
                    ├── <730> Assign [=]
                    │   ├── <722> Var [c]
                    │   ╰── <729> Cast
                    │       ├── Target
                    │       │   ╰── Char
                    │       ╰── Expression
                    │           ╰── <728> Unary [-]
                    │               ╰── <727> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <738>  [!=]
                    │   │       ├── <734> FunctionCall [uint_to_char]
                    │   │       │   ╰── <733> Constant UInt [2147483898]
                    │   │       ╰── <737> Var [c]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <739> Constant Int [16]
                    ├── <753> Assign [=]
                    │   ├── <745> Var [c]
                    │   ╰── <752> Cast
                    │       ├── Target
                    │       │   ╰── Char
                    │       ╰── Expression
                    │           ╰── <751> Unary [-]
                    │               ╰── <750> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <763>  [!=]
                    │   │       ├── <759> FunctionCall [double_to_char]
                    │   │       │   ╰── <758> Unary [-]
                    │   │       │       ╰── <757> Constant Double [+2.6e0]
                    │   │       ╰── <762> Var [c]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <764> Constant Int [17]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <771> FunctionCall [long_to_schar]
                    │   │       ╰── <770> Constant Long [17592186044416]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <772> Constant Int [18]
                    ├── <786> Assign [=]
                    │   ├── <778> Var [sc]
                    │   ╰── <785> Cast
                    │       ├── Target
                    │       │   ╰── Signed Char
                    │       ╰── Expression
                    │           ╰── <784> Unary [-]
                    │               ╰── <783> Constant Int [126]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <794>  [!=]
                    │   │       ├── <790> FunctionCall [ulong_to_schar]
                    │   │       │   ╰── <789> Constant ULong [9224497936761618562]
                    │   │       ╰── <793> Var [sc]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <795> Constant Int [19]
                    ├── <807> Assign [=]
                    │   ├── <801> Var [uc]
                    │   ╰── <806> Cast
                    │       ├── Target
                    │       │   ╰── Unsigned Char
                    │       ╰── Expression
                    │           ╰── <805> Constant Int [200]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <817>  [!=]
                    │   │       ├── <813> FunctionCall [int_to_uchar]
                    │   │       │   ╰── <812> Unary [-]
                    │   │       │       ╰── <811> Constant Int [1234488]
                    │   │       ╰── <816> Var [uc]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <818> Constant Int [20]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <829>  [!=]
                    │   │       ├── <825> FunctionCall [uint_to_uchar]
                    │   │       │   ╰── <824> Constant Long [4293732808]
                    │   │       ╰── <828> Var [uc]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <830> Constant Int [21]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <843>  [!=]
                    │   │       ├── <839> FunctionCall [long_to_uchar]
                    │   │       │   ╰── <838> Unary [-]
                    │   │       │       ╰── <837> Constant Long [36283884951096]
                    │   │       ╰── <842> Var [uc]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <844> Constant Int [22]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <855>  [!=]
                    │   │       ├── <851> FunctionCall [ulong_to_uchar]
                    │   │       │   ╰── <850> Constant ULong [9224497936761618632]
                    │   │       ╰── <854> Var [uc]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <856> Constant Int [23]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <867>  [!=]
                    │   │       ├── <863> FunctionCall [double_to_uchar]
                    │   │       │   ╰── <862> Constant Double [+2.0099e2]
                    │   │       ╰── <866> Var [uc]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <868> Constant Int [24]
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
                    │       ╰── <887> Cast
                    │           ├── Target
                    │           │   ╰── Char
                    │           ╰── Expression
                    │               ╰── <886> Var [null_ptr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <891> Var [zero]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <892> Constant Int [25]
                    ├── <901> Assign [=]
                    │   ├── <898> Var [c]
                    │   ╰── <900> Constant Int [32]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <913> Cast
                    │           ├── Target
                    │           │   ╰── Pointer
                    │           │       ╰── Int
                    │           ╰── Expression
                    │               ╰── <912> Var [c]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <924>  [!=]
                    │   │       ├── <920> Cast
                    │   │       │   ├── Target
                    │   │       │   │   ╰── Char
                    │   │       │   ╰── Expression
                    │   │       │       ╰── <919> Var [i]
                    │   │       ╰── <923> Var [c]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <925> Constant Int [26]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <939>  [!=]
                    │   │       ├── <933> Cast
                    │   │       │   ├── Target
                    │   │       │   │   ╰── Char
                    │   │       │   ╰── Expression
                    │   │       │       ╰── <932> Constant Int [300]
                    │   │       ╰── <938> Cast
                    │   │           ├── Target
                    │   │           │   ╰── Char
                    │   │           ╰── Expression
                    │   │               ╰── <937> Constant Int [44]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <940> Constant Int [27]
                    ╰── Return
                        ╰── <945> Constant Int [0]
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
            │           ╰── <24>  [+]
            │               ├── <20>  [+]
            │               │   ├── <16> Var [c1]
            │               │   ╰── <19> Var [c2]
            │               ╰── <23> Var [c3]
            ├── Function [negate]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── uc
            │   │       ╰── Type
            │   │           ╰── Unsigned Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <39> Unary [-]
            │               ╰── <38> Var [uc]
            ├── Function [complement]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── uc
            │   │       ╰── Type
            │   │           ╰── Unsigned Char
            │   ╰── Body
            │       ╰── Return
            │           ╰── <54> Unary [~]
            │               ╰── <53> Var [uc]
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
            │           ╰── <82>  [/]
            │               ├── <78>  [+]
            │               │   ├── <73> Var [a]
            │               │   ╰── <76> Var [b]
            │               ╰── <81> Var [c]
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
            │           ╰── <102>  [*]
            │               ├── <98> Var [s]
            │               ╰── <101> Var [u]
            ├── Function [decrement]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── s
            │   │       ╰── Type
            │   │           ╰── Signed Char
            │   ╰── Body
            │       ├── <122> Assign [=]
            │       │   ├── <115> Var [s]
            │       │   ╰── <121>  [-]
            │       │       ├── <118> Var [s]
            │       │       ╰── <120> Constant Int [1]
            │       ╰── Return
            │           ╰── <125> Var [s]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── <137> Constant Int [100]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── <143> Constant Int [109]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <156>  [!=]
                    │   │       ├── <153> FunctionCall [add_chars]
                    │   │       │   ├── <148> Var [a]
                    │   │       │   ├── <150> Var [a]
                    │   │       │   ╰── <152> Var [b]
                    │   │       ╰── <155> Constant Int [309]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <157> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── one
                    │   ├── Type
                    │   │   ╰── Unsigned Char
                    │   ╰── Initializer
                    │       ╰── <165> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <176>  [!=]
                    │   │       ├── <171> FunctionCall [negate]
                    │   │       │   ╰── <170> Var [one]
                    │   │       ╰── <175> Unary [-]
                    │   │           ╰── <174> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <177> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <190>  [!=]
                    │   │       ├── <185> FunctionCall [complement]
                    │   │       │   ╰── <184> Var [one]
                    │   │       ╰── <189> Unary [-]
                    │   │           ╰── <188> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <191> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── w
                    │   ├── Type
                    │   │   ╰── Signed Char
                    │   ╰── Initializer
                    │       ╰── <199> Constant Int [127]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Signed Char
                    │   ╰── Initializer
                    │       ╰── <205> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── y
                    │   ├── Type
                    │   │   ╰── Signed Char
                    │   ╰── Initializer
                    │       ╰── <211> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <224>  [!=]
                    │   │       ├── <221> FunctionCall [add_then_div]
                    │   │       │   ├── <216> Var [w]
                    │   │       │   ├── <218> Var [x]
                    │   │       │   ╰── <220> Var [y]
                    │   │       ╰── <223> Constant Int [65]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <225> Constant Int [4]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── sc
                    │   ├── Type
                    │   │   ╰── Signed Char
                    │   ╰── Initializer
                    │       ╰── <233> Unary [-]
                    │           ╰── <232> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── uc
                    │   ├── Type
                    │   │   ╰── Unsigned Char
                    │   ╰── Initializer
                    │       ╰── <239> Constant Int [250]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <252>  [!=]
                    │   │       ├── <247> FunctionCall [mixed_multiply]
                    │   │       │   ├── <244> Var [sc]
                    │   │       │   ╰── <246> Var [uc]
                    │   │       ╰── <251> Unary [-]
                    │   │           ╰── <250> Constant Int [750]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <253> Constant Int [5]
                    ├── <262> Assign [=]
                    │   ├── <257> Var [sc]
                    │   ╰── <261> Unary [-]
                    │       ╰── <260> Constant Int [128]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <270>  [!=]
                    │   │       ├── <265> Var [sc]
                    │   │       ╰── <269> Unary [-]
                    │   │           ╰── <268> Constant Int [128]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <271> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <282>  [!=]
                    │   │       ├── <279> FunctionCall [decrement]
                    │   │       │   ╰── <278> Var [sc]
                    │   │       ╰── <281> Constant Int [127]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <283> Constant Int [7]
                    ╰── Return
                        ╰── <288> Constant Int [0]
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
            │           ├── <7> Constant Int [1]
            │           ╰── <9> Constant Int [2]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── static2
            │   ├── Type
            │   │   ╰── Array
            │   │       ├── 4
            │   │       ╰── Signed Char
            │   ╰── Initializer
            │       ╰── Compound
            │           ├── <19> Constant Int [3]
            │           ╰── <21> Constant Int [4]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── static3
            │   ├── Type
            │   │   ╰── Array
            │   │       ├── 3
            │   │       ╰── Unsigned Char
            │   ╰── Initializer
            │       ╰── Compound
            │           ╰── <31> Constant Int [5]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <67>  [||]
                    │   │       ├── <61>  [||]
                    │   │       │   ├── <55>  [||]
                    │   │       │   │   ├── <46>  [!=]
                    │   │       │   │   │   ├── <43> Subscript
                    │   │       │   │   │   │   ├── <41> Var [static1]
                    │   │       │   │   │   │   ╰── <42> Constant Int [0]
                    │   │       │   │   │   ╰── <45> Constant Int [1]
                    │   │       │   │   ╰── <54>  [!=]
                    │   │       │   │       ├── <51> Subscript
                    │   │       │   │       │   ├── <49> Var [static1]
                    │   │       │   │       │   ╰── <50> Constant Int [1]
                    │   │       │   │       ╰── <53> Constant Int [2]
                    │   │       │   ╰── <60> Subscript
                    │   │       │       ├── <58> Var [static1]
                    │   │       │       ╰── <59> Constant Int [2]
                    │   │       ╰── <66> Subscript
                    │   │           ├── <64> Var [static1]
                    │   │           ╰── <65> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <68> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <98>  [||]
                    │   │       ├── <92>  [||]
                    │   │       │   ├── <86>  [||]
                    │   │       │   │   ├── <77>  [!=]
                    │   │       │   │   │   ├── <74> Subscript
                    │   │       │   │   │   │   ├── <72> Var [static2]
                    │   │       │   │   │   │   ╰── <73> Constant Int [0]
                    │   │       │   │   │   ╰── <76> Constant Int [3]
                    │   │       │   │   ╰── <85>  [!=]
                    │   │       │   │       ├── <82> Subscript
                    │   │       │   │       │   ├── <80> Var [static2]
                    │   │       │   │       │   ╰── <81> Constant Int [1]
                    │   │       │   │       ╰── <84> Constant Int [4]
                    │   │       │   ╰── <91> Subscript
                    │   │       │       ├── <89> Var [static2]
                    │   │       │       ╰── <90> Constant Int [2]
                    │   │       ╰── <97> Subscript
                    │   │           ├── <95> Var [static2]
                    │   │           ╰── <96> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <99> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <120>  [||]
                    │   │       ├── <114>  [||]
                    │   │       │   ├── <108>  [!=]
                    │   │       │   │   ├── <105> Subscript
                    │   │       │   │   │   ├── <103> Var [static3]
                    │   │       │   │   │   ╰── <104> Constant Int [0]
                    │   │       │   │   ╰── <107> Constant Int [5]
                    │   │       │   ╰── <113> Subscript
                    │   │       │       ├── <111> Var [static3]
                    │   │       │       ╰── <112> Constant Int [1]
                    │   │       ╰── <119> Subscript
                    │   │           ├── <117> Var [static3]
                    │   │           ╰── <118> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <121> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── auto1
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 5
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <132> Unary [-]
                    │           │   ╰── <131> Constant Int [4]
                    │           ├── <134> Constant Int [66]
                    │           ╰── <136> Constant Double [+4e0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── auto2
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 3
                    │   │       ╰── Signed Char
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <149> Subscript
                    │           │   ├── <147> Var [static1]
                    │           │   ╰── <148> Constant Int [2]
                    │           ╰── <156> Unary [-]
                    │               ╰── <155> Subscript
                    │                   ├── <153> Var [static1]
                    │                   ╰── <154> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── auto3
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 2
                    │   │       ╰── Unsigned Char
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ╰── <166> Constant Int [97]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <208>  [||]
                    │   │       ├── <202>  [||]
                    │   │       │   ├── <196>  [||]
                    │   │       │   │   ├── <187>  [||]
                    │   │       │   │   │   ├── <178>  [!=]
                    │   │       │   │   │   │   ├── <173> Subscript
                    │   │       │   │   │   │   │   ├── <171> Var [auto1]
                    │   │       │   │   │   │   │   ╰── <172> Constant Int [0]
                    │   │       │   │   │   │   ╰── <177> Unary [-]
                    │   │       │   │   │   │       ╰── <176> Constant Int [4]
                    │   │       │   │   │   ╰── <186>  [!=]
                    │   │       │   │   │       ├── <183> Subscript
                    │   │       │   │   │       │   ├── <181> Var [auto1]
                    │   │       │   │   │       │   ╰── <182> Constant Int [1]
                    │   │       │   │   │       ╰── <185> Constant Int [66]
                    │   │       │   │   ╰── <195>  [!=]
                    │   │       │   │       ├── <192> Subscript
                    │   │       │   │       │   ├── <190> Var [auto1]
                    │   │       │   │       │   ╰── <191> Constant Int [2]
                    │   │       │   │       ╰── <194> Constant Int [4]
                    │   │       │   ╰── <201> Subscript
                    │   │       │       ├── <199> Var [auto1]
                    │   │       │       ╰── <200> Constant Int [3]
                    │   │       ╰── <207> Subscript
                    │   │           ├── <205> Var [auto1]
                    │   │           ╰── <206> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <209> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <232>  [||]
                    │   │       ├── <226>  [||]
                    │   │       │   ├── <215> Subscript
                    │   │       │   │   ├── <213> Var [auto2]
                    │   │       │   │   ╰── <214> Constant Int [0]
                    │   │       │   ╰── <225>  [!=]
                    │   │       │       ├── <220> Subscript
                    │   │       │       │   ├── <218> Var [auto2]
                    │   │       │       │   ╰── <219> Constant Int [1]
                    │   │       │       ╰── <224> Unary [-]
                    │   │       │           ╰── <223> Constant Int [1]
                    │   │       ╰── <231> Subscript
                    │   │           ├── <229> Var [auto2]
                    │   │           ╰── <230> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <233> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <248>  [||]
                    │   │       ├── <242>  [!=]
                    │   │       │   ├── <239> Subscript
                    │   │       │   │   ├── <237> Var [auto3]
                    │   │       │   │   ╰── <238> Constant Int [0]
                    │   │       │   ╰── <241> Constant Int [97]
                    │   │       ╰── <247> Subscript
                    │   │           ├── <245> Var [auto3]
                    │   │           ╰── <246> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <249> Constant Int [6]
                    ╰── Return
                        ╰── <252> Constant Int [0]
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
            │           ╰── <36>  [+]
            │               ├── <33> Var [g]
            │               ╰── <35> Constant Int [1]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <54> FunctionCall [foo]
                            ├── <46> Constant Int [0]
                            ├── <47> Constant Int [0]
                            ├── <48> Constant Int [0]
                            ├── <49> Constant Int [0]
                            ├── <50> Constant Int [0]
                            ├── <51> Constant Int [0]
                            ╰── <53> Var [zed]
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
            │           ╰── <6> Constant Long [5369233654]
            ├── Function [return_schar]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <15> Constant Long [5369233654]
            ├── Function [return_uchar]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <24> Constant Long [5369233654]
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
                    │           ├── <39> Constant Int [121]
                    │           ├── <43> Unary [-]
                    │           │   ╰── <42> Constant Int [122]
                    │           ╰── <47> Unary [-]
                    │               ╰── <46> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── retval_c
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── <55> FunctionCall [return_char]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── char_array2
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 3
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <66> Unary [-]
                    │           │   ╰── <65> Constant Int [5]
                    │           ├── <68> Constant Int [88]
                    │           ╰── <72> Unary [-]
                    │               ╰── <71> Constant Int [100]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── retval_sc
                    │   ├── Type
                    │   │   ╰── Signed Char
                    │   ╰── Initializer
                    │       ╰── <80> FunctionCall [return_schar]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── char_array3
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 3
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <89> Constant Int [10]
                    │           ├── <91> Constant Int [11]
                    │           ╰── <93> Constant Int [12]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── retval_uc
                    │   ├── Type
                    │   │   ╰── Unsigned Char
                    │   ╰── Initializer
                    │       ╰── <101> FunctionCall [return_uchar]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── char_array4
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 2
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <112> Unary [-]
                    │           │   ╰── <111> Constant Int [5]
                    │           ╰── <116> Unary [-]
                    │               ╰── <115> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <148>  [||]
                    │   │       ├── <137>  [||]
                    │   │       │   ├── <126>  [!=]
                    │   │       │   │   ├── <123> Subscript
                    │   │       │   │   │   ├── <121> Var [char_array]
                    │   │       │   │   │   ╰── <122> Constant Int [0]
                    │   │       │   │   ╰── <125> Constant Int [121]
                    │   │       │   ╰── <136>  [!=]
                    │   │       │       ├── <131> Subscript
                    │   │       │       │   ├── <129> Var [char_array]
                    │   │       │       │   ╰── <130> Constant Int [1]
                    │   │       │       ╰── <135> Unary [-]
                    │   │       │           ╰── <134> Constant Int [122]
                    │   │       ╰── <147>  [!=]
                    │   │           ├── <142> Subscript
                    │   │           │   ├── <140> Var [char_array]
                    │   │           │   ╰── <141> Constant Int [2]
                    │   │           ╰── <146> Unary [-]
                    │   │               ╰── <145> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <149> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <160>  [!=]
                    │   │       ├── <155> Var [retval_c]
                    │   │       ╰── <159> Unary [-]
                    │   │           ╰── <158> Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <161> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <194>  [||]
                    │   │       ├── <183>  [||]
                    │   │       │   ├── <174>  [!=]
                    │   │       │   │   ├── <169> Subscript
                    │   │       │   │   │   ├── <167> Var [char_array2]
                    │   │       │   │   │   ╰── <168> Constant Int [0]
                    │   │       │   │   ╰── <173> Unary [-]
                    │   │       │   │       ╰── <172> Constant Int [5]
                    │   │       │   ╰── <182>  [!=]
                    │   │       │       ├── <179> Subscript
                    │   │       │       │   ├── <177> Var [char_array2]
                    │   │       │       │   ╰── <178> Constant Int [1]
                    │   │       │       ╰── <181> Constant Int [88]
                    │   │       ╰── <193>  [!=]
                    │   │           ├── <188> Subscript
                    │   │           │   ├── <186> Var [char_array2]
                    │   │           │   ╰── <187> Constant Int [2]
                    │   │           ╰── <192> Unary [-]
                    │   │               ╰── <191> Constant Int [100]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <195> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <206>  [!=]
                    │   │       ├── <201> Var [retval_sc]
                    │   │       ╰── <205> Unary [-]
                    │   │           ╰── <204> Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <207> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <236>  [||]
                    │   │       ├── <227>  [||]
                    │   │       │   ├── <218>  [!=]
                    │   │       │   │   ├── <215> Subscript
                    │   │       │   │   │   ├── <213> Var [char_array3]
                    │   │       │   │   │   ╰── <214> Constant Int [0]
                    │   │       │   │   ╰── <217> Constant Int [10]
                    │   │       │   ╰── <226>  [!=]
                    │   │       │       ├── <223> Subscript
                    │   │       │       │   ├── <221> Var [char_array3]
                    │   │       │       │   ╰── <222> Constant Int [1]
                    │   │       │       ╰── <225> Constant Int [11]
                    │   │       ╰── <235>  [!=]
                    │   │           ├── <232> Subscript
                    │   │           │   ├── <230> Var [char_array3]
                    │   │           │   ╰── <231> Constant Int [2]
                    │   │           ╰── <234> Constant Int [12]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <237> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <246>  [!=]
                    │   │       ├── <243> Var [retval_uc]
                    │   │       ╰── <245> Constant Int [246]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <247> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <271>  [||]
                    │   │       ├── <260>  [!=]
                    │   │       │   ├── <255> Subscript
                    │   │       │   │   ├── <253> Var [char_array4]
                    │   │       │   │   ╰── <254> Constant Int [0]
                    │   │       │   ╰── <259> Unary [-]
                    │   │       │       ╰── <258> Constant Int [5]
                    │   │       ╰── <270>  [!=]
                    │   │           ├── <265> Subscript
                    │   │           │   ├── <263> Var [char_array4]
                    │   │           │   ╰── <264> Constant Int [1]
                    │   │           ╰── <269> Unary [-]
                    │   │               ╰── <268> Constant Int [6]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <272> Constant Int [7]
                    ╰── Return
                        ╰── <277> Constant Int [0]
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
            │       ╰── <49> Constant Int [5]
            ├── Function [main]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── should_spill
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <64> Cast
            │       │           ├── Target
            │       │           │   ╰── Int
            │       │           ╰── Expression
            │       │               ╰── <63> Var [glob]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── one
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <74>  [-]
            │       │           ├── <71> Var [glob]
            │       │           ╰── <73> Constant Int [4]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── two
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <85>  [+]
            │       │           ├── <81> Var [one]
            │       │           ╰── <84> Var [one]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── three
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <95>  [+]
            │       │           ├── <91> Constant Int [2]
            │       │           ╰── <94> Var [one]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── four
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <106>  [*]
            │       │           ├── <102> Var [two]
            │       │           ╰── <105> Var [two]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── five
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <116>  [-]
            │       │           ├── <112> Constant Int [6]
            │       │           ╰── <115> Var [one]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── six
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <127>  [*]
            │       │           ├── <123> Var [two]
            │       │           ╰── <126> Var [three]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── seven
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <137>  [+]
            │       │           ├── <134> Var [one]
            │       │           ╰── <136> Constant Int [6]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── eight
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <147>  [*]
            │       │           ├── <144> Var [two]
            │       │           ╰── <146> Constant Int [4]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── nine
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <158>  [*]
            │       │           ├── <154> Var [three]
            │       │           ╰── <157> Var [three]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ten
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <169>  [+]
            │       │           ├── <165> Var [four]
            │       │           ╰── <168> Var [six]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── eleven
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <179>  [-]
            │       │           ├── <175> Constant Int [16]
            │       │           ╰── <178> Var [five]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── twelve
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <190>  [+]
            │       │           ├── <186> Var [six]
            │       │           ╰── <189> Var [six]
            │       ├── <219> FunctionCall [check_12_ints]
            │       │   ├── <195> Var [one]
            │       │   ├── <197> Var [two]
            │       │   ├── <199> Var [three]
            │       │   ├── <201> Var [four]
            │       │   ├── <203> Var [five]
            │       │   ├── <205> Var [six]
            │       │   ├── <207> Var [seven]
            │       │   ├── <209> Var [eight]
            │       │   ├── <211> Var [nine]
            │       │   ├── <213> Var [ten]
            │       │   ├── <215> Var [eleven]
            │       │   ├── <217> Var [twelve]
            │       │   ╰── <218> Constant Int [1]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── thirteen
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <228>  [+]
            │       │           ├── <224> Constant Int [8]
            │       │           ╰── <227> Var [glob]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── fourteen
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <238>  [+]
            │       │           ├── <235> Var [thirteen]
            │       │           ╰── <237> Constant Int [1]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── fifteen
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <248>  [-]
            │       │           ├── <244> Constant Int [28]
            │       │           ╰── <247> Var [thirteen]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── sixteen
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <258>  [+]
            │       │           ├── <255> Var [fourteen]
            │       │           ╰── <257> Constant Int [2]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── seventeen
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <268>  [+]
            │       │           ├── <264> Constant Int [4]
            │       │           ╰── <267> Var [thirteen]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── eighteen
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <278>  [-]
            │       │           ├── <274> Constant Int [32]
            │       │           ╰── <277> Var [fourteen]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── nineteen
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <288>  [-]
            │       │           ├── <284> Constant Int [35]
            │       │           ╰── <287> Var [sixteen]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── twenty
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <298>  [+]
            │       │           ├── <295> Var [fifteen]
            │       │           ╰── <297> Constant Int [5]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── twenty_one
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <311>  [-]
            │       │           ├── <308>  [*]
            │       │           │   ├── <305> Var [thirteen]
            │       │           │   ╰── <307> Constant Int [2]
            │       │           ╰── <310> Constant Int [5]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── twenty_two
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <321>  [+]
            │       │           ├── <318> Var [fifteen]
            │       │           ╰── <320> Constant Int [7]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── twenty_three
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <331>  [+]
            │       │           ├── <327> Constant Int [6]
            │       │           ╰── <330> Var [seventeen]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── twenty_four
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <341>  [+]
            │       │           ├── <338> Var [thirteen]
            │       │           ╰── <340> Constant Int [11]
            │       ├── <370> FunctionCall [check_12_ints]
            │       │   ├── <346> Var [thirteen]
            │       │   ├── <348> Var [fourteen]
            │       │   ├── <350> Var [fifteen]
            │       │   ├── <352> Var [sixteen]
            │       │   ├── <354> Var [seventeen]
            │       │   ├── <356> Var [eighteen]
            │       │   ├── <358> Var [nineteen]
            │       │   ├── <360> Var [twenty]
            │       │   ├── <362> Var [twenty_one]
            │       │   ├── <364> Var [twenty_two]
            │       │   ├── <366> Var [twenty_three]
            │       │   ├── <368> Var [twenty_four]
            │       │   ╰── <369> Constant Int [13]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <376>  [!=]
            │       │   │       ├── <373> Var [should_spill]
            │       │   │       ╰── <375> Constant Int [5]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <379> Unary [-]
            │       │                   ╰── <378> Constant Int [1]
            │       ╰── Return
            │           ╰── <384> Constant Int [0]
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
                    │       ╰── <435> Constant Int [0]
                    ├── <446> Assign [=]
                    │   ├── <439> Var [expected]
                    │   ╰── <445>  [+]
                    │       ├── <442> Var [start]
                    │       ╰── <444> Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <453>  [!=]
                    │   │       ├── <449> Var [a]
                    │   │       ╰── <452> Var [expected]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <455> Var [expected]
                    ├── <468> Assign [=]
                    │   ├── <461> Var [expected]
                    │   ╰── <467>  [+]
                    │       ├── <464> Var [start]
                    │       ╰── <466> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <475>  [!=]
                    │   │       ├── <471> Var [b]
                    │   │       ╰── <474> Var [expected]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <477> Var [expected]
                    ├── <490> Assign [=]
                    │   ├── <483> Var [expected]
                    │   ╰── <489>  [+]
                    │       ├── <486> Var [start]
                    │       ╰── <488> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <497>  [!=]
                    │   │       ├── <493> Var [c]
                    │   │       ╰── <496> Var [expected]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <499> Var [expected]
                    ├── <512> Assign [=]
                    │   ├── <505> Var [expected]
                    │   ╰── <511>  [+]
                    │       ├── <508> Var [start]
                    │       ╰── <510> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <519>  [!=]
                    │   │       ├── <515> Var [d]
                    │   │       ╰── <518> Var [expected]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <521> Var [expected]
                    ├── <534> Assign [=]
                    │   ├── <527> Var [expected]
                    │   ╰── <533>  [+]
                    │       ├── <530> Var [start]
                    │       ╰── <532> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <541>  [!=]
                    │   │       ├── <537> Var [e]
                    │   │       ╰── <540> Var [expected]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <543> Var [expected]
                    ├── <556> Assign [=]
                    │   ├── <549> Var [expected]
                    │   ╰── <555>  [+]
                    │       ├── <552> Var [start]
                    │       ╰── <554> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <563>  [!=]
                    │   │       ├── <559> Var [f]
                    │   │       ╰── <562> Var [expected]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <565> Var [expected]
                    ├── <578> Assign [=]
                    │   ├── <571> Var [expected]
                    │   ╰── <577>  [+]
                    │       ├── <574> Var [start]
                    │       ╰── <576> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <585>  [!=]
                    │   │       ├── <581> Var [g]
                    │   │       ╰── <584> Var [expected]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <587> Var [expected]
                    ├── <600> Assign [=]
                    │   ├── <593> Var [expected]
                    │   ╰── <599>  [+]
                    │       ├── <596> Var [start]
                    │       ╰── <598> Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <607>  [!=]
                    │   │       ├── <603> Var [h]
                    │   │       ╰── <606> Var [expected]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <609> Var [expected]
                    ├── <622> Assign [=]
                    │   ├── <615> Var [expected]
                    │   ╰── <621>  [+]
                    │       ├── <618> Var [start]
                    │       ╰── <620> Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <629>  [!=]
                    │   │       ├── <625> Var [i]
                    │   │       ╰── <628> Var [expected]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <631> Var [expected]
                    ├── <644> Assign [=]
                    │   ├── <637> Var [expected]
                    │   ╰── <643>  [+]
                    │       ├── <640> Var [start]
                    │       ╰── <642> Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <651>  [!=]
                    │   │       ├── <647> Var [j]
                    │   │       ╰── <650> Var [expected]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <653> Var [expected]
                    ├── <666> Assign [=]
                    │   ├── <659> Var [expected]
                    │   ╰── <665>  [+]
                    │       ├── <662> Var [start]
                    │       ╰── <664> Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <673>  [!=]
                    │   │       ├── <669> Var [k]
                    │   │       ╰── <672> Var [expected]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <675> Var [expected]
                    ├── <688> Assign [=]
                    │   ├── <681> Var [expected]
                    │   ╰── <687>  [+]
                    │       ├── <684> Var [start]
                    │       ╰── <686> Constant Int [11]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <695>  [!=]
                    │   │       ├── <691> Var [l]
                    │   │       ╰── <694> Var [expected]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <697> Var [expected]
                    ╰── Return
                        ╰── <702> Constant Int [0]
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
            │       ╰── <4> Constant Long [17592186044416]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── from_double
            │   ├── Type
            │   │   ╰── Char
            │   ╰── Initializer
            │       ╰── <10> Constant Double [+1.56e1]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── from_uint
            │   ├── Type
            │   │   ╰── Char
            │   ╰── Initializer
            │       ╰── <16> Constant UInt [2147483777]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── from_ulong
            │   ├── Type
            │   │   ╰── Char
            │   ╰── Initializer
            │       ╰── <22> Constant ULong [9223372037928517642]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── schar_from_long
            │   ├── Type
            │   │   ╰── Signed Char
            │   ╰── Initializer
            │       ╰── <28> Constant Long [17592186044419]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── schar_from_uint
            │   ├── Type
            │   │   ╰── Signed Char
            │   ╰── Initializer
            │       ╰── <34> Constant UInt [2147483898]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── schar_from_ulong
            │   ├── Type
            │   │   ╰── Signed Char
            │   ╰── Initializer
            │       ╰── <40> Constant ULong [9223372037928517642]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── schar_from_double
            │   ├── Type
            │   │   ╰── Signed Char
            │   ╰── Initializer
            │       ╰── <46> Constant Double [+1e-10]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── uchar_from_int
            │   ├── Type
            │   │   ╰── Unsigned Char
            │   ╰── Initializer
            │       ╰── <52> Constant Int [13526]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── uchar_from_uint
            │   ├── Type
            │   │   ╰── Unsigned Char
            │   ╰── Initializer
            │       ╰── <58> Constant UInt [2147483898]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── uchar_from_long
            │   ├── Type
            │   │   ╰── Unsigned Char
            │   ╰── Initializer
            │       ╰── <64> Constant Long [1101659111674]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── uchar_from_ulong
            │   ├── Type
            │   │   ╰── Unsigned Char
            │   ╰── Initializer
            │       ╰── <70> Constant ULong [9223372037928517642]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── uchar_from_double
            │   ├── Type
            │   │   ╰── Unsigned Char
            │   ╰── Initializer
            │       ╰── <76> Constant Double [+7.77e1]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <88>  [!=]
                    │   │       ├── <85> Var [from_long]
                    │   │       ╰── <87> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <89> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <98>  [!=]
                    │   │       ├── <95> Var [from_double]
                    │   │       ╰── <97> Constant Int [15]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <99> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <110>  [!=]
                    │   │       ├── <105> Var [from_uint]
                    │   │       ╰── <109> Unary [-]
                    │   │           ╰── <108> Constant Int [127]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <111> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <120>  [!=]
                    │   │       ├── <117> Var [from_ulong]
                    │   │       ╰── <119> Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <121> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <132>  [!=]
                    │   │       ├── <127> Var [schar_from_uint]
                    │   │       ╰── <131> Unary [-]
                    │   │           ╰── <130> Constant Int [6]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <133> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <142>  [!=]
                    │   │       ├── <139> Var [schar_from_ulong]
                    │   │       ╰── <141> Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <143> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <152>  [!=]
                    │   │       ├── <149> Var [schar_from_double]
                    │   │       ╰── <151> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <153> Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <162>  [!=]
                    │   │       ├── <159> Var [uchar_from_int]
                    │   │       ╰── <161> Constant Int [214]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <163> Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <172>  [!=]
                    │   │       ├── <169> Var [uchar_from_uint]
                    │   │       ╰── <171> Constant Int [250]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <173> Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <182>  [!=]
                    │   │       ├── <179> Var [uchar_from_ulong]
                    │   │       ╰── <181> Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <183> Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <192>  [!=]
                    │   │       ├── <189> Var [uchar_from_double]
                    │   │       ╰── <191> Constant Int [77]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <193> Constant Int [11]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <202>  [!=]
                    │   │       ├── <199> Var [schar_from_long]
                    │   │       ╰── <201> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <203> Constant Int [12]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <212>  [!=]
                    │   │       ├── <209> Var [uchar_from_long]
                    │   │       ╰── <211> Constant Int [250]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <213> Constant Int [13]
                    ╰── Return
                        ╰── <218> Constant Int [0]
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
            │   │   ╰── <5> Constant Int [10]
            │   ╰── Static
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── b
            │   ├── Type
            │   │   ╰── Unsigned Char
            │   ├── Initializer
            │   │   ╰── <12> Constant Int [20]
            │   ╰── Static
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── c
            │   ├── Type
            │   │   ╰── Char
            │   ╰── Initializer
            │       ╰── <18> Constant Int [30]
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
                    │   │   ╰── <45>  [!=]
                    │   │       ├── <42> Var [a]
                    │   │       ╰── <44> Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <46> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <55>  [!=]
                    │   │       ├── <52> Var [b]
                    │   │       ╰── <54> Constant Int [20]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <56> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <65>  [!=]
                    │   │       ├── <62> Var [c]
                    │   │       ╰── <64> Constant Int [30]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <66> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── loop_counter
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <74> Constant Int [0]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── d
                    │   │       ├── Type
                    │   │       │   ╰── Unsigned Char
                    │   │       ╰── Initializer
                    │   │           ╰── <80> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <88>  [<]
                    │   │       ├── <85> Var [d]
                    │   │       ╰── <87> Constant Int [100]
                    │   ├── Condition
                    │   │   ╰── <97> Assign [=]
                    │   │       ├── <90> Var [d]
                    │   │       ╰── <96>  [+]
                    │   │           ├── <93> Var [d]
                    │   │           ╰── <95> Constant Int [1]
                    │   ╰── Block
                    │       ╰── <106> Assign [=]
                    │           ├── <99> Var [loop_counter]
                    │           ╰── <105>  [+]
                    │               ├── <102> Var [loop_counter]
                    │               ╰── <104> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <115>  [!=]
                    │   │       ├── <112> Var [loop_counter]
                    │   │       ╰── <114> Constant Int [100]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <116> Constant Int [4]
                    ╰── Return
                        ╰── <121> Constant Int [0]
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
                    │       ╰── <9> Constant Int [255]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <20>  [!=]
                    │   │       ├── <17>  [>>]
                    │   │       │   ├── <13> Var [uc]
                    │   │       │   ╰── <15> Constant Int [3]
                    │   │       ╰── <19> Constant Int [31]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <21> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── sc
                    │   ├── Type
                    │   │   ╰── Signed Char
                    │   ╰── Initializer
                    │       ╰── <31> Unary [-]
                    │           ╰── <30> Constant Int [127]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── <37> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <51>  [!=]
                    │   │       ├── <46>  [>>]
                    │   │       │   ├── <41> Var [sc]
                    │   │       │   ╰── <44> Var [c]
                    │   │       ╰── <50> Unary [-]
                    │   │           ╰── <49> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <52> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <74>  [!=]
                    │   │       ├── <69>  [>>]
                    │   │       │   ├── <65> Unary [-]
                    │   │       │   │   ╰── <63>  [<<]
                    │   │       │   │       ├── <59> Var [c]
                    │   │       │   │       ╰── <61> Constant ULong [3]
                    │   │       │   ╰── <67> Constant Int [3]
                    │   │       ╰── <73> Unary [-]
                    │   │           ╰── <72> Constant Int [5]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <75> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <96>  [!=]
                    │   │       ├── <91>  [>>]
                    │   │       │   ├── <87> Unary [-]
                    │   │       │   │   ╰── <86>  [<<]
                    │   │       │   │       ├── <82> Var [uc]
                    │   │       │   │       ╰── <84> Constant UInt [5]
                    │   │       │   ╰── <89> Constant UInt [5]
                    │   │       ╰── <95> Unary [-]
                    │   │           ╰── <94> Constant Long [255]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <97> Constant Int [5]
                    ╰── Return
                        ╰── <102> Constant Int [0]
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
                    │       ╰── <9> Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <20>  [!=]
                    │   │       ├── <17>  [^]
                    │   │       │   ├── <13> Var [x]
                    │   │       │   ╰── <15> Constant Int [65]
                    │   │       ╰── <19> Constant Int [75]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <21> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Char
                    │   ├── Initializer
                    │   │   ╰── <30> Constant Int [132]
                    │   ╰── Static
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <43>  [!=]
                    │   │       ├── <38>  [|]
                    │   │       │   ├── <33> Constant Int [33]
                    │   │       │   ╰── <36> Var [c]
                    │   │       ╰── <42> Unary [-]
                    │   │           ╰── <41> Constant Int [91]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <44> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ul
                    │   ├── Type
                    │   │   ╰── Unsigned Long
                    │   ├── Initializer
                    │   │   ╰── <53> Constant ULong [9259400834947493926]
                    │   ╰── Static
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <64>  [!=]
                    │   │       ├── <61>  [&]
                    │   │       │   ├── <57> Var [ul]
                    │   │       │   ╰── <59> Constant Int [126]
                    │   │       ╰── <63> Constant Int [38]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <65> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <78>  [!=]
                    │   │       ├── <75>  [<<]
                    │   │       │   ├── <71> Var [ul]
                    │   │       │   ╰── <73> Constant Int [32]
                    │   │       ╰── <77> Constant ULong [4611738958194278400]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <79> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <91>  [!=]
                    │   │       ├── <88>  [>>]
                    │   │       │   ├── <84> Constant Int [123]
                    │   │       │   ╰── <86> Constant Int [3]
                    │   │       ╰── <90> Constant Int [15]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <92> Constant Int [5]
                    ╰── Return
                        ╰── <97> Constant Int [0]
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
                    │       ╰── <9> Constant Int [135]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── <17> Unary [-]
                    │           ╰── <16> Constant Int [116]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <29>  [!=]
                    │   │       ├── <26>  [&]
                    │   │       │   ├── <21> Var [uc]
                    │   │       │   ╰── <24> Var [c]
                    │   │       ╰── <28> Constant Int [132]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <30> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <46>  [!=]
                    │   │       ├── <41>  [|]
                    │   │       │   ├── <36> Var [uc]
                    │   │       │   ╰── <39> Var [c]
                    │   │       ╰── <45> Unary [-]
                    │   │           ╰── <44> Constant Int [113]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <47> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <64>  [!=]
                    │   │       ├── <61>  [|]
                    │   │       │   ├── <57>  [^]
                    │   │       │   │   ├── <53> Var [c]
                    │   │       │   │   ╰── <55> Constant UInt [1001]
                    │   │       │   ╰── <59> Constant Long [360]
                    │   │       ╰── <63> Constant Long [4294966637]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <65> Constant Int [3]
                    ╰── Return
                        ╰── <70> Constant Int [0]
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
                    │   │   ╰── <10> Constant Int [65]
                    │   ╰── Static
                    ╰── Switch
                        ├── Expression
                        │   ╰── <14> Var [i]
                        ╰── Block
                            ├── Case [100]
                            │   ╰── Return
                            │       ╰── <16> Constant Int [1]
                            ├── Case [65]
                            │   ╰── Return
                            │       ╰── <20> Constant Int [0]
                            ├── Case [66]
                            │   ╰── Return
                            │       ╰── <24> Constant Int [2]
                            ├── Case [2000]
                            │   ╰── Return
                            │       ╰── <28> Constant Int [3]
                            ╰── Default
                                ╰── Return
                                    ╰── <33> Unary [-]
                                        ╰── <32> Constant Int [1]
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
                    │   │   ╰── <10> Constant Int [100]
                    │   ╰── Static
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c2
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── <16> Constant Int [100]
                    ├── <24> Assign [+=]
                    │   ├── <20> Var [c]
                    │   ╰── <23> Var [c2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <32>  [!=]
                    │   │       ├── <27> Var [c]
                    │   │       ╰── <31> Unary [-]
                    │   │           ╰── <30> Constant Int [56]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <33> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── uc
                    │   ├── Type
                    │   │   ╰── Unsigned Char
                    │   ├── Initializer
                    │   │   ╰── <42> Constant Int [200]
                    │   ╰── Static
                    ├── <51> Assign [=]
                    │   ├── <46> Var [c2]
                    │   ╰── <50> Unary [-]
                    │       ╰── <49> Constant Int [100]
                    ├── <58> Assign [/=]
                    │   ├── <54> Var [uc]
                    │   ╰── <57> Var [c2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <64>  [!=]
                    │   │       ├── <61> Var [uc]
                    │   │       ╰── <63> Constant Int [254]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <65> Constant Int [2]
                    ├── <74> Assign [-=]
                    │   ├── <71> Var [uc]
                    │   ╰── <73> Constant Double [+2.5e2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <80>  [!=]
                    │   │       ├── <77> Var [uc]
                    │   │       ╰── <79> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <81> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── sc
                    │   ├── Type
                    │   │   ╰── Signed Char
                    │   ├── Initializer
                    │   │   ╰── <90> Constant Int [70]
                    │   ╰── Static
                    ├── <100> Assign [=]
                    │   ├── <94> Var [sc]
                    │   ╰── <99> Unary [-]
                    │       ╰── <98> Var [sc]
                    ├── <107> Assign [*=]
                    │   ├── <103> Var [sc]
                    │   ╰── <106> Var [c]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <113>  [!=]
                    │   │       ├── <110> Var [sc]
                    │   │       ╰── <112> Constant Int [80]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <114> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <128>  [!=]
                    │   │       ├── <125> Assign [&=]
                    │   │       │   ├── <120> Var [sc]
                    │   │       │   ╰── <123> Var [c]
                    │   │       ╰── <127> Constant Int [24]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <129> Constant Int [5]
                    ╰── Return
                        ╰── <134> Constant Int [0]
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
                    │           ├── <14> Unary [-]
                    │           │   ╰── <13> Constant Int [128]
                    │           ├── <18> Unary [-]
                    │           │   ╰── <17> Constant Int [120]
                    │           ├── <22> Unary [-]
                    │           │   ╰── <21> Constant Int [2]
                    │           ├── <24> Constant Int [1]
                    │           ╰── <26> Constant Int [120]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── u_arr
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 4
                    │   │       ╰── Unsigned Char
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <36> Constant Int [0]
                    │           ├── <38> Constant Int [170]
                    │           ├── <40> Constant Int [250]
                    │           ╰── <42> Constant Int [255]
                    ├── <52> Assign [^=]
                    │   ├── <49> Subscript
                    │   │   ├── <47> Var [arr]
                    │   │   ╰── <48> Constant Int [0]
                    │   ╰── <51> Constant Int [12345]
                    ├── <63> Assign [|=]
                    │   ├── <57> Subscript
                    │   │   ├── <55> Var [arr]
                    │   │   ╰── <56> Constant Int [1]
                    │   ╰── <62> Subscript
                    │       ├── <60> Var [u_arr]
                    │       ╰── <61> Constant Int [3]
                    ├── <80> Assign [&=]
                    │   ├── <68> Subscript
                    │   │   ├── <66> Var [arr]
                    │   │   ╰── <67> Constant Int [2]
                    │   ╰── <79>  [-]
                    │       ├── <73> Subscript
                    │       │   ├── <71> Var [u_arr]
                    │       │   ╰── <72> Constant Int [1]
                    │       ╰── <78> Cast
                    │           ├── Target
                    │           │   ╰── Unsigned Char
                    │           ╰── Expression
                    │               ╰── <77> Constant Int [185]
                    ├── <88> Assign [<<=]
                    │   ├── <85> Subscript
                    │   │   ├── <83> Var [arr]
                    │   │   ╰── <84> Constant Int [3]
                    │   ╰── <87> Constant UInt [7]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Long
                    │   ├── Initializer
                    │   │   ╰── <94> Constant Int [32]
                    │   ╰── Static
                    ├── <103> Assign [>>=]
                    │   ├── <100> Subscript
                    │   │   ├── <98> Var [arr]
                    │   │   ╰── <99> Constant Int [4]
                    │   ╰── <102> Constant Int [31]
                    ├── <111> Assign [<<=]
                    │   ├── <108> Subscript
                    │   │   ├── <106> Var [u_arr]
                    │   │   ╰── <107> Constant Int [3]
                    │   ╰── <110> Constant Int [12]
                    ├── <124> Assign [>>=]
                    │   ├── <116> Subscript
                    │   │   ├── <114> Var [u_arr]
                    │   │   ╰── <115> Constant Int [2]
                    │   ╰── <123>  [-]
                    │       ├── <119> Var [x]
                    │       ╰── <121> Constant Int [1]
                    ├── <134> Assign [|=]
                    │   ├── <129> Subscript
                    │   │   ├── <127> Var [u_arr]
                    │   │   ╰── <128> Constant Int [1]
                    │   ╰── <133> Unary [-]
                    │       ╰── <132> Constant Int [399]
                    ├── <142> Assign [=]
                    │   ├── <137> Var [x]
                    │   ╰── <141> Unary [-]
                    │       ╰── <140> Constant Long [4296140120]
                    ├── <151> Assign [^=]
                    │   ├── <147> Subscript
                    │   │   ├── <145> Var [u_arr]
                    │   │   ╰── <146> Constant Int [0]
                    │   ╰── <150> Var [x]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <161>  [!=]
                    │   │       ├── <156> Subscript
                    │   │       │   ├── <154> Var [arr]
                    │   │       │   ╰── <155> Constant Int [0]
                    │   │       ╰── <160> Unary [-]
                    │   │           ╰── <159> Constant Int [71]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <162> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <175>  [!=]
                    │   │       ├── <170> Subscript
                    │   │       │   ├── <168> Var [arr]
                    │   │       │   ╰── <169> Constant Int [1]
                    │   │       ╰── <174> Unary [-]
                    │   │           ╰── <173> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <176> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <189>  [!=]
                    │   │       ├── <184> Subscript
                    │   │       │   ├── <182> Var [arr]
                    │   │       │   ╰── <183> Constant Int [2]
                    │   │       ╰── <188> Unary [-]
                    │   │           ╰── <187> Constant Int [16]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <190> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <203>  [!=]
                    │   │       ├── <198> Subscript
                    │   │       │   ├── <196> Var [arr]
                    │   │       │   ╰── <197> Constant Int [3]
                    │   │       ╰── <202> Unary [-]
                    │   │           ╰── <201> Constant Int [128]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <204> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <215>  [!=]
                    │   │       ├── <212> Subscript
                    │   │       │   ├── <210> Var [arr]
                    │   │       │   ╰── <211> Constant Int [4]
                    │   │       ╰── <214> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <216> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <227>  [!=]
                    │   │       ├── <224> Subscript
                    │   │       │   ├── <222> Var [u_arr]
                    │   │       │   ╰── <223> Constant Int [0]
                    │   │       ╰── <226> Constant Int [168]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <228> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <239>  [!=]
                    │   │       ├── <236> Subscript
                    │   │       │   ├── <234> Var [u_arr]
                    │   │       │   ╰── <235> Constant Int [1]
                    │   │       ╰── <238> Constant Int [251]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <240> Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <251>  [!=]
                    │   │       ├── <248> Subscript
                    │   │       │   ├── <246> Var [u_arr]
                    │   │       │   ╰── <247> Constant Int [2]
                    │   │       ╰── <250> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <252> Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <263>  [!=]
                    │   │       ├── <260> Subscript
                    │   │       │   ├── <258> Var [u_arr]
                    │   │       │   ╰── <259> Constant Int [3]
                    │   │       ╰── <262> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <264> Constant Int [9]
                    ╰── Return
                        ╰── <269> Constant Int [0]
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
                    │   │       ├── <13> Constant Int [123]
                    │   │       ├── <15> Constant Int [124]
                    │   │       ├── <17> Constant Int [125]
                    │   │       ├── <19> Constant Int [126]
                    │   │       ╰── <21> Constant Int [127]
                    │   ╰── Static
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <33>  [!=]
                    │   │       ├── <30> Postfix [++]
                    │   │       │   ╰── <28> Subscript
                    │   │       │       ├── <26> Var [chars]
                    │   │       │       ╰── <27> Constant Int [0]
                    │   │       ╰── <32> Constant Int [123]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <34> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <47>  [!=]
                    │   │       ├── <44> Postfix [--]
                    │   │       │   ╰── <42> Subscript
                    │   │       │       ├── <40> Var [chars]
                    │   │       │       ╰── <41> Constant Int [1]
                    │   │       ╰── <46> Constant Int [124]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <48> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <61>  [!=]
                    │   │       ├── <58> Unary [++]
                    │   │       │   ╰── <57> Subscript
                    │   │       │       ├── <55> Var [chars]
                    │   │       │       ╰── <56> Constant Int [2]
                    │   │       ╰── <60> Constant Int [126]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <62> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <75>  [!=]
                    │   │       ├── <72> Unary [--]
                    │   │       │   ╰── <71> Subscript
                    │   │       │       ├── <69> Var [chars]
                    │   │       │       ╰── <70> Constant Int [3]
                    │   │       ╰── <74> Constant Int [125]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <76> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <91>  [!=]
                    │   │       ├── <86> Unary [++]
                    │   │       │   ╰── <85> Subscript
                    │   │       │       ├── <83> Var [chars]
                    │   │       │       ╰── <84> Constant Int [4]
                    │   │       ╰── <90> Unary [-]
                    │   │           ╰── <89> Constant Int [128]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <92> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <103>  [!=]
                    │   │       ├── <100> Subscript
                    │   │       │   ├── <98> Var [chars]
                    │   │       │   ╰── <99> Constant Int [0]
                    │   │       ╰── <102> Constant Int [124]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <104> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <115>  [!=]
                    │   │       ├── <112> Subscript
                    │   │       │   ├── <110> Var [chars]
                    │   │       │   ╰── <111> Constant Int [1]
                    │   │       ╰── <114> Constant Int [123]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <116> Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <127>  [!=]
                    │   │       ├── <124> Subscript
                    │   │       │   ├── <122> Var [chars]
                    │   │       │   ╰── <123> Constant Int [2]
                    │   │       ╰── <126> Constant Int [126]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <128> Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <139>  [!=]
                    │   │       ├── <136> Subscript
                    │   │       │   ├── <134> Var [chars]
                    │   │       │   ╰── <135> Constant Int [3]
                    │   │       ╰── <138> Constant Int [125]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <140> Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <153>  [!=]
                    │   │       ├── <148> Subscript
                    │   │       │   ├── <146> Var [chars]
                    │   │       │   ╰── <147> Constant Int [4]
                    │   │       ╰── <152> Unary [-]
                    │   │           ╰── <151> Constant Int [128]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <154> Constant Int [10]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Signed Char
                    │   ╰── Initializer
                    │       ╰── <164> Unary [-]
                    │           ╰── <163> Constant Int [128]
                    ├── <170> Postfix [--]
                    │   ╰── <168> Var [c]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <176>  [!=]
                    │   │       ├── <173> Var [c]
                    │   │       ╰── <175> Constant Int [127]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <177> Constant Int [11]
                    ╰── Return
                        ╰── <182> Constant Int [0]
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
                    │           ├── <12> Constant Int [0]
                    │           ├── <14> Constant Int [2]
                    │           ├── <16> Constant Int [4]
                    │           ├── <18> Constant Int [253]
                    │           ╰── <20> Constant Int [255]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <29> Postfix [--]
                    │   │       ╰── <27> Subscript
                    │   │           ├── <25> Var [chars]
                    │   │           ╰── <26> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <30> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <43>  [!=]
                    │   │       ├── <40> Postfix [++]
                    │   │       │   ╰── <38> Subscript
                    │   │       │       ├── <36> Var [chars]
                    │   │       │       ╰── <37> Constant Int [1]
                    │   │       ╰── <42> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <44> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <57>  [!=]
                    │   │       ├── <54> Unary [--]
                    │   │       │   ╰── <53> Subscript
                    │   │       │       ├── <51> Var [chars]
                    │   │       │       ╰── <52> Constant Int [3]
                    │   │       ╰── <56> Constant Int [252]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <58> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <71>  [!=]
                    │   │       ├── <68> Unary [++]
                    │   │       │   ╰── <67> Subscript
                    │   │       │       ├── <65> Var [chars]
                    │   │       │       ╰── <66> Constant Int [4]
                    │   │       ╰── <70> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <72> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <83>  [!=]
                    │   │       ├── <80> Subscript
                    │   │       │   ├── <78> Var [chars]
                    │   │       │   ╰── <79> Constant Int [0]
                    │   │       ╰── <82> Constant Int [255]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <84> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <95>  [!=]
                    │   │       ├── <92> Subscript
                    │   │       │   ├── <90> Var [chars]
                    │   │       │   ╰── <91> Constant Int [1]
                    │   │       ╰── <94> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <96> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <107>  [!=]
                    │   │       ├── <104> Subscript
                    │   │       │   ├── <102> Var [chars]
                    │   │       │   ╰── <103> Constant Int [2]
                    │   │       ╰── <106> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <108> Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <119>  [!=]
                    │   │       ├── <116> Subscript
                    │   │       │   ├── <114> Var [chars]
                    │   │       │   ╰── <115> Constant Int [3]
                    │   │       ╰── <118> Constant Int [252]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <120> Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <128> Subscript
                    │   │       ├── <126> Var [chars]
                    │   │       ╰── <127> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <129> Constant Int [9]
                    ╰── Return
                        ╰── <134> Constant Int [0]
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
                    │       ╰── <9> Constant Int [100]
                    ╰── Switch
                        ├── Expression
                        │   ╰── <13> Var [c]
                        ╰── Block
                            ├── Case [0]
                            │   ╰── Return
                            │       ╰── <15> Constant Int [1]
                            ├── Case [100]
                            │   ╰── Return
                            │       ╰── <19> Constant Int [0]
                            ├── Case [356]
                            │   ╰── Return
                            │       ╰── <23> Constant Int [2]
                            ╰── Default
                                ╰── Return
                                    ╰── <26> Constant Int [3]
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
                    │       ╰── <11> Unary [-]
                    │           ╰── <10> Constant Int [56]
                    ╰── Switch
                        ├── Expression
                        │   ╰── <15> Var [c]
                        ╰── Block
                            ├── Case [33554632]
                            │   ╰── Return
                            │       ╰── <17> Constant Int [1]
                            ╰── Default
                                ╰── Return
                                    ╰── <20> Constant Int [0]
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
                        │   ╰── <6> Constant Int [120]
                        ╰── Block
                            ├── Case [1]
                            │   ╰── Return
                            │       ╰── <8> Constant Int [1]
                            ├── Case [2]
                            │   ╰── Return
                            │       ╰── <12> Constant Int [2]
                            ├── Case [120]
                            │   ╰── Return
                            │       ╰── <16> Constant Int [0]
                            ╰── Default
                                ╰── Return
                                    ╰── <21> Unary [-]
                                        ╰── <20> Constant Int [1]
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
                    │       ╰── <33> Constant Int [5]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── expected_b
                    │   ├── Type
                    │   │   ╰── Signed Char
                    │   ╰── Initializer
                    │       ╰── <41> Unary [-]
                    │           ╰── <40> Constant Int [12]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── expected_c
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── <47> Constant Int [117]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── expected_d
                    │   ├── Type
                    │   │   ╰── Unsigned Char
                    │   ╰── Initializer
                    │       ╰── <53> Constant Int [254]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── expected_e
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── <59> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── expected_f
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── <67> Unary [-]
                    │           ╰── <66> Constant Int [20]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── expected_g
                    │   ├── Type
                    │   │   ╰── Signed Char
                    │   ╰── Initializer
                    │       ╰── <73> Constant Int [60]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── expected_h
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── <79> Constant Int [100]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <87>  [!=]
                    │   │       ├── <83> Var [expected_a]
                    │   │       ╰── <86> Var [a]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <88> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <98>  [!=]
                    │   │       ├── <94> Var [expected_b]
                    │   │       ╰── <97> Var [b]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <99> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <109>  [!=]
                    │   │       ├── <105> Var [expected_c]
                    │   │       ╰── <108> Var [c]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <110> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <120>  [!=]
                    │   │       ├── <116> Var [expected_d]
                    │   │       ╰── <119> Var [d]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <121> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <131>  [!=]
                    │   │       ├── <127> Var [expected_e]
                    │   │       ╰── <130> Var [e]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <132> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <142>  [!=]
                    │   │       ├── <138> Var [expected_f]
                    │   │       ╰── <141> Var [f]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <143> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <153>  [!=]
                    │   │       ├── <149> Var [expected_g]
                    │   │       ╰── <152> Var [g]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <154> Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <164>  [!=]
                    │   │       ├── <160> Var [expected_h]
                    │   │       ╰── <163> Var [h]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <165> Constant Int [8]
                    ╰── Return
                        ╰── <170> Constant Int [0]
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
                    │       ╰── <39> Constant Int [5]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Signed Char
                    │   ╰── Initializer
                    │       ╰── <47> Unary [-]
                    │           ╰── <46> Constant Int [12]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── <53> Constant Int [117]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Unsigned Char
                    │   ╰── Initializer
                    │       ╰── <59> Constant Int [254]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── e
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── <65> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── f
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── <73> Unary [-]
                    │           ╰── <72> Constant Int [20]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── g
                    │   ├── Type
                    │   │   ╰── Signed Char
                    │   ╰── Initializer
                    │       ╰── <79> Constant Int [60]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── h
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── <85> Constant Int [100]
                    ╰── Return
                        ╰── <105> FunctionCall [check_args]
                            ├── <90> Var [a]
                            ├── <92> Var [b]
                            ├── <94> Var [c]
                            ├── <96> Var [d]
                            ├── <98> Var [e]
                            ├── <100> Var [f]
                            ├── <102> Var [g]
                            ╰── <104> Var [h]
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
            │       ╰── <4> Constant Int [100]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── uc
            │   ├── Type
            │   │   ╰── Unsigned Char
            │   ╰── Initializer
            │       ╰── <10> Constant Int [250]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── sc
            │   ├── Type
            │   │   ╰── Signed Char
            │   ╰── Initializer
            │       ╰── <16> Constant Int [0]
            ╰── Function [update_global_chars]
                ╰── Body
                    ├── <32> Assign [=]
                    │   ├── <25> Var [c]
                    │   ╰── <31>  [+]
                    │       ├── <28> Var [c]
                    │       ╰── <30> Constant Int [10]
                    ├── <42> Assign [=]
                    │   ├── <35> Var [uc]
                    │   ╰── <41>  [+]
                    │       ├── <38> Var [uc]
                    │       ╰── <40> Constant Int [10]
                    ├── <52> Assign [=]
                    │   ├── <45> Var [sc]
                    │   ╰── <51>  [-]
                    │       ├── <48> Var [sc]
                    │       ╰── <50> Constant Int [10]
                    ╰── Return
                        ╰── <54> Constant Int [0]
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
                    │   │   ╰── <31>  [!=]
                    │   │       ├── <28> Var [c]
                    │   │       ╰── <30> Constant Int [100]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <32> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <41>  [!=]
                    │   │       ├── <38> Var [uc]
                    │   │       ╰── <40> Constant Int [250]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <42> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <51>  [!=]
                    │   │       ├── <48> Var [sc]
                    │   │       ╰── <50> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <52> Constant Int [3]
                    ├── <58> FunctionCall [update_global_chars]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <64>  [!=]
                    │   │       ├── <61> Var [c]
                    │   │       ╰── <63> Constant Int [110]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <65> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <74>  [!=]
                    │   │       ├── <71> Var [uc]
                    │   │       ╰── <73> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <75> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <86>  [!=]
                    │   │       ├── <81> Var [sc]
                    │   │       ╰── <85> Unary [-]
                    │   │           ╰── <84> Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <87> Constant Int [6]
                    ╰── Return
                        ╰── <92> Constant Int [0]
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
            │           ╰── <6> Constant Long [5369233654]
            ├── Function [return_schar]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <15> Constant Long [5369233654]
            ╰── Function [return_uchar]
                ╰── Body
                    ╰── Return
                        ╰── <24> Constant Long [5369233654]
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
                    │           ├── <30> Constant Int [121]
                    │           ├── <34> Unary [-]
                    │           │   ╰── <33> Constant Int [122]
                    │           ╰── <38> Unary [-]
                    │               ╰── <37> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── retval_c
                    │   ├── Type
                    │   │   ╰── Char
                    │   ╰── Initializer
                    │       ╰── <46> FunctionCall [return_char]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── char_array2
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 3
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <57> Unary [-]
                    │           │   ╰── <56> Constant Int [5]
                    │           ├── <59> Constant Int [88]
                    │           ╰── <63> Unary [-]
                    │               ╰── <62> Constant Int [100]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── retval_sc
                    │   ├── Type
                    │   │   ╰── Signed Char
                    │   ╰── Initializer
                    │       ╰── <71> FunctionCall [return_schar]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── char_array3
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 3
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <80> Constant Int [10]
                    │           ├── <82> Constant Int [11]
                    │           ╰── <84> Constant Int [12]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── retval_uc
                    │   ├── Type
                    │   │   ╰── Unsigned Char
                    │   ╰── Initializer
                    │       ╰── <92> FunctionCall [return_uchar]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── char_array4
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 2
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <103> Unary [-]
                    │           │   ╰── <102> Constant Int [5]
                    │           ╰── <107> Unary [-]
                    │               ╰── <106> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <139>  [||]
                    │   │       ├── <128>  [||]
                    │   │       │   ├── <117>  [!=]
                    │   │       │   │   ├── <114> Subscript
                    │   │       │   │   │   ├── <112> Var [char_array]
                    │   │       │   │   │   ╰── <113> Constant Int [0]
                    │   │       │   │   ╰── <116> Constant Int [121]
                    │   │       │   ╰── <127>  [!=]
                    │   │       │       ├── <122> Subscript
                    │   │       │       │   ├── <120> Var [char_array]
                    │   │       │       │   ╰── <121> Constant Int [1]
                    │   │       │       ╰── <126> Unary [-]
                    │   │       │           ╰── <125> Constant Int [122]
                    │   │       ╰── <138>  [!=]
                    │   │           ├── <133> Subscript
                    │   │           │   ├── <131> Var [char_array]
                    │   │           │   ╰── <132> Constant Int [2]
                    │   │           ╰── <137> Unary [-]
                    │   │               ╰── <136> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <140> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <151>  [!=]
                    │   │       ├── <146> Var [retval_c]
                    │   │       ╰── <150> Unary [-]
                    │   │           ╰── <149> Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <152> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <185>  [||]
                    │   │       ├── <174>  [||]
                    │   │       │   ├── <165>  [!=]
                    │   │       │   │   ├── <160> Subscript
                    │   │       │   │   │   ├── <158> Var [char_array2]
                    │   │       │   │   │   ╰── <159> Constant Int [0]
                    │   │       │   │   ╰── <164> Unary [-]
                    │   │       │   │       ╰── <163> Constant Int [5]
                    │   │       │   ╰── <173>  [!=]
                    │   │       │       ├── <170> Subscript
                    │   │       │       │   ├── <168> Var [char_array2]
                    │   │       │       │   ╰── <169> Constant Int [1]
                    │   │       │       ╰── <172> Constant Int [88]
                    │   │       ╰── <184>  [!=]
                    │   │           ├── <179> Subscript
                    │   │           │   ├── <177> Var [char_array2]
                    │   │           │   ╰── <178> Constant Int [2]
                    │   │           ╰── <183> Unary [-]
                    │   │               ╰── <182> Constant Int [100]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <186> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <197>  [!=]
                    │   │       ├── <192> Var [retval_sc]
                    │   │       ╰── <196> Unary [-]
                    │   │           ╰── <195> Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <198> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <227>  [||]
                    │   │       ├── <218>  [||]
                    │   │       │   ├── <209>  [!=]
                    │   │       │   │   ├── <206> Subscript
                    │   │       │   │   │   ├── <204> Var [char_array3]
                    │   │       │   │   │   ╰── <205> Constant Int [0]
                    │   │       │   │   ╰── <208> Constant Int [10]
                    │   │       │   ╰── <217>  [!=]
                    │   │       │       ├── <214> Subscript
                    │   │       │       │   ├── <212> Var [char_array3]
                    │   │       │       │   ╰── <213> Constant Int [1]
                    │   │       │       ╰── <216> Constant Int [11]
                    │   │       ╰── <226>  [!=]
                    │   │           ├── <223> Subscript
                    │   │           │   ├── <221> Var [char_array3]
                    │   │           │   ╰── <222> Constant Int [2]
                    │   │           ╰── <225> Constant Int [12]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <228> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <237>  [!=]
                    │   │       ├── <234> Var [retval_uc]
                    │   │       ╰── <236> Constant Int [246]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <238> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <262>  [||]
                    │   │       ├── <251>  [!=]
                    │   │       │   ├── <246> Subscript
                    │   │       │   │   ├── <244> Var [char_array4]
                    │   │       │   │   ╰── <245> Constant Int [0]
                    │   │       │   ╰── <250> Unary [-]
                    │   │       │       ╰── <249> Constant Int [5]
                    │   │       ╰── <261>  [!=]
                    │   │           ├── <256> Subscript
                    │   │           │   ├── <254> Var [char_array4]
                    │   │           │   ╰── <255> Constant Int [1]
                    │   │           ╰── <260> Unary [-]
                    │   │               ╰── <259> Constant Int [6]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <263> Constant Int [7]
                    ╰── Return
                        ╰── <268> Constant Int [0]
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
                    │       ╰── <28> "yesno"
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
                    │           ├── <40> "ab"
                    │           ╰── <42> "cd"
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <50> FunctionCall [strcmp]
                    │   │       ├── <48> Var [multi_string]
                    │   │       ╰── <49> "yesno"
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <51> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <60> FunctionCall [strcmp]
                    │   │       ├── <58> Subscript
                    │   │       │   ├── <56> Var [nested_multi_string]
                    │   │       │   ╰── <57> Constant Int [0]
                    │   │       ╰── <59> "ab"
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <61> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <70> FunctionCall [strcmp]
                    │   │       ├── <68> Subscript
                    │   │       │   ├── <66> Var [nested_multi_string]
                    │   │       │   ╰── <67> Constant Int [1]
                    │   │       ╰── <69> "cd"
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <71> Constant Int [3]
                    ╰── Return
                        ╰── <74> Constant Int [0]
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
                    │       ╰── <12> "
        	"
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <21>  [!=]
                    │   │       ├── <18> Subscript
                    │   │       │   ├── <16> Var [special]
                    │   │       │   ╰── <17> Constant Int [0]
                    │   │       ╰── <20> Constant Int [7]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <22> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <33>  [!=]
                    │   │       ├── <30> Subscript
                    │   │       │   ├── <28> Var [special]
                    │   │       │   ╰── <29> Constant Int [1]
                    │   │       ╰── <32> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <34> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <45>  [!=]
                    │   │       ├── <42> Subscript
                    │   │       │   ├── <40> Var [special]
                    │   │       │   ╰── <41> Constant Int [2]
                    │   │       ╰── <44> Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <46> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <57>  [!=]
                    │   │       ├── <54> Subscript
                    │   │       │   ├── <52> Var [special]
                    │   │       │   ╰── <53> Constant Int [3]
                    │   │       ╰── <56> Constant Int [11]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <58> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <69>  [!=]
                    │   │       ├── <66> Subscript
                    │   │       │   ├── <64> Var [special]
                    │   │       │   ╰── <65> Constant Int [4]
                    │   │       ╰── <68> Constant Int [12]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <70> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <81>  [!=]
                    │   │       ├── <78> Subscript
                    │   │       │   ├── <76> Var [special]
                    │   │       │   ╰── <77> Constant Int [5]
                    │   │       ╰── <80> Constant Int [9]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <82> Constant Int [6]
                    ╰── Return
                        ╰── <87> Constant Int [0]
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
            │           │   ├── <10> Constant Int [97]
            │           │   ├── <12> Constant Int [98]
            │           │   ├── <14> Constant Int [99]
            │           │   ╰── <16> Constant Int [100]
            │           ├── <19> "efgh"
            │           ╰── <21> "ijk"
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
                    │           ├── <39> "lmn"
                    │           ╰── Compound
                    │               ├── <41> Constant Int [111]
                    │               ╰── <43> Constant Int [112]
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
                    │   │       ╰── <58> Constant Int [3]
                    │   ├── Condition
                    │   │   ╰── <68> Assign [=]
                    │   │       ├── <61> Var [i]
                    │   │       ╰── <67>  [+]
                    │   │           ├── <64> Var [i]
                    │   │           ╰── <66> Constant Int [1]
                    │   ╰── For
                    │       ├── Init
                    │       │   ╰── VarDeclaration
                    │       │       ├── Name
                    │       │       │   ╰── j
                    │       │       ├── Type
                    │       │       │   ╰── Int
                    │       │       ╰── Initializer
                    │       │           ╰── <72> Constant Int [0]
                    │       ├── Condition
                    │       │   ╰── <80>  [<]
                    │       │       ├── <77> Var [j]
                    │       │       ╰── <79> Constant Int [4]
                    │       ├── Condition
                    │       │   ╰── <89> Assign [=]
                    │       │       ├── <82> Var [j]
                    │       │       ╰── <88>  [+]
                    │       │           ├── <85> Var [j]
                    │       │           ╰── <87> Constant Int [1]
                    │       ╰── If
                    │           ├── Condition
                    │           │   ╰── <110>  [!=]
                    │           │       ├── <97> Subscript
                    │           │       │   ├── <94> Subscript
                    │           │       │   │   ├── <91> Var [static_array]
                    │           │       │   │   ╰── <93> Var [i]
                    │           │       │   ╰── <96> Var [j]
                    │           │       ╰── <109> Subscript
                    │           │           ├── <99> "abcdefghijk"
                    │           │           ╰── <108>  [+]
                    │           │               ├── <104>  [*]
                    │           │               │   ├── <101> Var [i]
                    │           │               │   ╰── <103> Constant Int [4]
                    │           │               ╰── <107> Var [j]
                    │           ╰── Then
                    │               ╰── Return
                    │                   ╰── <111> Constant Int [1]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <119> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <127>  [<]
                    │   │       ├── <124> Var [i]
                    │   │       ╰── <126> Constant Int [2]
                    │   ├── Condition
                    │   │   ╰── <136> Assign [=]
                    │   │       ├── <129> Var [i]
                    │   │       ╰── <135>  [+]
                    │   │           ├── <132> Var [i]
                    │   │           ╰── <134> Constant Int [1]
                    │   ╰── For
                    │       ├── Init
                    │       │   ╰── VarDeclaration
                    │       │       ├── Name
                    │       │       │   ╰── j
                    │       │       ├── Type
                    │       │       │   ╰── Int
                    │       │       ╰── Initializer
                    │       │           ╰── <140> Constant Int [0]
                    │       ├── Condition
                    │       │   ╰── <148>  [<]
                    │       │       ├── <145> Var [j]
                    │       │       ╰── <147> Constant Int [3]
                    │       ├── Condition
                    │       │   ╰── <157> Assign [=]
                    │       │       ├── <150> Var [j]
                    │       │       ╰── <156>  [+]
                    │       │           ├── <153> Var [j]
                    │       │           ╰── <155> Constant Int [1]
                    │       ╰── If
                    │           ├── Condition
                    │           │   ╰── <178>  [!=]
                    │           │       ├── <165> Subscript
                    │           │       │   ├── <162> Subscript
                    │           │       │   │   ├── <159> Var [auto_array]
                    │           │       │   │   ╰── <161> Var [i]
                    │           │       │   ╰── <164> Var [j]
                    │           │       ╰── <177> Subscript
                    │           │           ├── <167> "lmnop"
                    │           │           ╰── <176>  [+]
                    │           │               ├── <172>  [*]
                    │           │               │   ├── <169> Var [i]
                    │           │               │   ╰── <171> Constant Int [3]
                    │           │               ╰── <175> Var [j]
                    │           ╰── Then
                    │               ╰── Return
                    │                   ╰── <179> Constant Int [2]
                    ╰── Return
                        ╰── <184> Constant Int [0]
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
            │   │   ╰── <8> "hi"
            │   ╰── Static
            ├── Function [test_static]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <53>  [&&]
            │               ├── <31>  [&&]
            │               │   ├── <22>  [==]
            │               │   │   ├── <19> Subscript
            │               │   │   │   ├── <17> Var [static_arr]
            │               │   │   │   ╰── <18> Constant Int [0]
            │               │   │   ╰── <21> Constant Int [104]
            │               │   ╰── <30>  [==]
            │               │       ├── <27> Subscript
            │               │       │   ├── <25> Var [static_arr]
            │               │       │   ╰── <26> Constant Int [1]
            │               │       ╰── <29> Constant Int [105]
            │               ╰── <51> Unary [!]
            │                   ╰── <50>  [||]
            │                       ├── <43>  [||]
            │                       │   ├── <37> Subscript
            │                       │   │   ├── <35> Var [static_arr]
            │                       │   │   ╰── <36> Constant Int [2]
            │                       │   ╰── <42> Subscript
            │                       │       ├── <40> Var [static_arr]
            │                       │       ╰── <41> Constant Int [3]
            │                       ╰── <48> Subscript
            │                           ├── <46> Var [static_arr]
            │                           ╰── <47> Constant Int [4]
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
            │   │       ├── <67> ""
            │   │       ╰── <69> "bc"
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
            │       │   │           ╰── <81> Constant Int [0]
            │       │   ├── Condition
            │       │   │   ╰── <89>  [<]
            │       │   │       ├── <86> Var [i]
            │       │   │       ╰── <88> Constant Int [3]
            │       │   ├── Condition
            │       │   │   ╰── <98> Assign [=]
            │       │   │       ├── <91> Var [i]
            │       │   │       ╰── <97>  [+]
            │       │   │           ├── <94> Var [i]
            │       │   │           ╰── <96> Constant Int [1]
            │       │   ╰── For
            │       │       ├── Init
            │       │       │   ╰── VarDeclaration
            │       │       │       ├── Name
            │       │       │       │   ╰── j
            │       │       │       ├── Type
            │       │       │       │   ╰── Int
            │       │       │       ╰── Initializer
            │       │       │           ╰── <102> Constant Int [0]
            │       │       ├── Condition
            │       │       │   ╰── <110>  [<]
            │       │       │       ├── <107> Var [j]
            │       │       │       ╰── <109> Constant Int [4]
            │       │       ├── Condition
            │       │       │   ╰── <119> Assign [=]
            │       │       │       ├── <112> Var [j]
            │       │       │       ╰── <118>  [+]
            │       │       │           ├── <115> Var [j]
            │       │       │           ╰── <117> Constant Int [1]
            │       │       ╰── Block
            │       │           ├── VarDeclaration
            │       │           │   ├── Name
            │       │           │   │   ╰── c
            │       │           │   ├── Type
            │       │           │   │   ╰── Signed Char
            │       │           │   ╰── Initializer
            │       │           │       ╰── <130> Subscript
            │       │           │           ├── <127> Subscript
            │       │           │           │   ├── <124> Var [nested_static_arr]
            │       │           │           │   ╰── <126> Var [i]
            │       │           │           ╰── <129> Var [j]
            │       │           ├── VarDeclaration
            │       │           │   ├── Name
            │       │           │   │   ╰── expected
            │       │           │   ├── Type
            │       │           │   │   ╰── Signed Char
            │       │           │   ╰── Initializer
            │       │           │       ╰── <136> Constant Int [0]
            │       │           ├── If
            │       │           │   ├── Condition
            │       │           │   │   ╰── <150>  [&&]
            │       │           │   │       ├── <143>  [==]
            │       │           │   │       │   ├── <140> Var [i]
            │       │           │   │       │   ╰── <142> Constant Int [1]
            │       │           │   │       ╰── <149>  [==]
            │       │           │   │           ├── <146> Var [j]
            │       │           │   │           ╰── <148> Constant Int [0]
            │       │           │   ├── Then
            │       │           │   │   ╰── Block
            │       │           │   │       ╰── <155> Assign [=]
            │       │           │   │           ├── <152> Var [expected]
            │       │           │   │           ╰── <154> Constant Int [98]
            │       │           │   ╰── Else
            │       │           │       ╰── If
            │       │           │           ├── Condition
            │       │           │           │   ╰── <170>  [&&]
            │       │           │           │       ├── <163>  [==]
            │       │           │           │       │   ├── <160> Var [i]
            │       │           │           │       │   ╰── <162> Constant Int [1]
            │       │           │           │       ╰── <169>  [==]
            │       │           │           │           ├── <166> Var [j]
            │       │           │           │           ╰── <168> Constant Int [1]
            │       │           │           ╰── Then
            │       │           │               ╰── Block
            │       │           │                   ╰── <175> Assign [=]
            │       │           │                       ├── <172> Var [expected]
            │       │           │                       ╰── <174> Constant Int [99]
            │       │           ╰── If
            │       │               ├── Condition
            │       │               │   ╰── <186>  [!=]
            │       │               │       ├── <182> Var [c]
            │       │               │       ╰── <185> Var [expected]
            │       │               ╰── Then
            │       │                   ╰── Block
            │       │                       ╰── Return
            │       │                           ╰── <187> Constant Int [0]
            │       ╰── Return
            │           ╰── <196> Constant Int [1]
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
            │       │       ╰── <211> "ab"
            │       ╰── Return
            │           ╰── <245>  [&&]
            │               ├── <229>  [&&]
            │               │   ├── <220>  [==]
            │               │   │   ├── <217> Subscript
            │               │   │   │   ├── <215> Var [aut]
            │               │   │   │   ╰── <216> Constant Int [0]
            │               │   │   ╰── <219> Constant Int [97]
            │               │   ╰── <228>  [==]
            │               │       ├── <225> Subscript
            │               │       │   ├── <223> Var [aut]
            │               │       │   ╰── <224> Constant Int [1]
            │               │       ╰── <227> Constant Int [98]
            │               ╰── <243> Unary [!]
            │                   ╰── <242>  [||]
            │                       ├── <235> Subscript
            │                       │   ├── <233> Var [aut]
            │                       │   ╰── <234> Constant Int [2]
            │                       ╰── <240> Subscript
            │                           ├── <238> Var [aut]
            │                           ╰── <239> Constant Int [3]
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
            │       │           │   ╰── <266> "foo"
            │       │           ╰── Compound
            │       │               ├── <269> "x"
            │       │               ╰── <271> "yz"
            │       ├── For
            │       │   ├── Init
            │       │   │   ╰── VarDeclaration
            │       │   │       ├── Name
            │       │   │       │   ╰── i
            │       │   │       ├── Type
            │       │   │       │   ╰── Int
            │       │   │       ╰── Initializer
            │       │   │           ╰── <279> Constant Int [0]
            │       │   ├── Condition
            │       │   │   ╰── <287>  [<]
            │       │   │       ├── <284> Var [i]
            │       │   │       ╰── <286> Constant Int [2]
            │       │   ├── Condition
            │       │   │   ╰── <296> Assign [=]
            │       │   │       ├── <289> Var [i]
            │       │   │       ╰── <295>  [+]
            │       │   │           ├── <292> Var [i]
            │       │   │           ╰── <294> Constant Int [1]
            │       │   ╰── Block
            │       │       ╰── For
            │       │           ├── Init
            │       │           │   ╰── VarDeclaration
            │       │           │       ├── Name
            │       │           │       │   ╰── j
            │       │           │       ├── Type
            │       │           │       │   ╰── Int
            │       │           │       ╰── Initializer
            │       │           │           ╰── <300> Constant Int [0]
            │       │           ├── Condition
            │       │           │   ╰── <308>  [<]
            │       │           │       ├── <305> Var [j]
            │       │           │       ╰── <307> Constant Int [2]
            │       │           ├── Condition
            │       │           │   ╰── <317> Assign [=]
            │       │           │       ├── <310> Var [j]
            │       │           │       ╰── <316>  [+]
            │       │           │           ├── <313> Var [j]
            │       │           │           ╰── <315> Constant Int [1]
            │       │           ╰── Block
            │       │               ╰── For
            │       │                   ├── Init
            │       │                   │   ╰── VarDeclaration
            │       │                   │       ├── Name
            │       │                   │       │   ╰── k
            │       │                   │       ├── Type
            │       │                   │       │   ╰── Int
            │       │                   │       ╰── Initializer
            │       │                   │           ╰── <321> Constant Int [0]
            │       │                   ├── Condition
            │       │                   │   ╰── <329>  [<]
            │       │                   │       ├── <326> Var [k]
            │       │                   │       ╰── <328> Constant Int [4]
            │       │                   ├── Condition
            │       │                   │   ╰── <338> Assign [=]
            │       │                   │       ├── <331> Var [k]
            │       │                   │       ╰── <337>  [+]
            │       │                   │           ├── <334> Var [k]
            │       │                   │           ╰── <336> Constant Int [1]
            │       │                   ╰── Block
            │       │                       ├── VarDeclaration
            │       │                       │   ├── Name
            │       │                       │   │   ╰── c
            │       │                       │   ├── Type
            │       │                       │   │   ╰── Signed Char
            │       │                       │   ╰── Initializer
            │       │                       │       ╰── <352> Subscript
            │       │                       │           ├── <349> Subscript
            │       │                       │           │   ├── <346> Subscript
            │       │                       │           │   │   ├── <343> Var [nested_auto]
            │       │                       │           │   │   ╰── <345> Var [i]
            │       │                       │           │   ╰── <348> Var [j]
            │       │                       │           ╰── <351> Var [k]
            │       │                       ├── VarDeclaration
            │       │                       │   ├── Name
            │       │                       │   │   ╰── expected
            │       │                       │   ├── Type
            │       │                       │   │   ╰── Signed Char
            │       │                       │   ╰── Initializer
            │       │                       │       ╰── <358> Constant Int [0]
            │       │                       ├── If
            │       │                       │   ├── Condition
            │       │                       │   │   ╰── <372>  [&&]
            │       │                       │   │       ├── <365>  [==]
            │       │                       │   │       │   ├── <362> Var [i]
            │       │                       │   │       │   ╰── <364> Constant Int [0]
            │       │                       │   │       ╰── <371>  [==]
            │       │                       │   │           ├── <368> Var [j]
            │       │                       │   │           ╰── <370> Constant Int [0]
            │       │                       │   ├── Then
            │       │                       │   │   ╰── Block
            │       │                       │   │       ╰── If
            │       │                       │   │           ├── Condition
            │       │                       │   │           │   ╰── <377>  [==]
            │       │                       │   │           │       ├── <374> Var [k]
            │       │                       │   │           │       ╰── <376> Constant Int [0]
            │       │                       │   │           ├── Then
            │       │                       │   │           │   ╰── Block
            │       │                       │   │           │       ╰── <382> Assign [=]
            │       │                       │   │           │           ├── <379> Var [expected]
            │       │                       │   │           │           ╰── <381> Constant Int [102]
            │       │                       │   │           ╰── Else
            │       │                       │   │               ╰── If
            │       │                       │   │                   ├── Condition
            │       │                       │   │                   │   ╰── <397>  [||]
            │       │                       │   │                   │       ├── <390>  [==]
            │       │                       │   │                   │       │   ├── <387> Var [k]
            │       │                       │   │                   │       │   ╰── <389> Constant Int [1]
            │       │                       │   │                   │       ╰── <396>  [==]
            │       │                       │   │                   │           ├── <393> Var [k]
            │       │                       │   │                   │           ╰── <395> Constant Int [2]
            │       │                       │   │                   ╰── Then
            │       │                       │   │                       ╰── Block
            │       │                       │   │                           ╰── <402> Assign [=]
            │       │                       │   │                               ├── <399> Var [expected]
            │       │                       │   │                               ╰── <401> Constant Int [111]
            │       │                       │   ╰── Else
            │       │                       │       ╰── If
            │       │                       │           ├── Condition
            │       │                       │           │   ╰── <428>  [&&]
            │       │                       │           │       ├── <421>  [&&]
            │       │                       │           │       │   ├── <414>  [==]
            │       │                       │           │       │   │   ├── <411> Var [i]
            │       │                       │           │       │   │   ╰── <413> Constant Int [1]
            │       │                       │           │       │   ╰── <420>  [==]
            │       │                       │           │       │       ├── <417> Var [j]
            │       │                       │           │       │       ╰── <419> Constant Int [0]
            │       │                       │           │       ╰── <427>  [==]
            │       │                       │           │           ├── <424> Var [k]
            │       │                       │           │           ╰── <426> Constant Int [0]
            │       │                       │           ├── Then
            │       │                       │           │   ╰── Block
            │       │                       │           │       ╰── <433> Assign [=]
            │       │                       │           │           ├── <430> Var [expected]
            │       │                       │           │           ╰── <432> Constant Int [120]
            │       │                       │           ╰── Else
            │       │                       │               ╰── If
            │       │                       │                   ├── Condition
            │       │                       │                   │   ╰── <455>  [&&]
            │       │                       │                   │       ├── <448>  [&&]
            │       │                       │                   │       │   ├── <441>  [==]
            │       │                       │                   │       │   │   ├── <438> Var [i]
            │       │                       │                   │       │   │   ╰── <440> Constant Int [1]
            │       │                       │                   │       │   ╰── <447>  [==]
            │       │                       │                   │       │       ├── <444> Var [j]
            │       │                       │                   │       │       ╰── <446> Constant Int [1]
            │       │                       │                   │       ╰── <454>  [==]
            │       │                       │                   │           ├── <451> Var [k]
            │       │                       │                   │           ╰── <453> Constant Int [0]
            │       │                       │                   ├── Then
            │       │                       │                   │   ╰── Block
            │       │                       │                   │       ╰── <460> Assign [=]
            │       │                       │                   │           ├── <457> Var [expected]
            │       │                       │                   │           ╰── <459> Constant Int [121]
            │       │                       │                   ╰── Else
            │       │                       │                       ╰── If
            │       │                       │                           ├── Condition
            │       │                       │                           │   ╰── <482>  [&&]
            │       │                       │                           │       ├── <475>  [&&]
            │       │                       │                           │       │   ├── <468>  [==]
            │       │                       │                           │       │   │   ├── <465> Var [i]
            │       │                       │                           │       │   │   ╰── <467> Constant Int [1]
            │       │                       │                           │       │   ╰── <474>  [==]
            │       │                       │                           │       │       ├── <471> Var [j]
            │       │                       │                           │       │       ╰── <473> Constant Int [1]
            │       │                       │                           │       ╰── <481>  [==]
            │       │                       │                           │           ├── <478> Var [k]
            │       │                       │                           │           ╰── <480> Constant Int [1]
            │       │                       │                           ╰── Then
            │       │                       │                               ╰── Block
            │       │                       │                                   ╰── <487> Assign [=]
            │       │                       │                                       ├── <484> Var [expected]
            │       │                       │                                       ╰── <486> Constant Int [122]
            │       │                       ╰── If
            │       │                           ├── Condition
            │       │                           │   ╰── <500>  [!=]
            │       │                           │       ├── <496> Var [c]
            │       │                           │       ╰── <499> Var [expected]
            │       │                           ╰── Then
            │       │                               ╰── Block
            │       │                                   ╰── Return
            │       │                                       ╰── <501> Constant Int [0]
            │       ╰── Return
            │           ╰── <515> Constant Int [1]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <527> Unary [!]
                    │   │       ╰── <526> FunctionCall [test_static]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <528> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <536> Unary [!]
                    │   │       ╰── <535> FunctionCall [test_static_nested]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <537> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <545> Unary [!]
                    │   │       ╰── <544> FunctionCall [test_automatic]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <546> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <554> Unary [!]
                    │   │       ╰── <553> FunctionCall [test_automatic_nested]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <555> Constant Int [4]
                    ╰── Return
                        ╰── <560> Constant Int [0]
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
                    │       ╰── <12> "abc"
                    ╰── Return
                        ╰── <18> Subscript
                            ├── <16> Var [chars]
                            ╰── <17> Constant Int [2]
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
            │       │   │   ╰── <29> "dog"
            │       │   ╰── Static
            │       ╰── Return
            │           ╰── <66>  [&&]
            │               ├── <56>  [&&]
            │               │   ├── <47>  [&&]
            │               │   │   ├── <38>  [==]
            │               │   │   │   ├── <35> Subscript
            │               │   │   │   │   ├── <33> Var [flat]
            │               │   │   │   │   ╰── <34> Constant Int [0]
            │               │   │   │   ╰── <37> Constant Int [100]
            │               │   │   ╰── <46>  [==]
            │               │   │       ├── <43> Subscript
            │               │   │       │   ├── <41> Var [flat]
            │               │   │       │   ╰── <42> Constant Int [1]
            │               │   │       ╰── <45> Constant Int [111]
            │               │   ╰── <55>  [==]
            │               │       ├── <52> Subscript
            │               │       │   ├── <50> Var [flat]
            │               │       │   ╰── <51> Constant Int [2]
            │               │       ╰── <54> Constant Int [103]
            │               ╰── <64>  [==]
            │                   ├── <61> Subscript
            │                   │   ├── <59> Var [flat]
            │                   │   ╰── <60> Constant Int [3]
            │                   ╰── <63> Constant Int [0]
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
            │       │   │       ├── <85> "yes"
            │       │   │       ╰── <87> "yup"
            │       │   ╰── Static
            │       ╰── Return
            │           ╰── <177>  [&&]
            │               ├── <165>  [&&]
            │               │   ├── <154>  [&&]
            │               │   │   ├── <143>  [&&]
            │               │   │   │   ├── <132>  [&&]
            │               │   │   │   │   ├── <121>  [&&]
            │               │   │   │   │   │   ├── <110>  [&&]
            │               │   │   │   │   │   │   ├── <99>  [==]
            │               │   │   │   │   │   │   │   ├── <96> Subscript
            │               │   │   │   │   │   │   │   │   ├── <94> Subscript
            │               │   │   │   │   │   │   │   │   │   ├── <92> Var [nested]
            │               │   │   │   │   │   │   │   │   │   ╰── <93> Constant Int [0]
            │               │   │   │   │   │   │   │   │   ╰── <95> Constant Int [0]
            │               │   │   │   │   │   │   │   ╰── <98> Constant Int [121]
            │               │   │   │   │   │   │   ╰── <109>  [==]
            │               │   │   │   │   │   │       ├── <106> Subscript
            │               │   │   │   │   │   │       │   ├── <104> Subscript
            │               │   │   │   │   │   │       │   │   ├── <102> Var [nested]
            │               │   │   │   │   │   │       │   │   ╰── <103> Constant Int [0]
            │               │   │   │   │   │   │       │   ╰── <105> Constant Int [1]
            │               │   │   │   │   │   │       ╰── <108> Constant Int [101]
            │               │   │   │   │   │   ╰── <120>  [==]
            │               │   │   │   │   │       ├── <117> Subscript
            │               │   │   │   │   │       │   ├── <115> Subscript
            │               │   │   │   │   │       │   │   ├── <113> Var [nested]
            │               │   │   │   │   │       │   │   ╰── <114> Constant Int [0]
            │               │   │   │   │   │       │   ╰── <116> Constant Int [2]
            │               │   │   │   │   │       ╰── <119> Constant Int [115]
            │               │   │   │   │   ╰── <131>  [==]
            │               │   │   │   │       ├── <128> Subscript
            │               │   │   │   │       │   ├── <126> Subscript
            │               │   │   │   │       │   │   ├── <124> Var [nested]
            │               │   │   │   │       │   │   ╰── <125> Constant Int [0]
            │               │   │   │   │       │   ╰── <127> Constant Int [3]
            │               │   │   │   │       ╰── <130> Constant Int [0]
            │               │   │   │   ╰── <142>  [==]
            │               │   │   │       ├── <139> Subscript
            │               │   │   │       │   ├── <137> Subscript
            │               │   │   │       │   │   ├── <135> Var [nested]
            │               │   │   │       │   │   ╰── <136> Constant Int [1]
            │               │   │   │       │   ╰── <138> Constant Int [0]
            │               │   │   │       ╰── <141> Constant Int [121]
            │               │   │   ╰── <153>  [==]
            │               │   │       ├── <150> Subscript
            │               │   │       │   ├── <148> Subscript
            │               │   │       │   │   ├── <146> Var [nested]
            │               │   │       │   │   ╰── <147> Constant Int [1]
            │               │   │       │   ╰── <149> Constant Int [1]
            │               │   │       ╰── <152> Constant Int [117]
            │               │   ╰── <164>  [==]
            │               │       ├── <161> Subscript
            │               │       │   ├── <159> Subscript
            │               │       │   │   ├── <157> Var [nested]
            │               │       │   │   ╰── <158> Constant Int [1]
            │               │       │   ╰── <160> Constant Int [2]
            │               │       ╰── <163> Constant Int [112]
            │               ╰── <175>  [==]
            │                   ├── <172> Subscript
            │                   │   ├── <170> Subscript
            │                   │   │   ├── <168> Var [nested]
            │                   │   │   ╰── <169> Constant Int [1]
            │                   │   ╰── <171> Constant Int [3]
            │                   ╰── <174> Constant Int [0]
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
            │       │       ╰── <192> "x"
            │       ╰── Return
            │           ╰── <211>  [&&]
            │               ├── <201>  [==]
            │               │   ├── <198> Subscript
            │               │   │   ├── <196> Var [flat_auto]
            │               │   │   ╰── <197> Constant Int [0]
            │               │   ╰── <200> Constant Int [120]
            │               ╰── <209>  [==]
            │                   ├── <206> Subscript
            │                   │   ├── <204> Var [flat_auto]
            │                   │   ╰── <205> Constant Int [1]
            │                   ╰── <208> Constant Int [0]
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
            │       │           │   ├── <232> "a"
            │       │           │   ╰── <234> "b"
            │       │           ╰── Compound
            │       │               ├── <237> "c"
            │       │               ╰── <239> "d"
            │       ╰── Return
            │           ╰── <346>  [&&]
            │               ├── <332>  [&&]
            │               │   ├── <319>  [&&]
            │               │   │   ├── <306>  [&&]
            │               │   │   │   ├── <293>  [&&]
            │               │   │   │   │   ├── <280>  [&&]
            │               │   │   │   │   │   ├── <267>  [&&]
            │               │   │   │   │   │   │   ├── <254>  [==]
            │               │   │   │   │   │   │   │   ├── <251> Subscript
            │               │   │   │   │   │   │   │   │   ├── <249> Subscript
            │               │   │   │   │   │   │   │   │   │   ├── <247> Subscript
            │               │   │   │   │   │   │   │   │   │   │   ├── <245> Var [nested_auto]
            │               │   │   │   │   │   │   │   │   │   │   ╰── <246> Constant Int [0]
            │               │   │   │   │   │   │   │   │   │   ╰── <248> Constant Int [0]
            │               │   │   │   │   │   │   │   │   ╰── <250> Constant Int [0]
            │               │   │   │   │   │   │   │   ╰── <253> Constant Int [97]
            │               │   │   │   │   │   │   ╰── <266>  [==]
            │               │   │   │   │   │   │       ├── <263> Subscript
            │               │   │   │   │   │   │       │   ├── <261> Subscript
            │               │   │   │   │   │   │       │   │   ├── <259> Subscript
            │               │   │   │   │   │   │       │   │   │   ├── <257> Var [nested_auto]
            │               │   │   │   │   │   │       │   │   │   ╰── <258> Constant Int [0]
            │               │   │   │   │   │   │       │   │   ╰── <260> Constant Int [0]
            │               │   │   │   │   │   │       │   ╰── <262> Constant Int [1]
            │               │   │   │   │   │   │       ╰── <265> Constant Int [0]
            │               │   │   │   │   │   ╰── <279>  [==]
            │               │   │   │   │   │       ├── <276> Subscript
            │               │   │   │   │   │       │   ├── <274> Subscript
            │               │   │   │   │   │       │   │   ├── <272> Subscript
            │               │   │   │   │   │       │   │   │   ├── <270> Var [nested_auto]
            │               │   │   │   │   │       │   │   │   ╰── <271> Constant Int [0]
            │               │   │   │   │   │       │   │   ╰── <273> Constant Int [1]
            │               │   │   │   │   │       │   ╰── <275> Constant Int [0]
            │               │   │   │   │   │       ╰── <278> Constant Int [98]
            │               │   │   │   │   ╰── <292>  [==]
            │               │   │   │   │       ├── <289> Subscript
            │               │   │   │   │       │   ├── <287> Subscript
            │               │   │   │   │       │   │   ├── <285> Subscript
            │               │   │   │   │       │   │   │   ├── <283> Var [nested_auto]
            │               │   │   │   │       │   │   │   ╰── <284> Constant Int [0]
            │               │   │   │   │       │   │   ╰── <286> Constant Int [1]
            │               │   │   │   │       │   ╰── <288> Constant Int [1]
            │               │   │   │   │       ╰── <291> Constant Int [0]
            │               │   │   │   ╰── <305>  [==]
            │               │   │   │       ├── <302> Subscript
            │               │   │   │       │   ├── <300> Subscript
            │               │   │   │       │   │   ├── <298> Subscript
            │               │   │   │       │   │   │   ├── <296> Var [nested_auto]
            │               │   │   │       │   │   │   ╰── <297> Constant Int [1]
            │               │   │   │       │   │   ╰── <299> Constant Int [0]
            │               │   │   │       │   ╰── <301> Constant Int [0]
            │               │   │   │       ╰── <304> Constant Int [99]
            │               │   │   ╰── <318>  [==]
            │               │   │       ├── <315> Subscript
            │               │   │       │   ├── <313> Subscript
            │               │   │       │   │   ├── <311> Subscript
            │               │   │       │   │   │   ├── <309> Var [nested_auto]
            │               │   │       │   │   │   ╰── <310> Constant Int [1]
            │               │   │       │   │   ╰── <312> Constant Int [0]
            │               │   │       │   ╰── <314> Constant Int [1]
            │               │   │       ╰── <317> Constant Int [0]
            │               │   ╰── <331>  [==]
            │               │       ├── <328> Subscript
            │               │       │   ├── <326> Subscript
            │               │       │   │   ├── <324> Subscript
            │               │       │   │   │   ├── <322> Var [nested_auto]
            │               │       │   │   │   ╰── <323> Constant Int [1]
            │               │       │   │   ╰── <325> Constant Int [1]
            │               │       │   ╰── <327> Constant Int [0]
            │               │       ╰── <330> Constant Int [100]
            │               ╰── <344>  [==]
            │                   ├── <341> Subscript
            │                   │   ├── <339> Subscript
            │                   │   │   ├── <337> Subscript
            │                   │   │   │   ├── <335> Var [nested_auto]
            │                   │   │   │   ╰── <336> Constant Int [1]
            │                   │   │   ╰── <338> Constant Int [1]
            │                   │   ╰── <340> Constant Int [1]
            │                   ╰── <343> Constant Int [0]
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
            │       │   │   ╰── <362> "abcd"
            │       │   ╰── Static
            │       ╰── Return
            │           ╰── <398>  [&&]
            │               ├── <389>  [&&]
            │               │   ├── <380>  [&&]
            │               │   │   ├── <371>  [==]
            │               │   │   │   ├── <368> Subscript
            │               │   │   │   │   ├── <366> Var [letters]
            │               │   │   │   │   ╰── <367> Constant Int [0]
            │               │   │   │   ╰── <370> Constant Int [97]
            │               │   │   ╰── <379>  [==]
            │               │   │       ├── <376> Subscript
            │               │   │       │   ├── <374> Var [letters]
            │               │   │       │   ╰── <375> Constant Int [1]
            │               │   │       ╰── <378> Constant Int [98]
            │               │   ╰── <388>  [==]
            │               │       ├── <385> Subscript
            │               │       │   ├── <383> Var [letters]
            │               │       │   ╰── <384> Constant Int [2]
            │               │       ╰── <387> Constant Int [99]
            │               ╰── <397>  [==]
            │                   ├── <394> Subscript
            │                   │   ├── <392> Var [letters]
            │                   │   ╰── <393> Constant Int [3]
            │                   ╰── <396> Constant Int [100]
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
            │           ├── <411> "yes"
            │           ├── <413> "no"
            │           ╰── <415> "ok"
            ├── Function [test_nested_static_without_null_byte]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── whole_array
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Char
            │       │   ╰── Initializer
            │       │       ╰── <434> Cast
            │       │           ├── Target
            │       │           │   ╰── Pointer
            │       │           │       ╰── Char
            │       │           ╰── Expression
            │       │               ╰── <433> Var [nested]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── word1
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Char
            │       │   ╰── Initializer
            │       │       ╰── <449> Cast
            │       │           ├── Target
            │       │           │   ╰── Pointer
            │       │           │       ╰── Char
            │       │           ╰── Expression
            │       │               ╰── <448> Subscript
            │       │                   ├── <446> Var [nested]
            │       │                   ╰── <447> Constant Int [0]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── word2
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Char
            │       │   ╰── Initializer
            │       │       ╰── <464> Cast
            │       │           ├── Target
            │       │           │   ╰── Pointer
            │       │           │       ╰── Char
            │       │           ╰── Expression
            │       │               ╰── <463> Subscript
            │       │                   ├── <461> Var [nested]
            │       │                   ╰── <462> Constant Int [1]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── word3
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Char
            │       │   ╰── Initializer
            │       │       ╰── <479> Cast
            │       │           ├── Target
            │       │           │   ╰── Pointer
            │       │           │       ╰── Char
            │       │           ╰── Expression
            │       │               ╰── <478> Subscript
            │       │                   ├── <476> Var [nested]
            │       │                   ╰── <477> Constant Int [2]
            │       ╰── Return
            │           ╰── <510> Unary [!]
            │               ╰── <509>  [||]
            │                   ├── <501>  [||]
            │                   │   ├── <494>  [||]
            │                   │   │   ├── <487> FunctionCall [strcmp]
            │                   │   │   │   ├── <485> Var [whole_array]
            │                   │   │   │   ╰── <486> "yesno"
            │                   │   │   ╰── <493> FunctionCall [strcmp]
            │                   │   │       ├── <491> Var [word1]
            │                   │   │       ╰── <492> "yesno"
            │                   │   ╰── <500> FunctionCall [strcmp]
            │                   │       ├── <498> Var [word2]
            │                   │       ╰── <499> "no"
            │                   ╰── <507> FunctionCall [strcmp]
            │                       ├── <505> Var [word3]
            │                       ╰── <506> "ok"
            ├── Function [test_flat_auto_without_null_byte]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── x
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <524> Unary [-]
            │       │           ╰── <523> Constant Int [1]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── letters
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 4
            │       │   │       ╰── Char
            │       │   ╰── Initializer
            │       │       ╰── <533> "abcd"
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── y
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <541> Unary [-]
            │       │           ╰── <540> Constant Int [1]
            │       ╰── Return
            │           ╰── <596>  [&&]
            │               ├── <586>  [&&]
            │               │   ├── <577>  [&&]
            │               │   │   ├── <568>  [&&]
            │               │   │   │   ├── <559>  [&&]
            │               │   │   │   │   ├── <550>  [==]
            │               │   │   │   │   │   ├── <545> Var [x]
            │               │   │   │   │   │   ╰── <549> Unary [-]
            │               │   │   │   │   │       ╰── <548> Constant Int [1]
            │               │   │   │   │   ╰── <558>  [==]
            │               │   │   │   │       ├── <553> Var [y]
            │               │   │   │   │       ╰── <557> Unary [-]
            │               │   │   │   │           ╰── <556> Constant Int [1]
            │               │   │   │   ╰── <567>  [==]
            │               │   │   │       ├── <564> Subscript
            │               │   │   │       │   ├── <562> Var [letters]
            │               │   │   │       │   ╰── <563> Constant Int [0]
            │               │   │   │       ╰── <566> Constant Int [97]
            │               │   │   ╰── <576>  [==]
            │               │   │       ├── <573> Subscript
            │               │   │       │   ├── <571> Var [letters]
            │               │   │       │   ╰── <572> Constant Int [1]
            │               │   │       ╰── <575> Constant Int [98]
            │               │   ╰── <585>  [==]
            │               │       ├── <582> Subscript
            │               │       │   ├── <580> Var [letters]
            │               │       │   ╰── <581> Constant Int [2]
            │               │       ╰── <584> Constant Int [99]
            │               ╰── <594>  [==]
            │                   ├── <591> Subscript
            │                   │   ├── <589> Var [letters]
            │                   │   ╰── <590> Constant Int [3]
            │                   ╰── <593> Constant Int [100]
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
            │       │           ├── <614> "yes"
            │       │           ├── <616> "no"
            │       │           ╰── <618> "ok"
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── whole_array
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Char
            │       │   ╰── Initializer
            │       │       ╰── <632> Cast
            │       │           ├── Target
            │       │           │   ╰── Pointer
            │       │           │       ╰── Char
            │       │           ╰── Expression
            │       │               ╰── <631> Var [nested]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── word1
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Char
            │       │   ╰── Initializer
            │       │       ╰── <647> Cast
            │       │           ├── Target
            │       │           │   ╰── Pointer
            │       │           │       ╰── Char
            │       │           ╰── Expression
            │       │               ╰── <646> Subscript
            │       │                   ├── <644> Var [nested]
            │       │                   ╰── <645> Constant Int [0]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── word2
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Char
            │       │   ╰── Initializer
            │       │       ╰── <662> Cast
            │       │           ├── Target
            │       │           │   ╰── Pointer
            │       │           │       ╰── Char
            │       │           ╰── Expression
            │       │               ╰── <661> Subscript
            │       │                   ├── <659> Var [nested]
            │       │                   ╰── <660> Constant Int [1]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── word3
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Char
            │       │   ╰── Initializer
            │       │       ╰── <677> Cast
            │       │           ├── Target
            │       │           │   ╰── Pointer
            │       │           │       ╰── Char
            │       │           ╰── Expression
            │       │               ╰── <676> Subscript
            │       │                   ├── <674> Var [nested]
            │       │                   ╰── <675> Constant Int [2]
            │       ╰── Return
            │           ╰── <708> Unary [!]
            │               ╰── <707>  [||]
            │                   ├── <699>  [||]
            │                   │   ├── <692>  [||]
            │                   │   │   ├── <685> FunctionCall [strcmp]
            │                   │   │   │   ├── <683> Var [whole_array]
            │                   │   │   │   ╰── <684> "yesno"
            │                   │   │   ╰── <691> FunctionCall [strcmp]
            │                   │   │       ├── <689> Var [word1]
            │                   │   │       ╰── <690> "yesno"
            │                   │   ╰── <698> FunctionCall [strcmp]
            │                   │       ├── <696> Var [word2]
            │                   │       ╰── <697> "no"
            │                   ╰── <705> FunctionCall [strcmp]
            │                       ├── <703> Var [word3]
            │                       ╰── <704> "ok"
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <720> Unary [!]
                    │   │       ╰── <719> FunctionCall [test_flat_static_with_null_byte]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <721> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <729> Unary [!]
                    │   │       ╰── <728> FunctionCall [test_nested_static_with_null_byte]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <730> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <738> Unary [!]
                    │   │       ╰── <737> FunctionCall [test_flat_auto_with_null_byte]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <739> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <747> Unary [!]
                    │   │       ╰── <746> FunctionCall [test_nested_auto_with_null_byte]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <748> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <756> Unary [!]
                    │   │       ╰── <755> FunctionCall [test_flat_static_without_null_byte]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <757> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <765> Unary [!]
                    │   │       ╰── <764> FunctionCall [test_nested_static_without_null_byte]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <766> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <774> Unary [!]
                    │   │       ╰── <773> FunctionCall [test_flat_auto_without_null_byte]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <775> Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <783> Unary [!]
                    │   │       ╰── <782> FunctionCall [test_nested_auto_without_null_byte]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <784> Constant Int [8]
                    ╰── Return
                        ╰── <789> Constant Int [0]
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
            │       │       ╰── <18> Cast
            │       │           ├── Target
            │       │           │   ╰── Unsigned Long
            │       │           ╰── Expression
            │       │               ╰── <17> Var [c]
            │       ╰── Return
            │           ╰── <29>  [==]
            │               ├── <25>  [%]
            │               │   ├── <22> Var [l]
            │               │   ╰── <24> Constant Int [16]
            │               ╰── <27> Constant Int [0]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── flat_static
            │   ├── Type
            │   │   ╰── Array
            │   │       ├── 16
            │   │       ╰── Signed Char
            │   ├── Initializer
            │   │   ╰── <40> "x"
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
            │   │       │   ╰── <56> "a"
            │   │       ╰── Compound
            │   │           ╰── <59> "b"
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
                    │   │   ╰── <95> Unary [!]
                    │   │       ╰── <94> FunctionCall [check_aligment]
                    │   │           ╰── <93> Cast
                    │   │               ├── Target
                    │   │               │   ╰── Pointer
                    │   │               │       ╰── Char
                    │   │               ╰── Expression
                    │   │                   ╰── <92> Var [flat_static]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <96> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <110> Unary [!]
                    │   │       ╰── <109> FunctionCall [check_aligment]
                    │   │           ╰── <108> Cast
                    │   │               ├── Target
                    │   │               │   ╰── Pointer
                    │   │               │       ╰── Char
                    │   │               ╰── Expression
                    │   │                   ╰── <107> Var [nested_static]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <111> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <125> Unary [!]
                    │   │       ╰── <124> FunctionCall [check_aligment]
                    │   │           ╰── <123> Cast
                    │   │               ├── Target
                    │   │               │   ╰── Pointer
                    │   │               │       ╰── Char
                    │   │               ╰── Expression
                    │   │                   ╰── <122> Var [flat_auto]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <126> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <140> Unary [!]
                    │   │       ╰── <139> FunctionCall [check_aligment]
                    │   │           ╰── <138> Cast
                    │   │               ├── Target
                    │   │               │   ╰── Pointer
                    │   │               │       ╰── Char
                    │   │               ╰── Expression
                    │   │                   ╰── <137> Var [nested_auto]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <141> Constant Int [4]
                    ╰── Return
                        ╰── <146> Constant Int [0]
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
                    │           ├── <31> "abcdefghijkl"
                    │           ╰── <33> "z"
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <43> FunctionCall [strcmp]
                    │   │       ├── <41> Subscript
                    │   │       │   ├── <39> Var [strings]
                    │   │       │   ╰── <40> Constant Int [0]
                    │   │       ╰── <42> "abcdefghijkl"
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <44> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <55>  [!=]
                    │   │       ├── <52> Subscript
                    │   │       │   ├── <50> Subscript
                    │   │       │   │   ├── <48> Var [strings]
                    │   │       │   │   ╰── <49> Constant Int [1]
                    │   │       │   ╰── <51> Constant Int [0]
                    │   │       ╰── <54> Constant Int [122]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <56> Constant Int [2]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <62> Constant Int [1]
                    │   ├── Condition
                    │   │   ╰── <70>  [<]
                    │   │       ├── <67> Var [i]
                    │   │       ╰── <69> Constant Int [13]
                    │   ├── Condition
                    │   │   ╰── <79> Assign [=]
                    │   │       ├── <72> Var [i]
                    │   │       ╰── <78>  [+]
                    │   │           ├── <75> Var [i]
                    │   │           ╰── <77> Constant Int [1]
                    │   ╰── Block
                    │       ╰── If
                    │           ├── Condition
                    │           │   ╰── <86> Subscript
                    │           │       ├── <83> Subscript
                    │           │       │   ├── <81> Var [strings]
                    │           │       │   ╰── <82> Constant Int [1]
                    │           │       ╰── <85> Var [i]
                    │           ╰── Then
                    │               ╰── Return
                    │                   ╰── <87> Constant Int [3]
                    ╰── Return
                        ╰── <93> Constant Int [0]
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
                    │       ╰── <23> "abc"
                    ├── <29> FunctionCall [puts]
                    │   ╰── <28> Var [flat_arr]
                    ├── <37> Assign [=]
                    │   ├── <34> Subscript
                    │   │   ├── <32> Var [flat_arr]
                    │   │   ╰── <33> Constant Int [2]
                    │   ╰── <36> Constant Int [120]
                    ├── <42> FunctionCall [puts]
                    │   ╰── <41> Var [flat_arr]
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
                    │           ├── <53> "Hello"
                    │           ╰── <55> "World"
                    ├── <64> FunctionCall [puts]
                    │   ╰── <63> Subscript
                    │       ├── <61> Var [nested_array]
                    │       ╰── <62> Constant Int [0]
                    ├── <71> FunctionCall [puts]
                    │   ╰── <70> Subscript
                    │       ├── <68> Var [nested_array]
                    │       ╰── <69> Constant Int [1]
                    ├── <81> Assign [=]
                    │   ├── <78> Subscript
                    │   │   ├── <76> Subscript
                    │   │   │   ├── <74> Var [nested_array]
                    │   │   │   ╰── <75> Constant Int [0]
                    │   │   ╰── <77> Constant Int [0]
                    │   ╰── <80> Constant Int [74]
                    ├── <88> FunctionCall [puts]
                    │   ╰── <87> Subscript
                    │       ├── <85> Var [nested_array]
                    │       ╰── <86> Constant Int [0]
                    ╰── Return
                        ╰── <90> Constant Int [0]
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
                    │       ╰── <27> AddressOf
                    │           ╰── <26> "Sample	string!
        "
                    ├── <34> FunctionCall [puts]
                    │   ╰── <33> Dereference
                    │       ╰── <32> Var [str]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── one_past_the_end
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Array
                    │   │           ├── 16
                    │   │           ╰── Char
                    │   ╰── Initializer
                    │       ╰── <49>  [+]
                    │           ├── <46> Var [str]
                    │           ╰── <48> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── last_byte_pointer
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── <65>  [-]
                    │           ├── <62> Cast
                    │           │   ├── Target
                    │           │   │   ╰── Pointer
                    │           │   │       ╰── Char
                    │           │   ╰── Expression
                    │           │       ╰── <61> Var [one_past_the_end]
                    │           ╰── <64> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <73>  [!=]
                    │   │       ├── <70> Dereference
                    │   │       │   ╰── <69> Var [last_byte_pointer]
                    │   │       ╰── <72> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <74> Constant Int [1]
                    ╰── Return
                        ╰── <79> Constant Int [0]
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
                    │       ╰── <22> "Hello, World"
                    ├── <28> FunctionCall [puts]
                    │   ╰── <27> Var [strings]
                    ╰── Return
                        ╰── <30> Constant Int [0]
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
                    │           ├── <30> "yes"
                    │           ├── <32> "no"
                    │           ╰── <34> "maybe"
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <44> FunctionCall [strcmp]
                    │   │       ├── <42> Subscript
                    │   │       │   ├── <40> Var [strings]
                    │   │       │   ╰── <41> Constant Int [0]
                    │   │       ╰── <43> "yes"
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <45> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <56> FunctionCall [strcmp]
                    │   │       ├── <54> Subscript
                    │   │       │   ├── <52> Var [strings]
                    │   │       │   ╰── <53> Constant Int [1]
                    │   │       ╰── <55> "no"
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <57> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <68> FunctionCall [strcmp]
                    │   │       ├── <66> Subscript
                    │   │       │   ├── <64> Var [strings]
                    │   │       │   ╰── <65> Constant Int [2]
                    │   │       ╰── <67> "maybe"
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <69> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <77> Subscript
                    │   │       ├── <75> Var [strings]
                    │   │       ╰── <76> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <78> Constant Int [4]
                    ╰── Return
                        ╰── <83> Constant Int [0]
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
                    │       ╰── <11> "This is a string!"
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── uc
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Unsigned Char
                    │   ╰── Initializer
                    │       ╰── <24> Cast
                    │           ├── Target
                    │           │   ╰── Pointer
                    │           │       ╰── Unsigned Char
                    │           ╰── Expression
                    │               ╰── <23> Var [c]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <33>  [!=]
                    │   │       ├── <30> Subscript
                    │   │       │   ├── <28> Var [uc]
                    │   │       │   ╰── <29> Constant Int [3]
                    │   │       ╰── <32> Constant Int [115]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <34> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── sc
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Signed Char
                    │   ╰── Initializer
                    │       ╰── <49> Cast
                    │           ├── Target
                    │           │   ╰── Pointer
                    │           │       ╰── Signed Char
                    │           ╰── Expression
                    │               ╰── <48> Var [c]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <58>  [!=]
                    │   │       ├── <55> Subscript
                    │   │       │   ├── <53> Var [sc]
                    │   │       │   ╰── <54> Constant Int [3]
                    │   │       ╰── <57> Constant Int [115]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <59> Constant Int [2]
                    ╰── Return
                        ╰── <64> Constant Int [0]
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
                    │       ╰── <11> ""
                    ╰── Return
                        ╰── <17> Subscript
                            ├── <15> Var [empty]
                            ╰── <16> Constant Int [0]
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
                    │   │   ╰── <11>  [!=]
                    │   │       ├── <8> Subscript
                    │   │       │   ├── <6> "abcdefg"
                    │   │       │   ╰── <7> Constant Int [2]
                    │   │       ╰── <10> Constant Int [99]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <12> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── <25>  [+]
                    │           ├── <22> "This is a string!"
                    │           ╰── <24> Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <33>  [!=]
                    │   │       ├── <30> Dereference
                    │   │       │   ╰── <29> Var [ptr]
                    │   │       ╰── <32> Constant Int [115]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <34> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <45>  [!=]
                    │   │       ├── <42> Subscript
                    │   │       │   ├── <40> Var [ptr]
                    │   │       │   ╰── <41> Constant Int [6]
                    │   │       ╰── <44> Constant Int [33]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <46> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <54> Subscript
                    │   │       ├── <52> Var [ptr]
                    │   │       ╰── <53> Constant Int [7]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <55> Constant Int [4]
                    ╰── If
                        ├── Condition
                        │   ╰── <62> Unary [!]
                        │       ╰── <61> "Not a null pointer!"
                        ╰── Then
                            ╰── Block
                                ╰── Return
                                    ╰── <63> Constant Int [5]
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
                    │       ╰── <11> "Hello, World!"
                    ╰── Return
                        ╰── <17> Subscript
                            ├── <15> Var [x]
                            ╰── <16> Constant Int [2]
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
                    │   │   ╰── <58> FunctionCall [strcmp]
                    │   │       ├── <56> "abc"
                    │   │       ╰── <57> "abc"
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <59> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <70>  [>=]
                    │   │       ├── <67> FunctionCall [strcmp]
                    │   │       │   ├── <65> "ab"
                    │   │       │   ╰── <66> "xy"
                    │   │       ╰── <69> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <71> Constant Int [2]
                    ├── <78> FunctionCall [puts]
                    │   ╰── <77> "Hello, World!"
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <82> FunctionCall [strlen]
                    │   │       ╰── <81> ""
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <83> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <93> FunctionCall [atoi]
                    │           ╰── <92> "10"
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <100>  [!=]
                    │   │       ├── <97> Var [i]
                    │   │       ╰── <99> Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <101> Constant Int [4]
                    ╰── Return
                        ╰── <106> Constant Int [0]
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
                    │       ╰── <38> ""
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <47>  [!=]
                    │   │       ├── <44> Subscript
                    │   │       │   ├── <42> Var [escape_sequence]
                    │   │       │   ╰── <43> Constant Int [0]
                    │   │       ╰── <46> Constant Int [7]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <48> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <59>  [!=]
                    │   │       ├── <56> Subscript
                    │   │       │   ├── <54> Var [escape_sequence]
                    │   │       │   ╰── <55> Constant Int [1]
                    │   │       ╰── <58> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <60> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <68> Subscript
                    │   │       ├── <66> Var [escape_sequence]
                    │   │       ╰── <67> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <69> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── with_double_quote
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── <79> "Hello"world"
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <88>  [!=]
                    │   │       ├── <85> Subscript
                    │   │       │   ├── <83> Var [with_double_quote]
                    │   │       │   ╰── <84> Constant Int [5]
                    │   │       ╰── <87> Constant Int [34]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <89> Constant Int [4]
                    ├── <97> FunctionCall [puts]
                    │   ╰── <96> Var [with_double_quote]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── with_backslash
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── <104> "Hello\World"
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <113>  [!=]
                    │   │       ├── <110> Subscript
                    │   │       │   ├── <108> Var [with_backslash]
                    │   │       │   ╰── <109> Constant Int [5]
                    │   │       ╰── <112> Constant Int [92]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <114> Constant Int [5]
                    ├── <122> FunctionCall [puts]
                    │   ╰── <121> Var [with_backslash]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── with_newline
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── <129> "Line
        break!"
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <138>  [!=]
                    │   │       ├── <135> Subscript
                    │   │       │   ├── <133> Var [with_newline]
                    │   │       │   ╰── <134> Constant Int [4]
                    │   │       ╰── <137> Constant Int [10]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <139> Constant Int [6]
                    ├── <147> FunctionCall [puts]
                    │   ╰── <146> Var [with_newline]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── tab
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── <154> "	"
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <161> FunctionCall [strcmp]
                    │   │       ├── <159> Var [tab]
                    │   │       ╰── <160> "	"
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <162> Constant Int [7]
                    ├── <169> FunctionCall [puts]
                    │   ╰── <168> "Testing, 123."
                    ├── <173> FunctionCall [puts]
                    │   ╰── <172> "^@1 _\]"
                    ╰── Return
                        ╰── <175> Constant Int [0]
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
            │           ╰── <19> "I'm a string!"
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
            │       │   │   ╰── <49>  [||]
            │       │   │       ├── <42>  [==]
            │       │   │       │   ├── <39> Var [s1]
            │       │   │       │   ╰── <41> Constant Int [0]
            │       │   │       ╰── <48>  [==]
            │       │   │           ├── <45> Var [s2]
            │       │   │           ╰── <47> Constant Int [0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <50> Constant Int [0]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <61>  [!=]
            │       │   │       ├── <58> FunctionCall [strlen]
            │       │   │       │   ╰── <57> Var [s1]
            │       │   │       ╰── <60> Constant Int [45]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <62> Constant Int [0]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <91>  [||]
            │       │   │       ├── <82>  [||]
            │       │   │       │   ├── <73>  [!=]
            │       │   │       │   │   ├── <70> Subscript
            │       │   │       │   │   │   ├── <68> Var [s1]
            │       │   │       │   │   │   ╰── <69> Constant Int [41]
            │       │   │       │   │   ╰── <72> Constant Int [100]
            │       │   │       │   ╰── <81>  [!=]
            │       │   │       │       ├── <78> Subscript
            │       │   │       │       │   ├── <76> Var [s1]
            │       │   │       │       │   ╰── <77> Constant Int [42]
            │       │   │       │       ╰── <80> Constant Int [111]
            │       │   │       ╰── <90>  [!=]
            │       │   │           ├── <87> Subscript
            │       │   │           │   ├── <85> Var [s1]
            │       │   │           │   ╰── <86> Constant Int [43]
            │       │   │           ╰── <89> Constant Int [103]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <92> Constant Int [0]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <100> Subscript
            │       │   │       ├── <98> Var [s2]
            │       │   │       ╰── <99> Constant Int [0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <101> Constant Int [0]
            │       ╰── Return
            │           ╰── <106> Constant Int [1]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Char
                    │   ╰── Initializer
                    │       ╰── <120> Constant Int [0]
                    ├── <128> Assign [=]
                    │   ├── <124> Var [ptr]
                    │   ╰── <127> FunctionCall [return_string]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <133> Unary [!]
                    │   │       ╰── <132> Var [ptr]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <134> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <158>  [||]
                    │   │       ├── <152>  [||]
                    │   │       │   ├── <143>  [!=]
                    │   │       │   │   ├── <140> Subscript
                    │   │       │   │   │   ├── <138> Var [ptr]
                    │   │       │   │   │   ╰── <139> Constant Int [0]
                    │   │       │   │   ╰── <142> Constant Int [73]
                    │   │       │   ╰── <151>  [!=]
                    │   │       │       ├── <148> Subscript
                    │   │       │       │   ├── <146> Var [ptr]
                    │   │       │       │   ╰── <147> Constant Int [1]
                    │   │       │       ╰── <150> Constant Int [39]
                    │   │       ╰── <157> Subscript
                    │   │           ├── <155> Var [ptr]
                    │   │           ╰── <156> Constant Int [13]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <159> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <169> Unary [!]
                    │   │       ╰── <168> FunctionCall [pass_string_args]
                    │   │           ├── <166> "The quick brown fox jumped over the lazy dog."
                    │   │           ╰── <167> ""
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <170> Constant Int [3]
                    ├── Return
                    │   ╰── <175> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr2
                    │   ╰── Type
                    │       ╰── Pointer
                    │           ╰── Char
                    ├── <198> Assign [=]
                    │   ├── <184> Var [ptr2]
                    │   ╰── <197> Conditional [?]
                    │       ├── <186> Constant Int [1]
                    │       ├── Then
                    │       │   ╰── <191>  [+]
                    │       │       ├── <188> Var [ptr]
                    │       │       ╰── <190> Constant Int [2]
                    │       ╰── Else
                    │           ╰── <196>  [+]
                    │               ├── <193> Var [ptr]
                    │               ╰── <195> Constant Int [4]
                    ╰── Return
                        ╰── <205>  [==]
                            ├── <202> Dereference
                            │   ╰── <201> Var [ptr2]
                            ╰── <204> Constant Int [109]
    "#;
    assert_parse(src, expected);
}
