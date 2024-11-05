use crate::pretty::{dedent, dump_tacky};

#[test]
fn test_chapter_1_valid_multi_digit() {
    let src = r#"
        int main(void) {
            return 100;
        }
    "#;
    let expected = r#"
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
fn test_chapter_4_valid_and_short_circuit() {
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
fn test_chapter_4_valid_and_true() {
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
fn test_chapter_4_valid_associativity() {
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
fn test_chapter_4_valid_compare_arithmetic_results() {
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
fn test_chapter_4_valid_eq_false() {
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
fn test_chapter_4_valid_eq_precedence() {
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
fn test_chapter_4_valid_eq_true() {
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
fn test_chapter_4_valid_extra_credit_bitwise_and_precedence() {
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
fn test_chapter_4_valid_extra_credit_bitwise_or_precedence() {
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
fn test_chapter_4_valid_extra_credit_bitwise_shift_precedence() {
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
fn test_chapter_4_valid_extra_credit_bitwise_xor_precedence() {
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
fn test_chapter_4_valid_ge_false() {
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
fn test_chapter_4_valid_ge_true() {
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
fn test_chapter_4_valid_gt_false() {
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
fn test_chapter_4_valid_gt_true() {
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
fn test_chapter_4_valid_le_false() {
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
fn test_chapter_4_valid_le_true() {
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
fn test_chapter_4_valid_lt_false() {
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
fn test_chapter_4_valid_lt_true() {
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
fn test_chapter_4_valid_multi_short_circuit() {
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
fn test_chapter_4_valid_ne_false() {
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
fn test_chapter_4_valid_ne_true() {
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
fn test_chapter_4_valid_nested_ops() {
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
fn test_chapter_4_valid_not() {
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
fn test_chapter_4_valid_not_sum() {
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
fn test_chapter_4_valid_not_sum_2() {
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
fn test_chapter_4_valid_not_zero() {
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
fn test_chapter_4_valid_operate_on_booleans() {
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
fn test_chapter_4_valid_or_false() {
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
fn test_chapter_4_valid_or_short_circuit() {
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
fn test_chapter_4_valid_or_true() {
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
fn test_chapter_4_valid_precedence() {
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
fn test_chapter_4_valid_precedence_2() {
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
fn test_chapter_4_valid_precedence_3() {
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
fn test_chapter_4_valid_precedence_4() {
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
fn test_chapter_4_valid_precedence_5() {
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
            if 0 jump or_true_0
            if 5 jump or_true_0
            tmp.1 = 0
            jump or_end_1
        
          or_true_0:
            tmp.1 = 1
        
          or_end_1:
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
            if !tmp.7 jump and_false_0
            tmp.10 = b.1 == 2000
            if !tmp.10 jump and_false_0
            tmp.9 = 1
            jump and_end_1
        
          and_false_0:
            tmp.9 = 0
        
          and_end_1:
            if !tmp.9 jump and_false_2
            tmp.14 = - 1800
            tmp.13 = c.2 == tmp.14
            if !tmp.13 jump and_false_2
            tmp.12 = 1
            jump and_end_3
        
          and_false_2:
            tmp.12 = 0
        
          and_end_3:
            if !tmp.12 jump and_false_4
            tmp.18 = - 18
            tmp.17 = d.3 == tmp.18
            if !tmp.17 jump and_false_4
            tmp.16 = 1
            jump and_end_5
        
          and_false_4:
            tmp.16 = 0
        
          and_end_5:
            if !tmp.16 jump and_false_6
            tmp.22 = - 4
            tmp.21 = e.4 == tmp.22
            if !tmp.21 jump and_false_6
            tmp.20 = 1
            jump and_end_7
        
          and_false_6:
            tmp.20 = 0
        
          and_end_7:
            if !tmp.20 jump and_false_8
            tmp.26 = - 7
            tmp.25 = f.5 == tmp.26
            if !tmp.25 jump and_false_8
            tmp.24 = 1
            jump and_end_9
        
          and_false_8:
            tmp.24 = 0
        
          and_end_9:
            if !tmp.24 jump and_false_10
            tmp.29 = x.6 == 2250
            if !tmp.29 jump and_false_10
            tmp.28 = 1
            jump and_end_11
        
          and_false_10:
            tmp.28 = 0
        
          and_end_11:
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
        global function main() { 
            a.0 = 10
            b.1 = 12
            if 0 jump or_true_0
            if b.1 jump or_true_0
            tmp.2 = 0
            jump or_end_1
        
          or_true_0:
            tmp.2 = 1
        
          or_end_1:
            tmp.0 = a.0 + tmp.2
            a.0 = tmp.0
            if !a.0 jump and_false_2
            if !0 jump and_false_2
            tmp.5 = 1
            jump and_end_3
        
          and_false_2:
            tmp.5 = 0
        
          and_end_3:
            tmp.3 = b.1 * tmp.5
            b.1 = tmp.3
            c.2 = 14
            if a.0 jump or_true_4
            if b.1 jump or_true_4
            tmp.8 = 0
            jump or_end_5
        
          or_true_4:
            tmp.8 = 1
        
          or_end_5:
            tmp.6 = c.2 - tmp.8
            c.2 = tmp.6
            d.3 = 16
            if c.2 jump or_true_6
            if d.3 jump or_true_6
            tmp.11 = 0
            jump or_end_7
        
          or_true_6:
            tmp.11 = 1
        
          or_end_7:
            tmp.9 = d.3 / tmp.11
            d.3 = tmp.9
            tmp.12 = a.0 == 11
            if !tmp.12 jump and_false_8
            tmp.15 = b.1 == 0
            if !tmp.15 jump and_false_8
            tmp.14 = 1
            jump and_end_9
        
          and_false_8:
            tmp.14 = 0
        
          and_end_9:
            if !tmp.14 jump and_false_10
            tmp.18 = c.2 == 13
            if !tmp.18 jump and_false_10
            tmp.17 = 1
            jump and_end_11
        
          and_false_10:
            tmp.17 = 0
        
          and_end_11:
            if !tmp.17 jump and_false_12
            tmp.21 = d.3 == 16
            if !tmp.21 jump and_false_12
            tmp.20 = 1
            jump and_end_13
        
          and_false_12:
            tmp.20 = 0
        
          and_end_13:
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
        global function main() { 
            x.0 = 1
            tmp.0 = x.0 + 3
            x.0 = tmp.0
            y.1 = x.0
            tmp.1 = x.0 == 4
            if !tmp.1 jump and_false_0
            tmp.4 = y.1 == 4
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
fn test_chapter_5_valid_extra_credit_compound_bitwise_and() {
    let src = r#"
        int main(void) {
            int to_and = 3;
            to_and &= 6;
            return to_and;
        }
    "#;
    let expected = r#"
        global function main() { 
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
        global function main() { 
            a.0 = 11
            b.1 = 12
            if 0 jump or_true_0
            if b.1 jump or_true_0
            tmp.2 = 0
            jump or_end_1
        
          or_true_0:
            tmp.2 = 1
        
          or_end_1:
            tmp.0 = a.0 & tmp.2
            a.0 = tmp.0
            if a.0 jump or_true_2
            if 1 jump or_true_2
            tmp.5 = 0
            jump or_end_3
        
          or_true_2:
            tmp.5 = 1
        
          or_end_3:
            tmp.3 = b.1 ^ tmp.5
            b.1 = tmp.3
            c.2 = 14
            if a.0 jump or_true_4
            if b.1 jump or_true_4
            tmp.8 = 0
            jump or_end_5
        
          or_true_4:
            tmp.8 = 1
        
          or_end_5:
            tmp.6 = c.2 | tmp.8
            c.2 = tmp.6
            d.3 = 16
            if c.2 jump or_true_6
            if d.3 jump or_true_6
            tmp.11 = 0
            jump or_end_7
        
          or_true_6:
            tmp.11 = 1
        
          or_end_7:
            tmp.9 = d.3 >> tmp.11
            d.3 = tmp.9
            e.4 = 18
            if c.2 jump or_true_8
            if d.3 jump or_true_8
            tmp.14 = 0
            jump or_end_9
        
          or_true_8:
            tmp.14 = 1
        
          or_end_9:
            tmp.12 = e.4 << tmp.14
            e.4 = tmp.12
            tmp.15 = a.0 == 1
            if !tmp.15 jump and_false_10
            tmp.18 = b.1 == 13
            if !tmp.18 jump and_false_10
            tmp.17 = 1
            jump and_end_11
        
          and_false_10:
            tmp.17 = 0
        
          and_end_11:
            if !tmp.17 jump and_false_12
            tmp.21 = c.2 == 15
            if !tmp.21 jump and_false_12
            tmp.20 = 1
            jump and_end_13
        
          and_false_12:
            tmp.20 = 0
        
          and_end_13:
            if !tmp.20 jump and_false_14
            tmp.24 = d.3 == 8
            if !tmp.24 jump and_false_14
            tmp.23 = 1
            jump and_end_15
        
          and_false_14:
            tmp.23 = 0
        
          and_end_15:
            if !tmp.23 jump and_false_16
            tmp.27 = e.4 == 36
            if !tmp.27 jump and_false_16
            tmp.26 = 1
            jump and_end_17
        
          and_false_16:
            tmp.26 = 0
        
          and_end_17:
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
        global function main() { 
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
            if !tmp.7 jump and_false_0
            tmp.10 = b.1 == 21800
            if !tmp.10 jump and_false_0
            tmp.9 = 1
            jump and_end_1
        
          and_false_0:
            tmp.9 = 0
        
          and_end_1:
            if !tmp.9 jump and_false_2
            tmp.13 = c.2 == 109
            if !tmp.13 jump and_false_2
            tmp.12 = 1
            jump and_end_3
        
          and_false_2:
            tmp.12 = 0
        
          and_end_3:
            if !tmp.12 jump and_false_4
            tmp.16 = d.3 == 41
            if !tmp.16 jump and_false_4
            tmp.15 = 1
            jump and_end_5
        
          and_false_4:
            tmp.15 = 0
        
          and_end_5:
            if !tmp.15 jump and_false_6
            tmp.19 = e.4 == 41
            if !tmp.19 jump and_false_6
            tmp.18 = 1
            jump and_end_7
        
          and_false_6:
            tmp.18 = 0
        
          and_end_7:
            if !tmp.18 jump and_false_8
            tmp.22 = f.5 == 27
            if !tmp.22 jump and_false_8
            tmp.21 = 1
            jump and_end_9
        
          and_false_8:
            tmp.21 = 0
        
          and_end_9:
            if !tmp.21 jump and_false_10
            tmp.25 = g.6 == 2
            if !tmp.25 jump and_false_10
            tmp.24 = 1
            jump and_end_11
        
          and_false_10:
            tmp.24 = 0
        
          and_end_11:
            if !tmp.24 jump and_false_12
            tmp.28 = h.7 == 2
            if !tmp.28 jump and_false_12
            tmp.27 = 1
            jump and_end_13
        
          and_false_12:
            tmp.27 = 0
        
          and_end_13:
            if !tmp.27 jump and_false_14
            tmp.31 = j.8 == 1
            if !tmp.31 jump and_false_14
            tmp.30 = 1
            jump and_end_15
        
          and_false_14:
            tmp.30 = 0
        
          and_end_15:
            if !tmp.30 jump and_false_16
            tmp.34 = x.9 == 40
            if !tmp.34 jump and_false_16
            tmp.33 = 1
            jump and_end_17
        
          and_false_16:
            tmp.33 = 0
        
          and_end_17:
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
            if !tmp.7 jump and_false_0
            tmp.11 = - 2
            tmp.10 = b.1 == tmp.11
            if !tmp.10 jump and_false_0
            tmp.9 = 1
            jump and_end_1
        
          and_false_0:
            tmp.9 = 0
        
          and_end_1:
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
        global function main() { 
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
            if !tmp.5 jump and_false_0
            tmp.8 = b.1 == 6
            if !tmp.8 jump and_false_0
            tmp.7 = 1
            jump and_end_1
        
          and_false_0:
            tmp.7 = 0
        
          and_end_1:
            if !tmp.7 jump and_false_2
            tmp.11 = c.2 == 10
            if !tmp.11 jump and_false_2
            tmp.10 = 1
            jump and_end_3
        
          and_false_2:
            tmp.10 = 0
        
          and_end_3:
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
        global function main() { 
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
            if !tmp.5 jump and_false_0
            tmp.8 = b.1 == 1
            if !tmp.8 jump and_false_0
            tmp.7 = 1
            jump and_end_1
        
          and_false_0:
            tmp.7 = 0
        
          and_end_1:
            if !tmp.7 jump and_false_2
            tmp.12 = - 2
            tmp.11 = c.2 == tmp.12
            if !tmp.11 jump and_false_2
            tmp.10 = 1
            jump and_end_3
        
          and_false_2:
            tmp.10 = 0
        
          and_end_3:
            if !tmp.10 jump and_false_4
            tmp.15 = d.3 == 0
            if !tmp.15 jump and_false_4
            tmp.14 = 1
            jump and_end_5
        
          and_false_4:
            tmp.14 = 0
        
          and_end_5:
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
        global function main() { 
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
            if !tmp.4 jump and_false_0
            tmp.7 = b.1 == 1
            if !tmp.7 jump and_false_0
            tmp.6 = 1
            jump and_end_1
        
          and_false_0:
            tmp.6 = 0
        
          and_end_1:
            if !tmp.6 jump and_false_2
            tmp.10 = c.2 == 1
            if !tmp.10 jump and_false_2
            tmp.9 = 1
            jump and_end_3
        
          and_false_2:
            tmp.9 = 0
        
          and_end_3:
            if !tmp.9 jump and_false_4
            tmp.13 = d.3 == 2
            if !tmp.13 jump and_false_4
            tmp.12 = 1
            jump and_end_5
        
          and_false_4:
            tmp.12 = 0
        
          and_end_5:
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
        global function main() { 
            a.0 = 1
            tmp.0 = a.0
            tmp.1 = inc a.0
            a.0 = tmp.1
            tmp.2 = ! tmp.0
            b.1 = tmp.2
            tmp.3 = a.0 == 2
            if !tmp.3 jump and_false_0
            tmp.6 = b.1 == 0
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
        global function main() { 
            a.0 = 1
            b.1 = 2
            tmp.0 = inc a.0
            a.0 = tmp.0
            c.2 = tmp.0
            tmp.1 = dec b.1
            b.1 = tmp.1
            d.3 = tmp.1
            tmp.2 = a.0 == 2
            if !tmp.2 jump and_false_0
            tmp.5 = b.1 == 1
            if !tmp.5 jump and_false_0
            tmp.4 = 1
            jump and_end_1
        
          and_false_0:
            tmp.4 = 0
        
          and_end_1:
            if !tmp.4 jump and_false_2
            tmp.8 = c.2 == 2
            if !tmp.8 jump and_false_2
            tmp.7 = 1
            jump and_end_3
        
          and_false_2:
            tmp.7 = 0
        
          and_end_3:
            if !tmp.7 jump and_false_4
            tmp.11 = d.3 == 1
            if !tmp.11 jump and_false_4
            tmp.10 = 1
            jump and_end_5
        
          and_false_4:
            tmp.10 = 0
        
          and_end_5:
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
            a.0 = 0
            if 0 jump or_true_0
            a.0 = 1
            if a.0 jump or_true_0
            tmp.1 = 0
            jump or_end_1
        
          or_true_0:
            tmp.1 = 1
        
          or_end_1:
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
            a.0 = 0
            if !0 jump and_false_0
            a.0 = 5
            if !a.0 jump and_false_0
            tmp.1 = 1
            jump and_end_1
        
          and_false_0:
            tmp.1 = 0
        
          and_end_1:
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
        global function main() { 
            a.0 = 0
            if 1 jump or_true_0
            a.0 = 1
            if a.0 jump or_true_0
            tmp.1 = 0
            jump or_end_1
        
          or_true_0:
            tmp.1 = 1
        
          or_end_1:
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
            if !0 jump and_false_0
            if !a.0 jump and_false_0
            tmp.1 = 1
            jump and_end_1
        
          and_false_0:
            tmp.1 = 0
        
          and_end_1:
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
fn test_chapter_6_valid_binary_condition() {
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
fn test_chapter_6_valid_binary_false_condition() {
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
fn test_chapter_6_valid_extra_credit_bitwise_ternary() {
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
            tmp.1 = result.0
            jump end_if_0
        
          else_1:
            result.0 = 5
            tmp.1 = result.0
        
          end_if_0:
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
        global function main() { 
            a.0 = 0
            tmp.0 = a.0 + 1
            a.0 = tmp.0
            if !a.0 jump end_if_0
            return a.0
        
          end_if_0:
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
        global function main() { 
            x.0 = 1
            jump post_declaration_0
            x.0 = 0
            i.1 = x.0
        
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
fn test_chapter_6_valid_extra_credit_lh_compound_assignment() {
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
            if !x.0 jump else_1
            tmp.2 = x.0 / 2
            x.0 = tmp.2
            tmp.1 = x.0
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
fn test_chapter_6_valid_extra_credit_postfix_in_ternary() {
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
fn test_chapter_6_valid_extra_credit_prefix_in_ternary() {
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
fn test_chapter_6_valid_extra_credit_unused_label() {
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
        global function main() { 
            a.0 = 0
            a.0 = 1
            if !a.0 jump end_if_0
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
        global function main() { 
            x.0 = 10
            y.1 = 0
            x.0 = 5
            if !x.0 jump else_1
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
fn test_chapter_6_valid_nested_ternary_2() {
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
        global function main() { 
            flag.0 = 1
            a.1 = 0
            if !flag.0 jump else_1
            a.1 = 1
            tmp.0 = a.1
            jump end_if_0
        
          else_1:
            a.1 = 0
            tmp.0 = a.1
        
          end_if_0:
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
fn test_chapter_6_valid_ternary_middle_assignment() {
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
            tmp.1 = a.0
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
fn test_chapter_6_valid_ternary_middle_binop() {
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
fn test_chapter_6_valid_ternary_precedence() {
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
fn test_chapter_6_valid_ternary_rh_binop() {
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
        global function main() { 
            a.0 = 1
            b.1 = 0
            if !a.0 jump else_1
            b.1 = 1
            tmp.0 = b.1
            jump end_if_0
        
          else_1:
            b.1 = 2
            tmp.0 = b.1
        
          end_if_0:
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
        global function main() { 
            a.0 = 0
            b.1 = 0
            if !a.0 jump else_1
            b.1 = 1
            tmp.0 = b.1
            jump end_if_0
        
          else_1:
            b.1 = 2
            tmp.0 = b.1
        
          end_if_0:
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
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
        global function main() { 
            x.0 = 3
            return x.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_break() {
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
fn test_chapter_8_valid_break_immediate() {
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
            if !a.0 jump break_loop_0
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
fn test_chapter_8_valid_continue() {
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
fn test_chapter_8_valid_continue_empty_post() {
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
fn test_chapter_8_valid_do_while() {
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
fn test_chapter_8_valid_do_while_break_immediate() {
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
            if a.0 jump start_loop_0
        
          break_loop_0:
            return a.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_empty_expression() {
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
fn test_chapter_8_valid_empty_loop_body() {
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
            tmp.1 = i.0 >= 256
            if tmp.1 jump start_loop_0
        
          break_loop_0:
            return i.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_8_valid_extra_credit_case_block() {
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
fn test_chapter_8_valid_extra_credit_compound_assignment_controlling_expression() {
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
            if i.0 jump start_loop_0
        
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
fn test_chapter_8_valid_extra_credit_compound_assignment_for_loop() {
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
fn test_chapter_8_valid_extra_credit_duffs_device() {
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
            tmp.3 = tmp.2 == 0
            if tmp.3 jump switch_0_case__1
            tmp.4 = tmp.2 == 4
            if tmp.4 jump switch_0_case__3
            tmp.5 = tmp.2 == 3
            if tmp.5 jump switch_0_case__4
            tmp.6 = tmp.2 == 2
            if tmp.6 jump switch_0_case__5
            tmp.7 = tmp.2 == 1
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
            tmp.14 = iterations.1 > 0
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
fn test_chapter_8_valid_extra_credit_goto_bypass_condition() {
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
fn test_chapter_8_valid_extra_credit_goto_bypass_init_exp() {
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
fn test_chapter_8_valid_extra_credit_goto_bypass_post_exp() {
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
fn test_chapter_8_valid_extra_credit_label_loop_body() {
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
fn test_chapter_8_valid_extra_credit_label_loops_breaks_and_continues() {
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
fn test_chapter_8_valid_extra_credit_loop_header_postfix_and_prefix() {
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
fn test_chapter_8_valid_extra_credit_loop_in_switch() {
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
            tmp.0 = cond.0 == 1
            if tmp.0 jump switch_0_case__1
            tmp.1 = cond.0 == 10
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
fn test_chapter_8_valid_extra_credit_post_exp_incr() {
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
fn test_chapter_8_valid_extra_credit_switch() {
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
            tmp.0 = 3 == 0
            if tmp.0 jump switch_0_case__1
            tmp.1 = 3 == 1
            if tmp.1 jump switch_0_case__2
            tmp.2 = 3 == 3
            if tmp.2 jump switch_0_case__3
            tmp.3 = 3 == 5
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
fn test_chapter_8_valid_extra_credit_switch_assign_in_condition() {
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
            tmp.0 = a.0 == 0
            if tmp.0 jump switch_0_case__1
            tmp.1 = a.0 == 1
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
fn test_chapter_8_valid_extra_credit_switch_break() {
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
            tmp.0 = a.0 == 5
            if tmp.0 jump switch_0_case__1
            tmp.1 = a.0 == 6
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
fn test_chapter_8_valid_extra_credit_switch_decl() {
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
            tmp.0 = a.0 == 3
            if tmp.0 jump switch_0_case__1
            jump break_switch_0
            b.1 = 5
            a.2 = b.1
        
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
fn test_chapter_8_valid_extra_credit_switch_default() {
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
            tmp.0 = a.0 == 1
            if tmp.0 jump switch_0_case__1
            tmp.1 = a.0 == 2
            if tmp.1 jump switch_0_case__2
            tmp.2 = a.0 == 4
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
fn test_chapter_8_valid_extra_credit_switch_default_fallthrough() {
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
            tmp.0 = 0 == 1
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
fn test_chapter_8_valid_extra_credit_switch_default_not_last() {
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
            b.1 = a.0
            tmp.0 = a.0 + b.1
            tmp.1 = tmp.0 == 2
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
fn test_chapter_8_valid_extra_credit_switch_default_only() {
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
fn test_chapter_8_valid_extra_credit_switch_empty() {
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
fn test_chapter_8_valid_extra_credit_switch_fallthrough() {
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
            tmp.1 = tmp.0 == 0
            if tmp.1 jump switch_0_case__1
            tmp.2 = tmp.0 == 7
            if tmp.2 jump switch_0_case__2
            tmp.3 = tmp.0 == 9
            if tmp.3 jump switch_0_case__3
            tmp.4 = tmp.0 == 1
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
fn test_chapter_8_valid_extra_credit_switch_goto_mid_case() {
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
fn test_chapter_8_valid_extra_credit_switch_in_loop() {
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
            tmp.1 = i.2 == 0
            if tmp.1 jump switch_1_case__2
            tmp.2 = i.2 == 1
            if tmp.2 jump switch_1_case__3
            tmp.3 = i.2 == 2
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
fn test_chapter_8_valid_extra_credit_switch_nested_cases() {
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
            tmp.0 = 3 == 0
            if tmp.0 jump switch_0_case__1
            tmp.1 = 3 == 1
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
            tmp.3 = 4 == 0
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
fn test_chapter_8_valid_extra_credit_switch_nested_not_taken() {
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
            tmp.0 = a.0 == 1
            if tmp.0 jump switch_0_case__1
            jump switch_0_default_5
            jump break_switch_0
        
          switch_0_case__1:
            tmp.1 = a.0 == 0
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
fn test_chapter_8_valid_extra_credit_switch_nested_switch() {
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
            tmp.0 = 3 == 0
            if tmp.0 jump switch_0_case__1
            tmp.1 = 3 == 3
            if tmp.1 jump switch_0_case__2
            tmp.2 = 3 == 4
            if tmp.2 jump switch_0_case__7
            jump switch_0_default_8
            jump break_switch_0
        
          switch_0_case__1:
            return 0
        
          switch_0_case__2:
            tmp.3 = 4 == 3
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
fn test_chapter_8_valid_extra_credit_switch_no_case() {
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
fn test_chapter_8_valid_extra_credit_switch_not_taken() {
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
            tmp.0 = a.0 == 0
            if tmp.0 jump switch_0_case__1
            tmp.1 = a.0 == 2
            if tmp.1 jump switch_0_case__2
            tmp.2 = a.0 == 3
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
fn test_chapter_8_valid_extra_credit_switch_single_case() {
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
            tmp.0 = a.0 == 1
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
fn test_chapter_8_valid_extra_credit_switch_with_continue() {
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
            tmp.0 = 4 == 0
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
fn test_chapter_8_valid_extra_credit_switch_with_continue_2() {
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
            tmp.2 = tmp.1 == 0
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
fn test_chapter_8_valid_for() {
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
fn test_chapter_8_valid_for_absent_condition() {
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
fn test_chapter_8_valid_for_absent_post() {
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
fn test_chapter_8_valid_for_decl() {
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
fn test_chapter_8_valid_for_nested_shadow() {
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
fn test_chapter_8_valid_for_shadow() {
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
fn test_chapter_8_valid_multi_break() {
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
fn test_chapter_8_valid_multi_continue_same_loop() {
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
fn test_chapter_8_valid_nested_break() {
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
fn test_chapter_8_valid_nested_continue() {
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
fn test_chapter_8_valid_nested_loop() {
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
fn test_chapter_8_valid_null_for_header() {
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
fn test_chapter_8_valid_while() {
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

#[test]
fn test_chapter_9_valid_arguments_in_registers_dont_clobber_edx() {
    let src = r#"
        int x(int a, int b, int c, int d, int e, int f) {
            return a == 1 && b == 2 && c == 3 && d == 4 && e == 5 && f == 6;
        }
        int main(void) {
            int a = 4;
            return x(1, 2, 3, 4, 5, 24 / a);
        }
    "#;
    let expected = r#"
        global function x(a.0, b.1, c.2, d.3, e.4, f.5) { 
            tmp.0 = a.0 == 1
            if !tmp.0 jump and_false_0
            tmp.3 = b.1 == 2
            if !tmp.3 jump and_false_0
            tmp.2 = 1
            jump and_end_1
        
          and_false_0:
            tmp.2 = 0
        
          and_end_1:
            if !tmp.2 jump and_false_2
            tmp.6 = c.2 == 3
            if !tmp.6 jump and_false_2
            tmp.5 = 1
            jump and_end_3
        
          and_false_2:
            tmp.5 = 0
        
          and_end_3:
            if !tmp.5 jump and_false_4
            tmp.9 = d.3 == 4
            if !tmp.9 jump and_false_4
            tmp.8 = 1
            jump and_end_5
        
          and_false_4:
            tmp.8 = 0
        
          and_end_5:
            if !tmp.8 jump and_false_6
            tmp.12 = e.4 == 5
            if !tmp.12 jump and_false_6
            tmp.11 = 1
            jump and_end_7
        
          and_false_6:
            tmp.11 = 0
        
          and_end_7:
            if !tmp.11 jump and_false_8
            tmp.15 = f.5 == 6
            if !tmp.15 jump and_false_8
            tmp.14 = 1
            jump and_end_9
        
          and_false_8:
            tmp.14 = 0
        
          and_end_9:
            return tmp.14
            return 0
        }
        global function main() { 
            a.6 = 4
            tmp.16 = 24 / a.6
            tmp.17 = x(1, 2, 3, 4, 5, tmp.16)
            return tmp.17
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_9_valid_arguments_in_registers_expression_args() {
    let src = r#"
        int sub(int a, int b) {
            return a - b;
        }
        int main(void) {
            int sum = sub(1 + 2, 1);
            return sum;
        }
    "#;
    let expected = r#"
        global function sub(a.0, b.1) { 
            tmp.0 = a.0 - b.1
            return tmp.0
            return 0
        }
        global function main() { 
            tmp.1 = 1 + 2
            tmp.2 = sub(tmp.1, 1)
            sum.2 = tmp.2
            return sum.2
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_9_valid_arguments_in_registers_fibonacci() {
    let src = r#"
        int fib(int n) {
            if (n == 0 || n == 1) {
                return n;
            } else {
                return fib(n - 1) + fib(n - 2);
            }
        }
        int main(void) {
            int n = 6;
            return fib(n);
        }
    "#;
    let expected = r#"
        global function fib(n.0) { 
            tmp.0 = n.0 == 0
            if tmp.0 jump or_true_0
            tmp.3 = n.0 == 1
            if tmp.3 jump or_true_0
            tmp.2 = 0
            jump or_end_1
        
          or_true_0:
            tmp.2 = 1
        
          or_end_1:
            if !tmp.2 jump else_3
            return n.0
            jump end_if_2
        
          else_3:
            tmp.4 = n.0 - 1
            tmp.5 = fib(tmp.4)
            tmp.7 = n.0 - 2
            tmp.8 = fib(tmp.7)
            tmp.6 = tmp.5 + tmp.8
            return tmp.6
        
          end_if_2:
            return 0
        }
        global function main() { 
            n.1 = 6
            tmp.9 = fib(n.1)
            return tmp.9
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_9_valid_arguments_in_registers_forward_decl_multi_arg() {
    let src = r#"
        int foo(int a, int b);
        int main(void) {
            return foo(2, 1);
        }
        int foo(int x, int y){
            return x - y;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = foo(2, 1)
            return tmp.0
            return 0
        }
        global function foo(x.2, y.3) { 
            tmp.1 = x.2 - y.3
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_9_valid_arguments_in_registers_hello_world() {
    let src = r#"
        int putchar(int c);
        int main(void) {
            putchar(72);
            putchar(101);
            putchar(108);
            putchar(108);
            putchar(111);
            putchar(44);
            putchar(32);
            putchar(87);
            putchar(111);
            putchar(114);
            putchar(108);
            putchar(100);
            putchar(33);
            putchar(10);
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = putchar(72)
            tmp.1 = putchar(101)
            tmp.2 = putchar(108)
            tmp.3 = putchar(108)
            tmp.4 = putchar(111)
            tmp.5 = putchar(44)
            tmp.6 = putchar(32)
            tmp.7 = putchar(87)
            tmp.8 = putchar(111)
            tmp.9 = putchar(114)
            tmp.10 = putchar(108)
            tmp.11 = putchar(100)
            tmp.12 = putchar(33)
            tmp.13 = putchar(10)
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_9_valid_arguments_in_registers_param_shadows_local_var() {
    let src = r#"
        int main(void) {
            int a = 10;
            int f(int a);
            return f(a);
        }
        int f(int a) {
            return a * 2;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 10
            tmp.0 = f(a.0)
            return tmp.0
            return 0
        }
        global function f(a.2) { 
            tmp.1 = a.2 * 2
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_9_valid_arguments_in_registers_parameter_shadows_function() {
    let src = r#"
        int a(void) {
            return 1;
        }
        int b(int a) {
            return a;
        }
        int main(void) {
            return a() + b(2);
        }
    "#;
    let expected = r#"
        global function a() { 
            return 1
            return 0
        }
        global function b(a.0) { 
            return a.0
            return 0
        }
        global function main() { 
            tmp.0 = a()
            tmp.2 = b(2)
            tmp.1 = tmp.0 + tmp.2
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_9_valid_arguments_in_registers_parameter_shadows_own_function() {
    let src = r#"
        int a(int a) {
            return a * 2;
        }
        int main(void) {
            return a(1);
        }
    "#;
    let expected = r#"
        global function a(a.0) { 
            tmp.0 = a.0 * 2
            return tmp.0
            return 0
        }
        global function main() { 
            tmp.1 = a(1)
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_9_valid_arguments_in_registers_parameters_are_preserved() {
    let src = r#"
        int g(int w, int x, int y, int z) {
            if (w == 2 && x == 4 && y == 6 && z == 8)
                return 1;
            return 0;
        }
        int f(int a, int b, int c, int d) {
            int result = g(a * 2, b * 2, c * 2, d * 2);
            return (result == 1 && a == 1 && b == 2 && c == 3 && d == 4);
        }
        int main(void) {
            return f(1, 2, 3, 4);
        }
    "#;
    let expected = r#"
        global function g(w.0, x.1, y.2, z.3) { 
            tmp.0 = w.0 == 2
            if !tmp.0 jump and_false_0
            tmp.3 = x.1 == 4
            if !tmp.3 jump and_false_0
            tmp.2 = 1
            jump and_end_1
        
          and_false_0:
            tmp.2 = 0
        
          and_end_1:
            if !tmp.2 jump and_false_2
            tmp.6 = y.2 == 6
            if !tmp.6 jump and_false_2
            tmp.5 = 1
            jump and_end_3
        
          and_false_2:
            tmp.5 = 0
        
          and_end_3:
            if !tmp.5 jump and_false_4
            tmp.9 = z.3 == 8
            if !tmp.9 jump and_false_4
            tmp.8 = 1
            jump and_end_5
        
          and_false_4:
            tmp.8 = 0
        
          and_end_5:
            if !tmp.8 jump end_if_6
            return 1
        
          end_if_6:
            return 0
            return 0
        }
        global function f(a.4, b.5, c.6, d.7) { 
            tmp.10 = a.4 * 2
            tmp.11 = b.5 * 2
            tmp.12 = c.6 * 2
            tmp.13 = d.7 * 2
            tmp.14 = g(tmp.10, tmp.11, tmp.12, tmp.13)
            result.8 = tmp.14
            tmp.15 = result.8 == 1
            if !tmp.15 jump and_false_8
            tmp.18 = a.4 == 1
            if !tmp.18 jump and_false_8
            tmp.17 = 1
            jump and_end_9
        
          and_false_8:
            tmp.17 = 0
        
          and_end_9:
            if !tmp.17 jump and_false_10
            tmp.21 = b.5 == 2
            if !tmp.21 jump and_false_10
            tmp.20 = 1
            jump and_end_11
        
          and_false_10:
            tmp.20 = 0
        
          and_end_11:
            if !tmp.20 jump and_false_12
            tmp.24 = c.6 == 3
            if !tmp.24 jump and_false_12
            tmp.23 = 1
            jump and_end_13
        
          and_false_12:
            tmp.23 = 0
        
          and_end_13:
            if !tmp.23 jump and_false_14
            tmp.27 = d.7 == 4
            if !tmp.27 jump and_false_14
            tmp.26 = 1
            jump and_end_15
        
          and_false_14:
            tmp.26 = 0
        
          and_end_15:
            return tmp.26
            return 0
        }
        global function main() { 
            tmp.28 = f(1, 2, 3, 4)
            return tmp.28
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_9_valid_arguments_in_registers_single_arg() {
    let src = r#"
        int twice(int x){
            return 2 * x;
        }
        int main(void) {
            return twice(3);
        }
    "#;
    let expected = r#"
        global function twice(x.0) { 
            tmp.0 = 2 * x.0
            return tmp.0
            return 0
        }
        global function main() { 
            tmp.1 = twice(3)
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_9_valid_extra_credit_compound_assign_function_result() {
    let src = r#"
        int foo(void) {
            return 2;
        }
        int main(void) {
            int x = 3;
            x -= foo();
            return x;
        }
    "#;
    let expected = r#"
        global function foo() { 
            return 2
            return 0
        }
        global function main() { 
            x.0 = 3
            tmp.1 = foo()
            tmp.0 = x.0 - tmp.1
            x.0 = tmp.0
            return x.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_9_valid_extra_credit_dont_clobber_ecx() {
    let src = r#"
        int x(int a, int b, int c, int d, int e, int f) {
            return a == 1 && b == 2 && c == 3 && d == 4 && e == 5 && f == 6;
        }
        int main(void) {
            int a = 4;
            return x(1, 2, 3, 4, 5, 24 >> (a / 2));
        }
    "#;
    let expected = r#"
        global function x(a.0, b.1, c.2, d.3, e.4, f.5) { 
            tmp.0 = a.0 == 1
            if !tmp.0 jump and_false_0
            tmp.3 = b.1 == 2
            if !tmp.3 jump and_false_0
            tmp.2 = 1
            jump and_end_1
        
          and_false_0:
            tmp.2 = 0
        
          and_end_1:
            if !tmp.2 jump and_false_2
            tmp.6 = c.2 == 3
            if !tmp.6 jump and_false_2
            tmp.5 = 1
            jump and_end_3
        
          and_false_2:
            tmp.5 = 0
        
          and_end_3:
            if !tmp.5 jump and_false_4
            tmp.9 = d.3 == 4
            if !tmp.9 jump and_false_4
            tmp.8 = 1
            jump and_end_5
        
          and_false_4:
            tmp.8 = 0
        
          and_end_5:
            if !tmp.8 jump and_false_6
            tmp.12 = e.4 == 5
            if !tmp.12 jump and_false_6
            tmp.11 = 1
            jump and_end_7
        
          and_false_6:
            tmp.11 = 0
        
          and_end_7:
            if !tmp.11 jump and_false_8
            tmp.15 = f.5 == 6
            if !tmp.15 jump and_false_8
            tmp.14 = 1
            jump and_end_9
        
          and_false_8:
            tmp.14 = 0
        
          and_end_9:
            return tmp.14
            return 0
        }
        global function main() { 
            a.6 = 4
            tmp.17 = a.6 / 2
            tmp.16 = 24 >> tmp.17
            tmp.18 = x(1, 2, 3, 4, 5, tmp.16)
            return tmp.18
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_9_valid_extra_credit_goto_label_multiple_functions() {
    let src = r#"
        
        int foo(void) {
            goto label;
            return 0;
            label:
                return 5;
        }
        int main(void) {
            goto label;
            return 0;
            label:
                return foo();
        }
    "#;
    let expected = r#"
        global function foo() { 
            jump label_0
            return 0
        
          label_0:
            return 5
            return 0
        }
        global function main() { 
            jump label_1
            return 0
        
          label_1:
            tmp.0 = foo()
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_9_valid_extra_credit_goto_shared_name() {
    let src = r#"
        int foo(void) {
            goto foo;
            return 0;
            foo:
                return 1;
        }
        int main(void) {
            return foo();
        }
    "#;
    let expected = r#"
        global function foo() { 
            jump foo_0
            return 0
        
          foo_0:
            return 1
            return 0
        }
        global function main() { 
            tmp.0 = foo()
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_9_valid_extra_credit_label_naming_scheme() {
    let src = r#"
        int main(void) {
            _label:
            label_:
            return 0;
        }
        int main_(void) {
            label:
            return 0;
        }
        int _main(void) {
            label: return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
        
          _label_0:
        
          label__1:
            return 0
            return 0
        }
        global function main_() { 
        
          label_2:
            return 0
            return 0
        }
        global function _main() { 
        
          label_3:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_9_valid_libraries_addition() {
    let src = r#"
        int add(int x, int y) {
            return x + y;
        }
    "#;
    let expected = r#"
        global function add(x.0, y.1) { 
            tmp.0 = x.0 + y.1
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_9_valid_libraries_addition_client() {
    let src = r#"
        int add(int x, int y);
        int main(void) {
            return add(1, 2);
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = add(1, 2)
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_9_valid_libraries_many_args() {
    let src = r#"
        int fib(int n) {
            if (n == 0 || n == 1) {
                return n;
            } else {
                return fib(n - 1) + fib(n - 2);
            }
        }
        int multiply_many_args(int a, int b, int c, int d, int e, int f, int g, int h) {
            return a * b * c * d * e * f * fib(g) * fib(h);
        }
    "#;
    let expected = r#"
        global function fib(n.0) { 
            tmp.0 = n.0 == 0
            if tmp.0 jump or_true_0
            tmp.3 = n.0 == 1
            if tmp.3 jump or_true_0
            tmp.2 = 0
            jump or_end_1
        
          or_true_0:
            tmp.2 = 1
        
          or_end_1:
            if !tmp.2 jump else_3
            return n.0
            jump end_if_2
        
          else_3:
            tmp.4 = n.0 - 1
            tmp.5 = fib(tmp.4)
            tmp.7 = n.0 - 2
            tmp.8 = fib(tmp.7)
            tmp.6 = tmp.5 + tmp.8
            return tmp.6
        
          end_if_2:
            return 0
        }
        global function multiply_many_args(a.1, b.2, c.3, d.4, e.5, f.6, g.7, h.8) { 
            tmp.9 = a.1 * b.2
            tmp.10 = tmp.9 * c.3
            tmp.11 = tmp.10 * d.4
            tmp.12 = tmp.11 * e.5
            tmp.13 = tmp.12 * f.6
            tmp.15 = fib(g.7)
            tmp.14 = tmp.13 * tmp.15
            tmp.17 = fib(h.8)
            tmp.16 = tmp.14 * tmp.17
            return tmp.16
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_9_valid_libraries_many_args_client() {
    let src = r#"
        int fib(int a);
        int multiply_many_args(int a, int b, int c, int d, int e, int f, int g, int h);
        int main(void) {
            int x = fib(4);
            int seven = 7;
            int eight = fib(6);
            int y = multiply_many_args(x, 2, 3, 4, 5, 6, seven, eight);
            if (x != 3) {
                return 1;
            }
            if (y != 589680) {
                return 2;
            }
            return x + (y % 256);
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = fib(4)
            x.9 = tmp.0
            seven.10 = 7
            tmp.1 = fib(6)
            eight.11 = tmp.1
            tmp.2 = multiply_many_args(x.9, 2, 3, 4, 5, 6, seven.10, eight.11)
            y.12 = tmp.2
            tmp.3 = x.9 != 3
            if !tmp.3 jump end_if_0
            return 1
        
          end_if_0:
            tmp.4 = y.12 != 589680
            if !tmp.4 jump end_if_2
            return 2
        
          end_if_2:
            tmp.6 = y.12 % 256
            tmp.5 = x.9 + tmp.6
            return tmp.5
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_9_valid_libraries_no_function_calls_division() {
    let src = r#"
        int f(int a, int b, int c, int d) {
            int x = a / b;
            if (a == 10 && b == 2 && c == 100 && d == 4 && x == 5)
                return 1;
            return 0;
        }
    "#;
    let expected = r#"
        global function f(a.0, b.1, c.2, d.3) { 
            tmp.0 = a.0 / b.1
            x.4 = tmp.0
            tmp.1 = a.0 == 10
            if !tmp.1 jump and_false_0
            tmp.4 = b.1 == 2
            if !tmp.4 jump and_false_0
            tmp.3 = 1
            jump and_end_1
        
          and_false_0:
            tmp.3 = 0
        
          and_end_1:
            if !tmp.3 jump and_false_2
            tmp.7 = c.2 == 100
            if !tmp.7 jump and_false_2
            tmp.6 = 1
            jump and_end_3
        
          and_false_2:
            tmp.6 = 0
        
          and_end_3:
            if !tmp.6 jump and_false_4
            tmp.10 = d.3 == 4
            if !tmp.10 jump and_false_4
            tmp.9 = 1
            jump and_end_5
        
          and_false_4:
            tmp.9 = 0
        
          and_end_5:
            if !tmp.9 jump and_false_6
            tmp.13 = x.4 == 5
            if !tmp.13 jump and_false_6
            tmp.12 = 1
            jump and_end_7
        
          and_false_6:
            tmp.12 = 0
        
          and_end_7:
            if !tmp.12 jump end_if_8
            return 1
        
          end_if_8:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_9_valid_libraries_no_function_calls_division_client() {
    let src = r#"
        int f(int a, int b, int c, int d);
        int main(void) {
            return f(10, 2, 100, 4);
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = f(10, 2, 100, 4)
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_9_valid_libraries_no_function_calls_local_stack_variables() {
    let src = r#"
        
        int f(int reg1, int reg2, int reg3, int reg4, int reg5, int reg6,
            int stack1, int stack2, int stack3) {
            int x = 10;
            if (reg1 == 1 && reg2 == 2 && reg3 == 3 && reg4 == 4 && reg5 == 5
                && reg6 == 6 && stack1 == -1 && stack2 == -2 && stack3 == -3
                && x == 10) {
                stack2 = 100;
                return stack2;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function f(reg1.0, reg2.1, reg3.2, reg4.3, reg5.4, reg6.5, stack1.6, stack2.7, stack3.8) { 
            x.9 = 10
            tmp.0 = reg1.0 == 1
            if !tmp.0 jump and_false_0
            tmp.3 = reg2.1 == 2
            if !tmp.3 jump and_false_0
            tmp.2 = 1
            jump and_end_1
        
          and_false_0:
            tmp.2 = 0
        
          and_end_1:
            if !tmp.2 jump and_false_2
            tmp.6 = reg3.2 == 3
            if !tmp.6 jump and_false_2
            tmp.5 = 1
            jump and_end_3
        
          and_false_2:
            tmp.5 = 0
        
          and_end_3:
            if !tmp.5 jump and_false_4
            tmp.9 = reg4.3 == 4
            if !tmp.9 jump and_false_4
            tmp.8 = 1
            jump and_end_5
        
          and_false_4:
            tmp.8 = 0
        
          and_end_5:
            if !tmp.8 jump and_false_6
            tmp.12 = reg5.4 == 5
            if !tmp.12 jump and_false_6
            tmp.11 = 1
            jump and_end_7
        
          and_false_6:
            tmp.11 = 0
        
          and_end_7:
            if !tmp.11 jump and_false_8
            tmp.15 = reg6.5 == 6
            if !tmp.15 jump and_false_8
            tmp.14 = 1
            jump and_end_9
        
          and_false_8:
            tmp.14 = 0
        
          and_end_9:
            if !tmp.14 jump and_false_10
            tmp.19 = - 1
            tmp.18 = stack1.6 == tmp.19
            if !tmp.18 jump and_false_10
            tmp.17 = 1
            jump and_end_11
        
          and_false_10:
            tmp.17 = 0
        
          and_end_11:
            if !tmp.17 jump and_false_12
            tmp.23 = - 2
            tmp.22 = stack2.7 == tmp.23
            if !tmp.22 jump and_false_12
            tmp.21 = 1
            jump and_end_13
        
          and_false_12:
            tmp.21 = 0
        
          and_end_13:
            if !tmp.21 jump and_false_14
            tmp.27 = - 3
            tmp.26 = stack3.8 == tmp.27
            if !tmp.26 jump and_false_14
            tmp.25 = 1
            jump and_end_15
        
          and_false_14:
            tmp.25 = 0
        
          and_end_15:
            if !tmp.25 jump and_false_16
            tmp.30 = x.9 == 10
            if !tmp.30 jump and_false_16
            tmp.29 = 1
            jump and_end_17
        
          and_false_16:
            tmp.29 = 0
        
          and_end_17:
            if !tmp.29 jump end_if_18
            stack2.7 = 100
            return stack2.7
        
          end_if_18:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_9_valid_libraries_no_function_calls_local_stack_variables_client() {
    let src = r#"
        int f(int reg1, int reg2, int reg3, int reg4, int reg5, int reg6,
            int stack1, int stack2, int stack3);
        int main(void) {
            return f(1, 2, 3, 4, 5, 6, -1, -2, -3);
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = - 1
            tmp.1 = - 2
            tmp.2 = - 3
            tmp.3 = f(1, 2, 3, 4, 5, 6, tmp.0, tmp.1, tmp.2)
            return tmp.3
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_9_valid_libraries_system_call() {
    let src = r#"
        int putchar(int c);
        int incr_and_print(int b) {
            return putchar(b + 2);
        }
    "#;
    let expected = r#"
        global function incr_and_print(b.1) { 
            tmp.0 = b.1 + 2
            tmp.1 = putchar(tmp.0)
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_9_valid_libraries_system_call_client() {
    let src = r#"
        int incr_and_print(int c);
        int main(void) {
            incr_and_print(70);
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = incr_and_print(70)
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_9_valid_no_arguments_forward_decl() {
    let src = r#"
        int foo(void);
        int main(void) {
            return foo();
        }
        int foo(void) {
            return 3;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = foo()
            return tmp.0
            return 0
        }
        global function foo() { 
            return 3
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_9_valid_no_arguments_function_shadows_variable() {
    let src = r#"
        int main(void) {
            int foo = 3;
            int bar = 4;
            if (foo + bar > 0) {
                int foo(void);
                bar = foo();
            }
            return foo + bar;
        }
        int foo(void) {
            return 8;
        }
    "#;
    let expected = r#"
        global function main() { 
            foo.0 = 3
            bar.1 = 4
            tmp.0 = foo.0 + bar.1
            tmp.1 = tmp.0 > 0
            if !tmp.1 jump end_if_0
            tmp.2 = foo()
            bar.1 = tmp.2
        
          end_if_0:
            tmp.3 = foo.0 + bar.1
            return tmp.3
            return 0
        }
        global function foo() { 
            return 8
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_9_valid_no_arguments_multiple_declarations() {
    let src = r#"
        int main(void) {
            int f(void);
            int f(void);
            return f();
        }
        int f(void) {
            return 3;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = f()
            return tmp.0
            return 0
        }
        global function f() { 
            return 3
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_9_valid_no_arguments_no_return_value() {
    let src = r#"
        int foo(void) {
            int x = 1;
        }
        int main(void) {
            foo();
            return 3;
        }
    "#;
    let expected = r#"
        global function foo() { 
            x.0 = 1
            return 0
        }
        global function main() { 
            tmp.0 = foo()
            return 3
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_9_valid_no_arguments_precedence() {
    let src = r#"
        int three(void) {
            return 3;
        }
        int main(void) {
            return !three();
        }
    "#;
    let expected = r#"
        global function three() { 
            return 3
            return 0
        }
        global function main() { 
            tmp.0 = three()
            tmp.1 = ! tmp.0
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_9_valid_no_arguments_use_function_in_expression() {
    let src = r#"
        int bar(void) {
            return 9;
        }
        int foo(void) {
            return 2 * bar();
        }
        int main(void) {
            return foo() + bar() / 3;
        }
    "#;
    let expected = r#"
        global function bar() { 
            return 9
            return 0
        }
        global function foo() { 
            tmp.1 = bar()
            tmp.0 = 2 * tmp.1
            return tmp.0
            return 0
        }
        global function main() { 
            tmp.2 = foo()
            tmp.4 = bar()
            tmp.5 = tmp.4 / 3
            tmp.3 = tmp.2 + tmp.5
            return tmp.3
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_9_valid_no_arguments_variable_shadows_function() {
    let src = r#"
        int main(void) {
            int foo(void);
            int x = foo();
            if (x > 0) {
                int foo = 3;
                x = x + foo;
            }
            return x;
        }
        int foo(void) {
            return 4;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = foo()
            x.0 = tmp.0
            tmp.1 = x.0 > 0
            if !tmp.1 jump end_if_0
            foo.1 = 3
            tmp.2 = x.0 + foo.1
            x.0 = tmp.2
        
          end_if_0:
            return x.0
            return 0
        }
        global function foo() { 
            return 4
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_9_valid_stack_arguments_call_putchar() {
    let src = r#"
        int putchar(int c);
        int foo(int a, int b, int c, int d, int e, int f, int g, int h) {
            putchar(h);
            return a + g;
        }
        int main(void) {
            return foo(1, 2, 3, 4, 5, 6, 7, 65);
        }
    "#;
    let expected = r#"
        global function foo(a.1, b.2, c.3, d.4, e.5, f.6, g.7, h.8) { 
            tmp.0 = putchar(h.8)
            tmp.1 = a.1 + g.7
            return tmp.1
            return 0
        }
        global function main() { 
            tmp.2 = foo(1, 2, 3, 4, 5, 6, 7, 65)
            return tmp.2
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_9_valid_stack_arguments_lots_of_arguments() {
    let src = r#"
        int foo(int a, int b, int c, int d, int e, int f, int g, int h) {
            return (a == 1 && b == 2 && c == 3 && d == 4 && e == 5
                    && f == 6 && g == 7 && h == 8);
        }
        int main(void) {
            return foo(1, 2, 3, 4, 5, 6, 7, 8);
        }
    "#;
    let expected = r#"
        global function foo(a.0, b.1, c.2, d.3, e.4, f.5, g.6, h.7) { 
            tmp.0 = a.0 == 1
            if !tmp.0 jump and_false_0
            tmp.3 = b.1 == 2
            if !tmp.3 jump and_false_0
            tmp.2 = 1
            jump and_end_1
        
          and_false_0:
            tmp.2 = 0
        
          and_end_1:
            if !tmp.2 jump and_false_2
            tmp.6 = c.2 == 3
            if !tmp.6 jump and_false_2
            tmp.5 = 1
            jump and_end_3
        
          and_false_2:
            tmp.5 = 0
        
          and_end_3:
            if !tmp.5 jump and_false_4
            tmp.9 = d.3 == 4
            if !tmp.9 jump and_false_4
            tmp.8 = 1
            jump and_end_5
        
          and_false_4:
            tmp.8 = 0
        
          and_end_5:
            if !tmp.8 jump and_false_6
            tmp.12 = e.4 == 5
            if !tmp.12 jump and_false_6
            tmp.11 = 1
            jump and_end_7
        
          and_false_6:
            tmp.11 = 0
        
          and_end_7:
            if !tmp.11 jump and_false_8
            tmp.15 = f.5 == 6
            if !tmp.15 jump and_false_8
            tmp.14 = 1
            jump and_end_9
        
          and_false_8:
            tmp.14 = 0
        
          and_end_9:
            if !tmp.14 jump and_false_10
            tmp.18 = g.6 == 7
            if !tmp.18 jump and_false_10
            tmp.17 = 1
            jump and_end_11
        
          and_false_10:
            tmp.17 = 0
        
          and_end_11:
            if !tmp.17 jump and_false_12
            tmp.21 = h.7 == 8
            if !tmp.21 jump and_false_12
            tmp.20 = 1
            jump and_end_13
        
          and_false_12:
            tmp.20 = 0
        
          and_end_13:
            return tmp.20
            return 0
        }
        global function main() { 
            tmp.22 = foo(1, 2, 3, 4, 5, 6, 7, 8)
            return tmp.22
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_9_valid_stack_arguments_stack_alignment() {
    let src = r#"
        int even_arguments(int a, int b, int c, int d, int e, int f, int g, int h);
        int odd_arguments(int a, int b, int c, int d, int e, int f, int g, int h, int i);
        int main(void) {
            int x = 3;
            even_arguments(1, 2, 3, 4, 5, 6, 7, 8);
            odd_arguments(1, 2, 3, 4, 5, 6, 7, 8, 9);
            return x;
        }
    "#;
    let expected = r#"
        global function main() { 
            x.17 = 3
            tmp.0 = even_arguments(1, 2, 3, 4, 5, 6, 7, 8)
            tmp.1 = odd_arguments(1, 2, 3, 4, 5, 6, 7, 8, 9)
            return x.17
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_9_valid_stack_arguments_test_for_memory_leaks() {
    let src = r#"
        int lots_of_args(int a, int b, int c, int d, int e, int f, int g, int h, int i, int j, int k, int l, int m, int n, int o) {
            return l + o;
        }
        int main(void) {
            int ret = 0;
            for (int i = 0; i < 10000000; i = i + 1) {
                ret = lots_of_args(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, ret, 13, 14, 15);
            }
            return ret == 150000000;
        }
    "#;
    let expected = r#"
        global function lots_of_args(a.0, b.1, c.2, d.3, e.4, f.5, g.6, h.7, i.8, j.9, k.10, l.11, m.12, n.13, o.14) { 
            tmp.0 = l.11 + o.14
            return tmp.0
            return 0
        }
        global function main() { 
            ret.15 = 0
            i.16 = 0
        
          start_loop_0:
            tmp.1 = i.16 < 10000000
            if !tmp.1 jump break_loop_0
            tmp.2 = lots_of_args(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, ret.15, 13, 14, 15)
            ret.15 = tmp.2
        
          continue_loop_0:
            tmp.3 = i.16 + 1
            i.16 = tmp.3
            jump start_loop_0
        
          break_loop_0:
            tmp.4 = ret.15 == 150000000
            return tmp.4
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_10_valid_distinct_local_and_extern() {
    let src = r#"
        int a = 5;
        int return_a(void) {
            return a;
        }
        int main(void) {
            int a = 3;
            {
                extern int a;
                if (a != 5)
                    return 1;
                a = 4;
            }
            return a + return_a();
        }
    "#;
    let expected = r#"
        global function return_a() { 
            return a
            return 0
        }
        global function main() { 
            a.0 = 3
            tmp.0 = a != 5
            if !tmp.0 jump end_if_0
            return 1
        
          end_if_0:
            a = 4
            tmp.2 = return_a()
            tmp.1 = a.0 + tmp.2
            return tmp.1
            return 0
        }
        static global a: Int = 5
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_10_valid_extern_block_scope_variable() {
    let src = r#"
        int main(void) {
            int outer = 1;
            int foo = 0;
            if (outer) {
                extern int foo;
                extern int foo;
                return foo;
            }
            return 0;
        }
        int foo = 3;
    "#;
    let expected = r#"
        global function main() { 
            outer.0 = 1
            foo.1 = 0
            if !outer.0 jump end_if_0
            return foo
        
          end_if_0:
            return 0
            return 0
        }
        static global foo: Int = 3
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_10_valid_extra_credit_bitwise_ops_file_scope_vars() {
    let src = r#"
        int x = 1;
        int y = 0;
        int main(void) {
            y = -1;
            x = (x << 1) | 1;
            if (x != 3) {
                return 1;
            }
            y = ((y & -5) ^ 12) >> 2;
            if (y != -3) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = - 1
            y = tmp.0
            tmp.1 = x << 1
            tmp.2 = tmp.1 | 1
            x = tmp.2
            tmp.3 = x != 3
            if !tmp.3 jump end_if_0
            return 1
        
          end_if_0:
            tmp.5 = - 5
            tmp.4 = y & tmp.5
            tmp.6 = tmp.4 ^ 12
            tmp.7 = tmp.6 >> 2
            y = tmp.7
            tmp.9 = - 3
            tmp.8 = y != tmp.9
            if !tmp.8 jump end_if_2
            return 2
        
          end_if_2:
            return 0
            return 0
        }
        static global x: Int = 1
        static global y: Int = 0
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_10_valid_extra_credit_compound_assignment_static_var() {
    let src = r#"
        int f(void) {
            static int i = 0;
            static int j = 0;
            static int k = 1;
            static int l = 48;
            i += 1;
            j -= i;
            k *= j;
            l /= 2;
            if (i != 3) {
                return 1;
            }
            if (j != -6) {
                return 2;
            }
            if (k != -18) {
                return 3;
            }
            if (l != 6) {
                return 4;
            }
            return 0;
        }
        int main(void) {
            f();
            f();
            return f();
        }
    "#;
    let expected = r#"
        global function f() { 
            tmp.0 = i.0 + 1
            i.0 = tmp.0
            tmp.1 = j.1 - i.0
            j.1 = tmp.1
            tmp.2 = k.2 * j.1
            k.2 = tmp.2
            tmp.3 = l.3 / 2
            l.3 = tmp.3
            tmp.4 = i.0 != 3
            if !tmp.4 jump end_if_0
            return 1
        
          end_if_0:
            tmp.6 = - 6
            tmp.5 = j.1 != tmp.6
            if !tmp.5 jump end_if_2
            return 2
        
          end_if_2:
            tmp.8 = - 18
            tmp.7 = k.2 != tmp.8
            if !tmp.7 jump end_if_4
            return 3
        
          end_if_4:
            tmp.9 = l.3 != 6
            if !tmp.9 jump end_if_6
            return 4
        
          end_if_6:
            return 0
            return 0
        }
        global function main() { 
            tmp.10 = f()
            tmp.11 = f()
            tmp.12 = f()
            return tmp.12
            return 0
        }
        static i.0: Int = 0
        static j.1: Int = 0
        static k.2: Int = 1
        static l.3: Int = 48
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_10_valid_extra_credit_goto_skip_static_initializer() {
    let src = r#"
        int main(void) {
            goto end;
            static int x = 10;
            end:
                return x;
        }
    "#;
    let expected = r#"
        global function main() { 
            jump end_0
        
          end_0:
            return x.0
            return 0
        }
        static x.0: Int = 10
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_10_valid_extra_credit_increment_global_vars() {
    let src = r#"
        
        int i = 0;
        int j = 0;
        int incr_i(void){
            if (i == 1) {
                i++;
                ++i;
            }
            return 0;
        }
        int decr_j(void) {
            if (j == -1) {
                j--;
            }
            return 0;
        }
        int main(void) {
            i++ ? 0 : incr_i();
            if (i != 3) {
                return 1;
            }
            --j? decr_j(): 0;
            if (j != -2) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function incr_i() { 
            tmp.0 = i == 1
            if !tmp.0 jump end_if_0
            tmp.1 = i
            tmp.2 = inc i
            i = tmp.2
            tmp.3 = inc i
            i = tmp.3
        
          end_if_0:
            return 0
            return 0
        }
        global function decr_j() { 
            tmp.5 = - 1
            tmp.4 = j == tmp.5
            if !tmp.4 jump end_if_2
            tmp.6 = j
            tmp.7 = dec j
            j = tmp.7
        
          end_if_2:
            return 0
            return 0
        }
        global function main() { 
            tmp.8 = i
            tmp.9 = inc i
            i = tmp.9
            if !tmp.8 jump else_5
            tmp.10 = 0
            jump end_if_4
        
          else_5:
            tmp.11 = incr_i()
            tmp.10 = tmp.11
        
          end_if_4:
            tmp.12 = i != 3
            if !tmp.12 jump end_if_6
            return 1
        
          end_if_6:
            tmp.13 = dec j
            j = tmp.13
            if !tmp.13 jump else_9
            tmp.15 = decr_j()
            tmp.14 = tmp.15
            jump end_if_8
        
          else_9:
            tmp.14 = 0
        
          end_if_8:
            tmp.17 = - 2
            tmp.16 = j != tmp.17
            if !tmp.16 jump end_if_10
            return 2
        
          end_if_10:
            return 0
            return 0
        }
        static global i: Int = 0
        static global j: Int = 0
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_10_valid_extra_credit_label_file_scope_var_same_name() {
    let src = r#"
        int x;
        int main(void) {
            int x = 10;
            goto x;
            return x;
            {
                extern int x;
            x:
                return x;
            }
        }
    "#;
    let expected = r#"
        global function main() { 
            x.0 = 10
            jump x_0
            return x.0
        
          x_0:
            return x
            return 0
        }
        static global x: Int = 0
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_10_valid_extra_credit_label_static_var_same_name() {
    let src = r#"
        int main(void) {
            static int x = 5;
            goto x;
            x = 0;
        x:
            return x;
        }
    "#;
    let expected = r#"
        global function main() { 
            jump x_0
            x.0 = 0
        
          x_0:
            return x.0
            return 0
        }
        static x.0: Int = 5
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_10_valid_extra_credit_libraries_same_label_same_fun() {
    let src = r#"
        static int f(void) {
            goto x;
            return 0;
            x:
            return 2;
        }
        int f_caller(void) {
            return f();
        }
    "#;
    let expected = r#"
        function f() { 
            jump x_0
            return 0
        
          x_0:
            return 2
            return 0
        }
        global function f_caller() { 
            tmp.0 = f()
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_10_valid_extra_credit_libraries_same_label_same_fun_client() {
    let src = r#"
        int f(void) {
            goto x;
            return 0;
        x:
            return 1;
        }
        int f_caller(void);
        int main(void) {
            if (f() != 1) {
                return 1;
            }
            if (f_caller() !=
                2) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function f() { 
            jump x_0
            return 0
        
          x_0:
            return 1
            return 0
        }
        global function main() { 
            tmp.0 = f()
            tmp.1 = tmp.0 != 1
            if !tmp.1 jump end_if_0
            return 1
        
          end_if_0:
            tmp.2 = f_caller()
            tmp.3 = tmp.2 != 2
            if !tmp.3 jump end_if_2
            return 2
        
          end_if_2:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_10_valid_extra_credit_switch_on_extern() {
    let src = r#"
        int update_x(void);
        int main(void) {
            update_x();
            extern int x;
            switch(x) {
                case 0: return 1;
                case 1: return 2;
                case 4: return 0;
                default: return 4;
            }
        }
        int x;
        int update_x(void) {
            x = 4;
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = update_x()
            tmp.1 = x == 0
            if tmp.1 jump switch_0_case__1
            tmp.2 = x == 1
            if tmp.2 jump switch_0_case__2
            tmp.3 = x == 4
            if tmp.3 jump switch_0_case__3
            jump switch_0_default_4
            jump break_switch_0
        
          switch_0_case__1:
            return 1
        
          switch_0_case__2:
            return 2
        
          switch_0_case__3:
            return 0
        
          switch_0_default_4:
            return 4
        
          break_switch_0:
            return 0
        }
        global function update_x() { 
            x = 4
            return 0
            return 0
        }
        static global x: Int = 0
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_10_valid_extra_credit_switch_skip_extern_decl() {
    let src = r#"
        int main(void) {
            int a = 10;
            switch(a) {
                case 1: return 1;
                extern int x;
                case 2: return 2;
                case 10:
                if (x * 2 == 30) {
                    return 0;
                }
                default: return 5;
            }
            return 6;
        }
        int x = 15;
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 10
            tmp.0 = a.0 == 1
            if tmp.0 jump switch_0_case__1
            tmp.1 = a.0 == 2
            if tmp.1 jump switch_0_case__2
            tmp.2 = a.0 == 10
            if tmp.2 jump switch_0_case__3
            jump switch_0_default_4
            jump break_switch_0
        
          switch_0_case__1:
            return 1
        
          switch_0_case__2:
            return 2
        
          switch_0_case__3:
            tmp.3 = x * 2
            tmp.4 = tmp.3 == 30
            if !tmp.4 jump end_if_0
            return 0
        
          end_if_0:
        
          switch_0_default_4:
            return 5
        
          break_switch_0:
            return 6
            return 0
        }
        static global x: Int = 15
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_10_valid_extra_credit_switch_skip_static_initializer() {
    let src = r#"
        int a = 3;
        int main(void) {
            switch (a) {
                case 1:;
                    static int x = 10;
                    x = 0;
                case 3:
                    return x;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = a == 1
            if tmp.0 jump switch_0_case__1
            tmp.1 = a == 3
            if tmp.1 jump switch_0_case__2
            jump break_switch_0
        
          switch_0_case__1:
            x.0 = 0
        
          switch_0_case__2:
            return x.0
        
          break_switch_0:
            return 0
            return 0
        }
        static global a: Int = 3
        static x.0: Int = 10
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_10_valid_libraries_external_linkage_function() {
    let src = r#"
        extern int sum(int a, int b);
        int sum(int i, int j) {
            return i + j;
        }
        int sum(int x, int y);
    "#;
    let expected = r#"
        global function sum(i.2, j.3) { 
            tmp.0 = i.2 + j.3
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_10_valid_libraries_external_linkage_function_client() {
    let src = r#"
        int add_one_and_two(void) {
            extern int sum(int a, int b);
            int sum(int a, int b);
            return sum(1, 2);
        }
        extern int sum(int x, int y);
        int sum(int x, int y);
        int add_three_and_four(void) {
            int f = 3;
            if (f > 2) {
                extern int sum(int one, int two);
                return sum(3, 4);
            }
            return 1;
        }
        int main(void) {
            if (add_three_and_four() != 7)
                return 1;
            if (add_one_and_two() != 3)
                return 1;
            return 0;
        }
    "#;
    let expected = r#"
        global function add_one_and_two() { 
            tmp.0 = sum(1, 2)
            return tmp.0
            return 0
        }
        global function add_three_and_four() { 
            f.8 = 3
            tmp.1 = f.8 > 2
            if !tmp.1 jump end_if_0
            tmp.2 = sum(3, 4)
            return tmp.2
        
          end_if_0:
            return 1
            return 0
        }
        global function main() { 
            tmp.3 = add_three_and_four()
            tmp.4 = tmp.3 != 7
            if !tmp.4 jump end_if_2
            return 1
        
          end_if_2:
            tmp.5 = add_one_and_two()
            tmp.6 = tmp.5 != 3
            if !tmp.6 jump end_if_4
            return 1
        
          end_if_4:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_10_valid_libraries_external_tentative_var() {
    let src = r#"
        
        int x;
        int read_x(void) {
            return x;
        }
    "#;
    let expected = r#"
        global function read_x() { 
            return x
            return 0
        }
        static global x: Int = 0
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_10_valid_libraries_external_tentative_var_client() {
    let src = r#"
        int read_x(void);
        int main(void) {
            extern int x;
            if (x != 0)
                return 1;
            x = 3;
            if (read_x() != 3)
                return 1;
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = x != 0
            if !tmp.0 jump end_if_0
            return 1
        
          end_if_0:
            x = 3
            tmp.1 = read_x()
            tmp.2 = tmp.1 != 3
            if !tmp.2 jump end_if_2
            return 1
        
          end_if_2:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_10_valid_libraries_external_var_scoping() {
    let src = r#"
        int read_x(void) {
            int x = 4;
            if (x == 4) {
                extern int x;
                return x;
            } else {
                return -1;
            }
        }
    "#;
    let expected = r#"
        global function read_x() { 
            x.0 = 4
            tmp.0 = x.0 == 4
            if !tmp.0 jump else_1
            return x
            jump end_if_0
        
          else_1:
            tmp.1 = - 1
            return tmp.1
        
          end_if_0:
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_10_valid_libraries_external_var_scoping_client() {
    let src = r#"
        int x = 10;
        int read_x(void);
        int main(void) {
            int x = 0;
            if (x == 0) {
                if (read_x() != 10)
                    return 1;
                extern int x;
                if (x != 10)
                    return 1;
                return 0;
            }
            return 1;
        }
    "#;
    let expected = r#"
        global function main() { 
            x.0 = 0
            tmp.0 = x.0 == 0
            if !tmp.0 jump end_if_0
            tmp.1 = read_x()
            tmp.2 = tmp.1 != 10
            if !tmp.2 jump end_if_2
            return 1
        
          end_if_2:
            tmp.3 = x != 10
            if !tmp.3 jump end_if_4
            return 1
        
          end_if_4:
            return 0
        
          end_if_0:
            return 1
            return 0
        }
        static global x: Int = 10
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_10_valid_libraries_external_variable() {
    let src = r#"
        int x;
        extern int x;
        int x;
        int update_x(int new_val) {
            x = new_val;
            return 0;
        }
        int read_x(void) {
            return x;
        }
        int x = 3;
    "#;
    let expected = r#"
        global function update_x(new_val.0) { 
            x = new_val.0
            return 0
            return 0
        }
        global function read_x() { 
            return x
            return 0
        }
        static global x: Int = 3
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_10_valid_libraries_external_variable_client() {
    let src = r#"
        int update_x(int new_val);
        int read_x(void);
        extern int x;
        int main(void) {
            if (x != 3)
                return 1;
            if (read_x() != 3)
                return 1;
            x = 4;
            if (x != 4)
                return 1;
            if (read_x() != 4)
                return 1;
            update_x(5);
            if (x != 5)
                return 1;
            if (read_x() != 5)
                return 1;
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = x != 3
            if !tmp.0 jump end_if_0
            return 1
        
          end_if_0:
            tmp.1 = read_x()
            tmp.2 = tmp.1 != 3
            if !tmp.2 jump end_if_2
            return 1
        
          end_if_2:
            x = 4
            tmp.3 = x != 4
            if !tmp.3 jump end_if_4
            return 1
        
          end_if_4:
            tmp.4 = read_x()
            tmp.5 = tmp.4 != 4
            if !tmp.5 jump end_if_6
            return 1
        
          end_if_6:
            tmp.6 = update_x(5)
            tmp.7 = x != 5
            if !tmp.7 jump end_if_8
            return 1
        
          end_if_8:
            tmp.8 = read_x()
            tmp.9 = tmp.8 != 5
            if !tmp.9 jump end_if_10
            return 1
        
          end_if_10:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_10_valid_libraries_internal_hides_external_linkage() {
    let src = r#"
        int x = 10;
        int read_x(void){
            return x;
        }
    "#;
    let expected = r#"
        global function read_x() { 
            return x
            return 0
        }
        static global x: Int = 10
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_10_valid_libraries_internal_hides_external_linkage_client() {
    let src = r#"
        static int x = 1;
        int read_internal_x(void);
        int read_x(void);
        int main(void) {
            extern int x;
            if (x != 1)
                return 1;
            x = 2;
            if (read_internal_x() != 2)
                return 1;
            if (read_x() != 10)
                return 1;
            return 0;
        }
        extern int x;
        int read_internal_x(void) {
            return x;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = x != 1
            if !tmp.0 jump end_if_0
            return 1
        
          end_if_0:
            x = 2
            tmp.1 = read_internal_x()
            tmp.2 = tmp.1 != 2
            if !tmp.2 jump end_if_2
            return 1
        
          end_if_2:
            tmp.3 = read_x()
            tmp.4 = tmp.3 != 10
            if !tmp.4 jump end_if_4
            return 1
        
          end_if_4:
            return 0
            return 0
        }
        global function read_internal_x() { 
            return x
            return 0
        }
        static x: Int = 1
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_10_valid_libraries_internal_linkage_function() {
    let src = r#"
        
        static int my_fun(void);
        int call_static_my_fun(void) {
            return my_fun();
        }
        int call_static_my_fun_2(void) {
            int my_fun(void);
            return my_fun();
        }
        extern int my_fun(void);
        static int my_fun(void);
        int my_fun(void) {
            static int i = 0;
            i = i + 1;
            return i;
        }
    "#;
    let expected = r#"
        global function call_static_my_fun() { 
            tmp.0 = my_fun()
            return tmp.0
            return 0
        }
        global function call_static_my_fun_2() { 
            tmp.1 = my_fun()
            return tmp.1
            return 0
        }
        function my_fun() { 
            tmp.2 = i.0 + 1
            i.0 = tmp.2
            return i.0
            return 0
        }
        static i.0: Int = 0
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_10_valid_libraries_internal_linkage_function_client() {
    let src = r#"
        extern int my_fun(void);
        int call_static_my_fun(void);
        int call_static_my_fun_2(void);
        int main(void) {
            if (call_static_my_fun() != 1)
                return 1;
            if (my_fun() != 100)
                return 1;
            if (call_static_my_fun_2() != 2)
                return 1;
            return 0;
        }
        int my_fun(void) {
            return 100;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = call_static_my_fun()
            tmp.1 = tmp.0 != 1
            if !tmp.1 jump end_if_0
            return 1
        
          end_if_0:
            tmp.2 = my_fun()
            tmp.3 = tmp.2 != 100
            if !tmp.3 jump end_if_2
            return 1
        
          end_if_2:
            tmp.4 = call_static_my_fun_2()
            tmp.5 = tmp.4 != 2
            if !tmp.5 jump end_if_4
            return 1
        
          end_if_4:
            return 0
            return 0
        }
        global function my_fun() { 
            return 100
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_10_valid_libraries_internal_linkage_var() {
    let src = r#"
        static int x;
        int read_x(void) {
            return x;
        }
        int update_x(int new_val) {
            extern int x;
            x = new_val;
            return 0;
        }
        extern int x;
        static int x = 5;
        static int x;
    "#;
    let expected = r#"
        global function read_x() { 
            return x
            return 0
        }
        global function update_x(new_val.0) { 
            x = new_val.0
            return 0
            return 0
        }
        static x: Int = 5
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_10_valid_libraries_internal_linkage_var_client() {
    let src = r#"
        static int x;
        static int x;
        int read_x(void);
        int update_x(int x);
        int main(void) {
            if (x != 0)
                return 1;
            if (read_x() != 5)
                return 1;
            extern int x;
            update_x(10);
            if (read_x() != 10)
                return 1;
            if (x != 0)
                return 1;
            x = 20;
            if (x != 20)
                return 1;
            if (read_x() != 10)
                return 1;
            return 0;
        }
        static int x;
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = x != 0
            if !tmp.0 jump end_if_0
            return 1
        
          end_if_0:
            tmp.1 = read_x()
            tmp.2 = tmp.1 != 5
            if !tmp.2 jump end_if_2
            return 1
        
          end_if_2:
            tmp.3 = update_x(10)
            tmp.4 = read_x()
            tmp.5 = tmp.4 != 10
            if !tmp.5 jump end_if_4
            return 1
        
          end_if_4:
            tmp.6 = x != 0
            if !tmp.6 jump end_if_6
            return 1
        
          end_if_6:
            x = 20
            tmp.7 = x != 20
            if !tmp.7 jump end_if_8
            return 1
        
          end_if_8:
            tmp.8 = read_x()
            tmp.9 = tmp.8 != 10
            if !tmp.9 jump end_if_10
            return 1
        
          end_if_10:
            return 0
            return 0
        }
        static x: Int = 0
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_10_valid_multiple_static_file_scope_vars() {
    let src = r#"
        static int foo;
        int main(void) {
            return foo;
        }
        extern int foo;
        static int foo = 4;
    "#;
    let expected = r#"
        global function main() { 
            return foo
            return 0
        }
        static foo: Int = 4
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_10_valid_multiple_static_local() {
    let src = r#"
        int foo(void) {
            static int a = 3;
            a = a * 2;
            return a;
        }
        int bar(void) {
            static int a = 4;
            a = a + 1;
            return a;
        }
        int main(void) {
            return foo() + bar() + foo() + bar();
        }
    "#;
    let expected = r#"
        global function foo() { 
            tmp.0 = a.0 * 2
            a.0 = tmp.0
            return a.0
            return 0
        }
        global function bar() { 
            tmp.1 = a.1 + 1
            a.1 = tmp.1
            return a.1
            return 0
        }
        global function main() { 
            tmp.2 = foo()
            tmp.4 = bar()
            tmp.3 = tmp.2 + tmp.4
            tmp.6 = foo()
            tmp.5 = tmp.3 + tmp.6
            tmp.8 = bar()
            tmp.7 = tmp.5 + tmp.8
            return tmp.7
            return 0
        }
        static a.0: Int = 3
        static a.1: Int = 4
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_10_valid_push_arg_on_page_boundary() {
    let src = r#"
        extern int zed;
        int foo(int a, int b, int c, int d, int e, int f, int g) {
            return g + 1;
        }
        int main(void) {
            return foo(0, 0, 0, 0, 0, 0, zed);
        }
    "#;
    let expected = r#"
        global function foo(a.0, b.1, c.2, d.3, e.4, f.5, g.6) { 
            tmp.0 = g.6 + 1
            return tmp.0
            return 0
        }
        global function main() { 
            tmp.1 = foo(0, 0, 0, 0, 0, 0, zed)
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_10_valid_shadow_static_local_var() {
    let src = r#"
        int i;
        int update_static_or_global(int update_global, int new_val)
        {
            static int i;
            if (update_global)
            {
                extern int i;
                i = new_val;
            }
            else
                i = new_val;
            return i;
        }
        int main(void)
        {
            if (i != 0)
                return 1;
            int result = update_static_or_global(1, 10);
            if (result != 0)
                return 1;
            if (i != 10)
                return 1;
            result = update_static_or_global(0, 9);
            if (result != 9)
                return 1;
            if (i != 10)
                return 1;
            result = update_static_or_global(1, 11);
            if (result != 9)
                return 1;
            if (i != 11)
                return 1;
            return 0;
        }
    "#;
    let expected = r#"
        global function update_static_or_global(update_global.0, new_val.1) { 
            if !update_global.0 jump else_1
            i = new_val.1
            jump end_if_0
        
          else_1:
            i.2 = new_val.1
        
          end_if_0:
            return i.2
            return 0
        }
        global function main() { 
            tmp.0 = i != 0
            if !tmp.0 jump end_if_2
            return 1
        
          end_if_2:
            tmp.1 = update_static_or_global(1, 10)
            result.4 = tmp.1
            tmp.2 = result.4 != 0
            if !tmp.2 jump end_if_4
            return 1
        
          end_if_4:
            tmp.3 = i != 10
            if !tmp.3 jump end_if_6
            return 1
        
          end_if_6:
            tmp.4 = update_static_or_global(0, 9)
            result.4 = tmp.4
            tmp.5 = result.4 != 9
            if !tmp.5 jump end_if_8
            return 1
        
          end_if_8:
            tmp.6 = i != 10
            if !tmp.6 jump end_if_10
            return 1
        
          end_if_10:
            tmp.7 = update_static_or_global(1, 11)
            result.4 = tmp.7
            tmp.8 = result.4 != 9
            if !tmp.8 jump end_if_12
            return 1
        
          end_if_12:
            tmp.9 = i != 11
            if !tmp.9 jump end_if_14
            return 1
        
          end_if_14:
            return 0
            return 0
        }
        static global i: Int = 0
        static i.2: Int = 0
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_10_valid_static_local_multiple_scopes() {
    let src = r#"
        int putchar (int ch);
        int print_letters(void) {
            static int i = 65;
            putchar(i);
            {
                i = i + 1;
                static int i = 97;
                putchar(i);
                i = i + 1;
            }
            putchar(10);
            return 0;
        }
        int main(void) {
            for (int i = 0; i < 26; i = i + 1)
                print_letters();
        }
    "#;
    let expected = r#"
        global function print_letters() { 
            tmp.0 = putchar(i.1)
            tmp.1 = i.1 + 1
            i.1 = tmp.1
            tmp.2 = putchar(i.2)
            tmp.3 = i.2 + 1
            i.2 = tmp.3
            tmp.4 = putchar(10)
            return 0
            return 0
        }
        global function main() { 
            i.3 = 0
        
          start_loop_0:
            tmp.5 = i.3 < 26
            if !tmp.5 jump break_loop_0
            tmp.6 = print_letters()
        
          continue_loop_0:
            tmp.7 = i.3 + 1
            i.3 = tmp.7
            jump start_loop_0
        
          break_loop_0:
            return 0
        }
        static i.1: Int = 65
        static i.2: Int = 97
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_10_valid_static_local_uninitialized() {
    let src = r#"
        int foo(void) {
            static int x;
            x = x + 1;
            return x;
        }
        int main(void) {
            int ret;
            for (int i = 0; i < 4; i = i + 1)
                ret = foo();
            return ret;
        }
    "#;
    let expected = r#"
        global function foo() { 
            tmp.0 = x.0 + 1
            x.0 = tmp.0
            return x.0
            return 0
        }
        global function main() { 
            i.2 = 0
        
          start_loop_0:
            tmp.1 = i.2 < 4
            if !tmp.1 jump break_loop_0
            tmp.2 = foo()
            ret.1 = tmp.2
        
          continue_loop_0:
            tmp.3 = i.2 + 1
            i.2 = tmp.3
            jump start_loop_0
        
          break_loop_0:
            return ret.1
            return 0
        }
        static x.0: Int = 0
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_10_valid_static_recursive_call() {
    let src = r#"
        int putchar (int ch);
        int print_alphabet(void) {
            static int count = 0;
            putchar(count + 65);
            count = count + 1;
            if (count < 26) {
                print_alphabet();
            }
            return count;
        }
        int main(void) {
            print_alphabet();
        }
    "#;
    let expected = r#"
        global function print_alphabet() { 
            tmp.0 = count.1 + 65
            tmp.1 = putchar(tmp.0)
            tmp.2 = count.1 + 1
            count.1 = tmp.2
            tmp.3 = count.1 < 26
            if !tmp.3 jump end_if_0
            tmp.4 = print_alphabet()
        
          end_if_0:
            return count.1
            return 0
        }
        global function main() { 
            tmp.5 = print_alphabet()
            return 0
        }
        static count.1: Int = 0
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_10_valid_static_then_extern() {
    let src = r#"
        static int foo = 3;
        int main(void) {
            return foo;
        }
        extern int foo;
    "#;
    let expected = r#"
        global function main() { 
            return foo
            return 0
        }
        static foo: Int = 3
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_10_valid_static_variables_in_expressions() {
    let src = r#"
        int main(void) {
            static int i = 2;
            static int j = 3;
            int cmp = i < j;
            if (!cmp)
                return 1;
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = i.0 < j.1
            cmp.2 = tmp.0
            tmp.1 = ! cmp.2
            if !tmp.1 jump end_if_0
            return 1
        
          end_if_0:
            return 0
            return 0
        }
        static i.0: Int = 2
        static j.1: Int = 3
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_10_valid_tentative_definition() {
    let src = r#"
        extern int foo;
        int foo;
        int foo;
        int main(void) {
            for (int i = 0; i < 5; i = i + 1)
                foo = foo + 1;
            return foo;
        }
        int foo;
    "#;
    let expected = r#"
        global function main() { 
            i.0 = 0
        
          start_loop_0:
            tmp.0 = i.0 < 5
            if !tmp.0 jump break_loop_0
            tmp.1 = foo + 1
            foo = tmp.1
        
          continue_loop_0:
            tmp.2 = i.0 + 1
            i.0 = tmp.2
            jump start_loop_0
        
          break_loop_0:
            return foo
            return 0
        }
        static global foo: Int = 0
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_10_valid_type_before_storage_class() {
    let src = r#"
        int static foo(void) {
            return 3;
        }
        int static bar = 4;
        int main(void) {
            int extern foo(void);
            int extern bar;
            return foo() + bar;
        }
    "#;
    let expected = r#"
        function foo() { 
            return 3
            return 0
        }
        global function main() { 
            tmp.0 = foo()
            tmp.1 = tmp.0 + bar
            return tmp.1
            return 0
        }
        static bar: Int = 4
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_11_valid_explicit_casts_sign_extend() {
    let src = r#"
        long sign_extend(int i, long expected) {
            long extended = (long) i;
            return (extended == expected);
        }
        int main(void) {
            if (!sign_extend(10, 10l)) {
                return 1;
            }
            if (!sign_extend(-10, -10l)) {
                return 2;
            }
            long l = (long) 100;
            if (l != 100l) {
                return 3;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function sign_extend(i.0, expected.1) { 
            tmp.0 = sign_extend i.0
            extended.2 = tmp.0
            tmp.1 = extended.2 == expected.1
            tmp.2 = sign_extend tmp.1
            return tmp.2
            return 0
        }
        global function main() { 
            tmp.3 = sign_extend(10, 10L)
            tmp.4 = ! tmp.3
            if !tmp.4 jump end_if_0
            return 1
        
          end_if_0:
            tmp.5 = - 10
            tmp.6 = - 10L
            tmp.7 = sign_extend(tmp.5, tmp.6)
            tmp.8 = ! tmp.7
            if !tmp.8 jump end_if_2
            return 2
        
          end_if_2:
            tmp.9 = sign_extend 100
            l.3 = tmp.9
            tmp.10 = l.3 != 100L
            if !tmp.10 jump end_if_4
            return 3
        
          end_if_4:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_11_valid_explicit_casts_truncate() {
    let src = r#"
        int truncate(long l, int expected) {
            int result = (int) l;
            return (result == expected);
        }
        int main(void)
        {
            if (!truncate(10l, 10)) {
                return 1;
            }
            if (!truncate(-10l, -10)) {
                return 2;
            }
            if (!truncate(17179869189l,
                          5)) {
                return 3;
            }
            if (!truncate(-17179869179l,
                          5l)) {
                return 4;
            }
            int i = (int)17179869189l;
            if (i != 5)
                return 5;
            return 0;
        }
    "#;
    let expected = r#"
        global function truncate(l.0, expected.1) { 
            tmp.0 = truncate l.0
            result.2 = tmp.0
            tmp.1 = result.2 == expected.1
            return tmp.1
            return 0
        }
        global function main() { 
            tmp.2 = truncate(10L, 10)
            tmp.3 = ! tmp.2
            if !tmp.3 jump end_if_0
            return 1
        
          end_if_0:
            tmp.4 = - 10L
            tmp.5 = - 10
            tmp.6 = truncate(tmp.4, tmp.5)
            tmp.7 = ! tmp.6
            if !tmp.7 jump end_if_2
            return 2
        
          end_if_2:
            tmp.8 = truncate(17179869189L, 5)
            tmp.9 = ! tmp.8
            if !tmp.9 jump end_if_4
            return 3
        
          end_if_4:
            tmp.10 = - 17179869179L
            tmp.11 = truncate 5L
            tmp.12 = truncate(tmp.10, tmp.11)
            tmp.13 = ! tmp.12
            if !tmp.13 jump end_if_6
            return 4
        
          end_if_6:
            tmp.14 = truncate 17179869189L
            i.3 = tmp.14
            tmp.15 = i.3 != 5
            if !tmp.15 jump end_if_8
            return 5
        
          end_if_8:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_11_valid_extra_credit_bitshift() {
    let src = r#"
        int main(void) {
            long l = 137438953472l;
            int shiftcount = 2;
            if (l >> shiftcount != 34359738368l ) {
                return 1;
            }
            if (l << shiftcount != 549755813888 ) {
                return 2;
            }
            if (l << 2 != 549755813888 ) {
                return 3;
            }
            if ((40l << 40) != 43980465111040l) {
                return 4;
            }
            long long_shiftcount = 3l;
            int i_neighbor1 = 0;
            int i = -2147483645;
            int i_neighbor2 = 0;
            if (i >> long_shiftcount != -268435456) {
                return 5;
            }
            i = -1;
            if (i >> 10l != -1) {
                return 6;
            }
            if (i_neighbor1) {
                return 7;
            }
            if (i_neighbor2) {
                return 8;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            l.0 = 137438953472L
            shiftcount.1 = 2
            tmp.0 = l.0 >> shiftcount.1
            tmp.1 = tmp.0 != 34359738368L
            if !tmp.1 jump end_if_0
            return 1
        
          end_if_0:
            tmp.2 = l.0 << shiftcount.1
            tmp.3 = tmp.2 != 549755813888L
            if !tmp.3 jump end_if_2
            return 2
        
          end_if_2:
            tmp.4 = l.0 << 2
            tmp.5 = tmp.4 != 549755813888L
            if !tmp.5 jump end_if_4
            return 3
        
          end_if_4:
            tmp.6 = 40L << 40
            tmp.7 = tmp.6 != 43980465111040L
            if !tmp.7 jump end_if_6
            return 4
        
          end_if_6:
            long_shiftcount.2 = 3L
            i_neighbor1.3 = 0
            tmp.8 = - 2147483645
            i.4 = tmp.8
            i_neighbor2.5 = 0
            tmp.9 = i.4 >> long_shiftcount.2
            tmp.11 = - 268435456
            tmp.10 = tmp.9 != tmp.11
            if !tmp.10 jump end_if_8
            return 5
        
          end_if_8:
            tmp.12 = - 1
            i.4 = tmp.12
            tmp.13 = i.4 >> 10L
            tmp.15 = - 1
            tmp.14 = tmp.13 != tmp.15
            if !tmp.14 jump end_if_10
            return 6
        
          end_if_10:
            if !i_neighbor1.3 jump end_if_12
            return 7
        
          end_if_12:
            if !i_neighbor2.5 jump end_if_14
            return 8
        
          end_if_14:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_11_valid_extra_credit_bitwise_long_op() {
    let src = r#"
        int main(void) {
            long l1 = 71777214294589695l;
            long l2 = -4294967296;
            if ((l1 & l2) != 71777214277877760l ) {
                return 1;
            }
            if ((l1 | l2) != -4278255361 ) {
                return 2;
            }
            if ((l1 ^ l2) != -71777218556133121 ) {
                return 3;
            }
            if ((-1l & 34359738368l) != 34359738368l) {
                return 4;
            }
            if ((0l | 34359738368l) != 34359738368l) {
                return 5;
            }
            if ((34359738368l ^ 137438953472l) != 171798691840l) {
                return 6;
            }
            long l = 4611686018427387903l;
            int i = -1073741824;
            int i2 = -1;
            if ((i & l) != 4611686017353646080l) {
                return 7;
            }
            if ((l | i) != -1) {
                return 8;
            }
            if ((l ^ i) != -4611686017353646081) {
                return 9;
            }
            if ((i2 ^ 4611686018427387903l) != ~4611686018427387903l) {
                return 10;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            l1.0 = 71777214294589695L
            tmp.0 = - 4294967296L
            l2.1 = tmp.0
            tmp.1 = l1.0 & l2.1
            tmp.2 = tmp.1 != 71777214277877760L
            if !tmp.2 jump end_if_0
            return 1
        
          end_if_0:
            tmp.3 = l1.0 | l2.1
            tmp.5 = - 4278255361L
            tmp.4 = tmp.3 != tmp.5
            if !tmp.4 jump end_if_2
            return 2
        
          end_if_2:
            tmp.6 = l1.0 ^ l2.1
            tmp.8 = - 71777218556133121L
            tmp.7 = tmp.6 != tmp.8
            if !tmp.7 jump end_if_4
            return 3
        
          end_if_4:
            tmp.9 = - 1L
            tmp.10 = tmp.9 & 34359738368L
            tmp.11 = tmp.10 != 34359738368L
            if !tmp.11 jump end_if_6
            return 4
        
          end_if_6:
            tmp.12 = 0L | 34359738368L
            tmp.13 = tmp.12 != 34359738368L
            if !tmp.13 jump end_if_8
            return 5
        
          end_if_8:
            tmp.14 = 34359738368L ^ 137438953472L
            tmp.15 = tmp.14 != 171798691840L
            if !tmp.15 jump end_if_10
            return 6
        
          end_if_10:
            l.2 = 4611686018427387903L
            tmp.16 = - 1073741824
            i.3 = tmp.16
            tmp.17 = - 1
            i2.4 = tmp.17
            tmp.18 = sign_extend i.3
            tmp.19 = tmp.18 & l.2
            tmp.20 = tmp.19 != 4611686017353646080L
            if !tmp.20 jump end_if_12
            return 7
        
          end_if_12:
            tmp.22 = sign_extend i.3
            tmp.21 = l.2 | tmp.22
            tmp.24 = - 1
            tmp.25 = sign_extend tmp.24
            tmp.23 = tmp.21 != tmp.25
            if !tmp.23 jump end_if_14
            return 8
        
          end_if_14:
            tmp.27 = sign_extend i.3
            tmp.26 = l.2 ^ tmp.27
            tmp.29 = - 4611686017353646081L
            tmp.28 = tmp.26 != tmp.29
            if !tmp.28 jump end_if_16
            return 9
        
          end_if_16:
            tmp.30 = sign_extend i2.4
            tmp.31 = tmp.30 ^ 4611686018427387903L
            tmp.33 = ~ 4611686018427387903L
            tmp.32 = tmp.31 != tmp.33
            if !tmp.32 jump end_if_18
            return 10
        
          end_if_18:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_11_valid_extra_credit_compound_assign_to_int() {
    let src = r#"
        int main(void) {
            int i = -20;
            int b = 2147483647;
            int c = -5000000;
            i += 2147483648l;
            if (i != 2147483628) {
                return 1;
            }
            if (b != 2147483647) {
                return 2;
            }
            b /= -34359738367l;
            if (b) {
                return 3;
            }
            if (i != 2147483628) {
                return 4;
            }
            if (c != -5000000) {
                return 5;
            }
            c *= 10000l;
            if (c != 1539607552) {
                return 6;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = - 20
            i.0 = tmp.0
            b.1 = 2147483647
            tmp.1 = - 5000000
            c.2 = tmp.1
            tmp.2 = sign_extend i.0
            tmp.3 = tmp.2 + 2147483648L
            tmp.4 = truncate tmp.3
            i.0 = tmp.4
            tmp.5 = truncate i.0
            tmp.6 = i.0 != 2147483628
            if !tmp.6 jump end_if_0
            return 1
        
          end_if_0:
            tmp.7 = b.1 != 2147483647
            if !tmp.7 jump end_if_2
            return 2
        
          end_if_2:
            tmp.8 = sign_extend b.1
            tmp.10 = - 34359738367L
            tmp.9 = tmp.8 / tmp.10
            tmp.11 = truncate tmp.9
            b.1 = tmp.11
            tmp.12 = truncate b.1
            if !b.1 jump end_if_4
            return 3
        
          end_if_4:
            tmp.13 = i.0 != 2147483628
            if !tmp.13 jump end_if_6
            return 4
        
          end_if_6:
            tmp.15 = - 5000000
            tmp.14 = c.2 != tmp.15
            if !tmp.14 jump end_if_8
            return 5
        
          end_if_8:
            tmp.16 = sign_extend c.2
            tmp.17 = tmp.16 * 10000L
            tmp.18 = truncate tmp.17
            c.2 = tmp.18
            tmp.19 = truncate c.2
            tmp.20 = c.2 != 1539607552
            if !tmp.20 jump end_if_10
            return 6
        
          end_if_10:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_11_valid_extra_credit_compound_assign_to_long() {
    let src = r#"
        int main(void) {
            long l = -34359738368l;
            int i = -10;
            l -= i;
            if (l != -34359738358l) {
                return 1;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = - 34359738368L
            l.0 = tmp.0
            tmp.1 = - 10
            i.1 = tmp.1
            tmp.3 = sign_extend i.1
            tmp.2 = l.0 - tmp.3
            l.0 = tmp.2
            tmp.5 = - 34359738358L
            tmp.4 = l.0 != tmp.5
            if !tmp.4 jump end_if_0
            return 1
        
          end_if_0:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_11_valid_extra_credit_compound_bitshift() {
    let src = r#"
        int main(void) {
            int x = 100;
            x <<= 22l;
            if (x != 419430400) {
                return 1;
            }
            if ((x >>= 4l) != 26214400) {
                return 2;
            }
            if (x != 26214400) {
                return 3;
            }
            long l = 12345l;
            if ((l <<= 33) != 106042742538240l) {
                return 4;
            }
            l = -l;
            if ((l >>= 10) != -103557365760l) {
                return 5;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            x.0 = 100
            tmp.1 = truncate 22L
            tmp.0 = x.0 << tmp.1
            x.0 = tmp.0
            tmp.2 = x.0 != 419430400
            if !tmp.2 jump end_if_0
            return 1
        
          end_if_0:
            tmp.4 = truncate 4L
            tmp.3 = x.0 >> tmp.4
            x.0 = tmp.3
            tmp.5 = x.0 != 26214400
            if !tmp.5 jump end_if_2
            return 2
        
          end_if_2:
            tmp.6 = x.0 != 26214400
            if !tmp.6 jump end_if_4
            return 3
        
          end_if_4:
            l.1 = 12345L
            tmp.8 = sign_extend 33
            tmp.7 = l.1 << tmp.8
            l.1 = tmp.7
            tmp.9 = l.1 != 106042742538240L
            if !tmp.9 jump end_if_6
            return 4
        
          end_if_6:
            tmp.10 = - l.1
            l.1 = tmp.10
            tmp.12 = sign_extend 10
            tmp.11 = l.1 >> tmp.12
            l.1 = tmp.11
            tmp.14 = - 103557365760L
            tmp.13 = l.1 != tmp.14
            if !tmp.13 jump end_if_8
            return 5
        
          end_if_8:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_11_valid_extra_credit_compound_bitwise() {
    let src = r#"
        int main(void) {
            long l1 = 71777214294589695l;
            long l2 = -4294967296;
            l1 &= l2;
            if (l1 != 71777214277877760l) {
                return 1;
            }
            l2 |= 100l;
            if (l2 != -4294967196) {
                return 2;
            }
            l1 ^= -9223372036854775807l;
            if (l1 != -9151594822576898047l ) {
                return 3;
            }
            l1 = 4611686018427387903l;
            int i = -1073741824;
            l1 &= i;
            if (l1 != 4611686017353646080l) {
                return 4;
            }
            i = -2147483648l;
            if ((i |= 71777214294589695l) != -2130771713) {
                return 5;
            }
            if (i != -2130771713) {
                return 6;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            l1.0 = 71777214294589695L
            tmp.0 = - 4294967296L
            l2.1 = tmp.0
            tmp.1 = l1.0 & l2.1
            l1.0 = tmp.1
            tmp.2 = l1.0 != 71777214277877760L
            if !tmp.2 jump end_if_0
            return 1
        
          end_if_0:
            tmp.3 = l2.1 | 100L
            l2.1 = tmp.3
            tmp.5 = - 4294967196L
            tmp.4 = l2.1 != tmp.5
            if !tmp.4 jump end_if_2
            return 2
        
          end_if_2:
            tmp.7 = - 9223372036854775807L
            tmp.6 = l1.0 ^ tmp.7
            l1.0 = tmp.6
            tmp.9 = - 9151594822576898047L
            tmp.8 = l1.0 != tmp.9
            if !tmp.8 jump end_if_4
            return 3
        
          end_if_4:
            l1.0 = 4611686018427387903L
            tmp.10 = - 1073741824
            i.2 = tmp.10
            tmp.12 = sign_extend i.2
            tmp.11 = l1.0 & tmp.12
            l1.0 = tmp.11
            tmp.13 = l1.0 != 4611686017353646080L
            if !tmp.13 jump end_if_6
            return 4
        
          end_if_6:
            tmp.14 = - 2147483648L
            tmp.15 = truncate tmp.14
            i.2 = tmp.15
            tmp.16 = sign_extend i.2
            tmp.17 = tmp.16 | 71777214294589695L
            tmp.18 = truncate tmp.17
            i.2 = tmp.18
            tmp.19 = truncate i.2
            tmp.21 = - 2130771713
            tmp.22 = sign_extend tmp.21
            tmp.20 = tmp.19 != tmp.22
            if !tmp.20 jump end_if_8
            return 5
        
          end_if_8:
            tmp.24 = - 2130771713
            tmp.23 = i.2 != tmp.24
            if !tmp.23 jump end_if_10
            return 6
        
          end_if_10:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_11_valid_extra_credit_increment_long() {
    let src = r#"
        
        int main(void) {
            long x = -9223372036854775807l;
            if (x++ != -9223372036854775807l) {
                return 1;
            }
            if (x != -9223372036854775806l) {
                return 2;
            }
            if (--x != -9223372036854775807l) {
                return 3;
            }
            if (x != -9223372036854775807l) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = - 9223372036854775807L
            x.0 = tmp.0
            tmp.1 = x.0
            tmp.2 = inc x.0
            x.0 = tmp.2
            tmp.4 = - 9223372036854775807L
            tmp.3 = tmp.1 != tmp.4
            if !tmp.3 jump end_if_0
            return 1
        
          end_if_0:
            tmp.6 = - 9223372036854775806L
            tmp.5 = x.0 != tmp.6
            if !tmp.5 jump end_if_2
            return 2
        
          end_if_2:
            tmp.7 = dec x.0
            x.0 = tmp.7
            tmp.9 = - 9223372036854775807L
            tmp.8 = tmp.7 != tmp.9
            if !tmp.8 jump end_if_4
            return 3
        
          end_if_4:
            tmp.11 = - 9223372036854775807L
            tmp.10 = x.0 != tmp.11
            if !tmp.10 jump end_if_6
            return 4
        
          end_if_6:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_11_valid_extra_credit_switch_int() {
    let src = r#"
        int switch_on_int(int i) {
            switch(i) {
                case 5:
                    return 0;
                case 8589934592l:
                    return 1;
                case 34359738367:
                    return 2;
                default:
                    return 3;
            }
        }
        int main(void) {
            if (switch_on_int(5) != 0)
                return 1;
            if (switch_on_int(0) != 1)
                return 2;
            if (switch_on_int(-1) != 2)
                return 3;
            if (switch_on_int(17179869184) != 1)
                return 4;
            return 0;
        }
    "#;
    let expected = r#"
        global function switch_on_int(i.0) { 
            tmp.0 = i.0 == 5
            if tmp.0 jump switch_0_case__1
            tmp.1 = i.0 == 0
            if tmp.1 jump switch_0_case__2
            tmp.2 = i.0 == -1
            if tmp.2 jump switch_0_case__3
            jump switch_0_default_4
            jump break_switch_0
        
          switch_0_case__1:
            return 0
        
          switch_0_case__2:
            return 1
        
          switch_0_case__3:
            return 2
        
          switch_0_default_4:
            return 3
        
          break_switch_0:
            return 0
        }
        global function main() { 
            tmp.3 = switch_on_int(5)
            tmp.4 = tmp.3 != 0
            if !tmp.4 jump end_if_0
            return 1
        
          end_if_0:
            tmp.5 = switch_on_int(0)
            tmp.6 = tmp.5 != 1
            if !tmp.6 jump end_if_2
            return 2
        
          end_if_2:
            tmp.7 = - 1
            tmp.8 = switch_on_int(tmp.7)
            tmp.9 = tmp.8 != 2
            if !tmp.9 jump end_if_4
            return 3
        
          end_if_4:
            tmp.10 = truncate 17179869184L
            tmp.11 = switch_on_int(tmp.10)
            tmp.12 = tmp.11 != 1
            if !tmp.12 jump end_if_6
            return 4
        
          end_if_6:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_11_valid_extra_credit_switch_long() {
    let src = r#"
        int switch_on_long(long l) {
            switch (l) {
                case 0: return 0;
                case 100: return 1;
                case 8589934592l:
                    return 2;
                default:
                    return -1;
            }
        }
        int main(void) {
            if (switch_on_long(8589934592) != 2)
                return 1;
            if (switch_on_long(100) != 1)
                return 2;
            return 0;
        }
    "#;
    let expected = r#"
        global function switch_on_long(l.0) { 
            tmp.0 = l.0 == 0L
            if tmp.0 jump switch_0_case__1
            tmp.1 = l.0 == 100L
            if tmp.1 jump switch_0_case__2
            tmp.2 = l.0 == 8589934592L
            if tmp.2 jump switch_0_case__3
            jump switch_0_default_4
            jump break_switch_0
        
          switch_0_case__1:
            return 0
        
          switch_0_case__2:
            return 1
        
          switch_0_case__3:
            return 2
        
          switch_0_default_4:
            tmp.3 = - 1
            return tmp.3
        
          break_switch_0:
            return 0
        }
        global function main() { 
            tmp.4 = switch_on_long(8589934592L)
            tmp.5 = tmp.4 != 2
            if !tmp.5 jump end_if_0
            return 1
        
          end_if_0:
            tmp.6 = sign_extend 100
            tmp.7 = switch_on_long(tmp.6)
            tmp.8 = tmp.7 != 1
            if !tmp.8 jump end_if_2
            return 2
        
          end_if_2:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_11_valid_implicit_casts_common_type() {
    let src = r#"
        long l;
        int i;
        int addition(void) {
            long result = i + l;
            return (result == 2147483663l);
        }
        int division(void) {
            int int_result = l / i;
            return (int_result == 214748364);
        }
        int comparison(void) {
            return (i <= l);
        }
        int conditional(void) {
            long result = 1 ? l : i;
            return (result == 8589934592l);
        }
        int main(void) {
            l = 2147483653;
            i = 10;
            if (!addition()) {
                return 1;
            }
            l = 2147483649l;
            if (!division()) {
                return 2;
            }
            i = -100;
            l = 4294967296;
            if (!comparison()) {
                return 3;
            }
            l = 8589934592l;
            i = 10;
            if (!conditional()) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function addition() { 
            tmp.0 = sign_extend i
            tmp.1 = tmp.0 + l
            result.0 = tmp.1
            tmp.2 = result.0 == 2147483663L
            return tmp.2
            return 0
        }
        global function division() { 
            tmp.4 = sign_extend i
            tmp.3 = l / tmp.4
            tmp.5 = truncate tmp.3
            int_result.1 = tmp.5
            tmp.6 = int_result.1 == 214748364
            return tmp.6
            return 0
        }
        global function comparison() { 
            tmp.7 = sign_extend i
            tmp.8 = tmp.7 <= l
            return tmp.8
            return 0
        }
        global function conditional() { 
            if !1 jump else_1
            tmp.9 = l
            jump end_if_0
        
          else_1:
            tmp.10 = sign_extend i
            tmp.9 = tmp.10
        
          end_if_0:
            result.2 = tmp.9
            tmp.11 = result.2 == 8589934592L
            return tmp.11
            return 0
        }
        global function main() { 
            l = 2147483653L
            i = 10
            tmp.12 = addition()
            tmp.13 = ! tmp.12
            if !tmp.13 jump end_if_2
            return 1
        
          end_if_2:
            l = 2147483649L
            tmp.14 = division()
            tmp.15 = ! tmp.14
            if !tmp.15 jump end_if_4
            return 2
        
          end_if_4:
            tmp.16 = - 100
            i = tmp.16
            l = 4294967296L
            tmp.17 = comparison()
            tmp.18 = ! tmp.17
            if !tmp.18 jump end_if_6
            return 3
        
          end_if_6:
            l = 8589934592L
            i = 10
            tmp.19 = conditional()
            tmp.20 = ! tmp.19
            if !tmp.20 jump end_if_8
            return 4
        
          end_if_8:
            return 0
            return 0
        }
        static global i: Int = 0
        static global l: Long = 0L
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_11_valid_implicit_casts_convert_by_assignment() {
    let src = r#"
        int return_truncated_long(long l) {
            return l;
        }
        long return_extended_int(int i) {
            return i;
        }
        int truncate_on_assignment(long l, int expected) {
            int result = l;
            return result == expected;
        }
        int main(void) {
            long result = return_truncated_long(4294967298l);
            if (result != 2l) {
                return 1;
            }
            result = return_extended_int(-10);
            if (result != -10) {
                return 2;
            }
            int i = 4294967298l;
            if (i != 2) {
                return 3;
            }
            if (!truncate_on_assignment(17179869184l, 0)) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function return_truncated_long(l.0) { 
            tmp.0 = truncate l.0
            return tmp.0
            return 0
        }
        global function return_extended_int(i.1) { 
            tmp.1 = sign_extend i.1
            return tmp.1
            return 0
        }
        global function truncate_on_assignment(l.2, expected.3) { 
            tmp.2 = truncate l.2
            result.4 = tmp.2
            tmp.3 = result.4 == expected.3
            return tmp.3
            return 0
        }
        global function main() { 
            tmp.4 = return_truncated_long(4294967298L)
            tmp.5 = sign_extend tmp.4
            result.5 = tmp.5
            tmp.6 = result.5 != 2L
            if !tmp.6 jump end_if_0
            return 1
        
          end_if_0:
            tmp.7 = - 10
            tmp.8 = return_extended_int(tmp.7)
            result.5 = tmp.8
            tmp.10 = - 10
            tmp.11 = sign_extend tmp.10
            tmp.9 = result.5 != tmp.11
            if !tmp.9 jump end_if_2
            return 2
        
          end_if_2:
            tmp.12 = truncate 4294967298L
            i.6 = tmp.12
            tmp.13 = i.6 != 2
            if !tmp.13 jump end_if_4
            return 3
        
          end_if_4:
            tmp.14 = truncate_on_assignment(17179869184L, 0)
            tmp.15 = ! tmp.14
            if !tmp.15 jump end_if_6
            return 4
        
          end_if_6:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_11_valid_implicit_casts_convert_function_arguments() {
    let src = r#"
        int foo(long a, int b, int c, int d, long e, int f, long g, int h) {
            if (a != -1l)
                return 1;
            if (b != 2)
                return 2;
            if (c != 0)
                return 3;
            if (d != -5)
                return 4;
            if (e != -101l)
                return 5;
            if (f != -123)
                return 6;
            if (g != -10l)
                return 7;
            if (h != 1234)
                return 8;
            return 0;
        }
        int main(void) {
            int a = -1;
            long int b = 4294967298;
            long c = -4294967296;
            long d =
                21474836475;
            int e = -101;
            long f = -123;
            int g = -10;
            long h = -9223372036854774574;
            return foo(a, b, c, d, e, f, g, h);
        }
    "#;
    let expected = r#"
        global function foo(a.0, b.1, c.2, d.3, e.4, f.5, g.6, h.7) { 
            tmp.1 = - 1L
            tmp.0 = a.0 != tmp.1
            if !tmp.0 jump end_if_0
            return 1
        
          end_if_0:
            tmp.2 = b.1 != 2
            if !tmp.2 jump end_if_2
            return 2
        
          end_if_2:
            tmp.3 = c.2 != 0
            if !tmp.3 jump end_if_4
            return 3
        
          end_if_4:
            tmp.5 = - 5
            tmp.4 = d.3 != tmp.5
            if !tmp.4 jump end_if_6
            return 4
        
          end_if_6:
            tmp.7 = - 101L
            tmp.6 = e.4 != tmp.7
            if !tmp.6 jump end_if_8
            return 5
        
          end_if_8:
            tmp.9 = - 123
            tmp.8 = f.5 != tmp.9
            if !tmp.8 jump end_if_10
            return 6
        
          end_if_10:
            tmp.11 = - 10L
            tmp.10 = g.6 != tmp.11
            if !tmp.10 jump end_if_12
            return 7
        
          end_if_12:
            tmp.12 = h.7 != 1234
            if !tmp.12 jump end_if_14
            return 8
        
          end_if_14:
            return 0
            return 0
        }
        global function main() { 
            tmp.13 = - 1
            a.8 = tmp.13
            b.9 = 4294967298L
            tmp.14 = - 4294967296L
            c.10 = tmp.14
            d.11 = 21474836475L
            tmp.15 = - 101
            e.12 = tmp.15
            tmp.16 = - 123
            tmp.17 = sign_extend tmp.16
            f.13 = tmp.17
            tmp.18 = - 10
            g.14 = tmp.18
            tmp.19 = - 9223372036854774574L
            h.15 = tmp.19
            tmp.20 = sign_extend a.8
            tmp.21 = truncate b.9
            tmp.22 = truncate c.10
            tmp.23 = truncate d.11
            tmp.24 = sign_extend e.12
            tmp.25 = truncate f.13
            tmp.26 = sign_extend g.14
            tmp.27 = truncate h.15
            tmp.28 = foo(tmp.20, tmp.21, tmp.22, tmp.23, tmp.24, tmp.25, tmp.26, tmp.27)
            return tmp.28
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_11_valid_implicit_casts_convert_static_initializer() {
    let src = r#"
        int i = 8589934592l;
        long j = 123456;
        int main(void) {
            if (i != 0) {
                return 1;
            }
            if (j != 123456l) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = i != 0
            if !tmp.0 jump end_if_0
            return 1
        
          end_if_0:
            tmp.1 = j != 123456L
            if !tmp.1 jump end_if_2
            return 2
        
          end_if_2:
            return 0
            return 0
        }
        static global i: Int = 0
        static global j: Long = 123456L
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_11_valid_implicit_casts_long_constants() {
    let src = r#"
        int main(void) {
            if (2147483647l + 2147483647l < 0l) {
                return 1;
            }
            if (17179869184 < 100l) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 2147483647L + 2147483647L
            tmp.1 = tmp.0 < 0L
            if !tmp.1 jump end_if_0
            return 1
        
          end_if_0:
            tmp.2 = 17179869184L < 100L
            if !tmp.2 jump end_if_2
            return 2
        
          end_if_2:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_11_valid_libraries_long_args() {
    let src = r#"
        int test_sum(int a, int b, int c, long d, int e, long f, int g, int h, long i) {
            if (d + f < 100l) {
                return 1;
            }
            if (i < 100l)
                return 2;
            return 0;
        }
    "#;
    let expected = r#"
        global function test_sum(a.0, b.1, c.2, d.3, e.4, f.5, g.6, h.7, i.8) { 
            tmp.0 = d.3 + f.5
            tmp.1 = tmp.0 < 100L
            if !tmp.1 jump end_if_0
            return 1
        
          end_if_0:
            tmp.2 = i.8 < 100L
            if !tmp.2 jump end_if_2
            return 2
        
          end_if_2:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_11_valid_libraries_long_args_client() {
    let src = r#"
        int test_sum(int a, int b, int c, long d, int e, long f, int g, int h, long i);
        int main(void) {
            return test_sum(0, 0, 0, 34359738368l, 0, 34359738368l, 0, 0, 34359738368l);
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = test_sum(0, 0, 0, 34359738368L, 0, 34359738368L, 0, 0, 34359738368L)
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_11_valid_libraries_long_global_var() {
    let src = r#"
        long int l = 8589934592l;
        long return_l(void) {
            return l;
        }
        int return_l_as_int(void) {
            return l;
        }
    "#;
    let expected = r#"
        global function return_l() { 
            return l
            return 0
        }
        global function return_l_as_int() { 
            tmp.0 = truncate l
            return tmp.0
            return 0
        }
        static global l: Long = 8589934592L
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_11_valid_libraries_long_global_var_client() {
    let src = r#"
        extern long int l;
        long return_l(void);
        int return_l_as_int(void);
        int main(void) {
            if (return_l() != 8589934592l)
                return 1;
            if (return_l_as_int() != 0)
                return 2;
            l = l - 10l;
            if (return_l() != 8589934582l)
                return 3;
            if (return_l_as_int() != -10)
                return 4;
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = return_l()
            tmp.1 = tmp.0 != 8589934592L
            if !tmp.1 jump end_if_0
            return 1
        
          end_if_0:
            tmp.2 = return_l_as_int()
            tmp.3 = tmp.2 != 0
            if !tmp.3 jump end_if_2
            return 2
        
          end_if_2:
            tmp.4 = l - 10L
            l = tmp.4
            tmp.5 = return_l()
            tmp.6 = tmp.5 != 8589934582L
            if !tmp.6 jump end_if_4
            return 3
        
          end_if_4:
            tmp.7 = return_l_as_int()
            tmp.9 = - 10
            tmp.8 = tmp.7 != tmp.9
            if !tmp.8 jump end_if_6
            return 4
        
          end_if_6:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_11_valid_libraries_maintain_stack_alignment() {
    let src = r#"
        long add_variables(long x, long y, int z){
            return x + y + z;
        }
    "#;
    let expected = r#"
        global function add_variables(x.0, y.1, z.2) { 
            tmp.0 = x.0 + y.1
            tmp.2 = sign_extend z.2
            tmp.1 = tmp.0 + tmp.2
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_11_valid_libraries_maintain_stack_alignment_client() {
    let src = r#"
        long add_variables(long x, long y, int z);
        int main(void) {
            long x = 3;
            long y = 4;
            int z = 5;
            return add_variables(x, y, z);
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = sign_extend 3
            x.3 = tmp.0
            tmp.1 = sign_extend 4
            y.4 = tmp.1
            z.5 = 5
            tmp.2 = add_variables(x.3, y.4, z.5)
            tmp.3 = truncate tmp.2
            return tmp.3
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_11_valid_libraries_return_long() {
    let src = r#"
        long add(int a, int b) {
            return (long) a + (long) b;
        }
    "#;
    let expected = r#"
        global function add(a.0, b.1) { 
            tmp.0 = sign_extend a.0
            tmp.2 = sign_extend b.1
            tmp.1 = tmp.0 + tmp.2
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_11_valid_libraries_return_long_client() {
    let src = r#"
        long add(int a, int b);
        int main(void) {
            long a = add(2147483645, 2147483645);
            if (a != 4294967290l) {
                return 1;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = add(2147483645, 2147483645)
            a.2 = tmp.0
            tmp.1 = a.2 != 4294967290L
            if !tmp.1 jump end_if_0
            return 1
        
          end_if_0:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_11_valid_long_expressions_arithmetic_ops() {
    let src = r#"
        long a;
        long b;
        int addition(void) {
            return (a + b == 4294967295l);
        }
        int subtraction(void) {
            return (a - b == -4294967380l);
        }
        int multiplication(void) {
            return (a * 4l == 17179869160l);
        }
        int division(void) {
            b = a / 128l;
            return (b == 33554431l);
        }
        int remaind(void) {
            b = -a % 4294967290l;
            return (b == -5l);
        }
        int complement(void) {
            return (~a == -9223372036854775807l);
        }
        int main(void) {
            a = 4294967290l;
            b = 5l;
            if (!addition()) {
                return 1;
            }
            a = -4294967290l;
            b = 90l;
            if (!subtraction()) {
                return 2;
            }
            a = 4294967290l;
            if (!multiplication()) {
                return 3;
            }
            a = 4294967290l;
            if (!division()) {
                return 4;
            }
            a = 8589934585l;
            if (!remaind()) {
                return 5;
            }
            a = 9223372036854775806l;
            if (!complement()) {
                return 6;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function addition() { 
            tmp.0 = a + b
            tmp.1 = tmp.0 == 4294967295L
            return tmp.1
            return 0
        }
        global function subtraction() { 
            tmp.2 = a - b
            tmp.4 = - 4294967380L
            tmp.3 = tmp.2 == tmp.4
            return tmp.3
            return 0
        }
        global function multiplication() { 
            tmp.5 = a * 4L
            tmp.6 = tmp.5 == 17179869160L
            return tmp.6
            return 0
        }
        global function division() { 
            tmp.7 = a / 128L
            b = tmp.7
            tmp.8 = b == 33554431L
            return tmp.8
            return 0
        }
        global function remaind() { 
            tmp.9 = - a
            tmp.10 = tmp.9 % 4294967290L
            b = tmp.10
            tmp.12 = - 5L
            tmp.11 = b == tmp.12
            return tmp.11
            return 0
        }
        global function complement() { 
            tmp.13 = ~ a
            tmp.15 = - 9223372036854775807L
            tmp.14 = tmp.13 == tmp.15
            return tmp.14
            return 0
        }
        global function main() { 
            a = 4294967290L
            b = 5L
            tmp.16 = addition()
            tmp.17 = ! tmp.16
            if !tmp.17 jump end_if_0
            return 1
        
          end_if_0:
            tmp.18 = - 4294967290L
            a = tmp.18
            b = 90L
            tmp.19 = subtraction()
            tmp.20 = ! tmp.19
            if !tmp.20 jump end_if_2
            return 2
        
          end_if_2:
            a = 4294967290L
            tmp.21 = multiplication()
            tmp.22 = ! tmp.21
            if !tmp.22 jump end_if_4
            return 3
        
          end_if_4:
            a = 4294967290L
            tmp.23 = division()
            tmp.24 = ! tmp.23
            if !tmp.24 jump end_if_6
            return 4
        
          end_if_6:
            a = 8589934585L
            tmp.25 = remaind()
            tmp.26 = ! tmp.25
            if !tmp.26 jump end_if_8
            return 5
        
          end_if_8:
            a = 9223372036854775806L
            tmp.27 = complement()
            tmp.28 = ! tmp.27
            if !tmp.28 jump end_if_10
            return 6
        
          end_if_10:
            return 0
            return 0
        }
        static global a: Long = 0L
        static global b: Long = 0L
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_11_valid_long_expressions_assign() {
    let src = r#"
        int main(void) {
            long a = 4294967290l;
            long b = 0l;
            b = a;
            return (b == 4294967290l);
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 4294967290L
            b.1 = 0L
            b.1 = a.0
            tmp.0 = b.1 == 4294967290L
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_11_valid_long_expressions_comparisons() {
    let src = r#"
        long l;
        long l2;
        int compare_constants(void) {
            return 8589934593l > 255l;
        }
        int compare_constants_2(void) {
            return 255l < 8589934593l;
        }
        int l_geq_2_60(void) {
            return (l >= 1152921504606846976l);
        }
        int uint_max_leq_l(void) {
            return (4294967295l <= l);
        }
        int l_eq_l2(void) {
            return (l == l2);
        }
        int main(void) {
            if (!compare_constants()) {
                return 1;
            }
            if (!compare_constants_2()) {
                return 2;
            }
            l = -9223372036854775807l;
            if (l_geq_2_60()) {
                return 3;
            }
            if (uint_max_leq_l()) {
                return 4;
            }
            l = 1152921504606846976l;
            if (!l_geq_2_60()) {
                return 5;
            }
            if (!uint_max_leq_l()) {
                return 6;
            }
            l2 = l;
            if (!l_eq_l2()) {
                return 7;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function compare_constants() { 
            tmp.0 = 8589934593L > 255L
            return tmp.0
            return 0
        }
        global function compare_constants_2() { 
            tmp.1 = 255L < 8589934593L
            return tmp.1
            return 0
        }
        global function l_geq_2_60() { 
            tmp.2 = l >= 1152921504606846976L
            return tmp.2
            return 0
        }
        global function uint_max_leq_l() { 
            tmp.3 = 4294967295L <= l
            return tmp.3
            return 0
        }
        global function l_eq_l2() { 
            tmp.4 = l == l2
            return tmp.4
            return 0
        }
        global function main() { 
            tmp.5 = compare_constants()
            tmp.6 = ! tmp.5
            if !tmp.6 jump end_if_0
            return 1
        
          end_if_0:
            tmp.7 = compare_constants_2()
            tmp.8 = ! tmp.7
            if !tmp.8 jump end_if_2
            return 2
        
          end_if_2:
            tmp.9 = - 9223372036854775807L
            l = tmp.9
            tmp.10 = l_geq_2_60()
            if !tmp.10 jump end_if_4
            return 3
        
          end_if_4:
            tmp.11 = uint_max_leq_l()
            if !tmp.11 jump end_if_6
            return 4
        
          end_if_6:
            l = 1152921504606846976L
            tmp.12 = l_geq_2_60()
            tmp.13 = ! tmp.12
            if !tmp.13 jump end_if_8
            return 5
        
          end_if_8:
            tmp.14 = uint_max_leq_l()
            tmp.15 = ! tmp.14
            if !tmp.15 jump end_if_10
            return 6
        
          end_if_10:
            l2 = l
            tmp.16 = l_eq_l2()
            tmp.17 = ! tmp.16
            if !tmp.17 jump end_if_12
            return 7
        
          end_if_12:
            return 0
            return 0
        }
        static global l: Long = 0L
        static global l2: Long = 0L
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_11_valid_long_expressions_large_constants() {
    let src = r#"
        long x = 5l;
        int add_large(void) {
            x = x + 4294967290l;
            return (x == 4294967295l);
        }
        int subtract_large(void) {
            x = x - 4294967290l;
            return (x == 5l);
        }
        int multiply_by_large(void) {
            x = x * 4294967290l;
            return (x == 21474836450l);
        }
        int main(void) {
            if (!add_large()) {
                return 1;
            }
            if (!subtract_large()) {
                return 2;
            }
            if (!multiply_by_large()) {
                return 3;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function add_large() { 
            tmp.0 = x + 4294967290L
            x = tmp.0
            tmp.1 = x == 4294967295L
            return tmp.1
            return 0
        }
        global function subtract_large() { 
            tmp.2 = x - 4294967290L
            x = tmp.2
            tmp.3 = x == 5L
            return tmp.3
            return 0
        }
        global function multiply_by_large() { 
            tmp.4 = x * 4294967290L
            x = tmp.4
            tmp.5 = x == 21474836450L
            return tmp.5
            return 0
        }
        global function main() { 
            tmp.6 = add_large()
            tmp.7 = ! tmp.6
            if !tmp.7 jump end_if_0
            return 1
        
          end_if_0:
            tmp.8 = subtract_large()
            tmp.9 = ! tmp.8
            if !tmp.9 jump end_if_2
            return 2
        
          end_if_2:
            tmp.10 = multiply_by_large()
            tmp.11 = ! tmp.10
            if !tmp.11 jump end_if_4
            return 3
        
          end_if_4:
            return 0
            return 0
        }
        static global x: Long = 5L
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_11_valid_long_expressions_logical() {
    let src = r#"
        int not(long l) {
            return !l;
        }
        int if_cond(long l) {
            if (l) {
                return 1;
            }
            return 0;
        }
        int and(long l1, int l2) {
            return l1 && l2;
        }
        int or(int l1, long l2) {
            return l1 || l2;
        }
        int main(void) {
            long l = 1152921504606846976l;
            long zero = 0l;
            if (not(l)) {
                return 1;
            }
            if (!not(zero)) {
                return 2;
            }
            if(!if_cond(l)) {
                return 3;
            }
            if(if_cond(zero)) {
                return 4;
            }
            if (and(zero, 1)) {
                return 5;
            }
            if (!or(1, l)) {
                return 6;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function not(l.0) { 
            tmp.0 = ! l.0
            return tmp.0
            return 0
        }
        global function if_cond(l.1) { 
            if !l.1 jump end_if_0
            return 1
        
          end_if_0:
            return 0
            return 0
        }
        global function and(l1.2, l2.3) { 
            if !l1.2 jump and_false_2
            if !l2.3 jump and_false_2
            tmp.2 = 1
            jump and_end_3
        
          and_false_2:
            tmp.2 = 0
        
          and_end_3:
            return tmp.2
            return 0
        }
        global function or(l1.4, l2.5) { 
            if l1.4 jump or_true_4
            if l2.5 jump or_true_4
            tmp.4 = 0
            jump or_end_5
        
          or_true_4:
            tmp.4 = 1
        
          or_end_5:
            return tmp.4
            return 0
        }
        global function main() { 
            l.6 = 1152921504606846976L
            zero.7 = 0L
            tmp.5 = not(l.6)
            if !tmp.5 jump end_if_6
            return 1
        
          end_if_6:
            tmp.6 = not(zero.7)
            tmp.7 = ! tmp.6
            if !tmp.7 jump end_if_8
            return 2
        
          end_if_8:
            tmp.8 = if_cond(l.6)
            tmp.9 = ! tmp.8
            if !tmp.9 jump end_if_10
            return 3
        
          end_if_10:
            tmp.10 = if_cond(zero.7)
            if !tmp.10 jump end_if_12
            return 4
        
          end_if_12:
            tmp.11 = and(zero.7, 1)
            if !tmp.11 jump end_if_14
            return 5
        
          end_if_14:
            tmp.12 = or(1, l.6)
            tmp.13 = ! tmp.12
            if !tmp.13 jump end_if_16
            return 6
        
          end_if_16:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_11_valid_long_expressions_long_and_int_locals() {
    let src = r#"
        int main(void) {
            long a = 8589934592l;
            int b = -1;
            long c = -8589934592l;
            int d = 10;
            if (a != 8589934592l) {
                return 1;
            }
            if (b != -1){
                return 2;
            }
            if (c != -8589934592l) {
                return 3;
            }
            if (d != 10) {
                return 4;
            }
            a = -a;
            b = b - 1;
            c = c + 8589934594l;
            d = d + 10;
            if (a != -8589934592l) {
                return 5;
            }
            if (b != -2) {
                return 6;
            }
            if (c != 2) {
                return 7;
            }
            if (d != 20) {
                return 8;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 8589934592L
            tmp.0 = - 1
            b.1 = tmp.0
            tmp.1 = - 8589934592L
            c.2 = tmp.1
            d.3 = 10
            tmp.2 = a.0 != 8589934592L
            if !tmp.2 jump end_if_0
            return 1
        
          end_if_0:
            tmp.4 = - 1
            tmp.3 = b.1 != tmp.4
            if !tmp.3 jump end_if_2
            return 2
        
          end_if_2:
            tmp.6 = - 8589934592L
            tmp.5 = c.2 != tmp.6
            if !tmp.5 jump end_if_4
            return 3
        
          end_if_4:
            tmp.7 = d.3 != 10
            if !tmp.7 jump end_if_6
            return 4
        
          end_if_6:
            tmp.8 = - a.0
            a.0 = tmp.8
            tmp.9 = b.1 - 1
            b.1 = tmp.9
            tmp.10 = c.2 + 8589934594L
            c.2 = tmp.10
            tmp.11 = d.3 + 10
            d.3 = tmp.11
            tmp.13 = - 8589934592L
            tmp.12 = a.0 != tmp.13
            if !tmp.12 jump end_if_8
            return 5
        
          end_if_8:
            tmp.15 = - 2
            tmp.14 = b.1 != tmp.15
            if !tmp.14 jump end_if_10
            return 6
        
          end_if_10:
            tmp.17 = sign_extend 2
            tmp.16 = c.2 != tmp.17
            if !tmp.16 jump end_if_12
            return 7
        
          end_if_12:
            tmp.18 = d.3 != 20
            if !tmp.18 jump end_if_14
            return 8
        
          end_if_14:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_11_valid_long_expressions_long_args() {
    let src = r#"
        int test_sum(long a, long b, int c, int d, int e, int f, int g, int h, long i) {
            if (a + b < 100l) {
                return 1;
            }
            if (i < 100l)
                return 2;
            return 0;
        }
        int main(void) {
            return test_sum(34359738368l, 34359738368l, 0, 0, 0, 0, 0, 0, 34359738368l);
        }
    "#;
    let expected = r#"
        global function test_sum(a.0, b.1, c.2, d.3, e.4, f.5, g.6, h.7, i.8) { 
            tmp.0 = a.0 + b.1
            tmp.1 = tmp.0 < 100L
            if !tmp.1 jump end_if_0
            return 1
        
          end_if_0:
            tmp.2 = i.8 < 100L
            if !tmp.2 jump end_if_2
            return 2
        
          end_if_2:
            return 0
            return 0
        }
        global function main() { 
            tmp.3 = test_sum(34359738368L, 34359738368L, 0, 0, 0, 0, 0, 0, 34359738368L)
            return tmp.3
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_11_valid_long_expressions_multi_op() {
    let src = r#"
        int target(long a) {
            long b = a * 5l - 10l;
            if (b == 21474836440l) {
                return 1;
            }
            return 0;
        }
        int main(void) {
            return target(4294967290l);
        }
    "#;
    let expected = r#"
        global function target(a.0) { 
            tmp.0 = a.0 * 5L
            tmp.1 = tmp.0 - 10L
            b.1 = tmp.1
            tmp.2 = b.1 == 21474836440L
            if !tmp.2 jump end_if_0
            return 1
        
          end_if_0:
            return 0
            return 0
        }
        global function main() { 
            tmp.3 = target(4294967290L)
            return tmp.3
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_11_valid_long_expressions_return_long() {
    let src = r#"
        long add(int a, int b) {
            return (long) a + (long) b;
        }
        int main(void) {
            long a = add(2147483645, 2147483645);
            if (a == 4294967290l) {
                return 1;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function add(a.0, b.1) { 
            tmp.0 = sign_extend a.0
            tmp.2 = sign_extend b.1
            tmp.1 = tmp.0 + tmp.2
            return tmp.1
            return 0
        }
        global function main() { 
            tmp.3 = add(2147483645, 2147483645)
            a.2 = tmp.3
            tmp.4 = a.2 == 4294967290L
            if !tmp.4 jump end_if_0
            return 1
        
          end_if_0:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_11_valid_long_expressions_rewrite_large_multiply_regression() {
    let src = r#"
        int check_12_ints(int start, int a, int b, int c, int d, int e, int f, int g,
                          int h, int i, int j, int k, int l);
        long glob = 5l;
        int main(void) {
            long should_spill = glob * 4294967307l;
            int one = glob - 4;
            int two = one + one;
            int three = 2 + one;
            int four = two * two;
            int five = 6 - one;
            int six = two * three;
            int seven = one + 6;
            int eight = two * 4;
            int nine = three * three;
            int ten = four + six;
            int eleven = 16 - five;
            int twelve = six + six;
            check_12_ints(one, two, three, four, five, six, seven, eight, nine, ten,
                          eleven, twelve, 1);
            int thirteen = glob + 8;
            int fourteen = thirteen + 1;
            int fifteen = 28 - thirteen;
            int sixteen = fourteen + 2;
            int seventeen = 4 + thirteen;
            int eighteen = 32 - fourteen;
            int nineteen = 35 - sixteen;
            int twenty = fifteen + 5;
            int twenty_one = thirteen * 2 - 5;
            int twenty_two = fifteen + 7;
            int twenty_three = 6 + seventeen;
            int twenty_four = thirteen + 11;
            check_12_ints(thirteen, fourteen, fifteen, sixteen, seventeen, eighteen,
                          nineteen, twenty, twenty_one, twenty_two, twenty_three,
                          twenty_four, 13);
            if (should_spill != 21474836535l) {
                return -1;
            }
            return 0;
        }
        int check_12_ints(int a, int b, int c, int d, int e, int f, int g, int h, int i,
                          int j, int k, int l, int start) {
            int expected = 0;
            expected = start + 0;
            if (a != expected) {
                return expected;
            }
            expected = start + 1;
            if (b != expected) {
                return expected;
            }
            expected = start + 2;
            if (c != expected) {
                return expected;
            }
            expected = start + 3;
            if (d != expected) {
                return expected;
            }
            expected = start + 4;
            if (e != expected) {
                return expected;
            }
            expected = start + 5;
            if (f != expected) {
                return expected;
            }
            expected = start + 6;
            if (g != expected) {
                return expected;
            }
            expected = start + 7;
            if (h != expected) {
                return expected;
            }
            expected = start + 8;
            if (i != expected) {
                return expected;
            }
            expected = start + 9;
            if (j != expected) {
                return expected;
            }
            expected = start + 10;
            if (k != expected) {
                return expected;
            }
            expected = start + 11;
            if (l != expected) {
                return expected;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = glob * 4294967307L
            should_spill.13 = tmp.0
            tmp.2 = sign_extend 4
            tmp.1 = glob - tmp.2
            tmp.3 = truncate tmp.1
            one.14 = tmp.3
            tmp.4 = one.14 + one.14
            two.15 = tmp.4
            tmp.5 = 2 + one.14
            three.16 = tmp.5
            tmp.6 = two.15 * two.15
            four.17 = tmp.6
            tmp.7 = 6 - one.14
            five.18 = tmp.7
            tmp.8 = two.15 * three.16
            six.19 = tmp.8
            tmp.9 = one.14 + 6
            seven.20 = tmp.9
            tmp.10 = two.15 * 4
            eight.21 = tmp.10
            tmp.11 = three.16 * three.16
            nine.22 = tmp.11
            tmp.12 = four.17 + six.19
            ten.23 = tmp.12
            tmp.13 = 16 - five.18
            eleven.24 = tmp.13
            tmp.14 = six.19 + six.19
            twelve.25 = tmp.14
            tmp.15 = check_12_ints(one.14, two.15, three.16, four.17, five.18, six.19, seven.20, eight.21, nine.22, ten.23, eleven.24, twelve.25, 1)
            tmp.17 = sign_extend 8
            tmp.16 = glob + tmp.17
            tmp.18 = truncate tmp.16
            thirteen.26 = tmp.18
            tmp.19 = thirteen.26 + 1
            fourteen.27 = tmp.19
            tmp.20 = 28 - thirteen.26
            fifteen.28 = tmp.20
            tmp.21 = fourteen.27 + 2
            sixteen.29 = tmp.21
            tmp.22 = 4 + thirteen.26
            seventeen.30 = tmp.22
            tmp.23 = 32 - fourteen.27
            eighteen.31 = tmp.23
            tmp.24 = 35 - sixteen.29
            nineteen.32 = tmp.24
            tmp.25 = fifteen.28 + 5
            twenty.33 = tmp.25
            tmp.26 = thirteen.26 * 2
            tmp.27 = tmp.26 - 5
            twenty_one.34 = tmp.27
            tmp.28 = fifteen.28 + 7
            twenty_two.35 = tmp.28
            tmp.29 = 6 + seventeen.30
            twenty_three.36 = tmp.29
            tmp.30 = thirteen.26 + 11
            twenty_four.37 = tmp.30
            tmp.31 = check_12_ints(thirteen.26, fourteen.27, fifteen.28, sixteen.29, seventeen.30, eighteen.31, nineteen.32, twenty.33, twenty_one.34, twenty_two.35, twenty_three.36, twenty_four.37, 13)
            tmp.32 = should_spill.13 != 21474836535L
            if !tmp.32 jump end_if_0
            tmp.33 = - 1
            return tmp.33
        
          end_if_0:
            return 0
            return 0
        }
        global function check_12_ints(a.38, b.39, c.40, d.41, e.42, f.43, g.44, h.45, i.46, j.47, k.48, l.49, start.50) { 
            expected.51 = 0
            tmp.34 = start.50 + 0
            expected.51 = tmp.34
            tmp.35 = a.38 != expected.51
            if !tmp.35 jump end_if_2
            return expected.51
        
          end_if_2:
            tmp.36 = start.50 + 1
            expected.51 = tmp.36
            tmp.37 = b.39 != expected.51
            if !tmp.37 jump end_if_4
            return expected.51
        
          end_if_4:
            tmp.38 = start.50 + 2
            expected.51 = tmp.38
            tmp.39 = c.40 != expected.51
            if !tmp.39 jump end_if_6
            return expected.51
        
          end_if_6:
            tmp.40 = start.50 + 3
            expected.51 = tmp.40
            tmp.41 = d.41 != expected.51
            if !tmp.41 jump end_if_8
            return expected.51
        
          end_if_8:
            tmp.42 = start.50 + 4
            expected.51 = tmp.42
            tmp.43 = e.42 != expected.51
            if !tmp.43 jump end_if_10
            return expected.51
        
          end_if_10:
            tmp.44 = start.50 + 5
            expected.51 = tmp.44
            tmp.45 = f.43 != expected.51
            if !tmp.45 jump end_if_12
            return expected.51
        
          end_if_12:
            tmp.46 = start.50 + 6
            expected.51 = tmp.46
            tmp.47 = g.44 != expected.51
            if !tmp.47 jump end_if_14
            return expected.51
        
          end_if_14:
            tmp.48 = start.50 + 7
            expected.51 = tmp.48
            tmp.49 = h.45 != expected.51
            if !tmp.49 jump end_if_16
            return expected.51
        
          end_if_16:
            tmp.50 = start.50 + 8
            expected.51 = tmp.50
            tmp.51 = i.46 != expected.51
            if !tmp.51 jump end_if_18
            return expected.51
        
          end_if_18:
            tmp.52 = start.50 + 9
            expected.51 = tmp.52
            tmp.53 = j.47 != expected.51
            if !tmp.53 jump end_if_20
            return expected.51
        
          end_if_20:
            tmp.54 = start.50 + 10
            expected.51 = tmp.54
            tmp.55 = k.48 != expected.51
            if !tmp.55 jump end_if_22
            return expected.51
        
          end_if_22:
            tmp.56 = start.50 + 11
            expected.51 = tmp.56
            tmp.57 = l.49 != expected.51
            if !tmp.57 jump end_if_24
            return expected.51
        
          end_if_24:
            return 0
            return 0
        }
        static global glob: Long = 5L
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_11_valid_long_expressions_simple() {
    let src = r#"
        int main(void) {
            long l = 9223372036854775807l;
            return (l - 2l == 9223372036854775805l);
        }
    "#;
    let expected = r#"
        global function main() { 
            l.0 = 9223372036854775807L
            tmp.0 = l.0 - 2L
            tmp.1 = tmp.0 == 9223372036854775805L
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_11_valid_long_expressions_static_long() {
    let src = r#"
        
        static long foo = 4294967290l;
        int main(void)
        {
            if (foo + 5l == 4294967295l)
            {
                foo = 1152921504606846988l;
                if (foo == 1152921504606846988l)
                    return 1;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = foo + 5L
            tmp.1 = tmp.0 == 4294967295L
            if !tmp.1 jump end_if_0
            foo = 1152921504606846988L
            tmp.2 = foo == 1152921504606846988L
            if !tmp.2 jump end_if_2
            return 1
        
          end_if_2:
        
          end_if_0:
            return 0
            return 0
        }
        static foo: Long = 4294967290L
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_11_valid_long_expressions_type_specifiers() {
    let src = r#"
        static int long a;
        int static long a;
        long static a;
        int my_function(long a, long int b, int long c);
        int my_function(long int x, int long y, long z) {
            return x + y + z;
        }
        int main(void) {
            long x = 1l;
            long int y = 2l;
            int long z = 3l;
            extern long a;
            a = 4;
           int sum = 0;
            for (long i = 1099511627776l; i > 0; i = i / 2) {
                sum = sum + 1;
            }
            if (x != 1) {
                return 1;
            }
            if (y != 2) {
                return 2;
            }
            if (a != 4) {
                return 3;
            }
            if (my_function(x, y, z) != 6) {
                return 4;
            }
            if (sum != 41) {
                return 5;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function my_function(x.3, y.4, z.5) { 
            tmp.0 = x.3 + y.4
            tmp.1 = tmp.0 + z.5
            tmp.2 = truncate tmp.1
            return tmp.2
            return 0
        }
        global function main() { 
            x.6 = 1L
            y.7 = 2L
            z.8 = 3L
            tmp.3 = sign_extend 4
            a = tmp.3
            sum.10 = 0
            i.11 = 1099511627776L
        
          start_loop_0:
            tmp.5 = sign_extend 0
            tmp.4 = i.11 > tmp.5
            if !tmp.4 jump break_loop_0
            tmp.6 = sum.10 + 1
            sum.10 = tmp.6
        
          continue_loop_0:
            tmp.8 = sign_extend 2
            tmp.7 = i.11 / tmp.8
            i.11 = tmp.7
            jump start_loop_0
        
          break_loop_0:
            tmp.10 = sign_extend 1
            tmp.9 = x.6 != tmp.10
            if !tmp.9 jump end_if_0
            return 1
        
          end_if_0:
            tmp.12 = sign_extend 2
            tmp.11 = y.7 != tmp.12
            if !tmp.11 jump end_if_2
            return 2
        
          end_if_2:
            tmp.14 = sign_extend 4
            tmp.13 = a != tmp.14
            if !tmp.13 jump end_if_4
            return 3
        
          end_if_4:
            tmp.15 = my_function(x.6, y.7, z.8)
            tmp.16 = tmp.15 != 6
            if !tmp.16 jump end_if_6
            return 4
        
          end_if_6:
            tmp.17 = sum.10 != 41
            if !tmp.17 jump end_if_8
            return 5
        
          end_if_8:
            return 0
            return 0
        }
        static a: Long = 0L
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_12_valid_explicit_casts_chained_casts() {
    let src = r#"
        unsigned int ui = 4294967200u;
        int main(void) {
            if ((long) (signed) ui != -96l)
                return 1;
            if ((unsigned long) (signed) ui != 18446744073709551520ul)
                return 2;
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = ui
            tmp.1 = sign_extend tmp.0
            tmp.3 = - 96L
            tmp.2 = tmp.1 != tmp.3
            if !tmp.2 jump end_if_0
            return 1
        
          end_if_0:
            tmp.4 = ui
            tmp.5 = sign_extend tmp.4
            tmp.6 = tmp.5 != 18446744073709551520UL
            if !tmp.6 jump end_if_2
            return 2
        
          end_if_2:
            return 0
            return 0
        }
        static global ui: Unsigned Int = 4294967200U
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_12_valid_explicit_casts_extension() {
    let src = r#"
        int int_to_ulong(int i, unsigned long expected) {
            unsigned long result = (unsigned long) i;
            return result == expected;
        }
        int uint_to_long(unsigned int ui, long expected) {
            long result = (long) ui;
            return result == expected;
        }
        int uint_to_ulong(unsigned ui, unsigned long expected){
            return (unsigned long) ui == expected;
        }
        int main(void) {
            if (!int_to_ulong(10, 10ul)) {
                return 1;
            }
            if (!int_to_ulong(-10, 18446744073709551606ul)) {
                return 2;
            }
            if (!uint_to_long(4294967200u, 4294967200l)) {
                return 3;
            }
            if (!uint_to_ulong(4294967200u, 4294967200ul)) {
                return 4;
            }
            if ((unsigned long) 4294967200u != 4294967200ul) {
                return 5;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function int_to_ulong(i.0, expected.1) { 
            tmp.0 = sign_extend i.0
            result.2 = tmp.0
            tmp.1 = result.2 == expected.1
            return tmp.1
            return 0
        }
        global function uint_to_long(ui.3, expected.4) { 
            tmp.2 = zero_extend ui.3
            result.5 = tmp.2
            tmp.3 = result.5 == expected.4
            return tmp.3
            return 0
        }
        global function uint_to_ulong(ui.6, expected.7) { 
            tmp.4 = zero_extend ui.6
            tmp.5 = tmp.4 == expected.7
            return tmp.5
            return 0
        }
        global function main() { 
            tmp.6 = int_to_ulong(10, 10UL)
            tmp.7 = ! tmp.6
            if !tmp.7 jump end_if_0
            return 1
        
          end_if_0:
            tmp.8 = - 10
            tmp.9 = int_to_ulong(tmp.8, 18446744073709551606UL)
            tmp.10 = ! tmp.9
            if !tmp.10 jump end_if_2
            return 2
        
          end_if_2:
            tmp.11 = uint_to_long(4294967200U, 4294967200L)
            tmp.12 = ! tmp.11
            if !tmp.12 jump end_if_4
            return 3
        
          end_if_4:
            tmp.13 = uint_to_ulong(4294967200U, 4294967200UL)
            tmp.14 = ! tmp.13
            if !tmp.14 jump end_if_6
            return 4
        
          end_if_6:
            tmp.15 = zero_extend 4294967200U
            tmp.16 = tmp.15 != 4294967200UL
            if !tmp.16 jump end_if_8
            return 5
        
          end_if_8:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_12_valid_explicit_casts_rewrite_movz_regression() {
    let src = r#"
        int check_12_ints(int start, int a, int b, int c, int d, int e, int f, int g,
                          int h, int i, int j, int k, int l);
        unsigned glob = 5000u;
        int main(void) {
            long should_spill = (long)glob;
            int one = glob - 4999;
            int two = one + one;
            int three = 2 + one;
            int four = two * two;
            int five = 6 - one;
            int six = two * three;
            int seven = one + 6;
            int eight = two * 4;
            int nine = three * three;
            int ten = four + six;
            int eleven = 16 - five;
            int twelve = six + six;
            check_12_ints(one, two, three, four, five, six, seven, eight, nine, ten,
                          eleven, twelve, 1);
            int thirteen = glob - 4987u;
            int fourteen = thirteen + 1;
            int fifteen = 28 - thirteen;
            int sixteen = fourteen + 2;
            int seventeen = 4 + thirteen;
            int eighteen = 32 - fourteen;
            int nineteen = 35 - sixteen;
            int twenty = fifteen + 5;
            int twenty_one = thirteen * 2 - 5;
            int twenty_two = fifteen + 7;
            int twenty_three = 6 + seventeen;
            int twenty_four = thirteen + 11;
            check_12_ints(thirteen, fourteen, fifteen, sixteen, seventeen, eighteen,
                          nineteen, twenty, twenty_one, twenty_two, twenty_three,
                          twenty_four, 13);
            if (should_spill != 5000l) {
                return -1;
            }
            return 0;
        }
        int check_12_ints(int a, int b, int c, int d, int e, int f, int g, int h, int i,
                          int j, int k, int l, int start) {
            int expected = 0;
            expected = start + 0;
            if (a != expected) {
                return expected;
            }
            expected = start + 1;
            if (b != expected) {
                return expected;
            }
            expected = start + 2;
            if (c != expected) {
                return expected;
            }
            expected = start + 3;
            if (d != expected) {
                return expected;
            }
            expected = start + 4;
            if (e != expected) {
                return expected;
            }
            expected = start + 5;
            if (f != expected) {
                return expected;
            }
            expected = start + 6;
            if (g != expected) {
                return expected;
            }
            expected = start + 7;
            if (h != expected) {
                return expected;
            }
            expected = start + 8;
            if (i != expected) {
                return expected;
            }
            expected = start + 9;
            if (j != expected) {
                return expected;
            }
            expected = start + 10;
            if (k != expected) {
                return expected;
            }
            expected = start + 11;
            if (l != expected) {
                return expected;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = zero_extend glob
            should_spill.13 = tmp.0
            tmp.2 = 4999
            tmp.1 = glob - tmp.2
            tmp.3 = tmp.1
            one.14 = tmp.3
            tmp.4 = one.14 + one.14
            two.15 = tmp.4
            tmp.5 = 2 + one.14
            three.16 = tmp.5
            tmp.6 = two.15 * two.15
            four.17 = tmp.6
            tmp.7 = 6 - one.14
            five.18 = tmp.7
            tmp.8 = two.15 * three.16
            six.19 = tmp.8
            tmp.9 = one.14 + 6
            seven.20 = tmp.9
            tmp.10 = two.15 * 4
            eight.21 = tmp.10
            tmp.11 = three.16 * three.16
            nine.22 = tmp.11
            tmp.12 = four.17 + six.19
            ten.23 = tmp.12
            tmp.13 = 16 - five.18
            eleven.24 = tmp.13
            tmp.14 = six.19 + six.19
            twelve.25 = tmp.14
            tmp.15 = check_12_ints(one.14, two.15, three.16, four.17, five.18, six.19, seven.20, eight.21, nine.22, ten.23, eleven.24, twelve.25, 1)
            tmp.16 = glob - 4987U
            tmp.17 = tmp.16
            thirteen.26 = tmp.17
            tmp.18 = thirteen.26 + 1
            fourteen.27 = tmp.18
            tmp.19 = 28 - thirteen.26
            fifteen.28 = tmp.19
            tmp.20 = fourteen.27 + 2
            sixteen.29 = tmp.20
            tmp.21 = 4 + thirteen.26
            seventeen.30 = tmp.21
            tmp.22 = 32 - fourteen.27
            eighteen.31 = tmp.22
            tmp.23 = 35 - sixteen.29
            nineteen.32 = tmp.23
            tmp.24 = fifteen.28 + 5
            twenty.33 = tmp.24
            tmp.25 = thirteen.26 * 2
            tmp.26 = tmp.25 - 5
            twenty_one.34 = tmp.26
            tmp.27 = fifteen.28 + 7
            twenty_two.35 = tmp.27
            tmp.28 = 6 + seventeen.30
            twenty_three.36 = tmp.28
            tmp.29 = thirteen.26 + 11
            twenty_four.37 = tmp.29
            tmp.30 = check_12_ints(thirteen.26, fourteen.27, fifteen.28, sixteen.29, seventeen.30, eighteen.31, nineteen.32, twenty.33, twenty_one.34, twenty_two.35, twenty_three.36, twenty_four.37, 13)
            tmp.31 = should_spill.13 != 5000L
            if !tmp.31 jump end_if_0
            tmp.32 = - 1
            return tmp.32
        
          end_if_0:
            return 0
            return 0
        }
        global function check_12_ints(a.38, b.39, c.40, d.41, e.42, f.43, g.44, h.45, i.46, j.47, k.48, l.49, start.50) { 
            expected.51 = 0
            tmp.33 = start.50 + 0
            expected.51 = tmp.33
            tmp.34 = a.38 != expected.51
            if !tmp.34 jump end_if_2
            return expected.51
        
          end_if_2:
            tmp.35 = start.50 + 1
            expected.51 = tmp.35
            tmp.36 = b.39 != expected.51
            if !tmp.36 jump end_if_4
            return expected.51
        
          end_if_4:
            tmp.37 = start.50 + 2
            expected.51 = tmp.37
            tmp.38 = c.40 != expected.51
            if !tmp.38 jump end_if_6
            return expected.51
        
          end_if_6:
            tmp.39 = start.50 + 3
            expected.51 = tmp.39
            tmp.40 = d.41 != expected.51
            if !tmp.40 jump end_if_8
            return expected.51
        
          end_if_8:
            tmp.41 = start.50 + 4
            expected.51 = tmp.41
            tmp.42 = e.42 != expected.51
            if !tmp.42 jump end_if_10
            return expected.51
        
          end_if_10:
            tmp.43 = start.50 + 5
            expected.51 = tmp.43
            tmp.44 = f.43 != expected.51
            if !tmp.44 jump end_if_12
            return expected.51
        
          end_if_12:
            tmp.45 = start.50 + 6
            expected.51 = tmp.45
            tmp.46 = g.44 != expected.51
            if !tmp.46 jump end_if_14
            return expected.51
        
          end_if_14:
            tmp.47 = start.50 + 7
            expected.51 = tmp.47
            tmp.48 = h.45 != expected.51
            if !tmp.48 jump end_if_16
            return expected.51
        
          end_if_16:
            tmp.49 = start.50 + 8
            expected.51 = tmp.49
            tmp.50 = i.46 != expected.51
            if !tmp.50 jump end_if_18
            return expected.51
        
          end_if_18:
            tmp.51 = start.50 + 9
            expected.51 = tmp.51
            tmp.52 = j.47 != expected.51
            if !tmp.52 jump end_if_20
            return expected.51
        
          end_if_20:
            tmp.53 = start.50 + 10
            expected.51 = tmp.53
            tmp.54 = k.48 != expected.51
            if !tmp.54 jump end_if_22
            return expected.51
        
          end_if_22:
            tmp.55 = start.50 + 11
            expected.51 = tmp.55
            tmp.56 = l.49 != expected.51
            if !tmp.56 jump end_if_24
            return expected.51
        
          end_if_24:
            return 0
            return 0
        }
        static global glob: Unsigned Int = 5000U
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_12_valid_explicit_casts_round_trip_casts() {
    let src = r#"
        unsigned long a = 8589934580ul;
        int main(void) {
            unsigned long b = (unsigned long) (unsigned int) a;
            if (b != 4294967284ul)
                return 1;
            b = (unsigned long) (signed int) a;
            if (b != 18446744073709551604ul)
                return 2;
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = truncate a
            tmp.1 = zero_extend tmp.0
            b.0 = tmp.1
            tmp.2 = b.0 != 4294967284UL
            if !tmp.2 jump end_if_0
            return 1
        
          end_if_0:
            tmp.3 = truncate a
            tmp.4 = sign_extend tmp.3
            b.0 = tmp.4
            tmp.5 = b.0 != 18446744073709551604UL
            if !tmp.5 jump end_if_2
            return 2
        
          end_if_2:
            return 0
            return 0
        }
        static global a: Unsigned Long = 8589934580UL
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_12_valid_explicit_casts_same_size_conversion() {
    let src = r#"
        int uint_to_int(unsigned int ui, int expected) {
            return (int) ui == expected;
        }
        int int_to_uint(int i, unsigned int expected) {
            return (unsigned int) i == expected;
        }
        int ulong_to_long(unsigned long ul, signed long expected) {
            return (signed long) ul == expected;
        }
        int long_to_ulong(long l, unsigned long expected) {
            return (unsigned long) l == expected;
        }
        int main(void) {
            if (!int_to_uint(10, 10u)) {
                return 1;
            }
            if (!uint_to_int(10u, 10)) {
                return 2;
            }
            if (!long_to_ulong(-1000l, 18446744073709550616ul)) {
                return 3;
            }
            if (!ulong_to_long(18446744073709550616ul, -1000l)) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function uint_to_int(ui.0, expected.1) { 
            tmp.0 = ui.0
            tmp.1 = tmp.0 == expected.1
            return tmp.1
            return 0
        }
        global function int_to_uint(i.2, expected.3) { 
            tmp.2 = i.2
            tmp.3 = tmp.2 == expected.3
            return tmp.3
            return 0
        }
        global function ulong_to_long(ul.4, expected.5) { 
            tmp.4 = ul.4
            tmp.5 = tmp.4 == expected.5
            return tmp.5
            return 0
        }
        global function long_to_ulong(l.6, expected.7) { 
            tmp.6 = l.6
            tmp.7 = tmp.6 == expected.7
            return tmp.7
            return 0
        }
        global function main() { 
            tmp.8 = int_to_uint(10, 10U)
            tmp.9 = ! tmp.8
            if !tmp.9 jump end_if_0
            return 1
        
          end_if_0:
            tmp.10 = uint_to_int(10U, 10)
            tmp.11 = ! tmp.10
            if !tmp.11 jump end_if_2
            return 2
        
          end_if_2:
            tmp.12 = - 1000L
            tmp.13 = long_to_ulong(tmp.12, 18446744073709550616UL)
            tmp.14 = ! tmp.13
            if !tmp.14 jump end_if_4
            return 3
        
          end_if_4:
            tmp.15 = - 1000L
            tmp.16 = ulong_to_long(18446744073709550616UL, tmp.15)
            tmp.17 = ! tmp.16
            if !tmp.17 jump end_if_6
            return 4
        
          end_if_6:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_12_valid_explicit_casts_truncate() {
    let src = r#"
        
        int ulong_to_int(unsigned long ul, int expected) {
            int result = (int) ul;
            return (result == expected);
        }
        int ulong_to_uint(unsigned long ul, unsigned expected) {
            return ((unsigned int) ul == expected);
        }
        int long_to_uint(long l, unsigned int expected) {
            return (unsigned int) l == expected;
        }
        int main(void) {
            if (!long_to_uint(100l, 100u)) {
                return 1;
            }
            if (!long_to_uint(-9223372036854774574l, 1234u)) {
                return 2;
            }
            if (!ulong_to_int(100ul, 100)) {
                return 3;
            }
            if (!ulong_to_uint(100ul, 100u)) {
                return 4;
            }
            if (!ulong_to_uint(4294967200ul, 4294967200u)) {
                return 5;
            }
            if (!ulong_to_int(4294967200ul, -96)) {
                return 6;
            }
            if (!ulong_to_uint(1152921506754330624ul, 2147483648u)) {
                return 7;
            }
            if (!ulong_to_int(1152921506754330624ul, -2147483648)){
                return 8;
            }
            unsigned int ui = (unsigned int)17179869189ul;
            if (ui != 5)
                return 9;
            return 0;
        }
    "#;
    let expected = r#"
        global function ulong_to_int(ul.0, expected.1) { 
            tmp.0 = truncate ul.0
            result.2 = tmp.0
            tmp.1 = result.2 == expected.1
            return tmp.1
            return 0
        }
        global function ulong_to_uint(ul.3, expected.4) { 
            tmp.2 = truncate ul.3
            tmp.3 = tmp.2 == expected.4
            return tmp.3
            return 0
        }
        global function long_to_uint(l.5, expected.6) { 
            tmp.4 = truncate l.5
            tmp.5 = tmp.4 == expected.6
            return tmp.5
            return 0
        }
        global function main() { 
            tmp.6 = long_to_uint(100L, 100U)
            tmp.7 = ! tmp.6
            if !tmp.7 jump end_if_0
            return 1
        
          end_if_0:
            tmp.8 = - 9223372036854774574L
            tmp.9 = long_to_uint(tmp.8, 1234U)
            tmp.10 = ! tmp.9
            if !tmp.10 jump end_if_2
            return 2
        
          end_if_2:
            tmp.11 = ulong_to_int(100UL, 100)
            tmp.12 = ! tmp.11
            if !tmp.12 jump end_if_4
            return 3
        
          end_if_4:
            tmp.13 = ulong_to_uint(100UL, 100U)
            tmp.14 = ! tmp.13
            if !tmp.14 jump end_if_6
            return 4
        
          end_if_6:
            tmp.15 = ulong_to_uint(4294967200UL, 4294967200U)
            tmp.16 = ! tmp.15
            if !tmp.16 jump end_if_8
            return 5
        
          end_if_8:
            tmp.17 = - 96
            tmp.18 = ulong_to_int(4294967200UL, tmp.17)
            tmp.19 = ! tmp.18
            if !tmp.19 jump end_if_10
            return 6
        
          end_if_10:
            tmp.20 = ulong_to_uint(1152921506754330624UL, 2147483648U)
            tmp.21 = ! tmp.20
            if !tmp.21 jump end_if_12
            return 7
        
          end_if_12:
            tmp.22 = - 2147483648L
            tmp.23 = truncate tmp.22
            tmp.24 = ulong_to_int(1152921506754330624UL, tmp.23)
            tmp.25 = ! tmp.24
            if !tmp.25 jump end_if_14
            return 8
        
          end_if_14:
            tmp.26 = truncate 17179869189UL
            ui.7 = tmp.26
            tmp.28 = 5
            tmp.27 = ui.7 != tmp.28
            if !tmp.27 jump end_if_16
            return 9
        
          end_if_16:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_12_valid_extra_credit_bitwise_unsigned_ops() {
    let src = r#"
        int main(void) {
            unsigned int ui = -1u;
            unsigned long ul = 9223372036854775808ul;
            if ((ui & ul) != 0)
                return 1;
            if ((ui | ul) != 9223372041149743103ul)
                return 2;
            signed int i = -1;
            if ((i & ul) != ul)
                return 3;
            if ((i | ul) != i)
                return 4;
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = - 1U
            ui.0 = tmp.0
            ul.1 = 9223372036854775808UL
            tmp.1 = zero_extend ui.0
            tmp.2 = tmp.1 & ul.1
            tmp.4 = sign_extend 0
            tmp.3 = tmp.2 != tmp.4
            if !tmp.3 jump end_if_0
            return 1
        
          end_if_0:
            tmp.5 = zero_extend ui.0
            tmp.6 = tmp.5 | ul.1
            tmp.7 = tmp.6 != 9223372041149743103UL
            if !tmp.7 jump end_if_2
            return 2
        
          end_if_2:
            tmp.8 = - 1
            i.2 = tmp.8
            tmp.9 = sign_extend i.2
            tmp.10 = tmp.9 & ul.1
            tmp.11 = tmp.10 != ul.1
            if !tmp.11 jump end_if_4
            return 3
        
          end_if_4:
            tmp.12 = sign_extend i.2
            tmp.13 = tmp.12 | ul.1
            tmp.15 = sign_extend i.2
            tmp.14 = tmp.13 != tmp.15
            if !tmp.14 jump end_if_6
            return 4
        
          end_if_6:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_12_valid_extra_credit_bitwise_unsigned_shift() {
    let src = r#"
        int main(void) {
            unsigned int ui = -1u;
            if ((ui << 2l) != 4294967292) {
                return 1;
            }
            if ((ui >> 2) != 1073741823) {
                return 2;
            }
            static int shiftcount = 5;
            if ((1000000u >> shiftcount) != 31250) {
                return 3;
            }
            if ((1000000u << shiftcount) != 32000000) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = - 1U
            ui.0 = tmp.0
            tmp.1 = ui.0 << 2L
            tmp.2 = zero_extend tmp.1
            tmp.3 = tmp.2 != 4294967292L
            if !tmp.3 jump end_if_0
            return 1
        
          end_if_0:
            tmp.4 = ui.0 >> 2
            tmp.6 = 1073741823
            tmp.5 = tmp.4 != tmp.6
            if !tmp.5 jump end_if_2
            return 2
        
          end_if_2:
            tmp.7 = 1000000U >> shiftcount.1
            tmp.9 = 31250
            tmp.8 = tmp.7 != tmp.9
            if !tmp.8 jump end_if_4
            return 3
        
          end_if_4:
            tmp.10 = 1000000U << shiftcount.1
            tmp.12 = 32000000
            tmp.11 = tmp.10 != tmp.12
            if !tmp.11 jump end_if_6
            return 4
        
          end_if_6:
            return 0
            return 0
        }
        static shiftcount.1: Int = 5
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_12_valid_extra_credit_compound_assign_uint() {
    let src = r#"
        int main(void) {
            unsigned int x = -1u;
            x /= -10l;
            return (x == 3865470567u);
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = - 1U
            x.0 = tmp.0
            tmp.1 = zero_extend x.0
            tmp.3 = - 10L
            tmp.2 = tmp.1 / tmp.3
            tmp.4 = truncate tmp.2
            x.0 = tmp.4
            tmp.5 = truncate x.0
            tmp.6 = x.0 == 3865470567U
            return tmp.6
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_12_valid_extra_credit_compound_bitshift() {
    let src = r#"
        int main(void) {
            int i = -2;
            i >>= 3u;
            if (i != -1) {
                return 1;
            }
            unsigned long ul = 18446744073709551615UL;
            ul <<= 44;
            if (ul != 18446726481523507200ul) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = - 2
            i.0 = tmp.0
            tmp.2 = 3U
            tmp.1 = i.0 >> tmp.2
            i.0 = tmp.1
            tmp.4 = - 1
            tmp.3 = i.0 != tmp.4
            if !tmp.3 jump end_if_0
            return 1
        
          end_if_0:
            ul.1 = 18446744073709551615UL
            tmp.6 = sign_extend 44
            tmp.5 = ul.1 << tmp.6
            ul.1 = tmp.5
            tmp.7 = ul.1 != 18446726481523507200UL
            if !tmp.7 jump end_if_2
            return 2
        
          end_if_2:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_12_valid_extra_credit_compound_bitwise() {
    let src = r#"
        int main(void) {
            unsigned long ul = 18446460386757245432ul;
            ul &= -1000;
            if (ul != 18446460386757244952ul ) {
                return 1;
            }
            ul |= 4294967040u;
            if (ul != 18446460386824683288ul ) {
                return 2;
            }
            int i = 123456;
            unsigned int ui = 4042322160u;
            long l = -252645136;
            if (ui ^= l) {
                return 3;
            }
            if (ui) {
                return 4;
            }
            if (i != 123456) {
                return 5;
            }
            if (l != -252645136) {
                return 6;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            ul.0 = 18446460386757245432UL
            tmp.1 = - 1000
            tmp.2 = sign_extend tmp.1
            tmp.0 = ul.0 & tmp.2
            ul.0 = tmp.0
            tmp.3 = ul.0 != 18446460386757244952UL
            if !tmp.3 jump end_if_0
            return 1
        
          end_if_0:
            tmp.5 = zero_extend 4294967040U
            tmp.4 = ul.0 | tmp.5
            ul.0 = tmp.4
            tmp.6 = ul.0 != 18446460386824683288UL
            if !tmp.6 jump end_if_2
            return 2
        
          end_if_2:
            i.1 = 123456
            ui.2 = 4042322160U
            tmp.7 = - 252645136
            tmp.8 = sign_extend tmp.7
            l.3 = tmp.8
            tmp.9 = zero_extend ui.2
            tmp.10 = tmp.9 ^ l.3
            tmp.11 = truncate tmp.10
            ui.2 = tmp.11
            tmp.12 = truncate ui.2
            if !tmp.12 jump end_if_4
            return 3
        
          end_if_4:
            if !ui.2 jump end_if_6
            return 4
        
          end_if_6:
            tmp.13 = i.1 != 123456
            if !tmp.13 jump end_if_8
            return 5
        
          end_if_8:
            tmp.15 = - 252645136
            tmp.16 = sign_extend tmp.15
            tmp.14 = l.3 != tmp.16
            if !tmp.14 jump end_if_10
            return 6
        
          end_if_10:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_12_valid_extra_credit_postfix_precedence() {
    let src = r#"
        int main(void) {
            unsigned int ui = 4294967295U;
            if (((unsigned long)ui++) != 4294967295U) {
                return 1;
            }
            if (ui) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            ui.0 = 4294967295U
            tmp.0 = ui.0
            tmp.1 = inc ui.0
            ui.0 = tmp.1
            tmp.2 = zero_extend tmp.0
            tmp.4 = zero_extend 4294967295U
            tmp.3 = tmp.2 != tmp.4
            if !tmp.3 jump end_if_0
            return 1
        
          end_if_0:
            if !ui.0 jump end_if_2
            return 2
        
          end_if_2:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_12_valid_extra_credit_switch_uint() {
    let src = r#"
        int switch_on_uint(unsigned int ui) {
            switch (ui) {
                case 5u:
                    return 0;
                case 4294967286l:
                    return 1;
                case 34359738378ul:
                    return 2;
                default:
                    return 3;
            }
        }
        int main(void) {
            if (switch_on_uint(5) != 0)
                return 1;
            if (switch_on_uint(4294967286) != 1)
                return 2;
            if (switch_on_uint(10) != 2)
                return 3;
            return 0;
        }
    "#;
    let expected = r#"
        global function switch_on_uint(ui.0) { 
            tmp.0 = ui.0 == 5U
            if tmp.0 jump switch_0_case__1
            tmp.1 = ui.0 == 4294967286U
            if tmp.1 jump switch_0_case__2
            tmp.2 = ui.0 == 10U
            if tmp.2 jump switch_0_case__3
            jump switch_0_default_4
            jump break_switch_0
        
          switch_0_case__1:
            return 0
        
          switch_0_case__2:
            return 1
        
          switch_0_case__3:
            return 2
        
          switch_0_default_4:
            return 3
        
          break_switch_0:
            return 0
        }
        global function main() { 
            tmp.3 = 5
            tmp.4 = switch_on_uint(tmp.3)
            tmp.5 = tmp.4 != 0
            if !tmp.5 jump end_if_0
            return 1
        
          end_if_0:
            tmp.6 = truncate 4294967286L
            tmp.7 = switch_on_uint(tmp.6)
            tmp.8 = tmp.7 != 1
            if !tmp.8 jump end_if_2
            return 2
        
          end_if_2:
            tmp.9 = 10
            tmp.10 = switch_on_uint(tmp.9)
            tmp.11 = tmp.10 != 2
            if !tmp.11 jump end_if_4
            return 3
        
          end_if_4:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_12_valid_extra_credit_unsigned_incr_decr() {
    let src = r#"
        
        int main(void) {
            unsigned int i = 0;
            if (i-- != 0) {
                return 1;
            }
            if (i != 4294967295U) {
                return 2;
            }
            if (--i != 4294967294U) {
                return 3;
            }
            if (i != 4294967294U) {
                return 4;
            }
            unsigned long l = 18446744073709551614UL;
            if (l++ != 18446744073709551614UL) {
                return 5;
            }
            if (l != 18446744073709551615UL) {
                return 6;
            }
            if (++l != 0) {
                return 7;
            }
            if (l != 0) {
                return 8;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 0
            i.0 = tmp.0
            tmp.1 = i.0
            tmp.2 = dec i.0
            i.0 = tmp.2
            tmp.4 = 0
            tmp.3 = tmp.1 != tmp.4
            if !tmp.3 jump end_if_0
            return 1
        
          end_if_0:
            tmp.5 = i.0 != 4294967295U
            if !tmp.5 jump end_if_2
            return 2
        
          end_if_2:
            tmp.6 = dec i.0
            i.0 = tmp.6
            tmp.7 = tmp.6 != 4294967294U
            if !tmp.7 jump end_if_4
            return 3
        
          end_if_4:
            tmp.8 = i.0 != 4294967294U
            if !tmp.8 jump end_if_6
            return 4
        
          end_if_6:
            l.1 = 18446744073709551614UL
            tmp.9 = l.1
            tmp.10 = inc l.1
            l.1 = tmp.10
            tmp.11 = tmp.9 != 18446744073709551614UL
            if !tmp.11 jump end_if_8
            return 5
        
          end_if_8:
            tmp.12 = l.1 != 18446744073709551615UL
            if !tmp.12 jump end_if_10
            return 6
        
          end_if_10:
            tmp.13 = inc l.1
            l.1 = tmp.13
            tmp.15 = sign_extend 0
            tmp.14 = tmp.13 != tmp.15
            if !tmp.14 jump end_if_12
            return 7
        
          end_if_12:
            tmp.17 = sign_extend 0
            tmp.16 = l.1 != tmp.17
            if !tmp.16 jump end_if_14
            return 8
        
          end_if_14:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_12_valid_implicit_casts_common_type() {
    let src = r#"
        int int_gt_uint(int i, unsigned int u) {
            return i > u;
        }
        int int_gt_ulong(int i, unsigned long ul) {
            return i > ul;
        }
        int uint_gt_long(unsigned int u, long l) {
            return u > l;
        }
        int uint_lt_ulong(unsigned int u, unsigned long ul) {
            return u < ul;
        }
        int long_gt_ulong(long l, unsigned long ul) {
            return l > ul;
        }
        int ternary_int_uint(int flag, int i, unsigned int ui) {
            long result = flag ? i : ui;
            return (result == 4294967295l);
        }
        int main(void) {
            if (!int_gt_uint(-100, 100u)) {
                return 1;
            }
            if (!(int_gt_ulong(-1, 18446744073709551606ul))) {
                return 2;
            }
            if (!uint_gt_long(100u, -100l)) {
                return 3;
            }
            if (!uint_lt_ulong(1073741824u, 34359738368ul)) {
                return 4;
            }
            if (!long_gt_ulong(-1l, 1000ul)) {
                return 5;
            }
            if (!ternary_int_uint(1, -1, 1u)) {
                return 6;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function int_gt_uint(i.0, u.1) { 
            tmp.0 = i.0
            tmp.1 = tmp.0 > u.1
            return tmp.1
            return 0
        }
        global function int_gt_ulong(i.2, ul.3) { 
            tmp.2 = sign_extend i.2
            tmp.3 = tmp.2 > ul.3
            return tmp.3
            return 0
        }
        global function uint_gt_long(u.4, l.5) { 
            tmp.4 = zero_extend u.4
            tmp.5 = tmp.4 > l.5
            return tmp.5
            return 0
        }
        global function uint_lt_ulong(u.6, ul.7) { 
            tmp.6 = zero_extend u.6
            tmp.7 = tmp.6 < ul.7
            return tmp.7
            return 0
        }
        global function long_gt_ulong(l.8, ul.9) { 
            tmp.8 = l.8
            tmp.9 = tmp.8 > ul.9
            return tmp.9
            return 0
        }
        global function ternary_int_uint(flag.10, i.11, ui.12) { 
            if !flag.10 jump else_1
            tmp.11 = i.11
            tmp.10 = tmp.11
            jump end_if_0
        
          else_1:
            tmp.10 = ui.12
        
          end_if_0:
            tmp.12 = zero_extend tmp.10
            result.13 = tmp.12
            tmp.13 = result.13 == 4294967295L
            return tmp.13
            return 0
        }
        global function main() { 
            tmp.14 = - 100
            tmp.15 = int_gt_uint(tmp.14, 100U)
            tmp.16 = ! tmp.15
            if !tmp.16 jump end_if_2
            return 1
        
          end_if_2:
            tmp.17 = - 1
            tmp.18 = int_gt_ulong(tmp.17, 18446744073709551606UL)
            tmp.19 = ! tmp.18
            if !tmp.19 jump end_if_4
            return 2
        
          end_if_4:
            tmp.20 = - 100L
            tmp.21 = uint_gt_long(100U, tmp.20)
            tmp.22 = ! tmp.21
            if !tmp.22 jump end_if_6
            return 3
        
          end_if_6:
            tmp.23 = uint_lt_ulong(1073741824U, 34359738368UL)
            tmp.24 = ! tmp.23
            if !tmp.24 jump end_if_8
            return 4
        
          end_if_8:
            tmp.25 = - 1L
            tmp.26 = long_gt_ulong(tmp.25, 1000UL)
            tmp.27 = ! tmp.26
            if !tmp.27 jump end_if_10
            return 5
        
          end_if_10:
            tmp.28 = - 1
            tmp.29 = ternary_int_uint(1, tmp.28, 1U)
            tmp.30 = ! tmp.29
            if !tmp.30 jump end_if_12
            return 6
        
          end_if_12:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_12_valid_implicit_casts_convert_by_assignment() {
    let src = r#"
        int check_int(int converted, int expected) {
            return (converted == expected);
        }
        int check_long(long converted, long expected) {
            return (converted == expected);
        }
        int check_ulong(unsigned long converted, unsigned long expected) {
            return (converted == expected);
        }
        long return_extended_uint(unsigned int u) {
            return u;
        }
        unsigned long return_extended_int(int i) {
            return i;
        }
        int return_truncated_ulong(unsigned long ul) {
            return ul;
        }
        int extend_on_assignment(unsigned int ui, long expected) {
            long result = ui;
            return result == expected;
        }
        int main(void) {
            if (!check_int(9223372036854775813ul, 5)) {
                return 1;
            }
            if (!check_long(2147483658u, 2147483658l)) {
                return 2;
            }
            if (!check_ulong(-1, 18446744073709551615UL)) {
                return 3;
            }
            if (return_extended_uint(2147483658u) != 2147483658l) {
                return 4;
            }
            if (return_extended_int(-1) != 18446744073709551615UL) {
                return 5;
            }
            long l = return_truncated_ulong(1125902054326372ul);
            if (l != -2147483548l) {
                return 6;
            }
            if (!extend_on_assignment(2147483658u, 2147483658l)){
                return 7;
            }
            int i = 4294967196u;
            if (i != -100) {
                return 8;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function check_int(converted.0, expected.1) { 
            tmp.0 = converted.0 == expected.1
            return tmp.0
            return 0
        }
        global function check_long(converted.2, expected.3) { 
            tmp.1 = converted.2 == expected.3
            return tmp.1
            return 0
        }
        global function check_ulong(converted.4, expected.5) { 
            tmp.2 = converted.4 == expected.5
            return tmp.2
            return 0
        }
        global function return_extended_uint(u.6) { 
            tmp.3 = zero_extend u.6
            return tmp.3
            return 0
        }
        global function return_extended_int(i.7) { 
            tmp.4 = sign_extend i.7
            return tmp.4
            return 0
        }
        global function return_truncated_ulong(ul.8) { 
            tmp.5 = truncate ul.8
            return tmp.5
            return 0
        }
        global function extend_on_assignment(ui.9, expected.10) { 
            tmp.6 = zero_extend ui.9
            result.11 = tmp.6
            tmp.7 = result.11 == expected.10
            return tmp.7
            return 0
        }
        global function main() { 
            tmp.8 = truncate 9223372036854775813UL
            tmp.9 = check_int(tmp.8, 5)
            tmp.10 = ! tmp.9
            if !tmp.10 jump end_if_0
            return 1
        
          end_if_0:
            tmp.11 = zero_extend 2147483658U
            tmp.12 = check_long(tmp.11, 2147483658L)
            tmp.13 = ! tmp.12
            if !tmp.13 jump end_if_2
            return 2
        
          end_if_2:
            tmp.14 = - 1
            tmp.15 = sign_extend tmp.14
            tmp.16 = check_ulong(tmp.15, 18446744073709551615UL)
            tmp.17 = ! tmp.16
            if !tmp.17 jump end_if_4
            return 3
        
          end_if_4:
            tmp.18 = return_extended_uint(2147483658U)
            tmp.19 = tmp.18 != 2147483658L
            if !tmp.19 jump end_if_6
            return 4
        
          end_if_6:
            tmp.20 = - 1
            tmp.21 = return_extended_int(tmp.20)
            tmp.22 = tmp.21 != 18446744073709551615UL
            if !tmp.22 jump end_if_8
            return 5
        
          end_if_8:
            tmp.23 = return_truncated_ulong(1125902054326372UL)
            tmp.24 = sign_extend tmp.23
            l.12 = tmp.24
            tmp.26 = - 2147483548L
            tmp.25 = l.12 != tmp.26
            if !tmp.25 jump end_if_10
            return 6
        
          end_if_10:
            tmp.27 = extend_on_assignment(2147483658U, 2147483658L)
            tmp.28 = ! tmp.27
            if !tmp.28 jump end_if_12
            return 7
        
          end_if_12:
            tmp.29 = 4294967196U
            i.13 = tmp.29
            tmp.31 = - 100
            tmp.30 = i.13 != tmp.31
            if !tmp.30 jump end_if_14
            return 8
        
          end_if_14:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_12_valid_implicit_casts_promote_constants() {
    let src = r#"
        long negative_one = 1l;
        long zero = 0l;
        int main(void) {
            negative_one = -negative_one;
            if (68719476736u >= negative_one) {
                return 1;
            }
            if (-2147483658 >= zero) {
                return 2;
            }
            if (!(3ul + 4294967293ul)) {
                return 3;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = - negative_one
            negative_one = tmp.0
            tmp.2 = negative_one
            tmp.1 = 68719476736UL >= tmp.2
            if !tmp.1 jump end_if_0
            return 1
        
          end_if_0:
            tmp.3 = - 2147483658L
            tmp.4 = tmp.3 >= zero
            if !tmp.4 jump end_if_2
            return 2
        
          end_if_2:
            tmp.5 = 3UL + 4294967293UL
            tmp.6 = ! tmp.5
            if !tmp.6 jump end_if_4
            return 3
        
          end_if_4:
            return 0
            return 0
        }
        static global negative_one: Long = 1L
        static global zero: Long = 0L
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_12_valid_implicit_casts_static_initializers() {
    let src = r#"
        unsigned int u = 1152921506754330636l;
        int i = 2147483650u;
        long l = 9223372036854775900u;
        long l2 = 2147483650u;
        unsigned long ul = 4294967294u;
        unsigned long ul2 = 9223372036854775798l;
        int i2 = 9223372039002259606ul;
        unsigned ui2 = 9223372039002259606ul;
        int main(void)
        {
            if (u != 2147483660u)
                return 1;
            if (i != -2147483646)
                return 2;
            if (l != -9223372036854775716l)
                return 3;
            if (l2 != 2147483650l)
                return 4;
            if (ul != 4294967294ul)
                return 5;
            if (ul2 != 9223372036854775798ul)
                return 6;
            if (i2 != -2147483498)
                return 7;
            if (ui2 != 2147483798u)
                return 8;
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = u != 2147483660U
            if !tmp.0 jump end_if_0
            return 1
        
          end_if_0:
            tmp.2 = - 2147483646
            tmp.1 = i != tmp.2
            if !tmp.1 jump end_if_2
            return 2
        
          end_if_2:
            tmp.4 = - 9223372036854775716L
            tmp.3 = l != tmp.4
            if !tmp.3 jump end_if_4
            return 3
        
          end_if_4:
            tmp.5 = l2 != 2147483650L
            if !tmp.5 jump end_if_6
            return 4
        
          end_if_6:
            tmp.6 = ul != 4294967294UL
            if !tmp.6 jump end_if_8
            return 5
        
          end_if_8:
            tmp.7 = ul2 != 9223372036854775798UL
            if !tmp.7 jump end_if_10
            return 6
        
          end_if_10:
            tmp.9 = - 2147483498
            tmp.8 = i2 != tmp.9
            if !tmp.8 jump end_if_12
            return 7
        
          end_if_12:
            tmp.10 = ui2 != 2147483798U
            if !tmp.10 jump end_if_14
            return 8
        
          end_if_14:
            return 0
            return 0
        }
        static global i: Int = -2147483646
        static global i2: Int = -2147483498
        static global l: Long = -9223372036854775716L
        static global l2: Long = 2147483650L
        static global u: Unsigned Int = 2147483660U
        static global ui2: Unsigned Int = 2147483798U
        static global ul: Unsigned Long = 4294967294UL
        static global ul2: Unsigned Long = 9223372036854775798UL
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_12_valid_libraries_unsigned_args() {
    let src = r#"
        int accept_unsigned(unsigned int a, unsigned int b, unsigned long c, unsigned long d,
                         unsigned int e, unsigned int f, unsigned long g, unsigned int h,
                         unsigned long i) {
            if (a != 1u) {
                return 1;
            }
            if (b != 4294967295U) {
                return 2;
            }
            if (c != 18446744073709551615UL) {
                return 3;
            }
            if (d != 9223372036854775808ul) {
                return 4;
            }
            if (e != 2147483648u) {
                return 5;
            }
            if (f != 0u) {
                return 8;
            }
            if (g != 123456u) {
                return 9;
            }
            if (h != 2147487744u) {
                return 10;
            }
            if (i != 9223372041149743104ul) {
                return 11;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function accept_unsigned(a.0, b.1, c.2, d.3, e.4, f.5, g.6, h.7, i.8) { 
            tmp.0 = a.0 != 1U
            if !tmp.0 jump end_if_0
            return 1
        
          end_if_0:
            tmp.1 = b.1 != 4294967295U
            if !tmp.1 jump end_if_2
            return 2
        
          end_if_2:
            tmp.2 = c.2 != 18446744073709551615UL
            if !tmp.2 jump end_if_4
            return 3
        
          end_if_4:
            tmp.3 = d.3 != 9223372036854775808UL
            if !tmp.3 jump end_if_6
            return 4
        
          end_if_6:
            tmp.4 = e.4 != 2147483648U
            if !tmp.4 jump end_if_8
            return 5
        
          end_if_8:
            tmp.5 = f.5 != 0U
            if !tmp.5 jump end_if_10
            return 8
        
          end_if_10:
            tmp.7 = zero_extend 123456U
            tmp.6 = g.6 != tmp.7
            if !tmp.6 jump end_if_12
            return 9
        
          end_if_12:
            tmp.8 = h.7 != 2147487744U
            if !tmp.8 jump end_if_14
            return 10
        
          end_if_14:
            tmp.9 = i.8 != 9223372041149743104UL
            if !tmp.9 jump end_if_16
            return 11
        
          end_if_16:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_12_valid_libraries_unsigned_args_client() {
    let src = r#"
        
        int accept_unsigned(unsigned int a, unsigned int b, unsigned long c, unsigned long d,
                         unsigned int e, unsigned int f, unsigned long g, unsigned int h,
                         unsigned long i);
        int main(void) {
            return accept_unsigned(1, -1, -1, 9223372036854775808ul, 2147483648ul, 0, 123456, 2147487744u, 9223372041149743104ul);
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 1
            tmp.1 = - 1
            tmp.2 = tmp.1
            tmp.3 = - 1
            tmp.4 = sign_extend tmp.3
            tmp.5 = truncate 2147483648UL
            tmp.6 = 0
            tmp.7 = sign_extend 123456
            tmp.8 = accept_unsigned(tmp.0, tmp.2, tmp.4, 9223372036854775808UL, tmp.5, tmp.6, tmp.7, 2147487744U, 9223372041149743104UL)
            return tmp.8
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_12_valid_libraries_unsigned_global_var() {
    let src = r#"
        unsigned int ui = 4294967200u;
        unsigned int return_uint(void) {
            return ui;
        }
        int return_uint_as_signed(void) {
            return ui;
        }
        long return_uint_as_long(void) {
            return ui;
        }
    "#;
    let expected = r#"
        global function return_uint() { 
            return ui
            return 0
        }
        global function return_uint_as_signed() { 
            tmp.0 = ui
            return tmp.0
            return 0
        }
        global function return_uint_as_long() { 
            tmp.1 = zero_extend ui
            return tmp.1
            return 0
        }
        static global ui: Unsigned Int = 4294967200U
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_12_valid_libraries_unsigned_global_var_client() {
    let src = r#"
        extern unsigned int ui;
        unsigned int return_uint(void);
        int return_uint_as_signed(void);
        long return_uint_as_long(void);
        int main(void) {
            if (ui != 4294967200u)
                return 0;
            ui = -1;
            long result = (long) return_uint();
            if (result != 4294967295l)
                return 0;
            result = (long) return_uint_as_signed();
            if (result != -1l)
                return 0;
            result = return_uint_as_long();
            if (result != 4294967295l)
                return 0;
            return 1;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = ui != 4294967200U
            if !tmp.0 jump end_if_0
            return 0
        
          end_if_0:
            tmp.1 = - 1
            tmp.2 = tmp.1
            ui = tmp.2
            tmp.3 = return_uint()
            tmp.4 = zero_extend tmp.3
            result.0 = tmp.4
            tmp.5 = result.0 != 4294967295L
            if !tmp.5 jump end_if_2
            return 0
        
          end_if_2:
            tmp.6 = return_uint_as_signed()
            tmp.7 = sign_extend tmp.6
            result.0 = tmp.7
            tmp.9 = - 1L
            tmp.8 = result.0 != tmp.9
            if !tmp.8 jump end_if_4
            return 0
        
          end_if_4:
            tmp.10 = return_uint_as_long()
            result.0 = tmp.10
            tmp.11 = result.0 != 4294967295L
            if !tmp.11 jump end_if_6
            return 0
        
          end_if_6:
            return 1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_12_valid_type_specifiers_signed_type_specifiers() {
    let src = r#"
        static int i;
        signed extern i;
        int static signed i = 5;
        signed int static i;
        long signed l;
        long l = 7;
        int long l;
        signed long int l;
        int main(void) {
            int signed extern i;
            extern signed long l;
            if (i != 5) {
                return 1;
            }
            if (l != 7) {
                return 2;
            }
            int counter = 0;
            for (signed int index = 10; index > 0; index = index - 1) {
                counter = counter + 1;
            }
            if (counter != 10) {
                return 3;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = i != 5
            if !tmp.0 jump end_if_0
            return 1
        
          end_if_0:
            tmp.2 = sign_extend 7
            tmp.1 = l != tmp.2
            if !tmp.1 jump end_if_2
            return 2
        
          end_if_2:
            counter.2 = 0
            index.3 = 10
        
          start_loop_0:
            tmp.3 = index.3 > 0
            if !tmp.3 jump break_loop_0
            tmp.4 = counter.2 + 1
            counter.2 = tmp.4
        
          continue_loop_0:
            tmp.5 = index.3 - 1
            index.3 = tmp.5
            jump start_loop_0
        
          break_loop_0:
            tmp.6 = counter.2 != 10
            if !tmp.6 jump end_if_4
            return 3
        
          end_if_4:
            return 0
            return 0
        }
        static i: Int = 5
        static global l: Long = 7L
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_12_valid_type_specifiers_unsigned_type_specifiers() {
    let src = r#"
        unsigned u;
        int unsigned u;
        unsigned int u = 6;
        unsigned long ul;
        long unsigned ul;
        long int unsigned ul;
        unsigned int long ul = 4;
        int main(void) {
            if (u != 6u) {
                return 1;
            }
            long extern unsigned ul;
            unsigned long extern ul;
            int extern unsigned long ul;
            if (ul != 4ul) {
                return 2;
            }
            int counter = 0;
            for (unsigned int index = 10; index < 4294967295U; index = index - 1) {
                counter = counter + 1;
            }
            if (counter != 11) {
                return 3;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = u != 6U
            if !tmp.0 jump end_if_0
            return 1
        
          end_if_0:
            tmp.1 = ul != 4UL
            if !tmp.1 jump end_if_2
            return 2
        
          end_if_2:
            counter.3 = 0
            tmp.2 = 10
            index.4 = tmp.2
        
          start_loop_0:
            tmp.3 = index.4 < 4294967295U
            if !tmp.3 jump break_loop_0
            tmp.4 = counter.3 + 1
            counter.3 = tmp.4
        
          continue_loop_0:
            tmp.6 = 1
            tmp.5 = index.4 - tmp.6
            index.4 = tmp.5
            jump start_loop_0
        
          break_loop_0:
            tmp.7 = counter.3 != 11
            if !tmp.7 jump end_if_4
            return 3
        
          end_if_4:
            return 0
            return 0
        }
        static global u: Unsigned Int = 6U
        static global ul: Unsigned Long = 4UL
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_12_valid_unsigned_expressions_arithmetic_ops() {
    let src = r#"
        unsigned int ui_a;
        unsigned int ui_b;
        unsigned long ul_a;
        unsigned long ul_b;
        int addition(void) {
            return (ui_a + 2147483653u == 2147483663u);
        }
        int subtraction(void) {
            return (ul_a - ul_b == 18446744072635808792ul);
        }
        int multiplication(void) {
            return (ui_a * ui_b == 3221225472u);
        }
        int division(void) {
            return (ui_a / ui_b == 0);
        }
        int division_large_dividend(void) {
            return (ui_a / ui_b == 2);
        }
        int division_by_literal(void) {
            return (ul_a / 5ul == 219902325555ul);
        }
        int remaind(void) {
            return (ul_b % ul_a == 5ul);
        }
        int complement(void) {
            return (~ui_a == 0);
        }
        int main(void) {
            ui_a = 10u;
            if (!addition()) {
                return 1;
            }
            ul_a = 18446744072635809792ul;
            ul_b = 1000ul;
            if (!subtraction()) {
                return 2;
            }
            ui_a = 1073741824u;
            ui_b = 3u;
            if (!multiplication()) {
                return 3;
            }
            ui_a = 100u;
            ui_b = 4294967294u;
            if (!division()) {
                return 4;
            }
            ui_a = 4294967294u;
            ui_b = 2147483647u;
            if (!division_large_dividend()) {
                return 5;
            }
            ul_a = 1099511627775ul;
            if (!division_by_literal()) {
                return 6;
            }
            ul_a = 100ul;
            ul_b = 18446744073709551605ul;
            if (!remaind()) {
                return 7;
            }
            ui_a = 4294967295U;
            if (!complement()) {
                return 8;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function addition() { 
            tmp.0 = ui_a + 2147483653U
            tmp.1 = tmp.0 == 2147483663U
            return tmp.1
            return 0
        }
        global function subtraction() { 
            tmp.2 = ul_a - ul_b
            tmp.3 = tmp.2 == 18446744072635808792UL
            return tmp.3
            return 0
        }
        global function multiplication() { 
            tmp.4 = ui_a * ui_b
            tmp.5 = tmp.4 == 3221225472U
            return tmp.5
            return 0
        }
        global function division() { 
            tmp.6 = ui_a / ui_b
            tmp.8 = 0
            tmp.7 = tmp.6 == tmp.8
            return tmp.7
            return 0
        }
        global function division_large_dividend() { 
            tmp.9 = ui_a / ui_b
            tmp.11 = 2
            tmp.10 = tmp.9 == tmp.11
            return tmp.10
            return 0
        }
        global function division_by_literal() { 
            tmp.12 = ul_a / 5UL
            tmp.13 = tmp.12 == 219902325555UL
            return tmp.13
            return 0
        }
        global function remaind() { 
            tmp.14 = ul_b % ul_a
            tmp.15 = tmp.14 == 5UL
            return tmp.15
            return 0
        }
        global function complement() { 
            tmp.16 = ~ ui_a
            tmp.18 = 0
            tmp.17 = tmp.16 == tmp.18
            return tmp.17
            return 0
        }
        global function main() { 
            ui_a = 10U
            tmp.19 = addition()
            tmp.20 = ! tmp.19
            if !tmp.20 jump end_if_0
            return 1
        
          end_if_0:
            ul_a = 18446744072635809792UL
            ul_b = 1000UL
            tmp.21 = subtraction()
            tmp.22 = ! tmp.21
            if !tmp.22 jump end_if_2
            return 2
        
          end_if_2:
            ui_a = 1073741824U
            ui_b = 3U
            tmp.23 = multiplication()
            tmp.24 = ! tmp.23
            if !tmp.24 jump end_if_4
            return 3
        
          end_if_4:
            ui_a = 100U
            ui_b = 4294967294U
            tmp.25 = division()
            tmp.26 = ! tmp.25
            if !tmp.26 jump end_if_6
            return 4
        
          end_if_6:
            ui_a = 4294967294U
            ui_b = 2147483647U
            tmp.27 = division_large_dividend()
            tmp.28 = ! tmp.27
            if !tmp.28 jump end_if_8
            return 5
        
          end_if_8:
            ul_a = 1099511627775UL
            tmp.29 = division_by_literal()
            tmp.30 = ! tmp.29
            if !tmp.30 jump end_if_10
            return 6
        
          end_if_10:
            ul_a = 100UL
            ul_b = 18446744073709551605UL
            tmp.31 = remaind()
            tmp.32 = ! tmp.31
            if !tmp.32 jump end_if_12
            return 7
        
          end_if_12:
            ui_a = 4294967295U
            tmp.33 = complement()
            tmp.34 = ! tmp.33
            if !tmp.34 jump end_if_14
            return 8
        
          end_if_14:
            return 0
            return 0
        }
        static global ui_a: Unsigned Int = 0U
        static global ui_b: Unsigned Int = 0U
        static global ul_a: Unsigned Long = 0UL
        static global ul_b: Unsigned Long = 0UL
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_12_valid_unsigned_expressions_arithmetic_wraparound() {
    let src = r#"
        unsigned int ui_a;
        unsigned int ui_b;
        unsigned long ul_a;
        unsigned long ul_b;
        int addition(void) {
            return ui_a + ui_b == 0u;
        }
        int subtraction(void) {
            return (ul_a - ul_b == 18446744073709551606ul);
        }
        int neg(void) {
            return -ul_a == 18446744073709551615UL;
        }
        int main(void) {
            ui_a = 4294967293u;
            ui_b = 2u;
            if (!addition()) {
                return 1;
            }
            ul_a = 10ul;
            ul_b = 20ul;
            if (!subtraction()) {
                return 2;
            }
            ul_a = 1ul;
            if (!neg()) {
                return 3;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function addition() { 
            tmp.0 = ui_a + ui_b
            tmp.1 = tmp.0 == 0U
            return tmp.1
            return 0
        }
        global function subtraction() { 
            tmp.2 = ul_a - ul_b
            tmp.3 = tmp.2 == 18446744073709551606UL
            return tmp.3
            return 0
        }
        global function neg() { 
            tmp.4 = - ul_a
            tmp.5 = tmp.4 == 18446744073709551615UL
            return tmp.5
            return 0
        }
        global function main() { 
            ui_a = 4294967293U
            ui_b = 2U
            tmp.6 = addition()
            tmp.7 = ! tmp.6
            if !tmp.7 jump end_if_0
            return 1
        
          end_if_0:
            ul_a = 10UL
            ul_b = 20UL
            tmp.8 = subtraction()
            tmp.9 = ! tmp.8
            if !tmp.9 jump end_if_2
            return 2
        
          end_if_2:
            ul_a = 1UL
            tmp.10 = neg()
            tmp.11 = ! tmp.10
            if !tmp.11 jump end_if_4
            return 3
        
          end_if_4:
            return 0
            return 0
        }
        static global ui_a: Unsigned Int = 0U
        static global ui_b: Unsigned Int = 0U
        static global ul_a: Unsigned Long = 0UL
        static global ul_b: Unsigned Long = 0UL
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_12_valid_unsigned_expressions_comparisons() {
    let src = r#"
        unsigned int one_hundred = 100u;
        unsigned int large_uint = 4294967294u;
        unsigned long one_hundred_ulong = 100ul;
        unsigned long large_ulong = 4294967294ul;
        int main(void) {
            if (large_uint < one_hundred)
                return 1;
            if (large_uint <= one_hundred)
                return 2;
            if (one_hundred >= large_uint)
                return 3;
            if (one_hundred > large_uint)
                return 4;
            if (!(one_hundred <= large_uint))
                return 5;
            if (!(one_hundred < large_uint))
                return 6;
            if (!(large_uint > one_hundred))
                return 7;
            if (!(large_uint >= one_hundred))
                return 8;
            if (large_ulong < one_hundred_ulong)
                return 9;
            if (large_ulong <= one_hundred_ulong)
                return 10;
            if (one_hundred_ulong >= large_ulong)
                return 11;
            if (one_hundred_ulong > large_ulong)
                return 12;
            if (!(one_hundred_ulong <= large_ulong))
                return 13;
            if (!(one_hundred_ulong < large_ulong))
                return 14;
            if (!(large_ulong > one_hundred_ulong))
                return 15;
            if (!(large_ulong >= one_hundred_ulong))
                return 16;
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = large_uint < one_hundred
            if !tmp.0 jump end_if_0
            return 1
        
          end_if_0:
            tmp.1 = large_uint <= one_hundred
            if !tmp.1 jump end_if_2
            return 2
        
          end_if_2:
            tmp.2 = one_hundred >= large_uint
            if !tmp.2 jump end_if_4
            return 3
        
          end_if_4:
            tmp.3 = one_hundred > large_uint
            if !tmp.3 jump end_if_6
            return 4
        
          end_if_6:
            tmp.4 = one_hundred <= large_uint
            tmp.5 = ! tmp.4
            if !tmp.5 jump end_if_8
            return 5
        
          end_if_8:
            tmp.6 = one_hundred < large_uint
            tmp.7 = ! tmp.6
            if !tmp.7 jump end_if_10
            return 6
        
          end_if_10:
            tmp.8 = large_uint > one_hundred
            tmp.9 = ! tmp.8
            if !tmp.9 jump end_if_12
            return 7
        
          end_if_12:
            tmp.10 = large_uint >= one_hundred
            tmp.11 = ! tmp.10
            if !tmp.11 jump end_if_14
            return 8
        
          end_if_14:
            tmp.12 = large_ulong < one_hundred_ulong
            if !tmp.12 jump end_if_16
            return 9
        
          end_if_16:
            tmp.13 = large_ulong <= one_hundred_ulong
            if !tmp.13 jump end_if_18
            return 10
        
          end_if_18:
            tmp.14 = one_hundred_ulong >= large_ulong
            if !tmp.14 jump end_if_20
            return 11
        
          end_if_20:
            tmp.15 = one_hundred_ulong > large_ulong
            if !tmp.15 jump end_if_22
            return 12
        
          end_if_22:
            tmp.16 = one_hundred_ulong <= large_ulong
            tmp.17 = ! tmp.16
            if !tmp.17 jump end_if_24
            return 13
        
          end_if_24:
            tmp.18 = one_hundred_ulong < large_ulong
            tmp.19 = ! tmp.18
            if !tmp.19 jump end_if_26
            return 14
        
          end_if_26:
            tmp.20 = large_ulong > one_hundred_ulong
            tmp.21 = ! tmp.20
            if !tmp.21 jump end_if_28
            return 15
        
          end_if_28:
            tmp.22 = large_ulong >= one_hundred_ulong
            tmp.23 = ! tmp.22
            if !tmp.23 jump end_if_30
            return 16
        
          end_if_30:
            return 0
            return 0
        }
        static global large_uint: Unsigned Int = 4294967294U
        static global large_ulong: Unsigned Long = 4294967294UL
        static global one_hundred: Unsigned Int = 100U
        static global one_hundred_ulong: Unsigned Long = 100UL
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_12_valid_unsigned_expressions_locals() {
    let src = r#"
        int main(void) {
            unsigned long a = 8589934592ul;
            int b = -1;
            long c = -8589934592l;
            unsigned int d = 10u;
            if (a != 8589934592ul) {
                return 1;
            }
            if (b != -1){
                return 2;
            }
            if (c != -8589934592l) {
                return 3;
            }
            if (d != 10u) {
                return 4;
            }
            a = -a;
            b = b - 1;
            c = c + 8589934594l;
            d = d * 268435456u;
            if (a != 18446744065119617024ul) {
                return 5;
            }
            if (b != -2) {
                return 6;
            }
            if (c != 2) {
                return 7;
            }
            if (d != 2684354560u) {
                return 8;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 8589934592UL
            tmp.0 = - 1
            b.1 = tmp.0
            tmp.1 = - 8589934592L
            c.2 = tmp.1
            d.3 = 10U
            tmp.2 = a.0 != 8589934592UL
            if !tmp.2 jump end_if_0
            return 1
        
          end_if_0:
            tmp.4 = - 1
            tmp.3 = b.1 != tmp.4
            if !tmp.3 jump end_if_2
            return 2
        
          end_if_2:
            tmp.6 = - 8589934592L
            tmp.5 = c.2 != tmp.6
            if !tmp.5 jump end_if_4
            return 3
        
          end_if_4:
            tmp.7 = d.3 != 10U
            if !tmp.7 jump end_if_6
            return 4
        
          end_if_6:
            tmp.8 = - a.0
            a.0 = tmp.8
            tmp.9 = b.1 - 1
            b.1 = tmp.9
            tmp.10 = c.2 + 8589934594L
            c.2 = tmp.10
            tmp.11 = d.3 * 268435456U
            d.3 = tmp.11
            tmp.12 = a.0 != 18446744065119617024UL
            if !tmp.12 jump end_if_8
            return 5
        
          end_if_8:
            tmp.14 = - 2
            tmp.13 = b.1 != tmp.14
            if !tmp.13 jump end_if_10
            return 6
        
          end_if_10:
            tmp.16 = sign_extend 2
            tmp.15 = c.2 != tmp.16
            if !tmp.15 jump end_if_12
            return 7
        
          end_if_12:
            tmp.17 = d.3 != 2684354560U
            if !tmp.17 jump end_if_14
            return 8
        
          end_if_14:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_12_valid_unsigned_expressions_logical() {
    let src = r#"
        int not(unsigned long ul) {
            return !ul;
        }
        int if_cond(unsigned u) {
            if (u) {
                return 1;
            }
            return 0;
        }
        int and(unsigned long ul, int i) {
            return ul && i;
        }
        int or(int i, unsigned u) {
            return i || u;
        }
        int main(void) {
            unsigned long ul = 1152921504606846976ul;
            unsigned int u = 2147483648u;
            unsigned long zero = 0l;
            if (not(ul)) {
                return 1;
            }
            if (!not(zero)) {
                return 2;
            }
            if(!if_cond(u)) {
                return 3;
            }
            if(if_cond(zero)) {
                return 4;
            }
            if (and(zero, 1)) {
                return 5;
            }
            if (!or(1, u)) {
                return 6;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function not(ul.0) { 
            tmp.0 = ! ul.0
            return tmp.0
            return 0
        }
        global function if_cond(u.1) { 
            if !u.1 jump end_if_0
            return 1
        
          end_if_0:
            return 0
            return 0
        }
        global function and(ul.2, i.3) { 
            if !ul.2 jump and_false_2
            if !i.3 jump and_false_2
            tmp.2 = 1
            jump and_end_3
        
          and_false_2:
            tmp.2 = 0
        
          and_end_3:
            return tmp.2
            return 0
        }
        global function or(i.4, u.5) { 
            if i.4 jump or_true_4
            if u.5 jump or_true_4
            tmp.4 = 0
            jump or_end_5
        
          or_true_4:
            tmp.4 = 1
        
          or_end_5:
            return tmp.4
            return 0
        }
        global function main() { 
            ul.6 = 1152921504606846976UL
            u.7 = 2147483648U
            tmp.5 = 0L
            zero.8 = tmp.5
            tmp.6 = not(ul.6)
            if !tmp.6 jump end_if_6
            return 1
        
          end_if_6:
            tmp.7 = not(zero.8)
            tmp.8 = ! tmp.7
            if !tmp.8 jump end_if_8
            return 2
        
          end_if_8:
            tmp.9 = if_cond(u.7)
            tmp.10 = ! tmp.9
            if !tmp.10 jump end_if_10
            return 3
        
          end_if_10:
            tmp.11 = truncate zero.8
            tmp.12 = if_cond(tmp.11)
            if !tmp.12 jump end_if_12
            return 4
        
          end_if_12:
            tmp.13 = and(zero.8, 1)
            if !tmp.13 jump end_if_14
            return 5
        
          end_if_14:
            tmp.14 = or(1, u.7)
            tmp.15 = ! tmp.14
            if !tmp.15 jump end_if_16
            return 6
        
          end_if_16:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_12_valid_unsigned_expressions_simple() {
    let src = r#"
        int main(void) {
            unsigned u = 2147483647u;
            return (u + 2u == 2147483649u);
        }
    "#;
    let expected = r#"
        global function main() { 
            u.0 = 2147483647U
            tmp.0 = u.0 + 2U
            tmp.1 = tmp.0 == 2147483649U
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_12_valid_unsigned_expressions_static_variables() {
    let src = r#"
        
        static unsigned long x = 9223372036854775803ul;
        unsigned long zero_long;
        unsigned zero_int;
        int main(void)
        {
            if (x != 9223372036854775803ul)
                return 0;
            x = x + 10;
            if (x != 9223372036854775813ul)
                return 0;
            if (zero_long || zero_int)
                return 0;
            return 1;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = x != 9223372036854775803UL
            if !tmp.0 jump end_if_0
            return 0
        
          end_if_0:
            tmp.2 = sign_extend 10
            tmp.1 = x + tmp.2
            x = tmp.1
            tmp.3 = x != 9223372036854775813UL
            if !tmp.3 jump end_if_2
            return 0
        
          end_if_2:
            if zero_long jump or_true_4
            if zero_int jump or_true_4
            tmp.5 = 0
            jump or_end_5
        
          or_true_4:
            tmp.5 = 1
        
          or_end_5:
            if !tmp.5 jump end_if_6
            return 0
        
          end_if_6:
            return 1
            return 0
        }
        static x: Unsigned Long = 9223372036854775803UL
        static global zero_int: Unsigned Int = 0U
        static global zero_long: Unsigned Long = 0UL
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_13_valid_constants_constant_doubles() {
    let src = r#"
        int main(void) {
            double a = 1.0;
            double b = 1.;
            double c = 1E0;
            double d = .01e+2;
            if (! (a == b && a == c && a == d) )
                return 1;
            if (a + b + c + d != 4.0)
                return 2;
            double e = .125;
            double f = 12.5e-2;
            double g = 125.E-3;
            double h = 1250000000e-10;
            if (! (e == f && e == g && e == h) )
                return 3;
            if (e + f + g + h != 0.5)
                return 4;
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 1D
            b.1 = 1D
            c.2 = 1D
            d.3 = 1D
            tmp.0 = a.0 == b.1
            if !tmp.0 jump and_false_0
            tmp.3 = a.0 == c.2
            if !tmp.3 jump and_false_0
            tmp.2 = 1
            jump and_end_1
        
          and_false_0:
            tmp.2 = 0
        
          and_end_1:
            if !tmp.2 jump and_false_2
            tmp.6 = a.0 == d.3
            if !tmp.6 jump and_false_2
            tmp.5 = 1
            jump and_end_3
        
          and_false_2:
            tmp.5 = 0
        
          and_end_3:
            tmp.7 = ! tmp.5
            if !tmp.7 jump end_if_4
            return 1
        
          end_if_4:
            tmp.8 = a.0 + b.1
            tmp.9 = tmp.8 + c.2
            tmp.10 = tmp.9 + d.3
            tmp.11 = tmp.10 != 4D
            if !tmp.11 jump end_if_6
            return 2
        
          end_if_6:
            e.4 = 0.125D
            f.5 = 0.125D
            g.6 = 0.125D
            h.7 = 0.125D
            tmp.12 = e.4 == f.5
            if !tmp.12 jump and_false_8
            tmp.15 = e.4 == g.6
            if !tmp.15 jump and_false_8
            tmp.14 = 1
            jump and_end_9
        
          and_false_8:
            tmp.14 = 0
        
          and_end_9:
            if !tmp.14 jump and_false_10
            tmp.18 = e.4 == h.7
            if !tmp.18 jump and_false_10
            tmp.17 = 1
            jump and_end_11
        
          and_false_10:
            tmp.17 = 0
        
          and_end_11:
            tmp.19 = ! tmp.17
            if !tmp.19 jump end_if_12
            return 3
        
          end_if_12:
            tmp.20 = e.4 + f.5
            tmp.21 = tmp.20 + g.6
            tmp.22 = tmp.21 + h.7
            tmp.23 = tmp.22 != 0.5D
            if !tmp.23 jump end_if_14
            return 4
        
          end_if_14:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_13_valid_constants_round_constants() {
    let src = r#"
        int main(void) {
            if (1.00000000000000033306690738754696212708950042724609375 != 1.0000000000000004) {
                return 1;
            }
            if (9223372036854776832.5 != 9223372036854777856.0) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 1.0000000000000004D != 1.0000000000000004D
            if !tmp.0 jump end_if_0
            return 1
        
          end_if_0:
            tmp.1 = 9223372036854778000D != 9223372036854778000D
            if !tmp.1 jump end_if_2
            return 2
        
          end_if_2:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_13_valid_explicit_casts_cvttsd2si_rewrite() {
    let src = r#"
        double glob = 3.0;
        int main(void) {
            long l = -1l;
            int i = -1;
            int j = (int) glob;
            int k = 20;
            if (l != -1l) {
                return 1;
            }
            if (i != -1) {
                return 2;
            }
            if (j != 3) {
                return 3;
            }
            if (k != 20) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = - 1L
            l.0 = tmp.0
            tmp.1 = - 1
            i.1 = tmp.1
            tmp.2 = double_to_int glob
            j.2 = tmp.2
            k.3 = 20
            tmp.4 = - 1L
            tmp.3 = l.0 != tmp.4
            if !tmp.3 jump end_if_0
            return 1
        
          end_if_0:
            tmp.6 = - 1
            tmp.5 = i.1 != tmp.6
            if !tmp.5 jump end_if_2
            return 2
        
          end_if_2:
            tmp.7 = j.2 != 3
            if !tmp.7 jump end_if_4
            return 3
        
          end_if_4:
            tmp.8 = k.3 != 20
            if !tmp.8 jump end_if_6
            return 4
        
          end_if_6:
            return 0
            return 0
        }
        static global glob: Double = 3D
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_13_valid_explicit_casts_double_to_signed() {
    let src = r#"
        int double_to_int(double d) {
            return (int) d;
        }
        long double_to_long(double d) {
            return (long) d;
        }
        int main(void) {
            long l = double_to_long(2148429099.3);
            if (l != 2148429099l) {
                return 1;
            }
            int i = double_to_int(-200000.9999);
            if (i != -200000) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function double_to_int(d.0) { 
            tmp.0 = double_to_int d.0
            return tmp.0
            return 0
        }
        global function double_to_long(d.1) { 
            tmp.1 = double_to_int d.1
            return tmp.1
            return 0
        }
        global function main() { 
            tmp.2 = double_to_long(2148429099.3D)
            l.2 = tmp.2
            tmp.3 = l.2 != 2148429099L
            if !tmp.3 jump end_if_0
            return 1
        
          end_if_0:
            tmp.4 = - 200000.9999D
            tmp.5 = double_to_int(tmp.4)
            i.3 = tmp.5
            tmp.7 = - 200000
            tmp.6 = i.3 != tmp.7
            if !tmp.6 jump end_if_2
            return 2
        
          end_if_2:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_13_valid_explicit_casts_double_to_unsigned() {
    let src = r#"
        unsigned int double_to_uint(double d) {
            return (unsigned int) d;
        }
        unsigned long double_to_ulong(double d) {
            return (unsigned long) d;
        }
        int main(void) {
            if (double_to_uint(10.9) != 10u) {
                return 1;
            }
            if (double_to_uint(2147483750.5) != 2147483750) {
                return 2;
            }
            if (double_to_ulong(34359738368.5) != 34359738368ul) {
                return 3;
            }
            if (double_to_ulong(3458764513821589504.0) != 3458764513821589504ul) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function double_to_uint(d.0) { 
            tmp.0 = double_to_uint d.0
            return tmp.0
            return 0
        }
        global function double_to_ulong(d.1) { 
            tmp.1 = double_to_uint d.1
            return tmp.1
            return 0
        }
        global function main() { 
            tmp.2 = double_to_uint(10.9D)
            tmp.3 = tmp.2 != 10U
            if !tmp.3 jump end_if_0
            return 1
        
          end_if_0:
            tmp.4 = double_to_uint(2147483750.5D)
            tmp.5 = zero_extend tmp.4
            tmp.6 = tmp.5 != 2147483750L
            if !tmp.6 jump end_if_2
            return 2
        
          end_if_2:
            tmp.7 = double_to_ulong(34359738368.5D)
            tmp.8 = tmp.7 != 34359738368UL
            if !tmp.8 jump end_if_4
            return 3
        
          end_if_4:
            tmp.9 = double_to_ulong(3458764513821589500D)
            tmp.10 = tmp.9 != 3458764513821589504UL
            if !tmp.10 jump end_if_6
            return 4
        
          end_if_6:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_13_valid_explicit_casts_rewrite_cvttsd2si_regression() {
    let src = r#"
        int check_12_ints(int start, int a, int b, int c, int d, int e, int f, int g,
                          int h, int i, int j, int k, int l);
        double glob = 5000.;
        int main(void) {
            long should_spill = (long)glob;
            int one = glob - 4999;
            int two = one + one;
            int three = 2 + one;
            int four = two * two;
            int five = 6 - one;
            int six = two * three;
            int seven = one + 6;
            int eight = two * 4;
            int nine = three * three;
            int ten = four + six;
            int eleven = 16 - five;
            int twelve = six + six;
            check_12_ints(one, two, three, four, five, six, seven, eight, nine, ten,
                          eleven, twelve, 1);
            int thirteen = glob - 4987;
            int fourteen = thirteen + 1;
            int fifteen = 28 - thirteen;
            int sixteen = fourteen + 2;
            int seventeen = 4 + thirteen;
            int eighteen = 32 - fourteen;
            int nineteen = 35 - sixteen;
            int twenty = fifteen + 5;
            int twenty_one = thirteen * 2 - 5;
            int twenty_two = fifteen + 7;
            int twenty_three = 6 + seventeen;
            int twenty_four = thirteen + 11;
            check_12_ints(thirteen, fourteen, fifteen, sixteen, seventeen, eighteen,
                          nineteen, twenty, twenty_one, twenty_two, twenty_three,
                          twenty_four, 13);
            if (should_spill != 5000) {
                return -1;
            }
            return 0;
        }
        int check_12_ints(int a, int b, int c, int d, int e, int f, int g, int h, int i,
                          int j, int k, int l, int start) {
            int expected = 0;
            expected = start + 0;
            if (a != expected) {
                return expected;
            }
            expected = start + 1;
            if (b != expected) {
                return expected;
            }
            expected = start + 2;
            if (c != expected) {
                return expected;
            }
            expected = start + 3;
            if (d != expected) {
                return expected;
            }
            expected = start + 4;
            if (e != expected) {
                return expected;
            }
            expected = start + 5;
            if (f != expected) {
                return expected;
            }
            expected = start + 6;
            if (g != expected) {
                return expected;
            }
            expected = start + 7;
            if (h != expected) {
                return expected;
            }
            expected = start + 8;
            if (i != expected) {
                return expected;
            }
            expected = start + 9;
            if (j != expected) {
                return expected;
            }
            expected = start + 10;
            if (k != expected) {
                return expected;
            }
            expected = start + 11;
            if (l != expected) {
                return expected;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = double_to_int glob
            should_spill.13 = tmp.0
            tmp.2 = int_to_double 4999
            tmp.1 = glob - tmp.2
            tmp.3 = double_to_int tmp.1
            one.14 = tmp.3
            tmp.4 = one.14 + one.14
            two.15 = tmp.4
            tmp.5 = 2 + one.14
            three.16 = tmp.5
            tmp.6 = two.15 * two.15
            four.17 = tmp.6
            tmp.7 = 6 - one.14
            five.18 = tmp.7
            tmp.8 = two.15 * three.16
            six.19 = tmp.8
            tmp.9 = one.14 + 6
            seven.20 = tmp.9
            tmp.10 = two.15 * 4
            eight.21 = tmp.10
            tmp.11 = three.16 * three.16
            nine.22 = tmp.11
            tmp.12 = four.17 + six.19
            ten.23 = tmp.12
            tmp.13 = 16 - five.18
            eleven.24 = tmp.13
            tmp.14 = six.19 + six.19
            twelve.25 = tmp.14
            tmp.15 = check_12_ints(one.14, two.15, three.16, four.17, five.18, six.19, seven.20, eight.21, nine.22, ten.23, eleven.24, twelve.25, 1)
            tmp.17 = int_to_double 4987
            tmp.16 = glob - tmp.17
            tmp.18 = double_to_int tmp.16
            thirteen.26 = tmp.18
            tmp.19 = thirteen.26 + 1
            fourteen.27 = tmp.19
            tmp.20 = 28 - thirteen.26
            fifteen.28 = tmp.20
            tmp.21 = fourteen.27 + 2
            sixteen.29 = tmp.21
            tmp.22 = 4 + thirteen.26
            seventeen.30 = tmp.22
            tmp.23 = 32 - fourteen.27
            eighteen.31 = tmp.23
            tmp.24 = 35 - sixteen.29
            nineteen.32 = tmp.24
            tmp.25 = fifteen.28 + 5
            twenty.33 = tmp.25
            tmp.26 = thirteen.26 * 2
            tmp.27 = tmp.26 - 5
            twenty_one.34 = tmp.27
            tmp.28 = fifteen.28 + 7
            twenty_two.35 = tmp.28
            tmp.29 = 6 + seventeen.30
            twenty_three.36 = tmp.29
            tmp.30 = thirteen.26 + 11
            twenty_four.37 = tmp.30
            tmp.31 = check_12_ints(thirteen.26, fourteen.27, fifteen.28, sixteen.29, seventeen.30, eighteen.31, nineteen.32, twenty.33, twenty_one.34, twenty_two.35, twenty_three.36, twenty_four.37, 13)
            tmp.33 = sign_extend 5000
            tmp.32 = should_spill.13 != tmp.33
            if !tmp.32 jump end_if_0
            tmp.34 = - 1
            return tmp.34
        
          end_if_0:
            return 0
            return 0
        }
        global function check_12_ints(a.38, b.39, c.40, d.41, e.42, f.43, g.44, h.45, i.46, j.47, k.48, l.49, start.50) { 
            expected.51 = 0
            tmp.35 = start.50 + 0
            expected.51 = tmp.35
            tmp.36 = a.38 != expected.51
            if !tmp.36 jump end_if_2
            return expected.51
        
          end_if_2:
            tmp.37 = start.50 + 1
            expected.51 = tmp.37
            tmp.38 = b.39 != expected.51
            if !tmp.38 jump end_if_4
            return expected.51
        
          end_if_4:
            tmp.39 = start.50 + 2
            expected.51 = tmp.39
            tmp.40 = c.40 != expected.51
            if !tmp.40 jump end_if_6
            return expected.51
        
          end_if_6:
            tmp.41 = start.50 + 3
            expected.51 = tmp.41
            tmp.42 = d.41 != expected.51
            if !tmp.42 jump end_if_8
            return expected.51
        
          end_if_8:
            tmp.43 = start.50 + 4
            expected.51 = tmp.43
            tmp.44 = e.42 != expected.51
            if !tmp.44 jump end_if_10
            return expected.51
        
          end_if_10:
            tmp.45 = start.50 + 5
            expected.51 = tmp.45
            tmp.46 = f.43 != expected.51
            if !tmp.46 jump end_if_12
            return expected.51
        
          end_if_12:
            tmp.47 = start.50 + 6
            expected.51 = tmp.47
            tmp.48 = g.44 != expected.51
            if !tmp.48 jump end_if_14
            return expected.51
        
          end_if_14:
            tmp.49 = start.50 + 7
            expected.51 = tmp.49
            tmp.50 = h.45 != expected.51
            if !tmp.50 jump end_if_16
            return expected.51
        
          end_if_16:
            tmp.51 = start.50 + 8
            expected.51 = tmp.51
            tmp.52 = i.46 != expected.51
            if !tmp.52 jump end_if_18
            return expected.51
        
          end_if_18:
            tmp.53 = start.50 + 9
            expected.51 = tmp.53
            tmp.54 = j.47 != expected.51
            if !tmp.54 jump end_if_20
            return expected.51
        
          end_if_20:
            tmp.55 = start.50 + 10
            expected.51 = tmp.55
            tmp.56 = k.48 != expected.51
            if !tmp.56 jump end_if_22
            return expected.51
        
          end_if_22:
            tmp.57 = start.50 + 11
            expected.51 = tmp.57
            tmp.58 = l.49 != expected.51
            if !tmp.58 jump end_if_24
            return expected.51
        
          end_if_24:
            return 0
            return 0
        }
        static global glob: Double = 5000D
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_13_valid_explicit_casts_signed_to_double() {
    let src = r#"
        
        double int_to_double(int i) {
            return (double) i;
        }
        double long_to_double(long l) {
            return (double) l;
        }
        int main(void) {
            if (int_to_double(-100000) != -100000.0) {
                return 1;
            }
            if (long_to_double(-9007199254751227l) != -9007199254751228.0) {
                return 2;
            }
            double d = (double) 1152921504606846977l;
            if (d != 1152921504606846976.0) {
                return 3;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function int_to_double(i.0) { 
            tmp.0 = int_to_double i.0
            return tmp.0
            return 0
        }
        global function long_to_double(l.1) { 
            tmp.1 = int_to_double l.1
            return tmp.1
            return 0
        }
        global function main() { 
            tmp.2 = - 100000
            tmp.3 = int_to_double(tmp.2)
            tmp.5 = - 100000D
            tmp.4 = tmp.3 != tmp.5
            if !tmp.4 jump end_if_0
            return 1
        
          end_if_0:
            tmp.6 = - 9007199254751227L
            tmp.7 = long_to_double(tmp.6)
            tmp.9 = - 9007199254751228D
            tmp.8 = tmp.7 != tmp.9
            if !tmp.8 jump end_if_2
            return 2
        
          end_if_2:
            tmp.10 = int_to_double 1152921504606846977L
            d.2 = tmp.10
            tmp.11 = d.2 != 1152921504606847000D
            if !tmp.11 jump end_if_4
            return 3
        
          end_if_4:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_13_valid_explicit_casts_unsigned_to_double() {
    let src = r#"
        
        double uint_to_double(unsigned int ui) {
            return (double) ui;
        }
        double ulong_to_double(unsigned long ul) {
            return (double) ul;
        }
        int main(void) {
            if (uint_to_double(1000u) != 1000.0) {
                return 1;
            }
            if (uint_to_double(4294967200u) != 4294967200.0) {
                return 2;
            }
            if (ulong_to_double(138512825844ul) != 138512825844.0) {
                return 3;
            }
            if (ulong_to_double(10223372036854775816ul) != 10223372036854775808.0) {
                return 4;
            }
            if (ulong_to_double(9223372036854776832ul) != 9223372036854775808.0) {
                return 5;
            }
            if (ulong_to_double(9223372036854776833ul) != 9223372036854777856.0) {
                return 6;
            }
            if (ulong_to_double(9223372036854776831ul) != 9223372036854775808.0) {
                return 7;
            }
            if (ulong_to_double(9223372036854776830ul) != 9223372036854775808.0) {
                return 8;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function uint_to_double(ui.0) { 
            tmp.0 = uint_to_double ui.0
            return tmp.0
            return 0
        }
        global function ulong_to_double(ul.1) { 
            tmp.1 = uint_to_double ul.1
            return tmp.1
            return 0
        }
        global function main() { 
            tmp.2 = uint_to_double(1000U)
            tmp.3 = tmp.2 != 1000D
            if !tmp.3 jump end_if_0
            return 1
        
          end_if_0:
            tmp.4 = uint_to_double(4294967200U)
            tmp.5 = tmp.4 != 4294967200D
            if !tmp.5 jump end_if_2
            return 2
        
          end_if_2:
            tmp.6 = ulong_to_double(138512825844UL)
            tmp.7 = tmp.6 != 138512825844D
            if !tmp.7 jump end_if_4
            return 3
        
          end_if_4:
            tmp.8 = ulong_to_double(10223372036854775816UL)
            tmp.9 = tmp.8 != 10223372036854776000D
            if !tmp.9 jump end_if_6
            return 4
        
          end_if_6:
            tmp.10 = ulong_to_double(9223372036854776832UL)
            tmp.11 = tmp.10 != 9223372036854776000D
            if !tmp.11 jump end_if_8
            return 5
        
          end_if_8:
            tmp.12 = ulong_to_double(9223372036854776833UL)
            tmp.13 = tmp.12 != 9223372036854778000D
            if !tmp.13 jump end_if_10
            return 6
        
          end_if_10:
            tmp.14 = ulong_to_double(9223372036854776831UL)
            tmp.15 = tmp.14 != 9223372036854776000D
            if !tmp.15 jump end_if_12
            return 7
        
          end_if_12:
            tmp.16 = ulong_to_double(9223372036854776830UL)
            tmp.17 = tmp.16 != 9223372036854776000D
            if !tmp.17 jump end_if_14
            return 8
        
          end_if_14:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_13_valid_extra_credit_compound_assign() {
    let src = r#"
        
        int main(void) {
            double d = 10.0;
            d /= 4.0;
            if (d != 2.5) {
                return 1;
            }
            d *= 10000.0;
            if (d != 25000.0) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            d.0 = 10D
            tmp.0 = d.0 / 4D
            d.0 = tmp.0
            tmp.1 = d.0 != 2.5D
            if !tmp.1 jump end_if_0
            return 1
        
          end_if_0:
            tmp.2 = d.0 * 10000D
            d.0 = tmp.2
            tmp.3 = d.0 != 25000D
            if !tmp.3 jump end_if_2
            return 2
        
          end_if_2:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_13_valid_extra_credit_compound_assign_implicit_cast() {
    let src = r#"
        int main(void) {
            double d = 1000.5;
            d += 1000;
            if (d != 2000.5) {
                return 1;
            }
            unsigned long ul = 18446744073709551586ul;
            ul -= 1.5E19;
            if (ul != 3446744073709551616ul) {
                return 2;
            }
            int i = 10;
            i += 0.99999;
            if (i != 10) {
                return 3;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            d.0 = 1000.5D
            tmp.1 = int_to_double 1000
            tmp.0 = d.0 + tmp.1
            d.0 = tmp.0
            tmp.2 = d.0 != 2000.5D
            if !tmp.2 jump end_if_0
            return 1
        
          end_if_0:
            ul.1 = 18446744073709551586UL
            tmp.3 = uint_to_double ul.1
            tmp.4 = tmp.3 - 15000000000000000000D
            tmp.5 = double_to_uint tmp.4
            ul.1 = tmp.5
            tmp.6 = double_to_uint ul.1
            tmp.7 = ul.1 != 3446744073709551616UL
            if !tmp.7 jump end_if_2
            return 2
        
          end_if_2:
            i.2 = 10
            tmp.8 = int_to_double i.2
            tmp.9 = tmp.8 + 0.99999D
            tmp.10 = double_to_int tmp.9
            i.2 = tmp.10
            tmp.11 = double_to_int i.2
            tmp.12 = i.2 != 10
            if !tmp.12 jump end_if_4
            return 3
        
          end_if_4:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_13_valid_extra_credit_incr_and_decr() {
    let src = r#"
        
        int main(void) {
            static double d = 0.75;
            if (d++ != 0.75) {
                return 1;
            }
            if (d != 1.75) {
                return 2;
            }
            d = -100.2;
            if (++d != -99.2) {
                return 3;
            }
            if (d != -99.2) {
                return 4;
            }
            if (d-- != -99.2) {
                return 5;
            }
            if (d != -100.2) {
                return 6;
            }
            if (--d != -101.2) {
                return 7;
            }
            if (d != -101.2) {
                return 8;
            }
            d = 0.000000000000000000001;
            d++;
            if (d != 1.0) {
                return 9;
            }
            d = 10e20;
            d--;
            if (d != 10e20) {
                return 10;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = d.0
            tmp.1 = inc d.0
            d.0 = tmp.1
            tmp.2 = tmp.0 != 0.75D
            if !tmp.2 jump end_if_0
            return 1
        
          end_if_0:
            tmp.3 = d.0 != 1.75D
            if !tmp.3 jump end_if_2
            return 2
        
          end_if_2:
            tmp.4 = - 100.2D
            d.0 = tmp.4
            tmp.5 = inc d.0
            d.0 = tmp.5
            tmp.7 = - 99.2D
            tmp.6 = tmp.5 != tmp.7
            if !tmp.6 jump end_if_4
            return 3
        
          end_if_4:
            tmp.9 = - 99.2D
            tmp.8 = d.0 != tmp.9
            if !tmp.8 jump end_if_6
            return 4
        
          end_if_6:
            tmp.10 = d.0
            tmp.11 = dec d.0
            d.0 = tmp.11
            tmp.13 = - 99.2D
            tmp.12 = tmp.10 != tmp.13
            if !tmp.12 jump end_if_8
            return 5
        
          end_if_8:
            tmp.15 = - 100.2D
            tmp.14 = d.0 != tmp.15
            if !tmp.14 jump end_if_10
            return 6
        
          end_if_10:
            tmp.16 = dec d.0
            d.0 = tmp.16
            tmp.18 = - 101.2D
            tmp.17 = tmp.16 != tmp.18
            if !tmp.17 jump end_if_12
            return 7
        
          end_if_12:
            tmp.20 = - 101.2D
            tmp.19 = d.0 != tmp.20
            if !tmp.19 jump end_if_14
            return 8
        
          end_if_14:
            d.0 = 0.000000000000000000001D
            tmp.21 = d.0
            tmp.22 = inc d.0
            d.0 = tmp.22
            tmp.23 = d.0 != 1D
            if !tmp.23 jump end_if_16
            return 9
        
          end_if_16:
            d.0 = 1000000000000000000000D
            tmp.24 = d.0
            tmp.25 = dec d.0
            d.0 = tmp.25
            tmp.26 = d.0 != 1000000000000000000000D
            if !tmp.26 jump end_if_18
            return 10
        
          end_if_18:
            return 0
            return 0
        }
        static d.0: Double = 0.75D
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_13_valid_extra_credit_nan() {
    let src = r#"
        int double_isnan(double d);
        int main(void) {
            static double zero = 0.0;
            double nan = 0.0 / zero;
            if (nan < 0.0 || nan == 0.0 || nan > 0.0 || nan <= 0.0 || nan >= 0.0)
                return 1;
            if (1 > nan || 1 == nan || 1 > nan || 1 <= nan || 1 >= nan)
                return 2;
            if (nan == nan)
                return 3;
            if (!(nan != nan)) {
                return 4;
            }
            if (!double_isnan(nan)) {
                return 5;
            }
            if (!double_isnan(4 * nan)) {
                return 6;
            }
            if (!double_isnan(22e2 / nan)) {
                return 7;
            }
            if (!double_isnan(-nan)) {
                return 8;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 0D / zero.1
            nan.2 = tmp.0
            tmp.1 = nan.2 < 0D
            if tmp.1 jump or_true_0
            tmp.4 = nan.2 == 0D
            if tmp.4 jump or_true_0
            tmp.3 = 0
            jump or_end_1
        
          or_true_0:
            tmp.3 = 1
        
          or_end_1:
            if tmp.3 jump or_true_2
            tmp.7 = nan.2 > 0D
            if tmp.7 jump or_true_2
            tmp.6 = 0
            jump or_end_3
        
          or_true_2:
            tmp.6 = 1
        
          or_end_3:
            if tmp.6 jump or_true_4
            tmp.10 = nan.2 <= 0D
            if tmp.10 jump or_true_4
            tmp.9 = 0
            jump or_end_5
        
          or_true_4:
            tmp.9 = 1
        
          or_end_5:
            if tmp.9 jump or_true_6
            tmp.13 = nan.2 >= 0D
            if tmp.13 jump or_true_6
            tmp.12 = 0
            jump or_end_7
        
          or_true_6:
            tmp.12 = 1
        
          or_end_7:
            if !tmp.12 jump end_if_8
            return 1
        
          end_if_8:
            tmp.14 = int_to_double 1
            tmp.15 = tmp.14 > nan.2
            if tmp.15 jump or_true_10
            tmp.18 = int_to_double 1
            tmp.19 = tmp.18 == nan.2
            if tmp.19 jump or_true_10
            tmp.17 = 0
            jump or_end_11
        
          or_true_10:
            tmp.17 = 1
        
          or_end_11:
            if tmp.17 jump or_true_12
            tmp.22 = int_to_double 1
            tmp.23 = tmp.22 > nan.2
            if tmp.23 jump or_true_12
            tmp.21 = 0
            jump or_end_13
        
          or_true_12:
            tmp.21 = 1
        
          or_end_13:
            if tmp.21 jump or_true_14
            tmp.26 = int_to_double 1
            tmp.27 = tmp.26 <= nan.2
            if tmp.27 jump or_true_14
            tmp.25 = 0
            jump or_end_15
        
          or_true_14:
            tmp.25 = 1
        
          or_end_15:
            if tmp.25 jump or_true_16
            tmp.30 = int_to_double 1
            tmp.31 = tmp.30 >= nan.2
            if tmp.31 jump or_true_16
            tmp.29 = 0
            jump or_end_17
        
          or_true_16:
            tmp.29 = 1
        
          or_end_17:
            if !tmp.29 jump end_if_18
            return 2
        
          end_if_18:
            tmp.32 = nan.2 == nan.2
            if !tmp.32 jump end_if_20
            return 3
        
          end_if_20:
            tmp.33 = nan.2 != nan.2
            tmp.34 = ! tmp.33
            if !tmp.34 jump end_if_22
            return 4
        
          end_if_22:
            tmp.35 = double_isnan(nan.2)
            tmp.36 = ! tmp.35
            if !tmp.36 jump end_if_24
            return 5
        
          end_if_24:
            tmp.37 = int_to_double 4
            tmp.38 = tmp.37 * nan.2
            tmp.39 = double_isnan(tmp.38)
            tmp.40 = ! tmp.39
            if !tmp.40 jump end_if_26
            return 6
        
          end_if_26:
            tmp.41 = 2200D / nan.2
            tmp.42 = double_isnan(tmp.41)
            tmp.43 = ! tmp.42
            if !tmp.43 jump end_if_28
            return 7
        
          end_if_28:
            tmp.44 = - nan.2
            tmp.45 = double_isnan(tmp.44)
            tmp.46 = ! tmp.45
            if !tmp.46 jump end_if_30
            return 8
        
          end_if_30:
            return 0
            return 0
        }
        static zero.1: Double = 0D
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_13_valid_floating_expressions_arithmetic_ops() {
    let src = r#"
        double point_one = 0.1;
        double point_two = 0.2;
        double point_three = 0.3;
        double two = 2.0;
        double three = 3.0;
        double four = 4.0;
        double twelveE30 = 12e30;
        int addition(void) {
            return (point_one + point_two == 0.30000000000000004);
        }
        int subtraction(void) {
            return (four - 1.0 == 3.0);
        }
        int multiplication(void) {
            return (0.01 * point_three == 0.003);
        }
        int division(void) {
            return (7.0 / two == 3.5);
        }
        int negation(void) {
            double neg = -twelveE30;
            return !(12e30 + neg);
        }
        int complex_expression(void) {
            double complex_expression = (two + three) - 127.5 * four;
            return complex_expression == -505.0;
        }
        int main(void) {
            if (!addition()) {
                return 1;
            }
            if (!subtraction()){
                return 2;
            }
            if (!multiplication()) {
                return 3;
            }
            if (!division()) {
                return 4;
            }
            if (!negation()) {
                return 5;
            }
            if (!complex_expression()) {
                return 5;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function addition() { 
            tmp.0 = point_one + point_two
            tmp.1 = tmp.0 == 0.30000000000000004D
            return tmp.1
            return 0
        }
        global function subtraction() { 
            tmp.2 = four - 1D
            tmp.3 = tmp.2 == 3D
            return tmp.3
            return 0
        }
        global function multiplication() { 
            tmp.4 = 0.01D * point_three
            tmp.5 = tmp.4 == 0.003D
            return tmp.5
            return 0
        }
        global function division() { 
            tmp.6 = 7D / two
            tmp.7 = tmp.6 == 3.5D
            return tmp.7
            return 0
        }
        global function negation() { 
            tmp.8 = - twelveE30
            neg.0 = tmp.8
            tmp.9 = 12000000000000000000000000000000D + neg.0
            tmp.10 = ! tmp.9
            return tmp.10
            return 0
        }
        global function complex_expression() { 
            tmp.11 = two + three
            tmp.13 = 127.5D * four
            tmp.12 = tmp.11 - tmp.13
            complex_expression.1 = tmp.12
            tmp.15 = - 505D
            tmp.14 = complex_expression.1 == tmp.15
            return tmp.14
            return 0
        }
        global function main() { 
            tmp.16 = addition()
            tmp.17 = ! tmp.16
            if !tmp.17 jump end_if_0
            return 1
        
          end_if_0:
            tmp.18 = subtraction()
            tmp.19 = ! tmp.18
            if !tmp.19 jump end_if_2
            return 2
        
          end_if_2:
            tmp.20 = multiplication()
            tmp.21 = ! tmp.20
            if !tmp.21 jump end_if_4
            return 3
        
          end_if_4:
            tmp.22 = division()
            tmp.23 = ! tmp.22
            if !tmp.23 jump end_if_6
            return 4
        
          end_if_6:
            tmp.24 = negation()
            tmp.25 = ! tmp.24
            if !tmp.25 jump end_if_8
            return 5
        
          end_if_8:
            tmp.26 = complex_expression()
            tmp.27 = ! tmp.26
            if !tmp.27 jump end_if_10
            return 5
        
          end_if_10:
            return 0
            return 0
        }
        static global four: Double = 4D
        static global point_one: Double = 0.1D
        static global point_three: Double = 0.3D
        static global point_two: Double = 0.2D
        static global three: Double = 3D
        static global twelveE30: Double = 12000000000000000000000000000000D
        static global two: Double = 2D
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_13_valid_floating_expressions_comparisons() {
    let src = r#"
        double fifty_fiveE5 = 55e5;
        double fifty_fourE4 = 54e4;
        double tiny = .00004;
        double four = 4.;
        double point_one = 0.1;
        int main(void) {
            if (fifty_fiveE5 < fifty_fourE4) {
                return 1;
            }
            if (four > 4.0) {
                return 2;
            }
            if (tiny <= 0.0) {
                return 3;
            }
            if (fifty_fourE4 >= fifty_fiveE5) {
                return 4;
            }
            if (tiny == 0.0) {
                return 5;
            }
            if (point_one != point_one) {
                return 6;
            }
            if (!(tiny > 00.000005)) {
                return 7;
            }
            if (!(-.00004 < four)) {
                return 8;
            }
            if (!(tiny <= tiny)) {
                return 9;
            }
            if (!(fifty_fiveE5 >= fifty_fiveE5)) {
                return 10;
            }
            if (!(0.1 == point_one)) {
                return 11;
            }
            if (!(tiny != .00003)) {
                return 12;
            }
            if (0.00003 < 0.000000000003) {
                return 13;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = fifty_fiveE5 < fifty_fourE4
            if !tmp.0 jump end_if_0
            return 1
        
          end_if_0:
            tmp.1 = four > 4D
            if !tmp.1 jump end_if_2
            return 2
        
          end_if_2:
            tmp.2 = tiny <= 0D
            if !tmp.2 jump end_if_4
            return 3
        
          end_if_4:
            tmp.3 = fifty_fourE4 >= fifty_fiveE5
            if !tmp.3 jump end_if_6
            return 4
        
          end_if_6:
            tmp.4 = tiny == 0D
            if !tmp.4 jump end_if_8
            return 5
        
          end_if_8:
            tmp.5 = point_one != point_one
            if !tmp.5 jump end_if_10
            return 6
        
          end_if_10:
            tmp.6 = tiny > 0.000005D
            tmp.7 = ! tmp.6
            if !tmp.7 jump end_if_12
            return 7
        
          end_if_12:
            tmp.8 = - 0.00004D
            tmp.9 = tmp.8 < four
            tmp.10 = ! tmp.9
            if !tmp.10 jump end_if_14
            return 8
        
          end_if_14:
            tmp.11 = tiny <= tiny
            tmp.12 = ! tmp.11
            if !tmp.12 jump end_if_16
            return 9
        
          end_if_16:
            tmp.13 = fifty_fiveE5 >= fifty_fiveE5
            tmp.14 = ! tmp.13
            if !tmp.14 jump end_if_18
            return 10
        
          end_if_18:
            tmp.15 = 0.1D == point_one
            tmp.16 = ! tmp.15
            if !tmp.16 jump end_if_20
            return 11
        
          end_if_20:
            tmp.17 = tiny != 0.00003D
            tmp.18 = ! tmp.17
            if !tmp.18 jump end_if_22
            return 12
        
          end_if_22:
            tmp.19 = 0.00003D < 0.000000000003D
            if !tmp.19 jump end_if_24
            return 13
        
          end_if_24:
            return 0
            return 0
        }
        static global fifty_fiveE5: Double = 5500000D
        static global fifty_fourE4: Double = 540000D
        static global four: Double = 4D
        static global point_one: Double = 0.1D
        static global tiny: Double = 0.00004D
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_13_valid_floating_expressions_logical() {
    let src = r#"
        double zero = 0.0;
        double non_zero = 1E-20;
        double one = 1.0;
        double rounded_to_zero = 1e-330;
        int main(void) {
            if (zero) {
                return 1;
            }
            if (rounded_to_zero) {
                return 2;
            }
            if (non_zero) {
            } else {
                return 3;
            }
            if (0.e10) {
                return 4;
            }
            if (!non_zero) {
                return 4;
            }
            if (!(!zero)) {
                return 5;
            }
            if (!(!rounded_to_zero)) {
                return 6;
            }
            if (!(non_zero && 1.0)) {
                return 8;
            }
            if (3.0 && zero) {
                return 8;
            }
            if (rounded_to_zero && 1000e10) {
                return 9;
            }
            if (18446744073709551615UL && zero) {
                return 10;
            }
            if (!(non_zero && 5l)) {
                return 11;
            }
            if (!(5.0 || zero)) {
                return 12;
            }
            if (zero || rounded_to_zero) {
                return 13;
            }
            if (!(rounded_to_zero || 0.0001)) {
                return 14;
            }
            if (!(non_zero || 0u)) {
                return 15;
            }
            if (!(0 || 0.0000005)) {
                return 16;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            if !zero jump end_if_0
            return 1
        
          end_if_0:
            if !rounded_to_zero jump end_if_2
            return 2
        
          end_if_2:
            if !non_zero jump else_5
            jump end_if_4
        
          else_5:
            return 3
        
          end_if_4:
            if !0D jump end_if_6
            return 4
        
          end_if_6:
            tmp.0 = ! non_zero
            if !tmp.0 jump end_if_8
            return 4
        
          end_if_8:
            tmp.1 = ! zero
            tmp.2 = ! tmp.1
            if !tmp.2 jump end_if_10
            return 5
        
          end_if_10:
            tmp.3 = ! rounded_to_zero
            tmp.4 = ! tmp.3
            if !tmp.4 jump end_if_12
            return 6
        
          end_if_12:
            if !non_zero jump and_false_14
            if !1D jump and_false_14
            tmp.6 = 1
            jump and_end_15
        
          and_false_14:
            tmp.6 = 0
        
          and_end_15:
            tmp.7 = ! tmp.6
            if !tmp.7 jump end_if_16
            return 8
        
          end_if_16:
            if !3D jump and_false_18
            if !zero jump and_false_18
            tmp.9 = 1
            jump and_end_19
        
          and_false_18:
            tmp.9 = 0
        
          and_end_19:
            if !tmp.9 jump end_if_20
            return 8
        
          end_if_20:
            if !rounded_to_zero jump and_false_22
            if !10000000000000D jump and_false_22
            tmp.11 = 1
            jump and_end_23
        
          and_false_22:
            tmp.11 = 0
        
          and_end_23:
            if !tmp.11 jump end_if_24
            return 9
        
          end_if_24:
            if !18446744073709551615UL jump and_false_26
            if !zero jump and_false_26
            tmp.13 = 1
            jump and_end_27
        
          and_false_26:
            tmp.13 = 0
        
          and_end_27:
            if !tmp.13 jump end_if_28
            return 10
        
          end_if_28:
            if !non_zero jump and_false_30
            if !5L jump and_false_30
            tmp.15 = 1
            jump and_end_31
        
          and_false_30:
            tmp.15 = 0
        
          and_end_31:
            tmp.16 = ! tmp.15
            if !tmp.16 jump end_if_32
            return 11
        
          end_if_32:
            if 5D jump or_true_34
            if zero jump or_true_34
            tmp.18 = 0
            jump or_end_35
        
          or_true_34:
            tmp.18 = 1
        
          or_end_35:
            tmp.19 = ! tmp.18
            if !tmp.19 jump end_if_36
            return 12
        
          end_if_36:
            if zero jump or_true_38
            if rounded_to_zero jump or_true_38
            tmp.21 = 0
            jump or_end_39
        
          or_true_38:
            tmp.21 = 1
        
          or_end_39:
            if !tmp.21 jump end_if_40
            return 13
        
          end_if_40:
            if rounded_to_zero jump or_true_42
            if 0.0001D jump or_true_42
            tmp.23 = 0
            jump or_end_43
        
          or_true_42:
            tmp.23 = 1
        
          or_end_43:
            tmp.24 = ! tmp.23
            if !tmp.24 jump end_if_44
            return 14
        
          end_if_44:
            if non_zero jump or_true_46
            if 0U jump or_true_46
            tmp.26 = 0
            jump or_end_47
        
          or_true_46:
            tmp.26 = 1
        
          or_end_47:
            tmp.27 = ! tmp.26
            if !tmp.27 jump end_if_48
            return 15
        
          end_if_48:
            if 0 jump or_true_50
            if 0.0000005D jump or_true_50
            tmp.29 = 0
            jump or_end_51
        
          or_true_50:
            tmp.29 = 1
        
          or_end_51:
            tmp.30 = ! tmp.29
            if !tmp.30 jump end_if_52
            return 16
        
          end_if_52:
            return 0
            return 0
        }
        static global non_zero: Double = 0.00000000000000000001D
        static global one: Double = 1D
        static global rounded_to_zero: Double = 0D
        static global zero: Double = 0D
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_13_valid_floating_expressions_loop_controlling_expression() {
    let src = r#"
        int main(void) {
            int a = 0;
            for(double d = 100.0; d > 0.0; d = d - 1.0) {
                a = a + 1;
            }
            return a;
        }
    "#;
    let expected = r#"
        global function main() { 
            a.0 = 0
            d.1 = 100D
        
          start_loop_0:
            tmp.0 = d.1 > 0D
            if !tmp.0 jump break_loop_0
            tmp.1 = a.0 + 1
            a.0 = tmp.1
        
          continue_loop_0:
            tmp.2 = d.1 - 1D
            d.1 = tmp.2
            jump start_loop_0
        
          break_loop_0:
            return a.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_13_valid_floating_expressions_simple() {
    let src = r#"
        
        int main(void) {
            double x = 2.0;
            return (x * 2.0 == 4.0);
        }
    "#;
    let expected = r#"
        global function main() { 
            x.0 = 2D
            tmp.0 = x.0 * 2D
            tmp.1 = tmp.0 == 4D
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_13_valid_floating_expressions_static_initialized_double() {
    let src = r#"
        double return_static_variable(void) {
            static double d = 0.5;
            double ret = d;
            d = d + 1.0;
            return ret;
        }
        int main(void) {
            double d1 = return_static_variable();
            double d2 = return_static_variable();
            double d3 = return_static_variable();
            if (d1 != 0.5) {
                return 1;
            }
            if (d2 != 1.5) {
                return 2;
            }
            if (d3 != 2.5) {
                return 3;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function return_static_variable() { 
            ret.1 = d.0
            tmp.0 = d.0 + 1D
            d.0 = tmp.0
            return ret.1
            return 0
        }
        global function main() { 
            tmp.1 = return_static_variable()
            d1.2 = tmp.1
            tmp.2 = return_static_variable()
            d2.3 = tmp.2
            tmp.3 = return_static_variable()
            d3.4 = tmp.3
            tmp.4 = d1.2 != 0.5D
            if !tmp.4 jump end_if_0
            return 1
        
          end_if_0:
            tmp.5 = d2.3 != 1.5D
            if !tmp.5 jump end_if_2
            return 2
        
          end_if_2:
            tmp.6 = d3.4 != 2.5D
            if !tmp.6 jump end_if_4
            return 3
        
          end_if_4:
            return 0
            return 0
        }
        static d.0: Double = 0.5D
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_13_valid_function_calls_double_and_int_parameters() {
    let src = r#"
        int check_arguments(double d1, double d2, int i1, double d3, double d4, int i2, int i3,
                            int i4, double d5, double d6, double d7, int i5, double d8) {
            if (d1 != 1.0) {
                return 1;
            }
            if (d2 != 2.0) {
                return 2;
            }
            if (d3 != 3.0) {
                return 3;
            }
            if (d4 != 4.0 ){
                return 4;
            }
            if (d5 != 5.0){
                return 5;
            }
            if (d6 != 6.0 ){
                return 6;
            }
            if (d7 != 7.0 ){
                return 7;
            }
            if (d8 != 8.0 ){
                return 8;
            }
            if (i1 != 101 ){
                return 9;
            }
            if (i2 != 102 ){
                return 10;
            }
            if (i3 != 103){
                return 11;
            }
            if (i4 != 104) {
                return 12;
            }
            if (i5 != 105) {
                return 13;
            }
            return 0;
        }
        int main(void) {
            return check_arguments(1.0, 2.0, 101, 3.0, 4.0, 102, 103, 104, 5.0, 6.0, 7.0, 105, 8.0);
        }
    "#;
    let expected = r#"
        global function check_arguments(d1.0, d2.1, i1.2, d3.3, d4.4, i2.5, i3.6, i4.7, d5.8, d6.9, d7.10, i5.11, d8.12) { 
            tmp.0 = d1.0 != 1D
            if !tmp.0 jump end_if_0
            return 1
        
          end_if_0:
            tmp.1 = d2.1 != 2D
            if !tmp.1 jump end_if_2
            return 2
        
          end_if_2:
            tmp.2 = d3.3 != 3D
            if !tmp.2 jump end_if_4
            return 3
        
          end_if_4:
            tmp.3 = d4.4 != 4D
            if !tmp.3 jump end_if_6
            return 4
        
          end_if_6:
            tmp.4 = d5.8 != 5D
            if !tmp.4 jump end_if_8
            return 5
        
          end_if_8:
            tmp.5 = d6.9 != 6D
            if !tmp.5 jump end_if_10
            return 6
        
          end_if_10:
            tmp.6 = d7.10 != 7D
            if !tmp.6 jump end_if_12
            return 7
        
          end_if_12:
            tmp.7 = d8.12 != 8D
            if !tmp.7 jump end_if_14
            return 8
        
          end_if_14:
            tmp.8 = i1.2 != 101
            if !tmp.8 jump end_if_16
            return 9
        
          end_if_16:
            tmp.9 = i2.5 != 102
            if !tmp.9 jump end_if_18
            return 10
        
          end_if_18:
            tmp.10 = i3.6 != 103
            if !tmp.10 jump end_if_20
            return 11
        
          end_if_20:
            tmp.11 = i4.7 != 104
            if !tmp.11 jump end_if_22
            return 12
        
          end_if_22:
            tmp.12 = i5.11 != 105
            if !tmp.12 jump end_if_24
            return 13
        
          end_if_24:
            return 0
            return 0
        }
        global function main() { 
            tmp.13 = check_arguments(1D, 2D, 101, 3D, 4D, 102, 103, 104, 5D, 6D, 7D, 105, 8D)
            return tmp.13
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_13_valid_function_calls_double_and_int_params_recursive() {
    let src = r#"
        int fun(int i1, double d1, int i2, double d2, int i3, double d3,
                int i4, double d4, int i5, double d5, int i6, double d6,
                int i7, double d7, int i8, double d8, int i9, double d9) {
            if (i1 != d9) {
                int call1 = fun(i1 + 1, d1, i2 + 1, d2, i3 + 1, d3, i4 + 1, d4, i5 + 1, d5, i6 + 1, d6, i7 + 1, d7, i8 + 1, d8, i9 + 1, d9);
                int call2 = fun(i1, d1 - 1, i2, d2 - 1, i3, d3 - 1, i4, d4 - 1, i5, d5 - 1, i6, d6 - 1, i7, d7 - 1, i8, d8 - 1, i9, d9 - 1);
                if (call1) {
                    return call1;
                }
                if (call2) {
                    return call2;
                }
            }
            if (i2 != i1 + 2) {
                return 2;
            }
            if (i3 != i1 + 4) {
                return 3;
            }
            if (i4 != i1 + 6) {
                return 4;
            }
            if (i5 != i1 + 8) {
                return 5;
            }
            if (i6 != i1 + 10) {
                return 6;
            }
            if (i7 != i1 + 12) {
                return 7;
            }
            if (i8 != i1 + 14) {
                return 8;
            }
            if (i9 != i1 + 16) {
                return 9;
            }
            if (d1 != d9 - 16) {
                return 11;
            }
            if (d2 != d9 - 14) {
                return 12;
            }
            if (d3 != d9 - 12) {
                return 13;
            }
            if (d4 != d9 - 10) {
                return 14;
            }
            if (d5 != d9 - 8) {
                return 15;
            }
            if (d6 != d9 - 6) {
                return 16;
            }
            if (d7 != d9 - 4) {
                return 17;
            }
            if (d8 != d9 - 2) {
                return 18;
            }
            return 0;
        }
        int main(void) {
            return fun(1, 2.0, 3, 4.0, 5, 6.0, 7, 8.0, 9, 10.0, 11, 12.0, 13, 14.0, 15, 16.0, 17, 18.0);
        }
    "#;
    let expected = r#"
        global function fun(i1.0, d1.1, i2.2, d2.3, i3.4, d3.5, i4.6, d4.7, i5.8, d5.9, i6.10, d6.11, i7.12, d7.13, i8.14, d8.15, i9.16, d9.17) { 
            tmp.0 = int_to_double i1.0
            tmp.1 = tmp.0 != d9.17
            if !tmp.1 jump end_if_0
            tmp.2 = i1.0 + 1
            tmp.3 = i2.2 + 1
            tmp.4 = i3.4 + 1
            tmp.5 = i4.6 + 1
            tmp.6 = i5.8 + 1
            tmp.7 = i6.10 + 1
            tmp.8 = i7.12 + 1
            tmp.9 = i8.14 + 1
            tmp.10 = i9.16 + 1
            tmp.11 = fun(tmp.2, d1.1, tmp.3, d2.3, tmp.4, d3.5, tmp.5, d4.7, tmp.6, d5.9, tmp.7, d6.11, tmp.8, d7.13, tmp.9, d8.15, tmp.10, d9.17)
            call1.18 = tmp.11
            tmp.13 = int_to_double 1
            tmp.12 = d1.1 - tmp.13
            tmp.15 = int_to_double 1
            tmp.14 = d2.3 - tmp.15
            tmp.17 = int_to_double 1
            tmp.16 = d3.5 - tmp.17
            tmp.19 = int_to_double 1
            tmp.18 = d4.7 - tmp.19
            tmp.21 = int_to_double 1
            tmp.20 = d5.9 - tmp.21
            tmp.23 = int_to_double 1
            tmp.22 = d6.11 - tmp.23
            tmp.25 = int_to_double 1
            tmp.24 = d7.13 - tmp.25
            tmp.27 = int_to_double 1
            tmp.26 = d8.15 - tmp.27
            tmp.29 = int_to_double 1
            tmp.28 = d9.17 - tmp.29
            tmp.30 = fun(i1.0, tmp.12, i2.2, tmp.14, i3.4, tmp.16, i4.6, tmp.18, i5.8, tmp.20, i6.10, tmp.22, i7.12, tmp.24, i8.14, tmp.26, i9.16, tmp.28)
            call2.19 = tmp.30
            if !call1.18 jump end_if_2
            return call1.18
        
          end_if_2:
            if !call2.19 jump end_if_4
            return call2.19
        
          end_if_4:
        
          end_if_0:
            tmp.32 = i1.0 + 2
            tmp.31 = i2.2 != tmp.32
            if !tmp.31 jump end_if_6
            return 2
        
          end_if_6:
            tmp.34 = i1.0 + 4
            tmp.33 = i3.4 != tmp.34
            if !tmp.33 jump end_if_8
            return 3
        
          end_if_8:
            tmp.36 = i1.0 + 6
            tmp.35 = i4.6 != tmp.36
            if !tmp.35 jump end_if_10
            return 4
        
          end_if_10:
            tmp.38 = i1.0 + 8
            tmp.37 = i5.8 != tmp.38
            if !tmp.37 jump end_if_12
            return 5
        
          end_if_12:
            tmp.40 = i1.0 + 10
            tmp.39 = i6.10 != tmp.40
            if !tmp.39 jump end_if_14
            return 6
        
          end_if_14:
            tmp.42 = i1.0 + 12
            tmp.41 = i7.12 != tmp.42
            if !tmp.41 jump end_if_16
            return 7
        
          end_if_16:
            tmp.44 = i1.0 + 14
            tmp.43 = i8.14 != tmp.44
            if !tmp.43 jump end_if_18
            return 8
        
          end_if_18:
            tmp.46 = i1.0 + 16
            tmp.45 = i9.16 != tmp.46
            if !tmp.45 jump end_if_20
            return 9
        
          end_if_20:
            tmp.49 = int_to_double 16
            tmp.48 = d9.17 - tmp.49
            tmp.47 = d1.1 != tmp.48
            if !tmp.47 jump end_if_22
            return 11
        
          end_if_22:
            tmp.52 = int_to_double 14
            tmp.51 = d9.17 - tmp.52
            tmp.50 = d2.3 != tmp.51
            if !tmp.50 jump end_if_24
            return 12
        
          end_if_24:
            tmp.55 = int_to_double 12
            tmp.54 = d9.17 - tmp.55
            tmp.53 = d3.5 != tmp.54
            if !tmp.53 jump end_if_26
            return 13
        
          end_if_26:
            tmp.58 = int_to_double 10
            tmp.57 = d9.17 - tmp.58
            tmp.56 = d4.7 != tmp.57
            if !tmp.56 jump end_if_28
            return 14
        
          end_if_28:
            tmp.61 = int_to_double 8
            tmp.60 = d9.17 - tmp.61
            tmp.59 = d5.9 != tmp.60
            if !tmp.59 jump end_if_30
            return 15
        
          end_if_30:
            tmp.64 = int_to_double 6
            tmp.63 = d9.17 - tmp.64
            tmp.62 = d6.11 != tmp.63
            if !tmp.62 jump end_if_32
            return 16
        
          end_if_32:
            tmp.67 = int_to_double 4
            tmp.66 = d9.17 - tmp.67
            tmp.65 = d7.13 != tmp.66
            if !tmp.65 jump end_if_34
            return 17
        
          end_if_34:
            tmp.70 = int_to_double 2
            tmp.69 = d9.17 - tmp.70
            tmp.68 = d8.15 != tmp.69
            if !tmp.68 jump end_if_36
            return 18
        
          end_if_36:
            return 0
            return 0
        }
        global function main() { 
            tmp.71 = fun(1, 2D, 3, 4D, 5, 6D, 7, 8D, 9, 10D, 11, 12D, 13, 14D, 15, 16D, 17, 18D)
            return tmp.71
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_13_valid_function_calls_double_parameters() {
    let src = r#"
        
        int check_arguments(double a, double b, double c, double d, double e, double f, double g, double h);
        int main(void) {
            return check_arguments(1.0, 2.0, 3.0, 4.0, -1.0, -2.0, -3.0, -4.0);
        }
        int check_arguments(double a, double b, double c, double d, double e, double f, double g, double h) {
            if (a != 1.0) {
                return 1;
            }
            if (b != 2.0) {
                return 2;
            }
            if (c != 3.0) {
                return 3;
            }
            if (d != 4.0) {
                return 4;
            }
            if (e != -1.0) {
                return 5;
            }
            if (f != -2.0) {
                return 6;
            }
            if (g != -3.0) {
                return 7;
            }
            if (h != -4.0) {
                return 8;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = - 1D
            tmp.1 = - 2D
            tmp.2 = - 3D
            tmp.3 = - 4D
            tmp.4 = check_arguments(1D, 2D, 3D, 4D, tmp.0, tmp.1, tmp.2, tmp.3)
            return tmp.4
            return 0
        }
        global function check_arguments(a.8, b.9, c.10, d.11, e.12, f.13, g.14, h.15) { 
            tmp.5 = a.8 != 1D
            if !tmp.5 jump end_if_0
            return 1
        
          end_if_0:
            tmp.6 = b.9 != 2D
            if !tmp.6 jump end_if_2
            return 2
        
          end_if_2:
            tmp.7 = c.10 != 3D
            if !tmp.7 jump end_if_4
            return 3
        
          end_if_4:
            tmp.8 = d.11 != 4D
            if !tmp.8 jump end_if_6
            return 4
        
          end_if_6:
            tmp.10 = - 1D
            tmp.9 = e.12 != tmp.10
            if !tmp.9 jump end_if_8
            return 5
        
          end_if_8:
            tmp.12 = - 2D
            tmp.11 = f.13 != tmp.12
            if !tmp.11 jump end_if_10
            return 6
        
          end_if_10:
            tmp.14 = - 3D
            tmp.13 = g.14 != tmp.14
            if !tmp.13 jump end_if_12
            return 7
        
          end_if_12:
            tmp.16 = - 4D
            tmp.15 = h.15 != tmp.16
            if !tmp.15 jump end_if_14
            return 8
        
          end_if_14:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_13_valid_function_calls_push_xmm() {
    let src = r#"
        int callee(double a, double b, double c, double d, double e, double f, double g,
                   double h, double i, double j, double k) {
            if (a != 0.) {
                return 1;
            }
            if (b != 1.) {
                return 2;
            }
            if (c != 2.) {
                return 3;
            }
            if (d != 3.) {
                return 4;
            }
            if (e != 4.) {
                return 5;
            }
            if (f != 5.) {
                return 6;
            }
            if (g != 6.) {
                return 7;
            }
            if (h != 7.) {
                return 8;
            }
            if (i != 8.) {
                return 9;
            }
            if (j != 9.) {
                return 10;
            }
            if (k != 10.) {
                return 11;
            }
            return 0;
        }
        int target(int a, int b, int c, int d, int e) {
            return callee(0., 1., 2., 3., 4., 5., e + 1., d + 3., c + 5., b + 7.,
                          a + 9.);
        }
        int main(void) {
            return target(1, 2, 3, 4, 5);
        }
    "#;
    let expected = r#"
        global function callee(a.0, b.1, c.2, d.3, e.4, f.5, g.6, h.7, i.8, j.9, k.10) { 
            tmp.0 = a.0 != 0D
            if !tmp.0 jump end_if_0
            return 1
        
          end_if_0:
            tmp.1 = b.1 != 1D
            if !tmp.1 jump end_if_2
            return 2
        
          end_if_2:
            tmp.2 = c.2 != 2D
            if !tmp.2 jump end_if_4
            return 3
        
          end_if_4:
            tmp.3 = d.3 != 3D
            if !tmp.3 jump end_if_6
            return 4
        
          end_if_6:
            tmp.4 = e.4 != 4D
            if !tmp.4 jump end_if_8
            return 5
        
          end_if_8:
            tmp.5 = f.5 != 5D
            if !tmp.5 jump end_if_10
            return 6
        
          end_if_10:
            tmp.6 = g.6 != 6D
            if !tmp.6 jump end_if_12
            return 7
        
          end_if_12:
            tmp.7 = h.7 != 7D
            if !tmp.7 jump end_if_14
            return 8
        
          end_if_14:
            tmp.8 = i.8 != 8D
            if !tmp.8 jump end_if_16
            return 9
        
          end_if_16:
            tmp.9 = j.9 != 9D
            if !tmp.9 jump end_if_18
            return 10
        
          end_if_18:
            tmp.10 = k.10 != 10D
            if !tmp.10 jump end_if_20
            return 11
        
          end_if_20:
            return 0
            return 0
        }
        global function target(a.11, b.12, c.13, d.14, e.15) { 
            tmp.11 = int_to_double e.15
            tmp.12 = tmp.11 + 1D
            tmp.13 = int_to_double d.14
            tmp.14 = tmp.13 + 3D
            tmp.15 = int_to_double c.13
            tmp.16 = tmp.15 + 5D
            tmp.17 = int_to_double b.12
            tmp.18 = tmp.17 + 7D
            tmp.19 = int_to_double a.11
            tmp.20 = tmp.19 + 9D
            tmp.21 = callee(0D, 1D, 2D, 3D, 4D, 5D, tmp.12, tmp.14, tmp.16, tmp.18, tmp.20)
            return tmp.21
            return 0
        }
        global function main() { 
            tmp.22 = target(1, 2, 3, 4, 5)
            return tmp.22
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_13_valid_function_calls_return_double() {
    let src = r#"
        
        double d(void) {
            return 1234.e75;
        }
        int main(void) {
            double retval = d();
            return retval == 1234.e75;
        }
    "#;
    let expected = r#"
        global function d() { 
            return 1234000000000000000000000000000000000000000000000000000000000000000000000000000D
            return 0
        }
        global function main() { 
            tmp.0 = d()
            retval.0 = tmp.0
            tmp.1 = retval.0 == 1234000000000000000000000000000000000000000000000000000000000000000000000000000D
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_13_valid_function_calls_standard_library_call() {
    let src = r#"
        double fma(double x, double y, double z);
        double ldexp(double x, int exp);
        int main(void) {
            double fma_result = fma(5.0, 1E22, 4000000.0);
            double ldexp_result = ldexp(92E73, 5);
            if (fma_result != 50000000000000004194304.0) {
                return 1;
            }
            if (ldexp_result != 2.944E76) {
                return 2;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = fma(5D, 10000000000000000000000D, 4000000D)
            fma_result.5 = tmp.0
            tmp.1 = ldexp(920000000000000000000000000000000000000000000000000000000000000000000000000D, 5)
            ldexp_result.6 = tmp.1
            tmp.2 = fma_result.5 != 50000000000000004000000D
            if !tmp.2 jump end_if_0
            return 1
        
          end_if_0:
            tmp.3 = ldexp_result.6 != 29440000000000000000000000000000000000000000000000000000000000000000000000000D
            if !tmp.3 jump end_if_2
            return 2
        
          end_if_2:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_13_valid_function_calls_use_arg_after_fun_call() {
    let src = r#"
        double fun(double x) {
            if (x > 2)
                return x;
            else {
                double ret = fun(x + 2);
                return ret + x;
            }
        }
        int main(void) {
            return fun(1.0);
        }
    "#;
    let expected = r#"
        global function fun(x.0) { 
            tmp.1 = int_to_double 2
            tmp.0 = x.0 > tmp.1
            if !tmp.0 jump else_1
            return x.0
            jump end_if_0
        
          else_1:
            tmp.3 = int_to_double 2
            tmp.2 = x.0 + tmp.3
            tmp.4 = fun(tmp.2)
            ret.1 = tmp.4
            tmp.5 = ret.1 + x.0
            return tmp.5
        
          end_if_0:
            return 0
        }
        global function main() { 
            tmp.6 = fun(1D)
            tmp.7 = double_to_int tmp.6
            return tmp.7
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_13_valid_implicit_casts_common_type() {
    let src = r#"
        int lt(double d, long l) {
            return d < l;
        }
        double tern_double_flag(double flag) {
            return (double) (flag ? -30 : 10ul);
        }
        double tern_double_result(int flag) {
            return flag ? 5.0 : 9223372036854777850ul;
        }
        int ten = 10;
        int multiply(void) {
            int i = 10.75 * ten;
            return i == 107;
        }
        int main(void) {
            if (lt(-9007199254751228.0, -9007199254751227l)) {
                return 1;
            }
            if (tern_double_flag(20.0) != 18446744073709551586.0) {
                return 2;
            }
            if (tern_double_flag(0.0) != 10.0) {
                return 3;
            }
            if (tern_double_result(1) != 5.0) {
                return 4;
            }
            if (tern_double_result(0) != 9223372036854777856.0) {
                return 5;
            }
            if (!multiply()) {
                return 6;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function lt(d.0, l.1) { 
            tmp.1 = int_to_double l.1
            tmp.0 = d.0 < tmp.1
            return tmp.0
            return 0
        }
        global function tern_double_flag(flag.2) { 
            if !flag.2 jump else_1
            tmp.3 = - 30
            tmp.4 = sign_extend tmp.3
            tmp.2 = tmp.4
            jump end_if_0
        
          else_1:
            tmp.2 = 10UL
        
          end_if_0:
            tmp.5 = uint_to_double tmp.2
            return tmp.5
            return 0
        }
        global function tern_double_result(flag.3) { 
            if !flag.3 jump else_3
            tmp.6 = 5D
            jump end_if_2
        
          else_3:
            tmp.7 = uint_to_double 9223372036854777850UL
            tmp.6 = tmp.7
        
          end_if_2:
            return tmp.6
            return 0
        }
        global function multiply() { 
            tmp.9 = int_to_double ten
            tmp.8 = 10.75D * tmp.9
            tmp.10 = double_to_int tmp.8
            i.4 = tmp.10
            tmp.11 = i.4 == 107
            return tmp.11
            return 0
        }
        global function main() { 
            tmp.12 = - 9007199254751228D
            tmp.13 = - 9007199254751227L
            tmp.14 = lt(tmp.12, tmp.13)
            if !tmp.14 jump end_if_4
            return 1
        
          end_if_4:
            tmp.15 = tern_double_flag(20D)
            tmp.16 = tmp.15 != 18446744073709552000D
            if !tmp.16 jump end_if_6
            return 2
        
          end_if_6:
            tmp.17 = tern_double_flag(0D)
            tmp.18 = tmp.17 != 10D
            if !tmp.18 jump end_if_8
            return 3
        
          end_if_8:
            tmp.19 = tern_double_result(1)
            tmp.20 = tmp.19 != 5D
            if !tmp.20 jump end_if_10
            return 4
        
          end_if_10:
            tmp.21 = tern_double_result(0)
            tmp.22 = tmp.21 != 9223372036854778000D
            if !tmp.22 jump end_if_12
            return 5
        
          end_if_12:
            tmp.23 = multiply()
            tmp.24 = ! tmp.23
            if !tmp.24 jump end_if_14
            return 6
        
          end_if_14:
            return 0
            return 0
        }
        static global ten: Int = 10
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_13_valid_implicit_casts_complex_arithmetic_common_type() {
    let src = r#"
        unsigned long ul = 10000ul;
        int main(void) {
            int i = -50;
            double d = (ul + i) * 3.125;
            return d == 31093.75;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = - 50
            i.0 = tmp.0
            tmp.2 = sign_extend i.0
            tmp.1 = ul + tmp.2
            tmp.3 = uint_to_double tmp.1
            tmp.4 = tmp.3 * 3.125D
            d.1 = tmp.4
            tmp.5 = d.1 == 31093.75D
            return tmp.5
            return 0
        }
        static global ul: Unsigned Long = 10000UL
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_13_valid_implicit_casts_convert_for_assignment() {
    let src = r#"
        int check_args(long l, double d) {
            return l == 2 && d == -6.0;
        }
        double return_double(void) {
            return 18446744073709551586ul;
        }
        int check_assignment(double arg) {
            int i = 0;
            i = arg;
            return i == 4;
        }
        int main(void) {
            if (!check_args(2.4, -6)) {
                return 1;
            }
            if (return_double() != 18446744073709551616.0) {
                return 2;
            }
            if (!check_assignment(4.9)) {
                return 3;
            }
            double d = 18446744073709551586ul;
            if (d != 18446744073709551616.) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function check_args(l.0, d.1) { 
            tmp.1 = sign_extend 2
            tmp.0 = l.0 == tmp.1
            if !tmp.0 jump and_false_0
            tmp.5 = - 6D
            tmp.4 = d.1 == tmp.5
            if !tmp.4 jump and_false_0
            tmp.3 = 1
            jump and_end_1
        
          and_false_0:
            tmp.3 = 0
        
          and_end_1:
            return tmp.3
            return 0
        }
        global function return_double() { 
            tmp.6 = uint_to_double 18446744073709551586UL
            return tmp.6
            return 0
        }
        global function check_assignment(arg.2) { 
            i.3 = 0
            tmp.7 = double_to_int arg.2
            i.3 = tmp.7
            tmp.8 = i.3 == 4
            return tmp.8
            return 0
        }
        global function main() { 
            tmp.9 = double_to_int 2.4D
            tmp.10 = - 6
            tmp.11 = int_to_double tmp.10
            tmp.12 = check_args(tmp.9, tmp.11)
            tmp.13 = ! tmp.12
            if !tmp.13 jump end_if_2
            return 1
        
          end_if_2:
            tmp.14 = return_double()
            tmp.15 = tmp.14 != 18446744073709552000D
            if !tmp.15 jump end_if_4
            return 2
        
          end_if_4:
            tmp.16 = check_assignment(4.9D)
            tmp.17 = ! tmp.16
            if !tmp.17 jump end_if_6
            return 3
        
          end_if_6:
            tmp.18 = uint_to_double 18446744073709551586UL
            d.4 = tmp.18
            tmp.19 = d.4 != 18446744073709552000D
            if !tmp.19 jump end_if_8
            return 4
        
          end_if_8:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_13_valid_implicit_casts_static_initializers() {
    let src = r#"
        double d1 = 2147483647;
        double d2 = 4294967295u;
        double d3 = 4611686018427389440l;
        double d4 = 4611686018427389955l;
        double d5 = 9223372036854775810ul;
        double d6 = 4611686018427389955ul;
        double d7 = 9223372036854776832ul;
        double uninitialized;
        static int i = 4.9;
        int unsigned u = 42949.672923E5;
        long l = 4611686018427389440.;
        unsigned long ul = 18446744073709549568.;
        int main(void) {
            if (d1 != 2147483647.) {
                return 1;
            }
            if (d2 != 4294967295.) {
                return 2;
            }
            if (d3 != 4611686018427389952.) {
                return 3;
            }
            if (d4 != d3) {
                return 4;
            }
            if (d5 != 9223372036854775808.) {
                return 5;
            }
            if (d6 != d3) {
                return 6;
            }
            if (d7 != d5) {
                return 7;
            }
            if (uninitialized) {
                return 8;
            }
            if (i != 4) {
                return 9;
            }
            if (u != 4294967292u) {
                return 10;
            }
            if (l != 4611686018427389952l) {
                return 11;
            }
            if (ul != 18446744073709549568ul) {
                return 12;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = d1 != 2147483647D
            if !tmp.0 jump end_if_0
            return 1
        
          end_if_0:
            tmp.1 = d2 != 4294967295D
            if !tmp.1 jump end_if_2
            return 2
        
          end_if_2:
            tmp.2 = d3 != 4611686018427390000D
            if !tmp.2 jump end_if_4
            return 3
        
          end_if_4:
            tmp.3 = d4 != d3
            if !tmp.3 jump end_if_6
            return 4
        
          end_if_6:
            tmp.4 = d5 != 9223372036854776000D
            if !tmp.4 jump end_if_8
            return 5
        
          end_if_8:
            tmp.5 = d6 != d3
            if !tmp.5 jump end_if_10
            return 6
        
          end_if_10:
            tmp.6 = d7 != d5
            if !tmp.6 jump end_if_12
            return 7
        
          end_if_12:
            if !uninitialized jump end_if_14
            return 8
        
          end_if_14:
            tmp.7 = i != 4
            if !tmp.7 jump end_if_16
            return 9
        
          end_if_16:
            tmp.8 = u != 4294967292U
            if !tmp.8 jump end_if_18
            return 10
        
          end_if_18:
            tmp.9 = l != 4611686018427389952L
            if !tmp.9 jump end_if_20
            return 11
        
          end_if_20:
            tmp.10 = ul != 18446744073709549568UL
            if !tmp.10 jump end_if_22
            return 12
        
          end_if_22:
            return 0
            return 0
        }
        static global d1: Double = 2147483647D
        static global d2: Double = 4294967295D
        static global d3: Double = 4611686018427390000D
        static global d4: Double = 4611686018427390000D
        static global d5: Double = 9223372036854776000D
        static global d6: Double = 4611686018427390000D
        static global d7: Double = 9223372036854776000D
        static i: Int = 4
        static global l: Long = 4611686018427389952L
        static global u: Unsigned Int = 4294967292U
        static global ul: Unsigned Long = 18446744073709549568UL
        static global uninitialized: Double = 0D
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_13_valid_libraries_double_and_int_params_recursive() {
    let src = r#"
        int fun(int i1, double d1, int i2, double d2, int i3, double d3,
                int i4, double d4, int i5, double d5, int i6, double d6,
                int i7, double d7, int i8, double d8, int i9, double d9) {
            if (i1 != d9) {
                int call1 = fun(i1 + 1, d1, i2 + 1, d2, i3 + 1, d3, i4 + 1, d4, i5 + 1, d5, i6 + 1, d6, i7 + 1, d7, i8 + 1, d8, i9 + 1, d9);
                int call2 = fun(i1, d1 - 1, i2, d2 - 1, i3, d3 - 1, i4, d4 - 1, i5, d5 - 1, i6, d6 - 1, i7, d7 - 1, i8, d8 - 1, i9, d9 - 1);
                if (call1) {
                    return call1;
                }
                if (call2) {
                    return call2;
                }
            }
            if (i2 != i1 + 2) {
                return 2;
            }
            if (i3 != i1 + 4) {
                return 3;
            }
            if (i4 != i1 + 6) {
                return 4;
            }
            if (i5 != i1 + 8) {
                return 5;
            }
            if (i6 != i1 + 10) {
                return 6;
            }
            if (i7 != i1 + 12) {
                return 7;
            }
            if (i8 != i1 + 14) {
                return 8;
            }
            if (i9 != i1 + 16) {
                return 9;
            }
            if (d1 != d9 - 16) {
                return 11;
            }
            if (d2 != d9 - 14) {
                return 12;
            }
            if (d3 != d9 - 12) {
                return 13;
            }
            if (d4 != d9 - 10) {
                return 14;
            }
            if (d5 != d9 - 8) {
                return 15;
            }
            if (d6 != d9 - 6) {
                return 16;
            }
            if (d7 != d9 - 4) {
                return 17;
            }
            if (d8 != d9 - 2) {
                return 18;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function fun(i1.0, d1.1, i2.2, d2.3, i3.4, d3.5, i4.6, d4.7, i5.8, d5.9, i6.10, d6.11, i7.12, d7.13, i8.14, d8.15, i9.16, d9.17) { 
            tmp.0 = int_to_double i1.0
            tmp.1 = tmp.0 != d9.17
            if !tmp.1 jump end_if_0
            tmp.2 = i1.0 + 1
            tmp.3 = i2.2 + 1
            tmp.4 = i3.4 + 1
            tmp.5 = i4.6 + 1
            tmp.6 = i5.8 + 1
            tmp.7 = i6.10 + 1
            tmp.8 = i7.12 + 1
            tmp.9 = i8.14 + 1
            tmp.10 = i9.16 + 1
            tmp.11 = fun(tmp.2, d1.1, tmp.3, d2.3, tmp.4, d3.5, tmp.5, d4.7, tmp.6, d5.9, tmp.7, d6.11, tmp.8, d7.13, tmp.9, d8.15, tmp.10, d9.17)
            call1.18 = tmp.11
            tmp.13 = int_to_double 1
            tmp.12 = d1.1 - tmp.13
            tmp.15 = int_to_double 1
            tmp.14 = d2.3 - tmp.15
            tmp.17 = int_to_double 1
            tmp.16 = d3.5 - tmp.17
            tmp.19 = int_to_double 1
            tmp.18 = d4.7 - tmp.19
            tmp.21 = int_to_double 1
            tmp.20 = d5.9 - tmp.21
            tmp.23 = int_to_double 1
            tmp.22 = d6.11 - tmp.23
            tmp.25 = int_to_double 1
            tmp.24 = d7.13 - tmp.25
            tmp.27 = int_to_double 1
            tmp.26 = d8.15 - tmp.27
            tmp.29 = int_to_double 1
            tmp.28 = d9.17 - tmp.29
            tmp.30 = fun(i1.0, tmp.12, i2.2, tmp.14, i3.4, tmp.16, i4.6, tmp.18, i5.8, tmp.20, i6.10, tmp.22, i7.12, tmp.24, i8.14, tmp.26, i9.16, tmp.28)
            call2.19 = tmp.30
            if !call1.18 jump end_if_2
            return call1.18
        
          end_if_2:
            if !call2.19 jump end_if_4
            return call2.19
        
          end_if_4:
        
          end_if_0:
            tmp.32 = i1.0 + 2
            tmp.31 = i2.2 != tmp.32
            if !tmp.31 jump end_if_6
            return 2
        
          end_if_6:
            tmp.34 = i1.0 + 4
            tmp.33 = i3.4 != tmp.34
            if !tmp.33 jump end_if_8
            return 3
        
          end_if_8:
            tmp.36 = i1.0 + 6
            tmp.35 = i4.6 != tmp.36
            if !tmp.35 jump end_if_10
            return 4
        
          end_if_10:
            tmp.38 = i1.0 + 8
            tmp.37 = i5.8 != tmp.38
            if !tmp.37 jump end_if_12
            return 5
        
          end_if_12:
            tmp.40 = i1.0 + 10
            tmp.39 = i6.10 != tmp.40
            if !tmp.39 jump end_if_14
            return 6
        
          end_if_14:
            tmp.42 = i1.0 + 12
            tmp.41 = i7.12 != tmp.42
            if !tmp.41 jump end_if_16
            return 7
        
          end_if_16:
            tmp.44 = i1.0 + 14
            tmp.43 = i8.14 != tmp.44
            if !tmp.43 jump end_if_18
            return 8
        
          end_if_18:
            tmp.46 = i1.0 + 16
            tmp.45 = i9.16 != tmp.46
            if !tmp.45 jump end_if_20
            return 9
        
          end_if_20:
            tmp.49 = int_to_double 16
            tmp.48 = d9.17 - tmp.49
            tmp.47 = d1.1 != tmp.48
            if !tmp.47 jump end_if_22
            return 11
        
          end_if_22:
            tmp.52 = int_to_double 14
            tmp.51 = d9.17 - tmp.52
            tmp.50 = d2.3 != tmp.51
            if !tmp.50 jump end_if_24
            return 12
        
          end_if_24:
            tmp.55 = int_to_double 12
            tmp.54 = d9.17 - tmp.55
            tmp.53 = d3.5 != tmp.54
            if !tmp.53 jump end_if_26
            return 13
        
          end_if_26:
            tmp.58 = int_to_double 10
            tmp.57 = d9.17 - tmp.58
            tmp.56 = d4.7 != tmp.57
            if !tmp.56 jump end_if_28
            return 14
        
          end_if_28:
            tmp.61 = int_to_double 8
            tmp.60 = d9.17 - tmp.61
            tmp.59 = d5.9 != tmp.60
            if !tmp.59 jump end_if_30
            return 15
        
          end_if_30:
            tmp.64 = int_to_double 6
            tmp.63 = d9.17 - tmp.64
            tmp.62 = d6.11 != tmp.63
            if !tmp.62 jump end_if_32
            return 16
        
          end_if_32:
            tmp.67 = int_to_double 4
            tmp.66 = d9.17 - tmp.67
            tmp.65 = d7.13 != tmp.66
            if !tmp.65 jump end_if_34
            return 17
        
          end_if_34:
            tmp.70 = int_to_double 2
            tmp.69 = d9.17 - tmp.70
            tmp.68 = d8.15 != tmp.69
            if !tmp.68 jump end_if_36
            return 18
        
          end_if_36:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_13_valid_libraries_double_and_int_params_recursive_client() {
    let src = r#"
        int fun(int i1, double d1, int i2, double d2, int i3, double d3,
                int i4, double d4, int i5, double d5, int i6, double d6,
                int i7, double d7, int i8, double d8, int i9, double d9);
        int main(void) {
            double d = fun(1, 2.0, 3, 4.0, 5, 6.0, 7, 8.0, 9, 10.0, 11, 12.0, 13, 14.0, 15, 16.0, 17, 18.0);
            return (d == 78.00);
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = fun(1, 2D, 3, 4D, 5, 6D, 7, 8D, 9, 10D, 11, 12D, 13, 14D, 15, 16D, 17, 18D)
            tmp.1 = int_to_double tmp.0
            d.18 = tmp.1
            tmp.2 = d.18 == 78D
            return tmp.2
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_13_valid_libraries_double_parameters() {
    let src = r#"
        int check_arguments(double a, double b, double c, double d, double e, double f, double g, double h) {
            if (a != 1.0) {
                return 1;
            }
            if (b != 2.0) {
                return 2;
            }
            if (c != 3.0) {
                return 3;
            }
            if (d != 4.0) {
                return 4;
            }
            if (e != -1.0) {
                return 5;
            }
            if (f != -2.0) {
                return 6;
            }
            if (g != -3.0) {
                return 7;
            }
            if (h != -4.0) {
                return 8;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function check_arguments(a.0, b.1, c.2, d.3, e.4, f.5, g.6, h.7) { 
            tmp.0 = a.0 != 1D
            if !tmp.0 jump end_if_0
            return 1
        
          end_if_0:
            tmp.1 = b.1 != 2D
            if !tmp.1 jump end_if_2
            return 2
        
          end_if_2:
            tmp.2 = c.2 != 3D
            if !tmp.2 jump end_if_4
            return 3
        
          end_if_4:
            tmp.3 = d.3 != 4D
            if !tmp.3 jump end_if_6
            return 4
        
          end_if_6:
            tmp.5 = - 1D
            tmp.4 = e.4 != tmp.5
            if !tmp.4 jump end_if_8
            return 5
        
          end_if_8:
            tmp.7 = - 2D
            tmp.6 = f.5 != tmp.7
            if !tmp.6 jump end_if_10
            return 6
        
          end_if_10:
            tmp.9 = - 3D
            tmp.8 = g.6 != tmp.9
            if !tmp.8 jump end_if_12
            return 7
        
          end_if_12:
            tmp.11 = - 4D
            tmp.10 = h.7 != tmp.11
            if !tmp.10 jump end_if_14
            return 8
        
          end_if_14:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_13_valid_libraries_double_parameters_client() {
    let src = r#"
        int check_arguments(double a, double b, double c, double d, double e, double f, double g, double h);
        int main(void) {
            return check_arguments(1.0, 2.0, 3.0, 4.0, -1.0, -2.0, -3.0, -4.0);
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = - 1D
            tmp.1 = - 2D
            tmp.2 = - 3D
            tmp.3 = - 4D
            tmp.4 = check_arguments(1D, 2D, 3D, 4D, tmp.0, tmp.1, tmp.2, tmp.3)
            return tmp.4
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_13_valid_libraries_double_params_and_result() {
    let src = r#"
        double fmax(double x, double y);
        double get_max(double a, double b, double c, double d,
                       double e, double f, double g, double h,
                       double i, double j, double k)
        {
            double max = fmax(
                fmax(
                    fmax(
                        fmax(a, b),
                        fmax(c, d)),
                    fmax(
                        fmax(e, f),
                        fmax(g, h))),
                fmax(i, fmax(j, k)));
            return max;
        }
    "#;
    let expected = r#"
        global function get_max(a.2, b.3, c.4, d.5, e.6, f.7, g.8, h.9, i.10, j.11, k.12) { 
            tmp.0 = fmax(a.2, b.3)
            tmp.1 = fmax(c.4, d.5)
            tmp.2 = fmax(tmp.0, tmp.1)
            tmp.3 = fmax(e.6, f.7)
            tmp.4 = fmax(g.8, h.9)
            tmp.5 = fmax(tmp.3, tmp.4)
            tmp.6 = fmax(tmp.2, tmp.5)
            tmp.7 = fmax(j.11, k.12)
            tmp.8 = fmax(i.10, tmp.7)
            tmp.9 = fmax(tmp.6, tmp.8)
            max.13 = tmp.9
            return max.13
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_13_valid_libraries_double_params_and_result_client() {
    let src = r#"
        double get_max(double a, double b, double c, double d,
                       double e, double f, double g, double h,
                       double i, double j, double k);
        int main(void)
        {
            double result = get_max(100.3, 200.1, 0.01, 1.00004e5, 55.555, -4., 6543.2,
                                    9e9, 8e8, 7.6, 10e3 * 11e5);
            return result == 10e3 * 11e5;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = - 4D
            tmp.1 = 10000D * 1100000D
            tmp.2 = get_max(100.3D, 200.1D, 0.01D, 100004D, 55.555D, tmp.0, 6543.2D, 9000000000D, 800000000D, 7.6D, tmp.1)
            result.11 = tmp.2
            tmp.4 = 10000D * 1100000D
            tmp.3 = result.11 == tmp.4
            return tmp.3
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_13_valid_libraries_extern_double() {
    let src = r#"
        double d = 1e20;
    "#;
    let expected = r#"
        static global d: Double = 100000000000000000000D
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_13_valid_libraries_extern_double_client() {
    let src = r#"
        
        extern double d;
        int main(void) {
            return d == 1e20;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = d == 100000000000000000000D
            return tmp.0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_13_valid_libraries_use_arg_after_fun_call() {
    let src = r#"
        double fun(double x) {
            if (x > 2)
                return x;
            else {
                double ret = fun(x + 2);
                return ret + x;
            }
        }
    "#;
    let expected = r#"
        global function fun(x.0) { 
            tmp.1 = int_to_double 2
            tmp.0 = x.0 > tmp.1
            if !tmp.0 jump else_1
            return x.0
            jump end_if_0
        
          else_1:
            tmp.3 = int_to_double 2
            tmp.2 = x.0 + tmp.3
            tmp.4 = fun(tmp.2)
            ret.1 = tmp.4
            tmp.5 = ret.1 + x.0
            return tmp.5
        
          end_if_0:
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_13_valid_libraries_use_arg_after_fun_call_client() {
    let src = r#"
        double fun(double x);
        int main(void) {
            return fun(1.0);
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = fun(1D)
            tmp.1 = double_to_int tmp.0
            return tmp.1
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_13_valid_special_values_infinity() {
    let src = r#"
        double inf = 2e308;
        double very_large = 1.79E308;
        double zero = 0.0;
        int main(void) {
            if (inf != 11e330) {
                return 1;
            }
            if (inf <= very_large) {
                return 2;
            }
            if(very_large * 10.0 != inf) {
                return 3;
            }
            if (1.0 / zero != inf) {
                return 4;
            }
            double negated_inf = -inf;
            double negated_inf2 = -1.0 / zero;
            if (negated_inf >= -very_large) {
                return 5;
            }
            if (negated_inf != negated_inf2) {
                return 6;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = inf != infD
            if !tmp.0 jump end_if_0
            return 1
        
          end_if_0:
            tmp.1 = inf <= very_large
            if !tmp.1 jump end_if_2
            return 2
        
          end_if_2:
            tmp.2 = very_large * 10D
            tmp.3 = tmp.2 != inf
            if !tmp.3 jump end_if_4
            return 3
        
          end_if_4:
            tmp.4 = 1D / zero
            tmp.5 = tmp.4 != inf
            if !tmp.5 jump end_if_6
            return 4
        
          end_if_6:
            tmp.6 = - inf
            negated_inf.0 = tmp.6
            tmp.7 = - 1D
            tmp.8 = tmp.7 / zero
            negated_inf2.1 = tmp.8
            tmp.10 = - very_large
            tmp.9 = negated_inf.0 >= tmp.10
            if !tmp.9 jump end_if_8
            return 5
        
          end_if_8:
            tmp.11 = negated_inf.0 != negated_inf2.1
            if !tmp.11 jump end_if_10
            return 6
        
          end_if_10:
            return 0
            return 0
        }
        static global inf: Double = infD
        static global very_large: Double = 179000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000D
        static global zero: Double = 0D
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_13_valid_special_values_negative_zero() {
    let src = r#"
        double copysign(double x, double y);
        double zero = 0.0;
        int main(void) {
            double negative_zero = -zero;
            if (negative_zero != 0)
                return 1;
            if ( 1/negative_zero != -10e308 )
                return 2;
            if ( (-10)/negative_zero != 10e308)
                return 3;
            int fail = 0;
            negative_zero && (fail = 1);
            if (fail)
                return 4;
            if (negative_zero) {
                return 5;
            }
            if (zero != -0.0) {
                return 6;
            }
            double negated = copysign(4.0, -0.0);
            double positive = copysign(-5.0, 0.0);
            if (negated != -4.0) {
                return 7;
            }
            if (positive != 5.0) {
                return 8;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = - zero
            negative_zero.2 = tmp.0
            tmp.2 = int_to_double 0
            tmp.1 = negative_zero.2 != tmp.2
            if !tmp.1 jump end_if_0
            return 1
        
          end_if_0:
            tmp.3 = int_to_double 1
            tmp.4 = tmp.3 / negative_zero.2
            tmp.6 = - infD
            tmp.5 = tmp.4 != tmp.6
            if !tmp.5 jump end_if_2
            return 2
        
          end_if_2:
            tmp.7 = - 10
            tmp.8 = int_to_double tmp.7
            tmp.9 = tmp.8 / negative_zero.2
            tmp.10 = tmp.9 != infD
            if !tmp.10 jump end_if_4
            return 3
        
          end_if_4:
            fail.3 = 0
            if !negative_zero.2 jump and_false_6
            fail.3 = 1
            if !fail.3 jump and_false_6
            tmp.12 = 1
            jump and_end_7
        
          and_false_6:
            tmp.12 = 0
        
          and_end_7:
            if !fail.3 jump end_if_8
            return 4
        
          end_if_8:
            if !negative_zero.2 jump end_if_10
            return 5
        
          end_if_10:
            tmp.14 = - 0D
            tmp.13 = zero != tmp.14
            if !tmp.13 jump end_if_12
            return 6
        
          end_if_12:
            tmp.15 = - 0D
            tmp.16 = copysign(4D, tmp.15)
            negated.4 = tmp.16
            tmp.17 = - 5D
            tmp.18 = copysign(tmp.17, 0D)
            positive.5 = tmp.18
            tmp.20 = - 4D
            tmp.19 = negated.4 != tmp.20
            if !tmp.19 jump end_if_14
            return 7
        
          end_if_14:
            tmp.21 = positive.5 != 5D
            if !tmp.21 jump end_if_16
            return 8
        
          end_if_16:
            return 0
            return 0
        }
        static global zero: Double = 0D
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_chapter_13_valid_special_values_subnormal_not_zero() {
    let src = r#"
        int non_zero(double d) {
            return !d;
        }
        double multiply_by_large_num(double d) {
            return d * 2e20;
        }
        int main(void) {
            double subnormal = 2.5e-320;
            if (multiply_by_large_num(subnormal) != 4.99994433591341498562e-300) {
                return 2;
            }
            return non_zero(subnormal);
        }
    "#;
    let expected = r#"
        global function non_zero(d.0) { 
            tmp.0 = ! d.0
            return tmp.0
            return 0
        }
        global function multiply_by_large_num(d.1) { 
            tmp.1 = d.1 * 200000000000000000000D
            return tmp.1
            return 0
        }
        global function main() { 
            subnormal.2 = 0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000025D
            tmp.2 = multiply_by_large_num(subnormal.2)
            tmp.3 = tmp.2 != 0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004999944335913415D
            if !tmp.3 jump end_if_0
            return 2
        
          end_if_0:
            tmp.4 = non_zero(subnormal.2)
            return tmp.4
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}
