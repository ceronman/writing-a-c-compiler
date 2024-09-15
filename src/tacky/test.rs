use crate::pretty::{dedent, dump_tacky};

#[test]
fn test_chapter_1_valid_multi_digit() {
    let src = r#"
        int main(void) {
            return 100;
        }
    "#;
    let expected = r#"
        function main { 
          return 100
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_1_valid_newlines() {
    let src = r#"
        int
        main
        (
        void
        )
        {
        return
        0
        ;
        }
    "#;
    let expected = r#"
        function main { 
          return 0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_1_valid_no_newlines() {
    let src = r#"
        int main(void){return 0;}
    "#;
    let expected = r#"
        function main { 
          return 0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_1_valid_return_0() {
    let src = r#"
        int main(void) {
            return 0;
        }
    "#;
    let expected = r#"
        function main { 
          return 0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_1_valid_return_2() {
    let src = r#"
        int main(void) {
            return 2;
        }
    "#;
    let expected = r#"
        function main { 
          return 2
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_1_valid_spaces() {
    let src = r#"
           int main ( void) { return 0 ; }
    "#;
    let expected = r#"
        function main { 
          return 0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_1_valid_tabs() {
    let src = r#"
        int main ( void) { return 0 ; }
    "#;
    let expected = r#"
        function main { 
          return 0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_2_valid_bitwise() {
    let src = r#"
        int main(void) {
            return ~12;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = ~ 12
          return tmp.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_2_valid_bitwise_int_min() {
    let src = r#"
        int main(void) {
            return ~-2147483647;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = - 2147483647
          tmp.1 = ~ tmp.0
          return tmp.1
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_2_valid_bitwise_zero() {
    let src = r#"
        int main(void) {
            return ~0;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = ~ 0
          return tmp.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_2_valid_neg() {
    let src = r#"
        int main(void) {
            return -5;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = - 5
          return tmp.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_2_valid_neg_zero() {
    let src = r#"
        int main(void) {
            return -0;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = - 0
          return tmp.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_2_valid_negate_int_max() {
    let src = r#"
        int main(void) {
            return -2147483647;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = - 2147483647
          return tmp.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_2_valid_nested_ops() {
    let src = r#"
        int main(void) {
            return ~-3;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = - 3
          tmp.1 = ~ tmp.0
          return tmp.1
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_2_valid_nested_ops_2() {
    let src = r#"
        int main(void) {
            return -~0;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = ~ 0
          tmp.1 = - tmp.0
          return tmp.1
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_2_valid_parens() {
    let src = r#"
        int main(void) {
            return (-2);
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = - 2
          return tmp.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_2_valid_parens_2() {
    let src = r#"
        int main(void) {
            return ~(2);
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = ~ 2
          return tmp.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_2_valid_parens_3() {
    let src = r#"
        int main(void) {
            return -(-4);
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = - 4
          tmp.1 = - tmp.0
          return tmp.1
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_2_valid_redundant_parens() {
    let src = r#"
        int main(void)
        {
            return -((((10))));
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = - 10
          return tmp.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_add() {
    let src = r#"
        int main(void) {
            return 1 + 2;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = 1 + 2
          return tmp.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_associativity() {
    let src = r#"
        int main(void) {
            return 1 - 2 - 3;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = 1 - 2
          tmp.1 = tmp.0 - 3
          return tmp.1
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_associativity_2() {
    let src = r#"
        int main(void) {
            return 6 / 3 / 2;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = 6 / 3
          tmp.1 = tmp.0 / 2
          return tmp.1
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_associativity_3() {
    let src = r#"
        int main(void) {
            return (3 / 2 * 4) + (5 - 4 + 3);
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = 3 / 2
          tmp.1 = tmp.0 * 4
          tmp.3 = 5 - 4
          tmp.4 = tmp.3 + 3
          tmp.2 = tmp.1 + tmp.4
          return tmp.2
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_associativity_and_precedence() {
    let src = r#"
        int main(void) {
            return 5 * 4 / 2 -
                3 % (2 + 1);
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = 5 * 4
          tmp.1 = tmp.0 / 2
          tmp.4 = 2 + 1
          tmp.3 = 3 % tmp.4
          tmp.2 = tmp.1 - tmp.3
          return tmp.2
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_div() {
    let src = r#"
        int main(void) {
            return 4 / 2;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = 4 / 2
          return tmp.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_div_neg() {
    let src = r#"
        int main(void) {
            return (-12) / 5;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = - 12
          tmp.1 = tmp.0 / 5
          return tmp.1
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_extra_credit_bitwise_and() {
    let src = r#"
        int main(void) {
            return 3 & 5;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = 3 & 5
          return tmp.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_extra_credit_bitwise_or() {
    let src = r#"
        int main(void) {
            return 1 | 2;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = 1 | 2
          return tmp.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_extra_credit_bitwise_precedence() {
    let src = r#"
        int main(void) {
            return 80 >> 2 | 1 ^ 5 & 7 << 1;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = 80 >> 2
          tmp.4 = 7 << 1
          tmp.3 = 5 & tmp.4
          tmp.2 = 1 ^ tmp.3
          tmp.1 = tmp.0 | tmp.2
          return tmp.1
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_extra_credit_bitwise_shift_associativity() {
    let src = r#"
        int main(void) {
            return 33 << 4 >> 2;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = 33 << 4
          tmp.1 = tmp.0 >> 2
          return tmp.1
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_extra_credit_bitwise_shift_associativity_2() {
    let src = r#"
        int main(void) {
            return 33 >> 2 << 1;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = 33 >> 2
          tmp.1 = tmp.0 << 1
          return tmp.1
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_extra_credit_bitwise_shift_precedence() {
    let src = r#"
        int main(void) {
            return 40 << 4 + 12 >> 1;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.1 = 4 + 12
          tmp.0 = 40 << tmp.1
          tmp.2 = tmp.0 >> 1
          return tmp.2
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_extra_credit_bitwise_shiftl() {
    let src = r#"
        int main(void) {
            return 35 << 2;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = 35 << 2
          return tmp.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_extra_credit_bitwise_shiftr() {
    let src = r#"
        int main(void) {
            return 1000 >> 4;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = 1000 >> 4
          return tmp.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_extra_credit_bitwise_shiftr_negative() {
    let src = r#"
        int main(void) {
            return -5 >> 30;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = - 5
          tmp.1 = tmp.0 >> 30
          return tmp.1
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_extra_credit_bitwise_variable_shift_count() {
    let src = r#"
        int main(void) {
            return (4 << (2 * 2)) + (100 >> (1 + 2));
        }
    "#;
    let expected = r#"
        function main { 
          tmp.1 = 2 * 2
          tmp.0 = 4 << tmp.1
          tmp.4 = 1 + 2
          tmp.3 = 100 >> tmp.4
          tmp.2 = tmp.0 + tmp.3
          return tmp.2
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_extra_credit_bitwise_xor() {
    let src = r#"
        int main(void) {
            return 7 ^ 1;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = 7 ^ 1
          return tmp.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_mod() {
    let src = r#"
        int main(void) {
            return 4 % 2;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = 4 % 2
          return tmp.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_mult() {
    let src = r#"
        int main(void) {
            return 2 * 3;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = 2 * 3
          return tmp.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_parens() {
    let src = r#"
        int main(void) {
            return 2 * (3 + 4);
        }
    "#;
    let expected = r#"
        function main { 
          tmp.1 = 3 + 4
          tmp.0 = 2 * tmp.1
          return tmp.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_precedence() {
    let src = r#"
        int main(void) {
            return 2 + 3 * 4;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.1 = 3 * 4
          tmp.0 = 2 + tmp.1
          return tmp.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_sub() {
    let src = r#"
        int main(void) {
            return 1 - 2;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = 1 - 2
          return tmp.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_sub_neg() {
    let src = r#"
        int main(void) {
            return 2- -1;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.1 = - 1
          tmp.0 = 2 - tmp.1
          return tmp.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_unop_add() {
    let src = r#"
        int main(void) {
            return ~2 + 3;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = ~ 2
          tmp.1 = tmp.0 + 3
          return tmp.1
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_3_valid_unop_parens() {
    let src = r#"
        int main(void) {
            return ~(1 + 1);
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = 1 + 1
          tmp.1 = ~ tmp.0
          return tmp.1
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_and_false() {
    let src = r#"
        int main(void) {
            return (10 && 0) + (0 && 4) + (0 && 0);
        }
    "#;
    let expected = r#"
        function main { 
          jump and_false__0 if !10
          jump and_false__0 if !0
          tmp.1 = 1
          jump and_end__1
        and_false__0:
          tmp.1 = 0
        and_end__1:
          jump and_false__2 if !0
          jump and_false__2 if !4
          tmp.4 = 1
          jump and_end__3
        and_false__2:
          tmp.4 = 0
        and_end__3:
          tmp.2 = tmp.1 + tmp.4
          jump and_false__4 if !0
          jump and_false__4 if !0
          tmp.7 = 1
          jump and_end__5
        and_false__4:
          tmp.7 = 0
        and_end__5:
          tmp.5 = tmp.2 + tmp.7
          return tmp.5
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_and_short_circuit() {
    let src = r#"
        int main(void) {
            return 0 && (1 / 0);
        }
    "#;
    let expected = r#"
        function main { 
          jump and_false__0 if !0
          tmp.2 = 1 / 0
          jump and_false__0 if !tmp.2
          tmp.1 = 1
          jump and_end__1
        and_false__0:
          tmp.1 = 0
        and_end__1:
          return tmp.1
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_and_true() {
    let src = r#"
        int main(void) {
            return 1 && -1;
        }
    "#;
    let expected = r#"
        function main { 
          jump and_false__0 if !1
          tmp.2 = - 1
          jump and_false__0 if !tmp.2
          tmp.1 = 1
          jump and_end__1
        and_false__0:
          tmp.1 = 0
        and_end__1:
          return tmp.1
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_associativity() {
    let src = r#"
        int main(void) {
            return 5 >= 0 > 1 <= 0;
        }
    "#;
    let expected = r#"
        function main { 
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
fn test_chapter_4_valid_compare_arithmetic_results() {
    let src = r#"
        int main(void) {
            return ~2 * -2 == 1 + 5;
        }
    "#;
    let expected = r#"
        function main { 
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
fn test_chapter_4_valid_eq_false() {
    let src = r#"
        int main(void) {
            return 1 == 2;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = 1 == 2
          return tmp.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_eq_precedence() {
    let src = r#"
        int main(void) {
            return 3 == 1 != 2;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = 3 == 1
          tmp.1 = tmp.0 != 2
          return tmp.1
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_eq_true() {
    let src = r#"
        int main(void) {
            return 1 == 1;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = 1 == 1
          return tmp.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_extra_credit_bitwise_and_precedence() {
    let src = r#"
        int main(void) {
            return 5 & 7 == 5;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.1 = 7 == 5
          tmp.0 = 5 & tmp.1
          return tmp.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_extra_credit_bitwise_or_precedence() {
    let src = r#"
        int main(void) {
            return 5 | 7 != 5;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.1 = 7 != 5
          tmp.0 = 5 | tmp.1
          return tmp.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_extra_credit_bitwise_shift_precedence() {
    let src = r#"
        int main(void) {
            return 20 >> 4 <= 3 << 1;
        }
    "#;
    let expected = r#"
        function main { 
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
fn test_chapter_4_valid_extra_credit_bitwise_xor_precedence() {
    let src = r#"
        int main(void) {
            return 5 ^ 7 < 5;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.1 = 7 < 5
          tmp.0 = 5 ^ tmp.1
          return tmp.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_ge_false() {
    let src = r#"
        int main(void) {
            return 1 >= 2;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = 1 >= 2
          return tmp.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_ge_true() {
    let src = r#"
        int main(void) {
            return (1 >= 1) + (1 >= -4);
        }
    "#;
    let expected = r#"
        function main { 
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
fn test_chapter_4_valid_gt_false() {
    let src = r#"
        int main(void) {
            return (1 > 2) + (1 > 1);
        }
    "#;
    let expected = r#"
        function main { 
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
fn test_chapter_4_valid_gt_true() {
    let src = r#"
        int main(void) {
            return 15 > 10;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = 15 > 10
          return tmp.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_le_false() {
    let src = r#"
        int main(void) {
            return 1 <= -1;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.1 = - 1
          tmp.0 = 1 <= tmp.1
          return tmp.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_le_true() {
    let src = r#"
        int main(void) {
            return (0 <= 2) + (0 <= 0);
        }
    "#;
    let expected = r#"
        function main { 
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
fn test_chapter_4_valid_lt_false() {
    let src = r#"
        int main(void) {
            return 2 < 1;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = 2 < 1
          return tmp.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_lt_true() {
    let src = r#"
        int main(void) {
            return 1 < 2;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = 1 < 2
          return tmp.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_multi_short_circuit() {
    let src = r#"
        int main(void) {
            return 0 || 0 && (1 / 0);
        }
    "#;
    let expected = r#"
        function main { 
          jump or_true__0if 0
          jump and_false__2 if !0
          tmp.4 = 1 / 0
          jump and_false__2 if !tmp.4
          tmp.3 = 1
          jump and_end__3
        and_false__2:
          tmp.3 = 0
        and_end__3:
          jump or_true__0if tmp.3
          tmp.1 = 0
          jump or_end__1
        or_true__0:
          tmp.1 = 1
        or_end__1:
          return tmp.1
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_ne_false() {
    let src = r#"
        int main(void) {
            return 0 != 0;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = 0 != 0
          return tmp.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_ne_true() {
    let src = r#"
        int main(void) {
            return -1 != -2;
        }
    "#;
    let expected = r#"
        function main { 
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
fn test_chapter_4_valid_nested_ops() {
    let src = r#"
        int main(void) {
            return !-3;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = - 3
          tmp.1 = ! tmp.0
          return tmp.1
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_not() {
    let src = r#"
        int main(void) {
            return !5;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = ! 5
          return tmp.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_not_sum() {
    let src = r#"
        int main(void) {
            return !(4-4);
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = 4 - 4
          tmp.1 = ! tmp.0
          return tmp.1
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_not_sum_2() {
    let src = r#"
        int main(void) {
            return !(3 - 44);
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = 3 - 44
          tmp.1 = ! tmp.0
          return tmp.1
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_not_zero() {
    let src = r#"
        int main(void) {
            return !0;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = ! 0
          return tmp.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_operate_on_booleans() {
    let src = r#"
        int main(void) {
            return ~(0 && 1) - -(4 || 3);
        }
    "#;
    let expected = r#"
        function main { 
          jump and_false__0 if !0
          jump and_false__0 if !1
          tmp.1 = 1
          jump and_end__1
        and_false__0:
          tmp.1 = 0
        and_end__1:
          tmp.2 = ~ tmp.1
          jump or_true__2if 4
          jump or_true__2if 3
          tmp.5 = 0
          jump or_end__3
        or_true__2:
          tmp.5 = 1
        or_end__3:
          tmp.6 = - tmp.5
          tmp.3 = tmp.2 - tmp.6
          return tmp.3
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_or_false() {
    let src = r#"
        int main(void) {
            return 0 || 0;
        }
    "#;
    let expected = r#"
        function main { 
          jump or_true__0if 0
          jump or_true__0if 0
          tmp.1 = 0
          jump or_end__1
        or_true__0:
          tmp.1 = 1
        or_end__1:
          return tmp.1
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_or_short_circuit() {
    let src = r#"
        int main(void) {
            return 1 || (1 / 0);
        }
    "#;
    let expected = r#"
        function main { 
          jump or_true__0if 1
          tmp.2 = 1 / 0
          jump or_true__0if tmp.2
          tmp.1 = 0
          jump or_end__1
        or_true__0:
          tmp.1 = 1
        or_end__1:
          return tmp.1
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_or_true() {
    let src = r#"
        int main(void) {
            return (4 || 0) + (0 || 3) + (5 || 5);
        }
    "#;
    let expected = r#"
        function main { 
          jump or_true__0if 4
          jump or_true__0if 0
          tmp.1 = 0
          jump or_end__1
        or_true__0:
          tmp.1 = 1
        or_end__1:
          jump or_true__2if 0
          jump or_true__2if 3
          tmp.4 = 0
          jump or_end__3
        or_true__2:
          tmp.4 = 1
        or_end__3:
          tmp.2 = tmp.1 + tmp.4
          jump or_true__4if 5
          jump or_true__4if 5
          tmp.7 = 0
          jump or_end__5
        or_true__4:
          tmp.7 = 1
        or_end__5:
          tmp.5 = tmp.2 + tmp.7
          return tmp.5
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_precedence() {
    let src = r#"
        int main(void) {
            return 1 || 0 && 2;
        }
    "#;
    let expected = r#"
        function main { 
          jump or_true__0if 1
          jump and_false__2 if !0
          jump and_false__2 if !2
          tmp.3 = 1
          jump and_end__3
        and_false__2:
          tmp.3 = 0
        and_end__3:
          jump or_true__0if tmp.3
          tmp.1 = 0
          jump or_end__1
        or_true__0:
          tmp.1 = 1
        or_end__1:
          return tmp.1
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_precedence_2() {
    let src = r#"
        int main(void) {
            return (1 || 0) && 0;
        }
    "#;
    let expected = r#"
        function main { 
          jump or_true__0if 1
          jump or_true__0if 0
          tmp.1 = 0
          jump or_end__1
        or_true__0:
          tmp.1 = 1
        or_end__1:
          jump and_false__2 if !tmp.1
          jump and_false__2 if !0
          tmp.3 = 1
          jump and_end__3
        and_false__2:
          tmp.3 = 0
        and_end__3:
          return tmp.3
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_precedence_3() {
    let src = r#"
        int main(void) {
            return 2 == 2 >= 0;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.1 = 2 >= 0
          tmp.0 = 2 == tmp.1
          return tmp.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_precedence_4() {
    let src = r#"
        int main(void) {
            return 2 == 2 || 0;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = 2 == 2
          jump or_true__0if tmp.0
          jump or_true__0if 0
          tmp.2 = 0
          jump or_end__1
        or_true__0:
          tmp.2 = 1
        or_end__1:
          return tmp.2
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_4_valid_precedence_5() {
    let src = r#"
        int main(void) {
            return (0 == 0 && 3 == 2 + 1 > 1) + 1;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = 0 == 0
          jump and_false__0 if !tmp.0
          tmp.4 = 2 + 1
          tmp.5 = tmp.4 > 1
          tmp.3 = 3 == tmp.5
          jump and_false__0 if !tmp.3
          tmp.2 = 1
          jump and_end__1
        and_false__0:
          tmp.2 = 0
        and_end__1:
          tmp.6 = tmp.2 + 1
          return tmp.6
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_add_variables() {
    let src = r#"
        int main(void) {
            int first_variable = 1;
            int second_variable = 2;
            return first_variable + second_variable;
        }
    "#;
    let expected = r#"
        function main { 
          first_variable.0 = 1
          second_variable.1 = 2
          tmp.0 = first_variable.0 + second_variable.1
          return tmp.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_allocate_temps_and_vars() {
    let src = r#"
        int main(void) {
            int a = 2147483646;
            int b = 0;
            int c = a / 6 + !b;
            return c * 2 == a - 1431655762;
        }
    "#;
    let expected = r#"
        function main { 
          a.0 = 2147483646
          b.1 = 0
          tmp.0 = a.0 / 6
          tmp.2 = ! b.1
          tmp.1 = tmp.0 + tmp.2
          c.2 = tmp.1
          tmp.3 = c.2 * 2
          tmp.5 = a.0 - 1431655762
          tmp.4 = tmp.3 == tmp.5
          return tmp.4
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_assign() {
    let src = r#"
        int main(void) {
            int var0;
            var0 = 2;
            return var0;
        }
    "#;
    let expected = r#"
        function main { 
          var0.0 = 2
          return var0.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_assign_val_in_initializer() {
    let src = r#"
        int main(void) {
            int a = a = 5;
            return a;
        }
    "#;
    let expected = r#"
        function main { 
          a.0 = 5
          a.0 = a.0
          return a.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_assignment_in_initializer() {
    let src = r#"
        int main(void) {
            int a;
            int b = a = 0;
            return b;
        }
    "#;
    let expected = r#"
        function main { 
          a.0 = 0
          b.1 = a.0
          return b.1
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_assignment_lowest_precedence() {
    let src = r#"
        int main(void) {
            int a;
            a = 0 || 5;
            return a;
        }
    "#;
    let expected = r#"
        function main { 
          jump or_true__0if 0
          jump or_true__0if 5
          tmp.1 = 0
          jump or_end__1
        or_true__0:
          tmp.1 = 1
        or_end__1:
          a.0 = tmp.1
          return a.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_empty_function_body() {
    let src = r#"
        int main(void) {
        }
    "#;
    let expected = r#"
        function main { 
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_exp_then_declaration() {
    let src = r#"
        int main(void) {
            int a = -2593;
            a = a % 3;
            int b = -a;
            return b;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = - 2593
          a.0 = tmp.0
          tmp.1 = a.0 % 3
          a.0 = tmp.1
          tmp.2 = - a.0
          b.1 = tmp.2
          return b.1
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_extra_credit_bitwise_in_initializer() {
    let src = r#"
        int main(void) {
            int a = 15;
            int b = a ^ 5;
            return 1 | b;
        }
    "#;
    let expected = r#"
        function main { 
          a.0 = 15
          tmp.0 = a.0 ^ 5
          b.1 = tmp.0
          tmp.1 = 1 | b.1
          return tmp.1
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_extra_credit_bitwise_ops_vars() {
    let src = r#"
        int main(void) {
            int a = 3;
            int b = 5;
            int c = 8;
            return a & b | c;
        }
    "#;
    let expected = r#"
        function main { 
          a.0 = 3
          b.1 = 5
          c.2 = 8
          tmp.0 = a.0 & b.1
          tmp.1 = tmp.0 | c.2
          return tmp.1
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_extra_credit_bitwise_shiftl_variable() {
    let src = r#"
        int main(void) {
            int x = 3;
            return x << 3;
        }
    "#;
    let expected = r#"
        function main { 
          x.0 = 3
          tmp.0 = x.0 << 3
          return tmp.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_extra_credit_bitwise_shiftr_assign() {
    let src = r#"
        int main(void) {
            int var_to_shift = 1234;
            int x = 0;
            x = var_to_shift >> 4;
            return x;
        }
    "#;
    let expected = r#"
        function main { 
          var_to_shift.0 = 1234
          x.1 = 0
          tmp.0 = var_to_shift.0 >> 4
          x.1 = tmp.0
          return x.1
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_extra_credit_compound_assignment_chained() {
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
        function main { 
          a.0 = 250
          b.1 = 200
          c.2 = 100
          d.3 = 75
          tmp.0 = - 25
          e.4 = tmp.0
          f.5 = 0
          x.6 = 0
          tmp.6 = - 7
          f.5 = tmp.6
          tmp.5 = e.4 % f.5
          e.4 = tmp.5
          tmp.4 = d.3 / e.4
          d.3 = tmp.4
          tmp.3 = c.2 * d.3
          c.2 = tmp.3
          tmp.2 = b.1 - c.2
          b.1 = tmp.2
          tmp.1 = a.0 + b.1
          a.0 = tmp.1
          x.6 = a.0
          tmp.7 = a.0 == 2250
          jump and_false__0 if !tmp.7
          tmp.10 = b.1 == 2000
          jump and_false__0 if !tmp.10
          tmp.9 = 1
          jump and_end__1
        and_false__0:
          tmp.9 = 0
        and_end__1:
          jump and_false__2 if !tmp.9
          tmp.14 = - 1800
          tmp.13 = c.2 == tmp.14
          jump and_false__2 if !tmp.13
          tmp.12 = 1
          jump and_end__3
        and_false__2:
          tmp.12 = 0
        and_end__3:
          jump and_false__4 if !tmp.12
          tmp.18 = - 18
          tmp.17 = d.3 == tmp.18
          jump and_false__4 if !tmp.17
          tmp.16 = 1
          jump and_end__5
        and_false__4:
          tmp.16 = 0
        and_end__5:
          jump and_false__6 if !tmp.16
          tmp.22 = - 4
          tmp.21 = e.4 == tmp.22
          jump and_false__6 if !tmp.21
          tmp.20 = 1
          jump and_end__7
        and_false__6:
          tmp.20 = 0
        and_end__7:
          jump and_false__8 if !tmp.20
          tmp.26 = - 7
          tmp.25 = f.5 == tmp.26
          jump and_false__8 if !tmp.25
          tmp.24 = 1
          jump and_end__9
        and_false__8:
          tmp.24 = 0
        and_end__9:
          jump and_false__10 if !tmp.24
          tmp.29 = x.6 == 2250
          jump and_false__10 if !tmp.29
          tmp.28 = 1
          jump and_end__11
        and_false__10:
          tmp.28 = 0
        and_end__11:
          return tmp.28
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_extra_credit_compound_assignment_lowest_precedence() {
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
        function main { 
          a.0 = 10
          b.1 = 12
          jump or_true__0if 0
          jump or_true__0if b.1
          tmp.2 = 0
          jump or_end__1
        or_true__0:
          tmp.2 = 1
        or_end__1:
          tmp.0 = a.0 + tmp.2
          a.0 = tmp.0
          jump and_false__2 if !a.0
          jump and_false__2 if !0
          tmp.5 = 1
          jump and_end__3
        and_false__2:
          tmp.5 = 0
        and_end__3:
          tmp.3 = b.1 * tmp.5
          b.1 = tmp.3
          c.2 = 14
          jump or_true__4if a.0
          jump or_true__4if b.1
          tmp.8 = 0
          jump or_end__5
        or_true__4:
          tmp.8 = 1
        or_end__5:
          tmp.6 = c.2 - tmp.8
          c.2 = tmp.6
          d.3 = 16
          jump or_true__6if c.2
          jump or_true__6if d.3
          tmp.11 = 0
          jump or_end__7
        or_true__6:
          tmp.11 = 1
        or_end__7:
          tmp.9 = d.3 / tmp.11
          d.3 = tmp.9
          tmp.12 = a.0 == 11
          jump and_false__8 if !tmp.12
          tmp.15 = b.1 == 0
          jump and_false__8 if !tmp.15
          tmp.14 = 1
          jump and_end__9
        and_false__8:
          tmp.14 = 0
        and_end__9:
          jump and_false__10 if !tmp.14
          tmp.18 = c.2 == 13
          jump and_false__10 if !tmp.18
          tmp.17 = 1
          jump and_end__11
        and_false__10:
          tmp.17 = 0
        and_end__11:
          jump and_false__12 if !tmp.17
          tmp.21 = d.3 == 16
          jump and_false__12 if !tmp.21
          tmp.20 = 1
          jump and_end__13
        and_false__12:
          tmp.20 = 0
        and_end__13:
          return tmp.20
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_extra_credit_compound_assignment_use_result() {
    let src = r#"
        int main(void) {
            int x = 1;
            int y = x += 3;
            return (x == 4 && y == 4);
        }
    "#;
    let expected = r#"
        function main { 
          x.0 = 1
          tmp.0 = x.0 + 3
          x.0 = tmp.0
          y.1 = x.0
          tmp.1 = x.0 == 4
          jump and_false__0 if !tmp.1
          tmp.4 = y.1 == 4
          jump and_false__0 if !tmp.4
          tmp.3 = 1
          jump and_end__1
        and_false__0:
          tmp.3 = 0
        and_end__1:
          return tmp.3
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_extra_credit_compound_bitwise_and() {
    let src = r#"
        int main(void) {
            int to_and = 3;
            to_and &= 6;
            return to_and;
        }
    "#;
    let expected = r#"
        function main { 
          to_and.0 = 3
          tmp.0 = to_and.0 & 6
          to_and.0 = tmp.0
          return to_and.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_extra_credit_compound_bitwise_assignment_lowest_precedence() {
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
        function main { 
          a.0 = 11
          b.1 = 12
          jump or_true__0if 0
          jump or_true__0if b.1
          tmp.2 = 0
          jump or_end__1
        or_true__0:
          tmp.2 = 1
        or_end__1:
          tmp.0 = a.0 & tmp.2
          a.0 = tmp.0
          jump or_true__2if a.0
          jump or_true__2if 1
          tmp.5 = 0
          jump or_end__3
        or_true__2:
          tmp.5 = 1
        or_end__3:
          tmp.3 = b.1 ^ tmp.5
          b.1 = tmp.3
          c.2 = 14
          jump or_true__4if a.0
          jump or_true__4if b.1
          tmp.8 = 0
          jump or_end__5
        or_true__4:
          tmp.8 = 1
        or_end__5:
          tmp.6 = c.2 | tmp.8
          c.2 = tmp.6
          d.3 = 16
          jump or_true__6if c.2
          jump or_true__6if d.3
          tmp.11 = 0
          jump or_end__7
        or_true__6:
          tmp.11 = 1
        or_end__7:
          tmp.9 = d.3 >> tmp.11
          d.3 = tmp.9
          e.4 = 18
          jump or_true__8if c.2
          jump or_true__8if d.3
          tmp.14 = 0
          jump or_end__9
        or_true__8:
          tmp.14 = 1
        or_end__9:
          tmp.12 = e.4 << tmp.14
          e.4 = tmp.12
          tmp.15 = a.0 == 1
          jump and_false__10 if !tmp.15
          tmp.18 = b.1 == 13
          jump and_false__10 if !tmp.18
          tmp.17 = 1
          jump and_end__11
        and_false__10:
          tmp.17 = 0
        and_end__11:
          jump and_false__12 if !tmp.17
          tmp.21 = c.2 == 15
          jump and_false__12 if !tmp.21
          tmp.20 = 1
          jump and_end__13
        and_false__12:
          tmp.20 = 0
        and_end__13:
          jump and_false__14 if !tmp.20
          tmp.24 = d.3 == 8
          jump and_false__14 if !tmp.24
          tmp.23 = 1
          jump and_end__15
        and_false__14:
          tmp.23 = 0
        and_end__15:
          jump and_false__16 if !tmp.23
          tmp.27 = e.4 == 36
          jump and_false__16 if !tmp.27
          tmp.26 = 1
          jump and_end__17
        and_false__16:
          tmp.26 = 0
        and_end__17:
          return tmp.26
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_extra_credit_compound_bitwise_chained() {
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
        function main { 
          a.0 = 250
          b.1 = 200
          c.2 = 100
          d.3 = 75
          e.4 = 50
          f.5 = 25
          g.6 = 10
          h.7 = 1
          j.8 = 0
          x.9 = 0
          j.8 = 1
          tmp.6 = h.7 << j.8
          h.7 = tmp.6
          tmp.5 = g.6 >> h.7
          g.6 = tmp.5
          tmp.4 = f.5 + g.6
          f.5 = tmp.4
          tmp.3 = e.4 ^ f.5
          e.4 = tmp.3
          d.3 = e.4
          tmp.2 = c.2 | d.3
          c.2 = tmp.2
          tmp.1 = b.1 * c.2
          b.1 = tmp.1
          tmp.0 = a.0 & b.1
          a.0 = tmp.0
          x.9 = a.0
          tmp.7 = a.0 == 40
          jump and_false__0 if !tmp.7
          tmp.10 = b.1 == 21800
          jump and_false__0 if !tmp.10
          tmp.9 = 1
          jump and_end__1
        and_false__0:
          tmp.9 = 0
        and_end__1:
          jump and_false__2 if !tmp.9
          tmp.13 = c.2 == 109
          jump and_false__2 if !tmp.13
          tmp.12 = 1
          jump and_end__3
        and_false__2:
          tmp.12 = 0
        and_end__3:
          jump and_false__4 if !tmp.12
          tmp.16 = d.3 == 41
          jump and_false__4 if !tmp.16
          tmp.15 = 1
          jump and_end__5
        and_false__4:
          tmp.15 = 0
        and_end__5:
          jump and_false__6 if !tmp.15
          tmp.19 = e.4 == 41
          jump and_false__6 if !tmp.19
          tmp.18 = 1
          jump and_end__7
        and_false__6:
          tmp.18 = 0
        and_end__7:
          jump and_false__8 if !tmp.18
          tmp.22 = f.5 == 27
          jump and_false__8 if !tmp.22
          tmp.21 = 1
          jump and_end__9
        and_false__8:
          tmp.21 = 0
        and_end__9:
          jump and_false__10 if !tmp.21
          tmp.25 = g.6 == 2
          jump and_false__10 if !tmp.25
          tmp.24 = 1
          jump and_end__11
        and_false__10:
          tmp.24 = 0
        and_end__11:
          jump and_false__12 if !tmp.24
          tmp.28 = h.7 == 2
          jump and_false__12 if !tmp.28
          tmp.27 = 1
          jump and_end__13
        and_false__12:
          tmp.27 = 0
        and_end__13:
          jump and_false__14 if !tmp.27
          tmp.31 = j.8 == 1
          jump and_false__14 if !tmp.31
          tmp.30 = 1
          jump and_end__15
        and_false__14:
          tmp.30 = 0
        and_end__15:
          jump and_false__16 if !tmp.30
          tmp.34 = x.9 == 40
          jump and_false__16 if !tmp.34
          tmp.33 = 1
          jump and_end__17
        and_false__16:
          tmp.33 = 0
        and_end__17:
          return tmp.33
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_extra_credit_compound_bitwise_or() {
    let src = r#"
        int main(void) {
            int to_or = 1;
            to_or |= 30;
            return to_or;
        }
    "#;
    let expected = r#"
        function main { 
          to_or.0 = 1
          tmp.0 = to_or.0 | 30
          to_or.0 = tmp.0
          return to_or.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_extra_credit_compound_bitwise_shiftl() {
    let src = r#"
        int main(void) {
            int to_shiftl = 3;
            to_shiftl <<= 4;
            return to_shiftl;
        }
    "#;
    let expected = r#"
        function main { 
          to_shiftl.0 = 3
          tmp.0 = to_shiftl.0 << 4
          to_shiftl.0 = tmp.0
          return to_shiftl.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_extra_credit_compound_bitwise_shiftr() {
    let src = r#"
        int main(void) {
            int to_shiftr = 382574;
            to_shiftr >>= 4;
            return to_shiftr;
        }
    "#;
    let expected = r#"
        function main { 
          to_shiftr.0 = 382574
          tmp.0 = to_shiftr.0 >> 4
          to_shiftr.0 = tmp.0
          return to_shiftr.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_extra_credit_compound_bitwise_xor() {
    let src = r#"
        int main(void) {
            int to_xor = 7;
            to_xor ^= 5;
            return to_xor;
        }
    "#;
    let expected = r#"
        function main { 
          to_xor.0 = 7
          tmp.0 = to_xor.0 ^ 5
          to_xor.0 = tmp.0
          return to_xor.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_extra_credit_compound_divide() {
    let src = r#"
        int main(void) {
            int to_divide = 8;
            to_divide /= 4;
            return to_divide;
        }
    "#;
    let expected = r#"
        function main { 
          to_divide.0 = 8
          tmp.0 = to_divide.0 / 4
          to_divide.0 = tmp.0
          return to_divide.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_extra_credit_compound_minus() {
    let src = r#"
        int main(void) {
            int to_subtract = 10;
            to_subtract -= 8;
            return to_subtract;
        }
    "#;
    let expected = r#"
        function main { 
          to_subtract.0 = 10
          tmp.0 = to_subtract.0 - 8
          to_subtract.0 = tmp.0
          return to_subtract.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_extra_credit_compound_mod() {
    let src = r#"
        int main(void) {
            int to_mod = 5;
            to_mod %= 3;
            return to_mod;
        }
    "#;
    let expected = r#"
        function main { 
          to_mod.0 = 5
          tmp.0 = to_mod.0 % 3
          to_mod.0 = tmp.0
          return to_mod.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_extra_credit_compound_multiply() {
    let src = r#"
        int main(void) {
            int to_multiply = 4;
            to_multiply *= 3;
            return to_multiply;
        }
    "#;
    let expected = r#"
        function main { 
          to_multiply.0 = 4
          tmp.0 = to_multiply.0 * 3
          to_multiply.0 = tmp.0
          return to_multiply.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_extra_credit_compound_plus() {
    let src = r#"
        int main(void) {
            int to_add = 0;
            to_add += 4;
            return to_add;
        }
    "#;
    let expected = r#"
        function main { 
          to_add.0 = 0
          tmp.0 = to_add.0 + 4
          to_add.0 = tmp.0
          return to_add.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_extra_credit_incr_expression_statement() {
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
        function main { 
          a.0 = 0
          b.1 = 0
          tmp.0 = a.0
          tmp.1 = inc a.0
          a.0 = tmp.1
          tmp.2 = inc a.0
          a.0 = tmp.2
          tmp.3 = inc a.0
          a.0 = tmp.3
          tmp.4 = b.1
          tmp.5 = dec b.1
          b.1 = tmp.5
          tmp.6 = dec b.1
          b.1 = tmp.6
          tmp.7 = a.0 == 3
          jump and_false__0 if !tmp.7
          tmp.11 = - 2
          tmp.10 = b.1 == tmp.11
          jump and_false__0 if !tmp.10
          tmp.9 = 1
          jump and_end__1
        and_false__0:
          tmp.9 = 0
        and_end__1:
          return tmp.9
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_extra_credit_incr_in_binary_expr() {
    let src = r#"
        int main(void) {
            int a = 2;
            int b = 3 + a++;
            int c = 4 + ++b;
            return (a == 3 && b == 6 && c == 10);
        }
    "#;
    let expected = r#"
        function main { 
          a.0 = 2
          tmp.1 = a.0
          tmp.2 = inc a.0
          a.0 = tmp.2
          tmp.0 = 3 + tmp.1
          b.1 = tmp.0
          tmp.4 = inc b.1
          b.1 = tmp.4
          tmp.3 = 4 + tmp.4
          c.2 = tmp.3
          tmp.5 = a.0 == 3
          jump and_false__0 if !tmp.5
          tmp.8 = b.1 == 6
          jump and_false__0 if !tmp.8
          tmp.7 = 1
          jump and_end__1
        and_false__0:
          tmp.7 = 0
        and_end__1:
          jump and_false__2 if !tmp.7
          tmp.11 = c.2 == 10
          jump and_false__2 if !tmp.11
          tmp.10 = 1
          jump and_end__3
        and_false__2:
          tmp.10 = 0
        and_end__3:
          return tmp.10
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_extra_credit_incr_parenthesized() {
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
        function main { 
          a.0 = 1
          b.1 = 2
          tmp.0 = inc a.0
          a.0 = tmp.0
          tmp.1 = - tmp.0
          c.2 = tmp.1
          tmp.2 = b.1
          tmp.3 = dec b.1
          b.1 = tmp.3
          tmp.4 = ! tmp.2
          d.3 = tmp.4
          tmp.5 = a.0 == 2
          jump and_false__0 if !tmp.5
          tmp.8 = b.1 == 1
          jump and_false__0 if !tmp.8
          tmp.7 = 1
          jump and_end__1
        and_false__0:
          tmp.7 = 0
        and_end__1:
          jump and_false__2 if !tmp.7
          tmp.12 = - 2
          tmp.11 = c.2 == tmp.12
          jump and_false__2 if !tmp.11
          tmp.10 = 1
          jump and_end__3
        and_false__2:
          tmp.10 = 0
        and_end__3:
          jump and_false__4 if !tmp.10
          tmp.15 = d.3 == 0
          jump and_false__4 if !tmp.15
          tmp.14 = 1
          jump and_end__5
        and_false__4:
          tmp.14 = 0
        and_end__5:
          return tmp.14
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_extra_credit_postfix_incr_and_decr() {
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
        function main { 
          a.0 = 1
          b.1 = 2
          tmp.0 = a.0
          tmp.1 = inc a.0
          a.0 = tmp.1
          c.2 = tmp.0
          tmp.2 = b.1
          tmp.3 = dec b.1
          b.1 = tmp.3
          d.3 = tmp.2
          tmp.4 = a.0 == 2
          jump and_false__0 if !tmp.4
          tmp.7 = b.1 == 1
          jump and_false__0 if !tmp.7
          tmp.6 = 1
          jump and_end__1
        and_false__0:
          tmp.6 = 0
        and_end__1:
          jump and_false__2 if !tmp.6
          tmp.10 = c.2 == 1
          jump and_false__2 if !tmp.10
          tmp.9 = 1
          jump and_end__3
        and_false__2:
          tmp.9 = 0
        and_end__3:
          jump and_false__4 if !tmp.9
          tmp.13 = d.3 == 2
          jump and_false__4 if !tmp.13
          tmp.12 = 1
          jump and_end__5
        and_false__4:
          tmp.12 = 0
        and_end__5:
          return tmp.12
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_extra_credit_postfix_precedence() {
    let src = r#"
        int main(void) {
            int a = 1;
            int b = !a++;
            return (a == 2 && b == 0);
        }
    "#;
    let expected = r#"
        function main { 
          a.0 = 1
          tmp.0 = a.0
          tmp.1 = inc a.0
          a.0 = tmp.1
          tmp.2 = ! tmp.0
          b.1 = tmp.2
          tmp.3 = a.0 == 2
          jump and_false__0 if !tmp.3
          tmp.6 = b.1 == 0
          jump and_false__0 if !tmp.6
          tmp.5 = 1
          jump and_end__1
        and_false__0:
          tmp.5 = 0
        and_end__1:
          return tmp.5
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_extra_credit_prefix_incr_and_decr() {
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
        function main { 
          a.0 = 1
          b.1 = 2
          tmp.0 = inc a.0
          a.0 = tmp.0
          c.2 = tmp.0
          tmp.1 = dec b.1
          b.1 = tmp.1
          d.3 = tmp.1
          tmp.2 = a.0 == 2
          jump and_false__0 if !tmp.2
          tmp.5 = b.1 == 1
          jump and_false__0 if !tmp.5
          tmp.4 = 1
          jump and_end__1
        and_false__0:
          tmp.4 = 0
        and_end__1:
          jump and_false__2 if !tmp.4
          tmp.8 = c.2 == 2
          jump and_false__2 if !tmp.8
          tmp.7 = 1
          jump and_end__3
        and_false__2:
          tmp.7 = 0
        and_end__3:
          jump and_false__4 if !tmp.7
          tmp.11 = d.3 == 1
          jump and_false__4 if !tmp.11
          tmp.10 = 1
          jump and_end__5
        and_false__4:
          tmp.10 = 0
        and_end__5:
          return tmp.10
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_kw_var_names() {
    let src = r#"
        int main(void) {
            int return_val = 3;
            int void2 = 2;
            return return_val + void2;
        }
    "#;
    let expected = r#"
        function main { 
          return_val.0 = 3
          void2.1 = 2
          tmp.0 = return_val.0 + void2.1
          return tmp.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_local_var_missing_return() {
    let src = r#"
        int main(void) {
            int a = 3;
            a = a + 5;
        }
    "#;
    let expected = r#"
        function main { 
          a.0 = 3
          tmp.0 = a.0 + 5
          a.0 = tmp.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_mixed_precedence_assignment() {
    let src = r#"
        int main(void) {
            int a = 1;
            int b = 0;
            a = 3 * (b = a);
            return a + b;
        }
    "#;
    let expected = r#"
        function main { 
          a.0 = 1
          b.1 = 0
          b.1 = a.0
          tmp.0 = 3 * b.1
          a.0 = tmp.0
          tmp.1 = a.0 + b.1
          return tmp.1
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_non_short_circuit_or() {
    let src = r#"
        int main(void) {
            int a = 0;
            0 || (a = 1);
            return a;
        }
    "#;
    let expected = r#"
        function main { 
          a.0 = 0
          jump or_true__0if 0
          a.0 = 1
          jump or_true__0if a.0
          tmp.1 = 0
          jump or_end__1
        or_true__0:
          tmp.1 = 1
        or_end__1:
          return a.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_null_statement() {
    let src = r#"
        int main(void) {
            ;
        }
    "#;
    let expected = r#"
        function main { 
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_null_then_return() {
    let src = r#"
        int main(void) {
            ;
            return 0;
        }
    "#;
    let expected = r#"
        function main { 
          return 0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_return_var() {
    let src = r#"
        int main(void) {
            int a = 2;
            return a;
        }
    "#;
    let expected = r#"
        function main { 
          a.0 = 2
          return a.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_short_circuit_and_fail() {
    let src = r#"
        int main(void) {
            int a = 0;
            0 && (a = 5);
            return a;
        }
    "#;
    let expected = r#"
        function main { 
          a.0 = 0
          jump and_false__0 if !0
          a.0 = 5
          jump and_false__0 if !a.0
          tmp.1 = 1
          jump and_end__1
        and_false__0:
          tmp.1 = 0
        and_end__1:
          return a.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_short_circuit_or() {
    let src = r#"
        int main(void) {
            int a = 0;
            1 || (a = 1);
            return a;
        }
    "#;
    let expected = r#"
        function main { 
          a.0 = 0
          jump or_true__0if 1
          a.0 = 1
          jump or_true__0if a.0
          tmp.1 = 0
          jump or_end__1
        or_true__0:
          tmp.1 = 1
        or_end__1:
          return a.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_unused_exp() {
    let src = r#"
        int main(void) {
            2 + 2;
            return 0;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = 2 + 2
          return 0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_use_assignment_result() {
    let src = r#"
        int main(void) {
            int a = 1;
            int b = 2;
            return a = b = 4;
        }
    "#;
    let expected = r#"
        function main { 
          a.0 = 1
          b.1 = 2
          b.1 = 4
          a.0 = b.1
          return a.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_5_valid_use_val_in_own_initializer() {
    let src = r#"
        int main(void) {
            int a = 0 && a;
            return a;
        }
    "#;
    let expected = r#"
        function main { 
          jump and_false__0 if !0
          jump and_false__0 if !a.0
          tmp.1 = 1
          jump and_end__1
        and_false__0:
          tmp.1 = 0
        and_end__1:
          a.0 = tmp.1
          return a.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_assign_ternary() {
    let src = r#"
        int main(void) {
            int a = 0;
            a = 1 ? 2 : 3;
            return a;
        }
    "#;
    let expected = r#"
        function main { 
          a.0 = 0
          jump else__1 if !1
          tmp.0 = 2
          jump end_if__0
        else__1:
          tmp.0 = 3
        end_if__0:
          a.0 = tmp.0
          return a.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_binary_condition() {
    let src = r#"
        int main(void) {
            if (1 + 2 == 3)
                return 5;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = 1 + 2
          tmp.1 = tmp.0 == 3
          jump end_if__0 if !tmp.1
          return 5
        end_if__0:
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_binary_false_condition() {
    let src = r#"
        int main(void) {
            if (1 + 2 == 4)
                return 5;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = 1 + 2
          tmp.1 = tmp.0 == 4
          jump end_if__0 if !tmp.1
          return 5
        end_if__0:
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_else() {
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
        function main { 
          a.0 = 0
          jump else__1 if !a.0
          return 1
          jump end_if__0
        else__1:
          return 2
        end_if__0:
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_extra_credit_bitwise_ternary() {
    let src = r#"
        int main(void) {
            int result;
            1 ^ 1 ? result = 4 : (result = 5);
            return result;
        }
    "#;
    let expected = r#"
        function main { 
          tmp.0 = 1 ^ 1
          jump else__1 if !tmp.0
          result.0 = 4
          tmp.1 = result.0
          jump end_if__0
        else__1:
          result.0 = 5
          tmp.1 = result.0
        end_if__0:
          return result.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_extra_credit_compound_assign_ternary() {
    let src = r#"
        int main(void) {
            int a = 4;
            a *= 1 ? 2 : 3;
            return a;
        }
    "#;
    let expected = r#"
        function main { 
          a.0 = 4
          jump else__1 if !1
          tmp.1 = 2
          jump end_if__0
        else__1:
          tmp.1 = 3
        end_if__0:
          tmp.0 = a.0 * tmp.1
          a.0 = tmp.0
          return a.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_extra_credit_compound_if_expression() {
    let src = r#"
        int main(void) {
            int a = 0;
            if (a += 1)
                return a;
            return 10;
        }
    "#;
    let expected = r#"
        function main { 
          a.0 = 0
          tmp.0 = a.0 + 1
          a.0 = tmp.0
          jump end_if__0 if !a.0
          return a.0
        end_if__0:
          return 10
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_extra_credit_goto_after_declaration() {
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
        function main { 
          x.0 = 1
          jump post_declaration_2
          x.0 = 0
          i.1 = x.0
        post_declaration:
          i.1 = 5
          tmp.0 = x.0 == 1
          jump and_false__0 if !tmp.0
          tmp.3 = i.1 == 5
          jump and_false__0 if !tmp.3
          tmp.2 = 1
          jump and_end__1
        and_false__0:
          tmp.2 = 0
        and_end__1:
          return tmp.2
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_extra_credit_goto_backwards() {
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
        function main { 
          jump end_if__0 if !0
        label:
          return 5
        end_if__0:
          jump label_0
          return 0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_extra_credit_goto_label() {
    let src = r#"
        int main(void) {
            goto label;
            return 0;
        label:
            return 1;
        }
    "#;
    let expected = r#"
        function main { 
          jump label_0
          return 0
        label:
          return 1
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_extra_credit_goto_label_and_var() {
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
        function main { 
          ident.0 = 5
          jump ident_1
          return 0
        ident:
          return ident.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_extra_credit_goto_label_main() {
    let src = r#"
        int main(void) {
            goto main;
            return 5;
        main:
            return 0;
        }
    "#;
    let expected = r#"
        function main { 
          jump main_0
          return 5
        main:
          return 0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_extra_credit_goto_label_main_2() {
    let src = r#"
        int main(void) {
            goto _main;
            return 0;
            _main:
                return 1;
        }
    "#;
    let expected = r#"
        function main { 
          jump _main_0
          return 0
        _main:
          return 1
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_extra_credit_goto_nested_label() {
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
        function main { 
          jump labelB_1
        labelA:
        labelB:
          return 5
          return 0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_extra_credit_label_all_statements() {
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
        function main { 
          a.0 = 1
        label_if:
          jump else__1 if !a.0
          jump label_expression
          jump end_if__0
        else__1:
          jump label_empty
        end_if__0:
        label_goto:
          jump label_return
          jump end_if__2 if !0
        label_expression:
          a.0 = 0
        end_if__2:
          jump label_if_1
        label_return:
          return a.0
        label_empty:
          a.0 = 100
          jump label_goto_2
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_extra_credit_label_token() {
    let src = r#"
        int main(void) {
            goto _foo_1_;
            return 0;
        _foo_1_:
            return 1;
        }
    "#;
    let expected = r#"
        function main { 
          jump _foo_1__0
          return 0
        _foo_1_:
          return 1
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_extra_credit_lh_compound_assignment() {
    let src = r#"
        int main(void) {
            int x = 10;
            (x -= 1) ? (x /= 2) : 0;
            return x == 4;
        }
    "#;
    let expected = r#"
        function main { 
          x.0 = 10
          tmp.0 = x.0 - 1
          x.0 = tmp.0
          jump else__1 if !x.0
          tmp.2 = x.0 / 2
          x.0 = tmp.2
          tmp.1 = x.0
          jump end_if__0
        else__1:
          tmp.1 = 0
        end_if__0:
          tmp.3 = x.0 == 4
          return tmp.3
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_extra_credit_postfix_if() {
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
        function main { 
          a.0 = 0
          tmp.0 = a.0
          tmp.1 = dec a.0
          a.0 = tmp.1
          jump else__1 if !tmp.0
          return 0
          jump end_if__0
        else__1:
          tmp.2 = a.0
          tmp.3 = dec a.0
          a.0 = tmp.3
          jump end_if__2 if !tmp.2
          return 1
        end_if__2:
        end_if__0:
          return 0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_extra_credit_postfix_in_ternary() {
    let src = r#"
        int main(void) {
            int x = 10;
            x - 10 ? 0 : x--;
            return x;
        }
    "#;
    let expected = r#"
        function main { 
          x.0 = 10
          tmp.0 = x.0 - 10
          jump else__1 if !tmp.0
          tmp.1 = 0
          jump end_if__0
        else__1:
          tmp.2 = x.0
          tmp.3 = dec x.0
          x.0 = tmp.3
          tmp.1 = tmp.2
        end_if__0:
          return x.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_extra_credit_prefix_if() {
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
        function main { 
          tmp.0 = - 1
          a.0 = tmp.0
          tmp.1 = inc a.0
          a.0 = tmp.1
          jump else__1 if !tmp.1
          return 0
          jump end_if__0
        else__1:
          tmp.2 = inc a.0
          a.0 = tmp.2
          jump end_if__2 if !tmp.2
          return 1
        end_if__2:
        end_if__0:
          return 0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_extra_credit_prefix_in_ternary() {
    let src = r#"
        int main(void) {
            int a = 0;
            return (++a ? ++a : 0);
        }
    "#;
    let expected = r#"
        function main { 
          a.0 = 0
          tmp.0 = inc a.0
          a.0 = tmp.0
          jump else__1 if !tmp.0
          tmp.2 = inc a.0
          a.0 = tmp.2
          tmp.1 = tmp.2
          jump end_if__0
        else__1:
          tmp.1 = 0
        end_if__0:
          return tmp.1
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_extra_credit_unused_label() {
    let src = r#"
        int main(void) {
        unused:
            return 0;
        }
    "#;
    let expected = r#"
        function main { 
        unused:
          return 0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_extra_credit_whitespace_after_label() {
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
        function main { 
          jump label2_1
          return 0
        label1:
        label2:
          return 1
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_if_nested() {
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
        function main { 
          a.0 = 1
          b.1 = 0
          jump else__1 if !a.0
          b.1 = 1
          jump end_if__0
        else__1:
          jump end_if__2 if !b.1
          b.1 = 2
        end_if__2:
        end_if__0:
          return b.1
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_if_nested_2() {
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
        function main { 
          a.0 = 0
          b.1 = 1
          jump else__1 if !a.0
          b.1 = 1
          jump end_if__0
        else__1:
          tmp.0 = ~ b.1
          jump end_if__2 if !tmp.0
          b.1 = 2
        end_if__2:
        end_if__0:
          return b.1
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_if_nested_3() {
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
        function main { 
          a.0 = 0
          a.0 = 1
          jump end_if__0 if !a.0
          tmp.0 = a.0 == 1
          jump else__3 if !tmp.0
          a.0 = 3
          jump end_if__2
        else__3:
          a.0 = 4
        end_if__2:
        end_if__0:
          return a.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_if_nested_4() {
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
        function main { 
          a.0 = 0
          tmp.0 = ! a.0
          jump end_if__0 if !tmp.0
          tmp.1 = 3 / 4
          jump else__3 if !tmp.1
          a.0 = 3
          jump end_if__2
        else__3:
          tmp.2 = 8 / 2
          a.0 = tmp.2
        end_if__2:
        end_if__0:
          return a.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_if_nested_5() {
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
        function main { 
          a.0 = 0
          jump else__1 if !0
          jump else__3 if !0
          a.0 = 3
          jump end_if__2
        else__3:
          a.0 = 4
        end_if__2:
          jump end_if__0
        else__1:
          a.0 = 1
        end_if__0:
          return a.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_if_not_taken() {
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
        function main { 
          a.0 = 0
          b.1 = 0
          jump end_if__0 if !a.0
          b.1 = 1
        end_if__0:
          return b.1
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_if_null_body() {
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
        function main { 
          x.0 = 0
          jump else__1 if !0
          jump end_if__0
        else__1:
          x.0 = 1
        end_if__0:
          return x.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_if_taken() {
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
        function main { 
          a.0 = 1
          b.1 = 0
          jump end_if__0 if !a.0
          b.1 = 1
        end_if__0:
          return b.1
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_lh_assignment() {
    let src = r#"
        int main(void) {
            int x = 10;
            int y = 0;
            y = (x = 5) ? x : 2;
            return (x == 5 && y == 5);
        }
    "#;
    let expected = r#"
        function main { 
          x.0 = 10
          y.1 = 0
          x.0 = 5
          jump else__1 if !x.0
          tmp.0 = x.0
          jump end_if__0
        else__1:
          tmp.0 = 2
        end_if__0:
          y.1 = tmp.0
          tmp.1 = x.0 == 5
          jump and_false__2 if !tmp.1
          tmp.4 = y.1 == 5
          jump and_false__2 if !tmp.4
          tmp.3 = 1
          jump and_end__3
        and_false__2:
          tmp.3 = 0
        and_end__3:
          return tmp.3
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_multiple_if() {
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
        function main { 
          a.0 = 0
          b.1 = 0
          jump else__1 if !a.0
          a.0 = 2
          jump end_if__0
        else__1:
          a.0 = 3
        end_if__0:
          jump else__3 if !b.1
          b.1 = 4
          jump end_if__2
        else__3:
          b.1 = 5
        end_if__2:
          tmp.0 = a.0 + b.1
          return tmp.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_nested_ternary() {
    let src = r#"
        int main(void) {
            int a = 1;
            int b = 2;
            int flag = 0;
            return a > b ? 5 : flag ? 6 : 7;
        }
    "#;
    let expected = r#"
        function main { 
          a.0 = 1
          b.1 = 2
          flag.2 = 0
          tmp.0 = a.0 > b.1
          jump else__1 if !tmp.0
          tmp.1 = 5
          jump end_if__0
        else__1:
          jump else__3 if !flag.2
          tmp.2 = 6
          jump end_if__2
        else__3:
          tmp.2 = 7
        end_if__2:
          tmp.1 = tmp.2
        end_if__0:
          return tmp.1
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_nested_ternary_2() {
    let src = r#"
        int main(void) {
            int a = 1 ? 2 ? 3 : 4 : 5;
            int b = 0 ? 2 ? 3 : 4 : 5;
            return a * b;
        }
    "#;
    let expected = r#"
        function main { 
          jump else__1 if !1
          jump else__3 if !2
          tmp.1 = 3
          jump end_if__2
        else__3:
          tmp.1 = 4
        end_if__2:
          tmp.0 = tmp.1
          jump end_if__0
        else__1:
          tmp.0 = 5
        end_if__0:
          a.0 = tmp.0
          jump else__5 if !0
          jump else__7 if !2
          tmp.3 = 3
          jump end_if__6
        else__7:
          tmp.3 = 4
        end_if__6:
          tmp.2 = tmp.3
          jump end_if__4
        else__5:
          tmp.2 = 5
        end_if__4:
          b.1 = tmp.2
          tmp.4 = a.0 * b.1
          return tmp.4
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_rh_assignment() {
    let src = r#"
        int main(void) {
            int flag = 1;
            int a = 0;
            flag ? a = 1 : (a = 0);
            return a;
        }
    "#;
    let expected = r#"
        function main { 
          flag.0 = 1
          a.1 = 0
          jump else__1 if !flag.0
          a.1 = 1
          tmp.0 = a.1
          jump end_if__0
        else__1:
          a.1 = 0
          tmp.0 = a.1
        end_if__0:
          return a.1
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_ternary() {
    let src = r#"
        int main(void) {
            int a = 0;
            return a > -1 ? 4 : 5;
        }
    "#;
    let expected = r#"
        function main { 
          a.0 = 0
          tmp.1 = - 1
          tmp.0 = a.0 > tmp.1
          jump else__1 if !tmp.0
          tmp.2 = 4
          jump end_if__0
        else__1:
          tmp.2 = 5
        end_if__0:
          return tmp.2
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_ternary_middle_assignment() {
    let src = r#"
        int main(void) {
            int a = 1;
            a != 2 ? a = 2 : 0;
            return a;
        }
    "#;
    let expected = r#"
        function main { 
          a.0 = 1
          tmp.0 = a.0 != 2
          jump else__1 if !tmp.0
          a.0 = 2
          tmp.1 = a.0
          jump end_if__0
        else__1:
          tmp.1 = 0
        end_if__0:
          return a.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_ternary_middle_binop() {
    let src = r#"
        int main(void) {
            int a = 1 ? 3 % 2 : 4;
            return a;
        }
    "#;
    let expected = r#"
        function main { 
          jump else__1 if !1
          tmp.1 = 3 % 2
          tmp.0 = tmp.1
          jump end_if__0
        else__1:
          tmp.0 = 4
        end_if__0:
          a.0 = tmp.0
          return a.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_ternary_precedence() {
    let src = r#"
        int main(void) {
            int a = 10;
            return a || 0 ? 20 : 0;
        }
    "#;
    let expected = r#"
        function main { 
          a.0 = 10
          jump or_true__0if a.0
          jump or_true__0if 0
          tmp.1 = 0
          jump or_end__1
        or_true__0:
          tmp.1 = 1
        or_end__1:
          jump else__3 if !tmp.1
          tmp.2 = 20
          jump end_if__2
        else__3:
          tmp.2 = 0
        end_if__2:
          return tmp.2
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_ternary_rh_binop() {
    let src = r#"
        int main(void) {
            return 0 ? 1 : 0 || 2;
        }
    "#;
    let expected = r#"
        function main { 
          jump else__1 if !0
          tmp.0 = 1
          jump end_if__0
        else__1:
          jump or_true__2if 0
          jump or_true__2if 2
          tmp.2 = 0
          jump or_end__3
        or_true__2:
          tmp.2 = 1
        or_end__3:
          tmp.0 = tmp.2
        end_if__0:
          return tmp.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_ternary_short_circuit() {
    let src = r#"
        int main(void) {
            int a = 1;
            int b = 0;
            a ? (b = 1) : (b = 2);
            return b;
        }
    "#;
    let expected = r#"
        function main { 
          a.0 = 1
          b.1 = 0
          jump else__1 if !a.0
          b.1 = 1
          tmp.0 = b.1
          jump end_if__0
        else__1:
          b.1 = 2
          tmp.0 = b.1
        end_if__0:
          return b.1
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_6_valid_ternary_short_circuit_2() {
    let src = r#"
        int main(void) {
            int a = 0;
            int b = 0;
            a ? (b = 1) : (b = 2);
            return b;
        }
    "#;
    let expected = r#"
        function main { 
          a.0 = 0
          b.1 = 0
          jump else__1 if !a.0
          b.1 = 1
          tmp.0 = b.1
          jump end_if__0
        else__1:
          b.1 = 2
          tmp.0 = b.1
        end_if__0:
          return b.1
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_7_valid_assign_to_self() {
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
        function main { 
          a.0 = 3
          a.1 = 4
          a.1 = a.1
          return a.1
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_7_valid_assign_to_self_2() {
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
        function main { 
          a.0 = 3
          a.1 = 4
          a.1 = a.1
          return a.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_7_valid_declaration_only() {
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
        function main { 
          a.0 = 1
          b.1 = a.0
          return a.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_7_valid_empty_blocks() {
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
        function main { 
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
fn test_chapter_7_valid_extra_credit_compound_subtract_in_block() {
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
        function main { 
          a.0 = 5
          tmp.0 = a.0 > 4
          jump end_if__0 if !tmp.0
          tmp.1 = a.0 - 4
          a.0 = tmp.1
          a.1 = 5
          tmp.2 = a.1 > 4
          jump end_if__2 if !tmp.2
          tmp.3 = a.1 - 4
          a.1 = tmp.3
        end_if__2:
        end_if__0:
          return a.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_7_valid_extra_credit_goto_before_declaration() {
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
        function main { 
          a.0 = 0
          tmp.0 = a.0 != 0
          jump end_if__0 if !tmp.0
        return_a:
          return a.0
        end_if__0:
          a.2 = 4
          jump return_a_1
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_7_valid_extra_credit_goto_inner_scope() {
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
        function main { 
          x.0 = 5
          jump inner_2
          x.1 = 0
        inner:
          x.1 = 1
          return x.1
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_7_valid_extra_credit_goto_outer_scope() {
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
        function main { 
          a.0 = 10
          b.1 = 0
          jump end_if__0 if !a.0
          a.2 = 1
          b.1 = a.2
          jump end_3
        end_if__0:
          a.0 = 9
        end:
          tmp.0 = a.0 == 10
          jump and_false__2 if !tmp.0
          tmp.3 = b.1 == 1
          jump and_false__2 if !tmp.3
          tmp.2 = 1
          jump and_end__3
        and_false__2:
          tmp.2 = 0
        and_end__3:
          return tmp.2
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_7_valid_extra_credit_goto_sibling_scope() {
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
        function main { 
          sum.0 = 0
          jump end_if__0 if !1
          a.1 = 5
          jump other_if_3
          sum.0 = 0
        first_if:
          a.1 = 5
          tmp.0 = sum.0 + a.1
          sum.0 = tmp.0
        end_if__0:
          jump end_if__2 if !0
        other_if:
          a.4 = 6
          tmp.1 = sum.0 + a.4
          sum.0 = tmp.1
          jump first_if_2
          sum.0 = 0
        end_if__2:
          return sum.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_7_valid_hidden_then_visible() {
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
        function main { 
          a.0 = 2
          tmp.0 = - 4
          a.0 = tmp.0
          a.2 = 7
          tmp.1 = a.2 + 1
          b.1 = tmp.1
          tmp.2 = b.1 == 8
          jump and_false__0 if !tmp.2
          tmp.6 = - 4
          tmp.5 = a.0 == tmp.6
          jump and_false__0 if !tmp.5
          tmp.4 = 1
          jump and_end__1
        and_false__0:
          tmp.4 = 0
        and_end__1:
          return tmp.4
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_7_valid_hidden_variable() {
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
        function main { 
          a.0 = 2
          a.1 = 1
          return a.1
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_7_valid_inner_uninitialized() {
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
        function main { 
          x.0 = 4
          return x.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_7_valid_multiple_vars_same_name() {
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
        function main { 
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
fn test_chapter_7_valid_nested_if() {
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
        function main { 
          a.0 = 0
          jump else__1 if !a.0
          b.1 = 2
          return b.1
          jump end_if__0
        else__1:
          c.2 = 3
          tmp.0 = a.0 < c.2
          jump else__3 if !tmp.0
          tmp.1 = ! a.0
          return tmp.1
          jump end_if__2
        else__3:
          return 5
        end_if__2:
        end_if__0:
          return a.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_7_valid_similar_var_names() {
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
        function main { 
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
fn test_chapter_7_valid_use_in_inner_scope() {
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
        function main { 
          x.0 = 3
          return x.0
          return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}
