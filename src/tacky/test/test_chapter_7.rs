use crate::pretty::{dedent, dump_tacky};

#[test]
fn test_valid_assign_to_self() {
    let src = r#"
        int main(void) {
            int a = 3;
            {
                int a = a = 4;
                return a;
            }
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 3
            a.1 = 4
            a.1 = 4
            return a.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_assign_to_self_2() {
    let src = r#"
        int main(void) {
            int a = 3;
            {
                int a = a = 4;
            }
            return a;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 3
            a.1 = 4
            a.1 = 4
            return a.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_declaration_only() {
    let src = r#"
        int main(void) {
            int a;
            {
                int b = a = 1;
            }
            return a;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 1
            b.1 = 1
            return a.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_empty_blocks() {
    let src = r#"
        int main(void) {
            int ten = 10;
            {}
            int twenty = 10 * 2;
            {{}}
            return ten + twenty;
        }
    "#;
    let expected = r#"
        global function main() { 
            ten.0 = 10
            tmp.0 = 10 * 2
            twenty.1 = tmp.0
            tmp.1 = ten.0 + twenty.1
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_compound_subtract_in_block() {
    let src = r#"
        int main(void) {
            int a = 5;
            if (a > 4) {
                a -= 4;
                int a = 5;
                if (a > 4) {
                    a -= 4;
                }
            }
            return a;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 5
            tmp.0 = a.0 > 4
            if !tmp.0 jump end_if_0
            tmp.1 = a.0 - 4
            a.0 = tmp.1
            a.1 = 5
            tmp.2 = a.1 > 4
            if !tmp.2 jump end_if_2
            tmp.3 = a.1 - 4
            a.1 = tmp.3
        
          end_if_2:
        
          end_if_0:
            return a.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_goto_before_declaration() {
    let src = r#"
        int main(void) {
            int a = 0;
            {
                if (a != 0)
                    return_a:
                        return a;
                int a = 4;
                goto return_a;
            }
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 0
            tmp.0 = a.0 != 0
            if !tmp.0 jump end_if_0
        
          return_a_0:
            return a.0
        
          end_if_0:
            a.1 = 4
            jump return_a_0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_goto_inner_scope() {
    let src = r#"
        int main(void) {
            int x = 5;
            goto inner;
            {
                int x = 0;
                inner:
                x = 1;
                return x;
            }
        }
    "#;
    let expected = r#"
        global function main() { 
            x.0 = 5
            jump inner_0
            x.1 = 0
        
          inner_0:
            x.1 = 1
            return x.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_goto_outer_scope() {
    let src = r#"
        int main(void) {
            int a = 10;
            int b = 0;
            if (a) {
                int a = 1;
                b = a;
                goto end;
            }
            a = 9;
        end:
            return (a == 10 && b == 1);
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 10
            b.1 = 0
            if !a.0 jump end_if_0
            a.2 = 1
            b.1 = a.2
            jump end_0
        
          end_if_0:
            a.0 = 9
        
          end_0:
            tmp.0 = a.0 == 10
            if !tmp.0 jump and_false_2
            tmp.3 = b.1 == 1
            if !tmp.3 jump and_false_2
            tmp.2 = 1
            jump and_end_3
        
          and_false_2:
            tmp.2 = 0
        
          and_end_3:
            return tmp.2
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_goto_sibling_scope() {
    let src = r#"
        int main(void) {
            int sum = 0;
            if (1) {
                int a = 5;
                goto other_if;
                sum = 0;
            first_if:
                a = 5;
                sum = sum + a;
            }
            if (0) {
            other_if:;
                int a = 6;
                sum = sum + a;
                goto first_if;
                sum = 0;
            }
            return sum;
        }
    "#;
    let expected = r#"
        global function main() { 
            sum.0 = 0
            if !1 jump end_if_0
            a.1 = 5
            jump other_if_1
            sum.0 = 0
        
          first_if_0:
            a.1 = 5
            tmp.0 = sum.0 + a.1
            sum.0 = tmp.0
        
          end_if_0:
            if !0 jump end_if_2
        
          other_if_1:
            a.2 = 6
            tmp.1 = sum.0 + a.2
            sum.0 = tmp.1
            jump first_if_0
            sum.0 = 0
        
          end_if_2:
            return sum.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_hidden_then_visible() {
    let src = r#"
        int main(void) {
            int a = 2;
            int b;
            {
                a = -4;
                int a = 7;
                b = a + 1;
            }
            return b == 8 && a == -4;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 2
            tmp.0 = - 4
            a.0 = tmp.0
            a.2 = 7
            tmp.1 = a.2 + 1
            b.1 = tmp.1
            tmp.2 = b.1 == 8
            if !tmp.2 jump and_false_0
            tmp.6 = - 4
            tmp.5 = a.0 == tmp.6
            if !tmp.5 jump and_false_0
            tmp.4 = 1
            jump and_end_1
        
          and_false_0:
            tmp.4 = 0
        
          and_end_1:
            return tmp.4
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_hidden_variable() {
    let src = r#"
        int main(void) {
            int a = 2;
            {
                int a = 1;
                return a;
            }
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 2
            a.1 = 1
            return a.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_inner_uninitialized() {
    let src = r#"
        int main(void) {
            int x = 4;
            {
                int x;
            }
            return x;
        }
    "#;
    let expected = r#"
        global function main() { 
            x.0 = 4
            return x.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_multiple_vars_same_name() {
    let src = r#"
        int main(void) {
            int a = 0;
            {
                int b = 4;
                a = b;
            }
            {
                int b = 2;
                a = a - b;
            }
            return a;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 0
            b.1 = 4
            a.0 = b.1
            b.2 = 2
            tmp.0 = a.0 - b.2
            a.0 = tmp.0
            return a.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_nested_if() {
    let src = r#"
        int main(void) {
            int a = 0;
            if (a) {
                int b = 2;
                return b;
            } else {
                int c = 3;
                if (a < c) {
                    return !a;
                } else {
                    return 5;
                }
            }
            return a;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 0
            if !a.0 jump else_1
            b.1 = 2
            return b.1
            jump end_if_0
        
          else_1:
            c.2 = 3
            tmp.0 = a.0 < c.2
            if !tmp.0 jump else_3
            tmp.1 = ! a.0
            return tmp.1
            jump end_if_2
        
          else_3:
            return 5
        
          end_if_2:
        
          end_if_0:
            return a.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_similar_var_names() {
    let src = r#"
        int main(void) {
            int a;
            int result;
            int a1 = 1;
            {
                int a = 2;
                int a1 = 2;
                {
                    int a;
                    {
                        int a;
                        {
                            int a;
                            {
                                int a;
                                {
                                    int a;
                                    {
                                        int a;
                                        {
                                            int a;
                                            {
                                                int a;
                                                {
                                                    int a = 20;
                                                    result = a;
                                                    {
                                                        int a;
                                                        a = 5;
                                                        result = result + a;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                result = result + a1;
            }
            return result + a1;
        }
    "#;
    let expected = r#"
        global function main() { 
            a1.2 = 1
            a.3 = 2
            a1.4 = 2
            a.13 = 20
            result.1 = a.13
            a.14 = 5
            tmp.0 = result.1 + a.14
            result.1 = tmp.0
            tmp.1 = result.1 + a1.4
            result.1 = tmp.1
            tmp.2 = result.1 + a1.2
            return tmp.2
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_use_in_inner_scope() {
    let src = r#"
        int main(void)
        {
            int x;
            {
                x = 3;
            }
            {
                return x;
            }
        }
    "#;
    let expected = r#"
        global function main() { 
            x.0 = 3
            return x.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}
