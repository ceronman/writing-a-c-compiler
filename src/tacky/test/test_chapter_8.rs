use crate::pretty::{dedent, dump_tacky};

#[test]
fn test_valid_break() {
    let src = r#"
        int main(void) {
            int a = 10;
            int b = 20;
            for (b = -20; b < 0; b = b + 1) {
                a = a - 1;
                if (a <= 0)
                    break;
            }
            return a == 0 && b == -11;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 10
            b.1 = 20
            tmp.0 = - 20
            b.1 = tmp.0
        
          start_loop_0:
            tmp.1 = b.1 < 0
            if !tmp.1 jump break_loop_0
            tmp.2 = a.0 - 1
            a.0 = tmp.2
            tmp.3 = a.0 <= 0
            if !tmp.3 jump end_if_0
            jump break_loop_0
        
          end_if_0:
        
          continue_loop_0:
            tmp.4 = b.1 + 1
            b.1 = tmp.4
            jump start_loop_0
        
          break_loop_0:
            tmp.5 = a.0 == 0
            if !tmp.5 jump and_false_2
            tmp.9 = - 11
            tmp.8 = b.1 == tmp.9
            if !tmp.8 jump and_false_2
            tmp.7 = 1
            jump and_end_3
        
          and_false_2:
            tmp.7 = 0
        
          and_end_3:
            return tmp.7
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_break_immediate() {
    let src = r#"
        int main(void) {
            int a = 10;
            while ((a = 1))
                break;
            return a;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 10
        
          continue_loop_0:
            a.0 = 1
            if !1 jump break_loop_0
            jump break_loop_0
            jump continue_loop_0
        
          break_loop_0:
            return a.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_continue() {
    let src = r#"
        int main(void) {
            int sum = 0;
            int counter;
            for (int i = 0; i <= 10; i = i + 1) {
                counter = i;
                if (i % 2 == 0)
                    continue;
                sum = sum + 1;
            }
            return sum == 5 && counter == 10;
        }
    "#;
    let expected = r#"
        global function main() { 
            sum.0 = 0
            i.2 = 0
        
          start_loop_0:
            tmp.0 = i.2 <= 10
            if !tmp.0 jump break_loop_0
            counter.1 = i.2
            tmp.1 = i.2 % 2
            tmp.2 = tmp.1 == 0
            if !tmp.2 jump end_if_0
            jump continue_loop_0
        
          end_if_0:
            tmp.3 = sum.0 + 1
            sum.0 = tmp.3
        
          continue_loop_0:
            tmp.4 = i.2 + 1
            i.2 = tmp.4
            jump start_loop_0
        
          break_loop_0:
            tmp.5 = sum.0 == 5
            if !tmp.5 jump and_false_2
            tmp.8 = counter.1 == 10
            if !tmp.8 jump and_false_2
            tmp.7 = 1
            jump and_end_3
        
          and_false_2:
            tmp.7 = 0
        
          and_end_3:
            return tmp.7
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_continue_empty_post() {
    let src = r#"
        int main(void) {
            int sum = 0;
            for (int i = 0; i < 10;) {
                i = i + 1;
                if (i % 2)
                    continue;
                sum = sum + i;
            }
            return sum;
        }
    "#;
    let expected = r#"
        global function main() { 
            sum.0 = 0
            i.1 = 0
        
          start_loop_0:
            tmp.0 = i.1 < 10
            if !tmp.0 jump break_loop_0
            tmp.1 = i.1 + 1
            i.1 = tmp.1
            tmp.2 = i.1 % 2
            if !tmp.2 jump end_if_0
            jump continue_loop_0
        
          end_if_0:
            tmp.3 = sum.0 + i.1
            sum.0 = tmp.3
        
          continue_loop_0:
            jump start_loop_0
        
          break_loop_0:
            return sum.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_do_while() {
    let src = r#"
        int main(void) {
            int a = 1;
            do {
                a = a * 2;
            } while(a < 11);
            return a;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 1
        
          start_loop_0:
            tmp.0 = a.0 * 2
            a.0 = tmp.0
        
          continue_loop_0:
            tmp.1 = a.0 < 11
            if tmp.1 jump start_loop_0
        
          break_loop_0:
            return a.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_do_while_break_immediate() {
    let src = r#"
        int main(void) {
            int a = 10;
            do
                break;
            while ((a = 1));
            return a;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 10
        
          start_loop_0:
            jump break_loop_0
        
          continue_loop_0:
            a.0 = 1
            if 1 jump start_loop_0
        
          break_loop_0:
            return a.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_empty_expression() {
    let src = r#"
        int main(void) {
            return 0;;;
        }
    "#;
    let expected = r#"
        global function main() { 
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_empty_loop_body() {
    let src = r#"
        int main(void) {
            int i = 2147;
            do ; while ((i = i - 5) >= 256);
            return i;
        }
    "#;
    let expected = r#"
        global function main() { 
            i.0 = 2147
        
          start_loop_0:
        
          continue_loop_0:
            tmp.0 = i.0 - 5
            i.0 = tmp.0
            tmp.1 = tmp.0 >= 256
            if tmp.1 jump start_loop_0
        
          break_loop_0:
            return i.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_case_block() {
    let src = r#"
        int main(void) {
            int a = 4;
            int b = 0;
            switch(2) {
                case 2: {
                    int a = 8;
                    b = a;
                }
            }
            return (a == 4 && b == 8);
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 4
            b.1 = 0
            tmp.0 = 2 == 2
            if tmp.0 jump switch_0_case__1
            jump break_switch_0
        
          switch_0_case__1:
            a.2 = 8
            b.1 = a.2
        
          break_switch_0:
            tmp.1 = a.0 == 4
            if !tmp.1 jump and_false_0
            tmp.4 = b.1 == 8
            if !tmp.4 jump and_false_0
            tmp.3 = 1
            jump and_end_1
        
          and_false_0:
            tmp.3 = 0
        
          and_end_1:
            return tmp.3
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_compound_assignment_controlling_expression() {
    let src = r#"
        int main(void) {
            int i = 100;
            int sum = 0;
            do sum += 2;
            while (i -= 1);
            return (i == 0 && sum == 200);
        }
    "#;
    let expected = r#"
        global function main() { 
            i.0 = 100
            sum.1 = 0
        
          start_loop_0:
            tmp.0 = sum.1 + 2
            sum.1 = tmp.0
        
          continue_loop_0:
            tmp.1 = i.0 - 1
            i.0 = tmp.1
            if tmp.1 jump start_loop_0
        
          break_loop_0:
            tmp.2 = i.0 == 0
            if !tmp.2 jump and_false_0
            tmp.5 = sum.1 == 200
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
fn test_valid_extra_credit_compound_assignment_for_loop() {
    let src = r#"
        int main(void) {
            int i = 1;
            for (i *= -1; i >= -100; i -=3)
                ;
            return (i == -103);
        }
    "#;
    let expected = r#"
        global function main() { 
            i.0 = 1
            tmp.1 = - 1
            tmp.0 = i.0 * tmp.1
            i.0 = tmp.0
        
          start_loop_0:
            tmp.3 = - 100
            tmp.2 = i.0 >= tmp.3
            if !tmp.2 jump break_loop_0
        
          continue_loop_0:
            tmp.4 = i.0 - 3
            i.0 = tmp.4
            jump start_loop_0
        
          break_loop_0:
            tmp.6 = - 103
            tmp.5 = i.0 == tmp.6
            return tmp.5
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_duffs_device() {
    let src = r#"
        
        int main(void) {
            int count = 37;
            int iterations = (count + 4) / 5;
            switch (count % 5) {
                case 0:
                    do {
                        count = count - 1;
                        case 4:
                            count = count - 1;
                        case 3:
                            count = count - 1;
                        case 2:
                            count = count - 1;
                        case 1:
                            count = count - 1;
                    } while ((iterations = iterations - 1) > 0);
            }
            return (count == 0 && iterations == 0);
        }
    "#;
    let expected = r#"
        global function main() { 
            count.0 = 37
            tmp.0 = count.0 + 4
            tmp.1 = tmp.0 / 5
            iterations.1 = tmp.1
            tmp.2 = count.0 % 5
            tmp.3 = 0 == tmp.2
            if tmp.3 jump switch_0_case__1
            tmp.4 = 4 == tmp.2
            if tmp.4 jump switch_0_case__3
            tmp.5 = 3 == tmp.2
            if tmp.5 jump switch_0_case__4
            tmp.6 = 2 == tmp.2
            if tmp.6 jump switch_0_case__5
            tmp.7 = 1 == tmp.2
            if tmp.7 jump switch_0_case__6
            jump break_switch_0
        
          switch_0_case__1:
        
          start_loop_2:
            tmp.8 = count.0 - 1
            count.0 = tmp.8
        
          switch_0_case__3:
            tmp.9 = count.0 - 1
            count.0 = tmp.9
        
          switch_0_case__4:
            tmp.10 = count.0 - 1
            count.0 = tmp.10
        
          switch_0_case__5:
            tmp.11 = count.0 - 1
            count.0 = tmp.11
        
          switch_0_case__6:
            tmp.12 = count.0 - 1
            count.0 = tmp.12
        
          continue_loop_2:
            tmp.13 = iterations.1 - 1
            iterations.1 = tmp.13
            tmp.14 = tmp.13 > 0
            if tmp.14 jump start_loop_2
        
          break_loop_2:
        
          break_switch_0:
            tmp.15 = count.0 == 0
            if !tmp.15 jump and_false_0
            tmp.18 = iterations.1 == 0
            if !tmp.18 jump and_false_0
            tmp.17 = 1
            jump and_end_1
        
          and_false_0:
            tmp.17 = 0
        
          and_end_1:
            return tmp.17
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_goto_bypass_condition() {
    let src = r#"
        int main(void) {
            int i = 1;
            do {
            while_start:
                i = i + 1;
                if (i < 10)
                    goto while_start;
            } while (0);
            return i;
        }
    "#;
    let expected = r#"
        global function main() { 
            i.0 = 1
        
          start_loop_0:
        
          while_start_1:
            tmp.0 = i.0 + 1
            i.0 = tmp.0
            tmp.1 = i.0 < 10
            if !tmp.1 jump end_if_0
            jump while_start_1
        
          end_if_0:
        
          continue_loop_0:
            if 0 jump start_loop_0
        
          break_loop_0:
            return i.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_goto_bypass_init_exp() {
    let src = r#"
        int main(void) {
            int i = 0;
            goto target;
            for (i = 5; i < 10; i = i + 1)
            target:
                if (i == 0)
                    return 1;
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            i.0 = 0
            jump target_1
            i.0 = 5
        
          start_loop_0:
            tmp.0 = i.0 < 10
            if !tmp.0 jump break_loop_0
        
          target_1:
            tmp.1 = i.0 == 0
            if !tmp.1 jump end_if_0
            return 1
        
          end_if_0:
        
          continue_loop_0:
            tmp.2 = i.0 + 1
            i.0 = tmp.2
            jump start_loop_0
        
          break_loop_0:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_goto_bypass_post_exp() {
    let src = r#"
        int main(void) {
            int sum = 0;
            for (int i = 0;; i = 0) {
            lbl:
                sum = sum + 1;
                i = i + 1;
                if (i > 10)
                    break;
                goto lbl;
            }
            return sum;
        }
    "#;
    let expected = r#"
        global function main() { 
            sum.0 = 0
            i.1 = 0
        
          start_loop_0:
            if !1 jump break_loop_0
        
          lbl_1:
            tmp.0 = sum.0 + 1
            sum.0 = tmp.0
            tmp.1 = i.1 + 1
            i.1 = tmp.1
            tmp.2 = i.1 > 10
            if !tmp.2 jump end_if_0
            jump break_loop_0
        
          end_if_0:
            jump lbl_1
        
          continue_loop_0:
            i.1 = 0
            jump start_loop_0
        
          break_loop_0:
            return sum.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_label_loop_body() {
    let src = r#"
        
        int main(void) {
            int result = 0;
            goto label;
            while (0)
            label: { result = 1; }
            return result;
        }
    "#;
    let expected = r#"
        global function main() { 
            result.0 = 0
            jump label_1
        
          continue_loop_0:
            if !0 jump break_loop_0
        
          label_1:
            result.0 = 1
            jump continue_loop_0
        
          break_loop_0:
            return result.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_label_loops_breaks_and_continues() {
    let src = r#"
        int main(void) {
            int sum = 0;
            goto do_label;
            return 0;
        do_label:
            do {
                sum = 1;
                goto while_label;
            } while (1);
        while_label:
            while (1) {
                sum = sum + 1;
                goto break_label;
                return 0;
            break_label:
                break;
            };
            goto for_label;
            return 0;
        for_label:
            for (int i = 0; i < 10; i = i + 1) {
                sum = sum + 1;
                goto continue_label;
                return 0;
            continue_label:
                continue;
                return 0;
            }
            return sum;
        }
    "#;
    let expected = r#"
        global function main() { 
            sum.0 = 0
            jump do_label_0
            return 0
        
          do_label_0:
        
          start_loop_1:
            sum.0 = 1
            jump while_label_2
        
          continue_loop_1:
            if 1 jump start_loop_1
        
          break_loop_1:
        
          while_label_2:
        
          continue_loop_3:
            if !1 jump break_loop_3
            tmp.0 = sum.0 + 1
            sum.0 = tmp.0
            jump break_label_4
            return 0
        
          break_label_4:
            jump break_loop_3
            jump continue_loop_3
        
          break_loop_3:
            jump for_label_5
            return 0
        
          for_label_5:
            i.1 = 0
        
          start_loop_6:
            tmp.1 = i.1 < 10
            if !tmp.1 jump break_loop_6
            tmp.2 = sum.0 + 1
            sum.0 = tmp.2
            jump continue_label_7
            return 0
        
          continue_label_7:
            jump continue_loop_6
            return 0
        
          continue_loop_6:
            tmp.3 = i.1 + 1
            i.1 = tmp.3
            jump start_loop_6
        
          break_loop_6:
            return sum.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_loop_header_postfix_and_prefix() {
    let src = r#"
        int main(void) {
            int i = 100;
            int count = 0;
            while (i--) count++;
            if (count != 100)
                return 0;
            i = 100;
            count = 0;
            while (--i) count++;
            if (count != 99)
                return 0;
            return 1;
        }
    "#;
    let expected = r#"
        global function main() { 
            i.0 = 100
            count.1 = 0
        
          continue_loop_0:
            tmp.0 = i.0
            tmp.1 = dec i.0
            i.0 = tmp.1
            if !tmp.0 jump break_loop_0
            tmp.2 = count.1
            tmp.3 = inc count.1
            count.1 = tmp.3
            jump continue_loop_0
        
          break_loop_0:
            tmp.4 = count.1 != 100
            if !tmp.4 jump end_if_0
            return 0
        
          end_if_0:
            i.0 = 100
            count.1 = 0
        
          continue_loop_1:
            tmp.5 = dec i.0
            i.0 = tmp.5
            if !tmp.5 jump break_loop_1
            tmp.6 = count.1
            tmp.7 = inc count.1
            count.1 = tmp.7
            jump continue_loop_1
        
          break_loop_1:
            tmp.8 = count.1 != 99
            if !tmp.8 jump end_if_2
            return 0
        
          end_if_2:
            return 1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_loop_in_switch() {
    let src = r#"
        int main(void) {
            int cond = 10;
            switch (cond) {
                case 1:
                    return 0;
                case 10:
                    for (int i = 0; i < 5; i = i + 1) {
                        cond = cond - 1;
                        if (cond == 8)
                            break;
                    }
                    return 123;
                default:
                    return 2;
            }
            return 3;
        }
    "#;
    let expected = r#"
        global function main() { 
            cond.0 = 10
            tmp.0 = 1 == cond.0
            if tmp.0 jump switch_0_case__1
            tmp.1 = 10 == cond.0
            if tmp.1 jump switch_0_case__2
            jump switch_0_default_4
            jump break_switch_0
        
          switch_0_case__1:
            return 0
        
          switch_0_case__2:
            i.1 = 0
        
          start_loop_3:
            tmp.2 = i.1 < 5
            if !tmp.2 jump break_loop_3
            tmp.3 = cond.0 - 1
            cond.0 = tmp.3
            tmp.4 = cond.0 == 8
            if !tmp.4 jump end_if_0
            jump break_loop_3
        
          end_if_0:
        
          continue_loop_3:
            tmp.5 = i.1 + 1
            i.1 = tmp.5
            jump start_loop_3
        
          break_loop_3:
            return 123
        
          switch_0_default_4:
            return 2
        
          break_switch_0:
            return 3
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_post_exp_incr() {
    let src = r#"
        int main(void) {
            int product = 1;
            for (int i = 0; i < 10; i++) {
                product = product + 2;
            }
            return product;
        }
    "#;
    let expected = r#"
        global function main() { 
            product.0 = 1
            i.1 = 0
        
          start_loop_0:
            tmp.0 = i.1 < 10
            if !tmp.0 jump break_loop_0
            tmp.1 = product.0 + 2
            product.0 = tmp.1
        
          continue_loop_0:
            tmp.2 = i.1
            tmp.3 = inc i.1
            i.1 = tmp.3
            jump start_loop_0
        
          break_loop_0:
            return product.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_switch() {
    let src = r#"
        
        int main(void) {
            switch(3) {
                case 0: return 0;
                case 1: return 1;
                case 3: return 3;
                case 5: return 5;
            }
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 0 == 3
            if tmp.0 jump switch_0_case__1
            tmp.1 = 1 == 3
            if tmp.1 jump switch_0_case__2
            tmp.2 = 3 == 3
            if tmp.2 jump switch_0_case__3
            tmp.3 = 5 == 3
            if tmp.3 jump switch_0_case__4
            jump break_switch_0
        
          switch_0_case__1:
            return 0
        
          switch_0_case__2:
            return 1
        
          switch_0_case__3:
            return 3
        
          switch_0_case__4:
            return 5
        
          break_switch_0:
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_switch_assign_in_condition() {
    let src = r#"
        int main(void) {
            int a = 0;
            switch (a = 1) {
                case 0:
                    return 10;
                case 1:
                    a = a * 2;
                    break;
                default:
                    a = 99;
            }
            return a;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 0
            a.0 = 1
            tmp.0 = 0 == 1
            if tmp.0 jump switch_0_case__1
            tmp.1 = 1 == 1
            if tmp.1 jump switch_0_case__2
            jump switch_0_default_3
            jump break_switch_0
        
          switch_0_case__1:
            return 10
        
          switch_0_case__2:
            tmp.2 = a.0 * 2
            a.0 = tmp.2
            jump break_switch_0
        
          switch_0_default_3:
            a.0 = 99
        
          break_switch_0:
            return a.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_switch_break() {
    let src = r#"
        int main(void) {
            int a = 5;
            switch (a) {
                case 5:
                    a = 10;
                    break;
                case 6:
                    a = 0;
                    break;
            }
            return a;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 5
            tmp.0 = 5 == a.0
            if tmp.0 jump switch_0_case__1
            tmp.1 = 6 == a.0
            if tmp.1 jump switch_0_case__2
            jump break_switch_0
        
          switch_0_case__1:
            a.0 = 10
            jump break_switch_0
        
          switch_0_case__2:
            a.0 = 0
            jump break_switch_0
        
          break_switch_0:
            return a.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_switch_decl() {
    let src = r#"
        int main(void) {
            int a = 3;
            int b = 0;
            switch(a) {
                int a = (b = 5);
            case 3:
                a = 4;
                b = b + a;
            }
            return a == 3 && b == 4;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 3
            b.1 = 0
            tmp.0 = 3 == a.0
            if tmp.0 jump switch_0_case__1
            jump break_switch_0
            b.1 = 5
            a.2 = 5
        
          switch_0_case__1:
            a.2 = 4
            tmp.1 = b.1 + a.2
            b.1 = tmp.1
        
          break_switch_0:
            tmp.2 = a.0 == 3
            if !tmp.2 jump and_false_0
            tmp.5 = b.1 == 4
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
fn test_valid_extra_credit_switch_default() {
    let src = r#"
        int main(void) {
            int a = 0;
            switch(a) {
                case 1:
                    return 1;
                case 2:
                    return 9;
                case 4:
                    a = 11;
                    break;
                default:
                    a = 22;
            }
            return a;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 0
            tmp.0 = 1 == a.0
            if tmp.0 jump switch_0_case__1
            tmp.1 = 2 == a.0
            if tmp.1 jump switch_0_case__2
            tmp.2 = 4 == a.0
            if tmp.2 jump switch_0_case__3
            jump switch_0_default_4
            jump break_switch_0
        
          switch_0_case__1:
            return 1
        
          switch_0_case__2:
            return 9
        
          switch_0_case__3:
            a.0 = 11
            jump break_switch_0
        
          switch_0_default_4:
            a.0 = 22
        
          break_switch_0:
            return a.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_switch_default_fallthrough() {
    let src = r#"
        int main(void) {
            int a = 5;
            switch(0) {
                default:
                    a = 0;
                case 1:
                    return a;
            }
            return a + 1;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 5
            tmp.0 = 1 == 0
            if tmp.0 jump switch_0_case__2
            jump switch_0_default_1
            jump break_switch_0
        
          switch_0_default_1:
            a.0 = 0
        
          switch_0_case__2:
            return a.0
        
          break_switch_0:
            tmp.1 = a.0 + 1
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_switch_default_not_last() {
    let src = r#"
        int main(void) {
            int a;
            int b = a = 7;
            switch (a + b) {
                default: return 0;
                case 2: return 1;
            }
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 7
            b.1 = 7
            tmp.0 = a.0 + b.1
            tmp.1 = 2 == tmp.0
            if tmp.1 jump switch_0_case__2
            jump switch_0_default_1
            jump break_switch_0
        
          switch_0_default_1:
            return 0
        
          switch_0_case__2:
            return 1
        
          break_switch_0:
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_switch_default_only() {
    let src = r#"
        int main(void) {
            int a = 1;
            switch(a) default: return 1;
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 1
            jump switch_0_default_1
            jump break_switch_0
        
          switch_0_default_1:
            return 1
        
          break_switch_0:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_switch_empty() {
    let src = r#"
        int main(void) {
            int x = 10;
            switch(x = x + 1) {
            }
            switch(x = x + 1)
            ;
            return x;
        }
    "#;
    let expected = r#"
        global function main() { 
            x.0 = 10
            tmp.0 = x.0 + 1
            x.0 = tmp.0
            jump break_switch_0
        
          break_switch_0:
            tmp.1 = x.0 + 1
            x.0 = tmp.1
            jump break_switch_1
        
          break_switch_1:
            return x.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_switch_fallthrough() {
    let src = r#"
        int main(void) {
            int a = 4;
            int b = 9;
            int c = 0;
            switch (a ? b : 7) {
                case 0:
                    return 5;
                case 7:
                    c = 1;
                case 9:
                    c = 2;
                case 1:
                    c = c + 4;
            }
            return c;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 4
            b.1 = 9
            c.2 = 0
            if !a.0 jump else_1
            tmp.0 = b.1
            jump end_if_0
        
          else_1:
            tmp.0 = 7
        
          end_if_0:
            tmp.1 = 0 == tmp.0
            if tmp.1 jump switch_0_case__1
            tmp.2 = 7 == tmp.0
            if tmp.2 jump switch_0_case__2
            tmp.3 = 9 == tmp.0
            if tmp.3 jump switch_0_case__3
            tmp.4 = 1 == tmp.0
            if tmp.4 jump switch_0_case__4
            jump break_switch_0
        
          switch_0_case__1:
            return 5
        
          switch_0_case__2:
            c.2 = 1
        
          switch_0_case__3:
            c.2 = 2
        
          switch_0_case__4:
            tmp.5 = c.2 + 4
            c.2 = tmp.5
        
          break_switch_0:
            return c.2
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_switch_goto_mid_case() {
    let src = r#"
        int main(void) {
            int a = 0;
            goto mid_case;
            switch (4) {
                case 4:
                    a = 5;
                mid_case:
                    a = a + 1;
                    return a;
            }
            return 100;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 0
            jump mid_case_2
            tmp.0 = 4 == 4
            if tmp.0 jump switch_0_case__1
            jump break_switch_0
        
          switch_0_case__1:
            a.0 = 5
        
          mid_case_2:
            tmp.1 = a.0 + 1
            a.0 = tmp.1
            return a.0
        
          break_switch_0:
            return 100
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_switch_in_loop() {
    let src = r#"
        int main(void) {
            int acc = 0;
            int ctr = 0;
            for (int i = 0; i < 10; i = i + 1) {
                switch(i) {
                    case 0:
                        acc = 2;
                        break;
                    case 1:
                        acc = acc * 3;
                        break;
                    case 2:
                        acc = acc * 4;
                        break;
                    default:
                        acc = acc + 1;
                }
                ctr = ctr + 1;
            }
            return ctr == 10 && acc == 31;
        }
    "#;
    let expected = r#"
        global function main() { 
            acc.0 = 0
            ctr.1 = 0
            i.2 = 0
        
          start_loop_0:
            tmp.0 = i.2 < 10
            if !tmp.0 jump break_loop_0
            tmp.1 = 0 == i.2
            if tmp.1 jump switch_1_case__2
            tmp.2 = 1 == i.2
            if tmp.2 jump switch_1_case__3
            tmp.3 = 2 == i.2
            if tmp.3 jump switch_1_case__4
            jump switch_1_default_5
            jump break_switch_1
        
          switch_1_case__2:
            acc.0 = 2
            jump break_switch_1
        
          switch_1_case__3:
            tmp.4 = acc.0 * 3
            acc.0 = tmp.4
            jump break_switch_1
        
          switch_1_case__4:
            tmp.5 = acc.0 * 4
            acc.0 = tmp.5
            jump break_switch_1
        
          switch_1_default_5:
            tmp.6 = acc.0 + 1
            acc.0 = tmp.6
        
          break_switch_1:
            tmp.7 = ctr.1 + 1
            ctr.1 = tmp.7
        
          continue_loop_0:
            tmp.8 = i.2 + 1
            i.2 = tmp.8
            jump start_loop_0
        
          break_loop_0:
            tmp.9 = ctr.1 == 10
            if !tmp.9 jump and_false_0
            tmp.12 = acc.0 == 31
            if !tmp.12 jump and_false_0
            tmp.11 = 1
            jump and_end_1
        
          and_false_0:
            tmp.11 = 0
        
          and_end_1:
            return tmp.11
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_switch_nested_cases() {
    let src = r#"
        int main(void) {
            int switch1 = 0;
            int switch2 = 0;
            int switch3 = 0;
            switch(3) {
                case 0: return 0;
                case 1: if (0) {
                    case 3: switch1 = 1; break;
                }
                default: return 0;
            }
            switch(4) {
                case 0: return 0;
                if (1) {
                    return 0;
                } else {
                    case 4: switch2 = 1; break;
                }
                default: return 0;
            }
            switch (5) {
                for (int i = 0; i < 10; i = i + 1) {
                    switch1 = 0;
                    case 5: switch3 = 1; break;
                    default: return 0;
                }
            }
            return (switch1 && switch2 && switch3);
        }
    "#;
    let expected = r#"
        global function main() { 
            switch1.0 = 0
            switch2.1 = 0
            switch3.2 = 0
            tmp.0 = 0 == 3
            if tmp.0 jump switch_0_case__1
            tmp.1 = 1 == 3
            if tmp.1 jump switch_0_case__2
            tmp.2 = 3 == 3
            if tmp.2 jump switch_0_case__3
            jump switch_0_default_4
            jump break_switch_0
        
          switch_0_case__1:
            return 0
        
          switch_0_case__2:
            if !0 jump end_if_0
        
          switch_0_case__3:
            switch1.0 = 1
            jump break_switch_0
        
          end_if_0:
        
          switch_0_default_4:
            return 0
        
          break_switch_0:
            tmp.3 = 0 == 4
            if tmp.3 jump switch_5_case__6
            tmp.4 = 4 == 4
            if tmp.4 jump switch_5_case__7
            jump switch_5_default_8
            jump break_switch_5
        
          switch_5_case__6:
            return 0
            if !1 jump else_3
            return 0
            jump end_if_2
        
          else_3:
        
          switch_5_case__7:
            switch2.1 = 1
            jump break_switch_5
        
          end_if_2:
        
          switch_5_default_8:
            return 0
        
          break_switch_5:
            tmp.5 = 5 == 5
            if tmp.5 jump switch_9_case__11
            jump switch_9_default_12
            jump break_switch_9
            i.3 = 0
        
          start_loop_10:
            tmp.6 = i.3 < 10
            if !tmp.6 jump break_loop_10
            switch1.0 = 0
        
          switch_9_case__11:
            switch3.2 = 1
            jump break_loop_10
        
          switch_9_default_12:
            return 0
        
          continue_loop_10:
            tmp.7 = i.3 + 1
            i.3 = tmp.7
            jump start_loop_10
        
          break_loop_10:
        
          break_switch_9:
            if !switch1.0 jump and_false_4
            if !switch2.1 jump and_false_4
            tmp.9 = 1
            jump and_end_5
        
          and_false_4:
            tmp.9 = 0
        
          and_end_5:
            if !tmp.9 jump and_false_6
            if !switch3.2 jump and_false_6
            tmp.11 = 1
            jump and_end_7
        
          and_false_6:
            tmp.11 = 0
        
          and_end_7:
            return tmp.11
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_switch_nested_not_taken() {
    let src = r#"
        
        int main(void) {
            int a = 0;
            switch(a) {
                case 1:
                    switch(a) {
                        case 0: return 0;
                        default: return 0;
                    }
                default: a = 2;
            }
            return a;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 0
            tmp.0 = 1 == a.0
            if tmp.0 jump switch_0_case__1
            jump switch_0_default_5
            jump break_switch_0
        
          switch_0_case__1:
            tmp.1 = 0 == a.0
            if tmp.1 jump switch_2_case__3
            jump switch_2_default_4
            jump break_switch_2
        
          switch_2_case__3:
            return 0
        
          switch_2_default_4:
            return 0
        
          break_switch_2:
        
          switch_0_default_5:
            a.0 = 2
        
          break_switch_0:
            return a.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_switch_nested_switch() {
    let src = r#"
        int main(void){
            switch(3) {
                case 0:
                    return 0;
                case 3: {
                    switch(4) {
                        case 3: return 0;
                        case 4: return 1;
                        default: return 0;
                    }
                }
                case 4: return 0;
                default: return 0;
            }
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 0 == 3
            if tmp.0 jump switch_0_case__1
            tmp.1 = 3 == 3
            if tmp.1 jump switch_0_case__2
            tmp.2 = 4 == 3
            if tmp.2 jump switch_0_case__7
            jump switch_0_default_8
            jump break_switch_0
        
          switch_0_case__1:
            return 0
        
          switch_0_case__2:
            tmp.3 = 3 == 4
            if tmp.3 jump switch_3_case__4
            tmp.4 = 4 == 4
            if tmp.4 jump switch_3_case__5
            jump switch_3_default_6
            jump break_switch_3
        
          switch_3_case__4:
            return 0
        
          switch_3_case__5:
            return 1
        
          switch_3_default_6:
            return 0
        
          break_switch_3:
        
          switch_0_case__7:
            return 0
        
          switch_0_default_8:
            return 0
        
          break_switch_0:
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_switch_no_case() {
    let src = r#"
        int main(void) {
            int a = 4;
            switch(a)
                return 0;
            return a;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 4
            jump break_switch_0
            return 0
        
          break_switch_0:
            return a.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_switch_not_taken() {
    let src = r#"
        int main(void) {
            int a = 1;
            switch(a) {
                case 0: return 0;
                case 2: return 0;
                case 3: return 0;
            }
            return 1;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 1
            tmp.0 = 0 == a.0
            if tmp.0 jump switch_0_case__1
            tmp.1 = 2 == a.0
            if tmp.1 jump switch_0_case__2
            tmp.2 = 3 == a.0
            if tmp.2 jump switch_0_case__3
            jump break_switch_0
        
          switch_0_case__1:
            return 0
        
          switch_0_case__2:
            return 0
        
          switch_0_case__3:
            return 0
        
          break_switch_0:
            return 1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_switch_single_case() {
    let src = r#"
        int main(void) {
            int a = 1;
            switch(a) case 1: return 1;
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 1
            tmp.0 = 1 == a.0
            if tmp.0 jump switch_0_case__1
            jump break_switch_0
        
          switch_0_case__1:
            return 1
        
          break_switch_0:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_switch_with_continue() {
    let src = r#"
        int main(void) {
            switch(4) {
                case 0:
                    return 0;
                case 4: {
                    int acc = 0;
                    for (int i = 0; i < 10; i = i + 1) {
                        if (i % 2)
                            continue;
                        acc = acc + 1;
                    }
                    return acc;
                }
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 0 == 4
            if tmp.0 jump switch_0_case__1
            tmp.1 = 4 == 4
            if tmp.1 jump switch_0_case__2
            jump break_switch_0
        
          switch_0_case__1:
            return 0
        
          switch_0_case__2:
            acc.0 = 0
            i.1 = 0
        
          start_loop_3:
            tmp.2 = i.1 < 10
            if !tmp.2 jump break_loop_3
            tmp.3 = i.1 % 2
            if !tmp.3 jump end_if_0
            jump continue_loop_3
        
          end_if_0:
            tmp.4 = acc.0 + 1
            acc.0 = tmp.4
        
          continue_loop_3:
            tmp.5 = i.1 + 1
            i.1 = tmp.5
            jump start_loop_3
        
          break_loop_3:
            return acc.0
        
          break_switch_0:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_switch_with_continue_2() {
    let src = r#"
        int main(void) {
            int sum = 0;
            for (int i = 0; i < 10; i = i + 1) {
                switch(i % 2) {
                    case 0: continue;
                    default: sum = sum + 1;
                }
            }
            return sum;
        }
    "#;
    let expected = r#"
        global function main() { 
            sum.0 = 0
            i.1 = 0
        
          start_loop_0:
            tmp.0 = i.1 < 10
            if !tmp.0 jump break_loop_0
            tmp.1 = i.1 % 2
            tmp.2 = 0 == tmp.1
            if tmp.2 jump switch_1_case__2
            jump switch_1_default_3
            jump break_switch_1
        
          switch_1_case__2:
            jump continue_loop_0
        
          switch_1_default_3:
            tmp.3 = sum.0 + 1
            sum.0 = tmp.3
        
          break_switch_1:
        
          continue_loop_0:
            tmp.4 = i.1 + 1
            i.1 = tmp.4
            jump start_loop_0
        
          break_loop_0:
            return sum.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_for() {
    let src = r#"
        int main(void) {
            int a = 12345;
            int i;
            for (i = 5; i >= 0; i = i - 1)
                a = a / 3;
            return a;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 12345
            i.1 = 5
        
          start_loop_0:
            tmp.0 = i.1 >= 0
            if !tmp.0 jump break_loop_0
            tmp.1 = a.0 / 3
            a.0 = tmp.1
        
          continue_loop_0:
            tmp.2 = i.1 - 1
            i.1 = tmp.2
            jump start_loop_0
        
          break_loop_0:
            return a.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_for_absent_condition() {
    let src = r#"
        int main(void) {
            for (int i = 400; ; i = i - 100)
                if (i == 100)
                    return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            i.0 = 400
        
          start_loop_0:
            if !1 jump break_loop_0
            tmp.0 = i.0 == 100
            if !tmp.0 jump end_if_0
            return 0
        
          end_if_0:
        
          continue_loop_0:
            tmp.1 = i.0 - 100
            i.0 = tmp.1
            jump start_loop_0
        
          break_loop_0:
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_for_absent_post() {
    let src = r#"
        int main(void) {
            int a = -2147;
            for (; a % 5 != 0;) {
                a = a + 1;
            }
            return a % 5 || a > 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = - 2147
            a.0 = tmp.0
        
          start_loop_0:
            tmp.1 = a.0 % 5
            tmp.2 = tmp.1 != 0
            if !tmp.2 jump break_loop_0
            tmp.3 = a.0 + 1
            a.0 = tmp.3
        
          continue_loop_0:
            jump start_loop_0
        
          break_loop_0:
            tmp.4 = a.0 % 5
            if tmp.4 jump or_true_0
            tmp.7 = a.0 > 0
            if tmp.7 jump or_true_0
            tmp.6 = 0
            jump or_end_1
        
          or_true_0:
            tmp.6 = 1
        
          or_end_1:
            return tmp.6
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_for_decl() {
    let src = r#"
        int main(void) {
            int a = 0;
            for (int i = -100; i <= 0; i = i + 1)
                a = a + 1;
            return a;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 0
            tmp.0 = - 100
            i.1 = tmp.0
        
          start_loop_0:
            tmp.1 = i.1 <= 0
            if !tmp.1 jump break_loop_0
            tmp.2 = a.0 + 1
            a.0 = tmp.2
        
          continue_loop_0:
            tmp.3 = i.1 + 1
            i.1 = tmp.3
            jump start_loop_0
        
          break_loop_0:
            return a.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_for_nested_shadow() {
    let src = r#"
        int main(void) {
            int i = 0;
            int j = 0;
            int k = 1;
            for (int i = 100; i > 0; i = i - 1) {
                int i = 1;
                int j = i + k;
                k = j;
            }
            return k == 101 && i == 0 && j == 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            i.0 = 0
            j.1 = 0
            k.2 = 1
            i.3 = 100
        
          start_loop_0:
            tmp.0 = i.3 > 0
            if !tmp.0 jump break_loop_0
            i.4 = 1
            tmp.1 = i.4 + k.2
            j.5 = tmp.1
            k.2 = j.5
        
          continue_loop_0:
            tmp.2 = i.3 - 1
            i.3 = tmp.2
            jump start_loop_0
        
          break_loop_0:
            tmp.3 = k.2 == 101
            if !tmp.3 jump and_false_0
            tmp.6 = i.0 == 0
            if !tmp.6 jump and_false_0
            tmp.5 = 1
            jump and_end_1
        
          and_false_0:
            tmp.5 = 0
        
          and_end_1:
            if !tmp.5 jump and_false_2
            tmp.9 = j.1 == 0
            if !tmp.9 jump and_false_2
            tmp.8 = 1
            jump and_end_3
        
          and_false_2:
            tmp.8 = 0
        
          and_end_3:
            return tmp.8
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_for_shadow() {
    let src = r#"
        int main(void) {
            int shadow = 1;
            int acc = 0;
            for (int shadow = 0; shadow < 10; shadow = shadow + 1) {
                acc = acc + shadow;
            }
            return acc == 45 && shadow == 1;
        }
    "#;
    let expected = r#"
        global function main() { 
            shadow.0 = 1
            acc.1 = 0
            shadow.2 = 0
        
          start_loop_0:
            tmp.0 = shadow.2 < 10
            if !tmp.0 jump break_loop_0
            tmp.1 = acc.1 + shadow.2
            acc.1 = tmp.1
        
          continue_loop_0:
            tmp.2 = shadow.2 + 1
            shadow.2 = tmp.2
            jump start_loop_0
        
          break_loop_0:
            tmp.3 = acc.1 == 45
            if !tmp.3 jump and_false_0
            tmp.6 = shadow.0 == 1
            if !tmp.6 jump and_false_0
            tmp.5 = 1
            jump and_end_1
        
          and_false_0:
            tmp.5 = 0
        
          and_end_1:
            return tmp.5
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_multi_break() {
    let src = r#"
        int main(void) {
            int i = 0;
            while (1) {
                i = i + 1;
                if (i > 10)
                    break;
            }
            int j = 10;
            while (1) {
                j = j - 1;
                if (j < 0)
                    break;
            }
            int result = j == -1 && i == 11;
            return result;
        }
    "#;
    let expected = r#"
        global function main() { 
            i.0 = 0
        
          continue_loop_0:
            if !1 jump break_loop_0
            tmp.0 = i.0 + 1
            i.0 = tmp.0
            tmp.1 = i.0 > 10
            if !tmp.1 jump end_if_0
            jump break_loop_0
        
          end_if_0:
            jump continue_loop_0
        
          break_loop_0:
            j.1 = 10
        
          continue_loop_1:
            if !1 jump break_loop_1
            tmp.2 = j.1 - 1
            j.1 = tmp.2
            tmp.3 = j.1 < 0
            if !tmp.3 jump end_if_2
            jump break_loop_1
        
          end_if_2:
            jump continue_loop_1
        
          break_loop_1:
            tmp.5 = - 1
            tmp.4 = j.1 == tmp.5
            if !tmp.4 jump and_false_4
            tmp.8 = i.0 == 11
            if !tmp.8 jump and_false_4
            tmp.7 = 1
            jump and_end_5
        
          and_false_4:
            tmp.7 = 0
        
          and_end_5:
            result.2 = tmp.7
            return result.2
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_multi_continue_same_loop() {
    let src = r#"
        int main(void) {
            int x = 10;
            int y = 0;
            int z = 0;
            do {
                z = z + 1;
                if (x <= 0)
                    continue;
                x = x - 1;
                if (y >= 10)
                    continue;
                y = y + 1;
            } while (z != 50);
            return z == 50 && x == 0 && y == 10;
        }
    "#;
    let expected = r#"
        global function main() { 
            x.0 = 10
            y.1 = 0
            z.2 = 0
        
          start_loop_0:
            tmp.0 = z.2 + 1
            z.2 = tmp.0
            tmp.1 = x.0 <= 0
            if !tmp.1 jump end_if_0
            jump continue_loop_0
        
          end_if_0:
            tmp.2 = x.0 - 1
            x.0 = tmp.2
            tmp.3 = y.1 >= 10
            if !tmp.3 jump end_if_2
            jump continue_loop_0
        
          end_if_2:
            tmp.4 = y.1 + 1
            y.1 = tmp.4
        
          continue_loop_0:
            tmp.5 = z.2 != 50
            if tmp.5 jump start_loop_0
        
          break_loop_0:
            tmp.6 = z.2 == 50
            if !tmp.6 jump and_false_4
            tmp.9 = x.0 == 0
            if !tmp.9 jump and_false_4
            tmp.8 = 1
            jump and_end_5
        
          and_false_4:
            tmp.8 = 0
        
          and_end_5:
            if !tmp.8 jump and_false_6
            tmp.12 = y.1 == 10
            if !tmp.12 jump and_false_6
            tmp.11 = 1
            jump and_end_7
        
          and_false_6:
            tmp.11 = 0
        
          and_end_7:
            return tmp.11
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_nested_break() {
    let src = r#"
        int main(void) {
            int ans = 0;
            for (int i = 0; i < 10; i = i + 1)
                for (int j = 0; j < 10; j = j + 1)
                    if ((i / 2)*2 == i)
                        break;
                    else
                        ans = ans + i;
            return ans;
        }
    "#;
    let expected = r#"
        global function main() { 
            ans.0 = 0
            i.1 = 0
        
          start_loop_0:
            tmp.0 = i.1 < 10
            if !tmp.0 jump break_loop_0
            j.2 = 0
        
          start_loop_1:
            tmp.1 = j.2 < 10
            if !tmp.1 jump break_loop_1
            tmp.2 = i.1 / 2
            tmp.3 = tmp.2 * 2
            tmp.4 = tmp.3 == i.1
            if !tmp.4 jump else_1
            jump break_loop_1
            jump end_if_0
        
          else_1:
            tmp.5 = ans.0 + i.1
            ans.0 = tmp.5
        
          end_if_0:
        
          continue_loop_1:
            tmp.6 = j.2 + 1
            j.2 = tmp.6
            jump start_loop_1
        
          break_loop_1:
        
          continue_loop_0:
            tmp.7 = i.1 + 1
            i.1 = tmp.7
            jump start_loop_0
        
          break_loop_0:
            return ans.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_nested_continue() {
    let src = r#"
        int main(void) {
            int x = 5;
            int acc = 0;
            while (x >= 0) {
                int i = x;
                while (i <= 10) {
                    i = i + 1;
                    if (i % 2)
                        continue;
                    acc = acc + 1;
                }
                x = x - 1;
            }
            return acc;
        }
    "#;
    let expected = r#"
        global function main() { 
            x.0 = 5
            acc.1 = 0
        
          continue_loop_0:
            tmp.0 = x.0 >= 0
            if !tmp.0 jump break_loop_0
            i.2 = x.0
        
          continue_loop_1:
            tmp.1 = i.2 <= 10
            if !tmp.1 jump break_loop_1
            tmp.2 = i.2 + 1
            i.2 = tmp.2
            tmp.3 = i.2 % 2
            if !tmp.3 jump end_if_0
            jump continue_loop_1
        
          end_if_0:
            tmp.4 = acc.1 + 1
            acc.1 = tmp.4
            jump continue_loop_1
        
          break_loop_1:
            tmp.5 = x.0 - 1
            x.0 = tmp.5
            jump continue_loop_0
        
          break_loop_0:
            return acc.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_nested_loop() {
    let src = r#"
        int main(void) {
            int acc = 0;
            int x = 100;
            while (x) {
                int y = 10;
                x = x - y;
                while (y) {
                    acc = acc + 1;
                    y = y - 1;
                }
            }
            return acc == 100 && x == 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            acc.0 = 0
            x.1 = 100
        
          continue_loop_0:
            if !x.1 jump break_loop_0
            y.2 = 10
            tmp.0 = x.1 - y.2
            x.1 = tmp.0
        
          continue_loop_1:
            if !y.2 jump break_loop_1
            tmp.1 = acc.0 + 1
            acc.0 = tmp.1
            tmp.2 = y.2 - 1
            y.2 = tmp.2
            jump continue_loop_1
        
          break_loop_1:
            jump continue_loop_0
        
          break_loop_0:
            tmp.3 = acc.0 == 100
            if !tmp.3 jump and_false_0
            tmp.6 = x.1 == 0
            if !tmp.6 jump and_false_0
            tmp.5 = 1
            jump and_end_1
        
          and_false_0:
            tmp.5 = 0
        
          and_end_1:
            return tmp.5
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_null_for_header() {
    let src = r#"
        int main(void) {
            int a = 0;
            for (; ; ) {
                a = a + 1;
                if (a > 3)
                    break;
            }
            return a;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 0
        
          start_loop_0:
            if !1 jump break_loop_0
            tmp.0 = a.0 + 1
            a.0 = tmp.0
            tmp.1 = a.0 > 3
            if !tmp.1 jump end_if_0
            jump break_loop_0
        
          end_if_0:
        
          continue_loop_0:
            jump start_loop_0
        
          break_loop_0:
            return a.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_while() {
    let src = r#"
        int main(void) {
            int a = 0;
            while (a < 5)
                a = a + 2;
            return a;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 0
        
          continue_loop_0:
            tmp.0 = a.0 < 5
            if !tmp.0 jump break_loop_0
            tmp.1 = a.0 + 2
            a.0 = tmp.1
            jump continue_loop_0
        
          break_loop_0:
            return a.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}
