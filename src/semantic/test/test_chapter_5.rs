use super::assert_error;

#[test]
fn test_invalid_semantics_declared_after_use() {
    assert_error(
        r#"
        int main(void) {
            a = 1 + 2;
          //^ Undeclared variable 'a'
            int a;
            return a;
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_extra_credit_compound_invalid_lvalue() {
    assert_error(
        r#"
        int main(void) {
            int a = 0;
            -a += 1;
          //^^ Expression is not assignable
            return a;
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_extra_credit_compound_invalid_lvalue_2() {
    assert_error(
        r#"
        int main(void) {
            int a = 10;
            (a += 1) -= 2;
          //^^^^^^^^ Expression is not assignable
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_extra_credit_postfix_decr_non_lvalue() {
    assert_error(
        r#"
        int main(void) {
            int a = 10;
            return a++--;
                 //^^^ Expression is not assignable
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_extra_credit_postfix_incr_non_lvalue() {
    assert_error(
        r#"
        int main(void) {
            int a = 0;
            (a = 4)++;
          //^^^^^^^ Expression is not assignable
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_extra_credit_prefix_decr_non_lvalue() {
    assert_error(
        r#"
        int main(void) {
            return --3;
                   //^ Expression is not assignable
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_extra_credit_prefix_incr_non_lvalue() {
    assert_error(
        r#"
        int main(void) {
            int a = 1;
            ++(a+1);
            //^^^^^ Expression is not assignable
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_extra_credit_undeclared_bitwise_op() {
    assert_error(
        r#"
        int main(void){
            return a >> 2;
                 //^ Undeclared variable 'a'
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_extra_credit_undeclared_compound_assignment() {
    assert_error(
        r#"
        int main(void) {
            a += 1;
          //^ Undeclared variable 'a'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_extra_credit_undeclared_compound_assignment_use() {
    assert_error(
        r#"
        int main(void) {
            int b = 10;
            b *= a;
               //^ Undeclared variable 'a'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_extra_credit_undeclared_postfix_decr() {
    assert_error(
        r#"
        int main(void) {
            a--;
          //^ Undeclared variable 'a'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_extra_credit_undeclared_prefix_incr() {
    assert_error(
        r#"
        int main(void) {
            a++;
          //^ Undeclared variable 'a'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_invalid_lvalue() {
    assert_error(
        r#"
        int main(void) {
            int a = 2;
            a + 3 = 4;
          //^^^^^ Expression is not assignable
            return a;
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_invalid_lvalue_2() {
    assert_error(
        r#"
        int main(void) {
            int a = 2;
            !a = 3;
          //^^ Expression is not assignable
            return a;
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_mixed_precedence_assignment() {
    assert_error(
        r#"
        int main(void) {
            int a = 1;
            int b = 2;
            a = 3 * b = a;
              //^^^^^ Expression is not assignable
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_redefine() {
    assert_error(
        r#"
        int main(void) {
            int a = 1;
            int a = 2;
              //^ Variable 'a' was already declared
            return a;
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_undeclared_var() {
    assert_error(
        r#"
        int main(void) {
            return a;
                 //^ Undeclared variable 'a'
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_undeclared_var_and() {
    assert_error(
        r#"
        int main(void) {
            return 0 && a;
                      //^ Undeclared variable 'a'
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_undeclared_var_compare() {
    assert_error(
        r#"
        int main(void) {
            return a < 5;
                 //^ Undeclared variable 'a'
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_undeclared_var_unary() {
    assert_error(
        r#"
        int main(void) {
            return -a;
                  //^ Undeclared variable 'a'
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_use_then_redefine() {
    assert_error(
        r#"
        int main(void) {
            int a = 0;
            return a;
            int a = 1;
              //^ Variable 'a' was already declared
            return a;
        }
    "#,
    );
}
