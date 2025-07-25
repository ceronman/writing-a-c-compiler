use super::assert_error;

#[test]
fn test_invalid_semantics_extra_credit_duplicate_labels() {
    assert_error(
        r#"
        
        int main(void) {
            int x = 0;
        label:
            x = 1;
        label:
      //^^^^^ Label 'label' was already defined
            return 2;
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_extra_credit_goto_missing_label() {
    assert_error(
        r#"
        int main(void) {
            goto label;
               //^^^^^ Undefined label 'label'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_extra_credit_goto_variable() {
    assert_error(
        r#"
        int main(void) {
            int a;
            goto a;
               //^ Undefined label 'a'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_extra_credit_undeclared_var_in_labeled_statement() {
    assert_error(
        r#"
        int main(void) {
        lbl:
            return a;
                 //^ Undeclared variable 'a'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_extra_credit_use_label_as_variable() {
    assert_error(
        r#"
        int main(void) {
            int x = 0;
            a:
            x = a;
              //^ Undeclared variable 'a'
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_invalid_var_in_if() {
    assert_error(
        r#"
        int main(void) {
            if (1)
                return c;
                     //^ Undeclared variable 'c'
            int c = 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_ternary_assign() {
    assert_error(
        r#"
        int main(void) {
            int a = 2;
            int b = 1;
            a > b ? a = 1 : a = 0;
          //^^^^^^^^^^^^^^^^^ Expression is not assignable
            return a;
        }
    "#,
    );
}

#[test]
fn test_invalid_semantics_undeclared_var_in_ternary() {
    assert_error(
        r#"
        int main(void) {
            return a > 0 ? 1 : 2;
                 //^ Undeclared variable 'a'
            int a = 5;
        }
    "#,
    );
}
