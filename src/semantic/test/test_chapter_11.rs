use super::assert_error;

#[test]
fn test_invalid_labels_extra_credit_bitshift_duplicate_cases() {
    assert_error(
        r#"
        int main(void) {
            int x = 100;
            switch (x << 2l) {
                case 34359738768l:
                    return 1;
                case 400:
                   //^^^ duplicate case value
                    return 0;
            }
            return 10;
        }
    "#,
    );
}

#[test]
fn test_invalid_labels_extra_credit_switch_duplicate_cases() {
    assert_error(
        r#"
        int switch_statement(int i) {
            switch(i) {
                case 0: return 0;
                case 17179869184: return 0;
                   //^^^^^^^^^^^ duplicate case value
                default: return 1;
            }
        }
        int main(void) {
            return switch_statement(0);
        }
    "#,
    );
}

#[test]
fn test_invalid_labels_extra_credit_switch_duplicate_cases_2() {
    assert_error(
        r#"
        int switch_statement(int i) {
            switch((long) i) {
                case 100l: return 0;
                case 100: return 0;
                   //^^^ duplicate case value
                default: return 1;
            }
        }
        int main(void) {
            return switch_statement(100);
        }
    "#,
    );
}

#[test]
fn test_invalid_types_call_long_as_function() {
    assert_error(
        r#"
        long x(void);
        int main(void) {
            long x = 0;
            return x();
                 //^ Variable used as function name
        }
    "#,
    );
}

#[test]
fn test_invalid_types_cast_lvalue() {
    assert_error(
        r#"
        int main(void) {
            int i = 0;
            i = (long) i = 10;
              //^^^^^^^^ Expression is not assignable
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_conflicting_function_types() {
    assert_error(
        r#"
        int foo(int a);
        int main(void) {
            return 0;
        }
        int foo(long a);
          //^^^ Conflicting declaration types for 'foo'
    "#,
    );
}

#[test]
fn test_invalid_types_conflicting_global_types() {
    assert_error(
        r#"
        int foo = 3;
        long foo;
           //^^^ Variable 'foo' is already declared with a different type
        int main(void) {
            return foo;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_conflicting_variable_types() {
    assert_error(
        r#"
        long a;
        int main(void) {
            extern int a;
                     //^ Name 'a' was already declared
            return 0;
        }
    "#,
    );
}
