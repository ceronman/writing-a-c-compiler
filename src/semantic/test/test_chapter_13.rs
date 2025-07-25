use super::assert_error;

#[test]
fn test_invalid_types_complement_double() {
    assert_error(
        r#"
        int main(void) {
            double d = ~10.0;
                      //^^^^ Unary operator requires an integer type
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_bitwise_and() {
    assert_error(
        r#"
        int main(void) {
            double d = 10.0 & -1;
                     //^^^^ Operator requires integer operands
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_bitwise_or() {
    assert_error(
        r#"
        int main(void) {
            double d = 0.0 | -0.0;
                     //^^^ Operator requires integer operands
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_bitwise_shift_double() {
    assert_error(
        r#"
        int main(void) {
            double d = 5.0 << 3;
                     //^^^ Operator requires integer operands
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_bitwise_shift_double_2() {
    assert_error(
        r#"
        int main(void) {
            return 1 << 2.0;
                      //^^^ Operator requires integer operands
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_bitwise_xor() {
    assert_error(
        r#"
        int main(void) {
            return 1e10 ^ -1e10;
                 //^^^^ Operator requires integer operands
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_compound_bitwise_and() {
    assert_error(
        r#"
        int main(void) {
            double d = 1.0;
            d &= 0;
          //^ Assign compound operation requires integer operands
            return (int) d;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_compound_bitwise_xor() {
    assert_error(
        r#"
        int main(void) {
            int i = 0;
            i |= 2.0;
               //^^^ Assign compound operation requires integer operands
            return (int) i;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_compound_left_bitshift() {
    assert_error(
        r#"
        int main(void) {
            double d = 1.0;
            d <<= 1;
          //^ Assign compound operation requires integer operands
            return d;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_compound_mod() {
    assert_error(
        r#"
        
        int main(void) {
            double d = 5.0;
            d %= 2;
          //^ Assign compound operation requires integer operands
            return (int) d;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_compound_mod_2() {
    assert_error(
        r#"
        
        int main(void) {
            int i = 5;
            i %= 1.0;
               //^^^ Assign compound operation requires integer operands
            return i;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_compound_right_bitshift() {
    assert_error(
        r#"
        int main(void) {
            int i = 1000;
            i >>= 2.0;
                //^^^ Assign compound operation requires integer operands
            return i;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_switch_double_case() {
    assert_error(
        r#"
        int main(void) {
            int x = 10;
            switch (x) {
                case 1.0: return 0;
                   //^^^ case label does not reduce to an integer constant
                default: return 4;
            }
        }
    "#,
    );
}

#[test]
fn test_invalid_types_extra_credit_switch_on_double() {
    assert_error(
        r#"
        int main(void) {
            double d = 10;
            switch (d) {
                  //^ Switch statement requires an integer expression
                case 10: return 0;
            }
            return 1;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_mod_double() {
    assert_error(
        r#"
        int main(void) {
            double d = 10.0;
            d = d % 3;
              //^ Operator requires integer operands
            return 0;
        }
    "#,
    );
}

#[test]
fn test_invalid_types_mod_double_2() {
    assert_error(
        r#"
        int main(void) {
            double e = 3.0 % 5;
                     //^^^ Operator requires integer operands
            return 0;
        }
    "#,
    );
}
