use super::{assert_error, assert_parse};

#[test]
fn test_invalid_parse_compound_invalid_operator() {
    assert_error(
        r#"
        int main(void) {
            int a = 0;
            a + = 1;
              //^ Expected expression, but found '='
            return a;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_declare_keyword_as_var() {
    assert_error(
        r#"
        int main(void) {
            int return = 4;
              //^^^^^^ Expected identifier, but found 'return'
            return return + 1;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_extra_credit_binary_decrement() {
    assert_error(
        r#"
        int main(void) {
            int a = 0;
            return a -- 1;
                      //^ Expected ';', but found '1'
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_extra_credit_binary_increment() {
    assert_error(
        r#"
        int main(void) {
            int a = 0;
            return a ++ 1;
                      //^ Expected ';', but found '1'
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_extra_credit_compound_initializer() {
    assert_error(
        r#"
        int main(void) {
            int a += 0;
                //^^ Expected ';', but found '+='
            return a;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_extra_credit_increment_declaration() {
    assert_error(
        r#"
        int main(void) {
            int a++;
               //^^ Expected ';', but found '++'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_invalid_specifier() {
    assert_error(
        r#"
        int main(void) {
            int foo bar = 3;
                  //^^^ Expected ';', but found 'bar'
            return bar;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_invalid_type() {
    assert_error(
        r#"
        int main(void) {
            ints a = 1;
               //^ Expected ';', but found 'a'
            return a;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_invalid_variable_name() {
    assert_error(
        r#"
        int main(void)
        {
            int 10 = 0;
              //^^ Expected identifier, but found '10'
            return 10;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_malformed_compound_assignment() {
    assert_error(
        r#"
        int main(void) {
            int a = 10;
            a =/ 1;
             //^ Expected expression, but found '/'
            return a;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_malformed_decrement() {
    assert_error(
        r#"
        int main(void) {
            int a = 0;
            a - -;
               //^ Expected expression, but found ';'
            return a;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_malformed_increment() {
    assert_error(
        r#"
        int main(void) {
            int a = 0;
            a + +;
              //^ Expected expression, but found '+'
            return a;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_malformed_less_equal() {
    assert_error(
        r#"
        int main(void)
        {
            return 1 < = 2;
                     //^ Expected expression, but found '='
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_malformed_not_equal() {
    assert_error(
        r#"
        int main(void)
        {
            return 1 ! = 0;
                   //^ Expected ';', but found '!'
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_missing_semicolon() {
    assert_error(
        r#"
        int main(void) {
            int a = 2
            a = a + 4;
          //^ Expected ';', but found 'a'
            return a;
        }
    "#,
    );
}

#[test]
fn test_invalid_parse_return_in_assignment() {
    assert_error(
        r#"
        int main(void)
        {
            int 10 = return 0;
              //^^ Expected identifier, but found '10'
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_declared_after_use() {
    let src = r#"
        int main(void) {
            a = 1 + 2;
            int a;
            return a;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── <12> Assign [=]
                    │   ├── <6> Var [a]
                    │   ╰── <11>  [+]
                    │       ├── <8> Constant Int [1]
                    │       ╰── <10> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ╰── Type
                    │       ╰── Int
                    ╰── Return
                        ╰── <19> Var [a]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_extra_credit_compound_invalid_lvalue() {
    let src = r#"
        int main(void) {
            int a = 0;
            -a += 1;
            return a;
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
                    │       ╰── <8> Constant Int [0]
                    ├── <17> Assign [+=]
                    │   ├── <14> Unary [-]
                    │   │   ╰── <13> Var [a]
                    │   ╰── <16> Constant Int [1]
                    ╰── Return
                        ╰── <20> Var [a]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_extra_credit_compound_invalid_lvalue_2() {
    let src = r#"
        int main(void) {
            int a = 10;
            (a += 1) -= 2;
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
                    │       ╰── <8> Constant Int [10]
                    ╰── <19> Assign [-=]
                        ├── <16> Assign [+=]
                        │   ├── <12> Var [a]
                        │   ╰── <14> Constant Int [1]
                        ╰── <18> Constant Int [2]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_extra_credit_postfix_decr_non_lvalue() {
    let src = r#"
        int main(void) {
            int a = 10;
            return a++--;
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
                    │       ╰── <8> Constant Int [10]
                    ╰── Return
                        ╰── <16> Postfix [--]
                            ╰── <14> Postfix [++]
                                ╰── <12> Var [a]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_extra_credit_postfix_incr_non_lvalue() {
    let src = r#"
        int main(void) {
            int a = 0;
            (a = 4)++;
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
                    │       ╰── <8> Constant Int [0]
                    ╰── <18> Postfix [++]
                        ╰── <16> Assign [=]
                            ├── <12> Var [a]
                            ╰── <14> Constant Int [4]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_extra_credit_prefix_decr_non_lvalue() {
    let src = r#"
        int main(void) {
            return --3;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <7> Unary [--]
                            ╰── <6> Constant Int [3]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_extra_credit_prefix_incr_non_lvalue() {
    let src = r#"
        int main(void) {
            int a = 1;
            ++(a+1);
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
                    │       ╰── <8> Constant Int [1]
                    ├── <18> Unary [++]
                    │   ╰── <17>  [+]
                    │       ├── <13> Var [a]
                    │       ╰── <15> Constant Int [1]
                    ╰── Return
                        ╰── <20> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_extra_credit_undeclared_bitwise_op() {
    let src = r#"
        int main(void){
            return a >> 2;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <9>  [>>]
                            ├── <6> Var [a]
                            ╰── <8> Constant Int [2]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_extra_credit_undeclared_compound_assignment() {
    let src = r#"
        int main(void) {
            a += 1;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── <9> Assign [+=]
                    │   ├── <6> Var [a]
                    │   ╰── <8> Constant Int [1]
                    ╰── Return
                        ╰── <11> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_extra_credit_undeclared_compound_assignment_use() {
    let src = r#"
        int main(void) {
            int b = 10;
            b *= a;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <8> Constant Int [10]
                    ├── <16> Assign [*=]
                    │   ├── <12> Var [b]
                    │   ╰── <15> Var [a]
                    ╰── Return
                        ╰── <18> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_extra_credit_undeclared_postfix_decr() {
    let src = r#"
        int main(void) {
            a--;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── <8> Postfix [--]
                    │   ╰── <6> Var [a]
                    ╰── Return
                        ╰── <10> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_extra_credit_undeclared_prefix_incr() {
    let src = r#"
        int main(void) {
            a++;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── <8> Postfix [++]
                    │   ╰── <6> Var [a]
                    ╰── Return
                        ╰── <10> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_invalid_lvalue() {
    let src = r#"
        int main(void) {
            int a = 2;
            a + 3 = 4;
            return a;
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
                    │       ╰── <8> Constant Int [2]
                    ├── <18> Assign [=]
                    │   ├── <15>  [+]
                    │   │   ├── <12> Var [a]
                    │   │   ╰── <14> Constant Int [3]
                    │   ╰── <17> Constant Int [4]
                    ╰── Return
                        ╰── <21> Var [a]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_invalid_lvalue_2() {
    let src = r#"
        int main(void) {
            int a = 2;
            !a = 3;
            return a;
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
                    │       ╰── <8> Constant Int [2]
                    ├── <17> Assign [=]
                    │   ├── <14> Unary [!]
                    │   │   ╰── <13> Var [a]
                    │   ╰── <16> Constant Int [3]
                    ╰── Return
                        ╰── <20> Var [a]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_mixed_precedence_assignment() {
    let src = r#"
        int main(void) {
            int a = 1;
            int b = 2;
            a = 3 * b = a;
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
                    │       ╰── <8> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <14> Constant Int [2]
                    ╰── <29> Assign [=]
                        ├── <18> Var [a]
                        ╰── <28> Assign [=]
                            ├── <24>  [*]
                            │   ├── <20> Constant Int [3]
                            │   ╰── <23> Var [b]
                            ╰── <27> Var [a]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_redefine() {
    let src = r#"
        int main(void) {
            int a = 1;
            int a = 2;
            return a;
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
                    │       ╰── <8> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <14> Constant Int [2]
                    ╰── Return
                        ╰── <18> Var [a]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_undeclared_var() {
    let src = r#"
        int main(void) {
            return a;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <6> Var [a]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_undeclared_var_and() {
    let src = r#"
        int main(void) {
            return 0 && a;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <9>  [&&]
                            ├── <5> Constant Int [0]
                            ╰── <8> Var [a]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_undeclared_var_compare() {
    let src = r#"
        int main(void) {
            return a < 5;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <9>  [<]
                            ├── <6> Var [a]
                            ╰── <8> Constant Int [5]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_undeclared_var_unary() {
    let src = r#"
        int main(void) {
            return -a;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Return
                        ╰── <8> Unary [-]
                            ╰── <7> Var [a]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_invalid_semantics_use_then_redefine() {
    let src = r#"
        int main(void) {
            int a = 0;
            return a;
            int a = 1;
            return a;
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
                    │       ╰── <8> Constant Int [0]
                    ├── Return
                    │   ╰── <12> Var [a]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <17> Constant Int [1]
                    ╰── Return
                        ╰── <21> Var [a]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_add_variables() {
    let src = r#"
        int main(void) {
            int first_variable = 1;
            int second_variable = 2;
            return first_variable + second_variable;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── first_variable
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <8> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── second_variable
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <14> Constant Int [2]
                    ╰── Return
                        ╰── <22>  [+]
                            ├── <18> Var [first_variable]
                            ╰── <21> Var [second_variable]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_allocate_temps_and_vars() {
    let src = r#"
        int main(void) {
            int a = 2147483646;
            int b = 0;
            int c = a / 6 + !b;
            return c * 2 == a - 1431655762;
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
                    │       ╰── <8> Constant Int [2147483646]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <14> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <30>  [+]
                    │           ├── <24>  [/]
                    │           │   ├── <21> Var [a]
                    │           │   ╰── <23> Constant Int [6]
                    │           ╰── <29> Unary [!]
                    │               ╰── <28> Var [b]
                    ╰── Return
                        ╰── <44>  [==]
                            ├── <37>  [*]
                            │   ├── <34> Var [c]
                            │   ╰── <36> Constant Int [2]
                            ╰── <43>  [-]
                                ├── <40> Var [a]
                                ╰── <42> Constant Int [1431655762]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_assign() {
    let src = r#"
        int main(void) {
            int var0;
            var0 = 2;
            return var0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── var0
                    │   ╰── Type
                    │       ╰── Int
                    ├── <13> Assign [=]
                    │   ├── <10> Var [var0]
                    │   ╰── <12> Constant Int [2]
                    ╰── Return
                        ╰── <16> Var [var0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_assign_val_in_initializer() {
    let src = r#"
        int main(void) {
            int a = a = 5;
            return a;
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
                    │       ╰── <12> Assign [=]
                    │           ├── <9> Var [a]
                    │           ╰── <11> Constant Int [5]
                    ╰── Return
                        ╰── <16> Var [a]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_assignment_in_initializer() {
    let src = r#"
        int main(void) {
            int a;
            int b = a = 0;
            return b;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ╰── Type
                    │       ╰── Int
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <16> Assign [=]
                    │           ├── <13> Var [a]
                    │           ╰── <15> Constant Int [0]
                    ╰── Return
                        ╰── <20> Var [b]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_assignment_lowest_precedence() {
    let src = r#"
        int main(void) {
            int a;
            a = 0 || 5;
            return a;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ╰── Type
                    │       ╰── Int
                    ├── <16> Assign [=]
                    │   ├── <10> Var [a]
                    │   ╰── <15>  [||]
                    │       ├── <12> Constant Int [0]
                    │       ╰── <14> Constant Int [5]
                    ╰── Return
                        ╰── <19> Var [a]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_empty_function_body() {
    let src = r#"
        int main(void) {
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_exp_then_declaration() {
    let src = r#"
        int main(void) {
            int a = -2593;
            a = a % 3;
            int b = -a;
            return b;
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
                    │       ╰── <10> Unary [-]
                    │           ╰── <9> Constant Int [2593]
                    ├── <21> Assign [=]
                    │   ├── <14> Var [a]
                    │   ╰── <20>  [%]
                    │       ├── <17> Var [a]
                    │       ╰── <19> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <29> Unary [-]
                    │           ╰── <28> Var [a]
                    ╰── Return
                        ╰── <33> Var [b]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_bitwise_in_initializer() {
    let src = r#"
        int main(void) {
            int a = 15;
            int b = a ^ 5;
            return 1 | b;
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
                    │       ╰── <8> Constant Int [15]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <18>  [^]
                    │           ├── <15> Var [a]
                    │           ╰── <17> Constant Int [5]
                    ╰── Return
                        ╰── <25>  [|]
                            ├── <21> Constant Int [1]
                            ╰── <24> Var [b]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_bitwise_ops_vars() {
    let src = r#"
        int main(void) {
            int a = 3;
            int b = 5;
            int c = 8;
            return a & b | c;
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
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <14> Constant Int [5]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <20> Constant Int [8]
                    ╰── Return
                        ╰── <32>  [|]
                            ├── <28>  [&]
                            │   ├── <24> Var [a]
                            │   ╰── <27> Var [b]
                            ╰── <31> Var [c]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_bitwise_shiftl_variable() {
    let src = r#"
        int main(void) {
            int x = 3;
            return x << 3;
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
                    │       ╰── <8> Constant Int [3]
                    ╰── Return
                        ╰── <15>  [<<]
                            ├── <12> Var [x]
                            ╰── <14> Constant Int [3]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_bitwise_shiftr_assign() {
    let src = r#"
        int main(void) {
            int var_to_shift = 1234;
            int x = 0;
            x = var_to_shift >> 4;
            return x;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── var_to_shift
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <8> Constant Int [1234]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <14> Constant Int [0]
                    ├── <25> Assign [=]
                    │   ├── <18> Var [x]
                    │   ╰── <24>  [>>]
                    │       ├── <21> Var [var_to_shift]
                    │       ╰── <23> Constant Int [4]
                    ╰── Return
                        ╰── <28> Var [x]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_compound_assignment_chained() {
    let src = r#"
        int main(void) {
            int a = 250;
            int b = 200;
            int c = 100;
            int d = 75;
            int e = -25;
            int f = 0;
            int x = 0;
            x = a += b -= c *= d /= e %= f = -7;
            return a == 2250 && b == 2000 && c == -1800 && d == -18 && e == -4 &&
                   f == -7 && x == 2250;
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
                    │       ╰── <8> Constant Int [250]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <14> Constant Int [200]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <20> Constant Int [100]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <26> Constant Int [75]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── e
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <34> Unary [-]
                    │           ╰── <33> Constant Int [25]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── f
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <40> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <46> Constant Int [0]
                    ├── <79> Assign [=]
                    │   ├── <50> Var [x]
                    │   ╰── <78> Assign [+=]
                    │       ├── <53> Var [a]
                    │       ╰── <77> Assign [-=]
                    │           ├── <56> Var [b]
                    │           ╰── <76> Assign [*=]
                    │               ├── <59> Var [c]
                    │               ╰── <75> Assign [/=]
                    │                   ├── <62> Var [d]
                    │                   ╰── <74> Assign [&=]
                    │                       ├── <65> Var [e]
                    │                       ╰── <73> Assign [=]
                    │                           ├── <68> Var [f]
                    │                           ╰── <72> Unary [-]
                    │                               ╰── <71> Constant Int [7]
                    ╰── Return
                        ╰── <135>  [&&]
                            ├── <128>  [&&]
                            │   ├── <119>  [&&]
                            │   │   ├── <110>  [&&]
                            │   │   │   ├── <101>  [&&]
                            │   │   │   │   ├── <92>  [&&]
                            │   │   │   │   │   ├── <85>  [==]
                            │   │   │   │   │   │   ├── <82> Var [a]
                            │   │   │   │   │   │   ╰── <84> Constant Int [2250]
                            │   │   │   │   │   ╰── <91>  [==]
                            │   │   │   │   │       ├── <88> Var [b]
                            │   │   │   │   │       ╰── <90> Constant Int [2000]
                            │   │   │   │   ╰── <100>  [==]
                            │   │   │   │       ├── <95> Var [c]
                            │   │   │   │       ╰── <99> Unary [-]
                            │   │   │   │           ╰── <98> Constant Int [1800]
                            │   │   │   ╰── <109>  [==]
                            │   │   │       ├── <104> Var [d]
                            │   │   │       ╰── <108> Unary [-]
                            │   │   │           ╰── <107> Constant Int [18]
                            │   │   ╰── <118>  [==]
                            │   │       ├── <113> Var [e]
                            │   │       ╰── <117> Unary [-]
                            │   │           ╰── <116> Constant Int [4]
                            │   ╰── <127>  [==]
                            │       ├── <122> Var [f]
                            │       ╰── <126> Unary [-]
                            │           ╰── <125> Constant Int [7]
                            ╰── <134>  [==]
                                ├── <131> Var [x]
                                ╰── <133> Constant Int [2250]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_compound_assignment_lowest_precedence() {
    let src = r#"
        int main(void) {
            int a = 10;
            int b = 12;
            a += 0 || b;
            b *= a && 0;
            int c = 14;
            c -= a || b;
            int d = 16;
            d /= c || d;
            return (a == 11 && b == 0 && c == 13 && d == 16);
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
                    │       ╰── <8> Constant Int [10]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <14> Constant Int [12]
                    ├── <25> Assign [+=]
                    │   ├── <18> Var [a]
                    │   ╰── <24>  [||]
                    │       ├── <20> Constant Int [0]
                    │       ╰── <23> Var [b]
                    ├── <35> Assign [*=]
                    │   ├── <28> Var [b]
                    │   ╰── <34>  [&&]
                    │       ├── <31> Var [a]
                    │       ╰── <33> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <40> Constant Int [14]
                    ├── <52> Assign [-=]
                    │   ├── <44> Var [c]
                    │   ╰── <51>  [||]
                    │       ├── <47> Var [a]
                    │       ╰── <50> Var [b]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <57> Constant Int [16]
                    ├── <69> Assign [/=]
                    │   ├── <61> Var [d]
                    │   ╰── <68>  [||]
                    │       ├── <64> Var [c]
                    │       ╰── <67> Var [d]
                    ╰── Return
                        ╰── <97>  [&&]
                            ├── <89>  [&&]
                            │   ├── <82>  [&&]
                            │   │   ├── <75>  [==]
                            │   │   │   ├── <72> Var [a]
                            │   │   │   ╰── <74> Constant Int [11]
                            │   │   ╰── <81>  [==]
                            │   │       ├── <78> Var [b]
                            │   │       ╰── <80> Constant Int [0]
                            │   ╰── <88>  [==]
                            │       ├── <85> Var [c]
                            │       ╰── <87> Constant Int [13]
                            ╰── <95>  [==]
                                ├── <92> Var [d]
                                ╰── <94> Constant Int [16]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_compound_assignment_use_result() {
    let src = r#"
        int main(void) {
            int x = 1;
            int y = x += 3;
            return (x == 4 && y == 4);
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
                    │       ╰── <8> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── y
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <18> Assign [+=]
                    │           ├── <15> Var [x]
                    │           ╰── <17> Constant Int [3]
                    ╰── Return
                        ╰── <33>  [&&]
                            ├── <25>  [==]
                            │   ├── <22> Var [x]
                            │   ╰── <24> Constant Int [4]
                            ╰── <31>  [==]
                                ├── <28> Var [y]
                                ╰── <30> Constant Int [4]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_compound_bitwise_and() {
    let src = r#"
        int main(void) {
            int to_and = 3;
            to_and &= 6;
            return to_and;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── to_and
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <8> Constant Int [3]
                    ├── <15> Assign [&=]
                    │   ├── <12> Var [to_and]
                    │   ╰── <14> Constant Int [6]
                    ╰── Return
                        ╰── <18> Var [to_and]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_compound_bitwise_assignment_lowest_precedence() {
    let src = r#"
        int main(void) {
            int a = 11;
            int b = 12;
            a &= 0 || b;
            b ^= a || 1;
            int c = 14;
            c |= a || b;
            int d = 16;
            d >>= c || d;
            int e = 18;
            e <<= c || d;
            return (a == 1 && b == 13 && c == 15 && d == 8 && e == 36);
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
                    │       ╰── <8> Constant Int [11]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <14> Constant Int [12]
                    ├── <25> Assign [&=]
                    │   ├── <18> Var [a]
                    │   ╰── <24>  [||]
                    │       ├── <20> Constant Int [0]
                    │       ╰── <23> Var [b]
                    ├── <35> Assign [^=]
                    │   ├── <28> Var [b]
                    │   ╰── <34>  [||]
                    │       ├── <31> Var [a]
                    │       ╰── <33> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <40> Constant Int [14]
                    ├── <52> Assign [|=]
                    │   ├── <44> Var [c]
                    │   ╰── <51>  [||]
                    │       ├── <47> Var [a]
                    │       ╰── <50> Var [b]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <57> Constant Int [16]
                    ├── <69> Assign [>>=]
                    │   ├── <61> Var [d]
                    │   ╰── <68>  [||]
                    │       ├── <64> Var [c]
                    │       ╰── <67> Var [d]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── e
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <74> Constant Int [18]
                    ├── <86> Assign [<<=]
                    │   ├── <78> Var [e]
                    │   ╰── <85>  [||]
                    │       ├── <81> Var [c]
                    │       ╰── <84> Var [d]
                    ╰── Return
                        ╰── <121>  [&&]
                            ├── <113>  [&&]
                            │   ├── <106>  [&&]
                            │   │   ├── <99>  [&&]
                            │   │   │   ├── <92>  [==]
                            │   │   │   │   ├── <89> Var [a]
                            │   │   │   │   ╰── <91> Constant Int [1]
                            │   │   │   ╰── <98>  [==]
                            │   │   │       ├── <95> Var [b]
                            │   │   │       ╰── <97> Constant Int [13]
                            │   │   ╰── <105>  [==]
                            │   │       ├── <102> Var [c]
                            │   │       ╰── <104> Constant Int [15]
                            │   ╰── <112>  [==]
                            │       ├── <109> Var [d]
                            │       ╰── <111> Constant Int [8]
                            ╰── <119>  [==]
                                ├── <116> Var [e]
                                ╰── <118> Constant Int [36]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_compound_bitwise_chained() {
    let src = r#"
        int main(void) {
            int a = 250;
            int b = 200;
            int c = 100;
            int d = 75;
            int e = 50;
            int f = 25;
            int g = 10;
            int h = 1;
            int j = 0;
            int x = 0;
            x = a &= b *= c |= d = e ^= f += g >>= h <<= j = 1;
            return (a == 40 && b == 21800 && c == 109 && d == 41 && e == 41 &&
                    f == 27 && g == 2 && h == 2 && j == 1 && x == 40);
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
                    │       ╰── <8> Constant Int [250]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <14> Constant Int [200]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <20> Constant Int [100]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <26> Constant Int [75]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── e
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <32> Constant Int [50]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── f
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <38> Constant Int [25]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── g
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <44> Constant Int [10]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── h
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <50> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── j
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <56> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <62> Constant Int [0]
                    ├── <105> Assign [=]
                    │   ├── <66> Var [x]
                    │   ╰── <104> Assign [&=]
                    │       ├── <69> Var [a]
                    │       ╰── <103> Assign [*=]
                    │           ├── <72> Var [b]
                    │           ╰── <102> Assign [|=]
                    │               ├── <75> Var [c]
                    │               ╰── <101> Assign [=]
                    │                   ├── <78> Var [d]
                    │                   ╰── <100> Assign [^=]
                    │                       ├── <81> Var [e]
                    │                       ╰── <99> Assign [+=]
                    │                           ├── <84> Var [f]
                    │                           ╰── <98> Assign [>>=]
                    │                               ├── <87> Var [g]
                    │                               ╰── <97> Assign [<<=]
                    │                                   ├── <90> Var [h]
                    │                                   ╰── <96> Assign [=]
                    │                                       ├── <93> Var [j]
                    │                                       ╰── <95> Constant Int [1]
                    ╰── Return
                        ╰── <175>  [&&]
                            ├── <167>  [&&]
                            │   ├── <160>  [&&]
                            │   │   ├── <153>  [&&]
                            │   │   │   ├── <146>  [&&]
                            │   │   │   │   ├── <139>  [&&]
                            │   │   │   │   │   ├── <132>  [&&]
                            │   │   │   │   │   │   ├── <125>  [&&]
                            │   │   │   │   │   │   │   ├── <118>  [&&]
                            │   │   │   │   │   │   │   │   ├── <111>  [==]
                            │   │   │   │   │   │   │   │   │   ├── <108> Var [a]
                            │   │   │   │   │   │   │   │   │   ╰── <110> Constant Int [40]
                            │   │   │   │   │   │   │   │   ╰── <117>  [==]
                            │   │   │   │   │   │   │   │       ├── <114> Var [b]
                            │   │   │   │   │   │   │   │       ╰── <116> Constant Int [21800]
                            │   │   │   │   │   │   │   ╰── <124>  [==]
                            │   │   │   │   │   │   │       ├── <121> Var [c]
                            │   │   │   │   │   │   │       ╰── <123> Constant Int [109]
                            │   │   │   │   │   │   ╰── <131>  [==]
                            │   │   │   │   │   │       ├── <128> Var [d]
                            │   │   │   │   │   │       ╰── <130> Constant Int [41]
                            │   │   │   │   │   ╰── <138>  [==]
                            │   │   │   │   │       ├── <135> Var [e]
                            │   │   │   │   │       ╰── <137> Constant Int [41]
                            │   │   │   │   ╰── <145>  [==]
                            │   │   │   │       ├── <142> Var [f]
                            │   │   │   │       ╰── <144> Constant Int [27]
                            │   │   │   ╰── <152>  [==]
                            │   │   │       ├── <149> Var [g]
                            │   │   │       ╰── <151> Constant Int [2]
                            │   │   ╰── <159>  [==]
                            │   │       ├── <156> Var [h]
                            │   │       ╰── <158> Constant Int [2]
                            │   ╰── <166>  [==]
                            │       ├── <163> Var [j]
                            │       ╰── <165> Constant Int [1]
                            ╰── <173>  [==]
                                ├── <170> Var [x]
                                ╰── <172> Constant Int [40]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_compound_bitwise_or() {
    let src = r#"
        int main(void) {
            int to_or = 1;
            to_or |= 30;
            return to_or;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── to_or
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <8> Constant Int [1]
                    ├── <15> Assign [|=]
                    │   ├── <12> Var [to_or]
                    │   ╰── <14> Constant Int [30]
                    ╰── Return
                        ╰── <18> Var [to_or]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_compound_bitwise_shiftl() {
    let src = r#"
        int main(void) {
            int to_shiftl = 3;
            to_shiftl <<= 4;
            return to_shiftl;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── to_shiftl
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <8> Constant Int [3]
                    ├── <15> Assign [<<=]
                    │   ├── <12> Var [to_shiftl]
                    │   ╰── <14> Constant Int [4]
                    ╰── Return
                        ╰── <18> Var [to_shiftl]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_compound_bitwise_shiftr() {
    let src = r#"
        int main(void) {
            int to_shiftr = 382574;
            to_shiftr >>= 4;
            return to_shiftr;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── to_shiftr
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <8> Constant Int [382574]
                    ├── <15> Assign [>>=]
                    │   ├── <12> Var [to_shiftr]
                    │   ╰── <14> Constant Int [4]
                    ╰── Return
                        ╰── <18> Var [to_shiftr]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_compound_bitwise_xor() {
    let src = r#"
        int main(void) {
            int to_xor = 7;
            to_xor ^= 5;
            return to_xor;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── to_xor
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <8> Constant Int [7]
                    ├── <15> Assign [^=]
                    │   ├── <12> Var [to_xor]
                    │   ╰── <14> Constant Int [5]
                    ╰── Return
                        ╰── <18> Var [to_xor]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_compound_divide() {
    let src = r#"
        int main(void) {
            int to_divide = 8;
            to_divide /= 4;
            return to_divide;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── to_divide
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <8> Constant Int [8]
                    ├── <15> Assign [/=]
                    │   ├── <12> Var [to_divide]
                    │   ╰── <14> Constant Int [4]
                    ╰── Return
                        ╰── <18> Var [to_divide]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_compound_minus() {
    let src = r#"
        int main(void) {
            int to_subtract = 10;
            to_subtract -= 8;
            return to_subtract;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── to_subtract
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <8> Constant Int [10]
                    ├── <15> Assign [-=]
                    │   ├── <12> Var [to_subtract]
                    │   ╰── <14> Constant Int [8]
                    ╰── Return
                        ╰── <18> Var [to_subtract]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_compound_mod() {
    let src = r#"
        int main(void) {
            int to_mod = 5;
            to_mod %= 3;
            return to_mod;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── to_mod
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <8> Constant Int [5]
                    ├── <15> Assign [&=]
                    │   ├── <12> Var [to_mod]
                    │   ╰── <14> Constant Int [3]
                    ╰── Return
                        ╰── <18> Var [to_mod]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_compound_multiply() {
    let src = r#"
        int main(void) {
            int to_multiply = 4;
            to_multiply *= 3;
            return to_multiply;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── to_multiply
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <8> Constant Int [4]
                    ├── <15> Assign [*=]
                    │   ├── <12> Var [to_multiply]
                    │   ╰── <14> Constant Int [3]
                    ╰── Return
                        ╰── <18> Var [to_multiply]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_compound_plus() {
    let src = r#"
        int main(void) {
            int to_add = 0;
            to_add += 4;
            return to_add;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── to_add
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <8> Constant Int [0]
                    ├── <15> Assign [+=]
                    │   ├── <12> Var [to_add]
                    │   ╰── <14> Constant Int [4]
                    ╰── Return
                        ╰── <18> Var [to_add]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_incr_expression_statement() {
    let src = r#"
        int main(void) {
            int a = 0;
            int b = 0;
            a++;
            ++a;
            ++a;
            b--;
            --b;
            return (a == 3 && b == -2);
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
                    │       ╰── <8> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <14> Constant Int [0]
                    ├── <20> Postfix [++]
                    │   ╰── <18> Var [a]
                    ├── <25> Unary [++]
                    │   ╰── <24> Var [a]
                    ├── <30> Unary [++]
                    │   ╰── <29> Var [a]
                    ├── <35> Postfix [--]
                    │   ╰── <33> Var [b]
                    ├── <40> Unary [--]
                    │   ╰── <39> Var [b]
                    ╰── Return
                        ╰── <56>  [&&]
                            ├── <46>  [==]
                            │   ├── <43> Var [a]
                            │   ╰── <45> Constant Int [3]
                            ╰── <54>  [==]
                                ├── <49> Var [b]
                                ╰── <53> Unary [-]
                                    ╰── <52> Constant Int [2]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_incr_in_binary_expr() {
    let src = r#"
        int main(void) {
            int a = 2;
            int b = 3 + a++;
            int c = 4 + ++b;
            return (a == 3 && b == 6 && c == 10);
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
                    │       ╰── <8> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <20>  [+]
                    │           ├── <14> Constant Int [3]
                    │           ╰── <19> Postfix [++]
                    │               ╰── <17> Var [a]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <32>  [+]
                    │           ├── <26> Constant Int [4]
                    │           ╰── <31> Unary [++]
                    │               ╰── <30> Var [b]
                    ╰── Return
                        ╰── <54>  [&&]
                            ├── <46>  [&&]
                            │   ├── <39>  [==]
                            │   │   ├── <36> Var [a]
                            │   │   ╰── <38> Constant Int [3]
                            │   ╰── <45>  [==]
                            │       ├── <42> Var [b]
                            │       ╰── <44> Constant Int [6]
                            ╰── <52>  [==]
                                ├── <49> Var [c]
                                ╰── <51> Constant Int [10]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_incr_parenthesized() {
    let src = r#"
        
        int main(void) {
            int a = 1;
            int b = 2;
            int c = -++(a);
            int d = !(b)--;
            return (a == 2 && b == 1 && c == -2 && d == 0);
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
                    │       ╰── <8> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <14> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <26> Unary [-]
                    │           ╰── <25> Unary [++]
                    │               ╰── <24> Var [a]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <38> Unary [!]
                    │           ╰── <37> Postfix [--]
                    │               ╰── <35> Var [b]
                    ╰── Return
                        ╰── <69>  [&&]
                            ├── <61>  [&&]
                            │   ├── <52>  [&&]
                            │   │   ├── <45>  [==]
                            │   │   │   ├── <42> Var [a]
                            │   │   │   ╰── <44> Constant Int [2]
                            │   │   ╰── <51>  [==]
                            │   │       ├── <48> Var [b]
                            │   │       ╰── <50> Constant Int [1]
                            │   ╰── <60>  [==]
                            │       ├── <55> Var [c]
                            │       ╰── <59> Unary [-]
                            │           ╰── <58> Constant Int [2]
                            ╰── <67>  [==]
                                ├── <64> Var [d]
                                ╰── <66> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_postfix_incr_and_decr() {
    let src = r#"
        int main(void) {
            int a = 1;
            int b = 2;
            int c = a++;
            int d = b--;
            return (a == 2 && b == 1 && c == 1 && d == 2);
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
                    │       ╰── <8> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <14> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <23> Postfix [++]
                    │           ╰── <21> Var [a]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <32> Postfix [--]
                    │           ╰── <30> Var [b]
                    ╰── Return
                        ╰── <61>  [&&]
                            ├── <53>  [&&]
                            │   ├── <46>  [&&]
                            │   │   ├── <39>  [==]
                            │   │   │   ├── <36> Var [a]
                            │   │   │   ╰── <38> Constant Int [2]
                            │   │   ╰── <45>  [==]
                            │   │       ├── <42> Var [b]
                            │   │       ╰── <44> Constant Int [1]
                            │   ╰── <52>  [==]
                            │       ├── <49> Var [c]
                            │       ╰── <51> Constant Int [1]
                            ╰── <59>  [==]
                                ├── <56> Var [d]
                                ╰── <58> Constant Int [2]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_postfix_precedence() {
    let src = r#"
        int main(void) {
            int a = 1;
            int b = !a++;
            return (a == 2 && b == 0);
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
                    │       ╰── <8> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <19> Unary [!]
                    │           ╰── <18> Postfix [++]
                    │               ╰── <16> Var [a]
                    ╰── Return
                        ╰── <34>  [&&]
                            ├── <26>  [==]
                            │   ├── <23> Var [a]
                            │   ╰── <25> Constant Int [2]
                            ╰── <32>  [==]
                                ├── <29> Var [b]
                                ╰── <31> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_extra_credit_prefix_incr_and_decr() {
    let src = r#"
        int main(void) {
            int a = 1;
            int b = 2;
            int c = ++a;
            int d = --b;
            return (a == 2 && b == 1 && c == 2 && d == 1);
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
                    │       ╰── <8> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <14> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <23> Unary [++]
                    │           ╰── <22> Var [a]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <32> Unary [--]
                    │           ╰── <31> Var [b]
                    ╰── Return
                        ╰── <61>  [&&]
                            ├── <53>  [&&]
                            │   ├── <46>  [&&]
                            │   │   ├── <39>  [==]
                            │   │   │   ├── <36> Var [a]
                            │   │   │   ╰── <38> Constant Int [2]
                            │   │   ╰── <45>  [==]
                            │   │       ├── <42> Var [b]
                            │   │       ╰── <44> Constant Int [1]
                            │   ╰── <52>  [==]
                            │       ├── <49> Var [c]
                            │       ╰── <51> Constant Int [2]
                            ╰── <59>  [==]
                                ├── <56> Var [d]
                                ╰── <58> Constant Int [1]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_kw_var_names() {
    let src = r#"
        int main(void) {
            int return_val = 3;
            int void2 = 2;
            return return_val + void2;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── return_val
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <8> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── void2
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <14> Constant Int [2]
                    ╰── Return
                        ╰── <22>  [+]
                            ├── <18> Var [return_val]
                            ╰── <21> Var [void2]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_local_var_missing_return() {
    let src = r#"
        int main(void) {
            int a = 3;
            a = a + 5;
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
                    ╰── <19> Assign [=]
                        ├── <12> Var [a]
                        ╰── <18>  [+]
                            ├── <15> Var [a]
                            ╰── <17> Constant Int [5]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_mixed_precedence_assignment() {
    let src = r#"
        int main(void) {
            int a = 1;
            int b = 0;
            a = 3 * (b = a);
            return a + b;
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
                    │       ╰── <8> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <14> Constant Int [0]
                    ├── <30> Assign [=]
                    │   ├── <18> Var [a]
                    │   ╰── <29>  [*]
                    │       ├── <20> Constant Int [3]
                    │       ╰── <28> Assign [=]
                    │           ├── <23> Var [b]
                    │           ╰── <26> Var [a]
                    ╰── Return
                        ╰── <37>  [+]
                            ├── <33> Var [a]
                            ╰── <36> Var [b]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_non_short_circuit_or() {
    let src = r#"
        int main(void) {
            int a = 0;
            0 || (a = 1);
            return a;
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
                    │       ╰── <8> Constant Int [0]
                    ├── <19>  [||]
                    │   ├── <11> Constant Int [0]
                    │   ╰── <18> Assign [=]
                    │       ├── <14> Var [a]
                    │       ╰── <16> Constant Int [1]
                    ╰── Return
                        ╰── <22> Var [a]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_null_statement() {
    let src = r#"
        int main(void) {
            ;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ╰── Empty
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_null_then_return() {
    let src = r#"
        int main(void) {
            ;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── Empty
                    ╰── Return
                        ╰── <6> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_return_var() {
    let src = r#"
        int main(void) {
            int a = 2;
            return a;
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
                    │       ╰── <8> Constant Int [2]
                    ╰── Return
                        ╰── <12> Var [a]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_short_circuit_and_fail() {
    let src = r#"
        int main(void) {
            int a = 0;
            0 && (a = 5);
            return a;
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
                    │       ╰── <8> Constant Int [0]
                    ├── <19>  [&&]
                    │   ├── <11> Constant Int [0]
                    │   ╰── <18> Assign [=]
                    │       ├── <14> Var [a]
                    │       ╰── <16> Constant Int [5]
                    ╰── Return
                        ╰── <22> Var [a]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_short_circuit_or() {
    let src = r#"
        int main(void) {
            int a = 0;
            1 || (a = 1);
            return a;
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
                    │       ╰── <8> Constant Int [0]
                    ├── <19>  [||]
                    │   ├── <11> Constant Int [1]
                    │   ╰── <18> Assign [=]
                    │       ├── <14> Var [a]
                    │       ╰── <16> Constant Int [1]
                    ╰── Return
                        ╰── <22> Var [a]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_unused_exp() {
    let src = r#"
        int main(void) {
            2 + 2;
            return 0;
        }
    "#;
    let expected = r#"
        Program
            ╰── Function [main]
                ╰── Body
                    ├── <8>  [+]
                    │   ├── <5> Constant Int [2]
                    │   ╰── <7> Constant Int [2]
                    ╰── Return
                        ╰── <10> Constant Int [0]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_use_assignment_result() {
    let src = r#"
        int main(void) {
            int a = 1;
            int b = 2;
            return a = b = 4;
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
                    │       ╰── <8> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <14> Constant Int [2]
                    ╰── Return
                        ╰── <25> Assign [=]
                            ├── <18> Var [a]
                            ╰── <24> Assign [=]
                                ├── <21> Var [b]
                                ╰── <23> Constant Int [4]
    "#;
    assert_parse(src, expected);
}

#[test]
fn test_valid_use_val_in_own_initializer() {
    let src = r#"
        int main(void) {
            int a = 0 && a;
            return a;
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
                    │       ╰── <12>  [&&]
                    │           ├── <8> Constant Int [0]
                    │           ╰── <11> Var [a]
                    ╰── Return
                        ╰── <16> Var [a]
    "#;
    assert_parse(src, expected);
}
