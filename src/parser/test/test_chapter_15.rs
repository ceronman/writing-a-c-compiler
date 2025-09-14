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
                    │       ╰── <11> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── y
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <19> Constant Int [0]
                    ╰── Return
                        ╰── <31>  [==]
                            ├── <27>  [+]
                            │   ├── <23> Var [x]
                            │   ╰── <26> Var [y]
                            ╰── <29> Constant Int [0]
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
                    │           ├── <12> Constant Int [1]
                    │           ├── <14> Constant Int [2]
                    │           ├── <16> Constant Int [3]
                    │           ╰── <18> Constant Int [4]
                    ╰── VarDeclaration
                        ├── Name
                        │   ╰── arr
                        ├── Type
                        │   ╰── Pointer
                        │       ╰── Array
                        │           ├── 3
                        │           ╰── Int
                        ╰── Initializer
                            ╰── <33> AddressOf
                                ╰── <32> Var [four_element_array]
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
                    │           ├── <12> Constant Int [1]
                    │           ├── <14> Constant Int [2]
                    │           ╰── <16> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── arr2
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 3
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <26> Constant Int [4]
                    │           ├── <28> Constant Int [5]
                    │           ╰── <30> Constant Int [6]
                    ├── <39> Assign [=]
                    │   ├── <35> Var [arr]
                    │   ╰── <38> Var [arr2]
                    ╰── Return
                        ╰── <44> Subscript
                            ├── <42> Var [arr]
                            ╰── <43> Constant Int [0]
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
                    │               ├── <15> Constant Int [1]
                    │               ╰── <17> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── dim
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 2
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <28> Constant Int [3]
                    │           ╰── <30> Constant Int [4]
                    ├── <41> Assign [=]
                    │   ├── <37> Subscript
                    │   │   ├── <35> Var [dim2]
                    │   │   ╰── <36> Constant Int [0]
                    │   ╰── <40> Var [dim]
                    ╰── Return
                        ╰── <46> Subscript
                            ├── <44> Var [dim]
                            ╰── <45> Constant Int [0]
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
                    │           ├── <12> Constant Int [1]
                    │           ├── <14> Constant Int [2]
                    │           ╰── <16> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr_to_array
                    │   ╰── Type
                    │       ╰── Pointer
                    │           ╰── Array
                    │               ├── 3
                    │               ╰── Int
                    ╰── <36> Assign [=]
                        ├── <32> Dereference
                        │   ╰── <31> Var [ptr_to_array]
                        ╰── <35> Var [arr]
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
            │           ╰── <18> Subscript
            │               ├── <16> Subscript
            │               │   ├── <14> Var [x]
            │               │   ╰── <15> Constant Int [0]
            │               ╰── <17> Constant Int [0]
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
                    │           ╰── <33> Constant Int [10]
                    ╰── Return
                        ╰── <41> FunctionCall [foo]
                            ╰── <40> AddressOf
                                ╰── <39> Var [arr]
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
                        ╰── <20> Cast
                            ├── Target
                            │   ╰── Array
                            │       ├── 10
                            │       ╰── Int
                            ╰── Expression
                                ╰── <19> Var [arr]
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
                        ╰── <21> Cast
                            ├── Target
                            │   ╰── Array
                            │       ├── 10
                            │       ╰── Pointer
                            │           ╰── Int
                            ╰── Expression
                                ╰── <20> Var [arr]
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
                        ╰── <25> Cast
                            ├── Target
                            │   ╰── Array
                            │       ├── 2
                            │       ╰── Array
                            │           ├── 3
                            │           ╰── Long
                            ╰── Expression
                                ╰── <23> Var [arr]
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
                    │       ╰── <9> Constant Int [10]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Long
                    │   ╰── Initializer
                    │       ╰── <22>  [+]
                    │           ├── <19> AddressOf
                    │           │   ╰── <18> Var [x]
                    │           ╰── <21> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── array_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Array
                    │   │           ├── 10
                    │   │           ╰── Long
                    │   ╰── Initializer
                    │       ╰── <43> Cast
                    │           ├── Target
                    │           │   ╰── Pointer
                    │           │       ╰── Array
                    │           │           ├── 10
                    │           │           ╰── Long
                    │           ╰── Expression
                    │               ╰── <42> AddressOf
                    │                   ╰── <41> Var [x]
                    ╰── Return
                        ╰── <51>  [<]
                            ├── <47> Var [array_ptr]
                            ╰── <50> Var [ptr]
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
                        ╰── <19>  [==]
                            ├── <14> Var [arr]
                            ╰── <18> AddressOf
                                ╰── <17> Var [arr]
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
                    │       ╰── <11> Constant Int [0]
                    ╰── Return
                        ╰── <18>  [<=]
                            ├── <15> Var [l]
                            ╰── <17> Constant ULong [100]
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
                    │       ╰── <11> Constant Int [0]
                    ╰── Return
                        ╰── <18>  [>]
                            ├── <15> Var [x]
                            ╰── <17> Constant Int [0]
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
                    │           ├── <9> Constant Int [1]
                    │           ├── <11> Constant Int [2]
                    │           ╰── <13> Constant Int [3]
                    ╰── Return
                        ╰── <18> Var [x]
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
                    │   │       ├── <10> Constant Int [1]
                    │   │       ├── <12> Constant Int [2]
                    │   │       ╰── <14> Constant Int [3]
                    │   ╰── Static
                    ╰── Return
                        ╰── <19> Var [x]
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
                    │   │       ├── <13> Constant Int [1]
                    │   │       ├── <15> Constant Int [2]
                    │   │       ├── <17> Constant Int [3]
                    │   │       ╰── <19> Constant Int [4]
                    │   ╰── Static
                    ╰── Return
                        ╰── <26> Subscript
                            ├── <24> Var [arr]
                            ╰── <25> Constant Int [2]
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
                    │           ├── <12> Constant Int [1]
                    │           ├── <14> Constant Int [2]
                    │           ├── <16> Constant Int [3]
                    │           ╰── <18> Constant Int [4]
                    ╰── Return
                        ╰── <25> Subscript
                            ├── <23> Var [arr]
                            ╰── <24> Constant Int [2]
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
            │           ╰── <16> Subscript
            │               ├── <14> Var [arr]
            │               ╰── <15> Constant Int [0]
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
                    │           ├── <12> Constant Int [4]
                    │           ├── <14> Constant Int [5]
                    │           ╰── <16> Constant Int [6]
                    ╰── Return
                        ╰── <23> Subscript
                            ├── <21> Var [arr]
                            ╰── <22> Constant Double [+2e0]
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
                    │           ├── <12> Constant Int [1]
                    │           ├── <14> Constant Int [2]
                    │           ╰── <16> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── elem
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <26> Var [arr]
                    ├── <33> Assign [+=]
                    │   ├── <30> Var [elem]
                    │   ╰── <32> Constant Double [+1e0]
                    ╰── Return
                        ╰── <35> Constant Int [0]
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
                    │           ├── <12> Constant Int [1]
                    │           ├── <14> Constant Int [2]
                    │           ╰── <16> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── elem0
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <26> Var [arr]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── elem1
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <38>  [+]
                    │           ├── <35> Var [arr]
                    │           ╰── <37> Constant Int [1]
                    ├── <46> Assign [+=]
                    │   ├── <42> Var [elem0]
                    │   ╰── <45> Var [elem1]
                    ╰── Return
                        ╰── <48> Constant Int [0]
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
                    │           ├── <12> Constant Int [1]
                    │           ├── <14> Constant Int [2]
                    │           ╰── <16> Constant Int [3]
                    ├── <24> Assign [-=]
                    │   ├── <21> Var [arr]
                    │   ╰── <23> Constant Int [1]
                    ╰── <26> Constant Int [0]
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
                    │           │   ├── <15> Constant Int [1]
                    │           │   ╰── <17> Constant Int [2]
                    │           ╰── Compound
                    │               ├── <20> Constant Int [3]
                    │               ╰── <22> Constant Int [4]
                    ├── <33> Assign [+=]
                    │   ├── <30> Subscript
                    │   │   ├── <28> Var [arr]
                    │   │   ╰── <29> Constant Int [1]
                    │   ╰── <32> Constant Int [1]
                    ╰── Return
                        ╰── <35> Constant Int [0]
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
                    │           ├── <12> Constant Int [1]
                    │           ├── <14> Constant Int [2]
                    │           ╰── <16> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── elem
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <29>  [+]
                    │           ├── <26> Var [arr]
                    │           ╰── <28> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <35> Constant Int [0]
                    ├── <43> Assign [-=]
                    │   ├── <39> Var [i]
                    │   ╰── <42> Var [elem]
                    ╰── Return
                        ╰── <45> Constant Int [0]
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
                    │           ├── <12> Constant Int [1]
                    │           ├── <14> Constant Int [2]
                    │           ╰── <16> Constant Int [3]
                    ├── <23> Postfix [++]
                    │   ╰── <21> Var [arr]
                    ╰── Return
                        ╰── <25> Constant Int [0]
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
                    │           │   ├── <15> Constant Int [1]
                    │           │   ├── <17> Constant Int [2]
                    │           │   ╰── <19> Constant Int [3]
                    │           ╰── Compound
                    │               ├── <22> Constant Int [4]
                    │               ├── <24> Constant Int [5]
                    │               ╰── <26> Constant Int [6]
                    ├── <36> Postfix [++]
                    │   ╰── <34> Subscript
                    │       ├── <32> Var [arr]
                    │       ╰── <33> Constant Int [2]
                    ╰── Return
                        ╰── <38> Constant Int [0]
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
                    │           ├── <12> Constant Int [1]
                    │           ├── <14> Constant Int [2]
                    │           ╰── <16> Constant Int [3]
                    ├── <23> Unary [--]
                    │   ╰── <22> Var [arr]
                    ╰── Return
                        ╰── <25> Constant Int [0]
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
                    │           │   ├── <15> Constant Int [1]
                    │           │   ├── <17> Constant Int [2]
                    │           │   ╰── <19> Constant Int [3]
                    │           ╰── Compound
                    │               ├── <22> Constant Int [4]
                    │               ├── <24> Constant Int [5]
                    │               ╰── <26> Constant Int [6]
                    ├── <36> Unary [--]
                    │   ╰── <35> Subscript
                    │       ├── <33> Var [arr]
                    │       ╰── <34> Constant Int [2]
                    ╰── Return
                        ╰── <38> Constant Int [0]
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
                    │           ├── <12> Constant Int [1]
                    │           ├── <14> Constant Int [2]
                    │           ╰── <16> Constant Int [3]
                    ├── Switch
                    │   ├── Expression
                    │   │   ╰── <21> Var [arr]
                    │   ╰── Block
                    │       ╰── Default
                    │           ╰── Return
                    │               ╰── <22> Constant Int [0]
                    ╰── Return
                        ╰── <28> Constant Int [1]
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
                                ├── <14> Constant Int [0]
                                ├── <16> Constant Int [0]
                                ╰── <18> Constant Double [+1e0]
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
            │           ├── <9> Constant Int [0]
            │           ├── <11> Constant Int [0]
            │           ╰── <13> Constant Double [+1e0]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <22> Constant Int [0]
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
                    │       ╰── <12> Constant Int [0]
                    ╰── Return
                        ╰── <18> Subscript
                            ├── <16> Var [arr]
                            ╰── <17> Constant Int [0]
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
                    │   │   ╰── <13> Constant Int [0]
                    │   ╰── Static
                    ╰── Return
                        ╰── <19> Subscript
                            ├── <17> Var [arr]
                            ╰── <18> Constant Int [0]
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
                    │       ╰── <12> Constant Int [4]
                    ╰── Return
                        ╰── <18> Subscript
                            ├── <16> Var [arr]
                            ╰── <17> Constant Int [0]
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
            │       ╰── <7> Constant Double [+1e0]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <15> Constant Int [0]
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
            │       │   │       ├── <17> Var [p]
            │       │   │       ├── <23>  [+]
            │       │   │       │   ├── <20> Var [p]
            │       │   │       │   ╰── <22> Constant Int [1]
            │       │   │       ╰── <25> Constant Int [0]
            │       │   ╰── Static
            │       ╰── Return
            │           ╰── <32> Subscript
            │               ├── <30> Var [arr]
            │               ╰── <31> Constant Int [2]
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <43> FunctionCall [foo]
                            ╰── <42> Constant Int [5]
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
                    │       ╰── <19> Var [x]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr2
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <32> Cast
                    │           ├── Target
                    │           │   ╰── Pointer
                    │           │       ╰── Unsigned Long
                    │           ╰── Expression
                    │               ╰── <31> Var [ptr]
                    ╰── Return
                        ╰── <40>  [-]
                            ├── <36> Var [ptr]
                            ╰── <39> Var [ptr2]
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
                    │       ╰── <11> Constant Int [0]
                    ╰── Return
                        ╰── <22>  [==]
                            ├── <18>  [-]
                            │   ├── <15> Var [y]
                            │   ╰── <17> Constant Double [+0e0]
                            ╰── <20> Constant Double [+0e0]
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
                    │       ╰── <11> Constant Int [0]
                    ╰── Return
                        ╰── <21>  [==]
                            ├── <18>  [-]
                            │   ├── <14> Constant Int [0]
                            │   ╰── <17> Var [x]
                            ╰── <20> Constant Int [0]
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
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── subscript
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <27> Constant Int [0]
                    ╰── Return
                        ╰── <34> Subscript
                            ├── <31> Var [ptr]
                            ╰── <33> Var [subscript]
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
                    │       ╰── <9> Constant Int [3]
                    ╰── Return
                        ╰── <15> Subscript
                            ├── <13> Var [a]
                            ╰── <14> Constant Int [4]
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
            │       │       ╰── <18> Cast
            │       │           ├── Target
            │       │           │   ╰── Unsigned Long
            │       │           ╰── Expression
            │       │               ╰── <17> Var [ptr]
            │       ╰── Return
            │           ╰── <29>  [==]
            │               ├── <25>  [%]
            │               │   ├── <22> Var [addr]
            │               │   ╰── <24> Constant Int [16]
            │               ╰── <27> Constant Int [0]
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
                    │           ╰── <44> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── arr2
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 7
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ╰── <54> Constant Int [0]
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
                    │               ╰── <67> Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <77> Unary [!]
                    │   │       ╰── <76> FunctionCall [check_alignment]
                    │   │           ╰── <75> Var [arr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <78> Constant Int [1]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <86> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <94>  [<]
                    │   │       ├── <91> Var [i]
                    │   │       ╰── <93> Constant Int [5]
                    │   ├── Condition
                    │   │   ╰── <103> Assign [=]
                    │   │       ├── <96> Var [i]
                    │   │       ╰── <102>  [+]
                    │   │           ├── <99> Var [i]
                    │   │           ╰── <101> Constant Int [1]
                    │   ╰── <112> Assign [=]
                    │       ├── <108> Subscript
                    │       │   ├── <105> Var [arr]
                    │       │   ╰── <107> Var [i]
                    │       ╰── <111> Var [i]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <120> Unary [!]
                    │   │       ╰── <119> FunctionCall [check_alignment]
                    │   │           ╰── <118> Var [arr2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <121> Constant Int [2]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <129> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <137>  [<]
                    │   │       ├── <134> Var [i]
                    │   │       ╰── <136> Constant Int [7]
                    │   ├── Condition
                    │   │   ╰── <146> Assign [=]
                    │   │       ├── <139> Var [i]
                    │   │       ╰── <145>  [+]
                    │   │           ├── <142> Var [i]
                    │   │           ╰── <144> Constant Int [1]
                    │   ╰── If
                    │       ├── Condition
                    │       │   ╰── <151> Subscript
                    │       │       ├── <148> Var [arr2]
                    │       │       ╰── <150> Var [i]
                    │       ╰── Then
                    │           ╰── Return
                    │               ╰── <152> Constant Int [3]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <159> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <167>  [<]
                    │   │       ├── <164> Var [i]
                    │   │       ╰── <166> Constant Int [7]
                    │   ├── Condition
                    │   │   ╰── <176> Assign [=]
                    │   │       ├── <169> Var [i]
                    │   │       ╰── <175>  [+]
                    │   │           ├── <172> Var [i]
                    │   │           ╰── <174> Constant Int [1]
                    │   ╰── Block
                    │       ╰── <187> Assign [=]
                    │           ├── <181> Subscript
                    │           │   ├── <178> Var [arr2]
                    │           │   ╰── <180> Var [i]
                    │           ╰── <186> Unary [-]
                    │               ╰── <185> Var [i]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <201> Unary [!]
                    │   │       ╰── <200> FunctionCall [check_alignment]
                    │   │           ╰── <199> Cast
                    │   │               ├── Target
                    │   │               │   ╰── Pointer
                    │   │               │       ╰── Int
                    │   │               ╰── Expression
                    │   │                   ╰── <198> Var [arr3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <202> Constant Int [4]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <210> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <218>  [<]
                    │   │       ├── <215> Var [i]
                    │   │       ╰── <217> Constant Int [5]
                    │   ├── Condition
                    │   │   ╰── <227> Assign [=]
                    │   │       ├── <220> Var [i]
                    │   │       ╰── <226>  [+]
                    │   │           ├── <223> Var [i]
                    │   │           ╰── <225> Constant Int [1]
                    │   ╰── Block
                    │       ╰── If
                    │           ├── Condition
                    │           │   ╰── <236>  [!=]
                    │           │       ├── <232> Subscript
                    │           │       │   ├── <229> Var [arr]
                    │           │       │   ╰── <231> Var [i]
                    │           │       ╰── <235> Var [i]
                    │           ╰── Then
                    │               ╰── Block
                    │                   ╰── Return
                    │                       ╰── <237> Constant Int [5]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <248> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <256>  [<]
                    │   │       ├── <253> Var [i]
                    │   │       ╰── <255> Constant Int [2]
                    │   ├── Condition
                    │   │   ╰── <265> Assign [=]
                    │   │       ├── <258> Var [i]
                    │   │       ╰── <264>  [+]
                    │   │           ├── <261> Var [i]
                    │   │           ╰── <263> Constant Int [1]
                    │   ╰── For
                    │       ├── Init
                    │       │   ╰── VarDeclaration
                    │       │       ├── Name
                    │       │       │   ╰── j
                    │       │       ├── Type
                    │       │       │   ╰── Int
                    │       │       ╰── Initializer
                    │       │           ╰── <269> Constant Int [0]
                    │       ├── Condition
                    │       │   ╰── <277>  [<]
                    │       │       ├── <274> Var [j]
                    │       │       ╰── <276> Constant Int [2]
                    │       ├── Condition
                    │       │   ╰── <286> Assign [=]
                    │       │       ├── <279> Var [j]
                    │       │       ╰── <285>  [+]
                    │       │           ├── <282> Var [j]
                    │       │           ╰── <284> Constant Int [1]
                    │       ╰── If
                    │           ├── Condition
                    │           │   ╰── <297>  [!=]
                    │           │       ├── <294> Subscript
                    │           │       │   ├── <291> Subscript
                    │           │       │   │   ├── <288> Var [arr3]
                    │           │       │   │   ╰── <290> Var [i]
                    │           │       │   ╰── <293> Var [j]
                    │           │       ╰── <296> Constant Int [0]
                    │           ╰── Then
                    │               ╰── Return
                    │                   ╰── <298> Constant Int [6]
                    ╰── Return
                        ╰── <303> Constant Int [0]
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
                    │           ├── <12> Constant Int [1]
                    │           ╰── <14> Constant Int [2]
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
                    │           ├── <32> AddressOf
                    │           │   ╰── <31> Var [simple_array]
                    │           ├── <34> Constant Int [0]
                    │           ╰── <38> AddressOf
                    │               ╰── <37> Var [simple_array]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── other_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Long
                    │   ╰── Initializer
                    │       ╰── <52> Cast
                    │           ├── Target
                    │           │   ╰── Pointer
                    │           │       ╰── Long
                    │           ╰── Expression
                    │               ╰── <51> Var [ptr_arr]
                    ╰── Return
                        ╰── <68>  [==]
                            ├── <64> Cast
                            │   ├── Target
                            │   │   ╰── Pointer
                            │   │       ╰── Pointer
                            │   │           ╰── Array
                            │   │               ├── 2
                            │   │               ╰── Int
                            │   ╰── Expression
                            │       ╰── <63> Var [other_ptr]
                            ╰── <67> Var [ptr_arr]
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
                    │           ├── <14> Unary [-]
                    │           │   ╰── <13> Constant Int [1]
                    │           ├── <18> Unary [-]
                    │           │   ╰── <17> Constant Int [2]
                    │           ├── <22> Unary [-]
                    │           │   ╰── <21> Constant Int [3]
                    │           ╰── <26> Unary [-]
                    │               ╰── <25> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <39>  [!=]
                    │   │       ├── <31> Var [arr]
                    │   │       ╰── <38> Cast
                    │   │           ├── Target
                    │   │           │   ╰── Pointer
                    │   │           │       ╰── Long
                    │   │           ╰── Expression
                    │   │               ╰── <37> Var [arr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <40> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <58>  [!=]
                    │   │       ├── <53> Cast
                    │   │       │   ├── Target
                    │   │       │   │   ╰── Pointer
                    │   │       │   │       ╰── Array
                    │   │       │   │           ├── 4
                    │   │       │   │           ╰── Long
                    │   │       │   ╰── Expression
                    │   │       │       ╰── <52> Var [arr]
                    │   │       ╰── <57> AddressOf
                    │   │           ╰── <56> Var [arr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <59> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── unsigned_arr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <74> Cast
                    │           ├── Target
                    │           │   ╰── Pointer
                    │           │       ╰── Unsigned Long
                    │           ╰── Expression
                    │               ╰── <73> Var [arr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <83>  [!=]
                    │   │       ├── <80> Subscript
                    │   │       │   ├── <78> Var [unsigned_arr]
                    │   │       │   ╰── <79> Constant Int [0]
                    │   │       ╰── <82> Constant ULong [18446744073709551615]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <84> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <95>  [!=]
                    │   │       ├── <92> Subscript
                    │   │       │   ├── <90> Var [unsigned_arr]
                    │   │       │   ╰── <91> Constant Int [3]
                    │   │       ╰── <94> Constant ULong [18446744073709551612]
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
                    │           │   ├── <15> Constant Int [0]
                    │           │   ├── <17> Constant Int [1]
                    │           │   ╰── <19> Constant Int [2]
                    │           ╰── Compound
                    │               ├── <22> Constant Int [3]
                    │               ├── <24> Constant Int [4]
                    │               ╰── <26> Constant Int [5]
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
                    │       ╰── <45> AddressOf
                    │           ╰── <44> Var [multi_dim]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── row_pointer
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Array
                    │   │           ├── 3
                    │   │           ╰── Int
                    │   ╰── Initializer
                    │       ╰── <65> Cast
                    │           ├── Target
                    │           │   ╰── Pointer
                    │           │       ╰── Array
                    │           │           ├── 3
                    │           │           ╰── Int
                    │           ╰── Expression
                    │               ╰── <64> Var [array_pointer]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <73>  [!=]
                    │   │       ├── <69> Var [row_pointer]
                    │   │       ╰── <72> Var [multi_dim]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <74> Constant Int [1]
                    ├── <87> Assign [=]
                    │   ├── <80> Var [row_pointer]
                    │   ╰── <86>  [+]
                    │       ├── <83> Var [row_pointer]
                    │       ╰── <85> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <97>  [!=]
                    │   │       ├── <94> Subscript
                    │   │       │   ├── <92> Subscript
                    │   │       │   │   ├── <90> Var [row_pointer]
                    │   │       │   │   ╰── <91> Constant Int [0]
                    │   │       │   ╰── <93> Constant Int [1]
                    │   │       ╰── <96> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <98> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── elem_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <113> Cast
                    │           ├── Target
                    │           │   ╰── Pointer
                    │           │       ╰── Int
                    │           ╰── Expression
                    │               ╰── <112> Var [row_pointer]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <121>  [!=]
                    │   │       ├── <118> Dereference
                    │   │       │   ╰── <117> Var [elem_ptr]
                    │   │       ╰── <120> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <122> Constant Int [3]
                    ├── <135> Assign [=]
                    │   ├── <128> Var [elem_ptr]
                    │   ╰── <134>  [+]
                    │       ├── <131> Var [elem_ptr]
                    │       ╰── <133> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <142>  [!=]
                    │   │       ├── <139> Dereference
                    │   │       │   ╰── <138> Var [elem_ptr]
                    │   │       ╰── <141> Constant Int [5]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <143> Constant Int [4]
                    ├── <156> Assign [=]
                    │   ├── <149> Var [row_pointer]
                    │   ╰── <155>  [-]
                    │       ├── <152> Var [row_pointer]
                    │       ╰── <154> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <172>  [!=]
                    │   │       ├── <168> Cast
                    │   │       │   ├── Target
                    │   │       │   │   ╰── Pointer
                    │   │       │   │       ╰── Array
                    │   │       │   │           ├── 2
                    │   │       │   │           ╰── Array
                    │   │       │   │               ├── 3
                    │   │       │   │               ╰── Int
                    │   │       │   ╰── Expression
                    │   │       │       ╰── <167> Var [row_pointer]
                    │   │       ╰── <171> Var [array_pointer]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <173> Constant Int [5]
                    ╰── Return
                        ╰── <178> Constant Int [0]
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
            │       ├── <18> Assign [=]
            │       │   ├── <15> Subscript
            │       │   │   ├── <13> Var [a]
            │       │   │   ╰── <14> Constant Int [4]
            │       │   ╰── <17> Constant Int [0]
            │       ╰── Return
            │           ╰── <20> Constant Int [0]
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
            │       ├── <46> Assign [=]
            │       │   ├── <43> Subscript
            │       │   │   ├── <41> Subscript
            │       │   │   │   ├── <39> Var [a]
            │       │   │   │   ╰── <40> Constant Int [1]
            │       │   │   ╰── <42> Constant Int [1]
            │       │   ╰── <45> Constant Int [1]
            │       ╰── Return
            │           ╰── <48> Constant Int [0]
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
            │       │           ├── <117> Constant Int [8]
            │       │           ├── <119> Constant Int [7]
            │       │           ├── <121> Constant Int [6]
            │       │           ├── <123> Constant Int [5]
            │       │           ├── <125> Constant Int [4]
            │       │           ├── <127> Constant Int [3]
            │       │           ├── <129> Constant Int [2]
            │       │           ╰── <131> Constant Int [1]
            │       ├── <138> FunctionCall [array_param]
            │       │   ╰── <137> Var [arr]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <143> Subscript
            │       │   │       ├── <141> Var [arr]
            │       │   │       ╰── <142> Constant Int [4]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <144> Constant Int [1]
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
            │       │   │       ╰── <159> Constant Int [8]
            │       │   ├── Condition
            │       │   │   ╰── <169> Assign [=]
            │       │   │       ├── <162> Var [i]
            │       │   │       ╰── <168>  [+]
            │       │   │           ├── <165> Var [i]
            │       │   │           ╰── <167> Constant Int [1]
            │       │   ╰── Block
            │       │       ╰── If
            │       │           ├── Condition
            │       │           │   ╰── <188>  [&&]
            │       │           │       ├── <174>  [!=]
            │       │           │       │   ├── <171> Var [i]
            │       │           │       │   ╰── <173> Constant Int [4]
            │       │           │       ╰── <187>  [!=]
            │       │           │           ├── <180> Subscript
            │       │           │           │   ├── <177> Var [arr]
            │       │           │           │   ╰── <179> Var [i]
            │       │           │           ╰── <186>  [-]
            │       │           │               ├── <182> Constant Int [8]
            │       │           │               ╰── <185> Var [i]
            │       │           ╰── Then
            │       │               ╰── Return
            │       │                   ╰── <189> Constant Int [2]
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
            │       │           │   ├── <206> Unary [-]
            │       │           │   │   ╰── <205> Constant Int [1]
            │       │           │   ├── <210> Unary [-]
            │       │           │   │   ╰── <209> Constant Int [1]
            │       │           │   ╰── <214> Unary [-]
            │       │           │       ╰── <213> Constant Int [1]
            │       │           ├── Compound
            │       │           │   ├── <219> Unary [-]
            │       │           │   │   ╰── <218> Constant Int [2]
            │       │           │   ├── <223> Unary [-]
            │       │           │   │   ╰── <222> Constant Int [2]
            │       │           │   ╰── <227> Unary [-]
            │       │           │       ╰── <226> Constant Int [2]
            │       │           ├── Compound
            │       │           │   ├── <232> Unary [-]
            │       │           │   │   ╰── <231> Constant Int [3]
            │       │           │   ├── <236> Unary [-]
            │       │           │   │   ╰── <235> Constant Int [3]
            │       │           │   ╰── <240> Unary [-]
            │       │           │       ╰── <239> Constant Int [3]
            │       │           ╰── Compound
            │       │               ├── <245> Unary [-]
            │       │               │   ╰── <244> Constant Int [4]
            │       │               ├── <249> Unary [-]
            │       │               │   ╰── <248> Constant Int [4]
            │       │               ╰── <253> Unary [-]
            │       │                   ╰── <252> Constant Int [4]
            │       ├── <261> FunctionCall [nested_array_param]
            │       │   ╰── <260> Var [nested_arr]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <271>  [!=]
            │       │   │       ├── <268> Subscript
            │       │   │       │   ├── <266> Subscript
            │       │   │       │   │   ├── <264> Var [nested_arr]
            │       │   │       │   │   ╰── <265> Constant Int [1]
            │       │   │       │   ╰── <267> Constant Int [1]
            │       │   │       ╰── <270> Constant Int [1]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <272> Constant Int [3]
            │       ├── For
            │       │   ├── Init
            │       │   │   ╰── VarDeclaration
            │       │   │       ├── Name
            │       │   │       │   ╰── i
            │       │   │       ├── Type
            │       │   │       │   ╰── Int
            │       │   │       ╰── Initializer
            │       │   │           ╰── <280> Constant Int [0]
            │       │   ├── Condition
            │       │   │   ╰── <288>  [<]
            │       │   │       ├── <285> Var [i]
            │       │   │       ╰── <287> Constant Int [4]
            │       │   ├── Condition
            │       │   │   ╰── <297> Assign [=]
            │       │   │       ├── <290> Var [i]
            │       │   │       ╰── <296>  [+]
            │       │   │           ├── <293> Var [i]
            │       │   │           ╰── <295> Constant Int [1]
            │       │   ╰── Block
            │       │       ├── VarDeclaration
            │       │       │   ├── Name
            │       │       │   │   ╰── expected
            │       │       │   ├── Type
            │       │       │   │   ╰── Int
            │       │       │   ╰── Initializer
            │       │       │       ╰── <307>  [-]
            │       │       │           ├── <303> Unary [-]
            │       │       │           │   ╰── <302> Constant Int [1]
            │       │       │           ╰── <306> Var [i]
            │       │       ╰── For
            │       │           ├── Init
            │       │           │   ╰── VarDeclaration
            │       │           │       ├── Name
            │       │           │       │   ╰── j
            │       │           │       ├── Type
            │       │           │       │   ╰── Int
            │       │           │       ╰── Initializer
            │       │           │           ╰── <313> Constant Int [0]
            │       │           ├── Condition
            │       │           │   ╰── <321>  [<]
            │       │           │       ├── <318> Var [j]
            │       │           │       ╰── <320> Constant Int [3]
            │       │           ├── Condition
            │       │           │   ╰── <330> Assign [=]
            │       │           │       ├── <323> Var [j]
            │       │           │       ╰── <329>  [+]
            │       │           │           ├── <326> Var [j]
            │       │           │           ╰── <328> Constant Int [1]
            │       │           ╰── Block
            │       │               ╰── If
            │       │                   ├── Condition
            │       │                   │   ╰── <358>  [&&]
            │       │                   │       ├── <343>  [||]
            │       │                   │       │   ├── <335>  [!=]
            │       │                   │       │   │   ├── <332> Var [i]
            │       │                   │       │   │   ╰── <334> Constant Int [1]
            │       │                   │       │   ╰── <341>  [!=]
            │       │                   │       │       ├── <338> Var [j]
            │       │                   │       │       ╰── <340> Constant Int [1]
            │       │                   │       ╰── <357>  [!=]
            │       │                   │           ├── <352> Subscript
            │       │                   │           │   ├── <349> Subscript
            │       │                   │           │   │   ├── <346> Var [nested_arr]
            │       │                   │           │   │   ╰── <348> Var [i]
            │       │                   │           │   ╰── <351> Var [j]
            │       │                   │           ╰── <355> Var [expected]
            │       │                   ╰── Then
            │       │                       ╰── Block
            │       │                           ╰── Return
            │       │                               ╰── <359> Constant Int [4]
            │       ╰── Return
            │           ╰── <370> Constant Int [0]
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
                        ╰── <17> Constant Int [0]
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
            │           ├── <8> Constant Int [1]
            │           ├── <10> Constant Int [2]
            │           ├── <12> Constant Int [3]
            │           ╰── <14> Constant Int [4]
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
            │       ╰── <52> Constant Int [0]
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
            │           ├── <63> Constant Int [0]
            │           ├── <65> Constant Int [0]
            │           ╰── <67> Constant Int [0]
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
            │       │   │           ╰── <79> Constant Int [0]
            │       │   ├── Condition
            │       │   │   ╰── <87>  [<]
            │       │   │       ├── <84> Var [i]
            │       │   │       ╰── <86> Constant Int [4]
            │       │   ├── Condition
            │       │   │   ╰── <96> Assign [=]
            │       │   │       ├── <89> Var [i]
            │       │   │       ╰── <95>  [+]
            │       │   │           ├── <92> Var [i]
            │       │   │           ╰── <94> Constant Int [1]
            │       │   ╰── Block
            │       │       ╰── If
            │       │           ├── Condition
            │       │           │   ╰── <108>  [!=]
            │       │           │       ├── <101> Subscript
            │       │           │       │   ├── <98> Var [arr]
            │       │           │       │   ╰── <100> Var [i]
            │       │           │       ╰── <107>  [+]
            │       │           │           ├── <104> Var [i]
            │       │           │           ╰── <106> Constant Int [1]
            │       │           ╰── Then
            │       │               ╰── Block
            │       │                   ╰── Return
            │       │                       ╰── <109> Constant Int [1]
            │       ╰── Return
            │           ╰── <117> Constant Int [0]
            ├── Function [test_ptr_to_arr]
            │   ╰── Body
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <127> Var [ptr_to_arr]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <128> Constant Int [2]
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
            │       ├── <150> Assign [=]
            │       │   ├── <145> Var [ptr_to_arr]
            │       │   ╰── <149> AddressOf
            │       │       ╰── <148> Var [nested_arr]
            │       ├── <162> Assign [=]
            │       │   ├── <159> Subscript
            │       │   │   ├── <157> Subscript
            │       │   │   │   ├── <155> Subscript
            │       │   │   │   │   ├── <153> Var [ptr_to_arr]
            │       │   │   │   │   ╰── <154> Constant Int [0]
            │       │   │   │   ╰── <156> Constant Int [2]
            │       │   │   ╰── <158> Constant Int [4]
            │       │   ╰── <161> Constant Int [100]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <172>  [!=]
            │       │   │       ├── <169> Subscript
            │       │   │       │   ├── <167> Subscript
            │       │   │       │   │   ├── <165> Var [nested_arr]
            │       │   │       │   │   ╰── <166> Constant Int [2]
            │       │   │       │   ╰── <168> Constant Int [4]
            │       │   │       ╰── <171> Constant Int [100]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <173> Constant Int [3]
            │       ╰── Return
            │           ╰── <178> Constant Int [0]
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
            │       │   │           ╰── <207> Constant Int [0]
            │       │   ├── Condition
            │       │   │   ╰── <215>  [<]
            │       │   │       ├── <212> Var [i]
            │       │   │       ╰── <214> Constant Int [3]
            │       │   ├── Condition
            │       │   │   ╰── <224> Assign [=]
            │       │   │       ├── <217> Var [i]
            │       │   │       ╰── <223>  [+]
            │       │   │           ├── <220> Var [i]
            │       │   │           ╰── <222> Constant Int [1]
            │       │   ╰── Block
            │       │       ├── If
            │       │       │   ├── Condition
            │       │       │   │   ╰── <229> Subscript
            │       │       │   │       ├── <226> Var [array_of_pointers]
            │       │       │   │       ╰── <228> Var [i]
            │       │       │   ╰── Then
            │       │       │       ╰── Return
            │       │       │           ╰── <230> Constant Int [4]
            │       │       ╰── <241> Assign [=]
            │       │           ├── <237> Subscript
            │       │           │   ├── <234> Var [array_of_pointers]
            │       │           │   ╰── <236> Var [i]
            │       │           ╰── <240> Var [ptr]
            │       ├── <254> Assign [=]
            │       │   ├── <251> Subscript
            │       │   │   ├── <249> Subscript
            │       │   │   │   ├── <247> Var [array_of_pointers]
            │       │   │   │   ╰── <248> Constant Int [2]
            │       │   │   ╰── <250> Constant Int [0]
            │       │   ╰── <253> Constant Int [11]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <261>  [!=]
            │       │   │       ├── <258> Dereference
            │       │   │       │   ╰── <257> Var [ptr]
            │       │   │       ╰── <260> Constant Int [11]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <262> Constant Int [5]
            │       ├── For
            │       │   ├── Init
            │       │   │   ╰── VarDeclaration
            │       │   │       ├── Name
            │       │   │       │   ╰── i
            │       │   │       ├── Type
            │       │   │       │   ╰── Int
            │       │   │       ╰── Initializer
            │       │   │           ╰── <270> Constant Int [0]
            │       │   ├── Condition
            │       │   │   ╰── <278>  [<]
            │       │   │       ├── <275> Var [i]
            │       │   │       ╰── <277> Constant Int [3]
            │       │   ├── Condition
            │       │   │   ╰── <287> Assign [=]
            │       │   │       ├── <280> Var [i]
            │       │   │       ╰── <286>  [+]
            │       │   │           ├── <283> Var [i]
            │       │   │           ╰── <285> Constant Int [1]
            │       │   ╰── Block
            │       │       ╰── If
            │       │           ├── Condition
            │       │           │   ╰── <297>  [!=]
            │       │           │       ├── <294> Subscript
            │       │           │       │   ├── <292> Subscript
            │       │           │       │   │   ├── <289> Var [array_of_pointers]
            │       │           │       │   │   ╰── <291> Var [i]
            │       │           │       │   ╰── <293> Constant Int [0]
            │       │           │       ╰── <296> Constant Int [11]
            │       │           ╰── Then
            │       │               ╰── Block
            │       │                   ╰── Return
            │       │                       ╰── <298> Constant Int [6]
            │       ╰── Return
            │           ╰── <306> Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── check
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <319> FunctionCall [test_arr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <323> Var [check]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <325> Var [check]
                    ├── <335> Assign [=]
                    │   ├── <331> Var [check]
                    │   ╰── <334> FunctionCall [test_ptr_to_arr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <338> Var [check]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <340> Var [check]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <348> Constant Int [0]
                    ├── <359> Assign [=]
                    │   ├── <352> Var [check]
                    │   ╰── <358> FunctionCall [test_array_of_pointers]
                    │       ╰── <357> AddressOf
                    │           ╰── <356> Var [x]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <362> Var [check]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <364> Var [check]
                    ╰── Return
                        ╰── <369> Constant Int [0]
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
                    │       ╰── <9> Constant Int [0]
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
                    │   │               ├── <18> Constant Int [1]
                    │   │               ├── <20> Constant Int [2]
                    │   │               ╰── <22> Constant Int [3]
                    │   ├── Condition
                    │   │   ╰── <31>  [<]
                    │   │       ├── <28> Var [counter]
                    │   │       ╰── <30> Constant Int [3]
                    │   ├── Condition
                    │   │   ╰── <40> Assign [=]
                    │   │       ├── <33> Var [counter]
                    │   │       ╰── <39>  [+]
                    │   │           ├── <36> Var [counter]
                    │   │           ╰── <38> Constant Int [1]
                    │   ╰── Block
                    │       ╰── If
                    │           ├── Condition
                    │           │   ╰── <52>  [!=]
                    │           │       ├── <45> Subscript
                    │           │       │   ├── <42> Var [i]
                    │           │       │   ╰── <44> Var [counter]
                    │           │       ╰── <51>  [+]
                    │           │           ├── <48> Var [counter]
                    │           │           ╰── <50> Constant Int [1]
                    │           ╰── Then
                    │               ╰── Block
                    │                   ╰── Return
                    │                       ╰── <53> Constant Int [1]
                    ╰── Return
                        ╰── <61> Constant Int [0]
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
            │           ├── <7> Constant Int [1]
            │           ├── <9> Constant Int [1]
            │           ╰── <11> Constant Int [1]
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
            │       ├── <39> Assign [=]
            │       │   ├── <35> Subscript
            │       │   │   ├── <33> Var [arr]
            │       │   │   ╰── <34> Constant Int [1]
            │       │   ╰── <38> Var [x]
            │       ├── <48> Assign [=]
            │       │   ├── <44> Subscript
            │       │   │   ├── <42> Var [arr]
            │       │   │   ╰── <43> Constant Int [2]
            │       │   ╰── <47> Var [y]
            │       ╰── Return
            │           ╰── <52> AddressOf
            │               ╰── <51> Var [arr]
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
                    │       ╰── <73> FunctionCall [foo]
                    │           ├── <71> Constant Int [2]
                    │           ╰── <72> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <84>  [!=]
                    │   │       ├── <81> Subscript
                    │   │       │   ├── <79> Subscript
                    │   │       │   │   ├── <77> Var [arr]
                    │   │       │   │   ╰── <78> Constant Int [0]
                    │   │       │   ╰── <80> Constant Int [0]
                    │   │       ╰── <83> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <85> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <98>  [!=]
                    │   │       ├── <95> Subscript
                    │   │       │   ├── <93> Subscript
                    │   │       │   │   ├── <91> Var [arr]
                    │   │       │   │   ╰── <92> Constant Int [0]
                    │   │       │   ╰── <94> Constant Int [1]
                    │   │       ╰── <97> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <99> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <112>  [!=]
                    │   │       ├── <109> Subscript
                    │   │       │   ├── <107> Subscript
                    │   │       │   │   ├── <105> Var [arr]
                    │   │       │   │   ╰── <106> Constant Int [0]
                    │   │       │   ╰── <108> Constant Int [2]
                    │   │       ╰── <111> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <113> Constant Int [3]
                    ╰── Return
                        ╰── <118> Constant Int [0]
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
                    │           ├── <14> Unary [-]
                    │           │   ╰── <13> Constant Int [10]
                    │           ├── <16> Constant Int [10]
                    │           ├── <20> Unary [-]
                    │           │   ╰── <19> Constant Int [11]
                    │           ├── <22> Constant Int [11]
                    │           ├── <26> Unary [-]
                    │           │   ╰── <25> Constant Int [12]
                    │           ╰── <28> Constant Int [12]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <45>  [!=]
                    │   │       ├── <42>  [&]
                    │   │       │   ├── <35> Subscript
                    │   │       │   │   ├── <33> Var [arr]
                    │   │       │   │   ╰── <34> Constant Int [0]
                    │   │       │   ╰── <40> Subscript
                    │   │       │       ├── <38> Var [arr]
                    │   │       │       ╰── <39> Constant Int [5]
                    │   │       ╰── <44> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <46> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <66>  [!=]
                    │   │       ├── <61>  [|]
                    │   │       │   ├── <54> Subscript
                    │   │       │   │   ├── <52> Var [arr]
                    │   │       │   │   ╰── <53> Constant Int [1]
                    │   │       │   ╰── <59> Subscript
                    │   │       │       ├── <57> Var [arr]
                    │   │       │       ╰── <58> Constant Int [4]
                    │   │       ╰── <65> Unary [-]
                    │   │           ╰── <64> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <67> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <87>  [!=]
                    │   │       ├── <82>  [^]
                    │   │       │   ├── <75> Subscript
                    │   │       │   │   ├── <73> Var [arr]
                    │   │       │   │   ╰── <74> Constant Int [2]
                    │   │       │   ╰── <80> Subscript
                    │   │       │       ├── <78> Var [arr]
                    │   │       │       ╰── <79> Constant Int [3]
                    │   │       ╰── <86> Unary [-]
                    │   │           ╰── <85> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <88> Constant Int [3]
                    ├── <99> Assign [=]
                    │   ├── <96> Subscript
                    │   │   ├── <94> Var [arr]
                    │   │   ╰── <95> Constant Int [0]
                    │   ╰── <98> Constant Int [2041302511]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <114>  [!=]
                    │   │       ├── <111>  [>>]
                    │   │       │   ├── <104> Subscript
                    │   │       │   │   ├── <102> Var [arr]
                    │   │       │   │   ╰── <103> Constant Int [0]
                    │   │       │   ╰── <109> Subscript
                    │   │       │       ├── <107> Var [arr]
                    │   │       │       ╰── <108> Constant Int [1]
                    │   │       ╰── <113> Constant Int [1993459]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <115> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <130>  [!=]
                    │   │       ├── <127>  [<<]
                    │   │       │   ├── <123> Subscript
                    │   │       │   │   ├── <121> Var [arr]
                    │   │       │   │   ╰── <122> Constant Int [5]
                    │   │       │   ╰── <125> Constant Int [3]
                    │   │       ╰── <129> Constant Int [96]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <131> Constant Int [5]
                    ╰── Return
                        ╰── <136> Constant Int [0]
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
                    │           ├── <14> Unary [-]
                    │           │   ╰── <13> Constant Int [1]
                    │           ├── <18> Unary [-]
                    │           │   ╰── <17> Constant Int [2]
                    │           ├── <22> Unary [-]
                    │           │   ╰── <21> Constant Int [3]
                    │           ╰── <26> Unary [-]
                    │               ╰── <25> Constant Int [4]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <36> Var [arr]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── idx
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <42> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <62>  [!=]
                    │   │       ├── <57> Assign [*=]
                    │   │       │   ├── <53> Subscript
                    │   │       │   │   ├── <48> Postfix [++]
                    │   │       │   │   │   ╰── <46> Var [ptr]
                    │   │       │   │   ╰── <52> Postfix [++]
                    │   │       │   │       ╰── <50> Var [idx]
                    │   │       │   ╰── <55> Constant Int [3]
                    │   │       ╰── <61> Unary [-]
                    │   │           ╰── <60> Constant Int [9]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <63> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <75>  [!=]
                    │   │       ├── <70> Dereference
                    │   │       │   ╰── <69> Var [ptr]
                    │   │       ╰── <74> Unary [-]
                    │   │           ╰── <73> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <76> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <85>  [!=]
                    │   │       ├── <82> Var [idx]
                    │   │       ╰── <84> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <86> Constant Int [3]
                    ├── <94> Postfix [--]
                    │   ╰── <92> Var [idx]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <105> Assign [+=]
                    │   │       ├── <102> Subscript
                    │   │       │   ├── <100> Unary [--]
                    │   │       │   │   ╰── <98> Var [ptr]
                    │   │       │   ╰── <101> Constant Int [3]
                    │   │       ╰── <104> Constant Int [4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <106> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <150>  [||]
                    │   │       ├── <141>  [||]
                    │   │       │   ├── <130>  [||]
                    │   │       │   │   ├── <119>  [!=]
                    │   │       │   │   │   ├── <114> Subscript
                    │   │       │   │   │   │   ├── <112> Var [arr]
                    │   │       │   │   │   │   ╰── <113> Constant Int [0]
                    │   │       │   │   │   ╰── <118> Unary [-]
                    │   │       │   │   │       ╰── <117> Constant Int [1]
                    │   │       │   │   ╰── <129>  [!=]
                    │   │       │   │       ├── <124> Subscript
                    │   │       │   │       │   ├── <122> Var [arr]
                    │   │       │   │       │   ╰── <123> Constant Int [1]
                    │   │       │   │       ╰── <128> Unary [-]
                    │   │       │   │           ╰── <127> Constant Int [2]
                    │   │       │   ╰── <140>  [!=]
                    │   │       │       ├── <135> Subscript
                    │   │       │       │   ├── <133> Var [arr]
                    │   │       │       │   ╰── <134> Constant Int [2]
                    │   │       │       ╰── <139> Unary [-]
                    │   │       │           ╰── <138> Constant Int [9]
                    │   │       ╰── <149>  [!=]
                    │   │           ├── <146> Subscript
                    │   │           │   ├── <144> Var [arr]
                    │   │           │   ╰── <145> Constant Int [3]
                    │   │           ╰── <148> Constant Int [0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <151> Constant Int [5]
                    ╰── Return
                        ╰── <156> Constant Int [0]
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
                    │   │       ├── <19> Constant Int [0]
                    │   │       ├── <21> Constant Int [0]
                    │   │       ╰── <23> Constant Int [0]
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
                    │           ├── <33> Constant Int [100]
                    │           ├── <35> Constant Int [101]
                    │           ├── <37> Constant Int [102]
                    │           ╰── <39> Constant Int [103]
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
                    │           │   ├── <52> Constant Int [200]
                    │           │   ├── <54> Constant Int [201]
                    │           │   ├── <56> Constant Int [202]
                    │           │   ╰── <58> Constant Int [203]
                    │           ╰── Compound
                    │               ├── <61> Constant Int [300]
                    │               ├── <63> Constant Int [301]
                    │               ├── <65> Constant Int [302]
                    │               ╰── <67> Constant Int [303]
                    ├── <80> Assign [=]
                    │   ├── <75> Subscript
                    │   │   ├── <73> Var [array_of_pointers]
                    │   │   ╰── <74> Constant Int [0]
                    │   ╰── <79> AddressOf
                    │       ╰── <78> Var [array1]
                    ├── <92> Assign [=]
                    │   ├── <85> Subscript
                    │   │   ├── <83> Var [array_of_pointers]
                    │   │   ╰── <84> Constant Int [1]
                    │   ╰── <91> AddressOf
                    │       ╰── <90> Subscript
                    │           ├── <88> Var [nested_array]
                    │           ╰── <89> Constant Int [0]
                    ├── <104> Assign [=]
                    │   ├── <97> Subscript
                    │   │   ├── <95> Var [array_of_pointers]
                    │   │   ╰── <96> Constant Int [2]
                    │   ╰── <103> AddressOf
                    │       ╰── <102> Subscript
                    │           ├── <100> Var [nested_array]
                    │           ╰── <101> Constant Int [1]
                    ├── <112> Assign [+=]
                    │   ├── <109> Subscript
                    │   │   ├── <107> Var [array_of_pointers]
                    │   │   ╰── <108> Constant Int [0]
                    │   ╰── <111> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <126>  [!=]
                    │   │       ├── <123> Subscript
                    │   │       │   ├── <121> Subscript
                    │   │       │   │   ├── <117> Subscript
                    │   │       │   │   │   ├── <115> Var [array_of_pointers]
                    │   │       │   │   │   ╰── <116> Constant Int [0]
                    │   │       │   │   ╰── <120> Unary [-]
                    │   │       │   │       ╰── <119> Constant Int [1]
                    │   │       │   ╰── <122> Constant Int [3]
                    │   │       ╰── <125> Constant Int [103]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <127> Constant Int [1]
                    ├── <138> Assign [+=]
                    │   ├── <135> Subscript
                    │   │   ├── <133> Var [array_of_pointers]
                    │   │   ╰── <134> Constant Int [1]
                    │   ╰── <137> Constant Int [1]
                    ├── <146> Assign [-=]
                    │   ├── <143> Subscript
                    │   │   ├── <141> Var [array_of_pointers]
                    │   │   ╰── <142> Constant Int [2]
                    │   ╰── <145> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <158>  [!=]
                    │   │       ├── <155> Subscript
                    │   │       │   ├── <153> Subscript
                    │   │       │   │   ├── <151> Subscript
                    │   │       │   │   │   ├── <149> Var [array_of_pointers]
                    │   │       │   │   │   ╰── <150> Constant Int [1]
                    │   │       │   │   ╰── <152> Constant Int [0]
                    │   │       │   ╰── <154> Constant Int [3]
                    │   │       ╰── <157> Constant Int [303]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <159> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <174>  [!=]
                    │   │       ├── <171> Subscript
                    │   │       │   ├── <169> Subscript
                    │   │       │   │   ├── <167> Subscript
                    │   │       │   │   │   ├── <165> Var [array_of_pointers]
                    │   │       │   │   │   ╰── <166> Constant Int [2]
                    │   │       │   │   ╰── <168> Constant Int [0]
                    │   │       │   ╰── <170> Constant Int [3]
                    │   │       ╰── <173> Constant Int [203]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <175> Constant Int [3]
                    ╰── Return
                        ╰── <180> Constant Int [0]
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
            │           │   ├── <10> Constant Int [1]
            │           │   ├── <12> Constant Int [2]
            │           │   ╰── <14> Constant Int [3]
            │           ╰── Compound
            │               ├── <17> Constant Int [4]
            │               ├── <19> Constant Int [5]
            │               ╰── <21> Constant Int [6]
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
            │           │   ├── <35> Constant Double [+1e2]
            │           │   ╰── <37> Constant Double [+1.01e2]
            │           ├── Compound
            │           │   ├── <40> Constant Double [+1.02e2]
            │           │   ╰── <42> Constant Double [+1.03e2]
            │           ╰── Compound
            │               ├── <45> Constant Double [+1.04e2]
            │               ╰── <47> Constant Double [+1.05e2]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── unsigned_index
            │   ├── Type
            │   │   ╰── Unsigned Int
            │   ╰── Initializer
            │       ╰── <55> Constant Int [10]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <83>  [!=]
                    │   │       ├── <78> Assign [*=]
                    │   │       │   ├── <72> Subscript
                    │   │       │   │   ├── <66> Subscript
                    │   │       │   │   │   ├── <64> Var [long_nested_arr]
                    │   │       │   │   │   ╰── <65> Constant Int [1]
                    │   │       │   │   ╰── <71>  [-]
                    │   │       │   │       ├── <68> Var [unsigned_index]
                    │   │       │   │       ╰── <70> Constant Int [8]
                    │   │       │   ╰── <76> Unary [-]
                    │   │       │       ╰── <75> Constant Int [1]
                    │   │       ╰── <82> Unary [-]
                    │   │           ╰── <81> Constant Int [6]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <84> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <99>  [!=]
                    │   │       ├── <94> Subscript
                    │   │       │   ├── <92> Subscript
                    │   │       │   │   ├── <90> Var [long_nested_arr]
                    │   │       │   │   ╰── <91> Constant Int [1]
                    │   │       │   ╰── <93> Constant Int [2]
                    │   │       ╰── <98> Unary [-]
                    │   │           ╰── <97> Constant Int [6]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <100> Constant Int [2]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <108> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <116>  [<]
                    │   │       ├── <113> Var [i]
                    │   │       ╰── <115> Constant Int [2]
                    │   ├── Condition
                    │   │   ╰── <121> Assign [+=]
                    │   │       ├── <118> Var [i]
                    │   │       ╰── <120> Constant Int [1]
                    │   ╰── Block
                    │       ╰── For
                    │           ├── Init
                    │           │   ╰── VarDeclaration
                    │           │       ├── Name
                    │           │       │   ╰── j
                    │           │       ├── Type
                    │           │       │   ╰── Int
                    │           │       ╰── Initializer
                    │           │           ╰── <125> Constant Int [0]
                    │           ├── Condition
                    │           │   ╰── <133>  [<]
                    │           │       ├── <130> Var [j]
                    │           │       ╰── <132> Constant Int [3]
                    │           ├── Condition
                    │           │   ╰── <138> Assign [+=]
                    │           │       ├── <135> Var [j]
                    │           │       ╰── <137> Constant Int [1]
                    │           ╰── Block
                    │               ├── If
                    │               │   ├── Condition
                    │               │   │   ╰── <150>  [&&]
                    │               │   │       ├── <143>  [==]
                    │               │   │       │   ├── <140> Var [i]
                    │               │   │       │   ╰── <142> Constant Int [1]
                    │               │   │       ╰── <149>  [==]
                    │               │   │           ├── <146> Var [j]
                    │               │   │           ╰── <148> Constant Int [2]
                    │               │   ╰── Then
                    │               │       ╰── Block
                    │               │           ╰── Break
                    │               ├── VarDeclaration
                    │               │   ├── Name
                    │               │   │   ╰── expected
                    │               │   ├── Type
                    │               │   │   ╰── Long
                    │               │   ╰── Initializer
                    │               │       ╰── <169>  [+]
                    │               │           ├── <166>  [+]
                    │               │           │   ├── <162>  [*]
                    │               │           │   │   ├── <159> Var [i]
                    │               │           │   │   ╰── <161> Constant Int [3]
                    │               │           │   ╰── <165> Var [j]
                    │               │           ╰── <168> Constant Int [1]
                    │               ╰── If
                    │                   ├── Condition
                    │                   │   ╰── <183>  [!=]
                    │                   │       ├── <179> Subscript
                    │                   │       │   ├── <176> Subscript
                    │                   │       │   │   ├── <173> Var [long_nested_arr]
                    │                   │       │   │   ╰── <175> Var [i]
                    │                   │       │   ╰── <178> Var [j]
                    │                   │       ╰── <182> Var [expected]
                    │                   ╰── Then
                    │                       ╰── Block
                    │                           ╰── Return
                    │                               ╰── <184> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <207>  [!=]
                    │   │       ├── <204> Assign [+=]
                    │   │       │   ├── <200> Subscript
                    │   │       │   │   ├── <198> Subscript
                    │   │       │   │   │   ├── <196> Var [dbl_nested_arr]
                    │   │       │   │   │   ╰── <197> Constant Int [1]
                    │   │       │   │   ╰── <199> Constant Int [1]
                    │   │       │   ╰── <202> Constant Double [+1e2]
                    │   │       ╰── <206> Constant Double [+2.03e2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <208> Constant Int [4]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <216> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <224>  [<]
                    │   │       ├── <221> Var [i]
                    │   │       ╰── <223> Constant Int [3]
                    │   ├── Condition
                    │   │   ╰── <229> Assign [+=]
                    │   │       ├── <226> Var [i]
                    │   │       ╰── <228> Constant Int [1]
                    │   ╰── Block
                    │       ╰── For
                    │           ├── Init
                    │           │   ╰── VarDeclaration
                    │           │       ├── Name
                    │           │       │   ╰── j
                    │           │       ├── Type
                    │           │       │   ╰── Int
                    │           │       ╰── Initializer
                    │           │           ╰── <233> Constant Int [0]
                    │           ├── Condition
                    │           │   ╰── <241>  [<]
                    │           │       ├── <238> Var [j]
                    │           │       ╰── <240> Constant Int [2]
                    │           ├── Condition
                    │           │   ╰── <246> Assign [+=]
                    │           │       ├── <243> Var [j]
                    │           │       ╰── <245> Constant Int [1]
                    │           ╰── Block
                    │               ├── If
                    │               │   ├── Condition
                    │               │   │   ╰── <258>  [&&]
                    │               │   │       ├── <251>  [==]
                    │               │   │       │   ├── <248> Var [i]
                    │               │   │       │   ╰── <250> Constant Int [1]
                    │               │   │       ╰── <257>  [==]
                    │               │   │           ├── <254> Var [j]
                    │               │   │           ╰── <256> Constant Int [1]
                    │               │   ╰── Then
                    │               │       ╰── Block
                    │               │           ╰── Continue
                    │               ├── VarDeclaration
                    │               │   ├── Name
                    │               │   │   ╰── expected
                    │               │   ├── Type
                    │               │   │   ╰── Int
                    │               │   ╰── Initializer
                    │               │       ╰── <277>  [+]
                    │               │           ├── <273>  [+]
                    │               │           │   ├── <266> Constant Int [100]
                    │               │           │   ╰── <272>  [*]
                    │               │           │       ├── <269> Var [i]
                    │               │           │       ╰── <271> Constant Int [2]
                    │               │           ╰── <276> Var [j]
                    │               ╰── If
                    │                   ├── Condition
                    │                   │   ╰── <291>  [!=]
                    │                   │       ├── <287> Subscript
                    │                   │       │   ├── <284> Subscript
                    │                   │       │   │   ├── <281> Var [dbl_nested_arr]
                    │                   │       │   │   ╰── <283> Var [i]
                    │                   │       │   ╰── <286> Var [j]
                    │                   │       ╰── <290> Var [expected]
                    │                   ╰── Then
                    │                       ╰── Block
                    │                           ╰── Return
                    │                               ╰── <292> Constant Int [5]
                    ╰── Return
                        ╰── <303> Constant Int [0]
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
            │           ├── <7> Constant UInt [4294967295]
            │           ├── <9> Constant UInt [4294967294]
            │           ├── <11> Constant UInt [4294967293]
            │           ╰── <13> Constant UInt [4294967292]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── idx
            │   ├── Type
            │   │   ╰── Int
            │   ╰── Initializer
            │       ╰── <20> Constant Int [2]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── long_idx
            │   ├── Type
            │   │   ╰── Long
            │   ╰── Initializer
            │       ╰── <26> Constant Int [1]
            ╰── Function [main]
                ╰── Body
                    ├── <41> Assign [=]
                    │   ├── <35> Var [long_idx]
                    │   ╰── <40> Unary [-]
                    │       ╰── <39> Var [long_idx]
                    ├── <49> Assign [+=]
                    │   ├── <46> Subscript
                    │   │   ├── <44> Var [unsigned_arr]
                    │   │   ╰── <45> Constant Int [1]
                    │   ╰── <48> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <54> Subscript
                    │   │       ├── <52> Var [unsigned_arr]
                    │   │       ╰── <53> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <55> Constant Int [1]
                    ├── <67> Assign [-=]
                    │   ├── <64> Subscript
                    │   │   ├── <61> Var [unsigned_arr]
                    │   │   ╰── <63> Var [idx]
                    │   ╰── <66> Constant Double [+1e1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <76>  [!=]
                    │   │       ├── <73> Subscript
                    │   │       │   ├── <70> Var [unsigned_arr]
                    │   │       │   ╰── <72> Var [idx]
                    │   │       ╰── <75> Constant UInt [4294967283]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <77> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── unsigned_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Unsigned Int
                    │   ╰── Initializer
                    │       ╰── <91>  [+]
                    │           ├── <88> Var [unsigned_arr]
                    │           ╰── <90> Constant Int [4]
                    ├── <101> Assign [/=]
                    │   ├── <98> Subscript
                    │   │   ├── <95> Var [unsigned_ptr]
                    │   │   ╰── <97> Var [long_idx]
                    │   ╰── <100> Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <109>  [!=]
                    │   │       ├── <106> Subscript
                    │   │       │   ├── <104> Var [unsigned_arr]
                    │   │       │   ╰── <105> Constant Int [3]
                    │   │       ╰── <108> Constant UInt [429496729]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <110> Constant Int [3]
                    ├── <128> Assign [*=]
                    │   ├── <122> Subscript
                    │   │   ├── <116> Var [unsigned_ptr]
                    │   │   ╰── <121> Assign [*=]
                    │   │       ├── <118> Var [long_idx]
                    │   │       ╰── <120> Constant Int [2]
                    │   ╰── <127> Subscript
                    │       ├── <125> Var [unsigned_arr]
                    │       ╰── <126> Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <136>  [!=]
                    │   │       ├── <133> Subscript
                    │   │       │   ├── <131> Var [unsigned_arr]
                    │   │       │   ╰── <132> Constant Int [2]
                    │   │       ╰── <135> Constant Int [13]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <137> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <157>  [!=]
                    │   │       ├── <154> Assign [%=]
                    │   │       │   ├── <150> Subscript
                    │   │       │   │   ├── <143> Var [unsigned_arr]
                    │   │       │   │   ╰── <149>  [+]
                    │   │       │   │       ├── <145> Var [idx]
                    │   │       │   │       ╰── <148> Var [long_idx]
                    │   │       │   ╰── <152> Constant Int [10]
                    │   │       ╰── <156> Constant Int [5]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <158> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <169>  [!=]
                    │   │       ├── <166> Subscript
                    │   │       │   ├── <164> Var [unsigned_arr]
                    │   │       │   ╰── <165> Constant Int [0]
                    │   │       ╰── <168> Constant UInt [5]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <170> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <178> Subscript
                    │   │       ├── <176> Var [unsigned_arr]
                    │   │       ╰── <177> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <179> Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <190>  [!=]
                    │   │       ├── <187> Subscript
                    │   │       │   ├── <185> Var [unsigned_arr]
                    │   │       │   ╰── <186> Constant Int [2]
                    │   │       ╰── <189> Constant Int [13]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <191> Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <202>  [!=]
                    │   │       ├── <199> Subscript
                    │   │       │   ├── <197> Var [unsigned_arr]
                    │   │       │   ╰── <198> Constant Int [3]
                    │   │       ╰── <201> Constant UInt [429496729]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <203> Constant Int [9]
                    ╰── Return
                        ╰── <208> Constant Int [0]
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
                    │           ├── <12> Constant Long [2147483648]
                    │           ├── <14> Constant ULong [18446744069414584320]
                    │           ├── <16> Constant ULong [9223372036854775808]
                    │           ╰── <18> Constant Long [1085102592571150095]
                    ├── <31> Assign [&=]
                    │   ├── <25> Subscript
                    │   │   ├── <23> Var [arr]
                    │   │   ╰── <24> Constant Int [1]
                    │   ╰── <30> Subscript
                    │       ├── <28> Var [arr]
                    │       ╰── <29> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <39>  [!=]
                    │   │       ├── <36> Subscript
                    │   │       │   ├── <34> Var [arr]
                    │   │       │   ╰── <35> Constant Int [1]
                    │   │       ╰── <38> Constant Long [1085102592318504960]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <40> Constant Int [1]
                    ├── <54> Assign [|=]
                    │   ├── <48> Subscript
                    │   │   ├── <46> Var [arr]
                    │   │   ╰── <47> Constant Int [0]
                    │   ╰── <53> Subscript
                    │       ├── <51> Var [arr]
                    │       ╰── <52> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <62>  [!=]
                    │   │       ├── <59> Subscript
                    │   │       │   ├── <57> Var [arr]
                    │   │       │   ╰── <58> Constant Int [0]
                    │   │       ╰── <61> Constant ULong [1085102594465988608]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <63> Constant Int [2]
                    ├── <77> Assign [^=]
                    │   ├── <71> Subscript
                    │   │   ├── <69> Var [arr]
                    │   │   ╰── <70> Constant Int [2]
                    │   ╰── <76> Subscript
                    │       ├── <74> Var [arr]
                    │       ╰── <75> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <85>  [!=]
                    │   │       ├── <82> Subscript
                    │   │       │   ├── <80> Var [arr]
                    │   │       │   ╰── <81> Constant Int [2]
                    │   │       ╰── <84> Constant ULong [10308474629425925903]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <86> Constant Int [3]
                    ├── <97> Assign [>>=]
                    │   ├── <94> Subscript
                    │   │   ├── <92> Var [arr]
                    │   │   ╰── <93> Constant Int [3]
                    │   ╰── <96> Constant Int [25]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <105>  [!=]
                    │   │       ├── <102> Subscript
                    │   │       │   ├── <100> Var [arr]
                    │   │       │   ╰── <101> Constant Int [3]
                    │   │       ╰── <104> Constant Long [32338577287]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <106> Constant Int [4]
                    ├── <117> Assign [<<=]
                    │   ├── <114> Subscript
                    │   │   ├── <112> Var [arr]
                    │   │   ╰── <113> Constant Int [1]
                    │   ╰── <116> Constant Int [12]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <125>  [!=]
                    │   │       ├── <122> Subscript
                    │   │       │   ├── <120> Var [arr]
                    │   │       │   ╰── <121> Constant Int [1]
                    │   │       ╰── <124> Constant ULong [17361640446303928320]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <126> Constant Int [5]
                    ╰── Return
                        ╰── <131> Constant Int [0]
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
            │       │   │   ╰── <10> Constant Int [0]
            │       │   ╰── Static
            │       ├── <17> Assign [+=]
            │       │   ├── <14> Var [count]
            │       │   ╰── <16> Constant Int [1]
            │       ╰── Return
            │           ╰── <20> Var [count]
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
                    │           ├── <35> Constant Int [10]
                    │           ├── <37> Constant Int [11]
                    │           ├── <39> Constant Int [12]
                    │           ╰── <41> Constant Int [13]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <52>  [!=]
                    │   │       ├── <49> Subscript
                    │   │       │   ├── <46> Var [arr]
                    │   │       │   ╰── <48> FunctionCall [get_call_count]
                    │   │       ╰── <51> Constant Int [11]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <53> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── end_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <67>  [+]
                    │           ├── <64> Var [arr]
                    │           ╰── <66> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <83>  [!=]
                    │   │       ├── <80> Subscript
                    │   │       │   ├── <75>  [-]
                    │   │       │   │   ├── <71> Var [end_ptr]
                    │   │       │   │   ╰── <73> Constant Int [1]
                    │   │       │   ╰── <79> Unary [-]
                    │   │       │       ╰── <78> FunctionCall [get_call_count]
                    │   │       ╰── <82> Constant Int [11]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <84> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <93>  [!=]
                    │   │       ├── <90> FunctionCall [get_call_count]
                    │   │       ╰── <92> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <94> Constant Int [3]
                    ╰── Return
                        ╰── <99> Constant Int [0]
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
            │   │       │   │   ├── <14> Constant Int [10]
            │   │       │   │   ├── <16> Constant Int [9]
            │   │       │   │   ╰── <18> Constant Int [8]
            │   │       │   ╰── Compound
            │   │       │       ├── <21> Constant Int [1]
            │   │       │       ╰── <23> Constant Int [2]
            │   │       ╰── Compound
            │   │           ╰── Compound
            │   │               ├── <27> Constant Int [100]
            │   │               ├── <29> Constant Int [99]
            │   │               ╰── <31> Constant Int [98]
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
                    │       ╰── <55> Var [nested_arr]
                    ├── <62> Assign [+=]
                    │   ├── <59> Var [outer_ptr]
                    │   ╰── <61> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <72>  [!=]
                    │   │       ├── <65> Var [outer_ptr]
                    │   │       ╰── <71>  [+]
                    │   │           ├── <68> Var [nested_arr]
                    │   │           ╰── <70> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <73> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <88>  [!=]
                    │   │       ├── <85> Subscript
                    │   │       │   ├── <83> Subscript
                    │   │       │   │   ├── <81> Subscript
                    │   │       │   │   │   ├── <79> Var [outer_ptr]
                    │   │       │   │   │   ╰── <80> Constant Int [0]
                    │   │       │   │   ╰── <82> Constant Int [0]
                    │   │       │   ╰── <84> Constant Int [0]
                    │   │       ╰── <87> Constant Int [100]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <89> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── inner_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Array
                    │   │           ├── 5
                    │   │           ╰── Long
                    │   ╰── Initializer
                    │       ╰── <109>  [+]
                    │           ├── <106> Subscript
                    │           │   ├── <104> Var [nested_arr]
                    │           │   ╰── <105> Constant Int [0]
                    │           ╰── <108> Constant Int [4]
                    ├── <116> Assign [-=]
                    │   ├── <113> Var [inner_ptr]
                    │   ╰── <115> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <126>  [!=]
                    │   │       ├── <123> Subscript
                    │   │       │   ├── <121> Subscript
                    │   │       │   │   ├── <119> Var [inner_ptr]
                    │   │       │   │   ╰── <120> Constant Int [0]
                    │   │       │   ╰── <122> Constant Int [1]
                    │   │       ╰── <125> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <127> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── idx
                    │   ├── Type
                    │   │   ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <145>  [-]
                    │           ├── <142> Subscript
                    │           │   ├── <140> Subscript
                    │           │   │   ├── <138> Subscript
                    │           │   │   │   ├── <136> Var [nested_arr]
                    │           │   │   │   ╰── <137> Constant Int [0]
                    │           │   │   ╰── <139> Constant Int [0]
                    │           │   ╰── <141> Constant Int [0]
                    │           ╰── <144> Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <163>  [!=]
                    │   │       ├── <154> Assign [+=]
                    │   │       │   ├── <149> Var [inner_ptr]
                    │   │       │   ╰── <152> Var [idx]
                    │   │       ╰── <162> AddressOf
                    │   │           ╰── <161> Subscript
                    │   │               ├── <159> Subscript
                    │   │               │   ├── <157> Var [nested_arr]
                    │   │               │   ╰── <158> Constant Int [0]
                    │   │               ╰── <160> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <164> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <180>  [!=]
                    │   │       ├── <176> Subscript
                    │   │       │   ├── <174> Subscript
                    │   │       │   │   ├── <170> Var [inner_ptr]
                    │   │       │   │   ╰── <173> Unary [-]
                    │   │       │   │       ╰── <172> Constant Int [2]
                    │   │       │   ╰── <175> Constant Int [1]
                    │   │       ╰── <178> Constant Int [9]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <181> Constant Int [5]
                    ╰── Return
                        ╰── <186> Constant Int [0]
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
            │       │           ├── <18> Constant Int [1]
            │       │           ├── <20> Constant Int [2]
            │       │           ├── <22> Constant Int [3]
            │       │           ├── <24> Constant Int [4]
            │       │           ├── <26> Constant Int [5]
            │       │           ╰── <28> Constant Int [6]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ptr
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <38> Var [arr]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <50>  [!=]
            │       │   │       ├── <47> Dereference
            │       │   │       │   ╰── <46> Assign [+=]
            │       │   │       │       ├── <42> Var [ptr]
            │       │   │       │       ╰── <44> Constant Int [5]
            │       │   │       ╰── <49> Constant Int [6]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <51> Constant Int [1]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <62>  [!=]
            │       │   │       ├── <59> Subscript
            │       │   │       │   ├── <57> Var [ptr]
            │       │   │       │   ╰── <58> Constant Int [0]
            │       │   │       ╰── <61> Constant Int [6]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <63> Constant Int [2]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <76>  [!=]
            │       │   │       ├── <69> Var [ptr]
            │       │   │       ╰── <75>  [+]
            │       │   │           ├── <72> Var [arr]
            │       │   │           ╰── <74> Constant Int [5]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <77> Constant Int [3]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <91>  [!=]
            │       │   │       ├── <88> Dereference
            │       │   │       │   ╰── <87> Assign [-=]
            │       │   │       │       ├── <83> Var [ptr]
            │       │   │       │       ╰── <85> Constant Int [3]
            │       │   │       ╰── <90> Constant Int [3]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <92> Constant Int [4]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <103>  [!=]
            │       │   │       ├── <100> Subscript
            │       │   │       │   ├── <98> Var [ptr]
            │       │   │       │   ╰── <99> Constant Int [0]
            │       │   │       ╰── <102> Constant Int [3]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <104> Constant Int [5]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <117>  [!=]
            │       │   │       ├── <110> Var [ptr]
            │       │   │       ╰── <116>  [+]
            │       │   │           ├── <113> Var [arr]
            │       │   │           ╰── <115> Constant Int [2]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <118> Constant Int [6]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <139>  [!=]
            │       │   │       ├── <132> Assign [+=]
            │       │   │       │   ├── <124> Var [ptr]
            │       │   │       │   ╰── <130>  [-]
            │       │   │       │       ├── <127> Var [i]
            │       │   │       │       ╰── <129> Constant Int [1]
            │       │   │       ╰── <138>  [+]
            │       │   │           ├── <135> Var [arr]
            │       │   │           ╰── <137> Constant Int [5]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <140> Constant Int [7]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <150>  [!=]
            │       │   │       ├── <147> Dereference
            │       │   │       │   ╰── <146> Var [ptr]
            │       │   │       ╰── <149> Constant Int [6]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <151> Constant Int [8]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <173>  [!=]
            │       │   │       ├── <166> Assign [-=]
            │       │   │       │   ├── <157> Var [ptr]
            │       │   │       │   ╰── <164>  [+]
            │       │   │       │       ├── <159> Constant UInt [4294967295]
            │       │   │       │       ╰── <162> Var [i]
            │       │   │       ╰── <172>  [+]
            │       │   │           ├── <169> Var [arr]
            │       │   │           ╰── <171> Constant Int [2]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <174> Constant Int [9]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <184>  [!=]
            │       │   │       ├── <181> Dereference
            │       │   │       │   ╰── <180> Var [ptr]
            │       │   │       ╰── <183> Constant Int [3]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <185> Constant Int [10]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── l
            │       │   ├── Type
            │       │   │   ╰── Long
            │       │   ╰── Initializer
            │       │       ╰── <193> Constant Long [9223372036854775807]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <212>  [!=]
            │       │   │       ├── <205> Assign [+=]
            │       │   │       │   ├── <197> Var [ptr]
            │       │   │       │   ╰── <203>  [-]
            │       │   │       │       ├── <200> Var [l]
            │       │   │       │       ╰── <202> Constant Long [9223372036854775806]
            │       │   │       ╰── <211>  [+]
            │       │   │           ├── <208> Var [arr]
            │       │   │           ╰── <210> Constant Int [3]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <213> Constant Int [11]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <223>  [!=]
            │       │   │       ├── <220> Dereference
            │       │   │       │   ╰── <219> Var [ptr]
            │       │   │       ╰── <222> Constant Int [4]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <224> Constant Int [12]
            │       ╰── Return
            │           ╰── <229> Constant Int [0]
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
            │       │   │       ├── <245> Constant Double [+1e0]
            │       │   │       ├── <247> Constant Double [+2e0]
            │       │   │       ├── <249> Constant Double [+3e0]
            │       │   │       ├── <251> Constant Double [+4e0]
            │       │   │       ├── <253> Constant Double [+5e0]
            │       │   │       ╰── <255> Constant Double [+6e0]
            │       │   ╰── Static
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ptr
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Double
            │       │   ╰── Initializer
            │       │       ╰── <265> Var [arr]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <277>  [!=]
            │       │   │       ├── <274> Dereference
            │       │   │       │   ╰── <273> Assign [+=]
            │       │   │       │       ├── <269> Var [ptr]
            │       │   │       │       ╰── <271> Constant Int [5]
            │       │   │       ╰── <276> Constant Int [6]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <278> Constant Int [1]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <289>  [!=]
            │       │   │       ├── <286> Subscript
            │       │   │       │   ├── <284> Var [ptr]
            │       │   │       │   ╰── <285> Constant Int [0]
            │       │   │       ╰── <288> Constant Int [6]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <290> Constant Int [2]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <303>  [!=]
            │       │   │       ├── <296> Var [ptr]
            │       │   │       ╰── <302>  [+]
            │       │   │           ├── <299> Var [arr]
            │       │   │           ╰── <301> Constant Int [5]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <304> Constant Int [3]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <318>  [!=]
            │       │   │       ├── <315> Dereference
            │       │   │       │   ╰── <314> Assign [-=]
            │       │   │       │       ├── <310> Var [ptr]
            │       │   │       │       ╰── <312> Constant Int [3]
            │       │   │       ╰── <317> Constant Int [3]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <319> Constant Int [4]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <330>  [!=]
            │       │   │       ├── <327> Subscript
            │       │   │       │   ├── <325> Var [ptr]
            │       │   │       │   ╰── <326> Constant Int [0]
            │       │   │       ╰── <329> Constant Int [3]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <331> Constant Int [5]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <344>  [!=]
            │       │   │       ├── <337> Var [ptr]
            │       │   │       ╰── <343>  [+]
            │       │   │           ├── <340> Var [arr]
            │       │   │           ╰── <342> Constant Int [2]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <345> Constant Int [6]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <366>  [!=]
            │       │   │       ├── <359> Assign [+=]
            │       │   │       │   ├── <351> Var [ptr]
            │       │   │       │   ╰── <357>  [-]
            │       │   │       │       ├── <354> Var [i]
            │       │   │       │       ╰── <356> Constant Int [1]
            │       │   │       ╰── <365>  [+]
            │       │   │           ├── <362> Var [arr]
            │       │   │           ╰── <364> Constant Int [5]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <367> Constant Int [7]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <377>  [!=]
            │       │   │       ├── <374> Dereference
            │       │   │       │   ╰── <373> Var [ptr]
            │       │   │       ╰── <376> Constant Int [6]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <378> Constant Int [8]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <400>  [!=]
            │       │   │       ├── <393> Assign [-=]
            │       │   │       │   ├── <384> Var [ptr]
            │       │   │       │   ╰── <391>  [+]
            │       │   │       │       ├── <386> Constant UInt [4294967295]
            │       │   │       │       ╰── <389> Var [i]
            │       │   │       ╰── <399>  [+]
            │       │   │           ├── <396> Var [arr]
            │       │   │           ╰── <398> Constant Int [2]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <401> Constant Int [9]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <411>  [!=]
            │       │   │       ├── <408> Dereference
            │       │   │       │   ╰── <407> Var [ptr]
            │       │   │       ╰── <410> Constant Int [3]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <412> Constant Int [10]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── l
            │       │   ├── Type
            │       │   │   ╰── Long
            │       │   ╰── Initializer
            │       │       ╰── <420> Constant Long [9223372036854775807]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <439>  [!=]
            │       │   │       ├── <432> Assign [+=]
            │       │   │       │   ├── <424> Var [ptr]
            │       │   │       │   ╰── <430>  [-]
            │       │   │       │       ├── <427> Var [l]
            │       │   │       │       ╰── <429> Constant Long [9223372036854775806]
            │       │   │       ╰── <438>  [+]
            │       │   │           ├── <435> Var [arr]
            │       │   │           ╰── <437> Constant Int [3]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <440> Constant Int [11]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <450>  [!=]
            │       │   │       ├── <447> Dereference
            │       │   │       │   ╰── <446> Var [ptr]
            │       │   │       ╰── <449> Constant Int [4]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <451> Constant Int [12]
            │       ╰── Return
            │           ╰── <456> Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── result
                    │   ╰── Type
                    │       ╰── Int
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <475> Assign [=]
                    │   │       ├── <470> Var [result]
                    │   │       ╰── <473> FunctionCall [int_array]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <477> Var [result]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <488> Assign [=]
                    │   │       ├── <483> Var [result]
                    │   │       ╰── <486> FunctionCall [double_array]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <493>  [+]
                    │                   ├── <490> Var [result]
                    │                   ╰── <492> Constant Int [12]
                    ╰── Return
                        ╰── <498> Constant Int [0]
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
                    │           │   │   ├── <18> Constant Int [1]
                    │           │   │   ├── <20> Constant Int [2]
                    │           │   │   ├── <22> Constant Int [3]
                    │           │   │   ╰── <24> Constant Int [4]
                    │           │   ├── Compound
                    │           │   │   ├── <27> Constant Int [5]
                    │           │   │   ├── <29> Constant Int [6]
                    │           │   │   ├── <31> Constant Int [7]
                    │           │   │   ╰── <33> Constant Int [8]
                    │           │   ╰── Compound
                    │           │       ├── <36> Constant Int [9]
                    │           │       ├── <38> Constant Int [10]
                    │           │       ├── <40> Constant Int [11]
                    │           │       ╰── <42> Constant Int [12]
                    │           ╰── Compound
                    │               ├── Compound
                    │               │   ├── <46> Constant Int [13]
                    │               │   ├── <48> Constant Int [14]
                    │               │   ├── <50> Constant Int [15]
                    │               │   ╰── <52> Constant Int [16]
                    │               ├── Compound
                    │               │   ├── <55> Constant Int [17]
                    │               │   ├── <57> Constant Int [18]
                    │               │   ├── <59> Constant Int [19]
                    │               │   ╰── <61> Constant Int [20]
                    │               ╰── Compound
                    │                   ├── <64> Constant Int [21]
                    │                   ├── <66> Constant Int [22]
                    │                   ├── <68> Constant Int [23]
                    │                   ╰── <70> Constant Int [24]
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
                    │       ╰── <92>  [+]
                    │           ├── <89> Var [arr]
                    │           ╰── <91> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <105>  [!=]
                    │   │       ├── <98> Postfix [--]
                    │   │       │   ╰── <96> Var [outer_ptr]
                    │   │       ╰── <104> AddressOf
                    │   │           ╰── <103> Subscript
                    │   │               ├── <101> Var [arr]
                    │   │               ╰── <102> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <106> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <124>  [!=]
                    │   │       ├── <116> Subscript
                    │   │       │   ├── <114> Subscript
                    │   │       │   │   ├── <112> Var [outer_ptr]
                    │   │       │   │   ╰── <113> Constant Int [0]
                    │   │       │   ╰── <115> Constant Int [1]
                    │   │       ╰── <123> Subscript
                    │   │           ├── <121> Subscript
                    │   │           │   ├── <119> Var [arr]
                    │   │           │   ╰── <120> Constant Int [0]
                    │   │           ╰── <122> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <125> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <143>  [!=]
                    │   │       ├── <140> Subscript
                    │   │       │   ├── <138> Subscript
                    │   │       │   │   ├── <136> Subscript
                    │   │       │   │   │   ├── <134> Unary [++]
                    │   │       │   │   │   │   ╰── <132> Var [outer_ptr]
                    │   │       │   │   │   ╰── <135> Constant Int [0]
                    │   │       │   │   ╰── <137> Constant Int [2]
                    │   │       │   ╰── <139> Constant Int [3]
                    │   │       ╰── <142> Constant Int [24]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <144> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <159>  [!=]
                    │   │       ├── <156> Subscript
                    │   │       │   ├── <154> Subscript
                    │   │       │   │   ├── <152> Subscript
                    │   │       │   │   │   ├── <150> Var [outer_ptr]
                    │   │       │   │   │   ╰── <151> Constant Int [0]
                    │   │       │   │   ╰── <153> Constant Int [2]
                    │   │       │   ╰── <155> Constant Int [3]
                    │   │       ╰── <158> Constant Int [24]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <160> Constant Int [4]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── inner_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Array
                    │   │           ├── 4
                    │   │           ╰── Long
                    │   ╰── Initializer
                    │       ╰── <180>  [+]
                    │           ├── <177> Subscript
                    │           │   ├── <175> Var [arr]
                    │           │   ╰── <176> Constant Int [0]
                    │           ╰── <179> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <193>  [!=]
                    │   │       ├── <190> Subscript
                    │   │       │   ├── <188> Subscript
                    │   │       │   │   ├── <186> Postfix [++]
                    │   │       │   │   │   ╰── <184> Var [inner_ptr]
                    │   │       │   │   ╰── <187> Constant Int [0]
                    │   │       │   ╰── <189> Constant Int [2]
                    │   │       ╰── <192> Constant Int [7]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <194> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <207>  [!=]
                    │   │       ├── <204> Subscript
                    │   │       │   ├── <202> Subscript
                    │   │       │   │   ├── <200> Var [inner_ptr]
                    │   │       │   │   ╰── <201> Constant Int [0]
                    │   │       │   ╰── <203> Constant Int [2]
                    │   │       ╰── <206> Constant Int [11]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <208> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <224>  [!=]
                    │   │       ├── <221> Subscript
                    │   │       │   ├── <219> Subscript
                    │   │       │   │   ├── <217> Unary [--]
                    │   │       │   │   │   ╰── <215> Var [inner_ptr]
                    │   │       │   │   ╰── <218> Constant Int [0]
                    │   │       │   ╰── <220> Constant Int [1]
                    │   │       ╰── <223> Constant Int [6]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <225> Constant Int [7]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── scalar_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Long
                    │   ╰── Initializer
                    │       ╰── <240> Subscript
                    │           ├── <238> Subscript
                    │           │   ├── <236> Var [arr]
                    │           │   ╰── <237> Constant Int [1]
                    │           ╰── <239> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <251>  [!=]
                    │   │       ├── <248> Subscript
                    │   │       │   ├── <246> Postfix [--]
                    │   │       │   │   ╰── <244> Var [scalar_ptr]
                    │   │       │   ╰── <247> Constant Int [2]
                    │   │       ╰── <250> Constant Int [23]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <252> Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <263>  [!=]
                    │   │       ├── <260> Subscript
                    │   │       │   ├── <258> Var [scalar_ptr]
                    │   │       │   ╰── <259> Constant Int [2]
                    │   │       ╰── <262> Constant Int [22]
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
                    │           ├── <12> Constant Double [+0e0]
                    │           ├── <14> Constant Double [+1e0]
                    │           ╰── <16> Constant Double [+2e0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Double
                    │   ╰── Initializer
                    │       ╰── <26> Var [x]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <39>  [!=]
                    │   │       ├── <32> Unary [++]
                    │   │       │   ╰── <31> Var [ptr]
                    │   │       ╰── <38>  [+]
                    │   │           ├── <35> Var [x]
                    │   │           ╰── <37> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <40> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <50>  [!=]
                    │   │       ├── <47> Dereference
                    │   │       │   ╰── <46> Var [ptr]
                    │   │       ╰── <49> Constant Double [+1e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <51> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <66>  [!=]
                    │   │       ├── <59> Postfix [++]
                    │   │       │   ╰── <57> Var [ptr]
                    │   │       ╰── <65>  [+]
                    │   │           ├── <62> Var [x]
                    │   │           ╰── <64> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <67> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <80>  [!=]
                    │   │       ├── <73> Var [ptr]
                    │   │       ╰── <79>  [+]
                    │   │           ├── <76> Var [x]
                    │   │           ╰── <78> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <81> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <91>  [!=]
                    │   │       ├── <88> Dereference
                    │   │       │   ╰── <87> Var [ptr]
                    │   │       ╰── <90> Constant Double [+2e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <92> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <107>  [!=]
                    │   │       ├── <100> Unary [--]
                    │   │       │   ╰── <99> Var [ptr]
                    │   │       ╰── <106>  [+]
                    │   │           ├── <103> Var [x]
                    │   │           ╰── <105> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <108> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <118>  [!=]
                    │   │       ├── <115> Dereference
                    │   │       │   ╰── <114> Var [ptr]
                    │   │       ╰── <117> Constant Double [+1e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <119> Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <134>  [!=]
                    │   │       ├── <127> Postfix [--]
                    │   │       │   ╰── <125> Var [ptr]
                    │   │       ╰── <133>  [+]
                    │   │           ├── <130> Var [x]
                    │   │           ╰── <132> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <135> Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <145>  [!=]
                    │   │       ├── <142> Dereference
                    │   │       │   ╰── <141> Var [ptr]
                    │   │       ╰── <144> Constant Double [+0e0]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <146> Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <156>  [!=]
                    │   │       ├── <152> Var [ptr]
                    │   │       ╰── <155> Var [x]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <157> Constant Int [10]
                    ╰── Return
                        ╰── <162> Constant Int [0]
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
                    │           │   │   ├── <36> Constant Int [1]
                    │           │   │   ╰── <38> Constant Int [2]
                    │           │   ╰── Compound
                    │           │       ├── <41> Constant Int [3]
                    │           │       ╰── <43> Constant Int [4]
                    │           ├── Compound
                    │           │   ├── Compound
                    │           │   │   ├── <47> Constant Int [5]
                    │           │   │   ╰── <49> Constant Int [6]
                    │           │   ╰── Compound
                    │           │       ├── <52> Constant Int [7]
                    │           │       ╰── <54> Constant Int [8]
                    │           ╰── Compound
                    │               ├── Compound
                    │               │   ├── <58> Constant Int [9]
                    │               │   ╰── <60> Constant Int [10]
                    │               ╰── Compound
                    │                   ├── <63> Constant Int [11]
                    │                   ╰── <65> Constant Int [12]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <86>  [!=]
                    │   │       ├── <83> Postfix [++]
                    │   │       │   ╰── <81> Subscript
                    │   │       │       ├── <78> Subscript
                    │   │       │       │   ├── <75> Subscript
                    │   │       │       │   │   ├── <72> Var [arr]
                    │   │       │       │   │   ╰── <74> Var [i]
                    │   │       │       │   ╰── <77> Var [j]
                    │   │       │       ╰── <80> Var [k]
                    │   │       ╰── <85> Constant Int [11]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <87> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <105>  [!=]
                    │   │       ├── <102> Subscript
                    │   │       │   ├── <99> Subscript
                    │   │       │   │   ├── <96> Subscript
                    │   │       │   │   │   ├── <93> Var [arr]
                    │   │       │   │   │   ╰── <95> Var [i]
                    │   │       │   │   ╰── <98> Var [j]
                    │   │       │   ╰── <101> Var [k]
                    │   │       ╰── <104> Constant Int [12]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <106> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <132>  [!=]
                    │   │       ├── <129> Unary [++]
                    │   │       │   ╰── <128> Subscript
                    │   │       │       ├── <123> Subscript
                    │   │       │       │   ├── <118> Subscript
                    │   │       │       │   │   ├── <113> Var [arr]
                    │   │       │       │   │   ╰── <117> Unary [--]
                    │   │       │       │   │       ╰── <116> Var [i]
                    │   │       │       │   ╰── <122> Postfix [--]
                    │   │       │       │       ╰── <120> Var [j]
                    │   │       │       ╰── <127> Unary [++]
                    │   │       │           ╰── <126> Var [k]
                    │   │       ╰── <131> Constant Int [9]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <133> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <151>  [!=]
                    │   │       ├── <148> Subscript
                    │   │       │   ├── <145> Subscript
                    │   │       │   │   ├── <142> Subscript
                    │   │       │   │   │   ├── <139> Var [arr]
                    │   │       │   │   │   ╰── <141> Var [i]
                    │   │       │   │   ╰── <144> Var [j]
                    │   │       │   ╰── <147> Var [k]
                    │   │       ╰── <150> Constant Int [6]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <152> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <172>  [!=]
                    │   │       ├── <169> Unary [--]
                    │   │       │   ╰── <168> Subscript
                    │   │       │       ├── <165> Subscript
                    │   │       │       │   ├── <162> Subscript
                    │   │       │       │   │   ├── <159> Var [arr]
                    │   │       │       │   │   ╰── <161> Var [i]
                    │   │       │       │   ╰── <164> Var [j]
                    │   │       │       ╰── <167> Var [k]
                    │   │       ╰── <171> Constant Int [5]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <173> Constant Int [5]
                    ╰── Return
                        ╰── <178> Constant Int [0]
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
                    │           ├── <18> Constant Int [1]
                    │           ├── <20> Constant Int [2]
                    │           ├── <22> Constant Int [3]
                    │           ├── <24> Constant Int [4]
                    │           ╰── <26> Constant Int [5]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <39>  [+]
                    │           ├── <36> Var [arr]
                    │           ╰── <38> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── result
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <53> Unary [++]
                    │           ╰── <52> Subscript
                    │               ├── <49> Postfix [--]
                    │               │   ╰── <47> Var [ptr]
                    │               ╰── <51> Var [idx]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <60>  [!=]
                    │   │       ├── <57> Var [result]
                    │   │       ╰── <59> Constant Int [6]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <61> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <71>  [!=]
                    │   │       ├── <68> Dereference
                    │   │       │   ╰── <67> Var [ptr]
                    │   │       ╰── <70> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <72> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <82>  [!=]
                    │   │       ├── <78> Var [ptr]
                    │   │       ╰── <81> Var [arr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <83> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <95>  [!=]
                    │   │       ├── <92> Dereference
                    │   │       │   ╰── <91> Postfix [++]
                    │   │       │       ╰── <89> Var [ptr]
                    │   │       ╰── <94> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <96> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <106>  [!=]
                    │   │       ├── <103> Dereference
                    │   │       │   ╰── <102> Var [ptr]
                    │   │       ╰── <105> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <107> Constant Int [5]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <115> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <123>  [<]
                    │   │       ├── <120> Var [i]
                    │   │       ╰── <122> Constant Int [4]
                    │   ├── Condition
                    │   │   ╰── <127> Postfix [++]
                    │   │       ╰── <125> Var [i]
                    │   ╰── Block
                    │       ╰── If
                    │           ├── Condition
                    │           │   ╰── <139>  [!=]
                    │           │       ├── <132> Subscript
                    │           │       │   ├── <129> Var [arr]
                    │           │       │   ╰── <131> Var [i]
                    │           │       ╰── <138>  [+]
                    │           │           ├── <135> Var [i]
                    │           │           ╰── <137> Constant Int [1]
                    │           ╰── Then
                    │               ╰── Block
                    │                   ╰── Return
                    │                       ╰── <140> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <154>  [!=]
                    │   │       ├── <151> Subscript
                    │   │       │   ├── <149> Var [arr]
                    │   │       │   ╰── <150> Constant Int [4]
                    │   │       ╰── <153> Constant Int [6]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <155> Constant Int [7]
                    ╰── Return
                        ╰── <160> Constant Int [0]
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
            │       │           ├── <12> Constant ULong [18446744073709551615]
            │       │           ├── <14> Constant ULong [9223372036854775807]
            │       │           ╰── <16> Constant ULong [100]
            │       ╰── Return
            │           ╰── <45>  [&&]
            │               ├── <35>  [&&]
            │               │   ├── <26>  [==]
            │               │   │   ├── <23> Subscript
            │               │   │   │   ├── <21> Var [arr]
            │               │   │   │   ╰── <22> Constant Int [0]
            │               │   │   ╰── <25> Constant ULong [18446744073709551615]
            │               │   ╰── <34>  [==]
            │               │       ├── <31> Subscript
            │               │       │   ├── <29> Var [arr]
            │               │       │   ╰── <30> Constant Int [1]
            │               │       ╰── <33> Constant ULong [9223372036854775807]
            │               ╰── <43>  [==]
            │                   ├── <40> Subscript
            │                   │   ├── <38> Var [arr]
            │                   │   ╰── <39> Constant Int [2]
            │                   ╰── <42> Constant ULong [100]
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
            │       │           ├── <60> Constant Double [+1e0]
            │       │           ╰── <62> Constant Double [+1.23e6]
            │       ╰── Return
            │           ╰── <106>  [&&]
            │               ├── <97>  [&&]
            │               │   ├── <89>  [&&]
            │               │   │   ├── <81>  [&&]
            │               │   │   │   ├── <72>  [==]
            │               │   │   │   │   ├── <69> Subscript
            │               │   │   │   │   │   ├── <67> Var [arr]
            │               │   │   │   │   │   ╰── <68> Constant Int [0]
            │               │   │   │   │   ╰── <71> Constant Double [+1e0]
            │               │   │   │   ╰── <80>  [==]
            │               │   │   │       ├── <77> Subscript
            │               │   │   │       │   ├── <75> Var [arr]
            │               │   │   │       │   ╰── <76> Constant Int [1]
            │               │   │   │       ╰── <79> Constant Double [+1.23e6]
            │               │   │   ╰── <88> Unary [!]
            │               │   │       ╰── <87> Subscript
            │               │   │           ├── <85> Var [arr]
            │               │   │           ╰── <86> Constant Int [2]
            │               │   ╰── <96> Unary [!]
            │               │       ╰── <95> Subscript
            │               │           ├── <93> Var [arr]
            │               │           ╰── <94> Constant Int [3]
            │               ╰── <104> Unary [!]
            │                   ╰── <103> Subscript
            │                       ├── <101> Var [arr]
            │                       ╰── <102> Constant Int [4]
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
            │       ├── <128> Assign [=]
            │       │   ├── <125> Dereference
            │       │   │   ╰── <124> Var [ptr]
            │       │   ╰── <127> Constant Int [1]
            │       ├── Function [extern three]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── var
            │       │   ├── Type
            │       │   │   ╰── Long
            │       │   ╰── Initializer
            │       │       ╰── <145>  [*]
            │       │           ├── <141> Var [negative_7billion]
            │       │           ╰── <144> FunctionCall [three]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── arr
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 5
            │       │   │       ╰── Long
            │       │   ╰── Initializer
            │       │       ╰── Compound
            │       │           ├── <155> Var [negative_7billion]
            │       │           ├── <161>  [*]
            │       │           │   ├── <158> FunctionCall [three]
            │       │           │   ╰── <160> Constant Long [7]
            │       │           ├── <170> Unary [-]
            │       │           │   ╰── <169> Cast
            │       │           │       ├── Target
            │       │           │       │   ╰── Long
            │       │           │       ╰── Expression
            │       │           │           ╰── <168> Dereference
            │       │           │               ╰── <167> Var [ptr]
            │       │           ╰── <181>  [+]
            │       │               ├── <173> Var [var]
            │       │               ╰── <180> Conditional [?]
            │       │                   ├── <176> Var [negative_7billion]
            │       │                   ├── Then
            │       │                   │   ╰── <177> Constant Int [2]
            │       │                   ╰── Else
            │       │                       ╰── <178> Constant Int [3]
            │       ╰── Return
            │           ╰── <234>  [&&]
            │               ├── <224>  [&&]
            │               │   ├── <213>  [&&]
            │               │   │   ├── <202>  [&&]
            │               │   │   │   ├── <193>  [==]
            │               │   │   │   │   ├── <188> Subscript
            │               │   │   │   │   │   ├── <186> Var [arr]
            │               │   │   │   │   │   ╰── <187> Constant Int [0]
            │               │   │   │   │   ╰── <192> Unary [-]
            │               │   │   │   │       ╰── <191> Constant Long [7000000000]
            │               │   │   │   ╰── <201>  [==]
            │               │   │   │       ├── <198> Subscript
            │               │   │   │       │   ├── <196> Var [arr]
            │               │   │   │       │   ╰── <197> Constant Int [1]
            │               │   │   │       ╰── <200> Constant Long [21]
            │               │   │   ╰── <212>  [==]
            │               │   │       ├── <207> Subscript
            │               │   │       │   ├── <205> Var [arr]
            │               │   │       │   ╰── <206> Constant Int [2]
            │               │   │       ╰── <211> Unary [-]
            │               │   │           ╰── <210> Constant Long [1]
            │               │   ╰── <223>  [==]
            │               │       ├── <218> Subscript
            │               │       │   ├── <216> Var [arr]
            │               │       │   ╰── <217> Constant Int [3]
            │               │       ╰── <222> Unary [-]
            │               │           ╰── <221> Constant Long [20999999998]
            │               ╰── <232>  [==]
            │                   ├── <229> Subscript
            │                   │   ├── <227> Var [arr]
            │                   │   ╰── <228> Constant Int [4]
            │                   ╰── <231> Constant Long [0]
            ├── Function [three]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <243> Constant Int [3]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── global_one
            │   ├── Type
            │   │   ╰── Long
            │   ╰── Initializer
            │       ╰── <250> Constant Long [1]
            ├── Function [test_type_conversion]
            │   ├── Parameters
            │   │   ╰── Param
            │   │       ├── Name
            │   │       │   ╰── ptr
            │   │       ╰── Type
            │   │           ╰── Pointer
            │   │               ╰── Int
            │   ╰── Body
            │       ├── <270> Assign [=]
            │       │   ├── <265> Dereference
            │       │   │   ╰── <264> Var [ptr]
            │       │   ╰── <269> Unary [-]
            │       │       ╰── <268> Constant Int [100]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── arr
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 4
            │       │   │       ╰── Unsigned Long
            │       │   ╰── Initializer
            │       │       ╰── Compound
            │       │           ├── <278> Constant Double [+3.4587645138215895e18]
            │       │           ├── <282> Dereference
            │       │           │   ╰── <281> Var [ptr]
            │       │           ├── <287> Cast
            │       │           │   ├── Target
            │       │           │   │   ╰── Unsigned Int
            │       │           │   ╰── Expression
            │       │           │       ╰── <286> Constant ULong [18446744073709551615]
            │       │           ╰── <292> Unary [-]
            │       │               ╰── <291> Var [global_one]
            │       ╰── Return
            │           ╰── <330>  [&&]
            │               ├── <320>  [&&]
            │               │   ├── <311>  [&&]
            │               │   │   ├── <302>  [==]
            │               │   │   │   ├── <299> Subscript
            │               │   │   │   │   ├── <297> Var [arr]
            │               │   │   │   │   ╰── <298> Constant Int [0]
            │               │   │   │   ╰── <301> Constant ULong [3458764513821589504]
            │               │   │   ╰── <310>  [==]
            │               │   │       ├── <307> Subscript
            │               │   │       │   ├── <305> Var [arr]
            │               │   │       │   ╰── <306> Constant Int [1]
            │               │   │       ╰── <309> Constant ULong [18446744073709551516]
            │               │   ╰── <319>  [==]
            │               │       ├── <316> Subscript
            │               │       │   ├── <314> Var [arr]
            │               │       │   ╰── <315> Constant Int [2]
            │               │       ╰── <318> Constant UInt [4294967295]
            │               ╰── <328>  [==]
            │                   ├── <325> Subscript
            │                   │   ├── <323> Var [arr]
            │                   │   ╰── <324> Constant Int [3]
            │                   ╰── <327> Constant ULong [18446744073709551615]
            ├── Function [test_preserve_stack]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── i
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <344> Unary [-]
            │       │           ╰── <343> Constant Int [1]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── arr
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 3
            │       │   │       ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── Compound
            │       │           ├── <357>  [*]
            │       │           │   ├── <354> Var [global_one]
            │       │           │   ╰── <356> Constant Long [2]
            │       │           ╰── <364>  [+]
            │       │               ├── <360> Var [global_one]
            │       │               ╰── <363> FunctionCall [three]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── u
            │       │   ├── Type
            │       │   │   ╰── Unsigned Int
            │       │   ╰── Initializer
            │       │       ╰── <371> Constant Long [2684366905]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <380>  [!=]
            │       │   │       ├── <375> Var [i]
            │       │   │       ╰── <379> Unary [-]
            │       │   │           ╰── <378> Constant Int [1]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <381> Constant Int [0]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <390>  [!=]
            │       │   │       ├── <387> Var [u]
            │       │   │       ╰── <389> Constant Long [2684366905]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <391> Constant Int [0]
            │       ╰── Return
            │           ╰── <420>  [&&]
            │               ├── <411>  [&&]
            │               │   ├── <402>  [==]
            │               │   │   ├── <399> Subscript
            │               │   │   │   ├── <397> Var [arr]
            │               │   │   │   ╰── <398> Constant Int [0]
            │               │   │   ╰── <401> Constant Int [2]
            │               │   ╰── <410>  [==]
            │               │       ├── <407> Subscript
            │               │       │   ├── <405> Var [arr]
            │               │       │   ╰── <406> Constant Int [1]
            │               │       ╰── <409> Constant Int [4]
            │               ╰── <418> Unary [!]
            │                   ╰── <417> Subscript
            │                       ├── <415> Var [arr]
            │                       ╰── <416> Constant Int [2]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <432> Unary [!]
                    │   │       ╰── <431> FunctionCall [test_simple]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <433> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <441> Unary [!]
                    │   │       ╰── <440> FunctionCall [test_partial]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <442> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── negative_seven_billion
                    │   ├── Type
                    │   │   ╰── Long
                    │   ╰── Initializer
                    │       ╰── <452> Unary [-]
                    │           ╰── <451> Constant Long [7000000000]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── i
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <458> Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <469> Unary [!]
                    │   │       ╰── <468> FunctionCall [test_non_constant]
                    │   │           ├── <464> Var [negative_seven_billion]
                    │   │           ╰── <467> AddressOf
                    │   │               ╰── <466> Var [i]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <470> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <481> Unary [!]
                    │   │       ╰── <480> FunctionCall [test_type_conversion]
                    │   │           ╰── <479> AddressOf
                    │   │               ╰── <478> Var [i]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <482> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <490> Unary [!]
                    │   │       ╰── <489> FunctionCall [test_preserve_stack]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <491> Constant Int [5]
                    ╰── Return
                        ╰── <496> Constant Int [0]
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
            │       │           │   ├── <15> Constant Int [1]
            │       │           │   ├── <17> Constant Int [2]
            │       │           │   ╰── <19> Constant Int [3]
            │       │           ├── Compound
            │       │           │   ├── <22> Constant Int [4]
            │       │           │   ├── <24> Constant Int [5]
            │       │           │   ╰── <26> Constant Int [6]
            │       │           ╰── Compound
            │       │               ├── <29> Constant Int [7]
            │       │               ├── <31> Constant Int [8]
            │       │               ╰── <33> Constant Int [9]
            │       ├── For
            │       │   ├── Init
            │       │   │   ╰── VarDeclaration
            │       │   │       ├── Name
            │       │   │       │   ╰── i
            │       │   │       ├── Type
            │       │   │       │   ╰── Int
            │       │   │       ╰── Initializer
            │       │   │           ╰── <41> Constant Int [0]
            │       │   ├── Condition
            │       │   │   ╰── <49>  [<]
            │       │   │       ├── <46> Var [i]
            │       │   │       ╰── <48> Constant Int [3]
            │       │   ├── Condition
            │       │   │   ╰── <58> Assign [=]
            │       │   │       ├── <51> Var [i]
            │       │   │       ╰── <57>  [+]
            │       │   │           ├── <54> Var [i]
            │       │   │           ╰── <56> Constant Int [1]
            │       │   ╰── Block
            │       │       ╰── For
            │       │           ├── Init
            │       │           │   ╰── VarDeclaration
            │       │           │       ├── Name
            │       │           │       │   ╰── j
            │       │           │       ├── Type
            │       │           │       │   ╰── Int
            │       │           │       ╰── Initializer
            │       │           │           ╰── <62> Constant Int [0]
            │       │           ├── Condition
            │       │           │   ╰── <70>  [<]
            │       │           │       ├── <67> Var [j]
            │       │           │       ╰── <69> Constant Int [3]
            │       │           ├── Condition
            │       │           │   ╰── <79> Assign [=]
            │       │           │       ├── <72> Var [j]
            │       │           │       ╰── <78>  [+]
            │       │           │           ├── <75> Var [j]
            │       │           │           ╰── <77> Constant Int [1]
            │       │           ╰── Block
            │       │               ╰── If
            │       │                   ├── Condition
            │       │                   │   ╰── <101>  [!=]
            │       │                   │       ├── <87> Subscript
            │       │                   │       │   ├── <84> Subscript
            │       │                   │       │   │   ├── <81> Var [arr]
            │       │                   │       │   │   ╰── <83> Var [i]
            │       │                   │       │   ╰── <86> Var [j]
            │       │                   │       ╰── <100>  [+]
            │       │                   │           ├── <97>  [+]
            │       │                   │           │   ├── <93>  [*]
            │       │                   │           │   │   ├── <90> Var [i]
            │       │                   │           │   │   ╰── <92> Constant Int [3]
            │       │                   │           │   ╰── <96> Var [j]
            │       │                   │           ╰── <99> Constant Int [1]
            │       │                   ╰── Then
            │       │                       ╰── Block
            │       │                           ╰── Return
            │       │                               ╰── <102> Constant Int [0]
            │       ╰── Return
            │           ╰── <113> Constant Int [1]
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
            │       │           │       ├── <134> Constant Int [1]
            │       │           │       ├── <136> Constant Int [2]
            │       │           │       ╰── <138> Constant Int [3]
            │       │           ╰── Compound
            │       │               ╰── Compound
            │       │                   ├── <142> Constant Int [4]
            │       │                   ├── <144> Constant Int [5]
            │       │                   ╰── <146> Constant Int [6]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── expected
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <155> Constant Int [1]
            │       ├── For
            │       │   ├── Init
            │       │   │   ╰── VarDeclaration
            │       │   │       ├── Name
            │       │   │       │   ╰── i
            │       │   │       ├── Type
            │       │   │       │   ╰── Int
            │       │   │       ╰── Initializer
            │       │   │           ╰── <161> Constant Int [0]
            │       │   ├── Condition
            │       │   │   ╰── <169>  [<]
            │       │   │       ├── <166> Var [i]
            │       │   │       ╰── <168> Constant Int [4]
            │       │   ├── Condition
            │       │   │   ╰── <178> Assign [=]
            │       │   │       ├── <171> Var [i]
            │       │   │       ╰── <177>  [+]
            │       │   │           ├── <174> Var [i]
            │       │   │           ╰── <176> Constant Int [1]
            │       │   ╰── Block
            │       │       ╰── For
            │       │           ├── Init
            │       │           │   ╰── VarDeclaration
            │       │           │       ├── Name
            │       │           │       │   ╰── j
            │       │           │       ├── Type
            │       │           │       │   ╰── Int
            │       │           │       ╰── Initializer
            │       │           │           ╰── <182> Constant Int [0]
            │       │           ├── Condition
            │       │           │   ╰── <190>  [<]
            │       │           │       ├── <187> Var [j]
            │       │           │       ╰── <189> Constant Int [2]
            │       │           ├── Condition
            │       │           │   ╰── <199> Assign [=]
            │       │           │       ├── <192> Var [j]
            │       │           │       ╰── <198>  [+]
            │       │           │           ├── <195> Var [j]
            │       │           │           ╰── <197> Constant Int [1]
            │       │           ╰── Block
            │       │               ╰── For
            │       │                   ├── Init
            │       │                   │   ╰── VarDeclaration
            │       │                   │       ├── Name
            │       │                   │       │   ╰── k
            │       │                   │       ├── Type
            │       │                   │       │   ╰── Int
            │       │                   │       ╰── Initializer
            │       │                   │           ╰── <203> Constant Int [0]
            │       │                   ├── Condition
            │       │                   │   ╰── <211>  [<]
            │       │                   │       ├── <208> Var [k]
            │       │                   │       ╰── <210> Constant Int [6]
            │       │                   ├── Condition
            │       │                   │   ╰── <220> Assign [=]
            │       │                   │       ├── <213> Var [k]
            │       │                   │       ╰── <219>  [+]
            │       │                   │           ├── <216> Var [k]
            │       │                   │           ╰── <218> Constant Int [1]
            │       │                   ╰── Block
            │       │                       ├── VarDeclaration
            │       │                       │   ├── Name
            │       │                       │   │   ╰── val
            │       │                       │   ├── Type
            │       │                       │   │   ╰── Int
            │       │                       │   ╰── Initializer
            │       │                       │       ╰── <234> Subscript
            │       │                       │           ├── <231> Subscript
            │       │                       │           │   ├── <228> Subscript
            │       │                       │           │   │   ├── <225> Var [first_half_only]
            │       │                       │           │   │   ╰── <227> Var [i]
            │       │                       │           │   ╰── <230> Var [j]
            │       │                       │           ╰── <233> Var [k]
            │       │                       ╰── If
            │       │                           ├── Condition
            │       │                           │   ╰── <255>  [||]
            │       │                           │       ├── <248>  [||]
            │       │                           │       │   ├── <241>  [>]
            │       │                           │       │   │   ├── <238> Var [i]
            │       │                           │       │   │   ╰── <240> Constant Int [1]
            │       │                           │       │   ╰── <247>  [>]
            │       │                           │       │       ├── <244> Var [j]
            │       │                           │       │       ╰── <246> Constant Int [0]
            │       │                           │       ╰── <254>  [>]
            │       │                           │           ├── <251> Var [k]
            │       │                           │           ╰── <253> Constant Int [2]
            │       │                           ├── Then
            │       │                           │   ╰── Block
            │       │                           │       ╰── If
            │       │                           │           ├── Condition
            │       │                           │           │   ╰── <257> Var [val]
            │       │                           │           ╰── Then
            │       │                           │               ╰── Block
            │       │                           │                   ╰── Return
            │       │                           │                       ╰── <258> Constant Int [0]
            │       │                           ╰── Else
            │       │                               ╰── Block
            │       │                                   ├── If
            │       │                                   │   ├── Condition
            │       │                                   │   │   ╰── <270>  [!=]
            │       │                                   │   │       ├── <266> Var [val]
            │       │                                   │   │       ╰── <269> Var [expected]
            │       │                                   │   ╰── Then
            │       │                                   │       ╰── Block
            │       │                                   │           ╰── Return
            │       │                                   │               ╰── <271> Constant Int [0]
            │       │                                   ╰── <284> Assign [=]
            │       │                                       ├── <277> Var [expected]
            │       │                                       ╰── <283>  [+]
            │       │                                           ├── <280> Var [expected]
            │       │                                           ╰── <282> Constant Int [1]
            │       ╰── Return
            │           ╰── <298> Constant Int [1]
            ├── Function [test_non_constant_and_type_conversion]
            │   ╰── Body
            │       ├── Function [extern three]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── x
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ├── Initializer
            │       │   │   ╰── <318> Constant Int [2000]
            │       │   ╰── Static
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── negative_four
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <326> Unary [-]
            │       │           ╰── <325> Constant Int [4]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ptr
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <336> AddressOf
            │       │           ╰── <335> Var [negative_four]
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
            │       │           │   ├── <349> Var [x]
            │       │           │   ╰── <357>  [/]
            │       │           │       ├── <352> Var [x]
            │       │           │       ╰── <356> Dereference
            │       │           │           ╰── <355> Var [ptr]
            │       │           ╰── Compound
            │       │               ╰── <361> FunctionCall [three]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <398>  [||]
            │       │   │       ├── <387>  [||]
            │       │   │       │   ├── <374>  [!=]
            │       │   │       │   │   ├── <371> Subscript
            │       │   │       │   │   │   ├── <369> Subscript
            │       │   │       │   │   │   │   ├── <367> Var [arr]
            │       │   │       │   │   │   │   ╰── <368> Constant Int [0]
            │       │   │       │   │   │   ╰── <370> Constant Int [0]
            │       │   │       │   │   ╰── <373> Constant Double [+2e3]
            │       │   │       │   ╰── <386>  [!=]
            │       │   │       │       ├── <381> Subscript
            │       │   │       │       │   ├── <379> Subscript
            │       │   │       │       │   │   ├── <377> Var [arr]
            │       │   │       │       │   │   ╰── <378> Constant Int [0]
            │       │   │       │       │   ╰── <380> Constant Int [1]
            │       │   │       │       ╰── <385> Unary [-]
            │       │   │       │           ╰── <384> Constant Double [+5e2]
            │       │   │       ╰── <397>  [!=]
            │       │   │           ├── <394> Subscript
            │       │   │           │   ├── <392> Subscript
            │       │   │           │   │   ├── <390> Var [arr]
            │       │   │           │   │   ╰── <391> Constant Int [1]
            │       │   │           │   ╰── <393> Constant Int [0]
            │       │   │           ╰── <396> Constant Double [+3e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <399> Constant Int [0]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <425>  [||]
            │       │   │       ├── <417>  [||]
            │       │   │       │   ├── <409> Subscript
            │       │   │       │   │   ├── <407> Subscript
            │       │   │       │   │   │   ├── <405> Var [arr]
            │       │   │       │   │   │   ╰── <406> Constant Int [1]
            │       │   │       │   │   ╰── <408> Constant Int [1]
            │       │   │       │   ╰── <416> Subscript
            │       │   │       │       ├── <414> Subscript
            │       │   │       │       │   ├── <412> Var [arr]
            │       │   │       │       │   ╰── <413> Constant Int [2]
            │       │   │       │       ╰── <415> Constant Int [0]
            │       │   │       ╰── <424> Subscript
            │       │   │           ├── <422> Subscript
            │       │   │           │   ├── <420> Var [arr]
            │       │   │           │   ╰── <421> Constant Int [2]
            │       │   │           ╰── <423> Constant Int [1]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <426> Constant Int [0]
            │       ╰── Return
            │           ╰── <431> Constant Int [1]
            ├── Function [three]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <440> Constant UInt [3]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── one
            │   ├── Type
            │   │   ╰── Long
            │   ╰── Initializer
            │       ╰── <447> Constant Long [1]
            ├── Function [test_preserve_stack]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── i
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <460> Unary [-]
            │       │           ╰── <459> Constant Int [1]
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
            │       │           │   ╰── <476>  [*]
            │       │           │       ├── <473> Var [one]
            │       │           │       ╰── <475> Constant Long [2]
            │       │           ╰── Compound
            │       │               ╰── <484>  [+]
            │       │                   ├── <480> Var [one]
            │       │                   ╰── <483> FunctionCall [three]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── u
            │       │   ├── Type
            │       │   │   ╰── Unsigned Int
            │       │   ╰── Initializer
            │       │       ╰── <492> Constant Long [2684366905]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <501>  [!=]
            │       │   │       ├── <496> Var [i]
            │       │   │       ╰── <500> Unary [-]
            │       │   │           ╰── <499> Constant Int [1]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <502> Constant Int [0]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <511>  [!=]
            │       │   │       ├── <508> Var [u]
            │       │   │       ╰── <510> Constant Long [2684366905]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <512> Constant Int [0]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <547>  [||]
            │       │   │       ├── <536>  [||]
            │       │   │       │   ├── <525>  [!=]
            │       │   │       │   │   ├── <522> Subscript
            │       │   │       │   │   │   ├── <520> Subscript
            │       │   │       │   │   │   │   ├── <518> Var [arr]
            │       │   │       │   │   │   │   ╰── <519> Constant Int [0]
            │       │   │       │   │   │   ╰── <521> Constant Int [0]
            │       │   │       │   │   ╰── <524> Constant Int [2]
            │       │   │       │   ╰── <535>  [!=]
            │       │   │       │       ├── <532> Subscript
            │       │   │       │       │   ├── <530> Subscript
            │       │   │       │       │   │   ├── <528> Var [arr]
            │       │   │       │       │   │   ╰── <529> Constant Int [1]
            │       │   │       │       │   ╰── <531> Constant Int [0]
            │       │   │       │       ╰── <534> Constant Int [4]
            │       │   │       ╰── <546>  [!=]
            │       │   │           ├── <543> Subscript
            │       │   │           │   ├── <541> Subscript
            │       │   │           │   │   ├── <539> Var [arr]
            │       │   │           │   │   ╰── <540> Constant Int [2]
            │       │   │           │   ╰── <542> Constant Int [0]
            │       │   │           ╰── <545> Constant Int [0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <548> Constant Int [0]
            │       ╰── Return
            │           ╰── <553> Constant Int [1]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <565> Unary [!]
                    │   │       ╰── <564> FunctionCall [test_simple]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <566> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <574> Unary [!]
                    │   │       ╰── <573> FunctionCall [test_partial]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <575> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <583> Unary [!]
                    │   │       ╰── <582> FunctionCall [test_non_constant_and_type_conversion]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <584> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <592> Unary [!]
                    │   │       ╰── <591> FunctionCall [test_preserve_stack]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <593> Constant Int [4]
                    ╰── Return
                        ╰── <598> Constant Int [0]
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
            │           ├── <7> Constant Double [+1e0]
            │           ├── <9> Constant Double [+2e0]
            │           ╰── <11> Constant Double [+3e0]
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
            │       │   │   ╰── <31>  [!=]
            │       │   │       ├── <28> Subscript
            │       │   │       │   ├── <26> Var [arr]
            │       │   │       │   ╰── <27> Constant Int [0]
            │       │   │       ╰── <30> Constant Double [+1e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <32> Constant Int [1]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <43>  [!=]
            │       │   │       ├── <40> Subscript
            │       │   │       │   ├── <38> Var [arr]
            │       │   │       │   ╰── <39> Constant Int [1]
            │       │   │       ╰── <42> Constant Double [+2e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <44> Constant Int [2]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <55>  [!=]
            │       │   │       ├── <52> Subscript
            │       │   │       │   ├── <50> Var [arr]
            │       │   │       │   ╰── <51> Constant Int [2]
            │       │   │       ╰── <54> Constant Double [+3e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <56> Constant Int [3]
            │       ╰── Return
            │           ╰── <61> Constant Int [0]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── uint_arr
            │   ├── Type
            │   │   ╰── Array
            │   │       ├── 5
            │   │       ╰── Unsigned Int
            │   ╰── Initializer
            │       ╰── Compound
            │           ├── <71> Constant UInt [1]
            │           ├── <73> Constant UInt [0]
            │           ╰── <75> Constant UInt [2147497230]
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
            │       │   │   ╰── <95>  [!=]
            │       │   │       ├── <92> Subscript
            │       │   │       │   ├── <90> Var [arr]
            │       │   │       │   ╰── <91> Constant Int [0]
            │       │   │       ╰── <94> Constant UInt [1]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <96> Constant Int [4]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <104> Subscript
            │       │   │       ├── <102> Var [arr]
            │       │   │       ╰── <103> Constant Int [1]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <105> Constant Int [5]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <116>  [!=]
            │       │   │       ├── <113> Subscript
            │       │   │       │   ├── <111> Var [arr]
            │       │   │       │   ╰── <112> Constant Int [2]
            │       │   │       ╰── <115> Constant UInt [2147497230]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <117> Constant Int [6]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <131>  [||]
            │       │   │       ├── <125> Subscript
            │       │   │       │   ├── <123> Var [arr]
            │       │   │       │   ╰── <124> Constant Int [3]
            │       │   │       ╰── <130> Subscript
            │       │   │           ├── <128> Var [arr]
            │       │   │           ╰── <129> Constant Int [4]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <132> Constant Int [7]
            │       ╰── Return
            │           ╰── <137> Constant Int [0]
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
            │       │   │           ╰── <161> Constant Int [0]
            │       │   ├── Condition
            │       │   │   ╰── <169>  [<]
            │       │   │       ├── <166> Var [i]
            │       │   │       ╰── <168> Constant Int [1000]
            │       │   ├── Condition
            │       │   │   ╰── <178> Assign [=]
            │       │   │       ├── <171> Var [i]
            │       │   │       ╰── <177>  [+]
            │       │   │           ├── <174> Var [i]
            │       │   │           ╰── <176> Constant Int [1]
            │       │   ╰── Block
            │       │       ╰── If
            │       │           ├── Condition
            │       │           │   ╰── <183> Subscript
            │       │           │       ├── <180> Var [arr]
            │       │           │       ╰── <182> Var [i]
            │       │           ╰── Then
            │       │               ╰── Block
            │       │                   ╰── Return
            │       │                       ╰── <184> Constant Int [8]
            │       ╰── Return
            │           ╰── <192> Constant Int [0]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── ulong_arr
            │   ├── Type
            │   │   ╰── Array
            │   │       ├── 4
            │   │       ╰── Unsigned Long
            │   ╰── Initializer
            │       ╰── Compound
            │           ├── <202> Constant Double [+1e2]
            │           ├── <204> Constant Int [11]
            │           ├── <206> Constant Long [12345]
            │           ╰── <208> Constant UInt [4294967295]
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
            │       │   │   ╰── <228>  [!=]
            │       │   │       ├── <225> Subscript
            │       │   │       │   ├── <223> Var [arr]
            │       │   │       │   ╰── <224> Constant Int [0]
            │       │   │       ╰── <227> Constant ULong [100]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <229> Constant Int [9]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <240>  [!=]
            │       │   │       ├── <237> Subscript
            │       │   │       │   ├── <235> Var [arr]
            │       │   │       │   ╰── <236> Constant Int [1]
            │       │   │       ╰── <239> Constant ULong [11]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <241> Constant Int [10]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <252>  [!=]
            │       │   │       ├── <249> Subscript
            │       │   │       │   ├── <247> Var [arr]
            │       │   │       │   ╰── <248> Constant Int [2]
            │       │   │       ╰── <251> Constant ULong [12345]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <253> Constant Int [11]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <264>  [!=]
            │       │   │       ├── <261> Subscript
            │       │   │       │   ├── <259> Var [arr]
            │       │   │       │   ╰── <260> Constant Int [3]
            │       │   │       ╰── <263> Constant ULong [4294967295]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <265> Constant Int [12]
            │       ╰── Return
            │           ╰── <270> Constant Int [0]
            ├── Function [test_global]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── check
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <285> FunctionCall [check_double_arr]
            │       │           ╰── <284> Var [double_arr]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <289> Var [check]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <291> Var [check]
            │       ├── <303> Assign [=]
            │       │   ├── <297> Var [check]
            │       │   ╰── <302> FunctionCall [check_uint_arr]
            │       │       ╰── <301> Var [uint_arr]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <306> Var [check]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <308> Var [check]
            │       ├── <320> Assign [=]
            │       │   ├── <314> Var [check]
            │       │   ╰── <319> FunctionCall [check_long_arr]
            │       │       ╰── <318> Var [long_arr]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <323> Var [check]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <325> Var [check]
            │       ├── <337> Assign [=]
            │       │   ├── <331> Var [check]
            │       │   ╰── <336> FunctionCall [check_ulong_arr]
            │       │       ╰── <335> Var [ulong_arr]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <340> Var [check]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <342> Var [check]
            │       ╰── Return
            │           ╰── <347> Constant Int [0]
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
            │       │           ├── <362> Constant Double [+1e0]
            │       │           ├── <364> Constant Double [+2e0]
            │       │           ╰── <366> Constant Double [+3e0]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── local_uint_arr
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 5
            │       │   │       ╰── Unsigned Int
            │       │   ├── Initializer
            │       │   │   ╰── Compound
            │       │   │       ├── <377> Constant UInt [1]
            │       │   │       ├── <379> Constant UInt [0]
            │       │   │       ╰── <381> Constant UInt [2147497230]
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
            │       │   │       ├── <400> Constant Double [+1e2]
            │       │   │       ├── <402> Constant Int [11]
            │       │   │       ├── <404> Constant Long [12345]
            │       │   │       ╰── <406> Constant UInt [4294967295]
            │       │   ╰── Static
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── check
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <416> FunctionCall [check_double_arr]
            │       │           ╰── <415> Var [local_double_arr]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <420> Var [check]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <425>  [+]
            │       │                   ├── <421> Constant Int [100]
            │       │                   ╰── <424> Var [check]
            │       ├── <437> Assign [=]
            │       │   ├── <431> Var [check]
            │       │   ╰── <436> FunctionCall [check_uint_arr]
            │       │       ╰── <435> Var [local_uint_arr]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <440> Var [check]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <445>  [+]
            │       │                   ├── <441> Constant Int [100]
            │       │                   ╰── <444> Var [check]
            │       ├── <457> Assign [=]
            │       │   ├── <451> Var [check]
            │       │   ╰── <456> FunctionCall [check_long_arr]
            │       │       ╰── <455> Var [local_long_arr]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <460> Var [check]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <465>  [+]
            │       │                   ├── <461> Constant Int [100]
            │       │                   ╰── <464> Var [check]
            │       ├── <477> Assign [=]
            │       │   ├── <471> Var [check]
            │       │   ╰── <476> FunctionCall [check_ulong_arr]
            │       │       ╰── <475> Var [local_ulong_arr]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <480> Var [check]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <485>  [+]
            │       │                   ├── <481> Constant Int [100]
            │       │                   ╰── <484> Var [check]
            │       ╰── Return
            │           ╰── <490> Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── check
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <503> FunctionCall [test_global]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <507> Var [check]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <509> Var [check]
                    ╰── Return
                        ╰── <515> FunctionCall [test_local]
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
            │           │   ├── <10> Constant Double [+1.1e0]
            │           │   ╰── <12> Constant Double [+2.2e0]
            │           ╰── Compound
            │               ├── <15> Constant Double [+3.3e0]
            │               ╰── <17> Constant Double [+4.4e0]
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
            │       │   │   ╰── <44>  [!=]
            │       │   │       ├── <41> Subscript
            │       │   │       │   ├── <39> Subscript
            │       │   │       │   │   ├── <37> Var [arr]
            │       │   │       │   │   ╰── <38> Constant Int [0]
            │       │   │       │   ╰── <40> Constant Int [0]
            │       │   │       ╰── <43> Constant Double [+1.1e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <45> Constant Int [1]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <58>  [!=]
            │       │   │       ├── <55> Subscript
            │       │   │       │   ├── <53> Subscript
            │       │   │       │   │   ├── <51> Var [arr]
            │       │   │       │   │   ╰── <52> Constant Int [0]
            │       │   │       │   ╰── <54> Constant Int [1]
            │       │   │       ╰── <57> Constant Double [+2.2e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <59> Constant Int [2]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <72>  [!=]
            │       │   │       ├── <69> Subscript
            │       │   │       │   ├── <67> Subscript
            │       │   │       │   │   ├── <65> Var [arr]
            │       │   │       │   │   ╰── <66> Constant Int [1]
            │       │   │       │   ╰── <68> Constant Int [0]
            │       │   │       ╰── <71> Constant Double [+3.3e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <73> Constant Int [3]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <86>  [!=]
            │       │   │       ├── <83> Subscript
            │       │   │       │   ├── <81> Subscript
            │       │   │       │   │   ├── <79> Var [arr]
            │       │   │       │   │   ╰── <80> Constant Int [1]
            │       │   │       │   ╰── <82> Constant Int [1]
            │       │   │       ╰── <85> Constant Double [+4.4e0]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <87> Constant Int [4]
            │       ╰── Return
            │           ╰── <92> Constant Int [0]
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
            │       │   │           ╰── <129> Constant Int [0]
            │       │   ├── Condition
            │       │   │   ╰── <137>  [<]
            │       │   │       ├── <134> Var [i]
            │       │   │       ╰── <136> Constant Int [30]
            │       │   ├── Condition
            │       │   │   ╰── <146> Assign [=]
            │       │   │       ├── <139> Var [i]
            │       │   │       ╰── <145>  [+]
            │       │   │           ├── <142> Var [i]
            │       │   │           ╰── <144> Constant Int [1]
            │       │   ╰── Block
            │       │       ╰── For
            │       │           ├── Init
            │       │           │   ╰── VarDeclaration
            │       │           │       ├── Name
            │       │           │       │   ╰── j
            │       │           │       ├── Type
            │       │           │       │   ╰── Int
            │       │           │       ╰── Initializer
            │       │           │           ╰── <150> Constant Int [0]
            │       │           ├── Condition
            │       │           │   ╰── <158>  [<]
            │       │           │       ├── <155> Var [j]
            │       │           │       ╰── <157> Constant Int [50]
            │       │           ├── Condition
            │       │           │   ╰── <167> Assign [=]
            │       │           │       ├── <160> Var [j]
            │       │           │       ╰── <166>  [+]
            │       │           │           ├── <163> Var [j]
            │       │           │           ╰── <165> Constant Int [1]
            │       │           ╰── Block
            │       │               ╰── For
            │       │                   ├── Init
            │       │                   │   ╰── VarDeclaration
            │       │                   │       ├── Name
            │       │                   │       │   ╰── k
            │       │                   │       ├── Type
            │       │                   │       │   ╰── Int
            │       │                   │       ╰── Initializer
            │       │                   │           ╰── <171> Constant Int [0]
            │       │                   ├── Condition
            │       │                   │   ╰── <179>  [<]
            │       │                   │       ├── <176> Var [k]
            │       │                   │       ╰── <178> Constant Int [40]
            │       │                   ├── Condition
            │       │                   │   ╰── <188> Assign [=]
            │       │                   │       ├── <181> Var [k]
            │       │                   │       ╰── <187>  [+]
            │       │                   │           ├── <184> Var [k]
            │       │                   │           ╰── <186> Constant Int [1]
            │       │                   ╰── Block
            │       │                       ╰── If
            │       │                           ├── Condition
            │       │                           │   ╰── <199> Subscript
            │       │                           │       ├── <196> Subscript
            │       │                           │       │   ├── <193> Subscript
            │       │                           │       │   │   ├── <190> Var [arr]
            │       │                           │       │   │   ╰── <192> Var [i]
            │       │                           │       │   ╰── <195> Var [j]
            │       │                           │       ╰── <198> Var [k]
            │       │                           ╰── Then
            │       │                               ╰── Block
            │       │                                   ╰── Return
            │       │                                       ╰── <200> Constant Int [5]
            │       ╰── Return
            │           ╰── <214> Constant Int [0]
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
            │           │   │   ╰── <230> Constant Double [+1.0003e3]
            │           │   ╰── Compound
            │           │       ╰── <233> Constant UInt [12]
            │           ╰── Compound
            │               ╰── Compound
            │                   ╰── <237> Constant Int [2]
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
            │       │   │           ╰── <263> Constant Int [0]
            │       │   ├── Condition
            │       │   │   ╰── <271>  [<]
            │       │   │       ├── <268> Var [i]
            │       │   │       ╰── <270> Constant Int [4]
            │       │   ├── Condition
            │       │   │   ╰── <280> Assign [=]
            │       │   │       ├── <273> Var [i]
            │       │   │       ╰── <279>  [+]
            │       │   │           ├── <276> Var [i]
            │       │   │           ╰── <278> Constant Int [1]
            │       │   ╰── Block
            │       │       ╰── For
            │       │           ├── Init
            │       │           │   ╰── VarDeclaration
            │       │           │       ├── Name
            │       │           │       │   ╰── j
            │       │           │       ├── Type
            │       │           │       │   ╰── Int
            │       │           │       ╰── Initializer
            │       │           │           ╰── <284> Constant Int [0]
            │       │           ├── Condition
            │       │           │   ╰── <292>  [<]
            │       │           │       ├── <289> Var [j]
            │       │           │       ╰── <291> Constant Int [6]
            │       │           ├── Condition
            │       │           │   ╰── <301> Assign [=]
            │       │           │       ├── <294> Var [j]
            │       │           │       ╰── <300>  [+]
            │       │           │           ├── <297> Var [j]
            │       │           │           ╰── <299> Constant Int [1]
            │       │           ╰── Block
            │       │               ╰── For
            │       │                   ├── Init
            │       │                   │   ╰── VarDeclaration
            │       │                   │       ├── Name
            │       │                   │       │   ╰── k
            │       │                   │       ├── Type
            │       │                   │       │   ╰── Int
            │       │                   │       ╰── Initializer
            │       │                   │           ╰── <305> Constant Int [0]
            │       │                   ├── Condition
            │       │                   │   ╰── <313>  [<]
            │       │                   │       ├── <310> Var [k]
            │       │                   │       ╰── <312> Constant Int [2]
            │       │                   ├── Condition
            │       │                   │   ╰── <322> Assign [=]
            │       │                   │       ├── <315> Var [k]
            │       │                   │       ╰── <321>  [+]
            │       │                   │           ├── <318> Var [k]
            │       │                   │           ╰── <320> Constant Int [1]
            │       │                   ╰── Block
            │       │                       ├── VarDeclaration
            │       │                       │   ├── Name
            │       │                       │   │   ╰── val
            │       │                       │   ├── Type
            │       │                       │   │   ╰── Int
            │       │                       │   ╰── Initializer
            │       │                       │       ╰── <336> Subscript
            │       │                       │           ├── <333> Subscript
            │       │                       │           │   ├── <330> Subscript
            │       │                       │           │   │   ├── <327> Var [arr]
            │       │                       │           │   │   ╰── <329> Var [i]
            │       │                       │           │   ╰── <332> Var [j]
            │       │                       │           ╰── <335> Var [k]
            │       │                       ╰── If
            │       │                           ├── Condition
            │       │                           │   ╰── <357>  [&&]
            │       │                           │       ├── <350>  [&&]
            │       │                           │       │   ├── <343>  [==]
            │       │                           │       │   │   ├── <340> Var [i]
            │       │                           │       │   │   ╰── <342> Constant Int [0]
            │       │                           │       │   ╰── <349>  [==]
            │       │                           │       │       ├── <346> Var [j]
            │       │                           │       │       ╰── <348> Constant Int [0]
            │       │                           │       ╰── <356>  [==]
            │       │                           │           ├── <353> Var [k]
            │       │                           │           ╰── <355> Constant Int [0]
            │       │                           ├── Then
            │       │                           │   ╰── Block
            │       │                           │       ╰── If
            │       │                           │           ├── Condition
            │       │                           │           │   ╰── <362>  [!=]
            │       │                           │           │       ├── <359> Var [val]
            │       │                           │           │       ╰── <361> Constant ULong [1000]
            │       │                           │           ╰── Then
            │       │                           │               ╰── Block
            │       │                           │                   ╰── Return
            │       │                           │                       ╰── <363> Constant Int [6]
            │       │                           ╰── Else
            │       │                               ╰── If
            │       │                                   ├── Condition
            │       │                                   │   ╰── <388>  [&&]
            │       │                                   │       ├── <381>  [&&]
            │       │                                   │       │   ├── <374>  [==]
            │       │                                   │       │   │   ├── <371> Var [i]
            │       │                                   │       │   │   ╰── <373> Constant Int [0]
            │       │                                   │       │   ╰── <380>  [==]
            │       │                                   │       │       ├── <377> Var [j]
            │       │                                   │       │       ╰── <379> Constant Int [1]
            │       │                                   │       ╰── <387>  [==]
            │       │                                   │           ├── <384> Var [k]
            │       │                                   │           ╰── <386> Constant Int [0]
            │       │                                   ├── Then
            │       │                                   │   ╰── Block
            │       │                                   │       ╰── If
            │       │                                   │           ├── Condition
            │       │                                   │           │   ╰── <393>  [!=]
            │       │                                   │           │       ├── <390> Var [val]
            │       │                                   │           │       ╰── <392> Constant ULong [12]
            │       │                                   │           ╰── Then
            │       │                                   │               ╰── Block
            │       │                                   │                   ╰── Return
            │       │                                   │                       ╰── <394> Constant Int [7]
            │       │                                   ╰── Else
            │       │                                       ╰── If
            │       │                                           ├── Condition
            │       │                                           │   ╰── <419>  [&&]
            │       │                                           │       ├── <412>  [&&]
            │       │                                           │       │   ├── <405>  [==]
            │       │                                           │       │   │   ├── <402> Var [i]
            │       │                                           │       │   │   ╰── <404> Constant Int [1]
            │       │                                           │       │   ╰── <411>  [==]
            │       │                                           │       │       ├── <408> Var [j]
            │       │                                           │       │       ╰── <410> Constant Int [0]
            │       │                                           │       ╰── <418>  [==]
            │       │                                           │           ├── <415> Var [k]
            │       │                                           │           ╰── <417> Constant Int [0]
            │       │                                           ├── Then
            │       │                                           │   ╰── Block
            │       │                                           │       ╰── If
            │       │                                           │           ├── Condition
            │       │                                           │           │   ╰── <424>  [!=]
            │       │                                           │           │       ├── <421> Var [val]
            │       │                                           │           │       ╰── <423> Constant ULong [2]
            │       │                                           │           ╰── Then
            │       │                                           │               ╰── Block
            │       │                                           │                   ╰── Return
            │       │                                           │                       ╰── <425> Constant Int [8]
            │       │                                           ╰── Else
            │       │                                               ╰── Block
            │       │                                                   ╰── If
            │       │                                                       ├── Condition
            │       │                                                       │   ╰── <433> Var [val]
            │       │                                                       ╰── Then
            │       │                                                           ╰── Block
            │       │                                                               ╰── Return
            │       │                                                                   ╰── <434> Constant Int [9]
            │       ╰── Return
            │           ╰── <453> Constant Int [0]
            ├── Function [test_global]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── check
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <468> FunctionCall [check_double_arr]
            │       │           ╰── <467> Var [double_arr]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <472> Var [check]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <474> Var [check]
            │       ├── <486> Assign [=]
            │       │   ├── <480> Var [check]
            │       │   ╰── <485> FunctionCall [check_long_arr]
            │       │       ╰── <484> Var [long_arr]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <489> Var [check]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <491> Var [check]
            │       ├── <503> Assign [=]
            │       │   ├── <497> Var [check]
            │       │   ╰── <502> FunctionCall [check_ulong_arr]
            │       │       ╰── <501> Var [ulong_arr]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <506> Var [check]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <508> Var [check]
            │       ╰── Return
            │           ╰── <513> Constant Int [0]
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
            │       │   │       │   ├── <532> Constant Double [+1.1e0]
            │       │   │       │   ╰── <534> Constant Double [+2.2e0]
            │       │   │       ╰── Compound
            │       │   │           ├── <537> Constant Double [+3.3e0]
            │       │   │           ╰── <539> Constant Double [+4.4e0]
            │       │   ╰── Static
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── check
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <550> FunctionCall [check_double_arr]
            │       │           ╰── <549> Var [local_double_arr]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <554> Var [check]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <559>  [+]
            │       │                   ├── <555> Constant Int [100]
            │       │                   ╰── <558> Var [check]
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
            │       ├── <585> Assign [=]
            │       │   ├── <579> Var [check]
            │       │   ╰── <584> FunctionCall [check_long_arr]
            │       │       ╰── <583> Var [local_long_arr]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <588> Var [check]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <593>  [+]
            │       │                   ├── <589> Constant Int [100]
            │       │                   ╰── <592> Var [check]
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
            │       │   │       │   │   ╰── <611> Constant Double [+1.0003e3]
            │       │   │       │   ╰── Compound
            │       │   │       │       ╰── <614> Constant UInt [12]
            │       │   │       ╰── Compound
            │       │   │           ╰── Compound
            │       │   │               ╰── <618> Constant Int [2]
            │       │   ╰── Static
            │       ├── <631> Assign [=]
            │       │   ├── <625> Var [check]
            │       │   ╰── <630> FunctionCall [check_ulong_arr]
            │       │       ╰── <629> Var [local_ulong_arr]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <634> Var [check]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <639>  [+]
            │       │                   ├── <635> Constant Int [100]
            │       │                   ╰── <638> Var [check]
            │       ╰── Return
            │           ╰── <644> Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── check
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <657> FunctionCall [test_global]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <661> Var [check]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <663> Var [check]
                    ╰── Return
                        ╰── <669> FunctionCall [test_local]
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
                    │           ├── <27> Constant Int [1]
                    │           ├── <29> Constant Int [2]
                    │           ╰── <31> Constant Int [3]
                    ╰── Return
                        ╰── <38> Subscript
                            ├── <36> Var [arr]
                            ╰── <37> Constant Int [2]
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
            │           ├── <7> Constant Int [1]
            │           ├── <9> Constant Int [2]
            │           ├── <11> Constant Int [3]
            │           ╰── <13> Constant Int [4]
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
                    │   │           ╰── <25> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <33>  [<]
                    │   │       ├── <30> Var [i]
                    │   │       ╰── <32> Constant Int [4]
                    │   ├── Condition
                    │   │   ╰── <42> Assign [=]
                    │   │       ├── <35> Var [i]
                    │   │       ╰── <41>  [+]
                    │   │           ├── <38> Var [i]
                    │   │           ╰── <40> Constant Int [1]
                    │   ╰── Block
                    │       ╰── <57> Assign [=]
                    │           ├── <47> Subscript
                    │           │   ├── <44> Var [arr]
                    │           │   ╰── <46> Var [i]
                    │           ╰── <56>  [*]
                    │               ├── <53> Subscript
                    │               │   ├── <50> Var [arr]
                    │               │   ╰── <52> Var [i]
                    │               ╰── <55> Constant Int [2]
                    ╰── Return
                        ╰── <62> Constant Int [0]
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
                    │       ╰── If
                    │           ├── Condition
                    │           │   ╰── <52>  [!=]
                    │           │       ├── <45> Subscript
                    │           │       │   ├── <42> Var [arr]
                    │           │       │   ╰── <44> Var [i]
                    │           │       ╰── <51>  [+]
                    │           │           ├── <48> Var [i]
                    │           │           ╰── <50> Constant Int [1]
                    │           ╰── Then
                    │               ╰── Block
                    │                   ╰── Return
                    │                       ╰── <57>  [+]
                    │                           ├── <54> Var [i]
                    │                           ╰── <56> Constant Int [1]
                    ├── <66> FunctionCall [double_each_element]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <71> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <79>  [<]
                    │   │       ├── <76> Var [i]
                    │   │       ╰── <78> Constant Int [4]
                    │   ├── Condition
                    │   │   ╰── <88> Assign [=]
                    │   │       ├── <81> Var [i]
                    │   │       ╰── <87>  [+]
                    │   │           ├── <84> Var [i]
                    │   │           ╰── <86> Constant Int [1]
                    │   ╰── Block
                    │       ╰── If
                    │           ├── Condition
                    │           │   ╰── <104>  [!=]
                    │           │       ├── <93> Subscript
                    │           │       │   ├── <90> Var [arr]
                    │           │       │   ╰── <92> Var [i]
                    │           │       ╰── <103>  [*]
                    │           │           ├── <100>  [+]
                    │           │           │   ├── <96> Var [i]
                    │           │           │   ╰── <98> Constant Int [1]
                    │           │           ╰── <102> Constant Int [2]
                    │           ╰── Then
                    │               ╰── Block
                    │                   ╰── Return
                    │                       ╰── <109>  [+]
                    │                           ├── <106> Var [i]
                    │                           ╰── <108> Constant Int [5]
                    ╰── Return
                        ╰── <117> Constant Int [0]
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
                        ╰── <31> Subscript
                            ├── <28> Var [arr]
                            ╰── <30> Var [idx]
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
                    │           │       ╰── <45> Constant Int [0]
                    │           ╰── Compound
                    │               ├── Compound
                    │               │   ├── <51> Unary [-]
                    │               │   │   ╰── <50> Constant Int [12]
                    │               │   ├── <55> Unary [-]
                    │               │   │   ╰── <54> Constant Int [13]
                    │               │   ├── <59> Unary [-]
                    │               │   │   ╰── <58> Constant Int [14]
                    │               │   ╰── <63> Unary [-]
                    │               │       ╰── <62> Constant Int [15]
                    │               ╰── Compound
                    │                   ╰── <68> Unary [-]
                    │                       ╰── <67> Constant Int [16]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── row_pointer
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Array
                    │   │           ├── 4
                    │   │           ╰── Long
                    │   ╰── Initializer
                    │       ╰── <87> FunctionCall [return_row]
                    │           ├── <85> Var [nested_array]
                    │           ╰── <86> Constant Int [1]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <93> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <101>  [<]
                    │   │       ├── <98> Var [i]
                    │   │       ╰── <100> Constant Int [3]
                    │   ├── Condition
                    │   │   ╰── <110> Assign [=]
                    │   │       ├── <103> Var [i]
                    │   │       ╰── <109>  [+]
                    │   │           ├── <106> Var [i]
                    │   │           ╰── <108> Constant Int [1]
                    │   ╰── Block
                    │       ╰── For
                    │           ├── Init
                    │           │   ╰── VarDeclaration
                    │           │       ├── Name
                    │           │       │   ╰── j
                    │           │       ├── Type
                    │           │       │   ╰── Int
                    │           │       ╰── Initializer
                    │           │           ╰── <114> Constant Int [0]
                    │           ├── Condition
                    │           │   ╰── <122>  [<]
                    │           │       ├── <119> Var [j]
                    │           │       ╰── <121> Constant Int [4]
                    │           ├── Condition
                    │           │   ╰── <131> Assign [=]
                    │           │       ├── <124> Var [j]
                    │           │       ╰── <130>  [+]
                    │           │           ├── <127> Var [j]
                    │           │           ╰── <129> Constant Int [1]
                    │           ╰── Block
                    │               ╰── If
                    │                   ├── Condition
                    │                   │   ╰── <151>  [!=]
                    │                   │       ├── <139> Subscript
                    │                   │       │   ├── <136> Subscript
                    │                   │       │   │   ├── <133> Var [row_pointer]
                    │                   │       │   │   ╰── <135> Var [i]
                    │                   │       │   ╰── <138> Var [j]
                    │                   │       ╰── <150> Subscript
                    │                   │           ├── <147> Subscript
                    │                   │           │   ├── <144> Subscript
                    │                   │           │   │   ├── <142> Var [nested_array]
                    │                   │           │   │   ╰── <143> Constant Int [1]
                    │                   │           │   ╰── <146> Var [i]
                    │                   │           ╰── <149> Var [j]
                    │                   ╰── Then
                    │                       ╰── Block
                    │                           ╰── Return
                    │                               ╰── <152> Constant Int [1]
                    ├── <171> Assign [=]
                    │   ├── <168> Subscript
                    │   │   ├── <166> Subscript
                    │   │   │   ├── <164> Var [row_pointer]
                    │   │   │   ╰── <165> Constant Int [2]
                    │   │   ╰── <167> Constant Int [1]
                    │   ╰── <170> Constant Int [100]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <183>  [!=]
                    │   │       ├── <180> Subscript
                    │   │       │   ├── <178> Subscript
                    │   │       │   │   ├── <176> Subscript
                    │   │       │   │   │   ├── <174> Var [nested_array]
                    │   │       │   │   │   ╰── <175> Constant Int [1]
                    │   │       │   │   ╰── <177> Constant Int [2]
                    │   │       │   ╰── <179> Constant Int [1]
                    │   │       ╰── <182> Constant Int [100]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <184> Constant Int [2]
                    ╰── Return
                        ╰── <189> Constant Int [0]
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
            │       │   │           ╰── <17> Constant Int [0]
            │       │   ├── Condition
            │       │   │   ╰── <25>  [<]
            │       │   │       ├── <22> Var [i]
            │       │   │       ╰── <24> Constant Int [5]
            │       │   ├── Condition
            │       │   │   ╰── <34> Assign [=]
            │       │   │       ├── <27> Var [i]
            │       │   │       ╰── <33>  [+]
            │       │   │           ├── <30> Var [i]
            │       │   │           ╰── <32> Constant Int [1]
            │       │   ╰── Block
            │       │       ╰── If
            │       │           ├── Condition
            │       │           │   ╰── <39> Subscript
            │       │           │       ├── <36> Var [arr]
            │       │           │       ╰── <38> Var [i]
            │       │           ╰── Then
            │       │               ╰── Block
            │       │                   ╰── Return
            │       │                       ╰── <40> Constant Int [1]
            │       ├── <55> Assign [=]
            │       │   ├── <52> Subscript
            │       │   │   ├── <49> Var [arr]
            │       │   │   ╰── <51> Var [idx]
            │       │   ╰── <54> Constant Int [8]
            │       ╰── Return
            │           ╰── <57> Constant Int [0]
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
                    │   │           ╰── <84> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <92>  [<]
                    │   │       ├── <89> Var [x]
                    │   │       ╰── <91> Constant Int [3]
                    │   ├── Condition
                    │   │   ╰── <101> Assign [=]
                    │   │       ├── <94> Var [x]
                    │   │       ╰── <100>  [+]
                    │   │           ├── <97> Var [x]
                    │   │           ╰── <99> Constant Int [1]
                    │   ╰── Block
                    │       ╰── For
                    │           ├── Init
                    │           │   ╰── VarDeclaration
                    │           │       ├── Name
                    │           │       │   ╰── y
                    │           │       ├── Type
                    │           │       │   ╰── Int
                    │           │       ╰── Initializer
                    │           │           ╰── <105> Constant Int [0]
                    │           ├── Condition
                    │           │   ╰── <113>  [<]
                    │           │       ├── <110> Var [y]
                    │           │       ╰── <112> Constant Int [2]
                    │           ├── Condition
                    │           │   ╰── <122> Assign [=]
                    │           │       ├── <115> Var [y]
                    │           │       ╰── <121>  [+]
                    │           │           ├── <118> Var [y]
                    │           │           ╰── <120> Constant Int [1]
                    │           ╰── Block
                    │               ├── VarDeclaration
                    │               │   ├── Name
                    │               │   │   ╰── expected
                    │               │   ├── Type
                    │               │   │   ╰── Int
                    │               │   ╰── Initializer
                    │               │       ╰── <139>  [+]
                    │               │           ├── <135>  [+]
                    │               │           │   ├── <128> Unary [-]
                    │               │           │   │   ╰── <127> Constant Int [10]
                    │               │           │   ╰── <134>  [*]
                    │               │           │       ├── <130> Constant Int [2]
                    │               │           │       ╰── <133> Var [x]
                    │               │           ╰── <138> Var [y]
                    │               ╰── If
                    │                   ├── Condition
                    │                   │   ╰── <153>  [!=]
                    │                   │       ├── <149> Subscript
                    │                   │       │   ├── <146> Subscript
                    │                   │       │   │   ├── <143> Var [arr]
                    │                   │       │   │   ╰── <145> Var [x]
                    │                   │       │   ╰── <148> Var [y]
                    │                   │       ╰── <152> Var [expected]
                    │                   ╰── Then
                    │                       ╰── Block
                    │                           ╰── Return
                    │                               ╰── <154> Constant Int [4]
                    ├── <175> Assign [=]
                    │   ├── <172> Subscript
                    │   │   ├── <169> Subscript
                    │   │   │   ├── <166> Var [arr]
                    │   │   │   ╰── <168> Var [i]
                    │   │   ╰── <171> Var [j]
                    │   ╰── <174> Constant Int [10]
                    ╰── Return
                        ╰── <177> Constant Int [0]
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
                    │           ├── <47> Constant Double [+0e0]
                    │           ├── <49> Constant Double [+0e0]
                    │           ├── <51> Constant Double [+0e0]
                    │           ├── <53> Constant Double [+0e0]
                    │           ╰── <55> Constant Double [+0e0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── check
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <66> FunctionCall [set_nth_element]
                    │           ├── <64> Var [arr]
                    │           ╰── <65> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <70> Var [check]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <72> Var [check]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <80> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <88>  [<]
                    │   │       ├── <85> Var [i]
                    │   │       ╰── <87> Constant Int [4]
                    │   ├── Condition
                    │   │   ╰── <97> Assign [=]
                    │   │       ├── <90> Var [i]
                    │   │       ╰── <96>  [+]
                    │   │           ├── <93> Var [i]
                    │   │           ╰── <95> Constant Int [1]
                    │   ╰── Block
                    │       ╰── If
                    │           ├── Condition
                    │           │   ╰── <105>  [!=]
                    │           │       ├── <102> Subscript
                    │           │       │   ├── <99> Var [arr]
                    │           │       │   ╰── <101> Var [i]
                    │           │       ╰── <104> Constant Int [0]
                    │           ╰── Then
                    │               ╰── Block
                    │                   ╰── Return
                    │                       ╰── <106> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <120>  [!=]
                    │   │       ├── <117> Subscript
                    │   │       │   ├── <115> Var [arr]
                    │   │       │   ╰── <116> Constant Int [4]
                    │   │       ╰── <119> Constant Int [8]
                    │   ╰── Then
                    │       ╰── Return
                    │           ╰── <121> Constant Int [3]
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
                    │           │   ├── <135> Unary [-]
                    │           │   │   ╰── <134> Constant Int [10]
                    │           │   ╰── <139> Unary [-]
                    │           │       ╰── <138> Constant Int [9]
                    │           ├── Compound
                    │           │   ├── <144> Unary [-]
                    │           │   │   ╰── <143> Constant Int [8]
                    │           │   ╰── <148> Unary [-]
                    │           │       ╰── <147> Constant Int [7]
                    │           ╰── Compound
                    │               ├── <153> Unary [-]
                    │               │   ╰── <152> Constant Int [6]
                    │               ╰── <157> Unary [-]
                    │                   ╰── <156> Constant Int [5]
                    ├── <171> Assign [=]
                    │   ├── <163> Var [check]
                    │   ╰── <170> FunctionCall [set_nested_element]
                    │       ├── <167> Var [nested_arr]
                    │       ├── <168> Constant Int [2]
                    │       ╰── <169> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <174> Var [check]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <176> Var [check]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <184> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <192>  [<]
                    │   │       ├── <189> Var [i]
                    │   │       ╰── <191> Constant Int [3]
                    │   ├── Condition
                    │   │   ╰── <201> Assign [=]
                    │   │       ├── <194> Var [i]
                    │   │       ╰── <200>  [+]
                    │   │           ├── <197> Var [i]
                    │   │           ╰── <199> Constant Int [1]
                    │   ╰── Block
                    │       ╰── For
                    │           ├── Init
                    │           │   ╰── VarDeclaration
                    │           │       ├── Name
                    │           │       │   ╰── j
                    │           │       ├── Type
                    │           │       │   ╰── Int
                    │           │       ╰── Initializer
                    │           │           ╰── <205> Constant Int [0]
                    │           ├── Condition
                    │           │   ╰── <213>  [<]
                    │           │       ├── <210> Var [j]
                    │           │       ╰── <212> Constant Int [2]
                    │           ├── Condition
                    │           │   ╰── <222> Assign [=]
                    │           │       ├── <215> Var [j]
                    │           │       ╰── <221>  [+]
                    │           │           ├── <218> Var [j]
                    │           │           ╰── <220> Constant Int [1]
                    │           ╰── Block
                    │               ╰── If
                    │                   ├── Condition
                    │                   │   ╰── <234>  [&&]
                    │                   │       ├── <227>  [==]
                    │                   │       │   ├── <224> Var [i]
                    │                   │       │   ╰── <226> Constant Int [2]
                    │                   │       ╰── <233>  [==]
                    │                   │           ├── <230> Var [j]
                    │                   │           ╰── <232> Constant Int [1]
                    │                   ├── Then
                    │                   │   ╰── Block
                    │                   │       ╰── If
                    │                   │           ├── Condition
                    │                   │           │   ╰── <245>  [!=]
                    │                   │           │       ├── <242> Subscript
                    │                   │           │       │   ├── <239> Subscript
                    │                   │           │       │   │   ├── <236> Var [nested_arr]
                    │                   │           │       │   │   ╰── <238> Var [i]
                    │                   │           │       │   ╰── <241> Var [j]
                    │                   │           │       ╰── <244> Constant Int [10]
                    │                   │           ╰── Then
                    │                   │               ╰── Block
                    │                   │                   ╰── Return
                    │                   │                       ╰── <246> Constant Int [5]
                    │                   ╰── Else
                    │                       ╰── Block
                    │                           ├── VarDeclaration
                    │                           │   ├── Name
                    │                           │   │   ╰── expected
                    │                           │   ├── Type
                    │                           │   │   ╰── Int
                    │                           │   ╰── Initializer
                    │                           │       ╰── <269>  [+]
                    │                           │           ├── <265>  [+]
                    │                           │           │   ├── <258> Unary [-]
                    │                           │           │   │   ╰── <257> Constant Int [10]
                    │                           │           │   ╰── <264>  [*]
                    │                           │           │       ├── <260> Constant Int [2]
                    │                           │           │       ╰── <263> Var [i]
                    │                           │           ╰── <268> Var [j]
                    │                           ╰── If
                    │                               ├── Condition
                    │                               │   ╰── <283>  [!=]
                    │                               │       ├── <279> Subscript
                    │                               │       │   ├── <276> Subscript
                    │                               │       │   │   ├── <273> Var [nested_arr]
                    │                               │       │   │   ╰── <275> Var [i]
                    │                               │       │   ╰── <278> Var [j]
                    │                               │       ╰── <282> Var [expected]
                    │                               ╰── Then
                    │                                   ╰── Block
                    │                                       ╰── Return
                    │                                           ╰── <284> Constant Int [6]
                    ╰── Return
                        ╰── <298> Constant Int [0]
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
                    │           ├── <12> Constant Int [1]
                    │           ╰── <14> Constant Int [2]
                    ├── <23> Assign [=]
                    │   ├── <20> Dereference
                    │   │   ╰── <19> Var [arr]
                    │   ╰── <22> Constant Int [3]
                    ├── <34> Assign [=]
                    │   ├── <31> Dereference
                    │   │   ╰── <30>  [+]
                    │   │       ├── <26> Var [arr]
                    │   │       ╰── <28> Constant Int [1]
                    │   ╰── <33> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <42>  [!=]
                    │   │       ├── <39> Subscript
                    │   │       │   ├── <37> Var [arr]
                    │   │       │   ╰── <38> Constant Int [0]
                    │   │       ╰── <41> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <43> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <54>  [!=]
                    │   │       ├── <51> Subscript
                    │   │       │   ├── <49> Var [arr]
                    │   │       │   ╰── <50> Constant Int [1]
                    │   │       ╰── <53> Constant Int [4]
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
            │           ╰── <21>  [>]
            │               ├── <17> Var [a]
            │               ╰── <20> Var [b]
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
            │           ╰── <45>  [<]
            │               ├── <41> Var [a]
            │               ╰── <44> Var [b]
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
            │           ╰── <69>  [>=]
            │               ├── <65> Var [a]
            │               ╰── <68> Var [b]
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
            │           ╰── <93>  [<=]
            │               ├── <89> Var [a]
            │               ╰── <92> Var [b]
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
            │           ╰── <125>  [>]
            │               ├── <121> Var [a]
            │               ╰── <124> Var [b]
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
            │           ╰── <157>  [>=]
            │               ├── <153> Var [a]
            │               ╰── <156> Var [b]
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
                    │       ╰── <182>  [+]
                    │           ├── <179> Var [arr]
                    │           ╰── <181> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── elem_4
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <194>  [+]
                    │           ├── <191> Var [arr]
                    │           ╰── <193> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <202> FunctionCall [gt]
                    │   │       ├── <199> Var [elem_1]
                    │   │       ╰── <201> Var [elem_4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <203> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <216> Unary [!]
                    │   │       ╰── <215> FunctionCall [lt]
                    │   │           ├── <211> Var [elem_1]
                    │   │           ╰── <213> Var [elem_4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <217> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <230> Unary [!]
                    │   │       ╰── <229> FunctionCall [ge]
                    │   │           ├── <225> Var [elem_1]
                    │   │           ╰── <227> Var [elem_1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <231> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <241> FunctionCall [le]
                    │   │       ├── <238> Var [elem_4]
                    │   │       ╰── <240> Var [elem_1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <242> Constant Int [4]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── one_past_the_end
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <256>  [+]
                    │           ├── <253> Var [arr]
                    │           ╰── <255> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <267> Unary [!]
                    │   │       ╰── <266> FunctionCall [gt]
                    │   │           ├── <262> Var [one_past_the_end]
                    │   │           ╰── <264> Var [elem_4]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <268> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <281>  [!=]
                    │   │       ├── <274> Var [one_past_the_end]
                    │   │       ╰── <280>  [+]
                    │   │           ├── <277> Var [elem_4]
                    │   │           ╰── <279> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <282> Constant Int [6]
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
                    │       ╰── <311>  [+]
                    │           ├── <308> Dereference
                    │           │   ╰── <307>  [+]
                    │           │       ├── <303> Var [nested_arr]
                    │           │       ╰── <305> Constant Int [3]
                    │           ╰── <310> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── elem_3_3
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <328>  [+]
                    │           ├── <325> Dereference
                    │           │   ╰── <324>  [+]
                    │           │       ├── <320> Var [nested_arr]
                    │           │       ╰── <322> Constant Int [3]
                    │           ╰── <327> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <336> FunctionCall [lt]
                    │   │       ├── <333> Var [elem_3_3]
                    │   │       ╰── <335> Var [elem_3_2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <337> Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <349> Unary [!]
                    │   │       ╰── <348> FunctionCall [ge]
                    │   │           ├── <345> Var [elem_3_3]
                    │   │           ╰── <347> Var [elem_3_2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <350> Constant Int [8]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── subarray_0
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Array
                    │   │           ├── 5
                    │   │           ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <365> Var [nested_arr]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── subarray_3
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Array
                    │   │           ├── 5
                    │   │           ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <381>  [+]
                    │           ├── <378> Var [nested_arr]
                    │           ╰── <380> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── subarray_one_past_the_end
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Array
                    │   │           ├── 5
                    │   │           ╰── Unsigned Long
                    │   ╰── Initializer
                    │       ╰── <397>  [+]
                    │           ├── <394> Var [nested_arr]
                    │           ╰── <396> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <405> FunctionCall [ge_nested]
                    │   │       ├── <402> Var [subarray_0]
                    │   │       ╰── <404> Var [subarray_3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <406> Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <419> Unary [!]
                    │   │       ╰── <418> FunctionCall [gt_nested]
                    │   │           ├── <414> Var [subarray_one_past_the_end]
                    │   │           ╰── <416> Var [subarray_3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <420> Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <433>  [!=]
                    │   │       ├── <426> Var [subarray_3]
                    │   │       ╰── <432>  [-]
                    │   │           ├── <429> Var [subarray_one_past_the_end]
                    │   │           ╰── <431> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <434> Constant Int [11]
                    ╰── Return
                        ╰── <439> Constant Int [0]
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
            │       │           ├── <12> Constant Int [0]
            │       │           ├── <14> Constant Int [0]
            │       │           ├── <16> Constant Int [3]
            │       │           ├── <18> Constant Int [0]
            │       │           ├── <20> Constant Int [0]
            │       │           ├── <22> Constant Int [0]
            │       │           ├── <24> Constant Int [0]
            │       │           ├── <26> Constant Int [0]
            │       │           ├── <28> Constant Int [0]
            │       │           ├── <30> Constant Int [0]
            │       │           ╰── <32> Constant Int [13]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ptr
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Long
            │       │   ╰── Initializer
            │       │       ╰── <45>  [+]
            │       │           ├── <42> Var [long_arr]
            │       │           ╰── <44> Constant Int [10]
            │       ╰── Return
            │           ╰── <53>  [==]
            │               ├── <50> Dereference
            │               │   ╰── <49> Var [ptr]
            │               ╰── <52> Constant Int [13]
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
            │       │           ├── <68> Constant Int [0]
            │       │           ├── <70> Constant Int [0]
            │       │           ├── <72> Constant Int [2]
            │       │           ├── <74> Constant Int [0]
            │       │           ├── <76> Constant Int [0]
            │       │           ├── <78> Constant Int [0]
            │       │           ├── <80> Constant Int [0]
            │       │           ├── <82> Constant Int [0]
            │       │           ├── <84> Constant Int [0]
            │       │           ├── <86> Constant Int [0]
            │       │           ╰── <88> Constant Int [42]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── end_ptr
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Unsigned Int
            │       │   ╰── Initializer
            │       │       ╰── <101>  [+]
            │       │           ├── <98> Var [unsigned_arr]
            │       │           ╰── <100> Constant Int [12]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ptr
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Unsigned Int
            │       │   ╰── Initializer
            │       │       ╰── <115>  [+]
            │       │           ├── <110> Var [end_ptr]
            │       │           ╰── <114> Unary [-]
            │       │               ╰── <113> Constant Int [10]
            │       ╰── Return
            │           ╰── <123>  [==]
            │               ├── <120> Dereference
            │               │   ╰── <119> Var [ptr]
            │               ╰── <122> Constant Int [2]
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
            │       │           ├── <138> Constant Int [0]
            │       │           ├── <140> Constant Int [98]
            │       │           ╰── <142> Constant Int [99]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ptr1
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <155>  [+]
            │       │           ├── <152> Var [int_arr]
            │       │           ╰── <154> Constant Int [2]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ptr2
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <167>  [+]
            │       │           ├── <163> Constant Int [2]
            │       │           ╰── <166> Var [int_arr]
            │       ╰── Return
            │           ╰── <184>  [&&]
            │               ├── <175>  [==]
            │               │   ├── <171> Var [ptr1]
            │               │   ╰── <174> Var [ptr2]
            │               ╰── <182>  [==]
            │                   ├── <179> Dereference
            │                   │   ╰── <178> Var [ptr2]
            │                   ╰── <181> Constant Int [99]
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
            │       │           ├── <199> Constant Int [0]
            │       │           ├── <201> Constant Int [0]
            │       │           ├── <203> Constant Int [0]
            │       │           ├── <205> Constant Int [0]
            │       │           ├── <207> Constant Int [0]
            │       │           ╰── <209> Constant Double [+6e0]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ptr1
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Double
            │       │   ╰── Initializer
            │       │       ╰── <222>  [+]
            │       │           ├── <219> Var [double_arr]
            │       │           ╰── <221> Constant Int [5]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ptr2
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Double
            │       │   ╰── Initializer
            │       │       ╰── <234>  [+]
            │       │           ├── <231> Var [double_arr]
            │       │           ╰── <233> Constant Long [5]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ptr3
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Double
            │       │   ╰── Initializer
            │       │       ╰── <246>  [+]
            │       │           ├── <243> Var [double_arr]
            │       │           ╰── <245> Constant UInt [5]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ptr4
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Double
            │       │   ╰── Initializer
            │       │       ╰── <258>  [+]
            │       │           ├── <255> Var [double_arr]
            │       │           ╰── <257> Constant ULong [5]
            │       ╰── Return
            │           ╰── <291>  [&&]
            │               ├── <282>  [&&]
            │               │   ├── <274>  [&&]
            │               │   │   ├── <266>  [==]
            │               │   │   │   ├── <262> Var [ptr1]
            │               │   │   │   ╰── <265> Var [ptr2]
            │               │   │   ╰── <273>  [==]
            │               │   │       ├── <269> Var [ptr1]
            │               │   │       ╰── <272> Var [ptr3]
            │               │   ╰── <281>  [==]
            │               │       ├── <277> Var [ptr1]
            │               │       ╰── <280> Var [ptr4]
            │               ╰── <289>  [==]
            │                   ├── <286> Dereference
            │                   │   ╰── <285> Var [ptr4]
            │                   ╰── <288> Constant Double [+6e0]
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
            │       │       ╰── <310> Unary [-]
            │       │           ╰── <309> Constant Int [2]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── small_int_ptr
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <320> AddressOf
            │       │           ╰── <319> Var [i]
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
            │       │   │       ├── <365> Constant Int [1]
            │       │   │       ├── <367> Constant Int [2]
            │       │   │       ├── <369> Constant Int [3]
            │       │   │       ╰── <371> Constant Int [4]
            │       │   ╰── Static
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ptr
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <401>  [+]
            │       │           ├── <387>  [+]
            │       │           │   ├── <381> FunctionCall [return_one]
            │       │           │   ╰── <386> Dereference
            │       │           │       ╰── <384> Var [small_int_ptr]
            │       │           ╰── <400> Conditional [?]
            │       │               ├── <390> Var [flag]
            │       │               ├── Then
            │       │               │   ╰── <394> FunctionCall [get_elem1_ptr]
            │       │               │       ╰── <393> Var [arr]
            │       │               ╰── Else
            │       │                   ╰── <398> FunctionCall [get_elem2_ptr]
            │       │                       ╰── <397> Var [arr]
            │       ╰── Return
            │           ╰── <421>  [&&]
            │               ├── <412>  [==]
            │               │   ├── <405> Var [ptr]
            │               │   ╰── <411>  [+]
            │               │       ├── <408> Var [arr]
            │               │       ╰── <410> Constant Int [1]
            │               ╰── <419>  [==]
            │                   ├── <416> Dereference
            │                   │   ╰── <415> Var [ptr]
            │                   ╰── <418> Constant Int [2]
            ├── Function [return_one]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <430> Constant Int [1]
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
            │           ╰── <450>  [+]
            │               ├── <447> Var [arr]
            │               ╰── <449> Constant Int [1]
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
            │           ╰── <470>  [+]
            │               ├── <467> Var [arr]
            │               ╰── <469> Constant Int [2]
            ├── Function [test_add_multi_dimensional]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── index
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ├── Initializer
            │       │   │   ╰── <483> Constant Int [2]
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
            │       │           │   ├── <495> Constant Int [1]
            │       │           │   ├── <497> Constant Int [2]
            │       │           │   ╰── <499> Constant Int [3]
            │       │           ├── Compound
            │       │           │   ├── <502> Constant Int [4]
            │       │           │   ├── <504> Constant Int [5]
            │       │           │   ╰── <506> Constant Int [6]
            │       │           ╰── Compound
            │       │               ├── <509> Constant Int [7]
            │       │               ├── <511> Constant Int [8]
            │       │               ╰── <513> Constant Int [9]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── row_pointer
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Array
            │       │   │           ├── 3
            │       │   │           ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <532>  [+]
            │       │           ├── <528> Var [nested_arr]
            │       │           ╰── <531> Var [index]
            │       ╰── Return
            │           ╰── <541>  [==]
            │               ├── <538> Dereference
            │               │   ╰── <537> Dereference
            │               │       ╰── <536> Var [row_pointer]
            │               ╰── <540> Constant Int [7]
            ├── Function [test_add_to_subarray_pointer]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── index
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ├── Initializer
            │       │   │   ╰── <554> Constant Int [2]
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
            │       │           │   ├── <566> Constant Int [1]
            │       │           │   ├── <568> Constant Int [2]
            │       │           │   ╰── <570> Constant Int [3]
            │       │           ├── Compound
            │       │           │   ├── <573> Constant Int [4]
            │       │           │   ├── <575> Constant Int [5]
            │       │           │   ╰── <577> Constant Int [6]
            │       │           ╰── Compound
            │       │               ├── <580> Constant Int [7]
            │       │               ├── <582> Constant Int [8]
            │       │               ╰── <584> Constant Int [9]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── row1
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <600> Dereference
            │       │           ╰── <599>  [+]
            │       │               ├── <595> Var [nested_arr]
            │       │               ╰── <597> Constant Int [1]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── elem_ptr
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <613>  [+]
            │       │           ├── <609> Var [row1]
            │       │           ╰── <612> Var [index]
            │       ╰── Return
            │           ╰── <621>  [==]
            │               ├── <618> Dereference
            │               │   ╰── <617> Var [elem_ptr]
            │               ╰── <620> Constant Int [6]
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
            │       │           ├── <636> Constant Int [10]
            │       │           ├── <638> Constant Int [9]
            │       │           ├── <640> Constant Int [8]
            │       │           ├── <642> Constant Int [7]
            │       │           ╰── <644> Constant Int [6]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── one_past_the_end
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Long
            │       │   ╰── Initializer
            │       │       ╰── <657>  [+]
            │       │           ├── <654> Var [long_arr]
            │       │           ╰── <656> Constant Int [5]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── index
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ├── Initializer
            │       │   │   ╰── <664> Constant Int [3]
            │       │   ╰── Static
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── subtraction_result
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Long
            │       │   ╰── Initializer
            │       │       ╰── <677>  [-]
            │       │           ├── <673> Var [one_past_the_end]
            │       │           ╰── <676> Var [index]
            │       ╰── Return
            │           ╰── <685>  [==]
            │               ├── <682> Dereference
            │               │   ╰── <681> Var [subtraction_result]
            │               ╰── <684> Constant Int [8]
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
            │       │           ├── <700> Constant Int [100]
            │       │           ├── <702> Constant Int [101]
            │       │           ├── <704> Constant Int [102]
            │       │           ├── <706> Constant Int [103]
            │       │           ╰── <708> Constant Int [104]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ptr
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Unsigned Int
            │       │   ╰── Initializer
            │       │       ╰── <724>  [-]
            │       │           ├── <718> Var [arr]
            │       │           ╰── <723> Unary [-]
            │       │               ╰── <721> Constant Int [3]
            │       ╰── Return
            │           ╰── <732>  [==]
            │               ├── <729> Dereference
            │               │   ╰── <728> Var [ptr]
            │               ╰── <731> Constant Int [103]
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
            │       │           ├── <747> Constant Int [0]
            │       │           ├── <749> Constant Int [0]
            │       │           ├── <751> Constant Int [0]
            │       │           ├── <753> Constant Int [0]
            │       │           ├── <755> Constant Int [0]
            │       │           ├── <757> Constant Int [0]
            │       │           ╰── <759> Constant Double [+6e0]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── end_ptr
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Double
            │       │   ╰── Initializer
            │       │       ╰── <772>  [+]
            │       │           ├── <769> Var [double_arr]
            │       │           ╰── <771> Constant Int [11]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ptr1
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Double
            │       │   ╰── Initializer
            │       │       ╰── <784>  [-]
            │       │           ├── <781> Var [end_ptr]
            │       │           ╰── <783> Constant Int [5]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ptr2
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Double
            │       │   ╰── Initializer
            │       │       ╰── <796>  [-]
            │       │           ├── <793> Var [end_ptr]
            │       │           ╰── <795> Constant Long [5]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ptr3
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Double
            │       │   ╰── Initializer
            │       │       ╰── <808>  [-]
            │       │           ├── <805> Var [end_ptr]
            │       │           ╰── <807> Constant UInt [5]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ptr4
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Double
            │       │   ╰── Initializer
            │       │       ╰── <820>  [-]
            │       │           ├── <817> Var [end_ptr]
            │       │           ╰── <819> Constant ULong [5]
            │       ╰── Return
            │           ╰── <853>  [&&]
            │               ├── <844>  [&&]
            │               │   ├── <836>  [&&]
            │               │   │   ├── <828>  [==]
            │               │   │   │   ├── <824> Var [ptr1]
            │               │   │   │   ╰── <827> Var [ptr2]
            │               │   │   ╰── <835>  [==]
            │               │   │       ├── <831> Var [ptr1]
            │               │   │       ╰── <834> Var [ptr3]
            │               │   ╰── <843>  [==]
            │               │       ├── <839> Var [ptr1]
            │               │       ╰── <842> Var [ptr4]
            │               ╰── <851>  [==]
            │                   ├── <848> Dereference
            │                   │   ╰── <847> Var [ptr4]
            │                   ╰── <850> Constant Double [+6e0]
            ├── Function [test_subtract_complex_expressions]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── flag
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ├── Initializer
            │       │   │   ╰── <866> Constant Int [1]
            │       │   ╰── Static
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── four
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ├── Initializer
            │       │   │   ╰── <873> Constant Int [4]
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
            │       │   │       ├── <883> Constant Int [1]
            │       │   │       ├── <885> Constant Int [2]
            │       │   │       ├── <887> Constant Int [3]
            │       │   │       ╰── <889> Constant Int [4]
            │       │   ╰── Static
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── ptr
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <919>  [-]
            │       │           ├── <909> Conditional [?]
            │       │           │   ├── <899> Var [flag]
            │       │           │   ├── Then
            │       │           │   │   ╰── <903> FunctionCall [get_elem1_ptr]
            │       │           │   │       ╰── <902> Var [arr]
            │       │           │   ╰── Else
            │       │           │       ╰── <907> FunctionCall [get_elem2_ptr]
            │       │           │           ╰── <906> Var [arr]
            │       │           ╰── <918>  [/]
            │       │               ├── <912> Var [four]
            │       │               ╰── <916> Unary [-]
            │       │                   ╰── <915> Constant Int [2]
            │       ╰── Return
            │           ╰── <928>  [==]
            │               ├── <924> Dereference
            │               │   ╰── <923> Var [ptr]
            │               ╰── <926> Constant Int [4]
            ├── Function [test_subtract_multi_dimensional]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── index
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ├── Initializer
            │       │   │   ╰── <941> Constant Int [1]
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
            │       │           │   ├── <953> Constant Int [1]
            │       │           │   ├── <955> Constant Int [2]
            │       │           │   ╰── <957> Constant Int [3]
            │       │           ├── Compound
            │       │           │   ├── <960> Constant Int [4]
            │       │           │   ├── <962> Constant Int [5]
            │       │           │   ╰── <964> Constant Int [6]
            │       │           ╰── Compound
            │       │               ├── <967> Constant Int [7]
            │       │               ├── <969> Constant Int [8]
            │       │               ╰── <971> Constant Int [9]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── last_row_pointer
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Array
            │       │   │           ├── 3
            │       │   │           ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <989>  [+]
            │       │           ├── <986> Var [nested_arr]
            │       │           ╰── <988> Constant Int [2]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── row_pointer
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Array
            │       │   │           ├── 3
            │       │   │           ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <1006>  [-]
            │       │           ├── <1002> Var [last_row_pointer]
            │       │           ╰── <1005> Var [index]
            │       ╰── Return
            │           ╰── <1016>  [==]
            │               ├── <1012> Dereference
            │               │   ╰── <1011> Dereference
            │               │       ╰── <1010> Var [row_pointer]
            │               ╰── <1014> Constant Int [4]
            ╰── Function [main]
                ╰── Body
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <1028> Unary [!]
                    │   │       ╰── <1027> FunctionCall [test_add_constant_to_pointer]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <1029> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <1037> Unary [!]
                    │   │       ╰── <1036> FunctionCall [test_add_negative_index]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <1038> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <1046> Unary [!]
                    │   │       ╰── <1045> FunctionCall [test_add_pointer_to_int]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <1047> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <1055> Unary [!]
                    │   │       ╰── <1054> FunctionCall [test_add_different_index_types]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <1056> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <1064> Unary [!]
                    │   │       ╰── <1063> FunctionCall [test_add_complex_expressions]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <1065> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <1073> Unary [!]
                    │   │       ╰── <1072> FunctionCall [test_add_multi_dimensional]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <1074> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <1082> Unary [!]
                    │   │       ╰── <1081> FunctionCall [test_add_to_subarray_pointer]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <1083> Constant Int [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <1091> Unary [!]
                    │   │       ╰── <1090> FunctionCall [test_subtract_from_pointer]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <1092> Constant Int [8]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <1100> Unary [!]
                    │   │       ╰── <1099> FunctionCall [test_subtract_negative_index]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <1101> Constant Int [9]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <1109> Unary [!]
                    │   │       ╰── <1108> FunctionCall [test_subtract_different_index_types]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <1110> Constant Int [10]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <1118> Unary [!]
                    │   │       ╰── <1117> FunctionCall [test_subtract_complex_expressions]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <1119> Constant Int [11]
                    ╰── Return
                        ╰── <1124> Constant Int [0]
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
            │           ╰── <22>  [-]
            │               ├── <17> Var [ptr2]
            │               ╰── <20> Var [ptr1]
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
            │           ╰── <47>  [-]
            │               ├── <42> Var [ptr2]
            │               ╰── <45> Var [ptr1]
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
            │           ╰── <86>  [-]
            │               ├── <81> Var [ptr2]
            │               ╰── <84> Var [ptr1]
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
            │           ╰── <119>  [-]
            │               ├── <114> Var [ptr2]
            │               ╰── <117> Var [ptr1]
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
                    │           ├── <134> Constant Int [5]
                    │           ├── <136> Constant Int [4]
                    │           ├── <138> Constant Int [3]
                    │           ├── <140> Constant Int [2]
                    │           ╰── <142> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── end_of_array
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <155>  [+]
                    │           ├── <152> Var [arr]
                    │           ╰── <154> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <166>  [!=]
                    │   │       ├── <163> FunctionCall [get_ptr_diff]
                    │   │       │   ├── <160> Var [arr]
                    │   │       │   ╰── <162> Var [end_of_array]
                    │   │       ╰── <165> Constant Int [5]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <167> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── long_arr
                    │   ╰── Type
                    │       ╰── Array
                    │           ├── 8
                    │           ╰── Long
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <192>  [!=]
                    │   │       ├── <187> FunctionCall [get_long_ptr_diff]
                    │   │       │   ├── <184>  [+]
                    │   │       │   │   ├── <181> Var [long_arr]
                    │   │       │   │   ╰── <183> Constant Int [3]
                    │   │       │   ╰── <186> Var [long_arr]
                    │   │       ╰── <191> Unary [-]
                    │   │           ╰── <190> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <193> Constant Int [2]
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
                    │   │   ╰── <233>  [!=]
                    │   │       ├── <230> FunctionCall [get_multidim_ptr_diff]
                    │   │       │   ├── <222>  [+]
                    │   │       │   │   ├── <219> Subscript
                    │   │       │   │   │   ├── <217> Var [multidim]
                    │   │       │   │   │   ╰── <218> Constant Int [2]
                    │   │       │   │   ╰── <221> Constant Int [1]
                    │   │       │   ╰── <229>  [+]
                    │   │       │       ├── <226> Subscript
                    │   │       │       │   ├── <224> Var [multidim]
                    │   │       │       │   ╰── <225> Constant Int [2]
                    │   │       │       ╰── <228> Constant Int [4]
                    │   │       ╰── <232> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <234> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <260>  [!=]
                    │   │       ├── <255> FunctionCall [get_multidim_ptr_diff_2]
                    │   │       │   ├── <248>  [+]
                    │   │       │   │   ├── <245> Subscript
                    │   │       │   │   │   ├── <243> Subscript
                    │   │       │   │   │   │   ├── <241> Var [multidim]
                    │   │       │   │   │   │   ╰── <242> Constant Int [2]
                    │   │       │   │   │   ╰── <244> Constant Int [2]
                    │   │       │   │   ╰── <247> Constant Int [2]
                    │   │       │   ╰── <254> Subscript
                    │   │       │       ├── <252> Subscript
                    │   │       │       │   ├── <250> Var [multidim]
                    │   │       │       │   ╰── <251> Constant Int [2]
                    │   │       │       ╰── <253> Constant Int [2]
                    │   │       ╰── <259> Unary [-]
                    │   │           ╰── <258> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <261> Constant Int [4]
                    ╰── Return
                        ╰── <266> Constant Int [0]
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
                    │   │           ╰── <19> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <27>  [<]
                    │   │       ├── <24> Var [i]
                    │   │       ╰── <26> Constant Int [300]
                    │   ├── Condition
                    │   │   ╰── <36> Assign [=]
                    │   │       ├── <29> Var [i]
                    │   │       ╰── <35>  [+]
                    │   │           ├── <32> Var [i]
                    │   │           ╰── <34> Constant Int [1]
                    │   ╰── Block
                    │       ╰── For
                    │           ├── Init
                    │           │   ╰── VarDeclaration
                    │           │       ├── Name
                    │           │       │   ╰── j
                    │           │       ├── Type
                    │           │       │   ╰── Int
                    │           │       ╰── Initializer
                    │           │           ╰── <40> Constant Int [0]
                    │           ├── Condition
                    │           │   ╰── <48>  [<]
                    │           │       ├── <45> Var [j]
                    │           │       ╰── <47> Constant Int [5]
                    │           ├── Condition
                    │           │   ╰── <57> Assign [=]
                    │           │       ├── <50> Var [j]
                    │           │       ╰── <56>  [+]
                    │           │           ├── <53> Var [j]
                    │           │           ╰── <55> Constant Int [1]
                    │           ╰── Block
                    │               ╰── <76> Assign [=]
                    │                   ├── <65> Subscript
                    │                   │   ├── <62> Subscript
                    │                   │   │   ├── <59> Var [x]
                    │                   │   │   ╰── <61> Var [i]
                    │                   │   ╰── <64> Var [j]
                    │                   ╰── <75>  [+]
                    │                       ├── <71>  [*]
                    │                       │   ├── <68> Var [i]
                    │                       │   ╰── <70> Constant Int [5]
                    │                       ╰── <74> Var [j]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <103>  [!=]
                    │   │       ├── <95> Dereference
                    │   │       │   ╰── <94>  [+]
                    │   │       │       ├── <90> Dereference
                    │   │       │       │   ╰── <89>  [+]
                    │   │       │       │       ├── <85> Var [x]
                    │   │       │       │       ╰── <87> Constant Int [20]
                    │   │       │       ╰── <92> Constant Int [3]
                    │   │       ╰── <102> Subscript
                    │   │           ├── <100> Subscript
                    │   │           │   ├── <98> Var [x]
                    │   │           │   ╰── <99> Constant Int [20]
                    │   │           ╰── <101> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <104> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <131>  [!=]
                    │   │       ├── <122> AddressOf
                    │   │       │   ╰── <121> Dereference
                    │   │       │       ╰── <119>  [+]
                    │   │       │           ├── <115> Dereference
                    │   │       │           │   ╰── <114>  [+]
                    │   │       │           │       ├── <110> Var [x]
                    │   │       │           │       ╰── <112> Constant Int [290]
                    │   │       │           ╰── <117> Constant Int [3]
                    │   │       ╰── <130> AddressOf
                    │   │           ╰── <129> Subscript
                    │   │               ├── <127> Subscript
                    │   │               │   ├── <125> Var [x]
                    │   │               │   ╰── <126> Constant Int [290]
                    │   │               ╰── <128> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <132> Constant Int [2]
                    ├── For
                    │   ├── Init
                    │   │   ╰── VarDeclaration
                    │   │       ├── Name
                    │   │       │   ╰── i
                    │   │       ├── Type
                    │   │       │   ╰── Int
                    │   │       ╰── Initializer
                    │   │           ╰── <140> Constant Int [0]
                    │   ├── Condition
                    │   │   ╰── <148>  [<]
                    │   │       ├── <145> Var [i]
                    │   │       ╰── <147> Constant Int [300]
                    │   ├── Condition
                    │   │   ╰── <157> Assign [=]
                    │   │       ├── <150> Var [i]
                    │   │       ╰── <156>  [+]
                    │   │           ├── <153> Var [i]
                    │   │           ╰── <155> Constant Int [1]
                    │   ╰── Block
                    │       ╰── For
                    │           ├── Init
                    │           │   ╰── VarDeclaration
                    │           │       ├── Name
                    │           │       │   ╰── j
                    │           │       ├── Type
                    │           │       │   ╰── Int
                    │           │       ╰── Initializer
                    │           │           ╰── <161> Constant Int [0]
                    │           ├── Condition
                    │           │   ╰── <169>  [<]
                    │           │       ├── <166> Var [j]
                    │           │       ╰── <168> Constant Int [5]
                    │           ├── Condition
                    │           │   ╰── <178> Assign [=]
                    │           │       ├── <171> Var [j]
                    │           │       ╰── <177>  [+]
                    │           │           ├── <174> Var [j]
                    │           │           ╰── <176> Constant Int [1]
                    │           ╰── Block
                    │               ╰── If
                    │                   ├── Condition
                    │                   │   ╰── <202>  [!=]
                    │                   │       ├── <192> Dereference
                    │                   │       │   ╰── <191>  [+]
                    │                   │       │       ├── <186> Dereference
                    │                   │       │       │   ╰── <185>  [+]
                    │                   │       │       │       ├── <180> Var [x]
                    │                   │       │       │       ╰── <183> Var [i]
                    │                   │       │       ╰── <189> Var [j]
                    │                   │       ╰── <201> Subscript
                    │                   │           ├── <198> Subscript
                    │                   │           │   ├── <195> Var [x]
                    │                   │           │   ╰── <197> Var [i]
                    │                   │           ╰── <200> Var [j]
                    │                   ╰── Then
                    │                       ╰── Block
                    │                           ╰── Return
                    │                               ╰── <203> Constant Int [3]
                    ├── <228> Assign [=]
                    │   ├── <225> Dereference
                    │   │   ╰── <224>  [+]
                    │   │       ├── <220> Dereference
                    │   │       │   ╰── <219>  [+]
                    │   │       │       ├── <215> Var [x]
                    │   │       │       ╰── <217> Constant Int [275]
                    │   │       ╰── <222> Constant Int [4]
                    │   ╰── <227> Constant ULong [22000]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <238>  [!=]
                    │   │       ├── <235> Subscript
                    │   │       │   ├── <233> Subscript
                    │   │       │   │   ├── <231> Var [x]
                    │   │       │   │   ╰── <232> Constant Int [275]
                    │   │       │   ╰── <234> Constant Int [4]
                    │   │       ╰── <237> Constant ULong [22000]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <239> Constant Int [4]
                    ╰── Return
                        ╰── <244> Constant Int [0]
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
                    │       ╰── <9> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── y
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── z
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <21> Constant Int [2]
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
                    │           ├── <34> AddressOf
                    │           │   ╰── <33> Var [x]
                    │           ├── <38> AddressOf
                    │           │   ╰── <37> Var [y]
                    │           ╰── <42> AddressOf
                    │               ╰── <41> Var [z]
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
                    │           ├── <56> AddressOf
                    │           │   ╰── <55> Var [z]
                    │           ├── <60> AddressOf
                    │           │   ╰── <59> Var [y]
                    │           ╰── <64> AddressOf
                    │               ╰── <63> Var [x]
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
                    │           ├── <84> AddressOf
                    │           │   ╰── <83> Var [arr]
                    │           ├── <88> AddressOf
                    │           │   ╰── <87> Var [arr2]
                    │           ╰── <92> AddressOf
                    │               ╰── <91> Var [arr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <111>  [!=]
                    │   │       ├── <99> Subscript
                    │   │       │   ├── <97> Var [array_of_pointers]
                    │   │       │   ╰── <98> Constant Int [0]
                    │   │       ╰── <110> Cast
                    │   │           ├── Target
                    │   │           │   ╰── Pointer
                    │   │           │       ╰── Array
                    │   │           │           ├── 3
                    │   │           │           ╰── Pointer
                    │   │           │               ╰── Int
                    │   │           ╰── Expression
                    │   │               ╰── <109> Var [arr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <112> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <132>  [!=]
                    │   │       ├── <120> Subscript
                    │   │       │   ├── <118> Var [array_of_pointers]
                    │   │       │   ╰── <119> Constant Int [1]
                    │   │       ╰── <131> Cast
                    │   │           ├── Target
                    │   │           │   ╰── Pointer
                    │   │           │       ╰── Array
                    │   │           │           ├── 3
                    │   │           │           ╰── Pointer
                    │   │           │               ╰── Int
                    │   │           ╰── Expression
                    │   │               ╰── <130> Var [arr2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <133> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <153>  [!=]
                    │   │       ├── <141> Subscript
                    │   │       │   ├── <139> Var [array_of_pointers]
                    │   │       │   ╰── <140> Constant Int [2]
                    │   │       ╰── <152> Cast
                    │   │           ├── Target
                    │   │           │   ╰── Pointer
                    │   │           │       ╰── Array
                    │   │           │           ├── 3
                    │   │           │           ╰── Pointer
                    │   │           │               ╰── Int
                    │   │           ╰── Expression
                    │   │               ╰── <151> Var [arr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <154> Constant Int [3]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <171>  [!=]
                    │   │       ├── <166> Subscript
                    │   │       │   ├── <164> Subscript
                    │   │       │   │   ├── <162> Subscript
                    │   │       │   │   │   ├── <160> Var [array_of_pointers]
                    │   │       │   │   │   ╰── <161> Constant Int [1]
                    │   │       │   │   ╰── <163> Constant Int [0]
                    │   │       │   ╰── <165> Constant Int [0]
                    │   │       ╰── <170> AddressOf
                    │   │           ╰── <169> Var [z]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <172> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <189>  [!=]
                    │   │       ├── <184> Subscript
                    │   │       │   ├── <182> Subscript
                    │   │       │   │   ├── <180> Subscript
                    │   │       │   │   │   ├── <178> Var [array_of_pointers]
                    │   │       │   │   │   ╰── <179> Constant Int [1]
                    │   │       │   │   ╰── <181> Constant Int [0]
                    │   │       │   ╰── <183> Constant Int [1]
                    │   │       ╰── <188> AddressOf
                    │   │           ╰── <187> Var [y]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <190> Constant Int [5]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <207>  [!=]
                    │   │       ├── <204> Subscript
                    │   │       │   ├── <202> Subscript
                    │   │       │   │   ├── <200> Subscript
                    │   │       │   │   │   ├── <198> Subscript
                    │   │       │   │   │   │   ├── <196> Var [array_of_pointers]
                    │   │       │   │   │   │   ╰── <197> Constant Int [2]
                    │   │       │   │   │   ╰── <199> Constant Int [0]
                    │   │       │   │   ╰── <201> Constant Int [2]
                    │   │       │   ╰── <203> Constant Int [0]
                    │   │       ╰── <206> Constant Int [2]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <208> Constant Int [6]
                    ╰── Return
                        ╰── <213> Constant Int [0]
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
            │       │           ├── <15> Constant Int [1]
            │       │           ├── <17> Constant Int [2]
            │       │           ╰── <19> Constant Int [3]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── val
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <37> Subscript
            │       │           ├── <27> Var [arr]
            │       │           ╰── <36> Assign [=]
            │       │               ├── <29> Var [idx]
            │       │               ╰── <35>  [+]
            │       │                   ├── <32> Var [idx]
            │       │                   ╰── <34> Constant Int [2]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <44>  [!=]
            │       │   │       ├── <41> Var [idx]
            │       │   │       ╰── <43> Constant Int [1]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <45> Constant Int [1]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <54>  [!=]
            │       │   │       ├── <51> Var [val]
            │       │   │       ╰── <53> Constant Int [2]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <55> Constant Int [2]
            │       ╰── Return
            │           ╰── <60> Constant Int [0]
            ├── Function [static_index]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── index
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ├── Initializer
            │       │   │   ╰── <73> Constant Int [0]
            │       │   ╰── Static
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── retval
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <80> Var [index]
            │       ├── <91> Assign [=]
            │       │   ├── <84> Var [index]
            │       │   ╰── <90>  [+]
            │       │       ├── <87> Var [index]
            │       │       ╰── <89> Constant Int [1]
            │       ╰── Return
            │           ╰── <94> Var [retval]
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
            │       │           ├── <109> Constant Int [1]
            │       │           ├── <111> Constant Int [2]
            │       │           ╰── <113> Constant Int [3]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── v1
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <124> Subscript
            │       │           ├── <121> Var [arr]
            │       │           ╰── <123> FunctionCall [static_index]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── v2
            │       │   ├── Type
            │       │   │   ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <134> Subscript
            │       │           ├── <131> Var [arr]
            │       │           ╰── <133> FunctionCall [static_index]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <141>  [!=]
            │       │   │       ├── <138> Var [v1]
            │       │   │       ╰── <140> Constant Int [1]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <142> Constant Int [3]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <151>  [!=]
            │       │   │       ├── <148> Var [v2]
            │       │   │       ╰── <150> Constant Int [2]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <152> Constant Int [4]
            │       ╰── Return
            │           ╰── <157> Constant Int [0]
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
            │           ╰── <186> Subscript
            │               ├── <180> Var [arr]
            │               ╰── <185> Subscript
            │                   ├── <182> Var [a]
            │                   ╰── <184> Var [b]
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
            │       │           ├── <201> Constant Int [4]
            │       │           ├── <203> Constant Int [3]
            │       │           ├── <205> Constant Int [2]
            │       │           ╰── <207> Constant Int [1]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── indices
            │       │   ├── Type
            │       │   │   ╰── Array
            │       │   │       ├── 2
            │       │   │       ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── Compound
            │       │           ├── <217> Constant Int [1]
            │       │           ╰── <219> Constant Int [2]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <232>  [!=]
            │       │   │       ├── <229> FunctionCall [subscript_inception]
            │       │   │       │   ├── <225> Var [arr]
            │       │   │       │   ├── <227> Var [indices]
            │       │   │       │   ╰── <228> Constant Int [1]
            │       │   │       ╰── <231> Constant Int [2]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <233> Constant Int [5]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <247>  [!=]
            │       │   │       ├── <244> FunctionCall [subscript_inception]
            │       │   │       │   ├── <240> Var [arr]
            │       │   │       │   ├── <242> Var [indices]
            │       │   │       │   ╰── <243> Constant Int [0]
            │       │   │       ╰── <246> Constant Int [3]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <248> Constant Int [6]
            │       ╰── Return
            │           ╰── <253> Constant Int [0]
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
            │           ╰── <273> Var [arr]
            ├── Function [subscript_function_result]
            │   ╰── Body
            │       ├── <288> Assign [=]
            │       │   ├── <285> Subscript
            │       │   │   ├── <283> FunctionCall [get_array]
            │       │   │   ╰── <284> Constant Int [2]
            │       │   ╰── <287> Constant Int [1]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <296>  [!=]
            │       │   │       ├── <293> Subscript
            │       │   │       │   ├── <291> FunctionCall [get_array]
            │       │   │       │   ╰── <292> Constant Int [2]
            │       │   │       ╰── <295> Constant Int [1]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <297> Constant Int [7]
            │       ╰── Return
            │           ╰── <302> Constant Int [0]
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
            │       │   │   ╰── <332>  [!=]
            │       │   │       ├── <328> Subscript
            │       │   │       │   ├── <323> Var [arr]
            │       │   │       │   ╰── <327> Unary [-]
            │       │   │       │       ╰── <326> Var [idx]
            │       │   │       ╰── <331> Var [expected]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <333> Constant Int [8]
            │       ╰── Return
            │           ╰── <338> Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── check
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <354> FunctionCall [assign_in_index]
                    │           ╰── <353> Unary [-]
                    │               ╰── <352> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <358> Var [check]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <360> Var [check]
                    ├── <370> Assign [=]
                    │   ├── <366> Var [check]
                    │   ╰── <369> FunctionCall [funcall_in_index]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <373> Var [check]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <375> Var [check]
                    ├── <385> Assign [=]
                    │   ├── <381> Var [check]
                    │   ╰── <384> FunctionCall [check_subscript_inception]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <388> Var [check]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <390> Var [check]
                    ├── <400> Assign [=]
                    │   ├── <396> Var [check]
                    │   ╰── <399> FunctionCall [subscript_function_result]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <403> Var [check]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <405> Var [check]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── arr
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 3
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <416> Constant Int [0]
                    │           ├── <418> Constant Int [1]
                    │           ╰── <420> Constant Int [2]
                    ├── <436> Assign [=]
                    │   ├── <425> Var [check]
                    │   ╰── <435> FunctionCall [negate_subscript]
                    │       ├── <432>  [+]
                    │       │   ├── <429> Var [arr]
                    │       │   ╰── <431> Constant Int [2]
                    │       ├── <433> Constant Int [2]
                    │       ╰── <434> Constant Int [0]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <439> Var [check]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <441> Var [check]
                    ╰── Return
                        ╰── <446> Constant Int [0]
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
                    │           ├── <12> Constant Int [1]
                    │           ├── <14> Constant Int [2]
                    │           ╰── <16> Constant Int [3]
                    ╰── Return
                        ╰── <23> Subscript
                            ├── <21> Var [arr]
                            ╰── <22> Constant Int [2]
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
            │       │       ╰── <20> Subscript
            │       │           ├── <18> Var [arr]
            │       │           ╰── <19> Constant Int [5]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── val2
            │       │   ├── Type
            │       │   │   ╰── Unsigned Int
            │       │   ╰── Initializer
            │       │       ╰── <29> Subscript
            │       │           ├── <27> Var [arr]
            │       │           ╰── <28> Constant UInt [5]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── val3
            │       │   ├── Type
            │       │   │   ╰── Unsigned Int
            │       │   ╰── Initializer
            │       │       ╰── <38> Subscript
            │       │           ├── <36> Var [arr]
            │       │           ╰── <37> Constant Long [5]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── val4
            │       │   ├── Type
            │       │   │   ╰── Unsigned Int
            │       │   ╰── Initializer
            │       │       ╰── <47> Subscript
            │       │           ├── <45> Var [arr]
            │       │           ╰── <46> Constant ULong [5]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <55>  [!=]
            │       │   │       ├── <51> Var [val1]
            │       │   │       ╰── <54> Var [expected]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <56> Constant Int [1]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <66>  [!=]
            │       │   │       ├── <62> Var [val2]
            │       │   │       ╰── <65> Var [expected]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <67> Constant Int [2]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <77>  [!=]
            │       │   │       ├── <73> Var [val3]
            │       │   │       ╰── <76> Var [expected]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <78> Constant Int [3]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <88>  [!=]
            │       │   │       ├── <84> Var [val4]
            │       │   │       ╰── <87> Var [expected]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <89> Constant Int [4]
            │       ╰── Return
            │           ╰── <94> Constant Int [0]
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
            │       │   │   ╰── <118>  [!=]
            │       │   │       ├── <114> Subscript
            │       │   │       │   ├── <112> Var [arr]
            │       │   │       │   ╰── <113> Constant Int [3]
            │       │   │       ╰── <117> Var [expected]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <119> Constant Int [5]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <131>  [!=]
            │       │   │       ├── <127> Subscript
            │       │   │       │   ├── <124> Constant Int [3]
            │       │   │       │   ╰── <126> Var [arr]
            │       │   │       ╰── <130> Var [expected]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <132> Constant Int [6]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <148>  [!=]
            │       │   │       ├── <141> AddressOf
            │       │   │       │   ╰── <140> Subscript
            │       │   │       │       ├── <137> Constant Int [3]
            │       │   │       │       ╰── <139> Var [arr]
            │       │   │       ╰── <147> AddressOf
            │       │   │           ╰── <146> Subscript
            │       │   │               ├── <144> Var [arr]
            │       │   │               ╰── <145> Constant Int [3]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <149> Constant Int [7]
            │       ╰── Return
            │           ╰── <154> Constant Int [0]
            ├── VarDeclaration
            │   ├── Name
            │   │   ╰── static_array
            │   ├── Type
            │   │   ╰── Array
            │   │       ├── 3
            │   │       ╰── Double
            │   ├── Initializer
            │   │   ╰── Compound
            │   │       ├── <165> Constant Double [+1e-1]
            │   │       ├── <167> Constant Double [+2e-1]
            │   │       ╰── <169> Constant Double [+3e-1]
            │   ╰── Static
            ├── Function [subscript_static]
            │   ╰── Body
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <184>  [!=]
            │       │   │       ├── <181> Subscript
            │       │   │       │   ├── <179> Var [static_array]
            │       │   │       │   ╰── <180> Constant Int [0]
            │       │   │       ╰── <183> Constant Double [+1e-1]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <185> Constant Int [8]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <196>  [!=]
            │       │   │       ├── <193> Subscript
            │       │   │       │   ├── <191> Var [static_array]
            │       │   │       │   ╰── <192> Constant Int [1]
            │       │   │       ╰── <195> Constant Double [+2e-1]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <197> Constant Int [9]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <208>  [!=]
            │       │   │       ├── <205> Subscript
            │       │   │       │   ├── <203> Var [static_array]
            │       │   │       │   ╰── <204> Constant Int [2]
            │       │   │       ╰── <207> Constant Double [+3e-1]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <209> Constant Int [10]
            │       ╰── Return
            │           ╰── <214> Constant Int [0]
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
            │       ├── <243> Assign [=]
            │       │   ├── <234> Subscript
            │       │   │   ├── <232> Var [arr]
            │       │   │   ╰── <233> Constant Int [10]
            │       │   ╰── <242>  [*]
            │       │       ├── <239> Subscript
            │       │       │   ├── <237> Var [arr]
            │       │       │   ╰── <238> Constant Int [10]
            │       │       ╰── <241> Constant Int [2]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <252>  [!=]
            │       │   │       ├── <248> Subscript
            │       │   │       │   ├── <246> Var [arr]
            │       │   │       │   ╰── <247> Constant Int [10]
            │       │   │       ╰── <251> Var [expected]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <253> Constant Int [11]
            │       ╰── Return
            │           ╰── <258> Constant Int [0]
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
            │       ├── <289> Assign [=]
            │       │   ├── <280> Subscript
            │       │   │   ├── <278> Var [arr]
            │       │   │   ╰── <279> Constant Int [3]
            │       │   ╰── <288>  [+]
            │       │       ├── <285> Subscript
            │       │       │   ├── <283> Var [arr]
            │       │       │   ╰── <284> Constant Int [3]
            │       │       ╰── <287> Constant Int [1]
            │       ╰── Return
            │           ╰── <292> Var [arr]
            ├── Function [check_increment_static_element]
            │   ╰── Body
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── arr1
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <307> FunctionCall [increment_static_element]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <316>  [!=]
            │       │   │       ├── <313> Subscript
            │       │   │       │   ├── <311> Var [arr1]
            │       │   │       │   ╰── <312> Constant Int [3]
            │       │   │       ╰── <315> Constant Int [1]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <317> Constant Int [12]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <337>  [||]
            │       │   │       ├── <331>  [||]
            │       │   │       │   ├── <325> Subscript
            │       │   │       │   │   ├── <323> Var [arr1]
            │       │   │       │   │   ╰── <324> Constant Int [0]
            │       │   │       │   ╰── <330> Subscript
            │       │   │       │       ├── <328> Var [arr1]
            │       │   │       │       ╰── <329> Constant Int [1]
            │       │   │       ╰── <336> Subscript
            │       │   │           ├── <334> Var [arr1]
            │       │   │           ╰── <335> Constant Int [2]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <338> Constant Int [13]
            │       ├── VarDeclaration
            │       │   ├── Name
            │       │   │   ╰── arr2
            │       │   ├── Type
            │       │   │   ╰── Pointer
            │       │   │       ╰── Int
            │       │   ╰── Initializer
            │       │       ╰── <349> FunctionCall [increment_static_element]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <357>  [!=]
            │       │   │       ├── <353> Var [arr1]
            │       │   │       ╰── <356> Var [arr2]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <358> Constant Int [14]
            │       ├── If
            │       │   ├── Condition
            │       │   │   ╰── <369>  [!=]
            │       │   │       ├── <366> Subscript
            │       │   │       │   ├── <364> Var [arr1]
            │       │   │       │   ╰── <365> Constant Int [3]
            │       │   │       ╰── <368> Constant Int [2]
            │       │   ╰── Then
            │       │       ╰── Block
            │       │           ╰── Return
            │       │               ╰── <370> Constant Int [15]
            │       ╰── Return
            │           ╰── <375> Constant Int [0]
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
                    │           ├── <390> Constant Int [0]
                    │           ├── <392> Constant Int [0]
                    │           ├── <394> Constant Int [0]
                    │           ├── <396> Constant Int [0]
                    │           ├── <398> Constant Int [0]
                    │           ╰── <400> Constant UInt [7]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── check
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <411> FunctionCall [integer_types]
                    │           ├── <409> Var [unsigned_arr]
                    │           ╰── <410> Constant UInt [7]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <415> Var [check]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <417> Var [check]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── long_arr
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 4
                    │   │       ╰── Long
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <428> Constant Int [100]
                    │           ├── <430> Constant Int [102]
                    │           ├── <432> Constant Int [104]
                    │           ╰── <434> Constant Int [106]
                    ├── <446> Assign [=]
                    │   ├── <439> Var [check]
                    │   ╰── <445> FunctionCall [reverse_subscript]
                    │       ├── <443> Var [long_arr]
                    │       ╰── <444> Constant Int [106]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <449> Var [check]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <451> Var [check]
                    ├── <461> Assign [=]
                    │   ├── <457> Var [check]
                    │   ╰── <460> FunctionCall [subscript_static]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <464> Var [check]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <466> Var [check]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── int_arr
                    │   ├── Type
                    │   │   ╰── Array
                    │   │       ├── 11
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── Compound
                    │           ├── <477> Constant Int [0]
                    │           ├── <479> Constant Int [0]
                    │           ├── <481> Constant Int [0]
                    │           ├── <483> Constant Int [0]
                    │           ├── <485> Constant Int [0]
                    │           ├── <487> Constant Int [0]
                    │           ├── <489> Constant Int [0]
                    │           ├── <491> Constant Int [0]
                    │           ├── <493> Constant Int [0]
                    │           ├── <495> Constant Int [0]
                    │           ╰── <497> Constant Int [15]
                    ├── <509> Assign [=]
                    │   ├── <502> Var [check]
                    │   ╰── <508> FunctionCall [update_element]
                    │       ├── <506> Var [int_arr]
                    │       ╰── <507> Constant Int [30]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <512> Var [check]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <514> Var [check]
                    ├── <524> Assign [=]
                    │   ├── <520> Var [check]
                    │   ╰── <523> FunctionCall [check_increment_static_element]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <527> Var [check]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <529> Var [check]
                    ╰── Return
                        ╰── <534> Constant Int [0]
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
            │           ╰── <36>  [==]
            │               ├── <31> Subscript
            │               │   ├── <28> Subscript
            │               │   │   ├── <25> Var [nested_arr]
            │               │   │   ╰── <27> Var [i]
            │               │   ╰── <30> Var [j]
            │               ╰── <34> Var [expected]
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
            │       ├── <74> Assign [=]
            │       │   ├── <70> Subscript
            │       │   │   ├── <67> Subscript
            │       │   │   │   ├── <64> Var [nested_arr]
            │       │   │   │   ╰── <66> Var [i]
            │       │   │   ╰── <69> Var [j]
            │       │   ╰── <73> Var [new_val]
            │       ╰── Return
            │           ╰── <76> Constant Int [0]
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
            │           ╰── <117>  [==]
            │               ├── <112> Subscript
            │               │   ├── <109> Subscript
            │               │   │   ├── <104> Var [nested_arr]
            │               │   │   ╰── <108> Unary [-]
            │               │   │       ╰── <107> Var [i]
            │               │   ╰── <111> Var [j]
            │               ╰── <115> Var [expected]
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
            │           ╰── <158>  [==]
            │               ├── <154> AddressOf
            │               │   ╰── <153> Subscript
            │               │       ├── <150> Subscript
            │               │       │   ├── <147> Var [nested_arr]
            │               │       │   ╰── <149> Var [i]
            │               │       ╰── <152> Var [j]
            │               ╰── <157> Var [expected]
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
            │   │       │   │   ├── <175> Constant Int [1]
            │   │       │   │   ╰── <177> Constant Int [2]
            │   │       │   ╰── Compound
            │   │       │       ╰── <180> Constant Int [3]
            │   │       ╰── Compound
            │   │           ├── Compound
            │   │           │   ╰── <184> Constant Int [4]
            │   │           ╰── Compound
            │   │               ╰── <187> Constant Int [5]
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
            │           ╰── <224>  [==]
            │               ├── <220> Subscript
            │               │   ├── <217> Subscript
            │               │   │   ├── <214> Subscript
            │               │   │   │   ├── <211> Var [nested_arr]
            │               │   │   │   ╰── <213> Var [i]
            │               │   │   ╰── <216> Var [j]
            │               │   ╰── <219> Var [k]
            │               ╰── <223> Var [expected]
            ├── Function [get_array]
            │   ╰── Body
            │       ╰── Return
            │           ╰── <243> Var [nested_arr]
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
            │       ├── <278> Assign [=]
            │       │   ├── <274> Subscript
            │       │   │   ├── <271> Subscript
            │       │   │   │   ├── <268> Subscript
            │       │   │   │   │   ├── <265> FunctionCall [get_array]
            │       │   │   │   │   ╰── <267> Var [i]
            │       │   │   │   ╰── <270> Var [j]
            │       │   │   ╰── <273> Var [k]
            │       │   ╰── <277> Var [val]
            │       ╰── Return
            │           ╰── <280> Constant Int [0]
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
            │           ╰── <307> Subscript
            │               ├── <304> Var [nested]
            │               ╰── <306> Var [i]
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
                    │           │   ├── <325> Constant Int [1]
                    │           │   ├── <327> Constant Int [2]
                    │           │   ╰── <329> Constant Int [3]
                    │           ╰── Compound
                    │               ├── <332> Constant Int [4]
                    │               ├── <334> Constant Int [5]
                    │               ╰── <336> Constant Int [6]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <349> Unary [!]
                    │   │       ╰── <348> FunctionCall [read_nested]
                    │   │           ├── <344> Var [nested_arr]
                    │   │           ├── <345> Constant Int [1]
                    │   │           ├── <346> Constant Int [2]
                    │   │           ╰── <347> Constant Int [6]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <350> Constant Int [1]
                    ├── <363> FunctionCall [write_nested]
                    │   ├── <357> Var [nested_arr]
                    │   ├── <358> Constant Int [1]
                    │   ├── <359> Constant Int [2]
                    │   ╰── <362> Unary [-]
                    │       ╰── <361> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <375>  [!=]
                    │   │       ├── <370> Subscript
                    │   │       │   ├── <368> Subscript
                    │   │       │   │   ├── <366> Var [nested_arr]
                    │   │       │   │   ╰── <367> Constant Int [1]
                    │   │       │   ╰── <369> Constant Int [2]
                    │   │       ╰── <374> Unary [-]
                    │   │           ╰── <373> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <376> Constant Int [2]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <392> Unary [!]
                    │   │       ╰── <391> FunctionCall [read_nested_negated]
                    │   │           ├── <387>  [+]
                    │   │           │   ├── <384> Var [nested_arr]
                    │   │           │   ╰── <386> Constant Int [2]
                    │   │           ├── <388> Constant Int [2]
                    │   │           ├── <389> Constant Int [0]
                    │   │           ╰── <390> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <393> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <410>  [+]
                    │           ├── <407> Subscript
                    │           │   ├── <404> Var [nested_arr]
                    │           │   ╰── <405> Constant Int [0]
                    │           ╰── <409> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <422> Unary [!]
                    │   │       ╰── <421> FunctionCall [get_nested_addr]
                    │   │           ├── <416> Var [nested_arr]
                    │   │           ├── <417> Constant Int [0]
                    │   │           ├── <418> Constant Int [1]
                    │   │           ╰── <420> Var [ptr]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <423> Constant Int [4]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <435> Unary [!]
                    │   │       ╰── <434> FunctionCall [read_static_nested]
                    │   │           ├── <430> Constant Int [1]
                    │   │           ├── <431> Constant Int [1]
                    │   │           ├── <432> Constant Int [0]
                    │   │           ╰── <433> Constant Int [5]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <436> Constant Int [5]
                    ├── <446> FunctionCall [write_nested_complex]
                    │   ├── <442> Constant Int [0]
                    │   ├── <443> Constant Int [2]
                    │   ├── <444> Constant Int [3]
                    │   ╰── <445> Constant Int [111]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <458>  [!=]
                    │   │       ├── <455> Subscript
                    │   │       │   ├── <453> Subscript
                    │   │       │   │   ├── <451> Subscript
                    │   │       │   │   │   ├── <449> FunctionCall [get_array]
                    │   │       │   │   │   ╰── <450> Constant Int [0]
                    │   │       │   │   ╰── <452> Constant Int [2]
                    │   │       │   ╰── <454> Constant Int [3]
                    │   │       ╰── <457> Constant Int [111]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <459> Constant Int [6]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── row_1
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <473> FunctionCall [get_subarray]
                    │           ├── <471> Var [nested_arr]
                    │           ╰── <472> Constant Int [1]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <489>  [!=]
                    │   │       ├── <480>  [+]
                    │   │       │   ├── <477> Var [row_1]
                    │   │       │   ╰── <479> Constant Int [1]
                    │   │       ╰── <488> AddressOf
                    │   │           ╰── <487> Subscript
                    │   │               ├── <485> Subscript
                    │   │               │   ├── <483> Var [nested_arr]
                    │   │               │   ╰── <484> Constant Int [1]
                    │   │               ╰── <486> Constant Int [1]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <490> Constant Int [7]
                    ╰── Return
                        ╰── <495> Constant Int [0]
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
            │           ╰── <18> Subscript
            │               ├── <16> Subscript
            │               │   ├── <14> Var [x]
            │               │   ╰── <15> Constant Int [0]
            │               ╰── <17> Constant Int [0]
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <30> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Int
                    │   ╰── Initializer
                    │       ╰── <40> AddressOf
                    │           ╰── <39> Var [a]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <49>  [!=]
                    │   │       ├── <46> Subscript
                    │   │       │   ├── <44> Var [ptr]
                    │   │       │   ╰── <45> Constant Int [0]
                    │   │       ╰── <48> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <50> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── ptr_ptr
                    │   ├── Type
                    │   │   ╰── Pointer
                    │   │       ╰── Pointer
                    │   │           ╰── Int
                    │   ╰── Initializer
                    │       ╰── <64> AddressOf
                    │           ╰── <63> Var [ptr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <75>  [!=]
                    │   │       ├── <72> Subscript
                    │   │       │   ├── <70> Subscript
                    │   │       │   │   ├── <68> Var [ptr_ptr]
                    │   │       │   │   ╰── <69> Constant Int [0]
                    │   │       │   ╰── <71> Constant Int [0]
                    │   │       ╰── <74> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <76> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── dereferenced
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <87> FunctionCall [subscript_pointer_to_pointer]
                    │           ╰── <86> Var [ptr_ptr]
                    ├── If
                    │   ├── Condition
                    │   │   ╰── <94>  [!=]
                    │   │       ├── <91> Var [dereferenced]
                    │   │       ╰── <93> Constant Int [3]
                    │   ╰── Then
                    │       ╰── Block
                    │           ╰── Return
                    │               ╰── <95> Constant Int [3]
                    ╰── Return
                        ╰── <100> Constant Int [0]
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
                    │           ├── <12> Constant Int [1]
                    │           ├── <14> Constant Int [2]
                    │           ╰── <16> Constant Int [3]
                    ╰── Return
                        ╰── <31>  [==]
                            ├── <25> Unary [-]
                            │   ╰── <24> Subscript
                            │       ├── <22> Var [arr]
                            │       ╰── <23> Constant Int [2]
                            ╰── <29> Unary [-]
                                ╰── <28> Constant Int [3]
    "#;
    assert_parse(src, expected);
}
