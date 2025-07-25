use crate::pretty::{dedent, dump_tacky};

#[test]
fn test_valid_and_false() {
    let src = r#"
        int main(void) {
            return (10 && 0) + (0 && 4) + (0 && 0);
        }
    "#;
    let expected = r#"
        global function main() { 
            if !10 jump and_false_0
            if !0 jump and_false_0
            tmp.1 = 1
            jump and_end_1
        
          and_false_0:
            tmp.1 = 0
        
          and_end_1:
            if !0 jump and_false_2
            if !4 jump and_false_2
            tmp.4 = 1
            jump and_end_3
        
          and_false_2:
            tmp.4 = 0
        
          and_end_3:
            tmp.2 = tmp.1 + tmp.4
            if !0 jump and_false_4
            if !0 jump and_false_4
            tmp.7 = 1
            jump and_end_5
        
          and_false_4:
            tmp.7 = 0
        
          and_end_5:
            tmp.5 = tmp.2 + tmp.7
            return tmp.5
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_and_short_circuit() {
    let src = r#"
        int main(void) {
            return 0 && (1 / 0);
        }
    "#;
    let expected = r#"
        global function main() { 
            if !0 jump and_false_0
            tmp.2 = 1 / 0
            if !tmp.2 jump and_false_0
            tmp.1 = 1
            jump and_end_1
        
          and_false_0:
            tmp.1 = 0
        
          and_end_1:
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_and_true() {
    let src = r#"
        int main(void) {
            return 1 && -1;
        }
    "#;
    let expected = r#"
        global function main() { 
            if !1 jump and_false_0
            tmp.2 = - 1
            if !tmp.2 jump and_false_0
            tmp.1 = 1
            jump and_end_1
        
          and_false_0:
            tmp.1 = 0
        
          and_end_1:
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_associativity() {
    let src = r#"
        int main(void) {
            return 5 >= 0 > 1 <= 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 5 >= 0
            tmp.1 = tmp.0 > 1
            tmp.2 = tmp.1 <= 0
            return tmp.2
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_compare_arithmetic_results() {
    let src = r#"
        int main(void) {
            return ~2 * -2 == 1 + 5;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = ~ 2
            tmp.2 = - 2
            tmp.1 = tmp.0 * tmp.2
            tmp.4 = 1 + 5
            tmp.3 = tmp.1 == tmp.4
            return tmp.3
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_eq_false() {
    let src = r#"
        int main(void) {
            return 1 == 2;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 1 == 2
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_eq_precedence() {
    let src = r#"
        int main(void) {
            return 3 == 1 != 2;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 3 == 1
            tmp.1 = tmp.0 != 2
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_eq_true() {
    let src = r#"
        int main(void) {
            return 1 == 1;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 1 == 1
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_bitwise_and_precedence() {
    let src = r#"
        int main(void) {
            return 5 & 7 == 5;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.1 = 7 == 5
            tmp.0 = 5 & tmp.1
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_bitwise_or_precedence() {
    let src = r#"
        int main(void) {
            return 5 | 7 != 5;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.1 = 7 != 5
            tmp.0 = 5 | tmp.1
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_bitwise_shift_precedence() {
    let src = r#"
        int main(void) {
            return 20 >> 4 <= 3 << 1;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 20 >> 4
            tmp.2 = 3 << 1
            tmp.1 = tmp.0 <= tmp.2
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_bitwise_xor_precedence() {
    let src = r#"
        int main(void) {
            return 5 ^ 7 < 5;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.1 = 7 < 5
            tmp.0 = 5 ^ tmp.1
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_ge_false() {
    let src = r#"
        int main(void) {
            return 1 >= 2;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 1 >= 2
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_ge_true() {
    let src = r#"
        int main(void) {
            return (1 >= 1) + (1 >= -4);
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 1 >= 1
            tmp.3 = - 4
            tmp.2 = 1 >= tmp.3
            tmp.1 = tmp.0 + tmp.2
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_gt_false() {
    let src = r#"
        int main(void) {
            return (1 > 2) + (1 > 1);
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 1 > 2
            tmp.2 = 1 > 1
            tmp.1 = tmp.0 + tmp.2
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_gt_true() {
    let src = r#"
        int main(void) {
            return 15 > 10;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 15 > 10
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_le_false() {
    let src = r#"
        int main(void) {
            return 1 <= -1;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.1 = - 1
            tmp.0 = 1 <= tmp.1
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_le_true() {
    let src = r#"
        int main(void) {
            return (0 <= 2) + (0 <= 0);
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 0 <= 2
            tmp.2 = 0 <= 0
            tmp.1 = tmp.0 + tmp.2
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_lt_false() {
    let src = r#"
        int main(void) {
            return 2 < 1;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 2 < 1
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_lt_true() {
    let src = r#"
        int main(void) {
            return 1 < 2;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 1 < 2
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_multi_short_circuit() {
    let src = r#"
        int main(void) {
            return 0 || 0 && (1 / 0);
        }
    "#;
    let expected = r#"
        global function main() { 
            if 0 jump or_true_0
            if !0 jump and_false_2
            tmp.4 = 1 / 0
            if !tmp.4 jump and_false_2
            tmp.3 = 1
            jump and_end_3
        
          and_false_2:
            tmp.3 = 0
        
          and_end_3:
            if tmp.3 jump or_true_0
            tmp.1 = 0
            jump or_end_1
        
          or_true_0:
            tmp.1 = 1
        
          or_end_1:
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_ne_false() {
    let src = r#"
        int main(void) {
            return 0 != 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 0 != 0
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_ne_true() {
    let src = r#"
        int main(void) {
            return -1 != -2;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = - 1
            tmp.2 = - 2
            tmp.1 = tmp.0 != tmp.2
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_nested_ops() {
    let src = r#"
        int main(void) {
            return !-3;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = - 3
            tmp.1 = ! tmp.0
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_not() {
    let src = r#"
        int main(void) {
            return !5;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = ! 5
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_not_sum() {
    let src = r#"
        int main(void) {
            return !(4-4);
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 4 - 4
            tmp.1 = ! tmp.0
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_not_sum_2() {
    let src = r#"
        int main(void) {
            return !(3 - 44);
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 3 - 44
            tmp.1 = ! tmp.0
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_not_zero() {
    let src = r#"
        int main(void) {
            return !0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = ! 0
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_operate_on_booleans() {
    let src = r#"
        int main(void) {
            return ~(0 && 1) - -(4 || 3);
        }
    "#;
    let expected = r#"
        global function main() { 
            if !0 jump and_false_0
            if !1 jump and_false_0
            tmp.1 = 1
            jump and_end_1
        
          and_false_0:
            tmp.1 = 0
        
          and_end_1:
            tmp.2 = ~ tmp.1
            if 4 jump or_true_2
            if 3 jump or_true_2
            tmp.5 = 0
            jump or_end_3
        
          or_true_2:
            tmp.5 = 1
        
          or_end_3:
            tmp.6 = - tmp.5
            tmp.3 = tmp.2 - tmp.6
            return tmp.3
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_or_false() {
    let src = r#"
        int main(void) {
            return 0 || 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            if 0 jump or_true_0
            if 0 jump or_true_0
            tmp.1 = 0
            jump or_end_1
        
          or_true_0:
            tmp.1 = 1
        
          or_end_1:
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_or_short_circuit() {
    let src = r#"
        int main(void) {
            return 1 || (1 / 0);
        }
    "#;
    let expected = r#"
        global function main() { 
            if 1 jump or_true_0
            tmp.2 = 1 / 0
            if tmp.2 jump or_true_0
            tmp.1 = 0
            jump or_end_1
        
          or_true_0:
            tmp.1 = 1
        
          or_end_1:
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_or_true() {
    let src = r#"
        int main(void) {
            return (4 || 0) + (0 || 3) + (5 || 5);
        }
    "#;
    let expected = r#"
        global function main() { 
            if 4 jump or_true_0
            if 0 jump or_true_0
            tmp.1 = 0
            jump or_end_1
        
          or_true_0:
            tmp.1 = 1
        
          or_end_1:
            if 0 jump or_true_2
            if 3 jump or_true_2
            tmp.4 = 0
            jump or_end_3
        
          or_true_2:
            tmp.4 = 1
        
          or_end_3:
            tmp.2 = tmp.1 + tmp.4
            if 5 jump or_true_4
            if 5 jump or_true_4
            tmp.7 = 0
            jump or_end_5
        
          or_true_4:
            tmp.7 = 1
        
          or_end_5:
            tmp.5 = tmp.2 + tmp.7
            return tmp.5
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_precedence() {
    let src = r#"
        int main(void) {
            return 1 || 0 && 2;
        }
    "#;
    let expected = r#"
        global function main() { 
            if 1 jump or_true_0
            if !0 jump and_false_2
            if !2 jump and_false_2
            tmp.3 = 1
            jump and_end_3
        
          and_false_2:
            tmp.3 = 0
        
          and_end_3:
            if tmp.3 jump or_true_0
            tmp.1 = 0
            jump or_end_1
        
          or_true_0:
            tmp.1 = 1
        
          or_end_1:
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_precedence_2() {
    let src = r#"
        int main(void) {
            return (1 || 0) && 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            if 1 jump or_true_0
            if 0 jump or_true_0
            tmp.1 = 0
            jump or_end_1
        
          or_true_0:
            tmp.1 = 1
        
          or_end_1:
            if !tmp.1 jump and_false_2
            if !0 jump and_false_2
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
fn test_valid_precedence_3() {
    let src = r#"
        int main(void) {
            return 2 == 2 >= 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.1 = 2 >= 0
            tmp.0 = 2 == tmp.1
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_precedence_4() {
    let src = r#"
        int main(void) {
            return 2 == 2 || 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 2 == 2
            if tmp.0 jump or_true_0
            if 0 jump or_true_0
            tmp.2 = 0
            jump or_end_1
        
          or_true_0:
            tmp.2 = 1
        
          or_end_1:
            return tmp.2
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_precedence_5() {
    let src = r#"
        int main(void) {
            return (0 == 0 && 3 == 2 + 1 > 1) + 1;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 0 == 0
            if !tmp.0 jump and_false_0
            tmp.4 = 2 + 1
            tmp.5 = tmp.4 > 1
            tmp.3 = 3 == tmp.5
            if !tmp.3 jump and_false_0
            tmp.2 = 1
            jump and_end_1
        
          and_false_0:
            tmp.2 = 0
        
          and_end_1:
            tmp.6 = tmp.2 + 1
            return tmp.6
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}
