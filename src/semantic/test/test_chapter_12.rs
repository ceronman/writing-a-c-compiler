use super::assert_error;

#[test]
fn test_invalid_labels_extra_credit_switch_duplicate_cases() {
    assert_error(
        r#"
        int main(void) {
            unsigned int ui = 10u;
            switch(ui) {
                case 4294967295u:
                    return 0;
                case 1099511627775l:
                   //^^^^^^^^^^^^^^ duplicate case value
                    return 1;
                default: return 2;
            }
        }
    "#,
    );
}

#[test]
fn test_invalid_types_conflicting_signed_unsigned() {
    assert_error(
        r#"
        unsigned x;
        int x;
          //^ Variable 'x' is already declared with a different type
        int main(void) {
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_conflicting_uint_ulong() {
    assert_error(
        r#"
        
        unsigned int foo(void);
        unsigned long foo(void) {
                    //^^^ Conflicting declaration types for 'foo'
            return 0;
        }
        int main(void) {
            return 0;
        }
    "#,
    );
}
