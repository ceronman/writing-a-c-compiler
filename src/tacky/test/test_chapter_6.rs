use crate::pretty::{dedent, dump_tacky};

#[test]
fn test_valid_assign_ternary() {
    let src = r#"
        int main(void) {
            int a = 0;
            a = 1 ? 2 : 3;
            return a;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 0
            if !1 jump else_1
            tmp.0 = 2
            jump end_if_0
        
          else_1:
            tmp.0 = 3
        
          end_if_0:
            a.0 = tmp.0
            return a.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_binary_condition() {
    let src = r#"
        int main(void) {
            if (1 + 2 == 3)
                return 5;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 1 + 2
            tmp.1 = tmp.0 == 3
            if !tmp.1 jump end_if_0
            return 5
        
          end_if_0:
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_binary_false_condition() {
    let src = r#"
        int main(void) {
            if (1 + 2 == 4)
                return 5;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 1 + 2
            tmp.1 = tmp.0 == 4
            if !tmp.1 jump end_if_0
            return 5
        
          end_if_0:
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_else() {
    let src = r#"
        int main(void) {
            int a = 0;
            if (a)
                return 1;
            else
                return 2;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 0
            if !a.0 jump else_1
            return 1
            jump end_if_0
        
          else_1:
            return 2
        
          end_if_0:
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_bitwise_ternary() {
    let src = r#"
        int main(void) {
            int result;
            1 ^ 1 ? result = 4 : (result = 5);
            return result;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 1 ^ 1
            if !tmp.0 jump else_1
            result.0 = 4
            tmp.1 = 4
            jump end_if_0
        
          else_1:
            result.0 = 5
            tmp.1 = 5
        
          end_if_0:
            return result.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_compound_assign_ternary() {
    let src = r#"
        int main(void) {
            int a = 4;
            a *= 1 ? 2 : 3;
            return a;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 4
            if !1 jump else_1
            tmp.1 = 2
            jump end_if_0
        
          else_1:
            tmp.1 = 3
        
          end_if_0:
            tmp.0 = a.0 * tmp.1
            a.0 = tmp.0
            return a.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_compound_if_expression() {
    let src = r#"
        int main(void) {
            int a = 0;
            if (a += 1)
                return a;
            return 10;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 0
            tmp.0 = a.0 + 1
            a.0 = tmp.0
            if !tmp.0 jump end_if_0
            return a.0
        
          end_if_0:
            return 10
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_goto_after_declaration() {
    let src = r#"
        int main(void) {
            int x = 1;
            goto post_declaration;
            int i = (x = 0);
        post_declaration:
            i = 5;
            return (x == 1 && i == 5);
        }
    "#;
    let expected = r#"
        global function main() { 
            x.0 = 1
            jump post_declaration_0
            x.0 = 0
            i.1 = 0
        
          post_declaration_0:
            i.1 = 5
            tmp.0 = x.0 == 1
            if !tmp.0 jump and_false_0
            tmp.3 = i.1 == 5
            if !tmp.3 jump and_false_0
            tmp.2 = 1
            jump and_end_1
        
          and_false_0:
            tmp.2 = 0
        
          and_end_1:
            return tmp.2
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_goto_backwards() {
    let src = r#"
        int main(void) {
            if (0)
            label:
                return 5;
            goto label;
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            if !0 jump end_if_0
        
          label_0:
            return 5
        
          end_if_0:
            jump label_0
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_goto_label() {
    let src = r#"
        int main(void) {
            goto label;
            return 0;
        label:
            return 1;
        }
    "#;
    let expected = r#"
        global function main() { 
            jump label_0
            return 0
        
          label_0:
            return 1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_goto_label_and_var() {
    let src = r#"
        int main(void) {
            int ident = 5;
            goto ident;
            return 0;
        ident:
            return ident;
        }
    "#;
    let expected = r#"
        global function main() { 
            ident.0 = 5
            jump ident_0
            return 0
        
          ident_0:
            return ident.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_goto_label_main() {
    let src = r#"
        int main(void) {
            goto main;
            return 5;
        main:
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            jump main_0
            return 5
        
          main_0:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_goto_label_main_2() {
    let src = r#"
        int main(void) {
            goto _main;
            return 0;
            _main:
                return 1;
        }
    "#;
    let expected = r#"
        global function main() { 
            jump _main_0
            return 0
        
          _main_0:
            return 1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_goto_nested_label() {
    let src = r#"
        int main(void) {
            goto labelB;
            labelA:
                labelB:
                    return 5;
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            jump labelB_1
        
          labelA_0:
        
          labelB_1:
            return 5
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_label_all_statements() {
    let src = r#"
        int main(void) {
            int a = 1;
        label_if:
            if (a)
                goto label_expression;
            else
                goto label_empty;
        label_goto:
            goto label_return;
            if (0)
            label_expression:
                a = 0;
            goto label_if;
        label_return:
            return a;
        label_empty:;
            a = 100;
            goto label_goto;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 1
        
          label_if_0:
            if !a.0 jump else_1
            jump label_expression_2
            jump end_if_0
        
          else_1:
            jump label_empty_4
        
          end_if_0:
        
          label_goto_1:
            jump label_return_3
            if !0 jump end_if_2
        
          label_expression_2:
            a.0 = 0
        
          end_if_2:
            jump label_if_0
        
          label_return_3:
            return a.0
        
          label_empty_4:
            a.0 = 100
            jump label_goto_1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_label_token() {
    let src = r#"
        int main(void) {
            goto _foo_1_;
            return 0;
        _foo_1_:
            return 1;
        }
    "#;
    let expected = r#"
        global function main() { 
            jump _foo_1__0
            return 0
        
          _foo_1__0:
            return 1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_lh_compound_assignment() {
    let src = r#"
        int main(void) {
            int x = 10;
            (x -= 1) ? (x /= 2) : 0;
            return x == 4;
        }
    "#;
    let expected = r#"
        global function main() { 
            x.0 = 10
            tmp.0 = x.0 - 1
            x.0 = tmp.0
            if !tmp.0 jump else_1
            tmp.2 = x.0 / 2
            x.0 = tmp.2
            tmp.1 = tmp.2
            jump end_if_0
        
          else_1:
            tmp.1 = 0
        
          end_if_0:
            tmp.3 = x.0 == 4
            return tmp.3
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_postfix_if() {
    let src = r#"
        int main(void) {
            int a = 0;
            if (a--)
                return 0;
            else if (a--)
                return 1;
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 0
            tmp.0 = a.0
            tmp.1 = dec a.0
            a.0 = tmp.1
            if !tmp.0 jump else_1
            return 0
            jump end_if_0
        
          else_1:
            tmp.2 = a.0
            tmp.3 = dec a.0
            a.0 = tmp.3
            if !tmp.2 jump end_if_2
            return 1
        
          end_if_2:
        
          end_if_0:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_postfix_in_ternary() {
    let src = r#"
        int main(void) {
            int x = 10;
            x - 10 ? 0 : x--;
            return x;
        }
    "#;
    let expected = r#"
        global function main() { 
            x.0 = 10
            tmp.0 = x.0 - 10
            if !tmp.0 jump else_1
            tmp.1 = 0
            jump end_if_0
        
          else_1:
            tmp.2 = x.0
            tmp.3 = dec x.0
            x.0 = tmp.3
            tmp.1 = tmp.2
        
          end_if_0:
            return x.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_prefix_if() {
    let src = r#"
        int main(void) {
            int a = -1;
            if (++a)
                return 0;
            else if (++a)
                return 1;
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = - 1
            a.0 = tmp.0
            tmp.1 = inc a.0
            a.0 = tmp.1
            if !tmp.1 jump else_1
            return 0
            jump end_if_0
        
          else_1:
            tmp.2 = inc a.0
            a.0 = tmp.2
            if !tmp.2 jump end_if_2
            return 1
        
          end_if_2:
        
          end_if_0:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_prefix_in_ternary() {
    let src = r#"
        int main(void) {
            int a = 0;
            return (++a ? ++a : 0);
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 0
            tmp.0 = inc a.0
            a.0 = tmp.0
            if !tmp.0 jump else_1
            tmp.2 = inc a.0
            a.0 = tmp.2
            tmp.1 = tmp.2
            jump end_if_0
        
          else_1:
            tmp.1 = 0
        
          end_if_0:
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_unused_label() {
    let src = r#"
        int main(void) {
        unused:
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
        
          unused_0:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_whitespace_after_label() {
    let src = r#"
        int main(void) {
            goto label2;
            return 0;
            label1 :
            label2
            :
            return 1;
        }
    "#;
    let expected = r#"
        global function main() { 
            jump label2_1
            return 0
        
          label1_0:
        
          label2_1:
            return 1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_if_nested() {
    let src = r#"
        int main(void) {
            int a = 1;
            int b = 0;
            if (a)
                b = 1;
            else if (b)
                b = 2;
            return b;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 1
            b.1 = 0
            if !a.0 jump else_1
            b.1 = 1
            jump end_if_0
        
          else_1:
            if !b.1 jump end_if_2
            b.1 = 2
        
          end_if_2:
        
          end_if_0:
            return b.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_if_nested_2() {
    let src = r#"
        int main(void) {
            int a = 0;
            int b = 1;
            if (a)
                b = 1;
            else if (~b)
                b = 2;
            return b;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 0
            b.1 = 1
            if !a.0 jump else_1
            b.1 = 1
            jump end_if_0
        
          else_1:
            tmp.0 = ~ b.1
            if !tmp.0 jump end_if_2
            b.1 = 2
        
          end_if_2:
        
          end_if_0:
            return b.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_if_nested_3() {
    let src = r#"
        int main(void) {
            int a = 0;
            if ( (a = 1) )
                if (a == 1)
                    a = 3;
                else
                    a = 4;
            return a;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 0
            a.0 = 1
            if !1 jump end_if_0
            tmp.0 = a.0 == 1
            if !tmp.0 jump else_3
            a.0 = 3
            jump end_if_2
        
          else_3:
            a.0 = 4
        
          end_if_2:
        
          end_if_0:
            return a.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_if_nested_4() {
    let src = r#"
        int main(void) {
            int a = 0;
            if (!a)
                if (3 / 4)
                    a = 3;
                else
                    a = 8 / 2;
            return a;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 0
            tmp.0 = ! a.0
            if !tmp.0 jump end_if_0
            tmp.1 = 3 / 4
            if !tmp.1 jump else_3
            a.0 = 3
            jump end_if_2
        
          else_3:
            tmp.2 = 8 / 2
            a.0 = tmp.2
        
          end_if_2:
        
          end_if_0:
            return a.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_if_nested_5() {
    let src = r#"
        int main(void) {
            int a = 0;
            if (0)
                if (0)
                    a = 3;
                else
                    a = 4;
            else
                a = 1;
            return a;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 0
            if !0 jump else_1
            if !0 jump else_3
            a.0 = 3
            jump end_if_2
        
          else_3:
            a.0 = 4
        
          end_if_2:
            jump end_if_0
        
          else_1:
            a.0 = 1
        
          end_if_0:
            return a.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_if_not_taken() {
    let src = r#"
        int main(void) {
            int a = 0;
            int b = 0;
            if (a)
                b = 1;
            return b;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 0
            b.1 = 0
            if !a.0 jump end_if_0
            b.1 = 1
        
          end_if_0:
            return b.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_if_null_body() {
    let src = r#"
        int main(void) {
            int x = 0;
            if (0)
                ;
            else
                x = 1;
            return x;
        }
    "#;
    let expected = r#"
        global function main() { 
            x.0 = 0
            if !0 jump else_1
            jump end_if_0
        
          else_1:
            x.0 = 1
        
          end_if_0:
            return x.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_if_taken() {
    let src = r#"
        int main(void) {
            int a = 1;
            int b = 0;
            if (a)
                b = 1;
            return b;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 1
            b.1 = 0
            if !a.0 jump end_if_0
            b.1 = 1
        
          end_if_0:
            return b.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_lh_assignment() {
    let src = r#"
        int main(void) {
            int x = 10;
            int y = 0;
            y = (x = 5) ? x : 2;
            return (x == 5 && y == 5);
        }
    "#;
    let expected = r#"
        global function main() { 
            x.0 = 10
            y.1 = 0
            x.0 = 5
            if !5 jump else_1
            tmp.0 = x.0
            jump end_if_0
        
          else_1:
            tmp.0 = 2
        
          end_if_0:
            y.1 = tmp.0
            tmp.1 = x.0 == 5
            if !tmp.1 jump and_false_2
            tmp.4 = y.1 == 5
            if !tmp.4 jump and_false_2
            tmp.3 = 1
            jump and_end_3
        
          and_false_2:
            tmp.3 = 0
        
          and_end_3:
            return tmp.3
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_multiple_if() {
    let src = r#"
        int main(void) {
            int a = 0;
            int b = 0;
            if (a)
                a = 2;
            else
                a = 3;
            if (b)
                b = 4;
            else
                b = 5;
            return a + b;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 0
            b.1 = 0
            if !a.0 jump else_1
            a.0 = 2
            jump end_if_0
        
          else_1:
            a.0 = 3
        
          end_if_0:
            if !b.1 jump else_3
            b.1 = 4
            jump end_if_2
        
          else_3:
            b.1 = 5
        
          end_if_2:
            tmp.0 = a.0 + b.1
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_nested_ternary() {
    let src = r#"
        int main(void) {
            int a = 1;
            int b = 2;
            int flag = 0;
            return a > b ? 5 : flag ? 6 : 7;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 1
            b.1 = 2
            flag.2 = 0
            tmp.0 = a.0 > b.1
            if !tmp.0 jump else_1
            tmp.1 = 5
            jump end_if_0
        
          else_1:
            if !flag.2 jump else_3
            tmp.2 = 6
            jump end_if_2
        
          else_3:
            tmp.2 = 7
        
          end_if_2:
            tmp.1 = tmp.2
        
          end_if_0:
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_nested_ternary_2() {
    let src = r#"
        int main(void) {
            int a = 1 ? 2 ? 3 : 4 : 5;
            int b = 0 ? 2 ? 3 : 4 : 5;
            return a * b;
        }
    "#;
    let expected = r#"
        global function main() { 
            if !1 jump else_1
            if !2 jump else_3
            tmp.1 = 3
            jump end_if_2
        
          else_3:
            tmp.1 = 4
        
          end_if_2:
            tmp.0 = tmp.1
            jump end_if_0
        
          else_1:
            tmp.0 = 5
        
          end_if_0:
            a.0 = tmp.0
            if !0 jump else_5
            if !2 jump else_7
            tmp.3 = 3
            jump end_if_6
        
          else_7:
            tmp.3 = 4
        
          end_if_6:
            tmp.2 = tmp.3
            jump end_if_4
        
          else_5:
            tmp.2 = 5
        
          end_if_4:
            b.1 = tmp.2
            tmp.4 = a.0 * b.1
            return tmp.4
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_rh_assignment() {
    let src = r#"
        int main(void) {
            int flag = 1;
            int a = 0;
            flag ? a = 1 : (a = 0);
            return a;
        }
    "#;
    let expected = r#"
        global function main() { 
            flag.0 = 1
            a.1 = 0
            if !flag.0 jump else_1
            a.1 = 1
            tmp.0 = 1
            jump end_if_0
        
          else_1:
            a.1 = 0
            tmp.0 = 0
        
          end_if_0:
            return a.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_ternary() {
    let src = r#"
        int main(void) {
            int a = 0;
            return a > -1 ? 4 : 5;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 0
            tmp.1 = - 1
            tmp.0 = a.0 > tmp.1
            if !tmp.0 jump else_1
            tmp.2 = 4
            jump end_if_0
        
          else_1:
            tmp.2 = 5
        
          end_if_0:
            return tmp.2
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_ternary_middle_assignment() {
    let src = r#"
        int main(void) {
            int a = 1;
            a != 2 ? a = 2 : 0;
            return a;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 1
            tmp.0 = a.0 != 2
            if !tmp.0 jump else_1
            a.0 = 2
            tmp.1 = 2
            jump end_if_0
        
          else_1:
            tmp.1 = 0
        
          end_if_0:
            return a.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_ternary_middle_binop() {
    let src = r#"
        int main(void) {
            int a = 1 ? 3 % 2 : 4;
            return a;
        }
    "#;
    let expected = r#"
        global function main() { 
            if !1 jump else_1
            tmp.1 = 3 % 2
            tmp.0 = tmp.1
            jump end_if_0
        
          else_1:
            tmp.0 = 4
        
          end_if_0:
            a.0 = tmp.0
            return a.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_ternary_precedence() {
    let src = r#"
        int main(void) {
            int a = 10;
            return a || 0 ? 20 : 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 10
            if a.0 jump or_true_0
            if 0 jump or_true_0
            tmp.1 = 0
            jump or_end_1
        
          or_true_0:
            tmp.1 = 1
        
          or_end_1:
            if !tmp.1 jump else_3
            tmp.2 = 20
            jump end_if_2
        
          else_3:
            tmp.2 = 0
        
          end_if_2:
            return tmp.2
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_ternary_rh_binop() {
    let src = r#"
        int main(void) {
            return 0 ? 1 : 0 || 2;
        }
    "#;
    let expected = r#"
        global function main() { 
            if !0 jump else_1
            tmp.0 = 1
            jump end_if_0
        
          else_1:
            if 0 jump or_true_2
            if 2 jump or_true_2
            tmp.2 = 0
            jump or_end_3
        
          or_true_2:
            tmp.2 = 1
        
          or_end_3:
            tmp.0 = tmp.2
        
          end_if_0:
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_ternary_short_circuit() {
    let src = r#"
        int main(void) {
            int a = 1;
            int b = 0;
            a ? (b = 1) : (b = 2);
            return b;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 1
            b.1 = 0
            if !a.0 jump else_1
            b.1 = 1
            tmp.0 = 1
            jump end_if_0
        
          else_1:
            b.1 = 2
            tmp.0 = 2
        
          end_if_0:
            return b.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_ternary_short_circuit_2() {
    let src = r#"
        int main(void) {
            int a = 0;
            int b = 0;
            a ? (b = 1) : (b = 2);
            return b;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 0
            b.1 = 0
            if !a.0 jump else_1
            b.1 = 1
            tmp.0 = 1
            jump end_if_0
        
          else_1:
            b.1 = 2
            tmp.0 = 2
        
          end_if_0:
            return b.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}
