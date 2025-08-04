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
                    ├── <13> Assign [=]
                    │   ├── <7> Var [a]
                    │   ╰── <12>  [+]
                    │       ├── <9> Constant Int [1]
                    │       ╰── <11> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ╰── Type
                    │       ╰── Int
                    ╰── Return
                        ╰── <20> Var [a]
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
                    │       ╰── <9> Constant Int [0]
                    ├── <18> Assign [+=]
                    │   ├── <15> Unary [-]
                    │   │   ╰── <14> Var [a]
                    │   ╰── <17> Constant Int [1]
                    ╰── Return
                        ╰── <21> Var [a]
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
                    │       ╰── <9> Constant Int [10]
                    ╰── <20> Assign [-=]
                        ├── <17> Assign [+=]
                        │   ├── <13> Var [a]
                        │   ╰── <15> Constant Int [1]
                        ╰── <19> Constant Int [2]
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
                    │       ╰── <9> Constant Int [10]
                    ╰── Return
                        ╰── <17> Postfix [--]
                            ╰── <15> Postfix [++]
                                ╰── <13> Var [a]
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
                    │       ╰── <9> Constant Int [0]
                    ╰── <19> Postfix [++]
                        ╰── <17> Assign [=]
                            ├── <13> Var [a]
                            ╰── <15> Constant Int [4]
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
                        ╰── <8> Unary [--]
                            ╰── <7> Constant Int [3]
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
                    │       ╰── <9> Constant Int [1]
                    ├── <19> Unary [++]
                    │   ╰── <18>  [+]
                    │       ├── <14> Var [a]
                    │       ╰── <16> Constant Int [1]
                    ╰── Return
                        ╰── <21> Constant Int [0]
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
                        ╰── <10>  [>>]
                            ├── <7> Var [a]
                            ╰── <9> Constant Int [2]
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
                    ├── <10> Assign [+=]
                    │   ├── <7> Var [a]
                    │   ╰── <9> Constant Int [1]
                    ╰── Return
                        ╰── <12> Constant Int [0]
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
                    │       ╰── <9> Constant Int [10]
                    ├── <17> Assign [*=]
                    │   ├── <13> Var [b]
                    │   ╰── <16> Var [a]
                    ╰── Return
                        ╰── <19> Constant Int [0]
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
                    ├── <9> Postfix [--]
                    │   ╰── <7> Var [a]
                    ╰── Return
                        ╰── <11> Constant Int [0]
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
                    ├── <9> Postfix [++]
                    │   ╰── <7> Var [a]
                    ╰── Return
                        ╰── <11> Constant Int [0]
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
                    │       ╰── <9> Constant Int [2]
                    ├── <19> Assign [=]
                    │   ├── <16>  [+]
                    │   │   ├── <13> Var [a]
                    │   │   ╰── <15> Constant Int [3]
                    │   ╰── <18> Constant Int [4]
                    ╰── Return
                        ╰── <22> Var [a]
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
                    │       ╰── <9> Constant Int [2]
                    ├── <18> Assign [=]
                    │   ├── <15> Unary [!]
                    │   │   ╰── <14> Var [a]
                    │   ╰── <17> Constant Int [3]
                    ╰── Return
                        ╰── <21> Var [a]
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
                    │       ╰── <9> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [2]
                    ╰── <30> Assign [=]
                        ├── <19> Var [a]
                        ╰── <29> Assign [=]
                            ├── <25>  [*]
                            │   ├── <21> Constant Int [3]
                            │   ╰── <24> Var [b]
                            ╰── <28> Var [a]
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
                    │       ╰── <9> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [2]
                    ╰── Return
                        ╰── <19> Var [a]
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
                        ╰── <7> Var [a]
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
                        ╰── <10>  [&&]
                            ├── <6> Constant Int [0]
                            ╰── <9> Var [a]
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
                        ╰── <10>  [<]
                            ├── <7> Var [a]
                            ╰── <9> Constant Int [5]
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
                        ╰── <9> Unary [-]
                            ╰── <8> Var [a]
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
                    │       ╰── <9> Constant Int [0]
                    ├── Return
                    │   ╰── <13> Var [a]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── a
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <18> Constant Int [1]
                    ╰── Return
                        ╰── <22> Var [a]
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
                    │       ╰── <9> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── second_variable
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [2]
                    ╰── Return
                        ╰── <23>  [+]
                            ├── <19> Var [first_variable]
                            ╰── <22> Var [second_variable]
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
                    │       ╰── <9> Constant Int [2147483646]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <31>  [+]
                    │           ├── <25>  [/]
                    │           │   ├── <22> Var [a]
                    │           │   ╰── <24> Constant Int [6]
                    │           ╰── <30> Unary [!]
                    │               ╰── <29> Var [b]
                    ╰── Return
                        ╰── <45>  [==]
                            ├── <38>  [*]
                            │   ├── <35> Var [c]
                            │   ╰── <37> Constant Int [2]
                            ╰── <44>  [-]
                                ├── <41> Var [a]
                                ╰── <43> Constant Int [1431655762]
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
                    ├── <14> Assign [=]
                    │   ├── <11> Var [var0]
                    │   ╰── <13> Constant Int [2]
                    ╰── Return
                        ╰── <17> Var [var0]
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
                    │       ╰── <13> Assign [=]
                    │           ├── <10> Var [a]
                    │           ╰── <12> Constant Int [5]
                    ╰── Return
                        ╰── <17> Var [a]
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
                    │       ╰── <17> Assign [=]
                    │           ├── <14> Var [a]
                    │           ╰── <16> Constant Int [0]
                    ╰── Return
                        ╰── <21> Var [b]
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
                    ├── <17> Assign [=]
                    │   ├── <11> Var [a]
                    │   ╰── <16>  [||]
                    │       ├── <13> Constant Int [0]
                    │       ╰── <15> Constant Int [5]
                    ╰── Return
                        ╰── <20> Var [a]
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
                    │       ╰── <11> Unary [-]
                    │           ╰── <10> Constant Int [2593]
                    ├── <22> Assign [=]
                    │   ├── <15> Var [a]
                    │   ╰── <21>  [%]
                    │       ├── <18> Var [a]
                    │       ╰── <20> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <30> Unary [-]
                    │           ╰── <29> Var [a]
                    ╰── Return
                        ╰── <34> Var [b]
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
                    │       ╰── <9> Constant Int [15]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <19>  [^]
                    │           ├── <16> Var [a]
                    │           ╰── <18> Constant Int [5]
                    ╰── Return
                        ╰── <26>  [|]
                            ├── <22> Constant Int [1]
                            ╰── <25> Var [b]
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
                    │       ╰── <9> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [5]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <21> Constant Int [8]
                    ╰── Return
                        ╰── <33>  [|]
                            ├── <29>  [&]
                            │   ├── <25> Var [a]
                            │   ╰── <28> Var [b]
                            ╰── <32> Var [c]
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
                    │       ╰── <9> Constant Int [3]
                    ╰── Return
                        ╰── <16>  [<<]
                            ├── <13> Var [x]
                            ╰── <15> Constant Int [3]
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
                    │       ╰── <9> Constant Int [1234]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [0]
                    ├── <26> Assign [=]
                    │   ├── <19> Var [x]
                    │   ╰── <25>  [>>]
                    │       ├── <22> Var [var_to_shift]
                    │       ╰── <24> Constant Int [4]
                    ╰── Return
                        ╰── <29> Var [x]
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
                    │       ╰── <9> Constant Int [250]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [200]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <21> Constant Int [100]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <27> Constant Int [75]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── e
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <35> Unary [-]
                    │           ╰── <34> Constant Int [25]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── f
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <41> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <47> Constant Int [0]
                    ├── <80> Assign [=]
                    │   ├── <51> Var [x]
                    │   ╰── <79> Assign [+=]
                    │       ├── <54> Var [a]
                    │       ╰── <78> Assign [-=]
                    │           ├── <57> Var [b]
                    │           ╰── <77> Assign [*=]
                    │               ├── <60> Var [c]
                    │               ╰── <76> Assign [/=]
                    │                   ├── <63> Var [d]
                    │                   ╰── <75> Assign [&=]
                    │                       ├── <66> Var [e]
                    │                       ╰── <74> Assign [=]
                    │                           ├── <69> Var [f]
                    │                           ╰── <73> Unary [-]
                    │                               ╰── <72> Constant Int [7]
                    ╰── Return
                        ╰── <136>  [&&]
                            ├── <129>  [&&]
                            │   ├── <120>  [&&]
                            │   │   ├── <111>  [&&]
                            │   │   │   ├── <102>  [&&]
                            │   │   │   │   ├── <93>  [&&]
                            │   │   │   │   │   ├── <86>  [==]
                            │   │   │   │   │   │   ├── <83> Var [a]
                            │   │   │   │   │   │   ╰── <85> Constant Int [2250]
                            │   │   │   │   │   ╰── <92>  [==]
                            │   │   │   │   │       ├── <89> Var [b]
                            │   │   │   │   │       ╰── <91> Constant Int [2000]
                            │   │   │   │   ╰── <101>  [==]
                            │   │   │   │       ├── <96> Var [c]
                            │   │   │   │       ╰── <100> Unary [-]
                            │   │   │   │           ╰── <99> Constant Int [1800]
                            │   │   │   ╰── <110>  [==]
                            │   │   │       ├── <105> Var [d]
                            │   │   │       ╰── <109> Unary [-]
                            │   │   │           ╰── <108> Constant Int [18]
                            │   │   ╰── <119>  [==]
                            │   │       ├── <114> Var [e]
                            │   │       ╰── <118> Unary [-]
                            │   │           ╰── <117> Constant Int [4]
                            │   ╰── <128>  [==]
                            │       ├── <123> Var [f]
                            │       ╰── <127> Unary [-]
                            │           ╰── <126> Constant Int [7]
                            ╰── <135>  [==]
                                ├── <132> Var [x]
                                ╰── <134> Constant Int [2250]
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
                    │       ╰── <9> Constant Int [10]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [12]
                    ├── <26> Assign [+=]
                    │   ├── <19> Var [a]
                    │   ╰── <25>  [||]
                    │       ├── <21> Constant Int [0]
                    │       ╰── <24> Var [b]
                    ├── <36> Assign [*=]
                    │   ├── <29> Var [b]
                    │   ╰── <35>  [&&]
                    │       ├── <32> Var [a]
                    │       ╰── <34> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <41> Constant Int [14]
                    ├── <53> Assign [-=]
                    │   ├── <45> Var [c]
                    │   ╰── <52>  [||]
                    │       ├── <48> Var [a]
                    │       ╰── <51> Var [b]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <58> Constant Int [16]
                    ├── <70> Assign [/=]
                    │   ├── <62> Var [d]
                    │   ╰── <69>  [||]
                    │       ├── <65> Var [c]
                    │       ╰── <68> Var [d]
                    ╰── Return
                        ╰── <98>  [&&]
                            ├── <90>  [&&]
                            │   ├── <83>  [&&]
                            │   │   ├── <76>  [==]
                            │   │   │   ├── <73> Var [a]
                            │   │   │   ╰── <75> Constant Int [11]
                            │   │   ╰── <82>  [==]
                            │   │       ├── <79> Var [b]
                            │   │       ╰── <81> Constant Int [0]
                            │   ╰── <89>  [==]
                            │       ├── <86> Var [c]
                            │       ╰── <88> Constant Int [13]
                            ╰── <96>  [==]
                                ├── <93> Var [d]
                                ╰── <95> Constant Int [16]
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
                    │       ╰── <9> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── y
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <19> Assign [+=]
                    │           ├── <16> Var [x]
                    │           ╰── <18> Constant Int [3]
                    ╰── Return
                        ╰── <34>  [&&]
                            ├── <26>  [==]
                            │   ├── <23> Var [x]
                            │   ╰── <25> Constant Int [4]
                            ╰── <32>  [==]
                                ├── <29> Var [y]
                                ╰── <31> Constant Int [4]
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
                    │       ╰── <9> Constant Int [3]
                    ├── <16> Assign [&=]
                    │   ├── <13> Var [to_and]
                    │   ╰── <15> Constant Int [6]
                    ╰── Return
                        ╰── <19> Var [to_and]
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
                    │       ╰── <9> Constant Int [11]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [12]
                    ├── <26> Assign [&=]
                    │   ├── <19> Var [a]
                    │   ╰── <25>  [||]
                    │       ├── <21> Constant Int [0]
                    │       ╰── <24> Var [b]
                    ├── <36> Assign [^=]
                    │   ├── <29> Var [b]
                    │   ╰── <35>  [||]
                    │       ├── <32> Var [a]
                    │       ╰── <34> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <41> Constant Int [14]
                    ├── <53> Assign [|=]
                    │   ├── <45> Var [c]
                    │   ╰── <52>  [||]
                    │       ├── <48> Var [a]
                    │       ╰── <51> Var [b]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <58> Constant Int [16]
                    ├── <70> Assign [>>=]
                    │   ├── <62> Var [d]
                    │   ╰── <69>  [||]
                    │       ├── <65> Var [c]
                    │       ╰── <68> Var [d]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── e
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <75> Constant Int [18]
                    ├── <87> Assign [<<=]
                    │   ├── <79> Var [e]
                    │   ╰── <86>  [||]
                    │       ├── <82> Var [c]
                    │       ╰── <85> Var [d]
                    ╰── Return
                        ╰── <122>  [&&]
                            ├── <114>  [&&]
                            │   ├── <107>  [&&]
                            │   │   ├── <100>  [&&]
                            │   │   │   ├── <93>  [==]
                            │   │   │   │   ├── <90> Var [a]
                            │   │   │   │   ╰── <92> Constant Int [1]
                            │   │   │   ╰── <99>  [==]
                            │   │   │       ├── <96> Var [b]
                            │   │   │       ╰── <98> Constant Int [13]
                            │   │   ╰── <106>  [==]
                            │   │       ├── <103> Var [c]
                            │   │       ╰── <105> Constant Int [15]
                            │   ╰── <113>  [==]
                            │       ├── <110> Var [d]
                            │       ╰── <112> Constant Int [8]
                            ╰── <120>  [==]
                                ├── <117> Var [e]
                                ╰── <119> Constant Int [36]
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
                    │       ╰── <9> Constant Int [250]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [200]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <21> Constant Int [100]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <27> Constant Int [75]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── e
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <33> Constant Int [50]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── f
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <39> Constant Int [25]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── g
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <45> Constant Int [10]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── h
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <51> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── j
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <57> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── x
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <63> Constant Int [0]
                    ├── <106> Assign [=]
                    │   ├── <67> Var [x]
                    │   ╰── <105> Assign [&=]
                    │       ├── <70> Var [a]
                    │       ╰── <104> Assign [*=]
                    │           ├── <73> Var [b]
                    │           ╰── <103> Assign [|=]
                    │               ├── <76> Var [c]
                    │               ╰── <102> Assign [=]
                    │                   ├── <79> Var [d]
                    │                   ╰── <101> Assign [^=]
                    │                       ├── <82> Var [e]
                    │                       ╰── <100> Assign [+=]
                    │                           ├── <85> Var [f]
                    │                           ╰── <99> Assign [>>=]
                    │                               ├── <88> Var [g]
                    │                               ╰── <98> Assign [<<=]
                    │                                   ├── <91> Var [h]
                    │                                   ╰── <97> Assign [=]
                    │                                       ├── <94> Var [j]
                    │                                       ╰── <96> Constant Int [1]
                    ╰── Return
                        ╰── <176>  [&&]
                            ├── <168>  [&&]
                            │   ├── <161>  [&&]
                            │   │   ├── <154>  [&&]
                            │   │   │   ├── <147>  [&&]
                            │   │   │   │   ├── <140>  [&&]
                            │   │   │   │   │   ├── <133>  [&&]
                            │   │   │   │   │   │   ├── <126>  [&&]
                            │   │   │   │   │   │   │   ├── <119>  [&&]
                            │   │   │   │   │   │   │   │   ├── <112>  [==]
                            │   │   │   │   │   │   │   │   │   ├── <109> Var [a]
                            │   │   │   │   │   │   │   │   │   ╰── <111> Constant Int [40]
                            │   │   │   │   │   │   │   │   ╰── <118>  [==]
                            │   │   │   │   │   │   │   │       ├── <115> Var [b]
                            │   │   │   │   │   │   │   │       ╰── <117> Constant Int [21800]
                            │   │   │   │   │   │   │   ╰── <125>  [==]
                            │   │   │   │   │   │   │       ├── <122> Var [c]
                            │   │   │   │   │   │   │       ╰── <124> Constant Int [109]
                            │   │   │   │   │   │   ╰── <132>  [==]
                            │   │   │   │   │   │       ├── <129> Var [d]
                            │   │   │   │   │   │       ╰── <131> Constant Int [41]
                            │   │   │   │   │   ╰── <139>  [==]
                            │   │   │   │   │       ├── <136> Var [e]
                            │   │   │   │   │       ╰── <138> Constant Int [41]
                            │   │   │   │   ╰── <146>  [==]
                            │   │   │   │       ├── <143> Var [f]
                            │   │   │   │       ╰── <145> Constant Int [27]
                            │   │   │   ╰── <153>  [==]
                            │   │   │       ├── <150> Var [g]
                            │   │   │       ╰── <152> Constant Int [2]
                            │   │   ╰── <160>  [==]
                            │   │       ├── <157> Var [h]
                            │   │       ╰── <159> Constant Int [2]
                            │   ╰── <167>  [==]
                            │       ├── <164> Var [j]
                            │       ╰── <166> Constant Int [1]
                            ╰── <174>  [==]
                                ├── <171> Var [x]
                                ╰── <173> Constant Int [40]
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
                    │       ╰── <9> Constant Int [1]
                    ├── <16> Assign [|=]
                    │   ├── <13> Var [to_or]
                    │   ╰── <15> Constant Int [30]
                    ╰── Return
                        ╰── <19> Var [to_or]
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
                    │       ╰── <9> Constant Int [3]
                    ├── <16> Assign [<<=]
                    │   ├── <13> Var [to_shiftl]
                    │   ╰── <15> Constant Int [4]
                    ╰── Return
                        ╰── <19> Var [to_shiftl]
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
                    │       ╰── <9> Constant Int [382574]
                    ├── <16> Assign [>>=]
                    │   ├── <13> Var [to_shiftr]
                    │   ╰── <15> Constant Int [4]
                    ╰── Return
                        ╰── <19> Var [to_shiftr]
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
                    │       ╰── <9> Constant Int [7]
                    ├── <16> Assign [^=]
                    │   ├── <13> Var [to_xor]
                    │   ╰── <15> Constant Int [5]
                    ╰── Return
                        ╰── <19> Var [to_xor]
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
                    │       ╰── <9> Constant Int [8]
                    ├── <16> Assign [/=]
                    │   ├── <13> Var [to_divide]
                    │   ╰── <15> Constant Int [4]
                    ╰── Return
                        ╰── <19> Var [to_divide]
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
                    │       ╰── <9> Constant Int [10]
                    ├── <16> Assign [-=]
                    │   ├── <13> Var [to_subtract]
                    │   ╰── <15> Constant Int [8]
                    ╰── Return
                        ╰── <19> Var [to_subtract]
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
                    │       ╰── <9> Constant Int [5]
                    ├── <16> Assign [&=]
                    │   ├── <13> Var [to_mod]
                    │   ╰── <15> Constant Int [3]
                    ╰── Return
                        ╰── <19> Var [to_mod]
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
                    │       ╰── <9> Constant Int [4]
                    ├── <16> Assign [*=]
                    │   ├── <13> Var [to_multiply]
                    │   ╰── <15> Constant Int [3]
                    ╰── Return
                        ╰── <19> Var [to_multiply]
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
                    │       ╰── <9> Constant Int [0]
                    ├── <16> Assign [+=]
                    │   ├── <13> Var [to_add]
                    │   ╰── <15> Constant Int [4]
                    ╰── Return
                        ╰── <19> Var [to_add]
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
                    │       ╰── <9> Constant Int [0]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [0]
                    ├── <21> Postfix [++]
                    │   ╰── <19> Var [a]
                    ├── <26> Unary [++]
                    │   ╰── <25> Var [a]
                    ├── <31> Unary [++]
                    │   ╰── <30> Var [a]
                    ├── <36> Postfix [--]
                    │   ╰── <34> Var [b]
                    ├── <41> Unary [--]
                    │   ╰── <40> Var [b]
                    ╰── Return
                        ╰── <57>  [&&]
                            ├── <47>  [==]
                            │   ├── <44> Var [a]
                            │   ╰── <46> Constant Int [3]
                            ╰── <55>  [==]
                                ├── <50> Var [b]
                                ╰── <54> Unary [-]
                                    ╰── <53> Constant Int [2]
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
                    │       ╰── <9> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <21>  [+]
                    │           ├── <15> Constant Int [3]
                    │           ╰── <20> Postfix [++]
                    │               ╰── <18> Var [a]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <33>  [+]
                    │           ├── <27> Constant Int [4]
                    │           ╰── <32> Unary [++]
                    │               ╰── <31> Var [b]
                    ╰── Return
                        ╰── <55>  [&&]
                            ├── <47>  [&&]
                            │   ├── <40>  [==]
                            │   │   ├── <37> Var [a]
                            │   │   ╰── <39> Constant Int [3]
                            │   ╰── <46>  [==]
                            │       ├── <43> Var [b]
                            │       ╰── <45> Constant Int [6]
                            ╰── <53>  [==]
                                ├── <50> Var [c]
                                ╰── <52> Constant Int [10]
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
                    │       ╰── <9> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <27> Unary [-]
                    │           ╰── <26> Unary [++]
                    │               ╰── <25> Var [a]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <39> Unary [!]
                    │           ╰── <38> Postfix [--]
                    │               ╰── <36> Var [b]
                    ╰── Return
                        ╰── <70>  [&&]
                            ├── <62>  [&&]
                            │   ├── <53>  [&&]
                            │   │   ├── <46>  [==]
                            │   │   │   ├── <43> Var [a]
                            │   │   │   ╰── <45> Constant Int [2]
                            │   │   ╰── <52>  [==]
                            │   │       ├── <49> Var [b]
                            │   │       ╰── <51> Constant Int [1]
                            │   ╰── <61>  [==]
                            │       ├── <56> Var [c]
                            │       ╰── <60> Unary [-]
                            │           ╰── <59> Constant Int [2]
                            ╰── <68>  [==]
                                ├── <65> Var [d]
                                ╰── <67> Constant Int [0]
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
                    │       ╰── <9> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <24> Postfix [++]
                    │           ╰── <22> Var [a]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <33> Postfix [--]
                    │           ╰── <31> Var [b]
                    ╰── Return
                        ╰── <62>  [&&]
                            ├── <54>  [&&]
                            │   ├── <47>  [&&]
                            │   │   ├── <40>  [==]
                            │   │   │   ├── <37> Var [a]
                            │   │   │   ╰── <39> Constant Int [2]
                            │   │   ╰── <46>  [==]
                            │   │       ├── <43> Var [b]
                            │   │       ╰── <45> Constant Int [1]
                            │   ╰── <53>  [==]
                            │       ├── <50> Var [c]
                            │       ╰── <52> Constant Int [1]
                            ╰── <60>  [==]
                                ├── <57> Var [d]
                                ╰── <59> Constant Int [2]
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
                    │       ╰── <9> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <20> Unary [!]
                    │           ╰── <19> Postfix [++]
                    │               ╰── <17> Var [a]
                    ╰── Return
                        ╰── <35>  [&&]
                            ├── <27>  [==]
                            │   ├── <24> Var [a]
                            │   ╰── <26> Constant Int [2]
                            ╰── <33>  [==]
                                ├── <30> Var [b]
                                ╰── <32> Constant Int [0]
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
                    │       ╰── <9> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [2]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── c
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <24> Unary [++]
                    │           ╰── <23> Var [a]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── d
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <33> Unary [--]
                    │           ╰── <32> Var [b]
                    ╰── Return
                        ╰── <62>  [&&]
                            ├── <54>  [&&]
                            │   ├── <47>  [&&]
                            │   │   ├── <40>  [==]
                            │   │   │   ├── <37> Var [a]
                            │   │   │   ╰── <39> Constant Int [2]
                            │   │   ╰── <46>  [==]
                            │   │       ├── <43> Var [b]
                            │   │       ╰── <45> Constant Int [1]
                            │   ╰── <53>  [==]
                            │       ├── <50> Var [c]
                            │       ╰── <52> Constant Int [2]
                            ╰── <60>  [==]
                                ├── <57> Var [d]
                                ╰── <59> Constant Int [1]
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
                    │       ╰── <9> Constant Int [3]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── void2
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [2]
                    ╰── Return
                        ╰── <23>  [+]
                            ├── <19> Var [return_val]
                            ╰── <22> Var [void2]
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
                    │       ╰── <9> Constant Int [3]
                    ╰── <20> Assign [=]
                        ├── <13> Var [a]
                        ╰── <19>  [+]
                            ├── <16> Var [a]
                            ╰── <18> Constant Int [5]
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
                    │       ╰── <9> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [0]
                    ├── <31> Assign [=]
                    │   ├── <19> Var [a]
                    │   ╰── <30>  [*]
                    │       ├── <21> Constant Int [3]
                    │       ╰── <29> Assign [=]
                    │           ├── <24> Var [b]
                    │           ╰── <27> Var [a]
                    ╰── Return
                        ╰── <38>  [+]
                            ├── <34> Var [a]
                            ╰── <37> Var [b]
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
                    │       ╰── <9> Constant Int [0]
                    ├── <20>  [||]
                    │   ├── <12> Constant Int [0]
                    │   ╰── <19> Assign [=]
                    │       ├── <15> Var [a]
                    │       ╰── <17> Constant Int [1]
                    ╰── Return
                        ╰── <23> Var [a]
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
                        ╰── <7> Constant Int [0]
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
                    │       ╰── <9> Constant Int [2]
                    ╰── Return
                        ╰── <13> Var [a]
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
                    │       ╰── <9> Constant Int [0]
                    ├── <20>  [&&]
                    │   ├── <12> Constant Int [0]
                    │   ╰── <19> Assign [=]
                    │       ├── <15> Var [a]
                    │       ╰── <17> Constant Int [5]
                    ╰── Return
                        ╰── <23> Var [a]
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
                    │       ╰── <9> Constant Int [0]
                    ├── <20>  [||]
                    │   ├── <12> Constant Int [1]
                    │   ╰── <19> Assign [=]
                    │       ├── <15> Var [a]
                    │       ╰── <17> Constant Int [1]
                    ╰── Return
                        ╰── <23> Var [a]
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
                    ├── <9>  [+]
                    │   ├── <6> Constant Int [2]
                    │   ╰── <8> Constant Int [2]
                    ╰── Return
                        ╰── <11> Constant Int [0]
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
                    │       ╰── <9> Constant Int [1]
                    ├── VarDeclaration
                    │   ├── Name
                    │   │   ╰── b
                    │   ├── Type
                    │   │   ╰── Int
                    │   ╰── Initializer
                    │       ╰── <15> Constant Int [2]
                    ╰── Return
                        ╰── <26> Assign [=]
                            ├── <19> Var [a]
                            ╰── <25> Assign [=]
                                ├── <22> Var [b]
                                ╰── <24> Constant Int [4]
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
                    │       ╰── <13>  [&&]
                    │           ├── <9> Constant Int [0]
                    │           ╰── <12> Var [a]
                    ╰── Return
                        ╰── <17> Var [a]
    "#;
    assert_parse(src, expected);
}
