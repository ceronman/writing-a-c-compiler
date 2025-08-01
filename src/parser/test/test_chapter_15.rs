use super::{assert_error, assert_parse};

#[test]
fn test_invalid_parse_array_of_functions() {
    assert_error(
        r#"
        int foo[3](int a);
                //^ Expected ';', but found '('
    "#,
    );
}

#[test]
fn test_invalid_parse_array_of_functions_2() {
    assert_error(
        r#"
        
        int (foo[3])(int a);
          //^^^^^^^^ Can't apply additional derivations to a function type
    "#,
    );
}

#[test]
fn test_invalid_parse_double_declarator() {
    assert_error(
        r#"
        int main(void) {
            int x[2.0];
                //^^^ Array size should be an integer constant
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_empty_initializer_list() {
    assert_error(
        r#"
        int main(void) {
            int arr[1] = {};
                        //^ Expected expression, but found '}'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_malformed_abstract_array_declarator() {
    assert_error(
        r#"
        int main(void) {
            return (int[3] *)0;
                         //^ Expected ')', but found '*'
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_malformed_abstract_array_declarator_2() {
    assert_error(
        r#"
        int main(void) {
            return (int[3](*))0;
                        //^ Expected ')', but found '('
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_malformed_array_declarator() {
    assert_error(
        r#"
        int main(void) {
            int foo[[10]];
                  //^ Expected expression, but found '['
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_malformed_array_declarator_2() {
    assert_error(
        r#"
        int main(void) {
            int (*)(ptr_to_array[3]) = 0;
                //^ Expected identifier, but found ')'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_malformed_array_declarator_3() {
    assert_error(
        r#"
        int main(void) {
            int [3] arr = {1, 2, 3};
              //^ Expected identifier, but found '['
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_malformed_type_name() {
    assert_error(
        r#"
        int main(void) {
            int a = 4;
            int *foo = &a;
            int *bar[3] = (*[3]) foo;
                          //^ Expected expression, but found '['
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_malformed_type_name_2() {
    assert_error(
        r#"
        int main(void) {
            int *ptr;
            int *array_pointer[3] = ([3](*)) ptr;
                                   //^ Expected expression, but found '['
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_mismatched_subscript() {
    assert_error(
        r#"
        int main(void) {
            int indices[3] = {1, 2, 3};
            int vals[3] = {4, 5, 6};
            return vals[indices[1];
                                //^ Expected ']', but found ';'
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_negative_array_dimension() {
    assert_error(
        r#"
        int main(void)
        {
            int arr[-3];
                  //^^ Array size should be a constant
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_parenthesized_array_of_functions() {
    assert_error(
        r#"
        int(foo[3])(int a);
         //^^^^^^^^ Can't apply additional derivations to a function type
    "#,
    );
}

#[test]
fn test_invalid_parse_return_array() {
    assert_error(
        r#"
        
        int foo(void)[3];
                   //^ Expected ';', but found '['
    "#,
    );
}

#[test]
fn test_invalid_parse_unclosed_initializer() {
    assert_error(
        r#"
        int main(void) {
            int arr = {1, 2;
                         //^ Expected '}', but found ';'
            return arr[0];
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_unclosed_nested_initializer() {
    assert_error(
        r#"
        int main(void) {
            int arr[2][2] = {{ 1, 2}, {3, 4};
                                          //^ Expected '}', but found ';'
            return arr[0][0];
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_unclosed_subscript() {
    assert_error(
        r#"
        int main(void) {
            int arr[] = {1, 2, 3};
                  //^ Expected expression, but found ']'
            return arr[1;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_add_two_pointers() {
    let src = r#"
        int main(void)
        {
            int *x = 0;
            int *y = 0;
            return (x + y == 0);
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
                    │       ╰── <9> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── y
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <16> Constant Int [0]
                    ╰── Return
                        ╰── <28>  [==]
                            ├── <24>  [+]
                            │   ├── <20> Var [x]
                            │   ╰── <23> Var [y]
                            ╰── <26> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_assign_incompatible_pointer_types() {
    let src = r#"
        int main(void) {
            int four_element_array[4] = {1, 2, 3, 4};
            int (*arr)[3] = &four_element_array;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── four_element_array
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 4
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <10> Constant Int [1]
                    │           ├── <12> Constant Int [2]
                    │           ├── <14> Constant Int [3]
                    │           ╰── <16> Constant Int [4]
                    ╰── VarDeclaration
                        ├── Name
                        │   ╰── arr
                        ├── Type
                        │   ╰── Pointer
                        │       ╰── Array
                        │           ├── 3
                        │           ╰── Int
                        ╰── Initializer
                            ╰── <29> AddressOf
                                ╰── <28> Var [four_element_array]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_assign_to_array() {
    let src = r#"
        int main(void)
        {
            int arr[3] = {1, 2, 3};
            int arr2[3] = {4, 5, 6};
            arr = arr2;
            return arr[0];
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
                    │   │   ╰── arr2
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 3
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <23> Constant Int [4]
                    │           ├── <25> Constant Int [5]
                    │           ╰── <27> Constant Int [6]
                    ├── <36> Assign [=]
                    │   ├── <32> Var [arr]
                    │   ╰── <35> Var [arr2]
                    ╰── Return
                        ╰── <41> Subscript
                            ├── <39> Var [arr]
                            ╰── <40> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_assign_to_array_2() {
    let src = r#"
        int main(void)
        {
            int dim2[1][2] = {{1, 2}};
            int dim[2] = {3, 4};
            dim2[0] = dim;
            return dim[0];
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── dim2
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 1
                    │   │       ╰── Array
                    │   │           ├── 2
                    │   │           ╰── Int
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ╰── Compound
                    │               ├── <12> Constant Int [1]
                    │               ╰── <14> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── dim
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 2
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <24> Constant Int [3]
                    │           ╰── <26> Constant Int [4]
                    ├── <37> Assign [=]
                    │   ├── <33> Subscript
                    │   │   ├── <31> Var [dim2]
                    │   │   ╰── <32> Constant Int [0]
                    │   ╰── <36> Var [dim]
                    ╰── Return
                        ╰── <42> Subscript
                            ├── <40> Var [dim]
                            ╰── <41> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_assign_to_array_3() {
    let src = r#"
        int main(void) {
            int arr[3] = { 1, 2, 3};
            int (*ptr_to_array)[3];
            *ptr_to_array = arr;
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
                    │   │   ╰── ptr_to_array
                    │   ╰── Type
                    │       ╰── Pointer
                    │           ╰── Array
                    │               ├── 3
                    │               ╰── Int
                    ╰── <32> Assign [=]
                        ├── <28> Dereference
                        │   ╰── <27> Var [ptr_to_array]
                        ╰── <31> Var [arr]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_bad_arg_type() {
    let src = r#"
        int foo(int **x) {
            return x[0][0];
        }
        int main(void) {
            int arr[1] = {10};
            return foo(&arr);
        }
    "#;
    let expected = r#"
        Program
            ├── Function [foo]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── x
            │   │       ╰── Type
            │   │           ╰── Pointer
            │   │               ╰── Pointer
            │   │                   ╰── Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <15> Subscript
            │               ├── <13> Subscript
            │               │   ├── <11> Var [x]
            │               │   ╰── <12> Constant Int [0]
            │               ╰── <14> Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── arr
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 1
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ╰── <28> Constant Int [10]
                    ╰── Return
                        ╰── <36> FunctionCall [foo]
                            ╰── <35> AddressOf
                                ╰── <34> Var [arr]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_cast_to_array_type() {
    let src = r#"
        int main(void)
        {
            int arr[10];
            return (int[10])arr;
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
                    │           ├── 10
                    │           ╰── Int
                    ╰── Return
                        ╰── <18> Cast
                            ├── Target
                            │   ╰── Array
                            │       ├── 10
                            │       ╰── Int
                            ╰── Expression
                                ╰── <17> Var [arr]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_cast_to_array_type_2() {
    let src = r#"
        int main(void)
        {
            long arr[10];
            return (int *[10])arr;
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
                    │           ├── 10
                    │           ╰── Long
                    ╰── Return
                        ╰── <19> Cast
                            ├── Target
                            │   ╰── Array
                            │       ├── 10
                            │       ╰── Pointer
                            │           ╰── Int
                            ╰── Expression
                                ╰── <18> Var [arr]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_cast_to_array_type_3() {
    let src = r#"
        int main(void)
        {
            long arr[6];
            return ((long(([2])[3]))arr);
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
                    │           ├── 6
                    │           ╰── Long
                    ╰── Return
                        ╰── <23> Cast
                            ├── Target
                            │   ╰── Array
                            │       ├── 2
                            │       ╰── Array
                            │           ├── 3
                            │           ╰── Long
                            ╰── Expression
                                ╰── <21> Var [arr]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_compare_different_pointer_types() {
    let src = r#"
        int main(void)
        {
            long x = 10;
            long *ptr = &x + 1;
            long(*array_ptr)[10] = (long (*)[10]) &x;
            return array_ptr < ptr;
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
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <8> Constant Int [10]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Long
                    │   ╰── Initializer
                    │       ╰── <20>  [+]
                    │           ├── <17> AddressOf
                    │           │   ╰── <16> Var [x]
                    │           ╰── <19> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── array_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Array
                    │   │           ├── 10
                    │   │           ╰── Long
                    │   ╰── Initializer
                    │       ╰── <39> Cast
                    │           ├── Target
                    │           │   ╰── Pointer
                    │           │       ╰── Array
                    │           │           ├── 10
                    │           │           ╰── Long
                    │           ╰── Expression
                    │               ╰── <38> AddressOf
                    │                   ╰── <37> Var [x]
                    ╰── Return
                        ╰── <47>  [<]
                            ├── <43> Var [array_ptr]
                            ╰── <46> Var [ptr]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_compare_explicit_and_implict_addr() {
    let src = r#"
        int main(void)
        {
            int arr[10];
            return arr == &arr;
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
                    │           ├── 10
                    │           ╰── Int
                    ╰── Return
                        ╰── <17>  [==]
                            ├── <12> Var [arr]
                            ╰── <16> AddressOf
                                ╰── <15> Var [arr]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_compare_pointer_to_int() {
    let src = r#"
        int main(void)
        {
            long *l = 0;
            return l <= 100ul;
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
                    │   │   ╰── Pointer
                    │   │       ╰── Long
                    │   ╰── Initializer
                    │       ╰── <9> Constant Int [0]
                    ╰── Return
                        ╰── <16>  [<=]
                            ├── <13> Var [l]
                            ╰── <15> Constant ULong [100]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_compare_pointer_to_zero() {
    let src = r#"
        int main(void)
        {
            int *x = 0;
            return x > 0;
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
                    │       ╰── <9> Constant Int [0]
                    ╰── Return
                        ╰── <16>  [>]
                            ├── <13> Var [x]
                            ╰── <15> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_compound_initializer_for_scalar() {
    let src = r#"
        int main(void)
        {
            int x = {1, 2, 3};
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
                    │       ╰── Compound
                    │           ├── <8> Constant Int [1]
                    │           ├── <10> Constant Int [2]
                    │           ╰── <12> Constant Int [3]
                    ╰── Return
                        ╰── <17> Var [x]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_compound_initializer_for_static_scalar() {
    let src = r#"
        int main(void)
        {
            static int x = {1, 2, 3};
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
                    │   ├── Initializer
                    │   │   ╰── Compound
                    │   │       ├── <9> Constant Int [1]
                    │   │       ├── <11> Constant Int [2]
                    │   │       ╰── <13> Constant Int [3]
                    │   ╰── Static
                    ╰── Return
                        ╰── <18> Var [x]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_compound_initializer_too_long_static() {
    let src = r#"
        int main(void) {
            static int arr[3] = {1, 2, 3, 4};
            return arr[2];
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
                    │   ├── Initializer
                    │   │   ╰── Compound
                    │   │       ├── <11> Constant Int [1]
                    │   │       ├── <13> Constant Int [2]
                    │   │       ├── <15> Constant Int [3]
                    │   │       ╰── <17> Constant Int [4]
                    │   ╰── Static
                    ╰── Return
                        ╰── <24> Subscript
                            ├── <22> Var [arr]
                            ╰── <23> Constant Int [2]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_compound_inititializer_too_long() {
    let src = r#"
        int main(void) {
            int arr[3] = {1, 2, 3, 4};
            return arr[2];
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
                    │           ├── <14> Constant Int [3]
                    │           ╰── <16> Constant Int [4]
                    ╰── Return
                        ╰── <23> Subscript
                            ├── <21> Var [arr]
                            ╰── <22> Constant Int [2]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_conflicting_array_declarations() {
    let src = r#"
        int arr[6];
        int main(void) {
            return arr[0];
        }
        int arr[5];
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── arr
            │   ╰── Type
            │       ╰── Array
            │           ├── 6
            │           ╰── Int
            ├── Function [main]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <14> Subscript
            │               ├── <12> Var [arr]
            │               ╰── <13> Constant Int [0]
            ╰── VarDeclaration
                ├── Name
                │   ╰── arr
                ╰── Type
                    ╰── Array
                        ├── 5
                        ╰── Int
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_conflicting_function_declarations() {
    let src = r#"
        int f(int arr[2][3]);
        int f(int arr[2][4]);
    "#;
    let expected = r#"
        Program
            ├── Function [f]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── arr
            │           ╰── Type
            │               ╰── Array
            │                   ├── 2
            │                   ╰── Array
            │                       ├── 3
            │                       ╰── Int
            ╰── Function [f]
                ╰── Parameters
                    ╰── Param
                        ├── Name
                        │   ╰── arr
                        ╰── Type
                            ╰── Array
                                ├── 2
                                ╰── Array
                                    ├── 4
                                    ╰── Int
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_double_subscript() {
    let src = r#"
        int main(void) {
            int arr[3] = {4, 5, 6};
            return arr[2.0];
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
                    │           ├── <10> Constant Int [4]
                    │           ├── <12> Constant Int [5]
                    │           ╰── <14> Constant Int [6]
                    ╰── Return
                        ╰── <21> Subscript
                            ├── <19> Var [arr]
                            ╰── <20> Constant Double [+2e0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_compound_add_double_to_pointer() {
    let src = r#"
        
        int main(void) {
            int arr[3] = {1, 2, 3};
            int *elem = arr;
            elem += 1.0;
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
                    │   │       ├── 3
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <10> Constant Int [1]
                    │           ├── <12> Constant Int [2]
                    │           ╰── <14> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── elem
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <23> Var [arr]
                    ├── <30> Assign [+=]
                    │   ├── <27> Var [elem]
                    │   ╰── <29> Constant Double [+1e0]
                    ╰── Return
                        ╰── <32> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_compound_add_two_pointers() {
    let src = r#"
        
        int main(void) {
            int arr[3] = {1, 2, 3};
            int *elem0 = arr;
            int *elem1 = arr + 1;
            elem0 += elem1;
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
                    │   │       ├── 3
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <10> Constant Int [1]
                    │           ├── <12> Constant Int [2]
                    │           ╰── <14> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── elem0
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <23> Var [arr]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── elem1
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <34>  [+]
                    │           ├── <31> Var [arr]
                    │           ╰── <33> Constant Int [1]
                    ├── <42> Assign [+=]
                    │   ├── <38> Var [elem0]
                    │   ╰── <41> Var [elem1]
                    ╰── Return
                        ╰── <44> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_compound_assign_to_array() {
    let src = r#"
        int main(void) {
            int arr[3] = {1, 2, 3};
            arr -= 1;
            0;
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
                    ├── <22> Assign [-=]
                    │   ├── <19> Var [arr]
                    │   ╰── <21> Constant Int [1]
                    ╰── <24> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_compound_assign_to_nested_array() {
    let src = r#"
        int main(void) {
            long arr[2][2] = {{1, 2}, {3, 4}};
            arr[1] += 1;
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
                    │   │       ├── 2
                    │   │       ╰── Array
                    │   │           ├── 2
                    │   │           ╰── Long
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── Compound
                    │           │   ├── <12> Constant Int [1]
                    │           │   ╰── <14> Constant Int [2]
                    │           ╰── Compound
                    │               ├── <17> Constant Int [3]
                    │               ╰── <19> Constant Int [4]
                    ├── <30> Assign [+=]
                    │   ├── <27> Subscript
                    │   │   ├── <25> Var [arr]
                    │   │   ╰── <26> Constant Int [1]
                    │   ╰── <29> Constant Int [1]
                    ╰── Return
                        ╰── <32> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_compound_sub_pointer_from_int() {
    let src = r#"
        
        int main(void) {
            int arr[3] = {1, 2, 3};
            int *elem = arr + 1;
            int i = 0;
            i -= elem;
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
                    │   │       ├── 3
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <10> Constant Int [1]
                    │           ├── <12> Constant Int [2]
                    │           ╰── <14> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── elem
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <26>  [+]
                    │           ├── <23> Var [arr]
                    │           ╰── <25> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <32> Constant Int [0]
                    ├── <40> Assign [-=]
                    │   ├── <36> Var [i]
                    │   ╰── <39> Var [elem]
                    ╰── Return
                        ╰── <42> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_postfix_incr_array() {
    let src = r#"
        int main(void) {
            int arr[3] = {1, 2, 3};
            arr++;
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
                    │   │       ├── 3
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <10> Constant Int [1]
                    │           ├── <12> Constant Int [2]
                    │           ╰── <14> Constant Int [3]
                    ├── <21> Postfix [++]
                    │   ╰── <19> Var [arr]
                    ╰── Return
                        ╰── <23> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_postfix_incr_nested_array() {
    let src = r#"
        int main(void) {
            int arr[2][3] = {{1, 2, 3}, {4, 5, 6}};
            arr[2]++;
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
                    │   │       ├── 2
                    │   │       ╰── Array
                    │   │           ├── 3
                    │   │           ╰── Int
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── Compound
                    │           │   ├── <12> Constant Int [1]
                    │           │   ├── <14> Constant Int [2]
                    │           │   ╰── <16> Constant Int [3]
                    │           ╰── Compound
                    │               ├── <19> Constant Int [4]
                    │               ├── <21> Constant Int [5]
                    │               ╰── <23> Constant Int [6]
                    ├── <33> Postfix [++]
                    │   ╰── <31> Subscript
                    │       ├── <29> Var [arr]
                    │       ╰── <30> Constant Int [2]
                    ╰── Return
                        ╰── <35> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_prefix_decr_array() {
    let src = r#"
        int main(void) {
            int arr[3] = {1, 2, 3};
            --arr;
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
                    │   │       ├── 3
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <10> Constant Int [1]
                    │           ├── <12> Constant Int [2]
                    │           ╰── <14> Constant Int [3]
                    ├── <21> Unary [--]
                    │   ╰── <20> Var [arr]
                    ╰── Return
                        ╰── <23> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_prefix_decr_nested_array() {
    let src = r#"
        int main(void) {
            int arr[2][3] = {{1, 2, 3}, {4, 5, 6}};
            --arr[2];
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
                    │   │       ├── 2
                    │   │       ╰── Array
                    │   │           ├── 3
                    │   │           ╰── Int
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── Compound
                    │           │   ├── <12> Constant Int [1]
                    │           │   ├── <14> Constant Int [2]
                    │           │   ╰── <16> Constant Int [3]
                    │           ╰── Compound
                    │               ├── <19> Constant Int [4]
                    │               ├── <21> Constant Int [5]
                    │               ╰── <23> Constant Int [6]
                    ├── <33> Unary [--]
                    │   ╰── <32> Subscript
                    │       ├── <30> Var [arr]
                    │       ╰── <31> Constant Int [2]
                    ╰── Return
                        ╰── <35> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_extra_credit_switch_on_array() {
    let src = r#"
        
        int main(void) {
            int arr[3] = {1, 2, 3};
            switch (arr) {
                default:
                    return 0;
            }
            return 1;
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
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <19> Var [arr]
                    │   ╰── Block
                    │       ╰── Default
                    │           ╰── Return
                    │               ╰── <20> Constant Int [0]
                    ╰── Return
                        ╰── <26> Constant Int [1]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_function_returns_array() {
    let src = r#"
        int(foo(void))[3][4];
    "#;
    let expected = r#"
        Program
            ╰── Function [foo]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_incompatible_elem_type_compound_init() {
    let src = r#"
        int main(void)
        {
            int *arr[3] = {0, 0, 1.0};
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── VarDeclaration
                        ├── Name
                        │   ╰── arr
                        ├── Type
                        │   ╰── Array
                        │       ├── 3
                        │       ╰── Pointer
                        │           ╰── Int
                        ╰── Initializer
                            ╰── Compound
                                ├── <11> Constant Int [0]
                                ├── <13> Constant Int [0]
                                ╰── <15> Constant Double [+1e0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_incompatible_elem_type_static_compound_init() {
    let src = r#"
        
        int *arr[3] = {0, 0, 1.0};
        int main(void)
        {
            return 0;
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
            │   │       ╰── Pointer
            │   │           ╰── Int
            │   ╰── Initializer
            │       ╰── Compound
            │           ├── <7> Constant Int [0]
            │           ├── <9> Constant Int [0]
            │           ╰── <11> Constant Double [+1e0]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <19> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_null_ptr_array_initializer() {
    let src = r#"
        int main(void)
        {
            int arr[1] = 0;
            return arr[0];
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
                    │   │       ├── 1
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <10> Constant Int [0]
                    ╰── Return
                        ╰── <16> Subscript
                            ├── <14> Var [arr]
                            ╰── <15> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_null_ptr_static_array_initializer() {
    let src = r#"
        int main(void)
        {
            static int arr[1] = 0;
            return arr[0];
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
                    │   │       ├── 1
                    │   │       ╰── Int
                    │   ├── Initializer
                    │   │   ╰── <11> Constant Int [0]
                    │   ╰── Static
                    ╰── Return
                        ╰── <17> Subscript
                            ├── <15> Var [arr]
                            ╰── <16> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_scalar_initializer_for_array() {
    let src = r#"
        int main(void)
        {
            int arr[1] = 4;
            return arr[0];
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
                    │   │       ├── 1
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <10> Constant Int [4]
                    ╰── Return
                        ╰── <16> Subscript
                            ├── <14> Var [arr]
                            ╰── <15> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_scalar_initializer_for_static_array() {
    let src = r#"
        
        double arr[3] = 1.0;
        int main(void)
        {
            return 0;
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
            │   │       ╰── Double
            │   ╰── Initializer
            │       ╰── <6> Constant Double [+1e0]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <13> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_static_non_const_array() {
    let src = r#"
        int foo(int p) {
            static int arr[3] = { p, p + 1, 0};
            return arr[2];
        }
        int main(void) {
            return foo(5);
        }
    "#;
    let expected = r#"
        Program
            ├── Function [foo]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── p
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── arr
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 3
            │       │   │       ╰── Int
            │       │   ├── Initializer
            │       │   │   ╰── Compound
            │       │   │       ├── <15> Var [p]
            │       │   │       ├── <21>  [+]
            │       │   │       │   ├── <18> Var [p]
            │       │   │       │   ╰── <20> Constant Int [1]
            │       │   │       ╰── <23> Constant Int [0]
            │       │   ╰── Static
            │       ╰── Return
            │           ╰── <30> Subscript
            │               ├── <28> Var [arr]
            │               ╰── <29> Constant Int [2]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <40> FunctionCall [foo]
                            ╰── <39> Constant Int [5]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_sub_different_pointer_types() {
    let src = r#"
        int main(void)
        {
            long x[10];
            long *ptr = x;
            unsigned long *ptr2 = (unsigned long *)ptr;
            return ptr - ptr2;
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
                    │       ╰── Array
                    │           ├── 10
                    │           ╰── Long
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Long
                    │   ╰── Initializer
                    │       ╰── <16> Var [x]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr2
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <28> Cast
                    │           ├── Target
                    │           │   ╰── Pointer
                    │           │       ╰── Unsigned Long
                    │           ╰── Expression
                    │               ╰── <27> Var [ptr]
                    ╰── Return
                        ╰── <36>  [-]
                            ├── <32> Var [ptr]
                            ╰── <35> Var [ptr2]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_sub_double_from_ptr() {
    let src = r#"
        int main(void)
        {
            int *y = 0;
            return (y - 0.0 == 0.0);
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
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <9> Constant Int [0]
                    ╰── Return
                        ╰── <20>  [==]
                            ├── <16>  [-]
                            │   ├── <13> Var [y]
                            │   ╰── <15> Constant Double [+0e0]
                            ╰── <18> Constant Double [+0e0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_sub_ptr_from_int() {
    let src = r#"
        int main(void)
        {
            int *x = 0;
            return 0 - x == 0;
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
                    │       ╰── <9> Constant Int [0]
                    ╰── Return
                        ╰── <19>  [==]
                            ├── <16>  [-]
                            │   ├── <12> Constant Int [0]
                            │   ╰── <15> Var [x]
                            ╰── <18> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_subscript_both_pointers() {
    let src = r#"
        int main(void)
        {
            int x = 10;
            int *ptr = &x;
            int *subscript = 0;
            return ptr[subscript];
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
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <17> AddressOf
                    │           ╰── <16> Var [x]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── subscript
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <24> Constant Int [0]
                    ╰── Return
                        ╰── <31> Subscript
                            ├── <28> Var [ptr]
                            ╰── <30> Var [subscript]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_types_subscript_non_ptr() {
    let src = r#"
        int main(void) {
            int a = 3;
            return a[4];
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
                    │       ╰── <8> Constant Int [3]
                    ╰── Return
                        ╰── <14> Subscript
                            ├── <12> Var [a]
                            ╰── <13> Constant Int [4]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_allocation_test_alignment() {
    let src = r#"
        int check_alignment(int *ptr) {
            unsigned long addr = (unsigned long) ptr;
            return (addr % 16 == 0);
        }
        int main(void)
        {
            int arr[5] = {0};
            int arr2[7] = {0};
            int arr3[2][2] = {{0}};
            if (!check_alignment(arr)) {
                return 1;
            }
            for (int i = 0; i < 5; i = i + 1)
                arr[i] = i;
            if (!check_alignment(arr2)) {
                return 2;
            }
            for (int i = 0; i < 7; i = i + 1)
                if (arr2[i])
                    return 3;
            for (int i = 0; i < 7; i = i + 1){
                arr2[i] = -i;
            }
            if (!check_alignment((int *)arr3)) {
                return 4;
            }
            for (int i = 0; i < 5; i = i + 1) {
                if (arr[i] != i) {
                    return 5;
                }
            }
            for (int i = 0; i < 2; i = i + 1)
                for (int j = 0; j < 2; j = j + 1)
                    if (arr3[i][j] != 0)
                        return 6;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [check_alignment]
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
            │       │   │   ╰── addr
            │       │   ├── Type
            │       │   │   ╰── Unsigned Long
            │       │   ╰── Initializer
            │       │       ╰── <16> Cast
            │       │           ├── Target
            │       │           │   ╰── Unsigned Long
            │       │           ╰── Expression
            │       │               ╰── <15> Var [ptr]
            │       ╰── Return
            │           ╰── <27>  [==]
            │               ├── <23>  [%]
            │               │   ├── <20> Var [addr]
            │               │   ╰── <22> Constant Int [16]
            │               ╰── <25> Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── arr
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 5
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ╰── <40> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── arr2
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 7
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ╰── <49> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── arr3
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 2
                    │   │       ╰── Array
                    │   │           ├── 2
                    │   │           ╰── Int
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ╰── Compound
                    │               ╰── <60> Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <70> Unary [!]
                    │   │       ╰── <69> FunctionCall [check_alignment]
                    │   │           ╰── <68> Var [arr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <71> Constant Int [1]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <79> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <87>  [<]
                    │   │       ├── <84> Var [i]
                    │   │       ╰── <86> Constant Int [5]
                    │   ├── Condition
                    │   │   ╰── <96> Assign [=]
                    │   │       ├── <89> Var [i]
                    │   │       ╰── <95>  [+]
                    │   │           ├── <92> Var [i]
                    │   │           ╰── <94> Constant Int [1]
                    │   ╰── <105> Assign [=]
                    │       ├── <101> Subscript
                    │       │   ├── <98> Var [arr]
                    │       │   ╰── <100> Var [i]
                    │       ╰── <104> Var [i]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <113> Unary [!]
                    │   │       ╰── <112> FunctionCall [check_alignment]
                    │   │           ╰── <111> Var [arr2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <114> Constant Int [2]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <122> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <130>  [<]
                    │   │       ├── <127> Var [i]
                    │   │       ╰── <129> Constant Int [7]
                    │   ├── Condition
                    │   │   ╰── <139> Assign [=]
                    │   │       ├── <132> Var [i]
                    │   │       ╰── <138>  [+]
                    │   │           ├── <135> Var [i]
                    │   │           ╰── <137> Constant Int [1]
                    │   ╰── If
                    │       ├── Condition
                    │       │   ╰── <144> Subscript
                    │       │       ├── <141> Var [arr2]
                    │       │       ╰── <143> Var [i]
                    │       ╰── Then
                    │           ╰── Return
                    │               ╰── <145> Constant Int [3]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <152> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <160>  [<]
                    │   │       ├── <157> Var [i]
                    │   │       ╰── <159> Constant Int [7]
                    │   ├── Condition
                    │   │   ╰── <169> Assign [=]
                    │   │       ├── <162> Var [i]
                    │   │       ╰── <168>  [+]
                    │   │           ├── <165> Var [i]
                    │   │           ╰── <167> Constant Int [1]
                    │   ╰── Block
                    │       ╰── <180> Assign [=]
                    │           ├── <174> Subscript
                    │           │   ├── <171> Var [arr2]
                    │           │   ╰── <173> Var [i]
                    │           ╰── <179> Unary [-]
                    │               ╰── <178> Var [i]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <194> Unary [!]
                    │   │       ╰── <193> FunctionCall [check_alignment]
                    │   │           ╰── <192> Cast
                    │   │               ├── Target
                    │   │               │   ╰── Pointer
                    │   │               │       ╰── Int
                    │   │               ╰── Expression
                    │   │                   ╰── <191> Var [arr3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <195> Constant Int [4]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <203> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <211>  [<]
                    │   │       ├── <208> Var [i]
                    │   │       ╰── <210> Constant Int [5]
                    │   ├── Condition
                    │   │   ╰── <220> Assign [=]
                    │   │       ├── <213> Var [i]
                    │   │       ╰── <219>  [+]
                    │   │           ├── <216> Var [i]
                    │   │           ╰── <218> Constant Int [1]
                    │   ╰── Block
                    │       ╰── If
                    │           ├── Condition
                    │           │   ╰── <229>  [!=]
                    │           │       ├── <225> Subscript
                    │           │       │   ├── <222> Var [arr]
                    │           │       │   ╰── <224> Var [i]
                    │           │       ╰── <228> Var [i]
                    │           ╰── Then
                    │               ╰── Block
                    │                   ╰── Return
                    │                       ╰── <230> Constant Int [5]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <241> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <249>  [<]
                    │   │       ├── <246> Var [i]
                    │   │       ╰── <248> Constant Int [2]
                    │   ├── Condition
                    │   │   ╰── <258> Assign [=]
                    │   │       ├── <251> Var [i]
                    │   │       ╰── <257>  [+]
                    │   │           ├── <254> Var [i]
                    │   │           ╰── <256> Constant Int [1]
                    │   ╰── For
                    │       ├── Init
                    │       │   ╰── VarDeclaration
                    │       │       ├── Name
                    │       │       │   ╰── j
                    │       │       ├── Type
                    │       │       │   ╰── Int
                    │       │       ╰── Initializer
                    │       │           ╰── <262> Constant Int [0]
                    │       ├── Condition
                    │       │   ╰── <270>  [<]
                    │       │       ├── <267> Var [j]
                    │       │       ╰── <269> Constant Int [2]
                    │       ├── Condition
                    │       │   ╰── <279> Assign [=]
                    │       │       ├── <272> Var [j]
                    │       │       ╰── <278>  [+]
                    │       │           ├── <275> Var [j]
                    │       │           ╰── <277> Constant Int [1]
                    │       ╰── If
                    │           ├── Condition
                    │           │   ╰── <290>  [!=]
                    │           │       ├── <287> Subscript
                    │           │       │   ├── <284> Subscript
                    │           │       │   │   ├── <281> Var [arr3]
                    │           │       │   │   ╰── <283> Var [i]
                    │           │       │   ╰── <286> Var [j]
                    │           │       ╰── <289> Constant Int [0]
                    │           ╰── Then
                    │               ╰── Return
                    │                   ╰── <291> Constant Int [6]
                    ╰── Return
                        ╰── <296> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_casts_cast_array_of_pointers() {
    let src = r#"
        int main(void) {
            int simple_array[2] = {1, 2};
            int(*ptr_arr[3])[2] = {&simple_array, 0, &simple_array};
            long *other_ptr = (long *)ptr_arr;
            return (int(**)[2])other_ptr == ptr_arr;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── simple_array
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 2
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <10> Constant Int [1]
                    │           ╰── <12> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr_arr
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 3
                    │   │       ╰── Pointer
                    │   │           ╰── Array
                    │   │               ├── 2
                    │   │               ╰── Int
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <27> AddressOf
                    │           │   ╰── <26> Var [simple_array]
                    │           ├── <29> Constant Int [0]
                    │           ╰── <33> AddressOf
                    │               ╰── <32> Var [simple_array]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── other_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Long
                    │   ╰── Initializer
                    │       ╰── <46> Cast
                    │           ├── Target
                    │           │   ╰── Pointer
                    │           │       ╰── Long
                    │           ╰── Expression
                    │               ╰── <45> Var [ptr_arr]
                    ╰── Return
                        ╰── <62>  [==]
                            ├── <58> Cast
                            │   ├── Target
                            │   │   ╰── Pointer
                            │   │       ╰── Pointer
                            │   │           ╰── Array
                            │   │               ├── 2
                            │   │               ╰── Int
                            │   ╰── Expression
                            │       ╰── <57> Var [other_ptr]
                            ╰── <61> Var [ptr_arr]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_casts_implicit_and_explicit_conversions() {
    let src = r#"
        int main(void) {
            long arr[4] = {-1,-2,-3,-4};
            if (arr != (long *) arr) {
                return 1;
            }
            if ((long (*)[4]) arr != &arr) {
                return 2;
            }
            unsigned long *unsigned_arr = (unsigned long *)arr;
            if (unsigned_arr[0] != 18446744073709551615UL) {
                return 3;
            }
            if (unsigned_arr[3] != 18446744073709551612UL) {
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
                    │   │   ╰── arr
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 4
                    │   │       ╰── Long
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <12> Unary [-]
                    │           │   ╰── <11> Constant Int [1]
                    │           ├── <16> Unary [-]
                    │           │   ╰── <15> Constant Int [2]
                    │           ├── <20> Unary [-]
                    │           │   ╰── <19> Constant Int [3]
                    │           ╰── <24> Unary [-]
                    │               ╰── <23> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <37>  [!=]
                    │   │       ├── <29> Var [arr]
                    │   │       ╰── <36> Cast
                    │   │           ├── Target
                    │   │           │   ╰── Pointer
                    │   │           │       ╰── Long
                    │   │           ╰── Expression
                    │   │               ╰── <35> Var [arr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <38> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <56>  [!=]
                    │   │       ├── <51> Cast
                    │   │       │   ├── Target
                    │   │       │   │   ╰── Pointer
                    │   │       │   │       ╰── Array
                    │   │       │   │           ├── 4
                    │   │       │   │           ╰── Long
                    │   │       │   ╰── Expression
                    │   │       │       ╰── <50> Var [arr]
                    │   │       ╰── <55> AddressOf
                    │   │           ╰── <54> Var [arr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <57> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── unsigned_arr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <71> Cast
                    │           ├── Target
                    │           │   ╰── Pointer
                    │           │       ╰── Unsigned Long
                    │           ╰── Expression
                    │               ╰── <70> Var [arr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <80>  [!=]
                    │   │       ├── <77> Subscript
                    │   │       │   ├── <75> Var [unsigned_arr]
                    │   │       │   ╰── <76> Constant Int [0]
                    │   │       ╰── <79> Constant ULong [18446744073709551615]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <81> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <92>  [!=]
                    │   │       ├── <89> Subscript
                    │   │       │   ├── <87> Var [unsigned_arr]
                    │   │       │   ╰── <88> Constant Int [3]
                    │   │       ╰── <91> Constant ULong [18446744073709551612]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <93> Constant Int [4]
                    ╰── Return
                        ╰── <98> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_casts_multi_dim_casts() {
    let src = r#"
        int main(void) {
            int multi_dim[2][3] = {{0, 1, 2}, {3, 4, 5}};
            int (*array_pointer)[2][3] = &multi_dim;
            int (*row_pointer)[3] = (int (*)[3]) array_pointer;
            if (row_pointer != multi_dim) {
                return 1;
            }
            row_pointer = row_pointer + 1;
            if (row_pointer[0][1] != 4) {
                return 2;
            }
            int *elem_ptr = (int *) row_pointer;
            if (*elem_ptr != 3 ){
                return 3;
            }
            elem_ptr = elem_ptr + 2;
            if (*elem_ptr != 5) {
                return 4;
            }
            row_pointer = row_pointer - 1;
            if ((int (*)[2][3]) row_pointer != array_pointer) {
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
                    │   │   ╰── multi_dim
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 2
                    │   │       ╰── Array
                    │   │           ├── 3
                    │   │           ╰── Int
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── Compound
                    │           │   ├── <12> Constant Int [0]
                    │           │   ├── <14> Constant Int [1]
                    │           │   ╰── <16> Constant Int [2]
                    │           ╰── Compound
                    │               ├── <19> Constant Int [3]
                    │               ├── <21> Constant Int [4]
                    │               ╰── <23> Constant Int [5]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── array_pointer
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Array
                    │   │           ├── 2
                    │   │           ╰── Array
                    │   │               ├── 3
                    │   │               ╰── Int
                    │   ╰── Initializer
                    │       ╰── <39> AddressOf
                    │           ╰── <38> Var [multi_dim]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── row_pointer
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Array
                    │   │           ├── 3
                    │   │           ╰── Int
                    │   ╰── Initializer
                    │       ╰── <57> Cast
                    │           ├── Target
                    │           │   ╰── Pointer
                    │           │       ╰── Array
                    │           │           ├── 3
                    │           │           ╰── Int
                    │           ╰── Expression
                    │               ╰── <56> Var [array_pointer]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <65>  [!=]
                    │   │       ├── <61> Var [row_pointer]
                    │   │       ╰── <64> Var [multi_dim]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <66> Constant Int [1]
                    ├── <79> Assign [=]
                    │   ├── <72> Var [row_pointer]
                    │   ╰── <78>  [+]
                    │       ├── <75> Var [row_pointer]
                    │       ╰── <77> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <89>  [!=]
                    │   │       ├── <86> Subscript
                    │   │       │   ├── <84> Subscript
                    │   │       │   │   ├── <82> Var [row_pointer]
                    │   │       │   │   ╰── <83> Constant Int [0]
                    │   │       │   ╰── <85> Constant Int [1]
                    │   │       ╰── <88> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <90> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── elem_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <104> Cast
                    │           ├── Target
                    │           │   ╰── Pointer
                    │           │       ╰── Int
                    │           ╰── Expression
                    │               ╰── <103> Var [row_pointer]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <112>  [!=]
                    │   │       ├── <109> Dereference
                    │   │       │   ╰── <108> Var [elem_ptr]
                    │   │       ╰── <111> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <113> Constant Int [3]
                    ├── <126> Assign [=]
                    │   ├── <119> Var [elem_ptr]
                    │   ╰── <125>  [+]
                    │       ├── <122> Var [elem_ptr]
                    │       ╰── <124> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <133>  [!=]
                    │   │       ├── <130> Dereference
                    │   │       │   ╰── <129> Var [elem_ptr]
                    │   │       ╰── <132> Constant Int [5]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <134> Constant Int [4]
                    ├── <147> Assign [=]
                    │   ├── <140> Var [row_pointer]
                    │   ╰── <146>  [-]
                    │       ├── <143> Var [row_pointer]
                    │       ╰── <145> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <163>  [!=]
                    │   │       ├── <159> Cast
                    │   │       │   ├── Target
                    │   │       │   │   ╰── Pointer
                    │   │       │   │       ╰── Array
                    │   │       │   │           ├── 2
                    │   │       │   │           ╰── Array
                    │   │       │   │               ├── 3
                    │   │       │   │               ╰── Int
                    │   │       │   ╰── Expression
                    │   │       │       ╰── <158> Var [row_pointer]
                    │   │       ╰── <162> Var [array_pointer]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <164> Constant Int [5]
                    ╰── Return
                        ╰── <169> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_declarators_array_as_argument() {
    let src = r#"
        int array_param(int a[5]) {
            a[4] = 0;
            return 0;
        }
        int nested_array_param(int a[2][3]) {
            a[1][1] = 1;
            return 0;
        }
        int array_param(int a[2]);
        int nested_array_param(int (*a)[3]);
        int main(void) {
            int array_param(int a[6]);
            int nested_array_param(int a[5][3]);
            int arr[8] = {8, 7, 6, 5, 4, 3, 2, 1};
            array_param(arr);
            if (arr[4]) {
                return 1;
            }
            for (int i = 0; i < 8; i = i + 1) {
                if (i != 4 && arr[i] != 8 - i)
                    return 2;
            }
            int nested_arr[4][3] = { {-1, -1, -1}, {-2, -2, -2}, {-3, -3, -3}, {-4, -4, -4}};
            nested_array_param(nested_arr);
            if (nested_arr[1][1] != 1) {
                return 3;
            }
            for (int i = 0; i < 4; i = i + 1) {
                int expected = -1 - i;
                for (int j = 0; j < 3; j = j + 1) {
                    if ((i != 1 || j != 1) &&
                        (nested_arr[i][j] != expected)) {
                            return 4;
                    }
                }
            }
            return 0;
        }
        int array_param(int *a);
    "#;
    let expected = r#"
        Program
            ├── Function [array_param]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── a
            │   │       ╰── Type
            │   │           ╰── Array
            │   │               ├── 5
            │   │               ╰── Int
            │   ╰── Body
            │       ├── <16> Assign [=]
            │       │   ├── <13> Subscript
            │       │   │   ├── <11> Var [a]
            │       │   │   ╰── <12> Constant Int [4]
            │       │   ╰── <15> Constant Int [0]
            │       ╰── Return
            │           ╰── <18> Constant Int [0]
            ├── Function [nested_array_param]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── a
            │   │       ╰── Type
            │   │           ╰── Array
            │   │               ├── 2
            │   │               ╰── Array
            │   │                   ├── 3
            │   │                   ╰── Int
            │   ╰── Body
            │       ├── <41> Assign [=]
            │       │   ├── <38> Subscript
            │       │   │   ├── <36> Subscript
            │       │   │   │   ├── <34> Var [a]
            │       │   │   │   ╰── <35> Constant Int [1]
            │       │   │   ╰── <37> Constant Int [1]
            │       │   ╰── <40> Constant Int [1]
            │       ╰── Return
            │           ╰── <43> Constant Int [0]
            ├── Function [array_param]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── a
            │           ╰── Type
            │               ╰── Array
            │                   ├── 2
            │                   ╰── Int
            ├── Function [nested_array_param]
            │   ╰── Parameters
            │       ╰── Param
            │           ├── Name
            │           │   ╰── a
            │           ╰── Type
            │               ╰── Pointer
            │                   ╰── Array
            │                       ├── 3
            │                       ╰── Int
            ├── Function [main]
            │   ╰── Body
            │       ├── Function [array_param]
            │       │   ╰── Parameters
            │       │       ╰── Param
            │       │           ├── Name
            │       │           │   ╰── a
            │       │           ╰── Type
            │       │               ╰── Array
            │       │                   ├── 6
            │       │                   ╰── Int
            │       ├── Function [nested_array_param]
            │       │   ╰── Parameters
            │       │       ╰── Param
            │       │           ├── Name
            │       │           │   ╰── a
            │       │           ╰── Type
            │       │               ╰── Array
            │       │                   ├── 5
            │       │                   ╰── Array
            │       │                       ├── 3
            │       │                       ╰── Int
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── arr
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 8
            │       │   │       ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── Compound
            │       │           ├── <100> Constant Int [8]
            │       │           ├── <102> Constant Int [7]
            │       │           ├── <104> Constant Int [6]
            │       │           ├── <106> Constant Int [5]
            │       │           ├── <108> Constant Int [4]
            │       │           ├── <110> Constant Int [3]
            │       │           ├── <112> Constant Int [2]
            │       │           ╰── <114> Constant Int [1]
            │       ├── <121> FunctionCall [array_param]
            │       │   ╰── <120> Var [arr]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <126> Subscript
            │       │   │       ├── <124> Var [arr]
            │       │   │       ╰── <125> Constant Int [4]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <127> Constant Int [1]
            │       ├── For
            │       │   ├── Init
            │       │   │   ╰── VarDeclaration
            │       │   │       ├── Name
            │       │   │       │   ╰── i
            │       │   │       ├── Type
            │       │   │       │   ╰── Int
            │       │   │       ╰── Initializer
            │       │   │           ╰── <135> Constant Int [0]
            │       │   ├── Condition
            │       │   │   ╰── <143>  [<]
            │       │   │       ├── <140> Var [i]
            │       │   │       ╰── <142> Constant Int [8]
            │       │   ├── Condition
            │       │   │   ╰── <152> Assign [=]
            │       │   │       ├── <145> Var [i]
            │       │   │       ╰── <151>  [+]
            │       │   │           ├── <148> Var [i]
            │       │   │           ╰── <150> Constant Int [1]
            │       │   ╰── Block
            │       │       ╰── If
            │       │           ├── Condition
            │       │           │   ╰── <171>  [&&]
            │       │           │       ├── <157>  [!=]
            │       │           │       │   ├── <154> Var [i]
            │       │           │       │   ╰── <156> Constant Int [4]
            │       │           │       ╰── <170>  [!=]
            │       │           │           ├── <163> Subscript
            │       │           │           │   ├── <160> Var [arr]
            │       │           │           │   ╰── <162> Var [i]
            │       │           │           ╰── <169>  [-]
            │       │           │               ├── <165> Constant Int [8]
            │       │           │               ╰── <168> Var [i]
            │       │           ╰── Then
            │       │               ╰── Return
            │       │                   ╰── <172> Constant Int [2]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── nested_arr
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 4
            │       │   │       ╰── Array
            │       │   │           ├── 3
            │       │   │           ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── Compound
            │       │           ├── Compound
            │       │           │   ├── <187> Unary [-]
            │       │           │   │   ╰── <186> Constant Int [1]
            │       │           │   ├── <191> Unary [-]
            │       │           │   │   ╰── <190> Constant Int [1]
            │       │           │   ╰── <195> Unary [-]
            │       │           │       ╰── <194> Constant Int [1]
            │       │           ├── Compound
            │       │           │   ├── <200> Unary [-]
            │       │           │   │   ╰── <199> Constant Int [2]
            │       │           │   ├── <204> Unary [-]
            │       │           │   │   ╰── <203> Constant Int [2]
            │       │           │   ╰── <208> Unary [-]
            │       │           │       ╰── <207> Constant Int [2]
            │       │           ├── Compound
            │       │           │   ├── <213> Unary [-]
            │       │           │   │   ╰── <212> Constant Int [3]
            │       │           │   ├── <217> Unary [-]
            │       │           │   │   ╰── <216> Constant Int [3]
            │       │           │   ╰── <221> Unary [-]
            │       │           │       ╰── <220> Constant Int [3]
            │       │           ╰── Compound
            │       │               ├── <226> Unary [-]
            │       │               │   ╰── <225> Constant Int [4]
            │       │               ├── <230> Unary [-]
            │       │               │   ╰── <229> Constant Int [4]
            │       │               ╰── <234> Unary [-]
            │       │                   ╰── <233> Constant Int [4]
            │       ├── <242> FunctionCall [nested_array_param]
            │       │   ╰── <241> Var [nested_arr]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <252>  [!=]
            │       │   │       ├── <249> Subscript
            │       │   │       │   ├── <247> Subscript
            │       │   │       │   │   ├── <245> Var [nested_arr]
            │       │   │       │   │   ╰── <246> Constant Int [1]
            │       │   │       │   ╰── <248> Constant Int [1]
            │       │   │       ╰── <251> Constant Int [1]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <253> Constant Int [3]
            │       ├── For
            │       │   ├── Init
            │       │   │   ╰── VarDeclaration
            │       │   │       ├── Name
            │       │   │       │   ╰── i
            │       │   │       ├── Type
            │       │   │       │   ╰── Int
            │       │   │       ╰── Initializer
            │       │   │           ╰── <261> Constant Int [0]
            │       │   ├── Condition
            │       │   │   ╰── <269>  [<]
            │       │   │       ├── <266> Var [i]
            │       │   │       ╰── <268> Constant Int [4]
            │       │   ├── Condition
            │       │   │   ╰── <278> Assign [=]
            │       │   │       ├── <271> Var [i]
            │       │   │       ╰── <277>  [+]
            │       │   │           ├── <274> Var [i]
            │       │   │           ╰── <276> Constant Int [1]
            │       │   ╰── Block
            │       │       ├── VarDeclaration
            │       │       │   ├── Name
            │       │       │   │   ╰── expected
            │       │       │   ├── Type
            │       │       │   │   ╰── Int
            │       │       │   ╰── Initializer
            │       │       │       ╰── <288>  [-]
            │       │       │           ├── <284> Unary [-]
            │       │       │           │   ╰── <283> Constant Int [1]
            │       │       │           ╰── <287> Var [i]
            │       │       ╰── For
            │       │           ├── Init
            │       │           │   ╰── VarDeclaration
            │       │           │       ├── Name
            │       │           │       │   ╰── j
            │       │           │       ├── Type
            │       │           │       │   ╰── Int
            │       │           │       ╰── Initializer
            │       │           │           ╰── <294> Constant Int [0]
            │       │           ├── Condition
            │       │           │   ╰── <302>  [<]
            │       │           │       ├── <299> Var [j]
            │       │           │       ╰── <301> Constant Int [3]
            │       │           ├── Condition
            │       │           │   ╰── <311> Assign [=]
            │       │           │       ├── <304> Var [j]
            │       │           │       ╰── <310>  [+]
            │       │           │           ├── <307> Var [j]
            │       │           │           ╰── <309> Constant Int [1]
            │       │           ╰── Block
            │       │               ╰── If
            │       │                   ├── Condition
            │       │                   │   ╰── <339>  [&&]
            │       │                   │       ├── <324>  [||]
            │       │                   │       │   ├── <316>  [!=]
            │       │                   │       │   │   ├── <313> Var [i]
            │       │                   │       │   │   ╰── <315> Constant Int [1]
            │       │                   │       │   ╰── <322>  [!=]
            │       │                   │       │       ├── <319> Var [j]
            │       │                   │       │       ╰── <321> Constant Int [1]
            │       │                   │       ╰── <338>  [!=]
            │       │                   │           ├── <333> Subscript
            │       │                   │           │   ├── <330> Subscript
            │       │                   │           │   │   ├── <327> Var [nested_arr]
            │       │                   │           │   │   ╰── <329> Var [i]
            │       │                   │           │   ╰── <332> Var [j]
            │       │                   │           ╰── <336> Var [expected]
            │       │                   ╰── Then
            │       │                       ╰── Block
            │       │                           ╰── Return
            │       │                               ╰── <340> Constant Int [4]
            │       ╰── Return
            │           ╰── <351> Constant Int [0]
            ╰── Function [array_param]
                ╰── Parameters
                    ╰── Param
                        ├── Name
                        │   ╰── a
                        ╰── Type
                            ╰── Pointer
                                ╰── Int
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_declarators_big_array() {
    let src = r#"
        extern int x[4294967297L][100000000];
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
            │   │   ╰── Array
            │   │       ├── 4294967297
            │   │       ╰── Array
            │   │           ├── 100000000
            │   │           ╰── Int
            │   ╰── Extern
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <14> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_declarators_equivalent_declarators() {
    let src = r#"
        long int(arr)[4] = {1, 2, 3, 4};
        int long arr[4ul];
        int (*ptr_to_arr)[3][6l];
        int((*(ptr_to_arr))[3l])[6u] = 0;
        int *array_of_pointers[3] = {0, 0, 0};
        int test_arr(void) {
            for (int i = 0; i < 4; i = i + 1) {
                if (arr[i] != i + 1) {
                    return 1;
                }
            }
            return 0;
        }
        int test_ptr_to_arr(void) {
            if (ptr_to_arr) {
                return 2;
            }
            static int nested_arr[3][6];
            ptr_to_arr = &nested_arr;
            ptr_to_arr[0][2][4] = 100;
            if (nested_arr[2][4] != 100) {
                return 3;
            }
            return 0;
        }
        int test_array_of_pointers(int *ptr) {
            extern int *((array_of_pointers)[3]);
            for (int i = 0; i < 3; i = i + 1) {
                if (array_of_pointers[i])
                    return 4;
                array_of_pointers[i] = ptr;
            }
            array_of_pointers[2][0] = 11;
            if (*ptr != 11) {
                return 5;
            }
            for (int i = 0; i < 3; i = i + 1) {
                if (array_of_pointers[i][0] != 11) {
                    return 6;
                }
            }
            return 0;
        }
        int main(void)
        {
            int check = test_arr();
            if (check) {
                return check;
            }
            check = test_ptr_to_arr();
            if (check) {
                return check;
            }
            int x = 0;
            check = test_array_of_pointers(&x);
            if (check) {
                return check;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── arr
            │   ├── Type
            │   │   ╰── Array
            │   │       ├── 4
            │   │       ╰── Long
            │   ╰── Initializer
            │       ╰── Compound
            │           ├── <7> Constant Int [1]
            │           ├── <9> Constant Int [2]
            │           ├── <11> Constant Int [3]
            │           ╰── <13> Constant Int [4]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── arr
            │   ╰── Type
            │       ╰── Array
            │           ├── 4
            │           ╰── Long
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── ptr_to_arr
            │   ╰── Type
            │       ╰── Pointer
            │           ╰── Array
            │               ├── 3
            │               ╰── Array
            │                   ├── 6
            │                   ╰── Int
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── ptr_to_arr
            │   ├── Type
            │   │   ╰── Pointer
            │   │       ╰── Array
            │   │           ├── 3
            │   │           ╰── Array
            │   │               ├── 6
            │   │               ╰── Int
            │   ╰── Initializer
            │       ╰── <44> Constant Int [0]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── array_of_pointers
            │   ├── Type
            │   │   ╰── Array
            │   │       ├── 3
            │   │       ╰── Pointer
            │   │           ╰── Int
            │   ╰── Initializer
            │       ╰── Compound
            │           ├── <53> Constant Int [0]
            │           ├── <55> Constant Int [0]
            │           ╰── <57> Constant Int [0]
            ├── Function [test_arr]
            │   ╰── Body
            │       ├── For
            │       │   ├── Init
            │       │   │   ╰── VarDeclaration
            │       │   │       ├── Name
            │       │   │       │   ╰── i
            │       │   │       ├── Type
            │       │   │       │   ╰── Int
            │       │   │       ╰── Initializer
            │       │   │           ╰── <68> Constant Int [0]
            │       │   ├── Condition
            │       │   │   ╰── <76>  [<]
            │       │   │       ├── <73> Var [i]
            │       │   │       ╰── <75> Constant Int [4]
            │       │   ├── Condition
            │       │   │   ╰── <85> Assign [=]
            │       │   │       ├── <78> Var [i]
            │       │   │       ╰── <84>  [+]
            │       │   │           ├── <81> Var [i]
            │       │   │           ╰── <83> Constant Int [1]
            │       │   ╰── Block
            │       │       ╰── If
            │       │           ├── Condition
            │       │           │   ╰── <97>  [!=]
            │       │           │       ├── <90> Subscript
            │       │           │       │   ├── <87> Var [arr]
            │       │           │       │   ╰── <89> Var [i]
            │       │           │       ╰── <96>  [+]
            │       │           │           ├── <93> Var [i]
            │       │           │           ╰── <95> Constant Int [1]
            │       │           ╰── Then
            │       │               ╰── Block
            │       │                   ╰── Return
            │       │                       ╰── <98> Constant Int [1]
            │       ╰── Return
            │           ╰── <106> Constant Int [0]
            ├── Function [test_ptr_to_arr]
            │   ╰── Body
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <115> Var [ptr_to_arr]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <116> Constant Int [2]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── nested_arr
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 3
            │       │   │       ╰── Array
            │       │   │           ├── 6
            │       │   │           ╰── Int
            │       │   ╰── Static
            │       ├── <136> Assign [=]
            │       │   ├── <131> Var [ptr_to_arr]
            │       │   ╰── <135> AddressOf
            │       │       ╰── <134> Var [nested_arr]
            │       ├── <148> Assign [=]
            │       │   ├── <145> Subscript
            │       │   │   ├── <143> Subscript
            │       │   │   │   ├── <141> Subscript
            │       │   │   │   │   ├── <139> Var [ptr_to_arr]
            │       │   │   │   │   ╰── <140> Constant Int [0]
            │       │   │   │   ╰── <142> Constant Int [2]
            │       │   │   ╰── <144> Constant Int [4]
            │       │   ╰── <147> Constant Int [100]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <158>  [!=]
            │       │   │       ├── <155> Subscript
            │       │   │       │   ├── <153> Subscript
            │       │   │       │   │   ├── <151> Var [nested_arr]
            │       │   │       │   │   ╰── <152> Constant Int [2]
            │       │   │       │   ╰── <154> Constant Int [4]
            │       │   │       ╰── <157> Constant Int [100]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <159> Constant Int [3]
            │       ╰── Return
            │           ╰── <164> Constant Int [0]
            ├── Function [test_array_of_pointers]
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
            │       │   │   ╰── array_of_pointers
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 3
            │       │   │       ╰── Pointer
            │       │   │           ╰── Int
            │       │   ╰── Extern
            │       ├── For
            │       │   ├── Init
            │       │   │   ╰── VarDeclaration
            │       │   │       ├── Name
            │       │   │       │   ╰── i
            │       │   │       ├── Type
            │       │   │       │   ╰── Int
            │       │   │       ╰── Initializer
            │       │   │           ╰── <189> Constant Int [0]
            │       │   ├── Condition
            │       │   │   ╰── <197>  [<]
            │       │   │       ├── <194> Var [i]
            │       │   │       ╰── <196> Constant Int [3]
            │       │   ├── Condition
            │       │   │   ╰── <206> Assign [=]
            │       │   │       ├── <199> Var [i]
            │       │   │       ╰── <205>  [+]
            │       │   │           ├── <202> Var [i]
            │       │   │           ╰── <204> Constant Int [1]
            │       │   ╰── Block
            │       │       ├── If
            │       │       │   ├── Condition
            │       │       │   │   ╰── <211> Subscript
            │       │       │   │       ├── <208> Var [array_of_pointers]
            │       │       │   │       ╰── <210> Var [i]
            │       │       │   ╰── Then
            │       │       │       ╰── Return
            │       │       │           ╰── <212> Constant Int [4]
            │       │       ╰── <223> Assign [=]
            │       │           ├── <219> Subscript
            │       │           │   ├── <216> Var [array_of_pointers]
            │       │           │   ╰── <218> Var [i]
            │       │           ╰── <222> Var [ptr]
            │       ├── <236> Assign [=]
            │       │   ├── <233> Subscript
            │       │   │   ├── <231> Subscript
            │       │   │   │   ├── <229> Var [array_of_pointers]
            │       │   │   │   ╰── <230> Constant Int [2]
            │       │   │   ╰── <232> Constant Int [0]
            │       │   ╰── <235> Constant Int [11]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <243>  [!=]
            │       │   │       ├── <240> Dereference
            │       │   │       │   ╰── <239> Var [ptr]
            │       │   │       ╰── <242> Constant Int [11]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <244> Constant Int [5]
            │       ├── For
            │       │   ├── Init
            │       │   │   ╰── VarDeclaration
            │       │   │       ├── Name
            │       │   │       │   ╰── i
            │       │   │       ├── Type
            │       │   │       │   ╰── Int
            │       │   │       ╰── Initializer
            │       │   │           ╰── <252> Constant Int [0]
            │       │   ├── Condition
            │       │   │   ╰── <260>  [<]
            │       │   │       ├── <257> Var [i]
            │       │   │       ╰── <259> Constant Int [3]
            │       │   ├── Condition
            │       │   │   ╰── <269> Assign [=]
            │       │   │       ├── <262> Var [i]
            │       │   │       ╰── <268>  [+]
            │       │   │           ├── <265> Var [i]
            │       │   │           ╰── <267> Constant Int [1]
            │       │   ╰── Block
            │       │       ╰── If
            │       │           ├── Condition
            │       │           │   ╰── <279>  [!=]
            │       │           │       ├── <276> Subscript
            │       │           │       │   ├── <274> Subscript
            │       │           │       │   │   ├── <271> Var [array_of_pointers]
            │       │           │       │   │   ╰── <273> Var [i]
            │       │           │       │   ╰── <275> Constant Int [0]
            │       │           │       ╰── <278> Constant Int [11]
            │       │           ╰── Then
            │       │               ╰── Block
            │       │                   ╰── Return
            │       │                       ╰── <280> Constant Int [6]
            │       ╰── Return
            │           ╰── <288> Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── check
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <300> FunctionCall [test_arr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <304> Var [check]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <306> Var [check]
                    ├── <316> Assign [=]
                    │   ├── <312> Var [check]
                    │   ╰── <315> FunctionCall [test_ptr_to_arr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <319> Var [check]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <321> Var [check]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <329> Constant Int [0]
                    ├── <340> Assign [=]
                    │   ├── <333> Var [check]
                    │   ╰── <339> FunctionCall [test_array_of_pointers]
                    │       ╰── <338> AddressOf
                    │           ╰── <337> Var [x]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <343> Var [check]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <345> Var [check]
                    ╰── Return
                        ╰── <350> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_declarators_for_loop_array() {
    let src = r#"
        
        int main(void) {
            int counter = 0;
            for (int i[3] = {1, 2, 3}; counter < 3; counter = counter + 1){
                if (i[counter] != counter + 1) {
                    return 1;
                }
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
                    │   │   ╰── counter
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <8> Constant Int [0]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Array
                    │   │       │       ├── 3
                    │   │       │       ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── Compound
                    │   │               ├── <16> Constant Int [1]
                    │   │               ├── <18> Constant Int [2]
                    │   │               ╰── <20> Constant Int [3]
                    │   ├── Condition
                    │   │   ╰── <29>  [<]
                    │   │       ├── <26> Var [counter]
                    │   │       ╰── <28> Constant Int [3]
                    │   ├── Condition
                    │   │   ╰── <38> Assign [=]
                    │   │       ├── <31> Var [counter]
                    │   │       ╰── <37>  [+]
                    │   │           ├── <34> Var [counter]
                    │   │           ╰── <36> Constant Int [1]
                    │   ╰── Block
                    │       ╰── If
                    │           ├── Condition
                    │           │   ╰── <50>  [!=]
                    │           │       ├── <43> Subscript
                    │           │       │   ├── <40> Var [i]
                    │           │       │   ╰── <42> Var [counter]
                    │           │       ╰── <49>  [+]
                    │           │           ├── <46> Var [counter]
                    │           │           ╰── <48> Constant Int [1]
                    │           ╰── Then
                    │               ╰── Block
                    │                   ╰── Return
                    │                       ╰── <51> Constant Int [1]
                    ╰── Return
                        ╰── <59> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_declarators_return_nested_array() {
    let src = r#"
        int arr[3] = {1, 1, 1};
        int (*foo(int x, int y))[3] {
            arr[1] = x;
            arr[2] = y;
            return &arr;
        }
        int main(void) {
            int (*arr)[3] = foo(2, 3);
            if (arr[0][0] != 1) {
                return 1;
            }
            if (arr[0][1] != 2) {
                return 2;
            }
            if (arr[0][2] != 3) {
                return 3;
            }
            return 0;
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
            │   │       ╰── Int
            │   ╰── Initializer
            │       ╰── Compound
            │           ├── <6> Constant Int [1]
            │           ├── <8> Constant Int [1]
            │           ╰── <10> Constant Int [1]
            ├── Function [foo]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── x
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── y
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ├── <35> Assign [=]
            │       │   ├── <31> Subscript
            │       │   │   ├── <29> Var [arr]
            │       │   │   ╰── <30> Constant Int [1]
            │       │   ╰── <34> Var [x]
            │       ├── <44> Assign [=]
            │       │   ├── <40> Subscript
            │       │   │   ├── <38> Var [arr]
            │       │   │   ╰── <39> Constant Int [2]
            │       │   ╰── <43> Var [y]
            │       ╰── Return
            │           ╰── <48> AddressOf
            │               ╰── <47> Var [arr]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── arr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Array
                    │   │           ├── 3
                    │   │           ╰── Int
                    │   ╰── Initializer
                    │       ╰── <66> FunctionCall [foo]
                    │           ├── <64> Constant Int [2]
                    │           ╰── <65> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <77>  [!=]
                    │   │       ├── <74> Subscript
                    │   │       │   ├── <72> Subscript
                    │   │       │   │   ├── <70> Var [arr]
                    │   │       │   │   ╰── <71> Constant Int [0]
                    │   │       │   ╰── <73> Constant Int [0]
                    │   │       ╰── <76> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <78> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <91>  [!=]
                    │   │       ├── <88> Subscript
                    │   │       │   ├── <86> Subscript
                    │   │       │   │   ├── <84> Var [arr]
                    │   │       │   │   ╰── <85> Constant Int [0]
                    │   │       │   ╰── <87> Constant Int [1]
                    │   │       ╰── <90> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <92> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <105>  [!=]
                    │   │       ├── <102> Subscript
                    │   │       │   ├── <100> Subscript
                    │   │       │   │   ├── <98> Var [arr]
                    │   │       │   │   ╰── <99> Constant Int [0]
                    │   │       │   ╰── <101> Constant Int [2]
                    │   │       ╰── <104> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <106> Constant Int [3]
                    ╰── Return
                        ╰── <111> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_bitwise_subscript() {
    let src = r#"
        
        int main(void) {
            int arr[6] = {-10, 10, -11, 11, -12, 12};
            if ((arr[0] & arr[5]) != 4) {
                return 1;
            }
            if ((arr[1] | arr[4]) != -2) {
                return 2;
            }
            if ((arr[2] ^ arr[3]) != -2) {
                return 3;
            }
            arr[0] = 2041302511;
            if ((arr[0] >> arr[1]) != 1993459) {
                return 4;
            }
            if ((arr[5] << 3 ) != 96) {
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
                    │   │   ╰── arr
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 6
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <12> Unary [-]
                    │           │   ╰── <11> Constant Int [10]
                    │           ├── <14> Constant Int [10]
                    │           ├── <18> Unary [-]
                    │           │   ╰── <17> Constant Int [11]
                    │           ├── <20> Constant Int [11]
                    │           ├── <24> Unary [-]
                    │           │   ╰── <23> Constant Int [12]
                    │           ╰── <26> Constant Int [12]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <43>  [!=]
                    │   │       ├── <40>  [&]
                    │   │       │   ├── <33> Subscript
                    │   │       │   │   ├── <31> Var [arr]
                    │   │       │   │   ╰── <32> Constant Int [0]
                    │   │       │   ╰── <38> Subscript
                    │   │       │       ├── <36> Var [arr]
                    │   │       │       ╰── <37> Constant Int [5]
                    │   │       ╰── <42> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <44> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <64>  [!=]
                    │   │       ├── <59>  [|]
                    │   │       │   ├── <52> Subscript
                    │   │       │   │   ├── <50> Var [arr]
                    │   │       │   │   ╰── <51> Constant Int [1]
                    │   │       │   ╰── <57> Subscript
                    │   │       │       ├── <55> Var [arr]
                    │   │       │       ╰── <56> Constant Int [4]
                    │   │       ╰── <63> Unary [-]
                    │   │           ╰── <62> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <65> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <85>  [!=]
                    │   │       ├── <80>  [^]
                    │   │       │   ├── <73> Subscript
                    │   │       │   │   ├── <71> Var [arr]
                    │   │       │   │   ╰── <72> Constant Int [2]
                    │   │       │   ╰── <78> Subscript
                    │   │       │       ├── <76> Var [arr]
                    │   │       │       ╰── <77> Constant Int [3]
                    │   │       ╰── <84> Unary [-]
                    │   │           ╰── <83> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <86> Constant Int [3]
                    ├── <97> Assign [=]
                    │   ├── <94> Subscript
                    │   │   ├── <92> Var [arr]
                    │   │   ╰── <93> Constant Int [0]
                    │   ╰── <96> Constant Int [2041302511]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <112>  [!=]
                    │   │       ├── <109>  [>>]
                    │   │       │   ├── <102> Subscript
                    │   │       │   │   ├── <100> Var [arr]
                    │   │       │   │   ╰── <101> Constant Int [0]
                    │   │       │   ╰── <107> Subscript
                    │   │       │       ├── <105> Var [arr]
                    │   │       │       ╰── <106> Constant Int [1]
                    │   │       ╰── <111> Constant Int [1993459]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <113> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <128>  [!=]
                    │   │       ├── <125>  [<<]
                    │   │       │   ├── <121> Subscript
                    │   │       │   │   ├── <119> Var [arr]
                    │   │       │   │   ╰── <120> Constant Int [5]
                    │   │       │   ╰── <123> Constant Int [3]
                    │   │       ╰── <127> Constant Int [96]
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
fn test_valid_extra_credit_compound_assign_and_increment() {
    let src = r#"
        
        int main(void) {
            int arr[4] = {-1, -2, -3, -4};
            int *ptr = arr;
            int idx = 2;
            if ((ptr++[idx++] *= 3) != -9) {
                return 1;
            }
            if (*ptr != -2) {
                return 2;
            }
            if (idx != 3) {
                return 3;
            }
            idx--;
            if ((--ptr)[3] += 4) {
                return 4;
            }
            if (arr[0] != -1 || arr[1] != -2 || arr[2] != -9 || arr[3] != 0) {
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
                    │   │   ╰── arr
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 4
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <12> Unary [-]
                    │           │   ╰── <11> Constant Int [1]
                    │           ├── <16> Unary [-]
                    │           │   ╰── <15> Constant Int [2]
                    │           ├── <20> Unary [-]
                    │           │   ╰── <19> Constant Int [3]
                    │           ╰── <24> Unary [-]
                    │               ╰── <23> Constant Int [4]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <33> Var [arr]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── idx
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <39> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <59>  [!=]
                    │   │       ├── <54> Assign [*=]
                    │   │       │   ├── <50> Subscript
                    │   │       │   │   ├── <45> Postfix [++]
                    │   │       │   │   │   ╰── <43> Var [ptr]
                    │   │       │   │   ╰── <49> Postfix [++]
                    │   │       │   │       ╰── <47> Var [idx]
                    │   │       │   ╰── <52> Constant Int [3]
                    │   │       ╰── <58> Unary [-]
                    │   │           ╰── <57> Constant Int [9]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <60> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <72>  [!=]
                    │   │       ├── <67> Dereference
                    │   │       │   ╰── <66> Var [ptr]
                    │   │       ╰── <71> Unary [-]
                    │   │           ╰── <70> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <73> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <82>  [!=]
                    │   │       ├── <79> Var [idx]
                    │   │       ╰── <81> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <83> Constant Int [3]
                    ├── <91> Postfix [--]
                    │   ╰── <89> Var [idx]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <102> Assign [+=]
                    │   │       ├── <99> Subscript
                    │   │       │   ├── <97> Unary [--]
                    │   │       │   │   ╰── <95> Var [ptr]
                    │   │       │   ╰── <98> Constant Int [3]
                    │   │       ╰── <101> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <103> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <147>  [||]
                    │   │       ├── <138>  [||]
                    │   │       │   ├── <127>  [||]
                    │   │       │   │   ├── <116>  [!=]
                    │   │       │   │   │   ├── <111> Subscript
                    │   │       │   │   │   │   ├── <109> Var [arr]
                    │   │       │   │   │   │   ╰── <110> Constant Int [0]
                    │   │       │   │   │   ╰── <115> Unary [-]
                    │   │       │   │   │       ╰── <114> Constant Int [1]
                    │   │       │   │   ╰── <126>  [!=]
                    │   │       │   │       ├── <121> Subscript
                    │   │       │   │       │   ├── <119> Var [arr]
                    │   │       │   │       │   ╰── <120> Constant Int [1]
                    │   │       │   │       ╰── <125> Unary [-]
                    │   │       │   │           ╰── <124> Constant Int [2]
                    │   │       │   ╰── <137>  [!=]
                    │   │       │       ├── <132> Subscript
                    │   │       │       │   ├── <130> Var [arr]
                    │   │       │       │   ╰── <131> Constant Int [2]
                    │   │       │       ╰── <136> Unary [-]
                    │   │       │           ╰── <135> Constant Int [9]
                    │   │       ╰── <146>  [!=]
                    │   │           ├── <143> Subscript
                    │   │           │   ├── <141> Var [arr]
                    │   │           │   ╰── <142> Constant Int [3]
                    │   │           ╰── <145> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <148> Constant Int [5]
                    ╰── Return
                        ╰── <153> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_compound_assign_array_of_pointers() {
    let src = r#"
        
        int main(void) {
            static int (*array_of_pointers[3])[4] = {0, 0, 0};
            int array1[4] = {100, 101, 102, 103};
            int nested_array[2][4] = {
                {200, 201, 202, 203},
                {300, 301, 302, 303}
            };
            array_of_pointers[0] = &array1;
            array_of_pointers[1] = &nested_array[0];
            array_of_pointers[2] = &nested_array[1];
            array_of_pointers[0] += 1;
            if (array_of_pointers[0][-1][3] != 103) {
                return 1;
            }
            array_of_pointers[1] += 1;
            array_of_pointers[2] -= 1;
            if (array_of_pointers[1][0][3] != 303) {
                return 2;
            }
            if (array_of_pointers[2][0][3] != 203) {
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
                    │   │   ╰── array_of_pointers
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 3
                    │   │       ╰── Pointer
                    │   │           ╰── Array
                    │   │               ├── 4
                    │   │               ╰── Int
                    │   ├── Initializer
                    │   │   ╰── Compound
                    │   │       ├── <15> Constant Int [0]
                    │   │       ├── <17> Constant Int [0]
                    │   │       ╰── <19> Constant Int [0]
                    │   ╰── Static
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── array1
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 4
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <28> Constant Int [100]
                    │           ├── <30> Constant Int [101]
                    │           ├── <32> Constant Int [102]
                    │           ╰── <34> Constant Int [103]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── nested_array
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 2
                    │   │       ╰── Array
                    │   │           ├── 4
                    │   │           ╰── Int
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── Compound
                    │           │   ├── <45> Constant Int [200]
                    │           │   ├── <47> Constant Int [201]
                    │           │   ├── <49> Constant Int [202]
                    │           │   ╰── <51> Constant Int [203]
                    │           ╰── Compound
                    │               ├── <54> Constant Int [300]
                    │               ├── <56> Constant Int [301]
                    │               ├── <58> Constant Int [302]
                    │               ╰── <60> Constant Int [303]
                    ├── <73> Assign [=]
                    │   ├── <68> Subscript
                    │   │   ├── <66> Var [array_of_pointers]
                    │   │   ╰── <67> Constant Int [0]
                    │   ╰── <72> AddressOf
                    │       ╰── <71> Var [array1]
                    ├── <85> Assign [=]
                    │   ├── <78> Subscript
                    │   │   ├── <76> Var [array_of_pointers]
                    │   │   ╰── <77> Constant Int [1]
                    │   ╰── <84> AddressOf
                    │       ╰── <83> Subscript
                    │           ├── <81> Var [nested_array]
                    │           ╰── <82> Constant Int [0]
                    ├── <97> Assign [=]
                    │   ├── <90> Subscript
                    │   │   ├── <88> Var [array_of_pointers]
                    │   │   ╰── <89> Constant Int [2]
                    │   ╰── <96> AddressOf
                    │       ╰── <95> Subscript
                    │           ├── <93> Var [nested_array]
                    │           ╰── <94> Constant Int [1]
                    ├── <105> Assign [+=]
                    │   ├── <102> Subscript
                    │   │   ├── <100> Var [array_of_pointers]
                    │   │   ╰── <101> Constant Int [0]
                    │   ╰── <104> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <119>  [!=]
                    │   │       ├── <116> Subscript
                    │   │       │   ├── <114> Subscript
                    │   │       │   │   ├── <110> Subscript
                    │   │       │   │   │   ├── <108> Var [array_of_pointers]
                    │   │       │   │   │   ╰── <109> Constant Int [0]
                    │   │       │   │   ╰── <113> Unary [-]
                    │   │       │   │       ╰── <112> Constant Int [1]
                    │   │       │   ╰── <115> Constant Int [3]
                    │   │       ╰── <118> Constant Int [103]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <120> Constant Int [1]
                    ├── <131> Assign [+=]
                    │   ├── <128> Subscript
                    │   │   ├── <126> Var [array_of_pointers]
                    │   │   ╰── <127> Constant Int [1]
                    │   ╰── <130> Constant Int [1]
                    ├── <139> Assign [-=]
                    │   ├── <136> Subscript
                    │   │   ├── <134> Var [array_of_pointers]
                    │   │   ╰── <135> Constant Int [2]
                    │   ╰── <138> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <151>  [!=]
                    │   │       ├── <148> Subscript
                    │   │       │   ├── <146> Subscript
                    │   │       │   │   ├── <144> Subscript
                    │   │       │   │   │   ├── <142> Var [array_of_pointers]
                    │   │       │   │   │   ╰── <143> Constant Int [1]
                    │   │       │   │   ╰── <145> Constant Int [0]
                    │   │       │   ╰── <147> Constant Int [3]
                    │   │       ╰── <150> Constant Int [303]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <152> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <167>  [!=]
                    │   │       ├── <164> Subscript
                    │   │       │   ├── <162> Subscript
                    │   │       │   │   ├── <160> Subscript
                    │   │       │   │   │   ├── <158> Var [array_of_pointers]
                    │   │       │   │   │   ╰── <159> Constant Int [2]
                    │   │       │   │   ╰── <161> Constant Int [0]
                    │   │       │   ╰── <163> Constant Int [3]
                    │   │       ╰── <166> Constant Int [203]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <168> Constant Int [3]
                    ╰── Return
                        ╰── <173> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_compound_assign_to_nested_subscript() {
    let src = r#"
        long long_nested_arr[2][3] = {{1, 2, 3}, {4, 5, 6}};
        double dbl_nested_arr[3][2] = {{100.0, 101.0}, {102.0, 103.0}, {104.0, 105.0}};
        unsigned unsigned_index = 10;
        int main(void) {
            if ((long_nested_arr[1][unsigned_index - 8] *= -1) != -6) {
                return 1;
            }
            if (long_nested_arr[1][2] != -6) {
                return 2;
            }
            for (int i = 0; i < 2; i += 1) {
                for (int j = 0; j < 3; j += 1) {
                    if (i == 1 && j == 2) {
                        break;
                    }
                    long expected = i * 3 + j + 1;
                    if (long_nested_arr[i][j] != expected) {
                        return 3;
                    }
                }
            }
            if ((dbl_nested_arr[1][1] += 100.0) != 203.0) {
                return 4;
            }
            for (int i = 0; i < 3; i += 1) {
                for (int j = 0; j < 2; j += 1) {
                    if (i == 1 && j == 1) {
                        continue;
                    }
                    int expected = 100 + i * 2 + j;
                    if (dbl_nested_arr[i][j] != expected) {
                        return 5;
                    }
                }
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── long_nested_arr
            │   ├── Type
            │   │   ╰── Array
            │   │       ├── 2
            │   │       ╰── Array
            │   │           ├── 3
            │   │           ╰── Long
            │   ╰── Initializer
            │       ╰── Compound
            │           ├── Compound
            │           │   ├── <8> Constant Int [1]
            │           │   ├── <10> Constant Int [2]
            │           │   ╰── <12> Constant Int [3]
            │           ╰── Compound
            │               ├── <15> Constant Int [4]
            │               ├── <17> Constant Int [5]
            │               ╰── <19> Constant Int [6]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── dbl_nested_arr
            │   ├── Type
            │   │   ╰── Array
            │   │       ├── 3
            │   │       ╰── Array
            │   │           ├── 2
            │   │           ╰── Double
            │   ╰── Initializer
            │       ╰── Compound
            │           ├── Compound
            │           │   ├── <31> Constant Double [+1e2]
            │           │   ╰── <33> Constant Double [+1.01e2]
            │           ├── Compound
            │           │   ├── <36> Constant Double [+1.02e2]
            │           │   ╰── <38> Constant Double [+1.03e2]
            │           ╰── Compound
            │               ├── <41> Constant Double [+1.04e2]
            │               ╰── <43> Constant Double [+1.05e2]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── unsigned_index
            │   ├── Type
            │   │   ╰── Unsigned Int
            │   ╰── Initializer
            │       ╰── <51> Constant Int [10]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <78>  [!=]
                    │   │       ├── <73> Assign [*=]
                    │   │       │   ├── <67> Subscript
                    │   │       │   │   ├── <61> Subscript
                    │   │       │   │   │   ├── <59> Var [long_nested_arr]
                    │   │       │   │   │   ╰── <60> Constant Int [1]
                    │   │       │   │   ╰── <66>  [-]
                    │   │       │   │       ├── <63> Var [unsigned_index]
                    │   │       │   │       ╰── <65> Constant Int [8]
                    │   │       │   ╰── <71> Unary [-]
                    │   │       │       ╰── <70> Constant Int [1]
                    │   │       ╰── <77> Unary [-]
                    │   │           ╰── <76> Constant Int [6]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <79> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <94>  [!=]
                    │   │       ├── <89> Subscript
                    │   │       │   ├── <87> Subscript
                    │   │       │   │   ├── <85> Var [long_nested_arr]
                    │   │       │   │   ╰── <86> Constant Int [1]
                    │   │       │   ╰── <88> Constant Int [2]
                    │   │       ╰── <93> Unary [-]
                    │   │           ╰── <92> Constant Int [6]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <95> Constant Int [2]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <103> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <111>  [<]
                    │   │       ├── <108> Var [i]
                    │   │       ╰── <110> Constant Int [2]
                    │   ├── Condition
                    │   │   ╰── <116> Assign [+=]
                    │   │       ├── <113> Var [i]
                    │   │       ╰── <115> Constant Int [1]
                    │   ╰── Block
                    │       ╰── For
                    │           ├── Init
                    │           │   ╰── VarDeclaration
                    │           │       ├── Name
                    │           │       │   ╰── j
                    │           │       ├── Type
                    │           │       │   ╰── Int
                    │           │       ╰── Initializer
                    │           │           ╰── <120> Constant Int [0]
                    │           ├── Condition
                    │           │   ╰── <128>  [<]
                    │           │       ├── <125> Var [j]
                    │           │       ╰── <127> Constant Int [3]
                    │           ├── Condition
                    │           │   ╰── <133> Assign [+=]
                    │           │       ├── <130> Var [j]
                    │           │       ╰── <132> Constant Int [1]
                    │           ╰── Block
                    │               ├── If
                    │               │   ├── Condition
                    │               │   │   ╰── <145>  [&&]
                    │               │   │       ├── <138>  [==]
                    │               │   │       │   ├── <135> Var [i]
                    │               │   │       │   ╰── <137> Constant Int [1]
                    │               │   │       ╰── <144>  [==]
                    │               │   │           ├── <141> Var [j]
                    │               │   │           ╰── <143> Constant Int [2]
                    │               │   ╰── Then
                    │               │       ╰── Block
                    │               │           ╰── Break
                    │               ├── VarDeclaration
                    │               │   ├── Name
                    │               │   │   ╰── expected
                    │               │   ├── Type
                    │               │   │   ╰── Long
                    │               │   ╰── Initializer
                    │               │       ╰── <164>  [+]
                    │               │           ├── <161>  [+]
                    │               │           │   ├── <157>  [*]
                    │               │           │   │   ├── <154> Var [i]
                    │               │           │   │   ╰── <156> Constant Int [3]
                    │               │           │   ╰── <160> Var [j]
                    │               │           ╰── <163> Constant Int [1]
                    │               ╰── If
                    │                   ├── Condition
                    │                   │   ╰── <178>  [!=]
                    │                   │       ├── <174> Subscript
                    │                   │       │   ├── <171> Subscript
                    │                   │       │   │   ├── <168> Var [long_nested_arr]
                    │                   │       │   │   ╰── <170> Var [i]
                    │                   │       │   ╰── <173> Var [j]
                    │                   │       ╰── <177> Var [expected]
                    │                   ╰── Then
                    │                       ╰── Block
                    │                           ╰── Return
                    │                               ╰── <179> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <202>  [!=]
                    │   │       ├── <199> Assign [+=]
                    │   │       │   ├── <195> Subscript
                    │   │       │   │   ├── <193> Subscript
                    │   │       │   │   │   ├── <191> Var [dbl_nested_arr]
                    │   │       │   │   │   ╰── <192> Constant Int [1]
                    │   │       │   │   ╰── <194> Constant Int [1]
                    │   │       │   ╰── <197> Constant Double [+1e2]
                    │   │       ╰── <201> Constant Double [+2.03e2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <203> Constant Int [4]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <211> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <219>  [<]
                    │   │       ├── <216> Var [i]
                    │   │       ╰── <218> Constant Int [3]
                    │   ├── Condition
                    │   │   ╰── <224> Assign [+=]
                    │   │       ├── <221> Var [i]
                    │   │       ╰── <223> Constant Int [1]
                    │   ╰── Block
                    │       ╰── For
                    │           ├── Init
                    │           │   ╰── VarDeclaration
                    │           │       ├── Name
                    │           │       │   ╰── j
                    │           │       ├── Type
                    │           │       │   ╰── Int
                    │           │       ╰── Initializer
                    │           │           ╰── <228> Constant Int [0]
                    │           ├── Condition
                    │           │   ╰── <236>  [<]
                    │           │       ├── <233> Var [j]
                    │           │       ╰── <235> Constant Int [2]
                    │           ├── Condition
                    │           │   ╰── <241> Assign [+=]
                    │           │       ├── <238> Var [j]
                    │           │       ╰── <240> Constant Int [1]
                    │           ╰── Block
                    │               ├── If
                    │               │   ├── Condition
                    │               │   │   ╰── <253>  [&&]
                    │               │   │       ├── <246>  [==]
                    │               │   │       │   ├── <243> Var [i]
                    │               │   │       │   ╰── <245> Constant Int [1]
                    │               │   │       ╰── <252>  [==]
                    │               │   │           ├── <249> Var [j]
                    │               │   │           ╰── <251> Constant Int [1]
                    │               │   ╰── Then
                    │               │       ╰── Block
                    │               │           ╰── Continue
                    │               ├── VarDeclaration
                    │               │   ├── Name
                    │               │   │   ╰── expected
                    │               │   ├── Type
                    │               │   │   ╰── Int
                    │               │   ╰── Initializer
                    │               │       ╰── <272>  [+]
                    │               │           ├── <268>  [+]
                    │               │           │   ├── <261> Constant Int [100]
                    │               │           │   ╰── <267>  [*]
                    │               │           │       ├── <264> Var [i]
                    │               │           │       ╰── <266> Constant Int [2]
                    │               │           ╰── <271> Var [j]
                    │               ╰── If
                    │                   ├── Condition
                    │                   │   ╰── <286>  [!=]
                    │                   │       ├── <282> Subscript
                    │                   │       │   ├── <279> Subscript
                    │                   │       │   │   ├── <276> Var [dbl_nested_arr]
                    │                   │       │   │   ╰── <278> Var [i]
                    │                   │       │   ╰── <281> Var [j]
                    │                   │       ╰── <285> Var [expected]
                    │                   ╰── Then
                    │                       ╰── Block
                    │                           ╰── Return
                    │                               ╰── <287> Constant Int [5]
                    ╰── Return
                        ╰── <298> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_compound_assign_to_subscripted_val() {
    let src = r#"
        unsigned unsigned_arr[4] = {4294967295U, 4294967294U, 4294967293U, 4294967292U};
        int idx = 2;
        long long_idx = 1;
        int main(void) {
            long_idx = -long_idx;
            unsigned_arr[1] += 2;
            if (unsigned_arr[1]) {
                return 1;
            }
            unsigned_arr[idx] -= 10.0;
            if (unsigned_arr[idx] != 4294967283U) {
                return 2;
            }
            unsigned *unsigned_ptr = unsigned_arr + 4;
            unsigned_ptr[long_idx] /= 10;
            if (unsigned_arr[3] != 429496729U) {
                return 3;
            }
            unsigned_ptr[long_idx *= 2] *= unsigned_arr[0];
            if (unsigned_arr[2] != 13) {
                return 4;
            }
            if ((unsigned_arr[idx + long_idx] %= 10) != 5) {
                return 5;
            }
            if (unsigned_arr[0] != 5u) {
                return 6;
            }
            if (unsigned_arr[1]) {
                return 7;
            }
            if (unsigned_arr[2] != 13) {
                return 8;
            }
            if (unsigned_arr[3] != 429496729U) {
                return 9;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── unsigned_arr
            │   ├── Type
            │   │   ╰── Array
            │   │       ├── 4
            │   │       ╰── Unsigned Int
            │   ╰── Initializer
            │       ╰── Compound
            │           ├── <6> Constant UInt [4294967295]
            │           ├── <8> Constant UInt [4294967294]
            │           ├── <10> Constant UInt [4294967293]
            │           ╰── <12> Constant UInt [4294967292]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── idx
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Initializer
            │       ╰── <19> Constant Int [2]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── long_idx
            │   ├── Type
            │   │   ╰── Long
            │   ╰── Initializer
            │       ╰── <25> Constant Int [1]
            ╰── Function [main]
                ╰── Body
                    ├── <39> Assign [=]
                    │   ├── <33> Var [long_idx]
                    │   ╰── <38> Unary [-]
                    │       ╰── <37> Var [long_idx]
                    ├── <47> Assign [+=]
                    │   ├── <44> Subscript
                    │   │   ├── <42> Var [unsigned_arr]
                    │   │   ╰── <43> Constant Int [1]
                    │   ╰── <46> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <52> Subscript
                    │   │       ├── <50> Var [unsigned_arr]
                    │   │       ╰── <51> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <53> Constant Int [1]
                    ├── <65> Assign [-=]
                    │   ├── <62> Subscript
                    │   │   ├── <59> Var [unsigned_arr]
                    │   │   ╰── <61> Var [idx]
                    │   ╰── <64> Constant Double [+1e1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <74>  [!=]
                    │   │       ├── <71> Subscript
                    │   │       │   ├── <68> Var [unsigned_arr]
                    │   │       │   ╰── <70> Var [idx]
                    │   │       ╰── <73> Constant UInt [4294967283]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <75> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── unsigned_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Unsigned Int
                    │   ╰── Initializer
                    │       ╰── <88>  [+]
                    │           ├── <85> Var [unsigned_arr]
                    │           ╰── <87> Constant Int [4]
                    ├── <98> Assign [/=]
                    │   ├── <95> Subscript
                    │   │   ├── <92> Var [unsigned_ptr]
                    │   │   ╰── <94> Var [long_idx]
                    │   ╰── <97> Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <106>  [!=]
                    │   │       ├── <103> Subscript
                    │   │       │   ├── <101> Var [unsigned_arr]
                    │   │       │   ╰── <102> Constant Int [3]
                    │   │       ╰── <105> Constant UInt [429496729]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <107> Constant Int [3]
                    ├── <125> Assign [*=]
                    │   ├── <119> Subscript
                    │   │   ├── <113> Var [unsigned_ptr]
                    │   │   ╰── <118> Assign [*=]
                    │   │       ├── <115> Var [long_idx]
                    │   │       ╰── <117> Constant Int [2]
                    │   ╰── <124> Subscript
                    │       ├── <122> Var [unsigned_arr]
                    │       ╰── <123> Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <133>  [!=]
                    │   │       ├── <130> Subscript
                    │   │       │   ├── <128> Var [unsigned_arr]
                    │   │       │   ╰── <129> Constant Int [2]
                    │   │       ╰── <132> Constant Int [13]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <134> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <154>  [!=]
                    │   │       ├── <151> Assign [&=]
                    │   │       │   ├── <147> Subscript
                    │   │       │   │   ├── <140> Var [unsigned_arr]
                    │   │       │   │   ╰── <146>  [+]
                    │   │       │   │       ├── <142> Var [idx]
                    │   │       │   │       ╰── <145> Var [long_idx]
                    │   │       │   ╰── <149> Constant Int [10]
                    │   │       ╰── <153> Constant Int [5]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <155> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <166>  [!=]
                    │   │       ├── <163> Subscript
                    │   │       │   ├── <161> Var [unsigned_arr]
                    │   │       │   ╰── <162> Constant Int [0]
                    │   │       ╰── <165> Constant UInt [5]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <167> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <175> Subscript
                    │   │       ├── <173> Var [unsigned_arr]
                    │   │       ╰── <174> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <176> Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <187>  [!=]
                    │   │       ├── <184> Subscript
                    │   │       │   ├── <182> Var [unsigned_arr]
                    │   │       │   ╰── <183> Constant Int [2]
                    │   │       ╰── <186> Constant Int [13]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <188> Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <199>  [!=]
                    │   │       ├── <196> Subscript
                    │   │       │   ├── <194> Var [unsigned_arr]
                    │   │       │   ╰── <195> Constant Int [3]
                    │   │       ╰── <198> Constant UInt [429496729]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <200> Constant Int [9]
                    ╰── Return
                        ╰── <205> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_compound_bitwise_subscript() {
    let src = r#"
        
        int main(void) {
            unsigned long arr[4] = {
                2147483648l,
                18446744069414584320ul,
                9223372036854775808ul,
                1085102592571150095l
            };
            arr[1] &= arr[3];
            if (arr[1] != 1085102592318504960 ) {
                return 1;
            }
            arr[0] |= arr[1];
            if (arr[0] != 1085102594465988608ul) {
                return 2;
            }
            arr[2] ^= arr[3];
            if (arr[2] != 10308474629425925903ul) {
                return 3;
            }
            arr[3] >>= 25;
            if (arr[3] != 32338577287l) {
                return 4;
            }
            arr[1] <<= 12;
            if (arr[1] != 17361640446303928320ul) {
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
                    │   │   ╰── arr
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 4
                    │   │       ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <10> Constant Long [2147483648]
                    │           ├── <12> Constant ULong [18446744069414584320]
                    │           ├── <14> Constant ULong [9223372036854775808]
                    │           ╰── <16> Constant Long [1085102592571150095]
                    ├── <29> Assign [&=]
                    │   ├── <23> Subscript
                    │   │   ├── <21> Var [arr]
                    │   │   ╰── <22> Constant Int [1]
                    │   ╰── <28> Subscript
                    │       ├── <26> Var [arr]
                    │       ╰── <27> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <37>  [!=]
                    │   │       ├── <34> Subscript
                    │   │       │   ├── <32> Var [arr]
                    │   │       │   ╰── <33> Constant Int [1]
                    │   │       ╰── <36> Constant Long [1085102592318504960]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <38> Constant Int [1]
                    ├── <52> Assign [|=]
                    │   ├── <46> Subscript
                    │   │   ├── <44> Var [arr]
                    │   │   ╰── <45> Constant Int [0]
                    │   ╰── <51> Subscript
                    │       ├── <49> Var [arr]
                    │       ╰── <50> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <60>  [!=]
                    │   │       ├── <57> Subscript
                    │   │       │   ├── <55> Var [arr]
                    │   │       │   ╰── <56> Constant Int [0]
                    │   │       ╰── <59> Constant ULong [1085102594465988608]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <61> Constant Int [2]
                    ├── <75> Assign [^=]
                    │   ├── <69> Subscript
                    │   │   ├── <67> Var [arr]
                    │   │   ╰── <68> Constant Int [2]
                    │   ╰── <74> Subscript
                    │       ├── <72> Var [arr]
                    │       ╰── <73> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <83>  [!=]
                    │   │       ├── <80> Subscript
                    │   │       │   ├── <78> Var [arr]
                    │   │       │   ╰── <79> Constant Int [2]
                    │   │       ╰── <82> Constant ULong [10308474629425925903]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <84> Constant Int [3]
                    ├── <95> Assign [>>=]
                    │   ├── <92> Subscript
                    │   │   ├── <90> Var [arr]
                    │   │   ╰── <91> Constant Int [3]
                    │   ╰── <94> Constant Int [25]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <103>  [!=]
                    │   │       ├── <100> Subscript
                    │   │       │   ├── <98> Var [arr]
                    │   │       │   ╰── <99> Constant Int [3]
                    │   │       ╰── <102> Constant Long [32338577287]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <104> Constant Int [4]
                    ├── <115> Assign [<<=]
                    │   ├── <112> Subscript
                    │   │   ├── <110> Var [arr]
                    │   │   ╰── <111> Constant Int [1]
                    │   ╰── <114> Constant Int [12]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <123>  [!=]
                    │   │       ├── <120> Subscript
                    │   │       │   ├── <118> Var [arr]
                    │   │       │   ╰── <119> Constant Int [1]
                    │   │       ╰── <122> Constant ULong [17361640446303928320]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <124> Constant Int [5]
                    ╰── Return
                        ╰── <129> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_compound_lval_evaluated_once() {
    let src = r#"
        int get_call_count(void) {
            static int count = 0;
            count += 1;
            return count;
        }
        int main(void) {
            int arr[4] = {10, 11, 12, 13};
            if (arr[get_call_count()] != 11) {
                return 1;
            }
            int *end_ptr = arr + 4;
            if ((end_ptr - 1)[-get_call_count()] != 11) {
                return 2;
            }
            if (get_call_count() != 3) {
                return 3;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [get_call_count]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── count
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ├── Initializer
            │       │   │   ╰── <9> Constant Int [0]
            │       │   ╰── Static
            │       ├── <16> Assign [+=]
            │       │   ├── <13> Var [count]
            │       │   ╰── <15> Constant Int [1]
            │       ╰── Return
            │           ╰── <19> Var [count]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── arr
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 4
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <32> Constant Int [10]
                    │           ├── <34> Constant Int [11]
                    │           ├── <36> Constant Int [12]
                    │           ╰── <38> Constant Int [13]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <49>  [!=]
                    │   │       ├── <46> Subscript
                    │   │       │   ├── <43> Var [arr]
                    │   │       │   ╰── <45> FunctionCall [get_call_count]
                    │   │       ╰── <48> Constant Int [11]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <50> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── end_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <63>  [+]
                    │           ├── <60> Var [arr]
                    │           ╰── <62> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <79>  [!=]
                    │   │       ├── <76> Subscript
                    │   │       │   ├── <71>  [-]
                    │   │       │   │   ├── <67> Var [end_ptr]
                    │   │       │   │   ╰── <69> Constant Int [1]
                    │   │       │   ╰── <75> Unary [-]
                    │   │       │       ╰── <74> FunctionCall [get_call_count]
                    │   │       ╰── <78> Constant Int [11]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <80> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <89>  [!=]
                    │   │       ├── <86> FunctionCall [get_call_count]
                    │   │       ╰── <88> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <90> Constant Int [3]
                    ╰── Return
                        ╰── <95> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_compound_nested_pointer_assignment() {
    let src = r#"
        static long nested_arr[3][4][5] = {{{10, 9, 8}, {1, 2}}, {{100, 99, 98}}};
        int main(void) {
            long(*outer_ptr)[4][5] = nested_arr;
            outer_ptr += 1;
            if (outer_ptr != nested_arr + 1) {
                return 1;
            }
            if (outer_ptr[0][0][0] != 100) {
                return 2;
            }
            long(*inner_ptr)[5] =
                nested_arr[0] + 4;
            inner_ptr -= 3;
            if (inner_ptr[0][1] != 2) {
                return 3;
            }
            unsigned long idx = nested_arr[0][0][0] - 9;
            if ((inner_ptr += idx) != &nested_arr[0][2]) {
                return 4;
            }
            if ((inner_ptr[-2][1] != 9)) {
                return 5;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── nested_arr
            │   ├── Type
            │   │   ╰── Array
            │   │       ├── 3
            │   │       ╰── Array
            │   │           ├── 4
            │   │           ╰── Array
            │   │               ├── 5
            │   │               ╰── Long
            │   ├── Initializer
            │   │   ╰── Compound
            │   │       ├── Compound
            │   │       │   ├── Compound
            │   │       │   │   ├── <11> Constant Int [10]
            │   │       │   │   ├── <13> Constant Int [9]
            │   │       │   │   ╰── <15> Constant Int [8]
            │   │       │   ╰── Compound
            │   │       │       ├── <18> Constant Int [1]
            │   │       │       ╰── <20> Constant Int [2]
            │   │       ╰── Compound
            │   │           ╰── Compound
            │   │               ├── <24> Constant Int [100]
            │   │               ├── <26> Constant Int [99]
            │   │               ╰── <28> Constant Int [98]
            │   ╰── Static
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── outer_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Array
                    │   │           ├── 4
                    │   │           ╰── Array
                    │   │               ├── 5
                    │   │               ╰── Long
                    │   ╰── Initializer
                    │       ╰── <48> Var [nested_arr]
                    ├── <55> Assign [+=]
                    │   ├── <52> Var [outer_ptr]
                    │   ╰── <54> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <65>  [!=]
                    │   │       ├── <58> Var [outer_ptr]
                    │   │       ╰── <64>  [+]
                    │   │           ├── <61> Var [nested_arr]
                    │   │           ╰── <63> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <66> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <81>  [!=]
                    │   │       ├── <78> Subscript
                    │   │       │   ├── <76> Subscript
                    │   │       │   │   ├── <74> Subscript
                    │   │       │   │   │   ├── <72> Var [outer_ptr]
                    │   │       │   │   │   ╰── <73> Constant Int [0]
                    │   │       │   │   ╰── <75> Constant Int [0]
                    │   │       │   ╰── <77> Constant Int [0]
                    │   │       ╰── <80> Constant Int [100]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <82> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── inner_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Array
                    │   │           ├── 5
                    │   │           ╰── Long
                    │   ╰── Initializer
                    │       ╰── <100>  [+]
                    │           ├── <97> Subscript
                    │           │   ├── <95> Var [nested_arr]
                    │           │   ╰── <96> Constant Int [0]
                    │           ╰── <99> Constant Int [4]
                    ├── <107> Assign [-=]
                    │   ├── <104> Var [inner_ptr]
                    │   ╰── <106> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <117>  [!=]
                    │   │       ├── <114> Subscript
                    │   │       │   ├── <112> Subscript
                    │   │       │   │   ├── <110> Var [inner_ptr]
                    │   │       │   │   ╰── <111> Constant Int [0]
                    │   │       │   ╰── <113> Constant Int [1]
                    │   │       ╰── <116> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <118> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── idx
                    │   ├── Type
                    │   │   ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <136>  [-]
                    │           ├── <133> Subscript
                    │           │   ├── <131> Subscript
                    │           │   │   ├── <129> Subscript
                    │           │   │   │   ├── <127> Var [nested_arr]
                    │           │   │   │   ╰── <128> Constant Int [0]
                    │           │   │   ╰── <130> Constant Int [0]
                    │           │   ╰── <132> Constant Int [0]
                    │           ╰── <135> Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <154>  [!=]
                    │   │       ├── <145> Assign [+=]
                    │   │       │   ├── <140> Var [inner_ptr]
                    │   │       │   ╰── <143> Var [idx]
                    │   │       ╰── <153> AddressOf
                    │   │           ╰── <152> Subscript
                    │   │               ├── <150> Subscript
                    │   │               │   ├── <148> Var [nested_arr]
                    │   │               │   ╰── <149> Constant Int [0]
                    │   │               ╰── <151> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <155> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <171>  [!=]
                    │   │       ├── <167> Subscript
                    │   │       │   ├── <165> Subscript
                    │   │       │   │   ├── <161> Var [inner_ptr]
                    │   │       │   │   ╰── <164> Unary [-]
                    │   │       │   │       ╰── <163> Constant Int [2]
                    │   │       │   ╰── <166> Constant Int [1]
                    │   │       ╰── <169> Constant Int [9]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <172> Constant Int [5]
                    ╰── Return
                        ╰── <177> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_compound_pointer_assignment() {
    let src = r#"
        int i = 4;
        int int_array(void) {
            int arr[6] = {1, 2, 3, 4, 5, 6};
            int *ptr = arr;
            if (*(ptr += 5) != 6) {
                return 1;
            }
            if (ptr[0] != 6) {
                 return 2;
            }
            if (ptr != arr + 5) {
                return 3;
            }
            if (*(ptr -=3) != 3) {
                return 4;
            }
            if (ptr[0] != 3) {
                return 5;
            }
            if (ptr != arr + 2) {
                return 6;
            }
            if ((ptr += i - 1) != arr + 5) {
                return 7;
            }
            if (*ptr != 6) {
                return 8;
            }
            if ((ptr -= (4294967295U + i)) != arr + 2) {
                return 9;
            }
            if (*ptr != 3) {
                return 10;
            }
            long l = 9223372036854775807l;
            if ((ptr += l - 9223372036854775806l) != arr + 3) {
                return 11;
            }
            if (*ptr != 4) {
                return 12;
            }
            return 0;
        }
        int double_array(void) {
            static double arr[6] = {1.0, 2.0, 3.0, 4.0, 5.0, 6.0};
            double *ptr = arr;
            if (*(ptr += 5) != 6) {
                return 1;
            }
            if (ptr[0] != 6) {
                 return 2;
            }
            if (ptr != arr + 5) {
                return 3;
            }
            if (*(ptr -=3) != 3) {
                return 4;
            }
            if (ptr[0] != 3) {
                return 5;
            }
            if (ptr != arr + 2) {
                return 6;
            }
            if ((ptr += i - 1) != arr + 5) {
                return 7;
            }
            if (*ptr != 6) {
                return 8;
            }
            if ((ptr -= (4294967295U + i)) != arr + 2) {
                return 9;
            }
            if (*ptr != 3) {
                return 10;
            }
            long l = 9223372036854775807l;
            if ((ptr += l - 9223372036854775806l) != arr + 3) {
                return 11;
            }
            if (*ptr != 4) {
                return 12;
            }
            return 0;
        }
        int main(void) {
            int result;
            if ((result = int_array())) {
                return result;
            }
            if ((result = double_array())) {
                return result + 12;
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
            ├── Function [int_array]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── arr
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 6
            │       │   │       ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── Compound
            │       │           ├── <16> Constant Int [1]
            │       │           ├── <18> Constant Int [2]
            │       │           ├── <20> Constant Int [3]
            │       │           ├── <22> Constant Int [4]
            │       │           ├── <24> Constant Int [5]
            │       │           ╰── <26> Constant Int [6]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ptr
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <35> Var [arr]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <47>  [!=]
            │       │   │       ├── <44> Dereference
            │       │   │       │   ╰── <43> Assign [+=]
            │       │   │       │       ├── <39> Var [ptr]
            │       │   │       │       ╰── <41> Constant Int [5]
            │       │   │       ╰── <46> Constant Int [6]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <48> Constant Int [1]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <59>  [!=]
            │       │   │       ├── <56> Subscript
            │       │   │       │   ├── <54> Var [ptr]
            │       │   │       │   ╰── <55> Constant Int [0]
            │       │   │       ╰── <58> Constant Int [6]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <60> Constant Int [2]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <73>  [!=]
            │       │   │       ├── <66> Var [ptr]
            │       │   │       ╰── <72>  [+]
            │       │   │           ├── <69> Var [arr]
            │       │   │           ╰── <71> Constant Int [5]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <74> Constant Int [3]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <88>  [!=]
            │       │   │       ├── <85> Dereference
            │       │   │       │   ╰── <84> Assign [-=]
            │       │   │       │       ├── <80> Var [ptr]
            │       │   │       │       ╰── <82> Constant Int [3]
            │       │   │       ╰── <87> Constant Int [3]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <89> Constant Int [4]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <100>  [!=]
            │       │   │       ├── <97> Subscript
            │       │   │       │   ├── <95> Var [ptr]
            │       │   │       │   ╰── <96> Constant Int [0]
            │       │   │       ╰── <99> Constant Int [3]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <101> Constant Int [5]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <114>  [!=]
            │       │   │       ├── <107> Var [ptr]
            │       │   │       ╰── <113>  [+]
            │       │   │           ├── <110> Var [arr]
            │       │   │           ╰── <112> Constant Int [2]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <115> Constant Int [6]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <136>  [!=]
            │       │   │       ├── <129> Assign [+=]
            │       │   │       │   ├── <121> Var [ptr]
            │       │   │       │   ╰── <127>  [-]
            │       │   │       │       ├── <124> Var [i]
            │       │   │       │       ╰── <126> Constant Int [1]
            │       │   │       ╰── <135>  [+]
            │       │   │           ├── <132> Var [arr]
            │       │   │           ╰── <134> Constant Int [5]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <137> Constant Int [7]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <147>  [!=]
            │       │   │       ├── <144> Dereference
            │       │   │       │   ╰── <143> Var [ptr]
            │       │   │       ╰── <146> Constant Int [6]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <148> Constant Int [8]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <170>  [!=]
            │       │   │       ├── <163> Assign [-=]
            │       │   │       │   ├── <154> Var [ptr]
            │       │   │       │   ╰── <161>  [+]
            │       │   │       │       ├── <156> Constant UInt [4294967295]
            │       │   │       │       ╰── <159> Var [i]
            │       │   │       ╰── <169>  [+]
            │       │   │           ├── <166> Var [arr]
            │       │   │           ╰── <168> Constant Int [2]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <171> Constant Int [9]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <181>  [!=]
            │       │   │       ├── <178> Dereference
            │       │   │       │   ╰── <177> Var [ptr]
            │       │   │       ╰── <180> Constant Int [3]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <182> Constant Int [10]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── l
            │       │   ├── Type
            │       │   │   ╰── Long
            │       │   ╰── Initializer
            │       │       ╰── <190> Constant Long [9223372036854775807]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <209>  [!=]
            │       │   │       ├── <202> Assign [+=]
            │       │   │       │   ├── <194> Var [ptr]
            │       │   │       │   ╰── <200>  [-]
            │       │   │       │       ├── <197> Var [l]
            │       │   │       │       ╰── <199> Constant Long [9223372036854775806]
            │       │   │       ╰── <208>  [+]
            │       │   │           ├── <205> Var [arr]
            │       │   │           ╰── <207> Constant Int [3]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <210> Constant Int [11]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <220>  [!=]
            │       │   │       ├── <217> Dereference
            │       │   │       │   ╰── <216> Var [ptr]
            │       │   │       ╰── <219> Constant Int [4]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <221> Constant Int [12]
            │       ╰── Return
            │           ╰── <226> Constant Int [0]
            ├── Function [double_array]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── arr
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 6
            │       │   │       ╰── Double
            │       │   ├── Initializer
            │       │   │   ╰── Compound
            │       │   │       ├── <240> Constant Double [+1e0]
            │       │   │       ├── <242> Constant Double [+2e0]
            │       │   │       ├── <244> Constant Double [+3e0]
            │       │   │       ├── <246> Constant Double [+4e0]
            │       │   │       ├── <248> Constant Double [+5e0]
            │       │   │       ╰── <250> Constant Double [+6e0]
            │       │   ╰── Static
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ptr
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Double
            │       │   ╰── Initializer
            │       │       ╰── <259> Var [arr]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <271>  [!=]
            │       │   │       ├── <268> Dereference
            │       │   │       │   ╰── <267> Assign [+=]
            │       │   │       │       ├── <263> Var [ptr]
            │       │   │       │       ╰── <265> Constant Int [5]
            │       │   │       ╰── <270> Constant Int [6]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <272> Constant Int [1]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <283>  [!=]
            │       │   │       ├── <280> Subscript
            │       │   │       │   ├── <278> Var [ptr]
            │       │   │       │   ╰── <279> Constant Int [0]
            │       │   │       ╰── <282> Constant Int [6]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <284> Constant Int [2]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <297>  [!=]
            │       │   │       ├── <290> Var [ptr]
            │       │   │       ╰── <296>  [+]
            │       │   │           ├── <293> Var [arr]
            │       │   │           ╰── <295> Constant Int [5]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <298> Constant Int [3]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <312>  [!=]
            │       │   │       ├── <309> Dereference
            │       │   │       │   ╰── <308> Assign [-=]
            │       │   │       │       ├── <304> Var [ptr]
            │       │   │       │       ╰── <306> Constant Int [3]
            │       │   │       ╰── <311> Constant Int [3]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <313> Constant Int [4]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <324>  [!=]
            │       │   │       ├── <321> Subscript
            │       │   │       │   ├── <319> Var [ptr]
            │       │   │       │   ╰── <320> Constant Int [0]
            │       │   │       ╰── <323> Constant Int [3]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <325> Constant Int [5]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <338>  [!=]
            │       │   │       ├── <331> Var [ptr]
            │       │   │       ╰── <337>  [+]
            │       │   │           ├── <334> Var [arr]
            │       │   │           ╰── <336> Constant Int [2]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <339> Constant Int [6]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <360>  [!=]
            │       │   │       ├── <353> Assign [+=]
            │       │   │       │   ├── <345> Var [ptr]
            │       │   │       │   ╰── <351>  [-]
            │       │   │       │       ├── <348> Var [i]
            │       │   │       │       ╰── <350> Constant Int [1]
            │       │   │       ╰── <359>  [+]
            │       │   │           ├── <356> Var [arr]
            │       │   │           ╰── <358> Constant Int [5]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <361> Constant Int [7]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <371>  [!=]
            │       │   │       ├── <368> Dereference
            │       │   │       │   ╰── <367> Var [ptr]
            │       │   │       ╰── <370> Constant Int [6]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <372> Constant Int [8]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <394>  [!=]
            │       │   │       ├── <387> Assign [-=]
            │       │   │       │   ├── <378> Var [ptr]
            │       │   │       │   ╰── <385>  [+]
            │       │   │       │       ├── <380> Constant UInt [4294967295]
            │       │   │       │       ╰── <383> Var [i]
            │       │   │       ╰── <393>  [+]
            │       │   │           ├── <390> Var [arr]
            │       │   │           ╰── <392> Constant Int [2]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <395> Constant Int [9]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <405>  [!=]
            │       │   │       ├── <402> Dereference
            │       │   │       │   ╰── <401> Var [ptr]
            │       │   │       ╰── <404> Constant Int [3]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <406> Constant Int [10]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── l
            │       │   ├── Type
            │       │   │   ╰── Long
            │       │   ╰── Initializer
            │       │       ╰── <414> Constant Long [9223372036854775807]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <433>  [!=]
            │       │   │       ├── <426> Assign [+=]
            │       │   │       │   ├── <418> Var [ptr]
            │       │   │       │   ╰── <424>  [-]
            │       │   │       │       ├── <421> Var [l]
            │       │   │       │       ╰── <423> Constant Long [9223372036854775806]
            │       │   │       ╰── <432>  [+]
            │       │   │           ├── <429> Var [arr]
            │       │   │           ╰── <431> Constant Int [3]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <434> Constant Int [11]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <444>  [!=]
            │       │   │       ├── <441> Dereference
            │       │   │       │   ╰── <440> Var [ptr]
            │       │   │       ╰── <443> Constant Int [4]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <445> Constant Int [12]
            │       ╰── Return
            │           ╰── <450> Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── result
                    │   ╰── Type
                    │       ╰── Int
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <468> Assign [=]
                    │   │       ├── <463> Var [result]
                    │   │       ╰── <466> FunctionCall [int_array]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <470> Var [result]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <481> Assign [=]
                    │   │       ├── <476> Var [result]
                    │   │       ╰── <479> FunctionCall [double_array]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <486>  [+]
                    │                   ├── <483> Var [result]
                    │                   ╰── <485> Constant Int [12]
                    ╰── Return
                        ╰── <491> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_incr_and_decr_nested_pointers() {
    let src = r#"
        
        int main(void) {
            long arr[2][3][4] = {
                {{1, 2, 3, 4}, {5, 6, 7, 8}, {9, 10, 11, 12}},
                {{13, 14, 15, 16}, {17, 18, 19, 20}, {21, 22, 23, 24}}};
            long (*outer_ptr)[3][4] = arr + 1;
            if (outer_ptr-- != &arr[1]) {
                return 1;
            }
            if (outer_ptr[0][1] != arr[0][1]) {
                return 2;
            }
            if ((++outer_ptr)[0][2][3] != 24) {
                return 3;
            }
            if (outer_ptr[0][2][3] != 24) {
                return 4;
            }
            long (*inner_ptr)[4] = arr[0] + 1;
            if (inner_ptr++[0][2] != 7) {
                return 5;
            }
            if (inner_ptr[0][2] != 11) {
                return 6;
            }
            if ((--inner_ptr)[0][1] != 6) {
                return 7;
            }
            long *scalar_ptr = arr[1][2];
            if (scalar_ptr--[2] != 23) {
                return 8;
            }
            if (scalar_ptr[2] != 22) {
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
                    │   │       ├── 2
                    │   │       ╰── Array
                    │   │           ├── 3
                    │   │           ╰── Array
                    │   │               ├── 4
                    │   │               ╰── Long
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── Compound
                    │           │   ├── Compound
                    │           │   │   ├── <14> Constant Int [1]
                    │           │   │   ├── <16> Constant Int [2]
                    │           │   │   ├── <18> Constant Int [3]
                    │           │   │   ╰── <20> Constant Int [4]
                    │           │   ├── Compound
                    │           │   │   ├── <23> Constant Int [5]
                    │           │   │   ├── <25> Constant Int [6]
                    │           │   │   ├── <27> Constant Int [7]
                    │           │   │   ╰── <29> Constant Int [8]
                    │           │   ╰── Compound
                    │           │       ├── <32> Constant Int [9]
                    │           │       ├── <34> Constant Int [10]
                    │           │       ├── <36> Constant Int [11]
                    │           │       ╰── <38> Constant Int [12]
                    │           ╰── Compound
                    │               ├── Compound
                    │               │   ├── <42> Constant Int [13]
                    │               │   ├── <44> Constant Int [14]
                    │               │   ├── <46> Constant Int [15]
                    │               │   ╰── <48> Constant Int [16]
                    │               ├── Compound
                    │               │   ├── <51> Constant Int [17]
                    │               │   ├── <53> Constant Int [18]
                    │               │   ├── <55> Constant Int [19]
                    │               │   ╰── <57> Constant Int [20]
                    │               ╰── Compound
                    │                   ├── <60> Constant Int [21]
                    │                   ├── <62> Constant Int [22]
                    │                   ├── <64> Constant Int [23]
                    │                   ╰── <66> Constant Int [24]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── outer_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Array
                    │   │           ├── 3
                    │   │           ╰── Array
                    │   │               ├── 4
                    │   │               ╰── Long
                    │   ╰── Initializer
                    │       ╰── <85>  [+]
                    │           ├── <82> Var [arr]
                    │           ╰── <84> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <98>  [!=]
                    │   │       ├── <91> Postfix [--]
                    │   │       │   ╰── <89> Var [outer_ptr]
                    │   │       ╰── <97> AddressOf
                    │   │           ╰── <96> Subscript
                    │   │               ├── <94> Var [arr]
                    │   │               ╰── <95> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <99> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <117>  [!=]
                    │   │       ├── <109> Subscript
                    │   │       │   ├── <107> Subscript
                    │   │       │   │   ├── <105> Var [outer_ptr]
                    │   │       │   │   ╰── <106> Constant Int [0]
                    │   │       │   ╰── <108> Constant Int [1]
                    │   │       ╰── <116> Subscript
                    │   │           ├── <114> Subscript
                    │   │           │   ├── <112> Var [arr]
                    │   │           │   ╰── <113> Constant Int [0]
                    │   │           ╰── <115> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <118> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <136>  [!=]
                    │   │       ├── <133> Subscript
                    │   │       │   ├── <131> Subscript
                    │   │       │   │   ├── <129> Subscript
                    │   │       │   │   │   ├── <127> Unary [++]
                    │   │       │   │   │   │   ╰── <125> Var [outer_ptr]
                    │   │       │   │   │   ╰── <128> Constant Int [0]
                    │   │       │   │   ╰── <130> Constant Int [2]
                    │   │       │   ╰── <132> Constant Int [3]
                    │   │       ╰── <135> Constant Int [24]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <137> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <152>  [!=]
                    │   │       ├── <149> Subscript
                    │   │       │   ├── <147> Subscript
                    │   │       │   │   ├── <145> Subscript
                    │   │       │   │   │   ├── <143> Var [outer_ptr]
                    │   │       │   │   │   ╰── <144> Constant Int [0]
                    │   │       │   │   ╰── <146> Constant Int [2]
                    │   │       │   ╰── <148> Constant Int [3]
                    │   │       ╰── <151> Constant Int [24]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <153> Constant Int [4]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── inner_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Array
                    │   │           ├── 4
                    │   │           ╰── Long
                    │   ╰── Initializer
                    │       ╰── <171>  [+]
                    │           ├── <168> Subscript
                    │           │   ├── <166> Var [arr]
                    │           │   ╰── <167> Constant Int [0]
                    │           ╰── <170> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <184>  [!=]
                    │   │       ├── <181> Subscript
                    │   │       │   ├── <179> Subscript
                    │   │       │   │   ├── <177> Postfix [++]
                    │   │       │   │   │   ╰── <175> Var [inner_ptr]
                    │   │       │   │   ╰── <178> Constant Int [0]
                    │   │       │   ╰── <180> Constant Int [2]
                    │   │       ╰── <183> Constant Int [7]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <185> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <198>  [!=]
                    │   │       ├── <195> Subscript
                    │   │       │   ├── <193> Subscript
                    │   │       │   │   ├── <191> Var [inner_ptr]
                    │   │       │   │   ╰── <192> Constant Int [0]
                    │   │       │   ╰── <194> Constant Int [2]
                    │   │       ╰── <197> Constant Int [11]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <199> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <215>  [!=]
                    │   │       ├── <212> Subscript
                    │   │       │   ├── <210> Subscript
                    │   │       │   │   ├── <208> Unary [--]
                    │   │       │   │   │   ╰── <206> Var [inner_ptr]
                    │   │       │   │   ╰── <209> Constant Int [0]
                    │   │       │   ╰── <211> Constant Int [1]
                    │   │       ╰── <214> Constant Int [6]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <216> Constant Int [7]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── scalar_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Long
                    │   ╰── Initializer
                    │       ╰── <230> Subscript
                    │           ├── <228> Subscript
                    │           │   ├── <226> Var [arr]
                    │           │   ╰── <227> Constant Int [1]
                    │           ╰── <229> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <241>  [!=]
                    │   │       ├── <238> Subscript
                    │   │       │   ├── <236> Postfix [--]
                    │   │       │   │   ╰── <234> Var [scalar_ptr]
                    │   │       │   ╰── <237> Constant Int [2]
                    │   │       ╰── <240> Constant Int [23]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <242> Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <253>  [!=]
                    │   │       ├── <250> Subscript
                    │   │       │   ├── <248> Var [scalar_ptr]
                    │   │       │   ╰── <249> Constant Int [2]
                    │   │       ╰── <252> Constant Int [22]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <254> Constant Int [9]
                    ╰── Return
                        ╰── <259> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_incr_and_decr_pointers() {
    let src = r#"
        
        int main(void) {
            double x[3] = {0.0, 1.0, 2.0};
            double *ptr = x;
            if (++ptr != x + 1) {
                return 1;
            }
            if (*ptr != 1.0) {
                return 2;
            }
            if (ptr++ != x + 1) {
                return 3;
            }
            if (ptr != x + 2) {
                return 4;
            }
            if (*ptr != 2.0) {
                return 5;
            }
            if (--ptr != x + 1) {
                return 6;
            }
            if (*ptr != 1.0) {
                return 7;
            }
            if (ptr-- != x + 1) {
                return 8;
            }
            if (*ptr != 0.0) {
                return 9;
            }
            if (ptr != x) {
                return 10;
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
                    │   │   ╰── Array
                    │   │       ├── 3
                    │   │       ╰── Double
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <10> Constant Double [+0e0]
                    │           ├── <12> Constant Double [+1e0]
                    │           ╰── <14> Constant Double [+2e0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Double
                    │   ╰── Initializer
                    │       ╰── <23> Var [x]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <36>  [!=]
                    │   │       ├── <29> Unary [++]
                    │   │       │   ╰── <28> Var [ptr]
                    │   │       ╰── <35>  [+]
                    │   │           ├── <32> Var [x]
                    │   │           ╰── <34> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <37> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <47>  [!=]
                    │   │       ├── <44> Dereference
                    │   │       │   ╰── <43> Var [ptr]
                    │   │       ╰── <46> Constant Double [+1e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <48> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <63>  [!=]
                    │   │       ├── <56> Postfix [++]
                    │   │       │   ╰── <54> Var [ptr]
                    │   │       ╰── <62>  [+]
                    │   │           ├── <59> Var [x]
                    │   │           ╰── <61> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <64> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <77>  [!=]
                    │   │       ├── <70> Var [ptr]
                    │   │       ╰── <76>  [+]
                    │   │           ├── <73> Var [x]
                    │   │           ╰── <75> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <78> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <88>  [!=]
                    │   │       ├── <85> Dereference
                    │   │       │   ╰── <84> Var [ptr]
                    │   │       ╰── <87> Constant Double [+2e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <89> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <104>  [!=]
                    │   │       ├── <97> Unary [--]
                    │   │       │   ╰── <96> Var [ptr]
                    │   │       ╰── <103>  [+]
                    │   │           ├── <100> Var [x]
                    │   │           ╰── <102> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <105> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <115>  [!=]
                    │   │       ├── <112> Dereference
                    │   │       │   ╰── <111> Var [ptr]
                    │   │       ╰── <114> Constant Double [+1e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <116> Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <131>  [!=]
                    │   │       ├── <124> Postfix [--]
                    │   │       │   ╰── <122> Var [ptr]
                    │   │       ╰── <130>  [+]
                    │   │           ├── <127> Var [x]
                    │   │           ╰── <129> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <132> Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <142>  [!=]
                    │   │       ├── <139> Dereference
                    │   │       │   ╰── <138> Var [ptr]
                    │   │       ╰── <141> Constant Double [+0e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <143> Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <153>  [!=]
                    │   │       ├── <149> Var [ptr]
                    │   │       ╰── <152> Var [x]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <154> Constant Int [10]
                    ╰── Return
                        ╰── <159> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_incr_decr_subscripted_vals() {
    let src = r#"
        int i = 2;
        int j = 1;
        int k = 0;
        int main(void) {
            int arr[3][2][2] = {
                {{1, 2}, {3, 4}}, {{5, 6}, {7, 8}}, {{9, 10}, {11, 12}}};
            if (arr[i][j][k]++ != 11) {
                return 1;
            }
            if (arr[i][j][k] != 12) {
                return 2;
            }
            if (++arr[--i][j--][++k] != 9) {
                return 3;
            }
            if (arr[i][j][k] != 6) {
                return 4;
            }
            if (--arr[i][j][k] != 5) {
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
            │       ╰── <4> Constant Int [2]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── j
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Initializer
            │       ╰── <10> Constant Int [1]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── k
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Initializer
            │       ╰── <16> Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── arr
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 3
                    │   │       ╰── Array
                    │   │           ├── 2
                    │   │           ╰── Array
                    │   │               ├── 2
                    │   │               ╰── Int
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── Compound
                    │           │   ├── Compound
                    │           │   │   ├── <32> Constant Int [1]
                    │           │   │   ╰── <34> Constant Int [2]
                    │           │   ╰── Compound
                    │           │       ├── <37> Constant Int [3]
                    │           │       ╰── <39> Constant Int [4]
                    │           ├── Compound
                    │           │   ├── Compound
                    │           │   │   ├── <43> Constant Int [5]
                    │           │   │   ╰── <45> Constant Int [6]
                    │           │   ╰── Compound
                    │           │       ├── <48> Constant Int [7]
                    │           │       ╰── <50> Constant Int [8]
                    │           ╰── Compound
                    │               ├── Compound
                    │               │   ├── <54> Constant Int [9]
                    │               │   ╰── <56> Constant Int [10]
                    │               ╰── Compound
                    │                   ├── <59> Constant Int [11]
                    │                   ╰── <61> Constant Int [12]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <82>  [!=]
                    │   │       ├── <79> Postfix [++]
                    │   │       │   ╰── <77> Subscript
                    │   │       │       ├── <74> Subscript
                    │   │       │       │   ├── <71> Subscript
                    │   │       │       │   │   ├── <68> Var [arr]
                    │   │       │       │   │   ╰── <70> Var [i]
                    │   │       │       │   ╰── <73> Var [j]
                    │   │       │       ╰── <76> Var [k]
                    │   │       ╰── <81> Constant Int [11]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <83> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <101>  [!=]
                    │   │       ├── <98> Subscript
                    │   │       │   ├── <95> Subscript
                    │   │       │   │   ├── <92> Subscript
                    │   │       │   │   │   ├── <89> Var [arr]
                    │   │       │   │   │   ╰── <91> Var [i]
                    │   │       │   │   ╰── <94> Var [j]
                    │   │       │   ╰── <97> Var [k]
                    │   │       ╰── <100> Constant Int [12]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <102> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <128>  [!=]
                    │   │       ├── <125> Unary [++]
                    │   │       │   ╰── <124> Subscript
                    │   │       │       ├── <119> Subscript
                    │   │       │       │   ├── <114> Subscript
                    │   │       │       │   │   ├── <109> Var [arr]
                    │   │       │       │   │   ╰── <113> Unary [--]
                    │   │       │       │   │       ╰── <112> Var [i]
                    │   │       │       │   ╰── <118> Postfix [--]
                    │   │       │       │       ╰── <116> Var [j]
                    │   │       │       ╰── <123> Unary [++]
                    │   │       │           ╰── <122> Var [k]
                    │   │       ╰── <127> Constant Int [9]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <129> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <147>  [!=]
                    │   │       ├── <144> Subscript
                    │   │       │   ├── <141> Subscript
                    │   │       │   │   ├── <138> Subscript
                    │   │       │   │   │   ├── <135> Var [arr]
                    │   │       │   │   │   ╰── <137> Var [i]
                    │   │       │   │   ╰── <140> Var [j]
                    │   │       │   ╰── <143> Var [k]
                    │   │       ╰── <146> Constant Int [6]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <148> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <168>  [!=]
                    │   │       ├── <165> Unary [--]
                    │   │       │   ╰── <164> Subscript
                    │   │       │       ├── <161> Subscript
                    │   │       │       │   ├── <158> Subscript
                    │   │       │       │   │   ├── <155> Var [arr]
                    │   │       │       │   │   ╰── <157> Var [i]
                    │   │       │       │   ╰── <160> Var [j]
                    │   │       │       ╰── <163> Var [k]
                    │   │       ╰── <167> Constant Int [5]
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
fn test_valid_extra_credit_postfix_prefix_precedence() {
    let src = r#"
        
        int idx = 3;
        int main(void) {
            int arr[5] = {1, 2, 3, 4, 5};
            int *ptr = arr + 1;
            int result = ++ptr--[idx];
            if (result != 6) {
                return 1;
            }
            if (*ptr != 1) {
                return 2;
            }
            if (ptr != arr) {
                return 3;
            }
            if (*ptr++ != 1) {
                return 4;
            }
            if (*ptr != 2) {
                return 5;
            }
            for (int i = 0; i < 4; i++) {
                if (arr[i] != i + 1) {
                    return 6;
                }
            }
            if (arr[4] != 6) {
                return 7;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── idx
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Initializer
            │       ╰── <4> Constant Int [3]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── arr
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 5
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <16> Constant Int [1]
                    │           ├── <18> Constant Int [2]
                    │           ├── <20> Constant Int [3]
                    │           ├── <22> Constant Int [4]
                    │           ╰── <24> Constant Int [5]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <36>  [+]
                    │           ├── <33> Var [arr]
                    │           ╰── <35> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── result
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <50> Unary [++]
                    │           ╰── <49> Subscript
                    │               ├── <46> Postfix [--]
                    │               │   ╰── <44> Var [ptr]
                    │               ╰── <48> Var [idx]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <57>  [!=]
                    │   │       ├── <54> Var [result]
                    │   │       ╰── <56> Constant Int [6]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <58> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <68>  [!=]
                    │   │       ├── <65> Dereference
                    │   │       │   ╰── <64> Var [ptr]
                    │   │       ╰── <67> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <69> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <79>  [!=]
                    │   │       ├── <75> Var [ptr]
                    │   │       ╰── <78> Var [arr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <80> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <92>  [!=]
                    │   │       ├── <89> Dereference
                    │   │       │   ╰── <88> Postfix [++]
                    │   │       │       ╰── <86> Var [ptr]
                    │   │       ╰── <91> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <93> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <103>  [!=]
                    │   │       ├── <100> Dereference
                    │   │       │   ╰── <99> Var [ptr]
                    │   │       ╰── <102> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <104> Constant Int [5]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <112> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <120>  [<]
                    │   │       ├── <117> Var [i]
                    │   │       ╰── <119> Constant Int [4]
                    │   ├── Condition
                    │   │   ╰── <124> Postfix [++]
                    │   │       ╰── <122> Var [i]
                    │   ╰── Block
                    │       ╰── If
                    │           ├── Condition
                    │           │   ╰── <136>  [!=]
                    │           │       ├── <129> Subscript
                    │           │       │   ├── <126> Var [arr]
                    │           │       │   ╰── <128> Var [i]
                    │           │       ╰── <135>  [+]
                    │           │           ├── <132> Var [i]
                    │           │           ╰── <134> Constant Int [1]
                    │           ╰── Then
                    │               ╰── Block
                    │                   ╰── Return
                    │                       ╰── <137> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <151>  [!=]
                    │   │       ├── <148> Subscript
                    │   │       │   ├── <146> Var [arr]
                    │   │       │   ╰── <147> Constant Int [4]
                    │   │       ╰── <150> Constant Int [6]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <152> Constant Int [7]
                    ╰── Return
                        ╰── <157> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_initialization_automatic() {
    let src = r#"
        int test_simple(void) {
            unsigned long arr[3] = {18446744073709551615UL, 9223372036854775807UL,
                                    100ul};
            return (arr[0] == 18446744073709551615UL &&
                    arr[1] == 9223372036854775807UL && arr[2] == 100ul);
        }
        int test_partial(void) {
            double arr[5] = {1.0, 123e4};
            return (arr[0] == 1.0 && arr[1] == 123e4 && !arr[2] && !arr[3] && !arr[4]);
        }
        int test_non_constant(long negative_7billion, int *ptr) {
            *ptr = 1;
            extern int three(void);
            long var = negative_7billion * three();
            long arr[5] = {
                negative_7billion,
                three() * 7l,
                -(long)*ptr,
                var + (negative_7billion ? 2 : 3)
            };
            return (arr[0] == -7000000000 && arr[1] == 21l && arr[2] == -1l &&
                    arr[3] == -20999999998l && arr[4] == 0l);
        }
        int three(void) {
            return 3;
        }
        long global_one = 1l;
        int test_type_conversion(int *ptr) {
            *ptr = -100;
            unsigned long arr[4] = {
                3458764513821589504.0,
                *ptr,
                (unsigned int)18446744073709551615UL,
                -global_one
            };
            return (arr[0] == 3458764513821589504ul &&
                    arr[1] == 18446744073709551516ul && arr[2] == 4294967295U &&
                    arr[3] == 18446744073709551615UL);
        }
        int test_preserve_stack(void) {
            int i = -1;
            int arr[3] = {global_one * 2l, global_one + three()};
            unsigned int u = 2684366905;
            if (i != -1) {
                return 0;
            }
            if (u != 2684366905) {
                return 0;
            }
            return (arr[0] == 2 && arr[1] == 4 && !arr[2]);
        }
        int main(void) {
            if (!test_simple()) {
                return 1;
            }
            if (!test_partial()) {
                return 2;
            }
            long negative_seven_billion = -7000000000l;
            int i = 0;
            if (!test_non_constant(negative_seven_billion, &i)) {
                return 3;
            }
            if (!test_type_conversion(&i)) {
                return 4;
            }
            if (!test_preserve_stack()) {
                return 5;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [test_simple]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── arr
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 3
            │       │   │       ╰── Unsigned Long
            │       │   ╰── Initializer
            │       │       ╰── Compound
            │       │           ├── <10> Constant ULong [18446744073709551615]
            │       │           ├── <12> Constant ULong [9223372036854775807]
            │       │           ╰── <14> Constant ULong [100]
            │       ╰── Return
            │           ╰── <43>  [&&]
            │               ├── <33>  [&&]
            │               │   ├── <24>  [==]
            │               │   │   ├── <21> Subscript
            │               │   │   │   ├── <19> Var [arr]
            │               │   │   │   ╰── <20> Constant Int [0]
            │               │   │   ╰── <23> Constant ULong [18446744073709551615]
            │               │   ╰── <32>  [==]
            │               │       ├── <29> Subscript
            │               │       │   ├── <27> Var [arr]
            │               │       │   ╰── <28> Constant Int [1]
            │               │       ╰── <31> Constant ULong [9223372036854775807]
            │               ╰── <41>  [==]
            │                   ├── <38> Subscript
            │                   │   ├── <36> Var [arr]
            │                   │   ╰── <37> Constant Int [2]
            │                   ╰── <40> Constant ULong [100]
            ├── Function [test_partial]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── arr
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 5
            │       │   │       ╰── Double
            │       │   ╰── Initializer
            │       │       ╰── Compound
            │       │           ├── <56> Constant Double [+1e0]
            │       │           ╰── <58> Constant Double [+1.23e6]
            │       ╰── Return
            │           ╰── <102>  [&&]
            │               ├── <93>  [&&]
            │               │   ├── <85>  [&&]
            │               │   │   ├── <77>  [&&]
            │               │   │   │   ├── <68>  [==]
            │               │   │   │   │   ├── <65> Subscript
            │               │   │   │   │   │   ├── <63> Var [arr]
            │               │   │   │   │   │   ╰── <64> Constant Int [0]
            │               │   │   │   │   ╰── <67> Constant Double [+1e0]
            │               │   │   │   ╰── <76>  [==]
            │               │   │   │       ├── <73> Subscript
            │               │   │   │       │   ├── <71> Var [arr]
            │               │   │   │       │   ╰── <72> Constant Int [1]
            │               │   │   │       ╰── <75> Constant Double [+1.23e6]
            │               │   │   ╰── <84> Unary [!]
            │               │   │       ╰── <83> Subscript
            │               │   │           ├── <81> Var [arr]
            │               │   │           ╰── <82> Constant Int [2]
            │               │   ╰── <92> Unary [!]
            │               │       ╰── <91> Subscript
            │               │           ├── <89> Var [arr]
            │               │           ╰── <90> Constant Int [3]
            │               ╰── <100> Unary [!]
            │                   ╰── <99> Subscript
            │                       ├── <97> Var [arr]
            │                       ╰── <98> Constant Int [4]
            ├── Function [test_non_constant]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── negative_7billion
            │   │   │   ╰── Type
            │   │   │       ╰── Long
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── ptr
            │   │       ╰── Type
            │   │           ╰── Pointer
            │   │               ╰── Int
            │   ╰── Body
            │       ├── <122> Assign [=]
            │       │   ├── <119> Dereference
            │       │   │   ╰── <118> Var [ptr]
            │       │   ╰── <121> Constant Int [1]
            │       ├── Function [extern three]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── var
            │       │   ├── Type
            │       │   │   ╰── Long
            │       │   ╰── Initializer
            │       │       ╰── <138>  [*]
            │       │           ├── <134> Var [negative_7billion]
            │       │           ╰── <137> FunctionCall [three]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── arr
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 5
            │       │   │       ╰── Long
            │       │   ╰── Initializer
            │       │       ╰── Compound
            │       │           ├── <147> Var [negative_7billion]
            │       │           ├── <153>  [*]
            │       │           │   ├── <150> FunctionCall [three]
            │       │           │   ╰── <152> Constant Long [7]
            │       │           ├── <162> Unary [-]
            │       │           │   ╰── <161> Cast
            │       │           │       ├── Target
            │       │           │       │   ╰── Long
            │       │           │       ╰── Expression
            │       │           │           ╰── <160> Dereference
            │       │           │               ╰── <159> Var [ptr]
            │       │           ╰── <173>  [+]
            │       │               ├── <165> Var [var]
            │       │               ╰── <172> Conditional [?]
            │       │                   ├── <168> Var [negative_7billion]
            │       │                   ├── Then
            │       │                   │   ╰── <169> Constant Int [2]
            │       │                   ╰── Else
            │       │                       ╰── <170> Constant Int [3]
            │       ╰── Return
            │           ╰── <226>  [&&]
            │               ├── <216>  [&&]
            │               │   ├── <205>  [&&]
            │               │   │   ├── <194>  [&&]
            │               │   │   │   ├── <185>  [==]
            │               │   │   │   │   ├── <180> Subscript
            │               │   │   │   │   │   ├── <178> Var [arr]
            │               │   │   │   │   │   ╰── <179> Constant Int [0]
            │               │   │   │   │   ╰── <184> Unary [-]
            │               │   │   │   │       ╰── <183> Constant Long [7000000000]
            │               │   │   │   ╰── <193>  [==]
            │               │   │   │       ├── <190> Subscript
            │               │   │   │       │   ├── <188> Var [arr]
            │               │   │   │       │   ╰── <189> Constant Int [1]
            │               │   │   │       ╰── <192> Constant Long [21]
            │               │   │   ╰── <204>  [==]
            │               │   │       ├── <199> Subscript
            │               │   │       │   ├── <197> Var [arr]
            │               │   │       │   ╰── <198> Constant Int [2]
            │               │   │       ╰── <203> Unary [-]
            │               │   │           ╰── <202> Constant Long [1]
            │               │   ╰── <215>  [==]
            │               │       ├── <210> Subscript
            │               │       │   ├── <208> Var [arr]
            │               │       │   ╰── <209> Constant Int [3]
            │               │       ╰── <214> Unary [-]
            │               │           ╰── <213> Constant Long [20999999998]
            │               ╰── <224>  [==]
            │                   ├── <221> Subscript
            │                   │   ├── <219> Var [arr]
            │                   │   ╰── <220> Constant Int [4]
            │                   ╰── <223> Constant Long [0]
            ├── Function [three]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <234> Constant Int [3]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── global_one
            │   ├── Type
            │   │   ╰── Long
            │   ╰── Initializer
            │       ╰── <241> Constant Long [1]
            ├── Function [test_type_conversion]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── ptr
            │   │       ╰── Type
            │   │           ╰── Pointer
            │   │               ╰── Int
            │   ╰── Body
            │       ├── <259> Assign [=]
            │       │   ├── <254> Dereference
            │       │   │   ╰── <253> Var [ptr]
            │       │   ╰── <258> Unary [-]
            │       │       ╰── <257> Constant Int [100]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── arr
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 4
            │       │   │       ╰── Unsigned Long
            │       │   ╰── Initializer
            │       │       ╰── Compound
            │       │           ├── <266> Constant Double [+3.4587645138215895e18]
            │       │           ├── <270> Dereference
            │       │           │   ╰── <269> Var [ptr]
            │       │           ├── <275> Cast
            │       │           │   ├── Target
            │       │           │   │   ╰── Unsigned Int
            │       │           │   ╰── Expression
            │       │           │       ╰── <274> Constant ULong [18446744073709551615]
            │       │           ╰── <280> Unary [-]
            │       │               ╰── <279> Var [global_one]
            │       ╰── Return
            │           ╰── <318>  [&&]
            │               ├── <308>  [&&]
            │               │   ├── <299>  [&&]
            │               │   │   ├── <290>  [==]
            │               │   │   │   ├── <287> Subscript
            │               │   │   │   │   ├── <285> Var [arr]
            │               │   │   │   │   ╰── <286> Constant Int [0]
            │               │   │   │   ╰── <289> Constant ULong [3458764513821589504]
            │               │   │   ╰── <298>  [==]
            │               │   │       ├── <295> Subscript
            │               │   │       │   ├── <293> Var [arr]
            │               │   │       │   ╰── <294> Constant Int [1]
            │               │   │       ╰── <297> Constant ULong [18446744073709551516]
            │               │   ╰── <307>  [==]
            │               │       ├── <304> Subscript
            │               │       │   ├── <302> Var [arr]
            │               │       │   ╰── <303> Constant Int [2]
            │               │       ╰── <306> Constant UInt [4294967295]
            │               ╰── <316>  [==]
            │                   ├── <313> Subscript
            │                   │   ├── <311> Var [arr]
            │                   │   ╰── <312> Constant Int [3]
            │                   ╰── <315> Constant ULong [18446744073709551615]
            ├── Function [test_preserve_stack]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── i
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <331> Unary [-]
            │       │           ╰── <330> Constant Int [1]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── arr
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 3
            │       │   │       ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── Compound
            │       │           ├── <343>  [*]
            │       │           │   ├── <340> Var [global_one]
            │       │           │   ╰── <342> Constant Long [2]
            │       │           ╰── <350>  [+]
            │       │               ├── <346> Var [global_one]
            │       │               ╰── <349> FunctionCall [three]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── u
            │       │   ├── Type
            │       │   │   ╰── Unsigned Int
            │       │   ╰── Initializer
            │       │       ╰── <357> Constant Long [2684366905]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <366>  [!=]
            │       │   │       ├── <361> Var [i]
            │       │   │       ╰── <365> Unary [-]
            │       │   │           ╰── <364> Constant Int [1]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <367> Constant Int [0]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <376>  [!=]
            │       │   │       ├── <373> Var [u]
            │       │   │       ╰── <375> Constant Long [2684366905]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <377> Constant Int [0]
            │       ╰── Return
            │           ╰── <406>  [&&]
            │               ├── <397>  [&&]
            │               │   ├── <388>  [==]
            │               │   │   ├── <385> Subscript
            │               │   │   │   ├── <383> Var [arr]
            │               │   │   │   ╰── <384> Constant Int [0]
            │               │   │   ╰── <387> Constant Int [2]
            │               │   ╰── <396>  [==]
            │               │       ├── <393> Subscript
            │               │       │   ├── <391> Var [arr]
            │               │       │   ╰── <392> Constant Int [1]
            │               │       ╰── <395> Constant Int [4]
            │               ╰── <404> Unary [!]
            │                   ╰── <403> Subscript
            │                       ├── <401> Var [arr]
            │                       ╰── <402> Constant Int [2]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <417> Unary [!]
                    │   │       ╰── <416> FunctionCall [test_simple]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <418> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <426> Unary [!]
                    │   │       ╰── <425> FunctionCall [test_partial]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <427> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── negative_seven_billion
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <437> Unary [-]
                    │           ╰── <436> Constant Long [7000000000]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <443> Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <454> Unary [!]
                    │   │       ╰── <453> FunctionCall [test_non_constant]
                    │   │           ├── <449> Var [negative_seven_billion]
                    │   │           ╰── <452> AddressOf
                    │   │               ╰── <451> Var [i]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <455> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <466> Unary [!]
                    │   │       ╰── <465> FunctionCall [test_type_conversion]
                    │   │           ╰── <464> AddressOf
                    │   │               ╰── <463> Var [i]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <467> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <475> Unary [!]
                    │   │       ╰── <474> FunctionCall [test_preserve_stack]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <476> Constant Int [5]
                    ╰── Return
                        ╰── <481> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_initialization_automatic_nested() {
    let src = r#"
        int test_simple(void) {
            int arr[3][3] = {{1, 2, 3}, {4, 5, 6}, {7, 8, 9}};
            for (int i = 0; i < 3; i = i + 1) {
                for (int j = 0; j < 3; j = j + 1) {
                    if (arr[i][j] != i * 3 + j + 1) {
                        return 0;
                    }
                }
            }
            return 1;
        }
        int test_partial(void) {
            int first_half_only[4][2][6] = {
                {{1, 2, 3}},
                {{4, 5, 6}}
            };
            int expected = 1;
            for (int i = 0; i < 4; i = i + 1) {
                for (int j = 0; j < 2; j = j + 1) {
                    for (int k = 0; k < 6; k = k + 1) {
                        int val = first_half_only[i][j][k];
                        if (i > 1 || j > 0 || k > 2) {
                            if (val) {
                                return 0;
                            }
                        } else {
                            if (val != expected) {
                                return 0;
                            }
                            expected = expected + 1;
                        }
                    }
                }
            }
            return 1;
        }
        int test_non_constant_and_type_conversion(void) {
            extern unsigned int three(void);
            static int x = 2000;
            int negative_four = -4;
            int *ptr = &negative_four;
            double arr[3][2] = {
                {x, x / *ptr},
                {three()},
            };
            if (arr[0][0] != 2000.0 || arr[0][1] != -500.0 || arr[1][0] != 3.0) {
                return 0;
            }
            if (arr[1][1] || arr[2][0] || arr[2][1]) {
                return 0;
            }
            return 1;
        }
        unsigned int three(void) {
            return 3u;
        }
        long one = 1l;
        int test_preserve_stack(void) {
            int i = -1;
            int arr[3][1] = {{one * 2l}, {one + three()}};
            unsigned int u = 2684366905;
            if (i != -1) {
                return 0;
            }
            if (u != 2684366905) {
                return 0;
            }
            if (arr[0][0] != 2 || arr[1][0] != 4 || arr[2][0] != 0) {
                return 0;
            }
            return 1;
        }
        int main(void) {
            if (!test_simple()) {
                return 1;
            }
            if (!test_partial()) {
                return 2;
            }
            if (!test_non_constant_and_type_conversion()) {
                return 3;
            }
            if (!test_preserve_stack()) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [test_simple]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── arr
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 3
            │       │   │       ╰── Array
            │       │   │           ├── 3
            │       │   │           ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── Compound
            │       │           ├── Compound
            │       │           │   ├── <12> Constant Int [1]
            │       │           │   ├── <14> Constant Int [2]
            │       │           │   ╰── <16> Constant Int [3]
            │       │           ├── Compound
            │       │           │   ├── <19> Constant Int [4]
            │       │           │   ├── <21> Constant Int [5]
            │       │           │   ╰── <23> Constant Int [6]
            │       │           ╰── Compound
            │       │               ├── <26> Constant Int [7]
            │       │               ├── <28> Constant Int [8]
            │       │               ╰── <30> Constant Int [9]
            │       ├── For
            │       │   ├── Init
            │       │   │   ╰── VarDeclaration
            │       │   │       ├── Name
            │       │   │       │   ╰── i
            │       │   │       ├── Type
            │       │   │       │   ╰── Int
            │       │   │       ╰── Initializer
            │       │   │           ╰── <38> Constant Int [0]
            │       │   ├── Condition
            │       │   │   ╰── <46>  [<]
            │       │   │       ├── <43> Var [i]
            │       │   │       ╰── <45> Constant Int [3]
            │       │   ├── Condition
            │       │   │   ╰── <55> Assign [=]
            │       │   │       ├── <48> Var [i]
            │       │   │       ╰── <54>  [+]
            │       │   │           ├── <51> Var [i]
            │       │   │           ╰── <53> Constant Int [1]
            │       │   ╰── Block
            │       │       ╰── For
            │       │           ├── Init
            │       │           │   ╰── VarDeclaration
            │       │           │       ├── Name
            │       │           │       │   ╰── j
            │       │           │       ├── Type
            │       │           │       │   ╰── Int
            │       │           │       ╰── Initializer
            │       │           │           ╰── <59> Constant Int [0]
            │       │           ├── Condition
            │       │           │   ╰── <67>  [<]
            │       │           │       ├── <64> Var [j]
            │       │           │       ╰── <66> Constant Int [3]
            │       │           ├── Condition
            │       │           │   ╰── <76> Assign [=]
            │       │           │       ├── <69> Var [j]
            │       │           │       ╰── <75>  [+]
            │       │           │           ├── <72> Var [j]
            │       │           │           ╰── <74> Constant Int [1]
            │       │           ╰── Block
            │       │               ╰── If
            │       │                   ├── Condition
            │       │                   │   ╰── <98>  [!=]
            │       │                   │       ├── <84> Subscript
            │       │                   │       │   ├── <81> Subscript
            │       │                   │       │   │   ├── <78> Var [arr]
            │       │                   │       │   │   ╰── <80> Var [i]
            │       │                   │       │   ╰── <83> Var [j]
            │       │                   │       ╰── <97>  [+]
            │       │                   │           ├── <94>  [+]
            │       │                   │           │   ├── <90>  [*]
            │       │                   │           │   │   ├── <87> Var [i]
            │       │                   │           │   │   ╰── <89> Constant Int [3]
            │       │                   │           │   ╰── <93> Var [j]
            │       │                   │           ╰── <96> Constant Int [1]
            │       │                   ╰── Then
            │       │                       ╰── Block
            │       │                           ╰── Return
            │       │                               ╰── <99> Constant Int [0]
            │       ╰── Return
            │           ╰── <110> Constant Int [1]
            ├── Function [test_partial]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── first_half_only
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 4
            │       │   │       ╰── Array
            │       │   │           ├── 2
            │       │   │           ╰── Array
            │       │   │               ├── 6
            │       │   │               ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── Compound
            │       │           ├── Compound
            │       │           │   ╰── Compound
            │       │           │       ├── <127> Constant Int [1]
            │       │           │       ├── <129> Constant Int [2]
            │       │           │       ╰── <131> Constant Int [3]
            │       │           ╰── Compound
            │       │               ╰── Compound
            │       │                   ├── <135> Constant Int [4]
            │       │                   ├── <137> Constant Int [5]
            │       │                   ╰── <139> Constant Int [6]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── expected
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <148> Constant Int [1]
            │       ├── For
            │       │   ├── Init
            │       │   │   ╰── VarDeclaration
            │       │   │       ├── Name
            │       │   │       │   ╰── i
            │       │   │       ├── Type
            │       │   │       │   ╰── Int
            │       │   │       ╰── Initializer
            │       │   │           ╰── <154> Constant Int [0]
            │       │   ├── Condition
            │       │   │   ╰── <162>  [<]
            │       │   │       ├── <159> Var [i]
            │       │   │       ╰── <161> Constant Int [4]
            │       │   ├── Condition
            │       │   │   ╰── <171> Assign [=]
            │       │   │       ├── <164> Var [i]
            │       │   │       ╰── <170>  [+]
            │       │   │           ├── <167> Var [i]
            │       │   │           ╰── <169> Constant Int [1]
            │       │   ╰── Block
            │       │       ╰── For
            │       │           ├── Init
            │       │           │   ╰── VarDeclaration
            │       │           │       ├── Name
            │       │           │       │   ╰── j
            │       │           │       ├── Type
            │       │           │       │   ╰── Int
            │       │           │       ╰── Initializer
            │       │           │           ╰── <175> Constant Int [0]
            │       │           ├── Condition
            │       │           │   ╰── <183>  [<]
            │       │           │       ├── <180> Var [j]
            │       │           │       ╰── <182> Constant Int [2]
            │       │           ├── Condition
            │       │           │   ╰── <192> Assign [=]
            │       │           │       ├── <185> Var [j]
            │       │           │       ╰── <191>  [+]
            │       │           │           ├── <188> Var [j]
            │       │           │           ╰── <190> Constant Int [1]
            │       │           ╰── Block
            │       │               ╰── For
            │       │                   ├── Init
            │       │                   │   ╰── VarDeclaration
            │       │                   │       ├── Name
            │       │                   │       │   ╰── k
            │       │                   │       ├── Type
            │       │                   │       │   ╰── Int
            │       │                   │       ╰── Initializer
            │       │                   │           ╰── <196> Constant Int [0]
            │       │                   ├── Condition
            │       │                   │   ╰── <204>  [<]
            │       │                   │       ├── <201> Var [k]
            │       │                   │       ╰── <203> Constant Int [6]
            │       │                   ├── Condition
            │       │                   │   ╰── <213> Assign [=]
            │       │                   │       ├── <206> Var [k]
            │       │                   │       ╰── <212>  [+]
            │       │                   │           ├── <209> Var [k]
            │       │                   │           ╰── <211> Constant Int [1]
            │       │                   ╰── Block
            │       │                       ├── VarDeclaration
            │       │                       │   ├── Name
            │       │                       │   │   ╰── val
            │       │                       │   ├── Type
            │       │                       │   │   ╰── Int
            │       │                       │   ╰── Initializer
            │       │                       │       ╰── <227> Subscript
            │       │                       │           ├── <224> Subscript
            │       │                       │           │   ├── <221> Subscript
            │       │                       │           │   │   ├── <218> Var [first_half_only]
            │       │                       │           │   │   ╰── <220> Var [i]
            │       │                       │           │   ╰── <223> Var [j]
            │       │                       │           ╰── <226> Var [k]
            │       │                       ╰── If
            │       │                           ├── Condition
            │       │                           │   ╰── <248>  [||]
            │       │                           │       ├── <241>  [||]
            │       │                           │       │   ├── <234>  [>]
            │       │                           │       │   │   ├── <231> Var [i]
            │       │                           │       │   │   ╰── <233> Constant Int [1]
            │       │                           │       │   ╰── <240>  [>]
            │       │                           │       │       ├── <237> Var [j]
            │       │                           │       │       ╰── <239> Constant Int [0]
            │       │                           │       ╰── <247>  [>]
            │       │                           │           ├── <244> Var [k]
            │       │                           │           ╰── <246> Constant Int [2]
            │       │                           ├── Then
            │       │                           │   ╰── Block
            │       │                           │       ╰── If
            │       │                           │           ├── Condition
            │       │                           │           │   ╰── <250> Var [val]
            │       │                           │           ╰── Then
            │       │                           │               ╰── Block
            │       │                           │                   ╰── Return
            │       │                           │                       ╰── <251> Constant Int [0]
            │       │                           ╰── Else
            │       │                               ╰── Block
            │       │                                   ├── If
            │       │                                   │   ├── Condition
            │       │                                   │   │   ╰── <263>  [!=]
            │       │                                   │   │       ├── <259> Var [val]
            │       │                                   │   │       ╰── <262> Var [expected]
            │       │                                   │   ╰── Then
            │       │                                   │       ╰── Block
            │       │                                   │           ╰── Return
            │       │                                   │               ╰── <264> Constant Int [0]
            │       │                                   ╰── <277> Assign [=]
            │       │                                       ├── <270> Var [expected]
            │       │                                       ╰── <276>  [+]
            │       │                                           ├── <273> Var [expected]
            │       │                                           ╰── <275> Constant Int [1]
            │       ╰── Return
            │           ╰── <291> Constant Int [1]
            ├── Function [test_non_constant_and_type_conversion]
            │   ╰── Body
            │       ├── Function [extern three]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── x
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ├── Initializer
            │       │   │   ╰── <309> Constant Int [2000]
            │       │   ╰── Static
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── negative_four
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <317> Unary [-]
            │       │           ╰── <316> Constant Int [4]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ptr
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <326> AddressOf
            │       │           ╰── <325> Var [negative_four]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── arr
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 3
            │       │   │       ╰── Array
            │       │   │           ├── 2
            │       │   │           ╰── Double
            │       │   ╰── Initializer
            │       │       ╰── Compound
            │       │           ├── Compound
            │       │           │   ├── <337> Var [x]
            │       │           │   ╰── <345>  [/]
            │       │           │       ├── <340> Var [x]
            │       │           │       ╰── <344> Dereference
            │       │           │           ╰── <343> Var [ptr]
            │       │           ╰── Compound
            │       │               ╰── <349> FunctionCall [three]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <386>  [||]
            │       │   │       ├── <375>  [||]
            │       │   │       │   ├── <362>  [!=]
            │       │   │       │   │   ├── <359> Subscript
            │       │   │       │   │   │   ├── <357> Subscript
            │       │   │       │   │   │   │   ├── <355> Var [arr]
            │       │   │       │   │   │   │   ╰── <356> Constant Int [0]
            │       │   │       │   │   │   ╰── <358> Constant Int [0]
            │       │   │       │   │   ╰── <361> Constant Double [+2e3]
            │       │   │       │   ╰── <374>  [!=]
            │       │   │       │       ├── <369> Subscript
            │       │   │       │       │   ├── <367> Subscript
            │       │   │       │       │   │   ├── <365> Var [arr]
            │       │   │       │       │   │   ╰── <366> Constant Int [0]
            │       │   │       │       │   ╰── <368> Constant Int [1]
            │       │   │       │       ╰── <373> Unary [-]
            │       │   │       │           ╰── <372> Constant Double [+5e2]
            │       │   │       ╰── <385>  [!=]
            │       │   │           ├── <382> Subscript
            │       │   │           │   ├── <380> Subscript
            │       │   │           │   │   ├── <378> Var [arr]
            │       │   │           │   │   ╰── <379> Constant Int [1]
            │       │   │           │   ╰── <381> Constant Int [0]
            │       │   │           ╰── <384> Constant Double [+3e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <387> Constant Int [0]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <413>  [||]
            │       │   │       ├── <405>  [||]
            │       │   │       │   ├── <397> Subscript
            │       │   │       │   │   ├── <395> Subscript
            │       │   │       │   │   │   ├── <393> Var [arr]
            │       │   │       │   │   │   ╰── <394> Constant Int [1]
            │       │   │       │   │   ╰── <396> Constant Int [1]
            │       │   │       │   ╰── <404> Subscript
            │       │   │       │       ├── <402> Subscript
            │       │   │       │       │   ├── <400> Var [arr]
            │       │   │       │       │   ╰── <401> Constant Int [2]
            │       │   │       │       ╰── <403> Constant Int [0]
            │       │   │       ╰── <412> Subscript
            │       │   │           ├── <410> Subscript
            │       │   │           │   ├── <408> Var [arr]
            │       │   │           │   ╰── <409> Constant Int [2]
            │       │   │           ╰── <411> Constant Int [1]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <414> Constant Int [0]
            │       ╰── Return
            │           ╰── <419> Constant Int [1]
            ├── Function [three]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <427> Constant UInt [3]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── one
            │   ├── Type
            │   │   ╰── Long
            │   ╰── Initializer
            │       ╰── <434> Constant Long [1]
            ├── Function [test_preserve_stack]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── i
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <446> Unary [-]
            │       │           ╰── <445> Constant Int [1]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── arr
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 3
            │       │   │       ╰── Array
            │       │   │           ├── 1
            │       │   │           ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── Compound
            │       │           ├── Compound
            │       │           │   ╰── <460>  [*]
            │       │           │       ├── <457> Var [one]
            │       │           │       ╰── <459> Constant Long [2]
            │       │           ╰── Compound
            │       │               ╰── <468>  [+]
            │       │                   ├── <464> Var [one]
            │       │                   ╰── <467> FunctionCall [three]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── u
            │       │   ├── Type
            │       │   │   ╰── Unsigned Int
            │       │   ╰── Initializer
            │       │       ╰── <476> Constant Long [2684366905]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <485>  [!=]
            │       │   │       ├── <480> Var [i]
            │       │   │       ╰── <484> Unary [-]
            │       │   │           ╰── <483> Constant Int [1]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <486> Constant Int [0]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <495>  [!=]
            │       │   │       ├── <492> Var [u]
            │       │   │       ╰── <494> Constant Long [2684366905]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <496> Constant Int [0]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <531>  [||]
            │       │   │       ├── <520>  [||]
            │       │   │       │   ├── <509>  [!=]
            │       │   │       │   │   ├── <506> Subscript
            │       │   │       │   │   │   ├── <504> Subscript
            │       │   │       │   │   │   │   ├── <502> Var [arr]
            │       │   │       │   │   │   │   ╰── <503> Constant Int [0]
            │       │   │       │   │   │   ╰── <505> Constant Int [0]
            │       │   │       │   │   ╰── <508> Constant Int [2]
            │       │   │       │   ╰── <519>  [!=]
            │       │   │       │       ├── <516> Subscript
            │       │   │       │       │   ├── <514> Subscript
            │       │   │       │       │   │   ├── <512> Var [arr]
            │       │   │       │       │   │   ╰── <513> Constant Int [1]
            │       │   │       │       │   ╰── <515> Constant Int [0]
            │       │   │       │       ╰── <518> Constant Int [4]
            │       │   │       ╰── <530>  [!=]
            │       │   │           ├── <527> Subscript
            │       │   │           │   ├── <525> Subscript
            │       │   │           │   │   ├── <523> Var [arr]
            │       │   │           │   │   ╰── <524> Constant Int [2]
            │       │   │           │   ╰── <526> Constant Int [0]
            │       │   │           ╰── <529> Constant Int [0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <532> Constant Int [0]
            │       ╰── Return
            │           ╰── <537> Constant Int [1]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <548> Unary [!]
                    │   │       ╰── <547> FunctionCall [test_simple]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <549> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <557> Unary [!]
                    │   │       ╰── <556> FunctionCall [test_partial]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <558> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <566> Unary [!]
                    │   │       ╰── <565> FunctionCall [test_non_constant_and_type_conversion]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <567> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <575> Unary [!]
                    │   │       ╰── <574> FunctionCall [test_preserve_stack]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <576> Constant Int [4]
                    ╰── Return
                        ╰── <581> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_initialization_static() {
    let src = r#"
        double double_arr[3] = {1.0, 2.0, 3.0};
        int check_double_arr(double *arr) {
            if (arr[0] != 1.0) {
                return 1;
            }
            if (arr[1] != 2.0) {
                return 2;
            }
            if (arr[2] != 3.0) {
                return 3;
            }
            return 0;
        }
        unsigned uint_arr[5] = {
            1u,
            0u,
            2147497230u,
        };
        int check_uint_arr(unsigned *arr) {
            if (arr[0] != 1u) {
                return 4;
            }
            if (arr[1]) {
                return 5;
            }
            if (arr[2] != 2147497230u) {
                return 6;
            }
            if (arr[3] || arr[4]) {
                return 7;
            }
            return 0;
        }
        long long_arr[1000];
        int check_long_arr(long *arr) {
            for (int i = 0; i < 1000; i = i + 1) {
                if (arr[i]) {
                    return 8;
                }
            }
            return 0;
        }
        unsigned long ulong_arr[4] = {
            100.0, 11, 12345l, 4294967295U
        };
        int check_ulong_arr(unsigned long *arr) {
            if (arr[0] != 100ul) {
                return 9;
            }
            if (arr[1] != 11ul) {
                return 10;
            }
            if (arr[2] != 12345ul) {
                return 11;
            }
            if (arr[3] != 4294967295Ul) {
                return 12;
            }
            return 0;
        }
        int test_global(void) {
            int check = check_double_arr(double_arr);
            if (check) {
                return check;
            }
            check = check_uint_arr(uint_arr);
            if (check) {
                return check;
            }
            check = check_long_arr(long_arr);
            if (check) {
                return check;
            }
            check = check_ulong_arr(ulong_arr);
            if (check) {
                return check;
            }
            return 0;
        }
        int test_local(void) {
            double local_double_arr[3] = {1.0, 2.0, 3.0};
            static unsigned local_uint_arr[5] = {
                1u,
                0u,
                2147497230u,
            };
            static long local_long_arr[1000];
            static unsigned long local_ulong_arr[4] = {
                100.0, 11, 12345l, 4294967295U
            };
            int check = check_double_arr(local_double_arr);
            if (check) {
                return 100 + check;
            }
            check = check_uint_arr(local_uint_arr);
            if (check) {
                return 100 + check;
            }
            check = check_long_arr(local_long_arr);
            if (check) {
                return 100 + check;
            }
            check = check_ulong_arr(local_ulong_arr);
            if (check) {
                return 100 + check;
            }
            return 0;
        }
        int main(void) {
            int check = test_global();
            if (check) {
                return check;
            }
            return test_local();
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── double_arr
            │   ├── Type
            │   │   ╰── Array
            │   │       ├── 3
            │   │       ╰── Double
            │   ╰── Initializer
            │       ╰── Compound
            │           ├── <6> Constant Double [+1e0]
            │           ├── <8> Constant Double [+2e0]
            │           ╰── <10> Constant Double [+3e0]
            ├── Function [check_double_arr]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── arr
            │   │       ╰── Type
            │   │           ╰── Pointer
            │   │               ╰── Double
            │   ╰── Body
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <28>  [!=]
            │       │   │       ├── <25> Subscript
            │       │   │       │   ├── <23> Var [arr]
            │       │   │       │   ╰── <24> Constant Int [0]
            │       │   │       ╰── <27> Constant Double [+1e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <29> Constant Int [1]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <40>  [!=]
            │       │   │       ├── <37> Subscript
            │       │   │       │   ├── <35> Var [arr]
            │       │   │       │   ╰── <36> Constant Int [1]
            │       │   │       ╰── <39> Constant Double [+2e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <41> Constant Int [2]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <52>  [!=]
            │       │   │       ├── <49> Subscript
            │       │   │       │   ├── <47> Var [arr]
            │       │   │       │   ╰── <48> Constant Int [2]
            │       │   │       ╰── <51> Constant Double [+3e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <53> Constant Int [3]
            │       ╰── Return
            │           ╰── <58> Constant Int [0]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── uint_arr
            │   ├── Type
            │   │   ╰── Array
            │   │       ├── 5
            │   │       ╰── Unsigned Int
            │   ╰── Initializer
            │       ╰── Compound
            │           ├── <67> Constant UInt [1]
            │           ├── <69> Constant UInt [0]
            │           ╰── <71> Constant UInt [2147497230]
            ├── Function [check_uint_arr]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── arr
            │   │       ╰── Type
            │   │           ╰── Pointer
            │   │               ╰── Unsigned Int
            │   ╰── Body
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <89>  [!=]
            │       │   │       ├── <86> Subscript
            │       │   │       │   ├── <84> Var [arr]
            │       │   │       │   ╰── <85> Constant Int [0]
            │       │   │       ╰── <88> Constant UInt [1]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <90> Constant Int [4]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <98> Subscript
            │       │   │       ├── <96> Var [arr]
            │       │   │       ╰── <97> Constant Int [1]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <99> Constant Int [5]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <110>  [!=]
            │       │   │       ├── <107> Subscript
            │       │   │       │   ├── <105> Var [arr]
            │       │   │       │   ╰── <106> Constant Int [2]
            │       │   │       ╰── <109> Constant UInt [2147497230]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <111> Constant Int [6]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <125>  [||]
            │       │   │       ├── <119> Subscript
            │       │   │       │   ├── <117> Var [arr]
            │       │   │       │   ╰── <118> Constant Int [3]
            │       │   │       ╰── <124> Subscript
            │       │   │           ├── <122> Var [arr]
            │       │   │           ╰── <123> Constant Int [4]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <126> Constant Int [7]
            │       ╰── Return
            │           ╰── <131> Constant Int [0]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── long_arr
            │   ╰── Type
            │       ╰── Array
            │           ├── 1000
            │           ╰── Long
            ├── Function [check_long_arr]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── arr
            │   │       ╰── Type
            │   │           ╰── Pointer
            │   │               ╰── Long
            │   ╰── Body
            │       ├── For
            │       │   ├── Init
            │       │   │   ╰── VarDeclaration
            │       │   │       ├── Name
            │       │   │       │   ╰── i
            │       │   │       ├── Type
            │       │   │       │   ╰── Int
            │       │   │       ╰── Initializer
            │       │   │           ╰── <152> Constant Int [0]
            │       │   ├── Condition
            │       │   │   ╰── <160>  [<]
            │       │   │       ├── <157> Var [i]
            │       │   │       ╰── <159> Constant Int [1000]
            │       │   ├── Condition
            │       │   │   ╰── <169> Assign [=]
            │       │   │       ├── <162> Var [i]
            │       │   │       ╰── <168>  [+]
            │       │   │           ├── <165> Var [i]
            │       │   │           ╰── <167> Constant Int [1]
            │       │   ╰── Block
            │       │       ╰── If
            │       │           ├── Condition
            │       │           │   ╰── <174> Subscript
            │       │           │       ├── <171> Var [arr]
            │       │           │       ╰── <173> Var [i]
            │       │           ╰── Then
            │       │               ╰── Block
            │       │                   ╰── Return
            │       │                       ╰── <175> Constant Int [8]
            │       ╰── Return
            │           ╰── <183> Constant Int [0]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── ulong_arr
            │   ├── Type
            │   │   ╰── Array
            │   │       ├── 4
            │   │       ╰── Unsigned Long
            │   ╰── Initializer
            │       ╰── Compound
            │           ├── <192> Constant Double [+1e2]
            │           ├── <194> Constant Int [11]
            │           ├── <196> Constant Long [12345]
            │           ╰── <198> Constant UInt [4294967295]
            ├── Function [check_ulong_arr]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── arr
            │   │       ╰── Type
            │   │           ╰── Pointer
            │   │               ╰── Unsigned Long
            │   ╰── Body
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <216>  [!=]
            │       │   │       ├── <213> Subscript
            │       │   │       │   ├── <211> Var [arr]
            │       │   │       │   ╰── <212> Constant Int [0]
            │       │   │       ╰── <215> Constant ULong [100]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <217> Constant Int [9]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <228>  [!=]
            │       │   │       ├── <225> Subscript
            │       │   │       │   ├── <223> Var [arr]
            │       │   │       │   ╰── <224> Constant Int [1]
            │       │   │       ╰── <227> Constant ULong [11]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <229> Constant Int [10]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <240>  [!=]
            │       │   │       ├── <237> Subscript
            │       │   │       │   ├── <235> Var [arr]
            │       │   │       │   ╰── <236> Constant Int [2]
            │       │   │       ╰── <239> Constant ULong [12345]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <241> Constant Int [11]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <252>  [!=]
            │       │   │       ├── <249> Subscript
            │       │   │       │   ├── <247> Var [arr]
            │       │   │       │   ╰── <248> Constant Int [3]
            │       │   │       ╰── <251> Constant ULong [4294967295]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <253> Constant Int [12]
            │       ╰── Return
            │           ╰── <258> Constant Int [0]
            ├── Function [test_global]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── check
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <272> FunctionCall [check_double_arr]
            │       │           ╰── <271> Var [double_arr]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <276> Var [check]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <278> Var [check]
            │       ├── <290> Assign [=]
            │       │   ├── <284> Var [check]
            │       │   ╰── <289> FunctionCall [check_uint_arr]
            │       │       ╰── <288> Var [uint_arr]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <293> Var [check]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <295> Var [check]
            │       ├── <307> Assign [=]
            │       │   ├── <301> Var [check]
            │       │   ╰── <306> FunctionCall [check_long_arr]
            │       │       ╰── <305> Var [long_arr]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <310> Var [check]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <312> Var [check]
            │       ├── <324> Assign [=]
            │       │   ├── <318> Var [check]
            │       │   ╰── <323> FunctionCall [check_ulong_arr]
            │       │       ╰── <322> Var [ulong_arr]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <327> Var [check]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <329> Var [check]
            │       ╰── Return
            │           ╰── <334> Constant Int [0]
            ├── Function [test_local]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── local_double_arr
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 3
            │       │   │       ╰── Double
            │       │   ╰── Initializer
            │       │       ╰── Compound
            │       │           ├── <347> Constant Double [+1e0]
            │       │           ├── <349> Constant Double [+2e0]
            │       │           ╰── <351> Constant Double [+3e0]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── local_uint_arr
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 5
            │       │   │       ╰── Unsigned Int
            │       │   ├── Initializer
            │       │   │   ╰── Compound
            │       │   │       ├── <361> Constant UInt [1]
            │       │   │       ├── <363> Constant UInt [0]
            │       │   │       ╰── <365> Constant UInt [2147497230]
            │       │   ╰── Static
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── local_long_arr
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 1000
            │       │   │       ╰── Long
            │       │   ╰── Static
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── local_ulong_arr
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 4
            │       │   │       ╰── Unsigned Long
            │       │   ├── Initializer
            │       │   │   ╰── Compound
            │       │   │       ├── <382> Constant Double [+1e2]
            │       │   │       ├── <384> Constant Int [11]
            │       │   │       ├── <386> Constant Long [12345]
            │       │   │       ╰── <388> Constant UInt [4294967295]
            │       │   ╰── Static
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── check
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <398> FunctionCall [check_double_arr]
            │       │           ╰── <397> Var [local_double_arr]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <402> Var [check]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <407>  [+]
            │       │                   ├── <403> Constant Int [100]
            │       │                   ╰── <406> Var [check]
            │       ├── <419> Assign [=]
            │       │   ├── <413> Var [check]
            │       │   ╰── <418> FunctionCall [check_uint_arr]
            │       │       ╰── <417> Var [local_uint_arr]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <422> Var [check]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <427>  [+]
            │       │                   ├── <423> Constant Int [100]
            │       │                   ╰── <426> Var [check]
            │       ├── <439> Assign [=]
            │       │   ├── <433> Var [check]
            │       │   ╰── <438> FunctionCall [check_long_arr]
            │       │       ╰── <437> Var [local_long_arr]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <442> Var [check]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <447>  [+]
            │       │                   ├── <443> Constant Int [100]
            │       │                   ╰── <446> Var [check]
            │       ├── <459> Assign [=]
            │       │   ├── <453> Var [check]
            │       │   ╰── <458> FunctionCall [check_ulong_arr]
            │       │       ╰── <457> Var [local_ulong_arr]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <462> Var [check]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <467>  [+]
            │       │                   ├── <463> Constant Int [100]
            │       │                   ╰── <466> Var [check]
            │       ╰── Return
            │           ╰── <472> Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── check
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <484> FunctionCall [test_global]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <488> Var [check]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <490> Var [check]
                    ╰── Return
                        ╰── <496> FunctionCall [test_local]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_initialization_static_nested() {
    let src = r#"
        double double_arr[2][2] = {{1.1, 2.2}, {3.3, 4.4}};
        int check_double_arr(double (*arr)[2]) {
            if (arr[0][0] != 1.1) {
                return 1;
            }
            if (arr[0][1] != 2.2) {
                return 2;
            }
            if (arr[1][0] != 3.3) {
                return 3;
            }
            if (arr[1][1] != 4.4) {
                return 4;
            }
            return 0;
        }
        long long_arr[30][50][40];
        int check_long_arr(long (*arr)[50][40]) {
            for (int i = 0; i < 30; i = i + 1) {
                for (int j = 0; j < 50; j = j + 1) {
                    for (int k = 0; k < 40; k = k + 1) {
                        if (arr[i][j][k]) {
                            return 5;
                        }
                    }
                }
            }
            return 0;
        }
        unsigned long ulong_arr[4][6][2] = {
            {{
                 1000.3,
             },
             {12u}},
            {{2}}};
        int check_ulong_arr(unsigned long (*arr)[6][2]) {
            for (int i = 0; i < 4; i = i + 1) {
                for (int j = 0; j < 6; j = j + 1) {
                    for (int k = 0; k < 2; k = k + 1) {
                        int val = arr[i][j][k];
                        if (i == 0 && j == 0 && k == 0) {
                            if (val != 1000ul) {
                                return 6;
                            }
                        } else if (i == 0 && j == 1 && k == 0) {
                            if (val != 12ul) {
                                return 7;
                            }
                        } else if (i == 1 && j == 0 && k == 0) {
                            if (val != 2ul) {
                                return 8;
                            }
                        } else {
                            if (val) {
                                return 9;
                            }
                        }
                    }
                }
            }
            return 0;
        }
        int test_global(void) {
            int check = check_double_arr(double_arr);
            if (check) {
                return check;
            }
            check = check_long_arr(long_arr);
            if (check) {
                return check;
            }
            check = check_ulong_arr(ulong_arr);
            if (check) {
                return check;
            }
            return 0;
        }
        int test_local(void) {
            static double local_double_arr[2][2] = {{1.1, 2.2}, {3.3, 4.4}};
            int check = check_double_arr(local_double_arr);
            if (check) {
                return 100 + check;
            }
            static long local_long_arr[30][50][40];
            check = check_long_arr(local_long_arr);
            if (check) {
                return 100 + check;
            }
            static unsigned long local_ulong_arr[4][6][2] = {
                {{
                    1000.3,
                },
                {12u}},
                {{2}}};
            check = check_ulong_arr(local_ulong_arr);
            if (check) {
                return 100 + check;
            }
            return 0;
        }
        int main(void) {
            int check = test_global();
            if (check) {
                return check;
            }
            return test_local();
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── double_arr
            │   ├── Type
            │   │   ╰── Array
            │   │       ├── 2
            │   │       ╰── Array
            │   │           ├── 2
            │   │           ╰── Double
            │   ╰── Initializer
            │       ╰── Compound
            │           ├── Compound
            │           │   ├── <8> Constant Double [+1.1e0]
            │           │   ╰── <10> Constant Double [+2.2e0]
            │           ╰── Compound
            │               ├── <13> Constant Double [+3.3e0]
            │               ╰── <15> Constant Double [+4.4e0]
            ├── Function [check_double_arr]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── arr
            │   │       ╰── Type
            │   │           ╰── Pointer
            │   │               ╰── Array
            │   │                   ├── 2
            │   │                   ╰── Double
            │   ╰── Body
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <39>  [!=]
            │       │   │       ├── <36> Subscript
            │       │   │       │   ├── <34> Subscript
            │       │   │       │   │   ├── <32> Var [arr]
            │       │   │       │   │   ╰── <33> Constant Int [0]
            │       │   │       │   ╰── <35> Constant Int [0]
            │       │   │       ╰── <38> Constant Double [+1.1e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <40> Constant Int [1]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <53>  [!=]
            │       │   │       ├── <50> Subscript
            │       │   │       │   ├── <48> Subscript
            │       │   │       │   │   ├── <46> Var [arr]
            │       │   │       │   │   ╰── <47> Constant Int [0]
            │       │   │       │   ╰── <49> Constant Int [1]
            │       │   │       ╰── <52> Constant Double [+2.2e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <54> Constant Int [2]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <67>  [!=]
            │       │   │       ├── <64> Subscript
            │       │   │       │   ├── <62> Subscript
            │       │   │       │   │   ├── <60> Var [arr]
            │       │   │       │   │   ╰── <61> Constant Int [1]
            │       │   │       │   ╰── <63> Constant Int [0]
            │       │   │       ╰── <66> Constant Double [+3.3e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <68> Constant Int [3]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <81>  [!=]
            │       │   │       ├── <78> Subscript
            │       │   │       │   ├── <76> Subscript
            │       │   │       │   │   ├── <74> Var [arr]
            │       │   │       │   │   ╰── <75> Constant Int [1]
            │       │   │       │   ╰── <77> Constant Int [1]
            │       │   │       ╰── <80> Constant Double [+4.4e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <82> Constant Int [4]
            │       ╰── Return
            │           ╰── <87> Constant Int [0]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── long_arr
            │   ╰── Type
            │       ╰── Array
            │           ├── 30
            │           ╰── Array
            │               ├── 50
            │               ╰── Array
            │                   ├── 40
            │                   ╰── Long
            ├── Function [check_long_arr]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── arr
            │   │       ╰── Type
            │   │           ╰── Pointer
            │   │               ╰── Array
            │   │                   ├── 50
            │   │                   ╰── Array
            │   │                       ├── 40
            │   │                       ╰── Long
            │   ╰── Body
            │       ├── For
            │       │   ├── Init
            │       │   │   ╰── VarDeclaration
            │       │   │       ├── Name
            │       │   │       │   ╰── i
            │       │   │       ├── Type
            │       │   │       │   ╰── Int
            │       │   │       ╰── Initializer
            │       │   │           ╰── <117> Constant Int [0]
            │       │   ├── Condition
            │       │   │   ╰── <125>  [<]
            │       │   │       ├── <122> Var [i]
            │       │   │       ╰── <124> Constant Int [30]
            │       │   ├── Condition
            │       │   │   ╰── <134> Assign [=]
            │       │   │       ├── <127> Var [i]
            │       │   │       ╰── <133>  [+]
            │       │   │           ├── <130> Var [i]
            │       │   │           ╰── <132> Constant Int [1]
            │       │   ╰── Block
            │       │       ╰── For
            │       │           ├── Init
            │       │           │   ╰── VarDeclaration
            │       │           │       ├── Name
            │       │           │       │   ╰── j
            │       │           │       ├── Type
            │       │           │       │   ╰── Int
            │       │           │       ╰── Initializer
            │       │           │           ╰── <138> Constant Int [0]
            │       │           ├── Condition
            │       │           │   ╰── <146>  [<]
            │       │           │       ├── <143> Var [j]
            │       │           │       ╰── <145> Constant Int [50]
            │       │           ├── Condition
            │       │           │   ╰── <155> Assign [=]
            │       │           │       ├── <148> Var [j]
            │       │           │       ╰── <154>  [+]
            │       │           │           ├── <151> Var [j]
            │       │           │           ╰── <153> Constant Int [1]
            │       │           ╰── Block
            │       │               ╰── For
            │       │                   ├── Init
            │       │                   │   ╰── VarDeclaration
            │       │                   │       ├── Name
            │       │                   │       │   ╰── k
            │       │                   │       ├── Type
            │       │                   │       │   ╰── Int
            │       │                   │       ╰── Initializer
            │       │                   │           ╰── <159> Constant Int [0]
            │       │                   ├── Condition
            │       │                   │   ╰── <167>  [<]
            │       │                   │       ├── <164> Var [k]
            │       │                   │       ╰── <166> Constant Int [40]
            │       │                   ├── Condition
            │       │                   │   ╰── <176> Assign [=]
            │       │                   │       ├── <169> Var [k]
            │       │                   │       ╰── <175>  [+]
            │       │                   │           ├── <172> Var [k]
            │       │                   │           ╰── <174> Constant Int [1]
            │       │                   ╰── Block
            │       │                       ╰── If
            │       │                           ├── Condition
            │       │                           │   ╰── <187> Subscript
            │       │                           │       ├── <184> Subscript
            │       │                           │       │   ├── <181> Subscript
            │       │                           │       │   │   ├── <178> Var [arr]
            │       │                           │       │   │   ╰── <180> Var [i]
            │       │                           │       │   ╰── <183> Var [j]
            │       │                           │       ╰── <186> Var [k]
            │       │                           ╰── Then
            │       │                               ╰── Block
            │       │                                   ╰── Return
            │       │                                       ╰── <188> Constant Int [5]
            │       ╰── Return
            │           ╰── <202> Constant Int [0]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── ulong_arr
            │   ├── Type
            │   │   ╰── Array
            │   │       ├── 4
            │   │       ╰── Array
            │   │           ├── 6
            │   │           ╰── Array
            │   │               ├── 2
            │   │               ╰── Unsigned Long
            │   ╰── Initializer
            │       ╰── Compound
            │           ├── Compound
            │           │   ├── Compound
            │           │   │   ╰── <215> Constant Double [+1.0003e3]
            │           │   ╰── Compound
            │           │       ╰── <218> Constant UInt [12]
            │           ╰── Compound
            │               ╰── Compound
            │                   ╰── <222> Constant Int [2]
            ├── Function [check_ulong_arr]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── arr
            │   │       ╰── Type
            │   │           ╰── Pointer
            │   │               ╰── Array
            │   │                   ├── 6
            │   │                   ╰── Array
            │   │                       ├── 2
            │   │                       ╰── Unsigned Long
            │   ╰── Body
            │       ├── For
            │       │   ├── Init
            │       │   │   ╰── VarDeclaration
            │       │   │       ├── Name
            │       │   │       │   ╰── i
            │       │   │       ├── Type
            │       │   │       │   ╰── Int
            │       │   │       ╰── Initializer
            │       │   │           ╰── <244> Constant Int [0]
            │       │   ├── Condition
            │       │   │   ╰── <252>  [<]
            │       │   │       ├── <249> Var [i]
            │       │   │       ╰── <251> Constant Int [4]
            │       │   ├── Condition
            │       │   │   ╰── <261> Assign [=]
            │       │   │       ├── <254> Var [i]
            │       │   │       ╰── <260>  [+]
            │       │   │           ├── <257> Var [i]
            │       │   │           ╰── <259> Constant Int [1]
            │       │   ╰── Block
            │       │       ╰── For
            │       │           ├── Init
            │       │           │   ╰── VarDeclaration
            │       │           │       ├── Name
            │       │           │       │   ╰── j
            │       │           │       ├── Type
            │       │           │       │   ╰── Int
            │       │           │       ╰── Initializer
            │       │           │           ╰── <265> Constant Int [0]
            │       │           ├── Condition
            │       │           │   ╰── <273>  [<]
            │       │           │       ├── <270> Var [j]
            │       │           │       ╰── <272> Constant Int [6]
            │       │           ├── Condition
            │       │           │   ╰── <282> Assign [=]
            │       │           │       ├── <275> Var [j]
            │       │           │       ╰── <281>  [+]
            │       │           │           ├── <278> Var [j]
            │       │           │           ╰── <280> Constant Int [1]
            │       │           ╰── Block
            │       │               ╰── For
            │       │                   ├── Init
            │       │                   │   ╰── VarDeclaration
            │       │                   │       ├── Name
            │       │                   │       │   ╰── k
            │       │                   │       ├── Type
            │       │                   │       │   ╰── Int
            │       │                   │       ╰── Initializer
            │       │                   │           ╰── <286> Constant Int [0]
            │       │                   ├── Condition
            │       │                   │   ╰── <294>  [<]
            │       │                   │       ├── <291> Var [k]
            │       │                   │       ╰── <293> Constant Int [2]
            │       │                   ├── Condition
            │       │                   │   ╰── <303> Assign [=]
            │       │                   │       ├── <296> Var [k]
            │       │                   │       ╰── <302>  [+]
            │       │                   │           ├── <299> Var [k]
            │       │                   │           ╰── <301> Constant Int [1]
            │       │                   ╰── Block
            │       │                       ├── VarDeclaration
            │       │                       │   ├── Name
            │       │                       │   │   ╰── val
            │       │                       │   ├── Type
            │       │                       │   │   ╰── Int
            │       │                       │   ╰── Initializer
            │       │                       │       ╰── <317> Subscript
            │       │                       │           ├── <314> Subscript
            │       │                       │           │   ├── <311> Subscript
            │       │                       │           │   │   ├── <308> Var [arr]
            │       │                       │           │   │   ╰── <310> Var [i]
            │       │                       │           │   ╰── <313> Var [j]
            │       │                       │           ╰── <316> Var [k]
            │       │                       ╰── If
            │       │                           ├── Condition
            │       │                           │   ╰── <338>  [&&]
            │       │                           │       ├── <331>  [&&]
            │       │                           │       │   ├── <324>  [==]
            │       │                           │       │   │   ├── <321> Var [i]
            │       │                           │       │   │   ╰── <323> Constant Int [0]
            │       │                           │       │   ╰── <330>  [==]
            │       │                           │       │       ├── <327> Var [j]
            │       │                           │       │       ╰── <329> Constant Int [0]
            │       │                           │       ╰── <337>  [==]
            │       │                           │           ├── <334> Var [k]
            │       │                           │           ╰── <336> Constant Int [0]
            │       │                           ├── Then
            │       │                           │   ╰── Block
            │       │                           │       ╰── If
            │       │                           │           ├── Condition
            │       │                           │           │   ╰── <343>  [!=]
            │       │                           │           │       ├── <340> Var [val]
            │       │                           │           │       ╰── <342> Constant ULong [1000]
            │       │                           │           ╰── Then
            │       │                           │               ╰── Block
            │       │                           │                   ╰── Return
            │       │                           │                       ╰── <344> Constant Int [6]
            │       │                           ╰── Else
            │       │                               ╰── If
            │       │                                   ├── Condition
            │       │                                   │   ╰── <369>  [&&]
            │       │                                   │       ├── <362>  [&&]
            │       │                                   │       │   ├── <355>  [==]
            │       │                                   │       │   │   ├── <352> Var [i]
            │       │                                   │       │   │   ╰── <354> Constant Int [0]
            │       │                                   │       │   ╰── <361>  [==]
            │       │                                   │       │       ├── <358> Var [j]
            │       │                                   │       │       ╰── <360> Constant Int [1]
            │       │                                   │       ╰── <368>  [==]
            │       │                                   │           ├── <365> Var [k]
            │       │                                   │           ╰── <367> Constant Int [0]
            │       │                                   ├── Then
            │       │                                   │   ╰── Block
            │       │                                   │       ╰── If
            │       │                                   │           ├── Condition
            │       │                                   │           │   ╰── <374>  [!=]
            │       │                                   │           │       ├── <371> Var [val]
            │       │                                   │           │       ╰── <373> Constant ULong [12]
            │       │                                   │           ╰── Then
            │       │                                   │               ╰── Block
            │       │                                   │                   ╰── Return
            │       │                                   │                       ╰── <375> Constant Int [7]
            │       │                                   ╰── Else
            │       │                                       ╰── If
            │       │                                           ├── Condition
            │       │                                           │   ╰── <400>  [&&]
            │       │                                           │       ├── <393>  [&&]
            │       │                                           │       │   ├── <386>  [==]
            │       │                                           │       │   │   ├── <383> Var [i]
            │       │                                           │       │   │   ╰── <385> Constant Int [1]
            │       │                                           │       │   ╰── <392>  [==]
            │       │                                           │       │       ├── <389> Var [j]
            │       │                                           │       │       ╰── <391> Constant Int [0]
            │       │                                           │       ╰── <399>  [==]
            │       │                                           │           ├── <396> Var [k]
            │       │                                           │           ╰── <398> Constant Int [0]
            │       │                                           ├── Then
            │       │                                           │   ╰── Block
            │       │                                           │       ╰── If
            │       │                                           │           ├── Condition
            │       │                                           │           │   ╰── <405>  [!=]
            │       │                                           │           │       ├── <402> Var [val]
            │       │                                           │           │       ╰── <404> Constant ULong [2]
            │       │                                           │           ╰── Then
            │       │                                           │               ╰── Block
            │       │                                           │                   ╰── Return
            │       │                                           │                       ╰── <406> Constant Int [8]
            │       │                                           ╰── Else
            │       │                                               ╰── Block
            │       │                                                   ╰── If
            │       │                                                       ├── Condition
            │       │                                                       │   ╰── <414> Var [val]
            │       │                                                       ╰── Then
            │       │                                                           ╰── Block
            │       │                                                               ╰── Return
            │       │                                                                   ╰── <415> Constant Int [9]
            │       ╰── Return
            │           ╰── <434> Constant Int [0]
            ├── Function [test_global]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── check
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <448> FunctionCall [check_double_arr]
            │       │           ╰── <447> Var [double_arr]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <452> Var [check]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <454> Var [check]
            │       ├── <466> Assign [=]
            │       │   ├── <460> Var [check]
            │       │   ╰── <465> FunctionCall [check_long_arr]
            │       │       ╰── <464> Var [long_arr]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <469> Var [check]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <471> Var [check]
            │       ├── <483> Assign [=]
            │       │   ├── <477> Var [check]
            │       │   ╰── <482> FunctionCall [check_ulong_arr]
            │       │       ╰── <481> Var [ulong_arr]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <486> Var [check]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <488> Var [check]
            │       ╰── Return
            │           ╰── <493> Constant Int [0]
            ├── Function [test_local]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── local_double_arr
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 2
            │       │   │       ╰── Array
            │       │   │           ├── 2
            │       │   │           ╰── Double
            │       │   ├── Initializer
            │       │   │   ╰── Compound
            │       │   │       ├── Compound
            │       │   │       │   ├── <509> Constant Double [+1.1e0]
            │       │   │       │   ╰── <511> Constant Double [+2.2e0]
            │       │   │       ╰── Compound
            │       │   │           ├── <514> Constant Double [+3.3e0]
            │       │   │           ╰── <516> Constant Double [+4.4e0]
            │       │   ╰── Static
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── check
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <527> FunctionCall [check_double_arr]
            │       │           ╰── <526> Var [local_double_arr]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <531> Var [check]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <536>  [+]
            │       │                   ├── <532> Constant Int [100]
            │       │                   ╰── <535> Var [check]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── local_long_arr
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 30
            │       │   │       ╰── Array
            │       │   │           ├── 50
            │       │   │           ╰── Array
            │       │   │               ├── 40
            │       │   │               ╰── Long
            │       │   ╰── Static
            │       ├── <559> Assign [=]
            │       │   ├── <553> Var [check]
            │       │   ╰── <558> FunctionCall [check_long_arr]
            │       │       ╰── <557> Var [local_long_arr]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <562> Var [check]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <567>  [+]
            │       │                   ├── <563> Constant Int [100]
            │       │                   ╰── <566> Var [check]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── local_ulong_arr
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 4
            │       │   │       ╰── Array
            │       │   │           ├── 6
            │       │   │           ╰── Array
            │       │   │               ├── 2
            │       │   │               ╰── Unsigned Long
            │       │   ├── Initializer
            │       │   │   ╰── Compound
            │       │   │       ├── Compound
            │       │   │       │   ├── Compound
            │       │   │       │   │   ╰── <582> Constant Double [+1.0003e3]
            │       │   │       │   ╰── Compound
            │       │   │       │       ╰── <585> Constant UInt [12]
            │       │   │       ╰── Compound
            │       │   │           ╰── Compound
            │       │   │               ╰── <589> Constant Int [2]
            │       │   ╰── Static
            │       ├── <602> Assign [=]
            │       │   ├── <596> Var [check]
            │       │   ╰── <601> FunctionCall [check_ulong_arr]
            │       │       ╰── <600> Var [local_ulong_arr]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <605> Var [check]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <610>  [+]
            │       │                   ├── <606> Constant Int [100]
            │       │                   ╰── <609> Var [check]
            │       ╰── Return
            │           ╰── <615> Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── check
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <627> FunctionCall [test_global]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <631> Var [check]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <633> Var [check]
                    ╰── Return
                        ╰── <639> FunctionCall [test_local]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_initialization_trailing_comma_initializer() {
    let src = r#"
        int foo(int a, int b, int c);
        int main(void) {
            int arr[3] = {
                1,
                2,
                3,
            };
            return arr[2];
        }
    "#;
    let expected = r#"
        Program
            ├── Function [foo]
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
            │       ╰── Param
            │           ├── Name
            │           │   ╰── c
            │           ╰── Type
            │               ╰── Int
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
                    │           ├── <24> Constant Int [1]
                    │           ├── <26> Constant Int [2]
                    │           ╰── <28> Constant Int [3]
                    ╰── Return
                        ╰── <35> Subscript
                            ├── <33> Var [arr]
                            ╰── <34> Constant Int [2]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_global_array() {
    let src = r#"
        long arr[4] = {1, 2, 3, 4};
        int double_each_element(void) {
            for (int i = 0; i < 4; i = i + 1) {
                arr[i] = arr[i] * 2;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── arr
            │   ├── Type
            │   │   ╰── Array
            │   │       ├── 4
            │   │       ╰── Long
            │   ╰── Initializer
            │       ╰── Compound
            │           ├── <6> Constant Int [1]
            │           ├── <8> Constant Int [2]
            │           ├── <10> Constant Int [3]
            │           ╰── <12> Constant Int [4]
            ╰── Function [double_each_element]
                ╰── Body
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <23> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <31>  [<]
                    │   │       ├── <28> Var [i]
                    │   │       ╰── <30> Constant Int [4]
                    │   ├── Condition
                    │   │   ╰── <40> Assign [=]
                    │   │       ├── <33> Var [i]
                    │   │       ╰── <39>  [+]
                    │   │           ├── <36> Var [i]
                    │   │           ╰── <38> Constant Int [1]
                    │   ╰── Block
                    │       ╰── <55> Assign [=]
                    │           ├── <45> Subscript
                    │           │   ├── <42> Var [arr]
                    │           │   ╰── <44> Var [i]
                    │           ╰── <54>  [*]
                    │               ├── <51> Subscript
                    │               │   ├── <48> Var [arr]
                    │               │   ╰── <50> Var [i]
                    │               ╰── <53> Constant Int [2]
                    ╰── Return
                        ╰── <60> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_global_array_client() {
    let src = r#"
        
        extern long arr[4];
        int double_each_element(void);
        int main(void) {
            for (int i = 0; i < 4; i = i + 1) {
                if (arr[i] != i + 1) {
                    return i + 1;
                }
            }
            double_each_element();
            for (int i = 0; i < 4; i = i + 1) {
                if (arr[i] != (i + 1) * 2) {
                    return i + 5;
                }
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── arr
            │   ├── Type
            │   │   ╰── Array
            │   │       ├── 4
            │   │       ╰── Long
            │   ╰── Extern
            ├── Function [double_each_element]
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
                    │   │           ╰── <20> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <28>  [<]
                    │   │       ├── <25> Var [i]
                    │   │       ╰── <27> Constant Int [4]
                    │   ├── Condition
                    │   │   ╰── <37> Assign [=]
                    │   │       ├── <30> Var [i]
                    │   │       ╰── <36>  [+]
                    │   │           ├── <33> Var [i]
                    │   │           ╰── <35> Constant Int [1]
                    │   ╰── Block
                    │       ╰── If
                    │           ├── Condition
                    │           │   ╰── <49>  [!=]
                    │           │       ├── <42> Subscript
                    │           │       │   ├── <39> Var [arr]
                    │           │       │   ╰── <41> Var [i]
                    │           │       ╰── <48>  [+]
                    │           │           ├── <45> Var [i]
                    │           │           ╰── <47> Constant Int [1]
                    │           ╰── Then
                    │               ╰── Block
                    │                   ╰── Return
                    │                       ╰── <54>  [+]
                    │                           ├── <51> Var [i]
                    │                           ╰── <53> Constant Int [1]
                    ├── <63> FunctionCall [double_each_element]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <68> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <76>  [<]
                    │   │       ├── <73> Var [i]
                    │   │       ╰── <75> Constant Int [4]
                    │   ├── Condition
                    │   │   ╰── <85> Assign [=]
                    │   │       ├── <78> Var [i]
                    │   │       ╰── <84>  [+]
                    │   │           ├── <81> Var [i]
                    │   │           ╰── <83> Constant Int [1]
                    │   ╰── Block
                    │       ╰── If
                    │           ├── Condition
                    │           │   ╰── <101>  [!=]
                    │           │       ├── <90> Subscript
                    │           │       │   ├── <87> Var [arr]
                    │           │       │   ╰── <89> Var [i]
                    │           │       ╰── <100>  [*]
                    │           │           ├── <97>  [+]
                    │           │           │   ├── <93> Var [i]
                    │           │           │   ╰── <95> Constant Int [1]
                    │           │           ╰── <99> Constant Int [2]
                    │           ╰── Then
                    │               ╰── Block
                    │                   ╰── Return
                    │                       ╰── <106>  [+]
                    │                           ├── <103> Var [i]
                    │                           ╰── <105> Constant Int [5]
                    ╰── Return
                        ╰── <114> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_return_pointer_to_array() {
    let src = r#"
        
        long (*return_row(long (*arr)[3][4], int idx))[4] {
            return arr[idx];
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [return_row]
                ├── Parameters
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── arr
                │   │   ╰── Type
                │   │       ╰── Pointer
                │   │           ╰── Array
                │   │               ├── 3
                │   │               ╰── Array
                │   │                   ├── 4
                │   │                   ╰── Long
                │   ╰── Param
                │       ├── Name
                │       │   ╰── idx
                │       ╰── Type
                │           ╰── Int
                ╰── Body
                    ╰── Return
                        ╰── <25> Subscript
                            ├── <22> Var [arr]
                            ╰── <24> Var [idx]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_return_pointer_to_array_client() {
    let src = r#"
        
        long (*return_row(long (*arr)[3][4], int idx))[4];
        int main(void) {
            long nested_array[2][3][4] = {
                {{0}},
                {{-12, -13, -14, -15}, {-16}}
            };
            long (*row_pointer)[4] = return_row(nested_array, 1);
            for (int i = 0; i < 3; i = i + 1) {
                for (int j = 0; j < 4; j = j + 1) {
                    if (row_pointer[i][j] != nested_array[1][i][j]) {
                        return 1;
                    }
                }
            }
            row_pointer[2][1] = 100;
            if (nested_array[1][2][1] != 100) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [return_row]
            │   ╰── Parameters
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── arr
            │       │   ╰── Type
            │       │       ╰── Pointer
            │       │           ╰── Array
            │       │               ├── 3
            │       │               ╰── Array
            │       │                   ├── 4
            │       │                   ╰── Long
            │       ╰── Param
            │           ├── Name
            │           │   ╰── idx
            │           ╰── Type
            │               ╰── Int
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── nested_array
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 2
                    │   │       ╰── Array
                    │   │           ├── 3
                    │   │           ╰── Array
                    │   │               ├── 4
                    │   │               ╰── Long
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── Compound
                    │           │   ╰── Compound
                    │           │       ╰── <35> Constant Int [0]
                    │           ╰── Compound
                    │               ├── Compound
                    │               │   ├── <41> Unary [-]
                    │               │   │   ╰── <40> Constant Int [12]
                    │               │   ├── <45> Unary [-]
                    │               │   │   ╰── <44> Constant Int [13]
                    │               │   ├── <49> Unary [-]
                    │               │   │   ╰── <48> Constant Int [14]
                    │               │   ╰── <53> Unary [-]
                    │               │       ╰── <52> Constant Int [15]
                    │               ╰── Compound
                    │                   ╰── <58> Unary [-]
                    │                       ╰── <57> Constant Int [16]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── row_pointer
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Array
                    │   │           ├── 4
                    │   │           ╰── Long
                    │   ╰── Initializer
                    │       ╰── <75> FunctionCall [return_row]
                    │           ├── <73> Var [nested_array]
                    │           ╰── <74> Constant Int [1]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <81> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <89>  [<]
                    │   │       ├── <86> Var [i]
                    │   │       ╰── <88> Constant Int [3]
                    │   ├── Condition
                    │   │   ╰── <98> Assign [=]
                    │   │       ├── <91> Var [i]
                    │   │       ╰── <97>  [+]
                    │   │           ├── <94> Var [i]
                    │   │           ╰── <96> Constant Int [1]
                    │   ╰── Block
                    │       ╰── For
                    │           ├── Init
                    │           │   ╰── VarDeclaration
                    │           │       ├── Name
                    │           │       │   ╰── j
                    │           │       ├── Type
                    │           │       │   ╰── Int
                    │           │       ╰── Initializer
                    │           │           ╰── <102> Constant Int [0]
                    │           ├── Condition
                    │           │   ╰── <110>  [<]
                    │           │       ├── <107> Var [j]
                    │           │       ╰── <109> Constant Int [4]
                    │           ├── Condition
                    │           │   ╰── <119> Assign [=]
                    │           │       ├── <112> Var [j]
                    │           │       ╰── <118>  [+]
                    │           │           ├── <115> Var [j]
                    │           │           ╰── <117> Constant Int [1]
                    │           ╰── Block
                    │               ╰── If
                    │                   ├── Condition
                    │                   │   ╰── <139>  [!=]
                    │                   │       ├── <127> Subscript
                    │                   │       │   ├── <124> Subscript
                    │                   │       │   │   ├── <121> Var [row_pointer]
                    │                   │       │   │   ╰── <123> Var [i]
                    │                   │       │   ╰── <126> Var [j]
                    │                   │       ╰── <138> Subscript
                    │                   │           ├── <135> Subscript
                    │                   │           │   ├── <132> Subscript
                    │                   │           │   │   ├── <130> Var [nested_array]
                    │                   │           │   │   ╰── <131> Constant Int [1]
                    │                   │           │   ╰── <134> Var [i]
                    │                   │           ╰── <137> Var [j]
                    │                   ╰── Then
                    │                       ╰── Block
                    │                           ╰── Return
                    │                               ╰── <140> Constant Int [1]
                    ├── <159> Assign [=]
                    │   ├── <156> Subscript
                    │   │   ├── <154> Subscript
                    │   │   │   ├── <152> Var [row_pointer]
                    │   │   │   ╰── <153> Constant Int [2]
                    │   │   ╰── <155> Constant Int [1]
                    │   ╰── <158> Constant Int [100]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <171>  [!=]
                    │   │       ├── <168> Subscript
                    │   │       │   ├── <166> Subscript
                    │   │       │   │   ├── <164> Subscript
                    │   │       │   │   │   ├── <162> Var [nested_array]
                    │   │       │   │   │   ╰── <163> Constant Int [1]
                    │   │       │   │   ╰── <165> Constant Int [2]
                    │   │       │   ╰── <167> Constant Int [1]
                    │   │       ╰── <170> Constant Int [100]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <172> Constant Int [2]
                    ╰── Return
                        ╰── <177> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_set_array_val() {
    let src = r#"
        int set_nth_element(double *arr, int idx) {
            for (int i = 0; i < 5; i = i + 1) {
                if (arr[i]) {
                    return 1;
                }
            }
            arr[idx] = 8;
            return 0;
        }
        int set_nested_element(int (*arr)[2], int i, int j) {
            for (int x = 0; x < 3; x = x + 1) {
                for (int y = 0; y < 2; y = y + 1) {
                    int expected = -10 + 2*x + y;
                    if (arr[x][y] != expected) {
                        return 4;
                    }
                }
            }
            arr[i][j] = 10;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [set_nth_element]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── arr
            │   │   │   ╰── Type
            │   │   │       ╰── Pointer
            │   │   │           ╰── Double
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── idx
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ├── For
            │       │   ├── Init
            │       │   │   ╰── VarDeclaration
            │       │   │       ├── Name
            │       │   │       │   ╰── i
            │       │   │       ├── Type
            │       │   │       │   ╰── Int
            │       │   │       ╰── Initializer
            │       │   │           ╰── <15> Constant Int [0]
            │       │   ├── Condition
            │       │   │   ╰── <23>  [<]
            │       │   │       ├── <20> Var [i]
            │       │   │       ╰── <22> Constant Int [5]
            │       │   ├── Condition
            │       │   │   ╰── <32> Assign [=]
            │       │   │       ├── <25> Var [i]
            │       │   │       ╰── <31>  [+]
            │       │   │           ├── <28> Var [i]
            │       │   │           ╰── <30> Constant Int [1]
            │       │   ╰── Block
            │       │       ╰── If
            │       │           ├── Condition
            │       │           │   ╰── <37> Subscript
            │       │           │       ├── <34> Var [arr]
            │       │           │       ╰── <36> Var [i]
            │       │           ╰── Then
            │       │               ╰── Block
            │       │                   ╰── Return
            │       │                       ╰── <38> Constant Int [1]
            │       ├── <53> Assign [=]
            │       │   ├── <50> Subscript
            │       │   │   ├── <47> Var [arr]
            │       │   │   ╰── <49> Var [idx]
            │       │   ╰── <52> Constant Int [8]
            │       ╰── Return
            │           ╰── <55> Constant Int [0]
            ╰── Function [set_nested_element]
                ├── Parameters
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── arr
                │   │   ╰── Type
                │   │       ╰── Pointer
                │   │           ╰── Array
                │   │               ├── 2
                │   │               ╰── Int
                │   ├── Param
                │   │   ├── Name
                │   │   │   ╰── i
                │   │   ╰── Type
                │   │       ╰── Int
                │   ╰── Param
                │       ├── Name
                │       │   ╰── j
                │       ╰── Type
                │           ╰── Int
                ╰── Body
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── x
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <79> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <87>  [<]
                    │   │       ├── <84> Var [x]
                    │   │       ╰── <86> Constant Int [3]
                    │   ├── Condition
                    │   │   ╰── <96> Assign [=]
                    │   │       ├── <89> Var [x]
                    │   │       ╰── <95>  [+]
                    │   │           ├── <92> Var [x]
                    │   │           ╰── <94> Constant Int [1]
                    │   ╰── Block
                    │       ╰── For
                    │           ├── Init
                    │           │   ╰── VarDeclaration
                    │           │       ├── Name
                    │           │       │   ╰── y
                    │           │       ├── Type
                    │           │       │   ╰── Int
                    │           │       ╰── Initializer
                    │           │           ╰── <100> Constant Int [0]
                    │           ├── Condition
                    │           │   ╰── <108>  [<]
                    │           │       ├── <105> Var [y]
                    │           │       ╰── <107> Constant Int [2]
                    │           ├── Condition
                    │           │   ╰── <117> Assign [=]
                    │           │       ├── <110> Var [y]
                    │           │       ╰── <116>  [+]
                    │           │           ├── <113> Var [y]
                    │           │           ╰── <115> Constant Int [1]
                    │           ╰── Block
                    │               ├── VarDeclaration
                    │               │   ├── Name
                    │               │   │   ╰── expected
                    │               │   ├── Type
                    │               │   │   ╰── Int
                    │               │   ╰── Initializer
                    │               │       ╰── <134>  [+]
                    │               │           ├── <130>  [+]
                    │               │           │   ├── <123> Unary [-]
                    │               │           │   │   ╰── <122> Constant Int [10]
                    │               │           │   ╰── <129>  [*]
                    │               │           │       ├── <125> Constant Int [2]
                    │               │           │       ╰── <128> Var [x]
                    │               │           ╰── <133> Var [y]
                    │               ╰── If
                    │                   ├── Condition
                    │                   │   ╰── <148>  [!=]
                    │                   │       ├── <144> Subscript
                    │                   │       │   ├── <141> Subscript
                    │                   │       │   │   ├── <138> Var [arr]
                    │                   │       │   │   ╰── <140> Var [x]
                    │                   │       │   ╰── <143> Var [y]
                    │                   │       ╰── <147> Var [expected]
                    │                   ╰── Then
                    │                       ╰── Block
                    │                           ╰── Return
                    │                               ╰── <149> Constant Int [4]
                    ├── <170> Assign [=]
                    │   ├── <167> Subscript
                    │   │   ├── <164> Subscript
                    │   │   │   ├── <161> Var [arr]
                    │   │   │   ╰── <163> Var [i]
                    │   │   ╰── <166> Var [j]
                    │   ╰── <169> Constant Int [10]
                    ╰── Return
                        ╰── <172> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_libraries_set_array_val_client() {
    let src = r#"
        int set_nth_element(double *arr, int idx);
        int set_nested_element(int (*arr)[2], int i, int j);
        int main(void) {
            double arr[5] = {0.0, 0.0, 0.0, 0.0, 0.0};
            int check = set_nth_element(arr, 4);
            if (check) {
                return check;
            }
            for (int i = 0; i < 4; i = i + 1) {
                if (arr[i] != 0) {
                    return 2;
                }
            }
            if (arr[4] != 8)
                return 3;
            int nested_arr[3][2] = {{-10, -9}, {-8, -7}, {-6, -5}};
            check = set_nested_element(nested_arr, 2, 1);
            if (check) {
                return check;
            }
            for (int i = 0; i < 3; i = i + 1) {
                for (int j = 0; j < 2; j = j + 1) {
                    if (i == 2 && j == 1) {
                        if (nested_arr[i][j] != 10) {
                            return 5;
                        }
                    } else {
                        int expected = -10 + 2 * i + j;
                        if (nested_arr[i][j] != expected) {
                            return 6;
                        }
                    }
                }
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [set_nth_element]
            │   ╰── Parameters
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── arr
            │       │   ╰── Type
            │       │       ╰── Pointer
            │       │           ╰── Double
            │       ╰── Param
            │           ├── Name
            │           │   ╰── idx
            │           ╰── Type
            │               ╰── Int
            ├── Function [set_nested_element]
            │   ╰── Parameters
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── arr
            │       │   ╰── Type
            │       │       ╰── Pointer
            │       │           ╰── Array
            │       │               ├── 2
            │       │               ╰── Int
            │       ├── Param
            │       │   ├── Name
            │       │   │   ╰── i
            │       │   ╰── Type
            │       │       ╰── Int
            │       ╰── Param
            │           ├── Name
            │           │   ╰── j
            │           ╰── Type
            │               ╰── Int
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── arr
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 5
                    │   │       ╰── Double
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <40> Constant Double [+0e0]
                    │           ├── <42> Constant Double [+0e0]
                    │           ├── <44> Constant Double [+0e0]
                    │           ├── <46> Constant Double [+0e0]
                    │           ╰── <48> Constant Double [+0e0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── check
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <59> FunctionCall [set_nth_element]
                    │           ├── <57> Var [arr]
                    │           ╰── <58> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <63> Var [check]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <65> Var [check]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <73> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <81>  [<]
                    │   │       ├── <78> Var [i]
                    │   │       ╰── <80> Constant Int [4]
                    │   ├── Condition
                    │   │   ╰── <90> Assign [=]
                    │   │       ├── <83> Var [i]
                    │   │       ╰── <89>  [+]
                    │   │           ├── <86> Var [i]
                    │   │           ╰── <88> Constant Int [1]
                    │   ╰── Block
                    │       ╰── If
                    │           ├── Condition
                    │           │   ╰── <98>  [!=]
                    │           │       ├── <95> Subscript
                    │           │       │   ├── <92> Var [arr]
                    │           │       │   ╰── <94> Var [i]
                    │           │       ╰── <97> Constant Int [0]
                    │           ╰── Then
                    │               ╰── Block
                    │                   ╰── Return
                    │                       ╰── <99> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <113>  [!=]
                    │   │       ├── <110> Subscript
                    │   │       │   ├── <108> Var [arr]
                    │   │       │   ╰── <109> Constant Int [4]
                    │   │       ╰── <112> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <114> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── nested_arr
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 3
                    │   │       ╰── Array
                    │   │           ├── 2
                    │   │           ╰── Int
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── Compound
                    │           │   ├── <126> Unary [-]
                    │           │   │   ╰── <125> Constant Int [10]
                    │           │   ╰── <130> Unary [-]
                    │           │       ╰── <129> Constant Int [9]
                    │           ├── Compound
                    │           │   ├── <135> Unary [-]
                    │           │   │   ╰── <134> Constant Int [8]
                    │           │   ╰── <139> Unary [-]
                    │           │       ╰── <138> Constant Int [7]
                    │           ╰── Compound
                    │               ├── <144> Unary [-]
                    │               │   ╰── <143> Constant Int [6]
                    │               ╰── <148> Unary [-]
                    │                   ╰── <147> Constant Int [5]
                    ├── <162> Assign [=]
                    │   ├── <154> Var [check]
                    │   ╰── <161> FunctionCall [set_nested_element]
                    │       ├── <158> Var [nested_arr]
                    │       ├── <159> Constant Int [2]
                    │       ╰── <160> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <165> Var [check]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <167> Var [check]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <175> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <183>  [<]
                    │   │       ├── <180> Var [i]
                    │   │       ╰── <182> Constant Int [3]
                    │   ├── Condition
                    │   │   ╰── <192> Assign [=]
                    │   │       ├── <185> Var [i]
                    │   │       ╰── <191>  [+]
                    │   │           ├── <188> Var [i]
                    │   │           ╰── <190> Constant Int [1]
                    │   ╰── Block
                    │       ╰── For
                    │           ├── Init
                    │           │   ╰── VarDeclaration
                    │           │       ├── Name
                    │           │       │   ╰── j
                    │           │       ├── Type
                    │           │       │   ╰── Int
                    │           │       ╰── Initializer
                    │           │           ╰── <196> Constant Int [0]
                    │           ├── Condition
                    │           │   ╰── <204>  [<]
                    │           │       ├── <201> Var [j]
                    │           │       ╰── <203> Constant Int [2]
                    │           ├── Condition
                    │           │   ╰── <213> Assign [=]
                    │           │       ├── <206> Var [j]
                    │           │       ╰── <212>  [+]
                    │           │           ├── <209> Var [j]
                    │           │           ╰── <211> Constant Int [1]
                    │           ╰── Block
                    │               ╰── If
                    │                   ├── Condition
                    │                   │   ╰── <225>  [&&]
                    │                   │       ├── <218>  [==]
                    │                   │       │   ├── <215> Var [i]
                    │                   │       │   ╰── <217> Constant Int [2]
                    │                   │       ╰── <224>  [==]
                    │                   │           ├── <221> Var [j]
                    │                   │           ╰── <223> Constant Int [1]
                    │                   ├── Then
                    │                   │   ╰── Block
                    │                   │       ╰── If
                    │                   │           ├── Condition
                    │                   │           │   ╰── <236>  [!=]
                    │                   │           │       ├── <233> Subscript
                    │                   │           │       │   ├── <230> Subscript
                    │                   │           │       │   │   ├── <227> Var [nested_arr]
                    │                   │           │       │   │   ╰── <229> Var [i]
                    │                   │           │       │   ╰── <232> Var [j]
                    │                   │           │       ╰── <235> Constant Int [10]
                    │                   │           ╰── Then
                    │                   │               ╰── Block
                    │                   │                   ╰── Return
                    │                   │                       ╰── <237> Constant Int [5]
                    │                   ╰── Else
                    │                       ╰── Block
                    │                           ├── VarDeclaration
                    │                           │   ├── Name
                    │                           │   │   ╰── expected
                    │                           │   ├── Type
                    │                           │   │   ╰── Int
                    │                           │   ╰── Initializer
                    │                           │       ╰── <260>  [+]
                    │                           │           ├── <256>  [+]
                    │                           │           │   ├── <249> Unary [-]
                    │                           │           │   │   ╰── <248> Constant Int [10]
                    │                           │           │   ╰── <255>  [*]
                    │                           │           │       ├── <251> Constant Int [2]
                    │                           │           │       ╰── <254> Var [i]
                    │                           │           ╰── <259> Var [j]
                    │                           ╰── If
                    │                               ├── Condition
                    │                               │   ╰── <274>  [!=]
                    │                               │       ├── <270> Subscript
                    │                               │       │   ├── <267> Subscript
                    │                               │       │   │   ├── <264> Var [nested_arr]
                    │                               │       │   │   ╰── <266> Var [i]
                    │                               │       │   ╰── <269> Var [j]
                    │                               │       ╰── <273> Var [expected]
                    │                               ╰── Then
                    │                                   ╰── Block
                    │                                       ╰── Return
                    │                                           ╰── <275> Constant Int [6]
                    ╰── Return
                        ╰── <289> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_pointer_arithmetic_add_dereference_and_assign() {
    let src = r#"
        int main(void) {
            int arr[2] = {1, 2};
            *arr = 3;
            *(arr + 1) = 4;
            if (arr[0] != 3) {
                return 1;
            }
            if (arr[1] != 4) {
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
                    │   │   ╰── arr
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 2
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <10> Constant Int [1]
                    │           ╰── <12> Constant Int [2]
                    ├── <21> Assign [=]
                    │   ├── <18> Dereference
                    │   │   ╰── <17> Var [arr]
                    │   ╰── <20> Constant Int [3]
                    ├── <32> Assign [=]
                    │   ├── <29> Dereference
                    │   │   ╰── <28>  [+]
                    │   │       ├── <24> Var [arr]
                    │   │       ╰── <26> Constant Int [1]
                    │   ╰── <31> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <40>  [!=]
                    │   │       ├── <37> Subscript
                    │   │       │   ├── <35> Var [arr]
                    │   │       │   ╰── <36> Constant Int [0]
                    │   │       ╰── <39> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <41> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <52>  [!=]
                    │   │       ├── <49> Subscript
                    │   │       │   ├── <47> Var [arr]
                    │   │       │   ╰── <48> Constant Int [1]
                    │   │       ╰── <51> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <53> Constant Int [2]
                    ╰── Return
                        ╰── <58> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_pointer_arithmetic_compare() {
    let src = r#"
        unsigned long gt(unsigned long *a, unsigned long *b) {
            return a > b;
        }
        unsigned long lt(unsigned long *a, unsigned long *b) {
            return a < b;
        }
        unsigned long ge(unsigned long *a, unsigned long *b) {
            return a >= b;
        }
        unsigned long le(unsigned long *a, unsigned long *b) {
            return a <= b;
        }
        unsigned long gt_nested(unsigned long (*a)[5], unsigned long (*b)[5]) {
            return a > b;
        }
        unsigned long ge_nested(unsigned long (*a)[5], unsigned long (*b)[5]) {
            return a >= b;
        }
        int main(void)
        {
            unsigned long arr[5];
            unsigned long *elem_1 = arr + 1;
            unsigned long *elem_4 = arr + 4;
            if (gt(elem_1, elem_4)) {
                return 1;
            }
            if (!(lt(elem_1, elem_4))) {
                return 2;
            }
            if (!(ge(elem_1, elem_1))) {
                return 3;
            }
            if (le(elem_4, elem_1)) {
                return 4;
            }
            unsigned long *one_past_the_end = arr + 5;
            if (!(gt(one_past_the_end, elem_4))) {
                return 5;
            }
            if (one_past_the_end != elem_4 + 1) {
                return 6;
            }
            unsigned long nested_arr[4][5];
            unsigned long *elem_3_2 = *(nested_arr + 3) + 2;
            unsigned long *elem_3_3 = *(nested_arr + 3) + 3;
            if (lt(elem_3_3, elem_3_2)) {
                return 7;
            }
            if (!ge(elem_3_3, elem_3_2)) {
                return 8;
            }
            unsigned long (*subarray_0)[5] = nested_arr;
            unsigned long (*subarray_3)[5] = nested_arr + 3;
            unsigned long (*subarray_one_past_the_end)[5] = nested_arr + 4;
            if (ge_nested(subarray_0, subarray_3)){
                return 9;
            }
            if (!(gt_nested(subarray_one_past_the_end, subarray_3))) {
                return 10;
            }
            if (subarray_3 != subarray_one_past_the_end - 1) {
                return 11;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [gt]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── a
            │   │   │   ╰── Type
            │   │   │       ╰── Pointer
            │   │   │           ╰── Unsigned Long
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── b
            │   │       ╰── Type
            │   │           ╰── Pointer
            │   │               ╰── Unsigned Long
            │   ╰── Body
            │       ╰── Return
            │           ╰── <18>  [>]
            │               ├── <14> Var [a]
            │               ╰── <17> Var [b]
            ├── Function [lt]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── a
            │   │   │   ╰── Type
            │   │   │       ╰── Pointer
            │   │   │           ╰── Unsigned Long
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── b
            │   │       ╰── Type
            │   │           ╰── Pointer
            │   │               ╰── Unsigned Long
            │   ╰── Body
            │       ╰── Return
            │           ╰── <39>  [<]
            │               ├── <35> Var [a]
            │               ╰── <38> Var [b]
            ├── Function [ge]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── a
            │   │   │   ╰── Type
            │   │   │       ╰── Pointer
            │   │   │           ╰── Unsigned Long
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── b
            │   │       ╰── Type
            │   │           ╰── Pointer
            │   │               ╰── Unsigned Long
            │   ╰── Body
            │       ╰── Return
            │           ╰── <60>  [>=]
            │               ├── <56> Var [a]
            │               ╰── <59> Var [b]
            ├── Function [le]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── a
            │   │   │   ╰── Type
            │   │   │       ╰── Pointer
            │   │   │           ╰── Unsigned Long
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── b
            │   │       ╰── Type
            │   │           ╰── Pointer
            │   │               ╰── Unsigned Long
            │   ╰── Body
            │       ╰── Return
            │           ╰── <81>  [<=]
            │               ├── <77> Var [a]
            │               ╰── <80> Var [b]
            ├── Function [gt_nested]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── a
            │   │   │   ╰── Type
            │   │   │       ╰── Pointer
            │   │   │           ╰── Array
            │   │   │               ├── 5
            │   │   │               ╰── Unsigned Long
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── b
            │   │       ╰── Type
            │   │           ╰── Pointer
            │   │               ╰── Array
            │   │                   ├── 5
            │   │                   ╰── Unsigned Long
            │   ╰── Body
            │       ╰── Return
            │           ╰── <108>  [>]
            │               ├── <104> Var [a]
            │               ╰── <107> Var [b]
            ├── Function [ge_nested]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── a
            │   │   │   ╰── Type
            │   │   │       ╰── Pointer
            │   │   │           ╰── Array
            │   │   │               ├── 5
            │   │   │               ╰── Unsigned Long
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── b
            │   │       ╰── Type
            │   │           ╰── Pointer
            │   │               ╰── Array
            │   │                   ├── 5
            │   │                   ╰── Unsigned Long
            │   ╰── Body
            │       ╰── Return
            │           ╰── <135>  [>=]
            │               ├── <131> Var [a]
            │               ╰── <134> Var [b]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── arr
                    │   ╰── Type
                    │       ╰── Array
                    │           ├── 5
                    │           ╰── Unsigned Long
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── elem_1
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <157>  [+]
                    │           ├── <154> Var [arr]
                    │           ╰── <156> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── elem_4
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <168>  [+]
                    │           ├── <165> Var [arr]
                    │           ╰── <167> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <176> FunctionCall [gt]
                    │   │       ├── <173> Var [elem_1]
                    │   │       ╰── <175> Var [elem_4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <177> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <190> Unary [!]
                    │   │       ╰── <189> FunctionCall [lt]
                    │   │           ├── <185> Var [elem_1]
                    │   │           ╰── <187> Var [elem_4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <191> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <204> Unary [!]
                    │   │       ╰── <203> FunctionCall [ge]
                    │   │           ├── <199> Var [elem_1]
                    │   │           ╰── <201> Var [elem_1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <205> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <215> FunctionCall [le]
                    │   │       ├── <212> Var [elem_4]
                    │   │       ╰── <214> Var [elem_1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <216> Constant Int [4]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── one_past_the_end
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <229>  [+]
                    │           ├── <226> Var [arr]
                    │           ╰── <228> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <240> Unary [!]
                    │   │       ╰── <239> FunctionCall [gt]
                    │   │           ├── <235> Var [one_past_the_end]
                    │   │           ╰── <237> Var [elem_4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <241> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <254>  [!=]
                    │   │       ├── <247> Var [one_past_the_end]
                    │   │       ╰── <253>  [+]
                    │   │           ├── <250> Var [elem_4]
                    │   │           ╰── <252> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <255> Constant Int [6]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── nested_arr
                    │   ╰── Type
                    │       ╰── Array
                    │           ├── 4
                    │           ╰── Array
                    │               ├── 5
                    │               ╰── Unsigned Long
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── elem_3_2
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <281>  [+]
                    │           ├── <278> Dereference
                    │           │   ╰── <277>  [+]
                    │           │       ├── <273> Var [nested_arr]
                    │           │       ╰── <275> Constant Int [3]
                    │           ╰── <280> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── elem_3_3
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <297>  [+]
                    │           ├── <294> Dereference
                    │           │   ╰── <293>  [+]
                    │           │       ├── <289> Var [nested_arr]
                    │           │       ╰── <291> Constant Int [3]
                    │           ╰── <296> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <305> FunctionCall [lt]
                    │   │       ├── <302> Var [elem_3_3]
                    │   │       ╰── <304> Var [elem_3_2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <306> Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <318> Unary [!]
                    │   │       ╰── <317> FunctionCall [ge]
                    │   │           ├── <314> Var [elem_3_3]
                    │   │           ╰── <316> Var [elem_3_2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <319> Constant Int [8]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── subarray_0
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Array
                    │   │           ├── 5
                    │   │           ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <332> Var [nested_arr]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── subarray_3
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Array
                    │   │           ├── 5
                    │   │           ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <346>  [+]
                    │           ├── <343> Var [nested_arr]
                    │           ╰── <345> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── subarray_one_past_the_end
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Array
                    │   │           ├── 5
                    │   │           ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <360>  [+]
                    │           ├── <357> Var [nested_arr]
                    │           ╰── <359> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <368> FunctionCall [ge_nested]
                    │   │       ├── <365> Var [subarray_0]
                    │   │       ╰── <367> Var [subarray_3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <369> Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <382> Unary [!]
                    │   │       ╰── <381> FunctionCall [gt_nested]
                    │   │           ├── <377> Var [subarray_one_past_the_end]
                    │   │           ╰── <379> Var [subarray_3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <383> Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <396>  [!=]
                    │   │       ├── <389> Var [subarray_3]
                    │   │       ╰── <395>  [-]
                    │   │           ├── <392> Var [subarray_one_past_the_end]
                    │   │           ╰── <394> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <397> Constant Int [11]
                    ╰── Return
                        ╰── <402> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_pointer_arithmetic_pointer_add() {
    let src = r#"
        int test_add_constant_to_pointer(void) {
            long long_arr[12] = {0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 13};
            long *ptr = long_arr + 10;
            return *ptr == 13;
        }
        int test_add_negative_index(void) {
            unsigned unsigned_arr[12] = {0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 42};
            unsigned *end_ptr = unsigned_arr + 12;
            unsigned *ptr = end_ptr + -10;
            return *ptr == 2;
        }
        int test_add_pointer_to_int(void) {
            int int_arr[5] = {0, 98, 99};
            int *ptr1 = int_arr + 2;
            int *ptr2 = 2 + int_arr;
            return (ptr1 == ptr2 && *ptr2 == 99);
        }
        int test_add_different_index_types(void) {
            double double_arr[11] = {0, 0, 0, 0, 0, 6.0};
            double *ptr1 = double_arr + 5;
            double *ptr2 = double_arr + 5l;
            double *ptr3 = double_arr + 5u;
            double *ptr4 = double_arr + 5ul;
            return (ptr1 == ptr2 && ptr1 == ptr3 && ptr1 == ptr4 && *ptr4 == 6.0);
        }
        int test_add_complex_expressions(void) {
            static int flag;
            int i = -2;
            int *small_int_ptr = &i;
            extern int return_one(void);
            extern int *get_elem1_ptr(int *arr);
            extern int *get_elem2_ptr(int *arr);
            static int arr[4] = {1, 2, 3, 4};
            int *ptr = return_one() + (*small_int_ptr) +
                       (flag ? get_elem1_ptr(arr) : get_elem2_ptr(arr));
            return (ptr == arr + 1 && *ptr == 2);
        }
        int return_one(void) {
            return 1;
        }
        int *get_elem1_ptr(int *arr) {
            return arr + 1;
        }
        int *get_elem2_ptr(int *arr) {
            return arr + 2;
        }
        int test_add_multi_dimensional(void) {
            static int index = 2;
            int nested_arr[3][3] = {{1, 2, 3}, {4, 5, 6}, {7, 8, 9}};
            int(*row_pointer)[3] = nested_arr + index;
            return **row_pointer == 7;
        }
        int test_add_to_subarray_pointer(void) {
            static int index = 2;
            int nested_arr[3][3] = {{1, 2, 3}, {4, 5, 6}, {7, 8, 9}};
            int *row1 = *(nested_arr + 1);
            int *elem_ptr = row1 + index;
            return *elem_ptr == 6;
        }
        int test_subtract_from_pointer(void) {
            long long_arr[5] = {10, 9, 8, 7, 6};
            long *one_past_the_end = long_arr + 5;
            static int index = 3;
            long *subtraction_result = one_past_the_end - index;
            return *subtraction_result == 8;
        }
        int test_subtract_negative_index(void) {
            unsigned arr[5] = {100, 101, 102, 103, 104};
            unsigned *ptr = arr - (-3);
            return *ptr == 103;
        }
        int test_subtract_different_index_types(void) {
            double double_arr[11] = {0, 0, 0, 0, 0, 0, 6.0};
            double *end_ptr = double_arr + 11;
            double *ptr1 = end_ptr - 5;
            double *ptr2 = end_ptr - 5l;
            double *ptr3 = end_ptr - 5u;
            double *ptr4 = end_ptr - 5ul;
            return (ptr1 == ptr2 && ptr1 == ptr3 && ptr1 == ptr4 && *ptr4 == 6.0);
        }
        int test_subtract_complex_expressions(void) {
            static int flag = 1;
            static int four = 4;
            static int arr[4] = {1, 2, 3, 4};
            int *ptr = (flag ? get_elem1_ptr(arr) : get_elem2_ptr(arr)) - (four / -2);
            return (*ptr == 4);
        }
        int test_subtract_multi_dimensional(void) {
            static int index = 1;
            int nested_arr[3][3] = {{1, 2, 3}, {4, 5, 6}, {7, 8, 9}};
            int(*last_row_pointer)[3] = nested_arr + 2;
            int(*row_pointer)[3] = last_row_pointer - index;
            return (**row_pointer == 4);
        }
        int main(void) {
            if (!test_add_constant_to_pointer()) {
                return 1;
            }
            if (!test_add_negative_index()) {
                return 2;
            }
            if (!test_add_pointer_to_int()) {
                return 3;
            }
            if (!test_add_different_index_types()) {
                return 4;
            }
            if (!test_add_complex_expressions()) {
                return 5;
            }
            if (!test_add_multi_dimensional()) {
                return 6;
            }
            if (!test_add_to_subarray_pointer()) {
                return 7;
            }
            if (!test_subtract_from_pointer()) {
                return 8;
            }
            if (!test_subtract_negative_index()) {
                return 9;
            }
            if (!test_subtract_different_index_types()) {
                return 10;
            }
            if (!test_subtract_complex_expressions()) {
                return 11;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [test_add_constant_to_pointer]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── long_arr
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 12
            │       │   │       ╰── Long
            │       │   ╰── Initializer
            │       │       ╰── Compound
            │       │           ├── <10> Constant Int [0]
            │       │           ├── <12> Constant Int [0]
            │       │           ├── <14> Constant Int [3]
            │       │           ├── <16> Constant Int [0]
            │       │           ├── <18> Constant Int [0]
            │       │           ├── <20> Constant Int [0]
            │       │           ├── <22> Constant Int [0]
            │       │           ├── <24> Constant Int [0]
            │       │           ├── <26> Constant Int [0]
            │       │           ├── <28> Constant Int [0]
            │       │           ╰── <30> Constant Int [13]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ptr
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Long
            │       │   ╰── Initializer
            │       │       ╰── <42>  [+]
            │       │           ├── <39> Var [long_arr]
            │       │           ╰── <41> Constant Int [10]
            │       ╰── Return
            │           ╰── <50>  [==]
            │               ├── <47> Dereference
            │               │   ╰── <46> Var [ptr]
            │               ╰── <49> Constant Int [13]
            ├── Function [test_add_negative_index]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── unsigned_arr
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 12
            │       │   │       ╰── Unsigned Int
            │       │   ╰── Initializer
            │       │       ╰── Compound
            │       │           ├── <63> Constant Int [0]
            │       │           ├── <65> Constant Int [0]
            │       │           ├── <67> Constant Int [2]
            │       │           ├── <69> Constant Int [0]
            │       │           ├── <71> Constant Int [0]
            │       │           ├── <73> Constant Int [0]
            │       │           ├── <75> Constant Int [0]
            │       │           ├── <77> Constant Int [0]
            │       │           ├── <79> Constant Int [0]
            │       │           ├── <81> Constant Int [0]
            │       │           ╰── <83> Constant Int [42]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── end_ptr
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Unsigned Int
            │       │   ╰── Initializer
            │       │       ╰── <95>  [+]
            │       │           ├── <92> Var [unsigned_arr]
            │       │           ╰── <94> Constant Int [12]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ptr
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Unsigned Int
            │       │   ╰── Initializer
            │       │       ╰── <108>  [+]
            │       │           ├── <103> Var [end_ptr]
            │       │           ╰── <107> Unary [-]
            │       │               ╰── <106> Constant Int [10]
            │       ╰── Return
            │           ╰── <116>  [==]
            │               ├── <113> Dereference
            │               │   ╰── <112> Var [ptr]
            │               ╰── <115> Constant Int [2]
            ├── Function [test_add_pointer_to_int]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── int_arr
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 5
            │       │   │       ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── Compound
            │       │           ├── <129> Constant Int [0]
            │       │           ├── <131> Constant Int [98]
            │       │           ╰── <133> Constant Int [99]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ptr1
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <145>  [+]
            │       │           ├── <142> Var [int_arr]
            │       │           ╰── <144> Constant Int [2]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ptr2
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <156>  [+]
            │       │           ├── <152> Constant Int [2]
            │       │           ╰── <155> Var [int_arr]
            │       ╰── Return
            │           ╰── <173>  [&&]
            │               ├── <164>  [==]
            │               │   ├── <160> Var [ptr1]
            │               │   ╰── <163> Var [ptr2]
            │               ╰── <171>  [==]
            │                   ├── <168> Dereference
            │                   │   ╰── <167> Var [ptr2]
            │                   ╰── <170> Constant Int [99]
            ├── Function [test_add_different_index_types]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── double_arr
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 11
            │       │   │       ╰── Double
            │       │   ╰── Initializer
            │       │       ╰── Compound
            │       │           ├── <186> Constant Int [0]
            │       │           ├── <188> Constant Int [0]
            │       │           ├── <190> Constant Int [0]
            │       │           ├── <192> Constant Int [0]
            │       │           ├── <194> Constant Int [0]
            │       │           ╰── <196> Constant Double [+6e0]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ptr1
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Double
            │       │   ╰── Initializer
            │       │       ╰── <208>  [+]
            │       │           ├── <205> Var [double_arr]
            │       │           ╰── <207> Constant Int [5]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ptr2
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Double
            │       │   ╰── Initializer
            │       │       ╰── <219>  [+]
            │       │           ├── <216> Var [double_arr]
            │       │           ╰── <218> Constant Long [5]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ptr3
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Double
            │       │   ╰── Initializer
            │       │       ╰── <230>  [+]
            │       │           ├── <227> Var [double_arr]
            │       │           ╰── <229> Constant UInt [5]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ptr4
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Double
            │       │   ╰── Initializer
            │       │       ╰── <241>  [+]
            │       │           ├── <238> Var [double_arr]
            │       │           ╰── <240> Constant ULong [5]
            │       ╰── Return
            │           ╰── <274>  [&&]
            │               ├── <265>  [&&]
            │               │   ├── <257>  [&&]
            │               │   │   ├── <249>  [==]
            │               │   │   │   ├── <245> Var [ptr1]
            │               │   │   │   ╰── <248> Var [ptr2]
            │               │   │   ╰── <256>  [==]
            │               │   │       ├── <252> Var [ptr1]
            │               │   │       ╰── <255> Var [ptr3]
            │               │   ╰── <264>  [==]
            │               │       ├── <260> Var [ptr1]
            │               │       ╰── <263> Var [ptr4]
            │               ╰── <272>  [==]
            │                   ├── <269> Dereference
            │                   │   ╰── <268> Var [ptr4]
            │                   ╰── <271> Constant Double [+6e0]
            ├── Function [test_add_complex_expressions]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── flag
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Static
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── i
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <292> Unary [-]
            │       │           ╰── <291> Constant Int [2]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── small_int_ptr
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <301> AddressOf
            │       │           ╰── <300> Var [i]
            │       ├── Function [extern return_one]
            │       ├── Function [extern get_elem1_ptr]
            │       │   ╰── Parameters
            │       │       ╰── Param
            │       │           ├── Name
            │       │           │   ╰── arr
            │       │           ╰── Type
            │       │               ╰── Pointer
            │       │                   ╰── Int
            │       ├── Function [extern get_elem2_ptr]
            │       │   ╰── Parameters
            │       │       ╰── Param
            │       │           ├── Name
            │       │           │   ╰── arr
            │       │           ╰── Type
            │       │               ╰── Pointer
            │       │                   ╰── Int
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── arr
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 4
            │       │   │       ╰── Int
            │       │   ├── Initializer
            │       │   │   ╰── Compound
            │       │   │       ├── <338> Constant Int [1]
            │       │   │       ├── <340> Constant Int [2]
            │       │   │       ├── <342> Constant Int [3]
            │       │   │       ╰── <344> Constant Int [4]
            │       │   ╰── Static
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ptr
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <373>  [+]
            │       │           ├── <359>  [+]
            │       │           │   ├── <353> FunctionCall [return_one]
            │       │           │   ╰── <358> Dereference
            │       │           │       ╰── <356> Var [small_int_ptr]
            │       │           ╰── <372> Conditional [?]
            │       │               ├── <362> Var [flag]
            │       │               ├── Then
            │       │               │   ╰── <366> FunctionCall [get_elem1_ptr]
            │       │               │       ╰── <365> Var [arr]
            │       │               ╰── Else
            │       │                   ╰── <370> FunctionCall [get_elem2_ptr]
            │       │                       ╰── <369> Var [arr]
            │       ╰── Return
            │           ╰── <393>  [&&]
            │               ├── <384>  [==]
            │               │   ├── <377> Var [ptr]
            │               │   ╰── <383>  [+]
            │               │       ├── <380> Var [arr]
            │               │       ╰── <382> Constant Int [1]
            │               ╰── <391>  [==]
            │                   ├── <388> Dereference
            │                   │   ╰── <387> Var [ptr]
            │                   ╰── <390> Constant Int [2]
            ├── Function [return_one]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <401> Constant Int [1]
            ├── Function [get_elem1_ptr]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── arr
            │   │       ╰── Type
            │   │           ╰── Pointer
            │   │               ╰── Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <418>  [+]
            │               ├── <415> Var [arr]
            │               ╰── <417> Constant Int [1]
            ├── Function [get_elem2_ptr]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── arr
            │   │       ╰── Type
            │   │           ╰── Pointer
            │   │               ╰── Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <435>  [+]
            │               ├── <432> Var [arr]
            │               ╰── <434> Constant Int [2]
            ├── Function [test_add_multi_dimensional]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── index
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ├── Initializer
            │       │   │   ╰── <447> Constant Int [2]
            │       │   ╰── Static
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── nested_arr
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 3
            │       │   │       ╰── Array
            │       │   │           ├── 3
            │       │   │           ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── Compound
            │       │           ├── Compound
            │       │           │   ├── <457> Constant Int [1]
            │       │           │   ├── <459> Constant Int [2]
            │       │           │   ╰── <461> Constant Int [3]
            │       │           ├── Compound
            │       │           │   ├── <464> Constant Int [4]
            │       │           │   ├── <466> Constant Int [5]
            │       │           │   ╰── <468> Constant Int [6]
            │       │           ╰── Compound
            │       │               ├── <471> Constant Int [7]
            │       │               ├── <473> Constant Int [8]
            │       │               ╰── <475> Constant Int [9]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── row_pointer
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Array
            │       │   │           ├── 3
            │       │   │           ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <492>  [+]
            │       │           ├── <488> Var [nested_arr]
            │       │           ╰── <491> Var [index]
            │       ╰── Return
            │           ╰── <501>  [==]
            │               ├── <498> Dereference
            │               │   ╰── <497> Dereference
            │               │       ╰── <496> Var [row_pointer]
            │               ╰── <500> Constant Int [7]
            ├── Function [test_add_to_subarray_pointer]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── index
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ├── Initializer
            │       │   │   ╰── <513> Constant Int [2]
            │       │   ╰── Static
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── nested_arr
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 3
            │       │   │       ╰── Array
            │       │   │           ├── 3
            │       │   │           ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── Compound
            │       │           ├── Compound
            │       │           │   ├── <523> Constant Int [1]
            │       │           │   ├── <525> Constant Int [2]
            │       │           │   ╰── <527> Constant Int [3]
            │       │           ├── Compound
            │       │           │   ├── <530> Constant Int [4]
            │       │           │   ├── <532> Constant Int [5]
            │       │           │   ╰── <534> Constant Int [6]
            │       │           ╰── Compound
            │       │               ├── <537> Constant Int [7]
            │       │               ├── <539> Constant Int [8]
            │       │               ╰── <541> Constant Int [9]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── row1
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <556> Dereference
            │       │           ╰── <555>  [+]
            │       │               ├── <551> Var [nested_arr]
            │       │               ╰── <553> Constant Int [1]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── elem_ptr
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <568>  [+]
            │       │           ├── <564> Var [row1]
            │       │           ╰── <567> Var [index]
            │       ╰── Return
            │           ╰── <576>  [==]
            │               ├── <573> Dereference
            │               │   ╰── <572> Var [elem_ptr]
            │               ╰── <575> Constant Int [6]
            ├── Function [test_subtract_from_pointer]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── long_arr
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 5
            │       │   │       ╰── Long
            │       │   ╰── Initializer
            │       │       ╰── Compound
            │       │           ├── <589> Constant Int [10]
            │       │           ├── <591> Constant Int [9]
            │       │           ├── <593> Constant Int [8]
            │       │           ├── <595> Constant Int [7]
            │       │           ╰── <597> Constant Int [6]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── one_past_the_end
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Long
            │       │   ╰── Initializer
            │       │       ╰── <609>  [+]
            │       │           ├── <606> Var [long_arr]
            │       │           ╰── <608> Constant Int [5]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── index
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ├── Initializer
            │       │   │   ╰── <616> Constant Int [3]
            │       │   ╰── Static
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── subtraction_result
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Long
            │       │   ╰── Initializer
            │       │       ╰── <628>  [-]
            │       │           ├── <624> Var [one_past_the_end]
            │       │           ╰── <627> Var [index]
            │       ╰── Return
            │           ╰── <636>  [==]
            │               ├── <633> Dereference
            │               │   ╰── <632> Var [subtraction_result]
            │               ╰── <635> Constant Int [8]
            ├── Function [test_subtract_negative_index]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── arr
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 5
            │       │   │       ╰── Unsigned Int
            │       │   ╰── Initializer
            │       │       ╰── Compound
            │       │           ├── <649> Constant Int [100]
            │       │           ├── <651> Constant Int [101]
            │       │           ├── <653> Constant Int [102]
            │       │           ├── <655> Constant Int [103]
            │       │           ╰── <657> Constant Int [104]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ptr
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Unsigned Int
            │       │   ╰── Initializer
            │       │       ╰── <672>  [-]
            │       │           ├── <666> Var [arr]
            │       │           ╰── <671> Unary [-]
            │       │               ╰── <669> Constant Int [3]
            │       ╰── Return
            │           ╰── <680>  [==]
            │               ├── <677> Dereference
            │               │   ╰── <676> Var [ptr]
            │               ╰── <679> Constant Int [103]
            ├── Function [test_subtract_different_index_types]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── double_arr
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 11
            │       │   │       ╰── Double
            │       │   ╰── Initializer
            │       │       ╰── Compound
            │       │           ├── <693> Constant Int [0]
            │       │           ├── <695> Constant Int [0]
            │       │           ├── <697> Constant Int [0]
            │       │           ├── <699> Constant Int [0]
            │       │           ├── <701> Constant Int [0]
            │       │           ├── <703> Constant Int [0]
            │       │           ╰── <705> Constant Double [+6e0]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── end_ptr
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Double
            │       │   ╰── Initializer
            │       │       ╰── <717>  [+]
            │       │           ├── <714> Var [double_arr]
            │       │           ╰── <716> Constant Int [11]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ptr1
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Double
            │       │   ╰── Initializer
            │       │       ╰── <728>  [-]
            │       │           ├── <725> Var [end_ptr]
            │       │           ╰── <727> Constant Int [5]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ptr2
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Double
            │       │   ╰── Initializer
            │       │       ╰── <739>  [-]
            │       │           ├── <736> Var [end_ptr]
            │       │           ╰── <738> Constant Long [5]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ptr3
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Double
            │       │   ╰── Initializer
            │       │       ╰── <750>  [-]
            │       │           ├── <747> Var [end_ptr]
            │       │           ╰── <749> Constant UInt [5]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ptr4
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Double
            │       │   ╰── Initializer
            │       │       ╰── <761>  [-]
            │       │           ├── <758> Var [end_ptr]
            │       │           ╰── <760> Constant ULong [5]
            │       ╰── Return
            │           ╰── <794>  [&&]
            │               ├── <785>  [&&]
            │               │   ├── <777>  [&&]
            │               │   │   ├── <769>  [==]
            │               │   │   │   ├── <765> Var [ptr1]
            │               │   │   │   ╰── <768> Var [ptr2]
            │               │   │   ╰── <776>  [==]
            │               │   │       ├── <772> Var [ptr1]
            │               │   │       ╰── <775> Var [ptr3]
            │               │   ╰── <784>  [==]
            │               │       ├── <780> Var [ptr1]
            │               │       ╰── <783> Var [ptr4]
            │               ╰── <792>  [==]
            │                   ├── <789> Dereference
            │                   │   ╰── <788> Var [ptr4]
            │                   ╰── <791> Constant Double [+6e0]
            ├── Function [test_subtract_complex_expressions]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── flag
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ├── Initializer
            │       │   │   ╰── <806> Constant Int [1]
            │       │   ╰── Static
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── four
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ├── Initializer
            │       │   │   ╰── <813> Constant Int [4]
            │       │   ╰── Static
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── arr
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 4
            │       │   │       ╰── Int
            │       │   ├── Initializer
            │       │   │   ╰── Compound
            │       │   │       ├── <822> Constant Int [1]
            │       │   │       ├── <824> Constant Int [2]
            │       │   │       ├── <826> Constant Int [3]
            │       │   │       ╰── <828> Constant Int [4]
            │       │   ╰── Static
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ptr
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <857>  [-]
            │       │           ├── <847> Conditional [?]
            │       │           │   ├── <837> Var [flag]
            │       │           │   ├── Then
            │       │           │   │   ╰── <841> FunctionCall [get_elem1_ptr]
            │       │           │   │       ╰── <840> Var [arr]
            │       │           │   ╰── Else
            │       │           │       ╰── <845> FunctionCall [get_elem2_ptr]
            │       │           │           ╰── <844> Var [arr]
            │       │           ╰── <856>  [/]
            │       │               ├── <850> Var [four]
            │       │               ╰── <854> Unary [-]
            │       │                   ╰── <853> Constant Int [2]
            │       ╰── Return
            │           ╰── <866>  [==]
            │               ├── <862> Dereference
            │               │   ╰── <861> Var [ptr]
            │               ╰── <864> Constant Int [4]
            ├── Function [test_subtract_multi_dimensional]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── index
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ├── Initializer
            │       │   │   ╰── <878> Constant Int [1]
            │       │   ╰── Static
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── nested_arr
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 3
            │       │   │       ╰── Array
            │       │   │           ├── 3
            │       │   │           ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── Compound
            │       │           ├── Compound
            │       │           │   ├── <888> Constant Int [1]
            │       │           │   ├── <890> Constant Int [2]
            │       │           │   ╰── <892> Constant Int [3]
            │       │           ├── Compound
            │       │           │   ├── <895> Constant Int [4]
            │       │           │   ├── <897> Constant Int [5]
            │       │           │   ╰── <899> Constant Int [6]
            │       │           ╰── Compound
            │       │               ├── <902> Constant Int [7]
            │       │               ├── <904> Constant Int [8]
            │       │               ╰── <906> Constant Int [9]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── last_row_pointer
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Array
            │       │   │           ├── 3
            │       │   │           ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <922>  [+]
            │       │           ├── <919> Var [nested_arr]
            │       │           ╰── <921> Constant Int [2]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── row_pointer
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Array
            │       │   │           ├── 3
            │       │   │           ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <937>  [-]
            │       │           ├── <933> Var [last_row_pointer]
            │       │           ╰── <936> Var [index]
            │       ╰── Return
            │           ╰── <947>  [==]
            │               ├── <943> Dereference
            │               │   ╰── <942> Dereference
            │               │       ╰── <941> Var [row_pointer]
            │               ╰── <945> Constant Int [4]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <958> Unary [!]
                    │   │       ╰── <957> FunctionCall [test_add_constant_to_pointer]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <959> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <967> Unary [!]
                    │   │       ╰── <966> FunctionCall [test_add_negative_index]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <968> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <976> Unary [!]
                    │   │       ╰── <975> FunctionCall [test_add_pointer_to_int]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <977> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <985> Unary [!]
                    │   │       ╰── <984> FunctionCall [test_add_different_index_types]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <986> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <994> Unary [!]
                    │   │       ╰── <993> FunctionCall [test_add_complex_expressions]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <995> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <1003> Unary [!]
                    │   │       ╰── <1002> FunctionCall [test_add_multi_dimensional]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <1004> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <1012> Unary [!]
                    │   │       ╰── <1011> FunctionCall [test_add_to_subarray_pointer]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <1013> Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <1021> Unary [!]
                    │   │       ╰── <1020> FunctionCall [test_subtract_from_pointer]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <1022> Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <1030> Unary [!]
                    │   │       ╰── <1029> FunctionCall [test_subtract_negative_index]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <1031> Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <1039> Unary [!]
                    │   │       ╰── <1038> FunctionCall [test_subtract_different_index_types]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <1040> Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <1048> Unary [!]
                    │   │       ╰── <1047> FunctionCall [test_subtract_complex_expressions]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <1049> Constant Int [11]
                    ╰── Return
                        ╰── <1054> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_pointer_arithmetic_pointer_diff() {
    let src = r#"
        int get_ptr_diff(int *ptr1, int *ptr2) {
            return (ptr2 - ptr1);
        }
        int get_long_ptr_diff(long *ptr1, long *ptr2) {
            return (ptr2 - ptr1);
        }
        int get_multidim_ptr_diff(double (*ptr1)[3][5], double (*ptr2)[3][5]) {
            return (ptr2 - ptr1);
        }
        int get_multidim_ptr_diff_2(double (*ptr1)[5], double (*ptr2)[5]) {
            return (ptr2 - ptr1);
        }
        int main(void) {
            int arr[5] = {5, 4, 3, 2, 1};
            int *end_of_array = arr + 5;
            if (get_ptr_diff(arr, end_of_array) != 5) {
                return 1;
            }
            long long_arr[8];
            if (get_long_ptr_diff(long_arr + 3, long_arr) != -3) {
                return 2;
            }
            static double multidim[6][7][3][5];
            if (get_multidim_ptr_diff(multidim[2] + 1, multidim[2] + 4) != 3) {
                return 3;
            }
            if (get_multidim_ptr_diff_2(multidim[2][2] + 2, multidim[2][2]) != -2) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [get_ptr_diff]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── ptr1
            │   │   │   ╰── Type
            │   │   │       ╰── Pointer
            │   │   │           ╰── Int
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── ptr2
            │   │       ╰── Type
            │   │           ╰── Pointer
            │   │               ╰── Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <19>  [-]
            │               ├── <14> Var [ptr2]
            │               ╰── <17> Var [ptr1]
            ├── Function [get_long_ptr_diff]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── ptr1
            │   │   │   ╰── Type
            │   │   │       ╰── Pointer
            │   │   │           ╰── Long
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── ptr2
            │   │       ╰── Type
            │   │           ╰── Pointer
            │   │               ╰── Long
            │   ╰── Body
            │       ╰── Return
            │           ╰── <41>  [-]
            │               ├── <36> Var [ptr2]
            │               ╰── <39> Var [ptr1]
            ├── Function [get_multidim_ptr_diff]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── ptr1
            │   │   │   ╰── Type
            │   │   │       ╰── Pointer
            │   │   │           ╰── Array
            │   │   │               ├── 3
            │   │   │               ╰── Array
            │   │   │                   ├── 5
            │   │   │                   ╰── Double
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── ptr2
            │   │       ╰── Type
            │   │           ╰── Pointer
            │   │               ╰── Array
            │   │                   ├── 3
            │   │                   ╰── Array
            │   │                       ├── 5
            │   │                       ╰── Double
            │   ╰── Body
            │       ╰── Return
            │           ╰── <73>  [-]
            │               ├── <68> Var [ptr2]
            │               ╰── <71> Var [ptr1]
            ├── Function [get_multidim_ptr_diff_2]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── ptr1
            │   │   │   ╰── Type
            │   │   │       ╰── Pointer
            │   │   │           ╰── Array
            │   │   │               ├── 5
            │   │   │               ╰── Double
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── ptr2
            │   │       ╰── Type
            │   │           ╰── Pointer
            │   │               ╰── Array
            │   │                   ├── 5
            │   │                   ╰── Double
            │   ╰── Body
            │       ╰── Return
            │           ╰── <101>  [-]
            │               ├── <96> Var [ptr2]
            │               ╰── <99> Var [ptr1]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── arr
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 5
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <114> Constant Int [5]
                    │           ├── <116> Constant Int [4]
                    │           ├── <118> Constant Int [3]
                    │           ├── <120> Constant Int [2]
                    │           ╰── <122> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── end_of_array
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <134>  [+]
                    │           ├── <131> Var [arr]
                    │           ╰── <133> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <145>  [!=]
                    │   │       ├── <142> FunctionCall [get_ptr_diff]
                    │   │       │   ├── <139> Var [arr]
                    │   │       │   ╰── <141> Var [end_of_array]
                    │   │       ╰── <144> Constant Int [5]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <146> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── long_arr
                    │   ╰── Type
                    │       ╰── Array
                    │           ├── 8
                    │           ╰── Long
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <170>  [!=]
                    │   │       ├── <165> FunctionCall [get_long_ptr_diff]
                    │   │       │   ├── <162>  [+]
                    │   │       │   │   ├── <159> Var [long_arr]
                    │   │       │   │   ╰── <161> Constant Int [3]
                    │   │       │   ╰── <164> Var [long_arr]
                    │   │       ╰── <169> Unary [-]
                    │   │           ╰── <168> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <171> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── multidim
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 6
                    │   │       ╰── Array
                    │   │           ├── 7
                    │   │           ╰── Array
                    │   │               ├── 3
                    │   │               ╰── Array
                    │   │                   ├── 5
                    │   │                   ╰── Double
                    │   ╰── Static
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <207>  [!=]
                    │   │       ├── <204> FunctionCall [get_multidim_ptr_diff]
                    │   │       │   ├── <196>  [+]
                    │   │       │   │   ├── <193> Subscript
                    │   │       │   │   │   ├── <191> Var [multidim]
                    │   │       │   │   │   ╰── <192> Constant Int [2]
                    │   │       │   │   ╰── <195> Constant Int [1]
                    │   │       │   ╰── <203>  [+]
                    │   │       │       ├── <200> Subscript
                    │   │       │       │   ├── <198> Var [multidim]
                    │   │       │       │   ╰── <199> Constant Int [2]
                    │   │       │       ╰── <202> Constant Int [4]
                    │   │       ╰── <206> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <208> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <234>  [!=]
                    │   │       ├── <229> FunctionCall [get_multidim_ptr_diff_2]
                    │   │       │   ├── <222>  [+]
                    │   │       │   │   ├── <219> Subscript
                    │   │       │   │   │   ├── <217> Subscript
                    │   │       │   │   │   │   ├── <215> Var [multidim]
                    │   │       │   │   │   │   ╰── <216> Constant Int [2]
                    │   │       │   │   │   ╰── <218> Constant Int [2]
                    │   │       │   │   ╰── <221> Constant Int [2]
                    │   │       │   ╰── <228> Subscript
                    │   │       │       ├── <226> Subscript
                    │   │       │       │   ├── <224> Var [multidim]
                    │   │       │       │   ╰── <225> Constant Int [2]
                    │   │       │       ╰── <227> Constant Int [2]
                    │   │       ╰── <233> Unary [-]
                    │   │           ╰── <232> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <235> Constant Int [4]
                    ╰── Return
                        ╰── <240> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_subscripting_addition_subscript_equivalence() {
    let src = r#"
        int main(void)
        {
            unsigned long x[300][5];
            for (int i = 0; i < 300; i = i + 1) {
                for (int j = 0; j < 5; j = j + 1) {
                    x[i][j] = i * 5 + j;
                }
            }
            if (*(*(x + 20) + 3) != x[20][3]) {
                return 1;
            }
            if (&(*(*(x + 290) + 3)) != &x[290][3]) {
                return 2;
            }
            for (int i = 0; i < 300; i = i + 1) {
                for (int j = 0; j < 5; j = j + 1) {
                    if (*(*(x + i) + j) != x[i][j]) {
                        return 3;
                    }
                }
            }
            *(*(x + 275) + 4) = 22000ul;
            if (x[275][4] != 22000ul) {
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
                    │   │   ╰── x
                    │   ╰── Type
                    │       ╰── Array
                    │           ├── 300
                    │           ╰── Array
                    │               ├── 5
                    │               ╰── Unsigned Long
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <16> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <24>  [<]
                    │   │       ├── <21> Var [i]
                    │   │       ╰── <23> Constant Int [300]
                    │   ├── Condition
                    │   │   ╰── <33> Assign [=]
                    │   │       ├── <26> Var [i]
                    │   │       ╰── <32>  [+]
                    │   │           ├── <29> Var [i]
                    │   │           ╰── <31> Constant Int [1]
                    │   ╰── Block
                    │       ╰── For
                    │           ├── Init
                    │           │   ╰── VarDeclaration
                    │           │       ├── Name
                    │           │       │   ╰── j
                    │           │       ├── Type
                    │           │       │   ╰── Int
                    │           │       ╰── Initializer
                    │           │           ╰── <37> Constant Int [0]
                    │           ├── Condition
                    │           │   ╰── <45>  [<]
                    │           │       ├── <42> Var [j]
                    │           │       ╰── <44> Constant Int [5]
                    │           ├── Condition
                    │           │   ╰── <54> Assign [=]
                    │           │       ├── <47> Var [j]
                    │           │       ╰── <53>  [+]
                    │           │           ├── <50> Var [j]
                    │           │           ╰── <52> Constant Int [1]
                    │           ╰── Block
                    │               ╰── <73> Assign [=]
                    │                   ├── <62> Subscript
                    │                   │   ├── <59> Subscript
                    │                   │   │   ├── <56> Var [x]
                    │                   │   │   ╰── <58> Var [i]
                    │                   │   ╰── <61> Var [j]
                    │                   ╰── <72>  [+]
                    │                       ├── <68>  [*]
                    │                       │   ├── <65> Var [i]
                    │                       │   ╰── <67> Constant Int [5]
                    │                       ╰── <71> Var [j]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <100>  [!=]
                    │   │       ├── <92> Dereference
                    │   │       │   ╰── <91>  [+]
                    │   │       │       ├── <87> Dereference
                    │   │       │       │   ╰── <86>  [+]
                    │   │       │       │       ├── <82> Var [x]
                    │   │       │       │       ╰── <84> Constant Int [20]
                    │   │       │       ╰── <89> Constant Int [3]
                    │   │       ╰── <99> Subscript
                    │   │           ├── <97> Subscript
                    │   │           │   ├── <95> Var [x]
                    │   │           │   ╰── <96> Constant Int [20]
                    │   │           ╰── <98> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <101> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <128>  [!=]
                    │   │       ├── <119> AddressOf
                    │   │       │   ╰── <118> Dereference
                    │   │       │       ╰── <116>  [+]
                    │   │       │           ├── <112> Dereference
                    │   │       │           │   ╰── <111>  [+]
                    │   │       │           │       ├── <107> Var [x]
                    │   │       │           │       ╰── <109> Constant Int [290]
                    │   │       │           ╰── <114> Constant Int [3]
                    │   │       ╰── <127> AddressOf
                    │   │           ╰── <126> Subscript
                    │   │               ├── <124> Subscript
                    │   │               │   ├── <122> Var [x]
                    │   │               │   ╰── <123> Constant Int [290]
                    │   │               ╰── <125> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <129> Constant Int [2]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <137> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <145>  [<]
                    │   │       ├── <142> Var [i]
                    │   │       ╰── <144> Constant Int [300]
                    │   ├── Condition
                    │   │   ╰── <154> Assign [=]
                    │   │       ├── <147> Var [i]
                    │   │       ╰── <153>  [+]
                    │   │           ├── <150> Var [i]
                    │   │           ╰── <152> Constant Int [1]
                    │   ╰── Block
                    │       ╰── For
                    │           ├── Init
                    │           │   ╰── VarDeclaration
                    │           │       ├── Name
                    │           │       │   ╰── j
                    │           │       ├── Type
                    │           │       │   ╰── Int
                    │           │       ╰── Initializer
                    │           │           ╰── <158> Constant Int [0]
                    │           ├── Condition
                    │           │   ╰── <166>  [<]
                    │           │       ├── <163> Var [j]
                    │           │       ╰── <165> Constant Int [5]
                    │           ├── Condition
                    │           │   ╰── <175> Assign [=]
                    │           │       ├── <168> Var [j]
                    │           │       ╰── <174>  [+]
                    │           │           ├── <171> Var [j]
                    │           │           ╰── <173> Constant Int [1]
                    │           ╰── Block
                    │               ╰── If
                    │                   ├── Condition
                    │                   │   ╰── <199>  [!=]
                    │                   │       ├── <189> Dereference
                    │                   │       │   ╰── <188>  [+]
                    │                   │       │       ├── <183> Dereference
                    │                   │       │       │   ╰── <182>  [+]
                    │                   │       │       │       ├── <177> Var [x]
                    │                   │       │       │       ╰── <180> Var [i]
                    │                   │       │       ╰── <186> Var [j]
                    │                   │       ╰── <198> Subscript
                    │                   │           ├── <195> Subscript
                    │                   │           │   ├── <192> Var [x]
                    │                   │           │   ╰── <194> Var [i]
                    │                   │           ╰── <197> Var [j]
                    │                   ╰── Then
                    │                       ╰── Block
                    │                           ╰── Return
                    │                               ╰── <200> Constant Int [3]
                    ├── <225> Assign [=]
                    │   ├── <222> Dereference
                    │   │   ╰── <221>  [+]
                    │   │       ├── <217> Dereference
                    │   │       │   ╰── <216>  [+]
                    │   │       │       ├── <212> Var [x]
                    │   │       │       ╰── <214> Constant Int [275]
                    │   │       ╰── <219> Constant Int [4]
                    │   ╰── <224> Constant ULong [22000]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <235>  [!=]
                    │   │       ├── <232> Subscript
                    │   │       │   ├── <230> Subscript
                    │   │       │   │   ├── <228> Var [x]
                    │   │       │   │   ╰── <229> Constant Int [275]
                    │   │       │   ╰── <231> Constant Int [4]
                    │   │       ╰── <234> Constant ULong [22000]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <236> Constant Int [4]
                    ╰── Return
                        ╰── <241> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_subscripting_array_of_pointers_to_arrays() {
    let src = r#"
        int main(void) {
            int x = 0;
            int y = 1;
            int z = 2;
            int *arr[3] = { &x, &y, &z };
            int *arr2[3] = {&z, &y, &x};
            int *(*array_of_pointers[3])[3] = {&arr, &arr2, &arr};
            if (array_of_pointers[0] != (int *(*)[3]) arr) {
                return 1;
            }
            if (array_of_pointers[1] != (int *(*)[3]) arr2) {
                return 2;
            }
            if (array_of_pointers[2] != (int *(*)[3]) arr) {
                return 3;
            }
            if (array_of_pointers[1][0][0] != &z) {
                return 4;
            }
            if (array_of_pointers[1][0][1] != &y) {
                return 5;
            }
            if (array_of_pointers[2][0][2][0] != 2) {
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
                    │       ╰── <8> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── y
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <14> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── z
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <20> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── arr
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 3
                    │   │       ╰── Pointer
                    │   │           ╰── Int
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <31> AddressOf
                    │           │   ╰── <30> Var [x]
                    │           ├── <35> AddressOf
                    │           │   ╰── <34> Var [y]
                    │           ╰── <39> AddressOf
                    │               ╰── <38> Var [z]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── arr2
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 3
                    │   │       ╰── Pointer
                    │   │           ╰── Int
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <51> AddressOf
                    │           │   ╰── <50> Var [z]
                    │           ├── <55> AddressOf
                    │           │   ╰── <54> Var [y]
                    │           ╰── <59> AddressOf
                    │               ╰── <58> Var [x]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── array_of_pointers
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 3
                    │   │       ╰── Pointer
                    │   │           ╰── Array
                    │   │               ├── 3
                    │   │               ╰── Pointer
                    │   │                   ╰── Int
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <75> AddressOf
                    │           │   ╰── <74> Var [arr]
                    │           ├── <79> AddressOf
                    │           │   ╰── <78> Var [arr2]
                    │           ╰── <83> AddressOf
                    │               ╰── <82> Var [arr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <102>  [!=]
                    │   │       ├── <90> Subscript
                    │   │       │   ├── <88> Var [array_of_pointers]
                    │   │       │   ╰── <89> Constant Int [0]
                    │   │       ╰── <101> Cast
                    │   │           ├── Target
                    │   │           │   ╰── Pointer
                    │   │           │       ╰── Array
                    │   │           │           ├── 3
                    │   │           │           ╰── Pointer
                    │   │           │               ╰── Int
                    │   │           ╰── Expression
                    │   │               ╰── <100> Var [arr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <103> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <123>  [!=]
                    │   │       ├── <111> Subscript
                    │   │       │   ├── <109> Var [array_of_pointers]
                    │   │       │   ╰── <110> Constant Int [1]
                    │   │       ╰── <122> Cast
                    │   │           ├── Target
                    │   │           │   ╰── Pointer
                    │   │           │       ╰── Array
                    │   │           │           ├── 3
                    │   │           │           ╰── Pointer
                    │   │           │               ╰── Int
                    │   │           ╰── Expression
                    │   │               ╰── <121> Var [arr2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <124> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <144>  [!=]
                    │   │       ├── <132> Subscript
                    │   │       │   ├── <130> Var [array_of_pointers]
                    │   │       │   ╰── <131> Constant Int [2]
                    │   │       ╰── <143> Cast
                    │   │           ├── Target
                    │   │           │   ╰── Pointer
                    │   │           │       ╰── Array
                    │   │           │           ├── 3
                    │   │           │           ╰── Pointer
                    │   │           │               ╰── Int
                    │   │           ╰── Expression
                    │   │               ╰── <142> Var [arr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <145> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <162>  [!=]
                    │   │       ├── <157> Subscript
                    │   │       │   ├── <155> Subscript
                    │   │       │   │   ├── <153> Subscript
                    │   │       │   │   │   ├── <151> Var [array_of_pointers]
                    │   │       │   │   │   ╰── <152> Constant Int [1]
                    │   │       │   │   ╰── <154> Constant Int [0]
                    │   │       │   ╰── <156> Constant Int [0]
                    │   │       ╰── <161> AddressOf
                    │   │           ╰── <160> Var [z]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <163> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <180>  [!=]
                    │   │       ├── <175> Subscript
                    │   │       │   ├── <173> Subscript
                    │   │       │   │   ├── <171> Subscript
                    │   │       │   │   │   ├── <169> Var [array_of_pointers]
                    │   │       │   │   │   ╰── <170> Constant Int [1]
                    │   │       │   │   ╰── <172> Constant Int [0]
                    │   │       │   ╰── <174> Constant Int [1]
                    │   │       ╰── <179> AddressOf
                    │   │           ╰── <178> Var [y]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <181> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <198>  [!=]
                    │   │       ├── <195> Subscript
                    │   │       │   ├── <193> Subscript
                    │   │       │   │   ├── <191> Subscript
                    │   │       │   │   │   ├── <189> Subscript
                    │   │       │   │   │   │   ├── <187> Var [array_of_pointers]
                    │   │       │   │   │   │   ╰── <188> Constant Int [2]
                    │   │       │   │   │   ╰── <190> Constant Int [0]
                    │   │       │   │   ╰── <192> Constant Int [2]
                    │   │       │   ╰── <194> Constant Int [0]
                    │   │       ╰── <197> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <199> Constant Int [6]
                    ╰── Return
                        ╰── <204> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_subscripting_complex_operands() {
    let src = r#"
        int assign_in_index(int idx) {
            int arr[3] = {1, 2, 3};
            int val = arr[idx = idx + 2];
            if (idx != 1) {
                return 1;
            }
            if (val != 2) {
                return 2;
            }
            return 0;
        }
        int static_index(void) {
            static int index = 0;
            int retval = index;
            index = index + 1;
            return retval;
        }
        int funcall_in_index(void) {
            int arr[3] = {1, 2, 3};
            int v1 = arr[static_index()];
            int v2 = arr[static_index()];
            if (v1 != 1) {
                return 3;
            }
            if (v2 != 2) {
                return 4;
            }
            return 0;
        }
        int subscript_inception(long *arr, int *a, int b){
            return arr[a[b]];
        }
        int check_subscript_inception(void) {
            long arr[4] = {4, 3, 2, 1};
            int indices[2] = {1, 2};
            if (subscript_inception(arr, indices, 1) != 2) {
                return 5;
            }
            if (subscript_inception(arr, indices, 0) != 3) {
                return 6;
            }
            return 0;
        }
        int *get_array(void) {
            static int arr[3];
            return arr;
        }
        int subscript_function_result(void){
            get_array()[2] = 1;
            if (get_array()[2] != 1) {
                return 7;
            }
            return 0;
        }
        int negate_subscript(int *arr, int idx, int expected) {
            if (arr[-idx] != expected) {
                return 8;
            }
            return 0;
        }
        int main(void) {
            int check = assign_in_index(-1);
            if (check) {
                return check;
            }
            check = funcall_in_index();
            if (check) {
                return check;
            }
            check = check_subscript_inception();
            if (check) {
                return check;
            }
            check = subscript_function_result();
            if (check) {
                return check;
            }
            int arr[3] = {0, 1, 2};
            check = negate_subscript(arr + 2, 2, 0);
            if (check) {
                return check;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [assign_in_index]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── idx
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── arr
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 3
            │       │   │       ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── Compound
            │       │           ├── <13> Constant Int [1]
            │       │           ├── <15> Constant Int [2]
            │       │           ╰── <17> Constant Int [3]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── val
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <35> Subscript
            │       │           ├── <25> Var [arr]
            │       │           ╰── <34> Assign [=]
            │       │               ├── <27> Var [idx]
            │       │               ╰── <33>  [+]
            │       │                   ├── <30> Var [idx]
            │       │                   ╰── <32> Constant Int [2]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <42>  [!=]
            │       │   │       ├── <39> Var [idx]
            │       │   │       ╰── <41> Constant Int [1]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <43> Constant Int [1]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <52>  [!=]
            │       │   │       ├── <49> Var [val]
            │       │   │       ╰── <51> Constant Int [2]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <53> Constant Int [2]
            │       ╰── Return
            │           ╰── <58> Constant Int [0]
            ├── Function [static_index]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── index
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ├── Initializer
            │       │   │   ╰── <70> Constant Int [0]
            │       │   ╰── Static
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── retval
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <77> Var [index]
            │       ├── <88> Assign [=]
            │       │   ├── <81> Var [index]
            │       │   ╰── <87>  [+]
            │       │       ├── <84> Var [index]
            │       │       ╰── <86> Constant Int [1]
            │       ╰── Return
            │           ╰── <91> Var [retval]
            ├── Function [funcall_in_index]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── arr
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 3
            │       │   │       ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── Compound
            │       │           ├── <104> Constant Int [1]
            │       │           ├── <106> Constant Int [2]
            │       │           ╰── <108> Constant Int [3]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── v1
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <119> Subscript
            │       │           ├── <116> Var [arr]
            │       │           ╰── <118> FunctionCall [static_index]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── v2
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <129> Subscript
            │       │           ├── <126> Var [arr]
            │       │           ╰── <128> FunctionCall [static_index]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <136>  [!=]
            │       │   │       ├── <133> Var [v1]
            │       │   │       ╰── <135> Constant Int [1]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <137> Constant Int [3]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <146>  [!=]
            │       │   │       ├── <143> Var [v2]
            │       │   │       ╰── <145> Constant Int [2]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <147> Constant Int [4]
            │       ╰── Return
            │           ╰── <152> Constant Int [0]
            ├── Function [subscript_inception]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── arr
            │   │   │   ╰── Type
            │   │   │       ╰── Pointer
            │   │   │           ╰── Long
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── a
            │   │   │   ╰── Type
            │   │   │       ╰── Pointer
            │   │   │           ╰── Int
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── b
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <178> Subscript
            │               ├── <172> Var [arr]
            │               ╰── <177> Subscript
            │                   ├── <174> Var [a]
            │                   ╰── <176> Var [b]
            ├── Function [check_subscript_inception]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── arr
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 4
            │       │   │       ╰── Long
            │       │   ╰── Initializer
            │       │       ╰── Compound
            │       │           ├── <191> Constant Int [4]
            │       │           ├── <193> Constant Int [3]
            │       │           ├── <195> Constant Int [2]
            │       │           ╰── <197> Constant Int [1]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── indices
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 2
            │       │   │       ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── Compound
            │       │           ├── <206> Constant Int [1]
            │       │           ╰── <208> Constant Int [2]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <221>  [!=]
            │       │   │       ├── <218> FunctionCall [subscript_inception]
            │       │   │       │   ├── <214> Var [arr]
            │       │   │       │   ├── <216> Var [indices]
            │       │   │       │   ╰── <217> Constant Int [1]
            │       │   │       ╰── <220> Constant Int [2]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <222> Constant Int [5]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <236>  [!=]
            │       │   │       ├── <233> FunctionCall [subscript_inception]
            │       │   │       │   ├── <229> Var [arr]
            │       │   │       │   ├── <231> Var [indices]
            │       │   │       │   ╰── <232> Constant Int [0]
            │       │   │       ╰── <235> Constant Int [3]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <237> Constant Int [6]
            │       ╰── Return
            │           ╰── <242> Constant Int [0]
            ├── Function [get_array]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── arr
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 3
            │       │   │       ╰── Int
            │       │   ╰── Static
            │       ╰── Return
            │           ╰── <259> Var [arr]
            ├── Function [subscript_function_result]
            │   ╰── Body
            │       ├── <273> Assign [=]
            │       │   ├── <270> Subscript
            │       │   │   ├── <268> FunctionCall [get_array]
            │       │   │   ╰── <269> Constant Int [2]
            │       │   ╰── <272> Constant Int [1]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <281>  [!=]
            │       │   │       ├── <278> Subscript
            │       │   │       │   ├── <276> FunctionCall [get_array]
            │       │   │       │   ╰── <277> Constant Int [2]
            │       │   │       ╰── <280> Constant Int [1]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <282> Constant Int [7]
            │       ╰── Return
            │           ╰── <287> Constant Int [0]
            ├── Function [negate_subscript]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── arr
            │   │   │   ╰── Type
            │   │   │       ╰── Pointer
            │   │   │           ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── idx
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── expected
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <315>  [!=]
            │       │   │       ├── <311> Subscript
            │       │   │       │   ├── <306> Var [arr]
            │       │   │       │   ╰── <310> Unary [-]
            │       │   │       │       ╰── <309> Var [idx]
            │       │   │       ╰── <314> Var [expected]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <316> Constant Int [8]
            │       ╰── Return
            │           ╰── <321> Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── check
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <336> FunctionCall [assign_in_index]
                    │           ╰── <335> Unary [-]
                    │               ╰── <334> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <340> Var [check]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <342> Var [check]
                    ├── <352> Assign [=]
                    │   ├── <348> Var [check]
                    │   ╰── <351> FunctionCall [funcall_in_index]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <355> Var [check]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <357> Var [check]
                    ├── <367> Assign [=]
                    │   ├── <363> Var [check]
                    │   ╰── <366> FunctionCall [check_subscript_inception]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <370> Var [check]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <372> Var [check]
                    ├── <382> Assign [=]
                    │   ├── <378> Var [check]
                    │   ╰── <381> FunctionCall [subscript_function_result]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <385> Var [check]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <387> Var [check]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── arr
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 3
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <397> Constant Int [0]
                    │           ├── <399> Constant Int [1]
                    │           ╰── <401> Constant Int [2]
                    ├── <417> Assign [=]
                    │   ├── <406> Var [check]
                    │   ╰── <416> FunctionCall [negate_subscript]
                    │       ├── <413>  [+]
                    │       │   ├── <410> Var [arr]
                    │       │   ╰── <412> Constant Int [2]
                    │       ├── <414> Constant Int [2]
                    │       ╰── <415> Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <420> Var [check]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <422> Var [check]
                    ╰── Return
                        ╰── <427> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_subscripting_simple() {
    let src = r#"
        int main(void) {
            int arr[3] = {1, 2, 3};
            return arr[2];
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
                    ╰── Return
                        ╰── <21> Subscript
                            ├── <19> Var [arr]
                            ╰── <20> Constant Int [2]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_subscripting_simple_subscripts() {
    let src = r#"
        int integer_types(unsigned *arr, unsigned expected) {
            unsigned val1 = arr[5];
            unsigned val2 = arr[5u];
            unsigned val3 = arr[5l];
            unsigned val4 = arr[5ul];
            if (val1 != expected) {
                return 1;
            }
            if (val2 != expected) {
                return 2;
            }
            if (val3 != expected) {
                return 3;
            }
            if (val4 != expected) {
                return 4;
            }
            return 0;
        }
        int reverse_subscript(long *arr, long expected) {
            if (arr[3] != expected) {
                return 5;
            }
            if (3[arr] != expected) {
                return 6;
            }
            if (&3[arr] != &arr[3]) {
                return 7;
            }
            return 0;
        }
        static double static_array[3] = {0.1, 0.2, 0.3};
        int subscript_static(void) {
            if (static_array[0] != 0.1) {
                return 8;
            }
            if (static_array[1] != 0.2) {
                return 9;
            }
            if (static_array[2] != 0.3) {
                return 10;
            }
            return 0;
        }
        int update_element(int *arr, int expected) {
            arr[10] = arr[10] * 2;
            if (arr[10] != expected) {
                return 11;
            }
            return 0;
        }
        int *increment_static_element(void) {
            static int arr[4];
            arr[3] = arr[3] + 1;
            return arr;
        }
        int check_increment_static_element(void) {
            int *arr1 = increment_static_element();
            if (arr1[3] != 1) {
                return 12;
            }
            if (arr1[0] || arr1[1] || arr1[2]) {
                return 13;
            }
            int *arr2 = increment_static_element();
            if (arr1 != arr2) {
                return 14;
            }
            if (arr1[3] != 2) {
                return 15;
            }
            return 0;
        }
        int main(void) {
            unsigned int unsigned_arr[6] = {0, 0, 0, 0, 0, 7u};
            int check = integer_types(unsigned_arr, 7u);
            if (check) {
                return check;
            }
            long int long_arr[4] = {100, 102, 104, 106};
            check = reverse_subscript(long_arr, 106);
            if (check) {
                return check;
            }
            check = subscript_static();
            if (check) {
                return check;
            }
            int int_arr[11] = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15};
            check = update_element(int_arr, 30);
            if (check) {
                return check;
            }
            check = check_increment_static_element();
            if (check) {
                return check;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [integer_types]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── arr
            │   │   │   ╰── Type
            │   │   │       ╰── Pointer
            │   │   │           ╰── Unsigned Int
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── expected
            │   │       ╰── Type
            │   │           ╰── Unsigned Int
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── val1
            │       │   ├── Type
            │       │   │   ╰── Unsigned Int
            │       │   ╰── Initializer
            │       │       ╰── <18> Subscript
            │       │           ├── <16> Var [arr]
            │       │           ╰── <17> Constant Int [5]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── val2
            │       │   ├── Type
            │       │   │   ╰── Unsigned Int
            │       │   ╰── Initializer
            │       │       ╰── <27> Subscript
            │       │           ├── <25> Var [arr]
            │       │           ╰── <26> Constant UInt [5]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── val3
            │       │   ├── Type
            │       │   │   ╰── Unsigned Int
            │       │   ╰── Initializer
            │       │       ╰── <36> Subscript
            │       │           ├── <34> Var [arr]
            │       │           ╰── <35> Constant Long [5]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── val4
            │       │   ├── Type
            │       │   │   ╰── Unsigned Int
            │       │   ╰── Initializer
            │       │       ╰── <45> Subscript
            │       │           ├── <43> Var [arr]
            │       │           ╰── <44> Constant ULong [5]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <53>  [!=]
            │       │   │       ├── <49> Var [val1]
            │       │   │       ╰── <52> Var [expected]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <54> Constant Int [1]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <64>  [!=]
            │       │   │       ├── <60> Var [val2]
            │       │   │       ╰── <63> Var [expected]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <65> Constant Int [2]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <75>  [!=]
            │       │   │       ├── <71> Var [val3]
            │       │   │       ╰── <74> Var [expected]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <76> Constant Int [3]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <86>  [!=]
            │       │   │       ├── <82> Var [val4]
            │       │   │       ╰── <85> Var [expected]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <87> Constant Int [4]
            │       ╰── Return
            │           ╰── <92> Constant Int [0]
            ├── Function [reverse_subscript]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── arr
            │   │   │   ╰── Type
            │   │   │       ╰── Pointer
            │   │   │           ╰── Long
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── expected
            │   │       ╰── Type
            │   │           ╰── Long
            │   ╰── Body
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <114>  [!=]
            │       │   │       ├── <110> Subscript
            │       │   │       │   ├── <108> Var [arr]
            │       │   │       │   ╰── <109> Constant Int [3]
            │       │   │       ╰── <113> Var [expected]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <115> Constant Int [5]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <127>  [!=]
            │       │   │       ├── <123> Subscript
            │       │   │       │   ├── <120> Constant Int [3]
            │       │   │       │   ╰── <122> Var [arr]
            │       │   │       ╰── <126> Var [expected]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <128> Constant Int [6]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <144>  [!=]
            │       │   │       ├── <137> AddressOf
            │       │   │       │   ╰── <136> Subscript
            │       │   │       │       ├── <133> Constant Int [3]
            │       │   │       │       ╰── <135> Var [arr]
            │       │   │       ╰── <143> AddressOf
            │       │   │           ╰── <142> Subscript
            │       │   │               ├── <140> Var [arr]
            │       │   │               ╰── <141> Constant Int [3]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <145> Constant Int [7]
            │       ╰── Return
            │           ╰── <150> Constant Int [0]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── static_array
            │   ├── Type
            │   │   ╰── Array
            │   │       ├── 3
            │   │       ╰── Double
            │   ├── Initializer
            │   │   ╰── Compound
            │   │       ├── <160> Constant Double [+1e-1]
            │   │       ├── <162> Constant Double [+2e-1]
            │   │       ╰── <164> Constant Double [+3e-1]
            │   ╰── Static
            ├── Function [subscript_static]
            │   ╰── Body
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <178>  [!=]
            │       │   │       ├── <175> Subscript
            │       │   │       │   ├── <173> Var [static_array]
            │       │   │       │   ╰── <174> Constant Int [0]
            │       │   │       ╰── <177> Constant Double [+1e-1]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <179> Constant Int [8]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <190>  [!=]
            │       │   │       ├── <187> Subscript
            │       │   │       │   ├── <185> Var [static_array]
            │       │   │       │   ╰── <186> Constant Int [1]
            │       │   │       ╰── <189> Constant Double [+2e-1]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <191> Constant Int [9]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <202>  [!=]
            │       │   │       ├── <199> Subscript
            │       │   │       │   ├── <197> Var [static_array]
            │       │   │       │   ╰── <198> Constant Int [2]
            │       │   │       ╰── <201> Constant Double [+3e-1]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <203> Constant Int [10]
            │       ╰── Return
            │           ╰── <208> Constant Int [0]
            ├── Function [update_element]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── arr
            │   │   │   ╰── Type
            │   │   │       ╰── Pointer
            │   │   │           ╰── Int
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── expected
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ├── <235> Assign [=]
            │       │   ├── <226> Subscript
            │       │   │   ├── <224> Var [arr]
            │       │   │   ╰── <225> Constant Int [10]
            │       │   ╰── <234>  [*]
            │       │       ├── <231> Subscript
            │       │       │   ├── <229> Var [arr]
            │       │       │   ╰── <230> Constant Int [10]
            │       │       ╰── <233> Constant Int [2]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <244>  [!=]
            │       │   │       ├── <240> Subscript
            │       │   │       │   ├── <238> Var [arr]
            │       │   │       │   ╰── <239> Constant Int [10]
            │       │   │       ╰── <243> Var [expected]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <245> Constant Int [11]
            │       ╰── Return
            │           ╰── <250> Constant Int [0]
            ├── Function [increment_static_element]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── arr
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 4
            │       │   │       ╰── Int
            │       │   ╰── Static
            │       ├── <278> Assign [=]
            │       │   ├── <269> Subscript
            │       │   │   ├── <267> Var [arr]
            │       │   │   ╰── <268> Constant Int [3]
            │       │   ╰── <277>  [+]
            │       │       ├── <274> Subscript
            │       │       │   ├── <272> Var [arr]
            │       │       │   ╰── <273> Constant Int [3]
            │       │       ╰── <276> Constant Int [1]
            │       ╰── Return
            │           ╰── <281> Var [arr]
            ├── Function [check_increment_static_element]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── arr1
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <294> FunctionCall [increment_static_element]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <303>  [!=]
            │       │   │       ├── <300> Subscript
            │       │   │       │   ├── <298> Var [arr1]
            │       │   │       │   ╰── <299> Constant Int [3]
            │       │   │       ╰── <302> Constant Int [1]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <304> Constant Int [12]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <324>  [||]
            │       │   │       ├── <318>  [||]
            │       │   │       │   ├── <312> Subscript
            │       │   │       │   │   ├── <310> Var [arr1]
            │       │   │       │   │   ╰── <311> Constant Int [0]
            │       │   │       │   ╰── <317> Subscript
            │       │   │       │       ├── <315> Var [arr1]
            │       │   │       │       ╰── <316> Constant Int [1]
            │       │   │       ╰── <323> Subscript
            │       │   │           ├── <321> Var [arr1]
            │       │   │           ╰── <322> Constant Int [2]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <325> Constant Int [13]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── arr2
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <335> FunctionCall [increment_static_element]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <343>  [!=]
            │       │   │       ├── <339> Var [arr1]
            │       │   │       ╰── <342> Var [arr2]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <344> Constant Int [14]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <355>  [!=]
            │       │   │       ├── <352> Subscript
            │       │   │       │   ├── <350> Var [arr1]
            │       │   │       │   ╰── <351> Constant Int [3]
            │       │   │       ╰── <354> Constant Int [2]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <356> Constant Int [15]
            │       ╰── Return
            │           ╰── <361> Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── unsigned_arr
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 6
                    │   │       ╰── Unsigned Int
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <374> Constant Int [0]
                    │           ├── <376> Constant Int [0]
                    │           ├── <378> Constant Int [0]
                    │           ├── <380> Constant Int [0]
                    │           ├── <382> Constant Int [0]
                    │           ╰── <384> Constant UInt [7]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── check
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <395> FunctionCall [integer_types]
                    │           ├── <393> Var [unsigned_arr]
                    │           ╰── <394> Constant UInt [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <399> Var [check]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <401> Var [check]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── long_arr
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 4
                    │   │       ╰── Long
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <411> Constant Int [100]
                    │           ├── <413> Constant Int [102]
                    │           ├── <415> Constant Int [104]
                    │           ╰── <417> Constant Int [106]
                    ├── <429> Assign [=]
                    │   ├── <422> Var [check]
                    │   ╰── <428> FunctionCall [reverse_subscript]
                    │       ├── <426> Var [long_arr]
                    │       ╰── <427> Constant Int [106]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <432> Var [check]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <434> Var [check]
                    ├── <444> Assign [=]
                    │   ├── <440> Var [check]
                    │   ╰── <443> FunctionCall [subscript_static]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <447> Var [check]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <449> Var [check]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── int_arr
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 11
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <459> Constant Int [0]
                    │           ├── <461> Constant Int [0]
                    │           ├── <463> Constant Int [0]
                    │           ├── <465> Constant Int [0]
                    │           ├── <467> Constant Int [0]
                    │           ├── <469> Constant Int [0]
                    │           ├── <471> Constant Int [0]
                    │           ├── <473> Constant Int [0]
                    │           ├── <475> Constant Int [0]
                    │           ├── <477> Constant Int [0]
                    │           ╰── <479> Constant Int [15]
                    ├── <491> Assign [=]
                    │   ├── <484> Var [check]
                    │   ╰── <490> FunctionCall [update_element]
                    │       ├── <488> Var [int_arr]
                    │       ╰── <489> Constant Int [30]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <494> Var [check]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <496> Var [check]
                    ├── <506> Assign [=]
                    │   ├── <502> Var [check]
                    │   ╰── <505> FunctionCall [check_increment_static_element]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <509> Var [check]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <511> Var [check]
                    ╰── Return
                        ╰── <516> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_subscripting_subscript_nested() {
    let src = r#"
        int read_nested(int nested_arr[2][3], int i, int j, int expected) {
            return (nested_arr[i][j] == expected);
        }
        int write_nested(int nested_arr[2][3], int i, int j, int new_val) {
            nested_arr[i][j] = new_val;
            return 0;
        }
        int read_nested_negated(int (*nested_arr)[3], int i, int j, int expected) {
            return (nested_arr[-i][j] == expected);
        }
        int get_nested_addr(int nested_arr[2][3], int i, int j, int *expected) {
            return &nested_arr[i][j] == expected;
        }
        static int nested_arr[4][3][5] = {
            {{1, 2}, {3}},
            {{4}, {5}}
        };
        int read_static_nested(int i, int j, int k, int expected) {
            return nested_arr[i][j][k] == expected;
        }
        int (*get_array(void))[3][5] {
            return nested_arr;
        }
        int write_nested_complex(int i, int j, int k, int val) {
            get_array()[i][j][k] = val;
            return 0;
        }
        int *get_subarray(int nested[2][3], int i) {
            return nested[i];
        }
        int main(void) {
            int nested_arr[2][3] = {{1, 2, 3}, {4, 5, 6}};
            if (!read_nested(nested_arr, 1, 2, 6)) {
                return 1;
            }
            write_nested(nested_arr, 1, 2, -1);
            if (nested_arr[1][2] != -1) {
                return 2;
            }
            if (!read_nested_negated(nested_arr + 2, 2, 0, 1)) {
                return 3;
            }
            int *ptr = (nested_arr[0]) + 1;
            if (!get_nested_addr(nested_arr, 0, 1, ptr)) {
                return 4;
            }
            if (!read_static_nested(1, 1, 0, 5)) {
                return 5;
            }
            write_nested_complex(0, 2, 3, 111);
            if (get_array()[0][2][3] != 111) {
                return 6;
            }
            int *row_1 = get_subarray(nested_arr, 1);
            if (row_1 + 1 != &nested_arr[1][1]) {
                return 7;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [read_nested]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── nested_arr
            │   │   │   ╰── Type
            │   │   │       ╰── Array
            │   │   │           ├── 2
            │   │   │           ╰── Array
            │   │   │               ├── 3
            │   │   │               ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── i
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── j
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── expected
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <33>  [==]
            │               ├── <28> Subscript
            │               │   ├── <25> Subscript
            │               │   │   ├── <22> Var [nested_arr]
            │               │   │   ╰── <24> Var [i]
            │               │   ╰── <27> Var [j]
            │               ╰── <31> Var [expected]
            ├── Function [write_nested]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── nested_arr
            │   │   │   ╰── Type
            │   │   │       ╰── Array
            │   │   │           ├── 2
            │   │   │           ╰── Array
            │   │   │               ├── 3
            │   │   │               ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── i
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── j
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── new_val
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ├── <68> Assign [=]
            │       │   ├── <64> Subscript
            │       │   │   ├── <61> Subscript
            │       │   │   │   ├── <58> Var [nested_arr]
            │       │   │   │   ╰── <60> Var [i]
            │       │   │   ╰── <63> Var [j]
            │       │   ╰── <67> Var [new_val]
            │       ╰── Return
            │           ╰── <70> Constant Int [0]
            ├── Function [read_nested_negated]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── nested_arr
            │   │   │   ╰── Type
            │   │   │       ╰── Pointer
            │   │   │           ╰── Array
            │   │   │               ├── 3
            │   │   │               ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── i
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── j
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── expected
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <108>  [==]
            │               ├── <103> Subscript
            │               │   ├── <100> Subscript
            │               │   │   ├── <95> Var [nested_arr]
            │               │   │   ╰── <99> Unary [-]
            │               │   │       ╰── <98> Var [i]
            │               │   ╰── <102> Var [j]
            │               ╰── <106> Var [expected]
            ├── Function [get_nested_addr]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── nested_arr
            │   │   │   ╰── Type
            │   │   │       ╰── Array
            │   │   │           ├── 2
            │   │   │           ╰── Array
            │   │   │               ├── 3
            │   │   │               ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── i
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── j
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── expected
            │   │       ╰── Type
            │   │           ╰── Pointer
            │   │               ╰── Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <145>  [==]
            │               ├── <141> AddressOf
            │               │   ╰── <140> Subscript
            │               │       ├── <137> Subscript
            │               │       │   ├── <134> Var [nested_arr]
            │               │       │   ╰── <136> Var [i]
            │               │       ╰── <139> Var [j]
            │               ╰── <144> Var [expected]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── nested_arr
            │   ├── Type
            │   │   ╰── Array
            │   │       ├── 4
            │   │       ╰── Array
            │   │           ├── 3
            │   │           ╰── Array
            │   │               ├── 5
            │   │               ╰── Int
            │   ├── Initializer
            │   │   ╰── Compound
            │   │       ├── Compound
            │   │       │   ├── Compound
            │   │       │   │   ├── <159> Constant Int [1]
            │   │       │   │   ╰── <161> Constant Int [2]
            │   │       │   ╰── Compound
            │   │       │       ╰── <164> Constant Int [3]
            │   │       ╰── Compound
            │   │           ├── Compound
            │   │           │   ╰── <168> Constant Int [4]
            │   │           ╰── Compound
            │   │               ╰── <171> Constant Int [5]
            │   ╰── Static
            ├── Function [read_static_nested]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── i
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── j
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── k
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── expected
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <207>  [==]
            │               ├── <203> Subscript
            │               │   ├── <200> Subscript
            │               │   │   ├── <197> Subscript
            │               │   │   │   ├── <194> Var [nested_arr]
            │               │   │   │   ╰── <196> Var [i]
            │               │   │   ╰── <199> Var [j]
            │               │   ╰── <202> Var [k]
            │               ╰── <206> Var [expected]
            ├── Function [get_array]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <222> Var [nested_arr]
            ├── Function [write_nested_complex]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── i
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── j
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── k
            │   │   │   ╰── Type
            │   │   │       ╰── Int
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── val
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ├── <256> Assign [=]
            │       │   ├── <252> Subscript
            │       │   │   ├── <249> Subscript
            │       │   │   │   ├── <246> Subscript
            │       │   │   │   │   ├── <243> FunctionCall [get_array]
            │       │   │   │   │   ╰── <245> Var [i]
            │       │   │   │   ╰── <248> Var [j]
            │       │   │   ╰── <251> Var [k]
            │       │   ╰── <255> Var [val]
            │       ╰── Return
            │           ╰── <258> Constant Int [0]
            ├── Function [get_subarray]
            │   ├── Parameters
            │   │   ├── Param
            │   │   │   ├── Name
            │   │   │   │   ╰── nested
            │   │   │   ╰── Type
            │   │   │       ╰── Array
            │   │   │           ├── 2
            │   │   │           ╰── Array
            │   │   │               ├── 3
            │   │   │               ╰── Int
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── i
            │   │       ╰── Type
            │   │           ╰── Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <281> Subscript
            │               ├── <278> Var [nested]
            │               ╰── <280> Var [i]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── nested_arr
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 2
                    │   │       ╰── Array
                    │   │           ├── 3
                    │   │           ╰── Int
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── Compound
                    │           │   ├── <296> Constant Int [1]
                    │           │   ├── <298> Constant Int [2]
                    │           │   ╰── <300> Constant Int [3]
                    │           ╰── Compound
                    │               ├── <303> Constant Int [4]
                    │               ├── <305> Constant Int [5]
                    │               ╰── <307> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <320> Unary [!]
                    │   │       ╰── <319> FunctionCall [read_nested]
                    │   │           ├── <315> Var [nested_arr]
                    │   │           ├── <316> Constant Int [1]
                    │   │           ├── <317> Constant Int [2]
                    │   │           ╰── <318> Constant Int [6]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <321> Constant Int [1]
                    ├── <334> FunctionCall [write_nested]
                    │   ├── <328> Var [nested_arr]
                    │   ├── <329> Constant Int [1]
                    │   ├── <330> Constant Int [2]
                    │   ╰── <333> Unary [-]
                    │       ╰── <332> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <346>  [!=]
                    │   │       ├── <341> Subscript
                    │   │       │   ├── <339> Subscript
                    │   │       │   │   ├── <337> Var [nested_arr]
                    │   │       │   │   ╰── <338> Constant Int [1]
                    │   │       │   ╰── <340> Constant Int [2]
                    │   │       ╰── <345> Unary [-]
                    │   │           ╰── <344> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <347> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <363> Unary [!]
                    │   │       ╰── <362> FunctionCall [read_nested_negated]
                    │   │           ├── <358>  [+]
                    │   │           │   ├── <355> Var [nested_arr]
                    │   │           │   ╰── <357> Constant Int [2]
                    │   │           ├── <359> Constant Int [2]
                    │   │           ├── <360> Constant Int [0]
                    │   │           ╰── <361> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <364> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <380>  [+]
                    │           ├── <377> Subscript
                    │           │   ├── <374> Var [nested_arr]
                    │           │   ╰── <375> Constant Int [0]
                    │           ╰── <379> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <392> Unary [!]
                    │   │       ╰── <391> FunctionCall [get_nested_addr]
                    │   │           ├── <386> Var [nested_arr]
                    │   │           ├── <387> Constant Int [0]
                    │   │           ├── <388> Constant Int [1]
                    │   │           ╰── <390> Var [ptr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <393> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <405> Unary [!]
                    │   │       ╰── <404> FunctionCall [read_static_nested]
                    │   │           ├── <400> Constant Int [1]
                    │   │           ├── <401> Constant Int [1]
                    │   │           ├── <402> Constant Int [0]
                    │   │           ╰── <403> Constant Int [5]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <406> Constant Int [5]
                    ├── <416> FunctionCall [write_nested_complex]
                    │   ├── <412> Constant Int [0]
                    │   ├── <413> Constant Int [2]
                    │   ├── <414> Constant Int [3]
                    │   ╰── <415> Constant Int [111]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <428>  [!=]
                    │   │       ├── <425> Subscript
                    │   │       │   ├── <423> Subscript
                    │   │       │   │   ├── <421> Subscript
                    │   │       │   │   │   ├── <419> FunctionCall [get_array]
                    │   │       │   │   │   ╰── <420> Constant Int [0]
                    │   │       │   │   ╰── <422> Constant Int [2]
                    │   │       │   ╰── <424> Constant Int [3]
                    │   │       ╰── <427> Constant Int [111]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <429> Constant Int [6]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── row_1
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <442> FunctionCall [get_subarray]
                    │           ├── <440> Var [nested_arr]
                    │           ╰── <441> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <458>  [!=]
                    │   │       ├── <449>  [+]
                    │   │       │   ├── <446> Var [row_1]
                    │   │       │   ╰── <448> Constant Int [1]
                    │   │       ╰── <457> AddressOf
                    │   │           ╰── <456> Subscript
                    │   │               ├── <454> Subscript
                    │   │               │   ├── <452> Var [nested_arr]
                    │   │               │   ╰── <453> Constant Int [1]
                    │   │               ╰── <455> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <459> Constant Int [7]
                    ╰── Return
                        ╰── <464> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_subscripting_subscript_pointer() {
    let src = r#"
        int subscript_pointer_to_pointer(int **x) {
            return x[0][0];
        }
        int main(void) {
            int a = 3;
            int *ptr = &a;
            if (ptr[0] != 3) {
                return 1;
            }
            int **ptr_ptr = &ptr;
            if (ptr_ptr[0][0] != 3) {
                return 2;
            }
            int dereferenced = subscript_pointer_to_pointer(ptr_ptr);
            if (dereferenced != 3) {
                return 3;
            }
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ├── Function [subscript_pointer_to_pointer]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── x
            │   │       ╰── Type
            │   │           ╰── Pointer
            │   │               ╰── Pointer
            │   │                   ╰── Int
            │   ╰── Body
            │       ╰── Return
            │           ╰── <15> Subscript
            │               ├── <13> Subscript
            │               │   ├── <11> Var [x]
            │               │   ╰── <12> Constant Int [0]
            │               ╰── <14> Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <26> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <35> AddressOf
                    │           ╰── <34> Var [a]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <44>  [!=]
                    │   │       ├── <41> Subscript
                    │   │       │   ├── <39> Var [ptr]
                    │   │       │   ╰── <40> Constant Int [0]
                    │   │       ╰── <43> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <45> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Pointer
                    │   │           ╰── Int
                    │   ╰── Initializer
                    │       ╰── <57> AddressOf
                    │           ╰── <56> Var [ptr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <68>  [!=]
                    │   │       ├── <65> Subscript
                    │   │       │   ├── <63> Subscript
                    │   │       │   │   ├── <61> Var [ptr_ptr]
                    │   │       │   │   ╰── <62> Constant Int [0]
                    │   │       │   ╰── <64> Constant Int [0]
                    │   │       ╰── <67> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <69> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── dereferenced
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <80> FunctionCall [subscript_pointer_to_pointer]
                    │           ╰── <79> Var [ptr_ptr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <87>  [!=]
                    │   │       ├── <84> Var [dereferenced]
                    │   │       ╰── <86> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <88> Constant Int [3]
                    ╰── Return
                        ╰── <93> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_subscripting_subscript_precedence() {
    let src = r#"
        int main(void) {
            int arr[3] = {1, 2, 3};
            return (-arr[2] == -3);
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
                    ╰── Return
                        ╰── <29>  [==]
                            ├── <23> Unary [-]
                            │   ╰── <22> Subscript
                            │       ├── <20> Var [arr]
                            │       ╰── <21> Constant Int [2]
                            ╰── <27> Unary [-]
                                ╰── <26> Constant Int [3]
    "#;
    assert_parse(src, expected);
}
