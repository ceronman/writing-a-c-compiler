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
            if tmp.0 jump case_2_switch_0
            jump break_switch_0
        
          case_2_switch_0:
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
            if tmp.3 jump case_0_switch_0
            tmp.4 = tmp.2 == 4
            if tmp.4 jump case_4_switch_0
            tmp.5 = tmp.2 == 3
            if tmp.5 jump case_3_switch_0
            tmp.6 = tmp.2 == 2
            if tmp.6 jump case_2_switch_0
            tmp.7 = tmp.2 == 1
            if tmp.7 jump case_1_switch_0
            jump break_switch_0
        
          case_0_switch_0:
        
          start_loop_1:
            tmp.8 = count.0 - 1
            count.0 = tmp.8
        
          case_4_switch_0:
            tmp.9 = count.0 - 1
            count.0 = tmp.9
        
          case_3_switch_0:
            tmp.10 = count.0 - 1
            count.0 = tmp.10
        
          case_2_switch_0:
            tmp.11 = count.0 - 1
            count.0 = tmp.11
        
          case_1_switch_0:
            tmp.12 = count.0 - 1
            count.0 = tmp.12
        
          continue_loop_1:
            tmp.13 = iterations.1 - 1
            iterations.1 = tmp.13
            tmp.14 = iterations.1 > 0
            if tmp.14 jump start_loop_1
        
          break_loop_1:
        
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
            if tmp.0 jump case_1_switch_0
            tmp.1 = cond.0 == 10
            if tmp.1 jump case_10_switch_0
            jump default_switch_0
            jump break_switch_0
        
          case_1_switch_0:
            return 0
        
          case_10_switch_0:
            i.1 = 0
        
          start_loop_1:
            tmp.2 = i.1 < 5
            if !tmp.2 jump break_loop_1
            tmp.3 = cond.0 - 1
            cond.0 = tmp.3
            tmp.4 = cond.0 == 8
            if !tmp.4 jump end_if_0
            jump break_loop_1
        
          end_if_0:
        
          continue_loop_1:
            tmp.5 = i.1 + 1
            i.1 = tmp.5
            jump start_loop_1
        
          break_loop_1:
            return 123
        
          default_switch_0:
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
            if tmp.0 jump case_0_switch_0
            tmp.1 = 3 == 1
            if tmp.1 jump case_1_switch_0
            tmp.2 = 3 == 3
            if tmp.2 jump case_3_switch_0
            tmp.3 = 3 == 5
            if tmp.3 jump case_5_switch_0
            jump break_switch_0
        
          case_0_switch_0:
            return 0
        
          case_1_switch_0:
            return 1
        
          case_3_switch_0:
            return 3
        
          case_5_switch_0:
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
            if tmp.0 jump case_0_switch_0
            tmp.1 = a.0 == 1
            if tmp.1 jump case_1_switch_0
            jump default_switch_0
            jump break_switch_0
        
          case_0_switch_0:
            return 10
        
          case_1_switch_0:
            tmp.2 = a.0 * 2
            a.0 = tmp.2
            jump break_switch_0
        
          default_switch_0:
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
            if tmp.0 jump case_5_switch_0
            tmp.1 = a.0 == 6
            if tmp.1 jump case_6_switch_0
            jump break_switch_0
        
          case_5_switch_0:
            a.0 = 10
            jump break_switch_0
        
          case_6_switch_0:
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
            if tmp.0 jump case_3_switch_0
            jump break_switch_0
            b.1 = 5
            a.2 = b.1
        
          case_3_switch_0:
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
            if tmp.0 jump case_1_switch_0
            tmp.1 = a.0 == 2
            if tmp.1 jump case_2_switch_0
            tmp.2 = a.0 == 4
            if tmp.2 jump case_4_switch_0
            jump default_switch_0
            jump break_switch_0
        
          case_1_switch_0:
            return 1
        
          case_2_switch_0:
            return 9
        
          case_4_switch_0:
            a.0 = 11
            jump break_switch_0
        
          default_switch_0:
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
            if tmp.0 jump case_1_switch_0
            jump default_switch_0
            jump break_switch_0
        
          default_switch_0:
            a.0 = 0
        
          case_1_switch_0:
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
            if tmp.1 jump case_2_switch_0
            jump default_switch_0
            jump break_switch_0
        
          default_switch_0:
            return 0
        
          case_2_switch_0:
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
            jump default_switch_0
            jump break_switch_0
        
          default_switch_0:
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
            if tmp.1 jump case_0_switch_0
            tmp.2 = tmp.0 == 7
            if tmp.2 jump case_7_switch_0
            tmp.3 = tmp.0 == 9
            if tmp.3 jump case_9_switch_0
            tmp.4 = tmp.0 == 1
            if tmp.4 jump case_1_switch_0
            jump break_switch_0
        
          case_0_switch_0:
            return 5
        
          case_7_switch_0:
            c.2 = 1
        
          case_9_switch_0:
            c.2 = 2
        
          case_1_switch_0:
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
            jump mid_case_1
            tmp.0 = 4 == 4
            if tmp.0 jump case_4_switch_0
            jump break_switch_0
        
          case_4_switch_0:
            a.0 = 5
        
          mid_case_1:
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
            if tmp.1 jump case_0_switch_1
            tmp.2 = i.2 == 1
            if tmp.2 jump case_1_switch_1
            tmp.3 = i.2 == 2
            if tmp.3 jump case_2_switch_1
            jump default_switch_1
            jump break_switch_1
        
          case_0_switch_1:
            acc.0 = 2
            jump break_switch_1
        
          case_1_switch_1:
            tmp.4 = acc.0 * 3
            acc.0 = tmp.4
            jump break_switch_1
        
          case_2_switch_1:
            tmp.5 = acc.0 * 4
            acc.0 = tmp.5
            jump break_switch_1
        
          default_switch_1:
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
            if tmp.0 jump case_0_switch_0
            tmp.1 = 3 == 1
            if tmp.1 jump case_1_switch_0
            tmp.2 = 3 == 3
            if tmp.2 jump case_3_switch_0
            jump default_switch_0
            jump break_switch_0
        
          case_0_switch_0:
            return 0
        
          case_1_switch_0:
            if !0 jump end_if_0
        
          case_3_switch_0:
            switch1.0 = 1
            jump break_switch_0
        
          end_if_0:
        
          default_switch_0:
            return 0
        
          break_switch_0:
            tmp.3 = 4 == 0
            if tmp.3 jump case_0_switch_1
            tmp.4 = 4 == 4
            if tmp.4 jump case_4_switch_1
            jump default_switch_1
            jump break_switch_1
        
          case_0_switch_1:
            return 0
            if !1 jump else_3
            return 0
            jump end_if_2
        
          else_3:
        
          case_4_switch_1:
            switch2.1 = 1
            jump break_switch_1
        
          end_if_2:
        
          default_switch_1:
            return 0
        
          break_switch_1:
            tmp.5 = 5 == 5
            if tmp.5 jump case_5_switch_2
            jump default_switch_2
            jump break_switch_2
            i.3 = 0
        
          start_loop_3:
            tmp.6 = i.3 < 10
            if !tmp.6 jump break_loop_3
            switch1.0 = 0
        
          case_5_switch_2:
            switch3.2 = 1
            jump break_loop_3
        
          default_switch_2:
            return 0
        
          continue_loop_3:
            tmp.7 = i.3 + 1
            i.3 = tmp.7
            jump start_loop_3
        
          break_loop_3:
        
          break_switch_2:
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
            if tmp.0 jump case_1_switch_0
            jump default_switch_0
            jump break_switch_0
        
          case_1_switch_0:
            tmp.1 = a.0 == 0
            if tmp.1 jump case_0_switch_1
            jump default_switch_1
            jump break_switch_1
        
          case_0_switch_1:
            return 0
        
          default_switch_1:
            return 0
        
          break_switch_1:
        
          default_switch_0:
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
            if tmp.0 jump case_0_switch_0
            tmp.1 = 3 == 3
            if tmp.1 jump case_3_switch_0
            tmp.2 = 3 == 4
            if tmp.2 jump case_4_switch_0
            jump default_switch_0
            jump break_switch_0
        
          case_0_switch_0:
            return 0
        
          case_3_switch_0:
            tmp.3 = 4 == 3
            if tmp.3 jump case_3_switch_1
            tmp.4 = 4 == 4
            if tmp.4 jump case_4_switch_1
            jump default_switch_1
            jump break_switch_1
        
          case_3_switch_1:
            return 0
        
          case_4_switch_1:
            return 1
        
          default_switch_1:
            return 0
        
          break_switch_1:
        
          case_4_switch_0:
            return 0
        
          default_switch_0:
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
            if tmp.0 jump case_0_switch_0
            tmp.1 = a.0 == 2
            if tmp.1 jump case_2_switch_0
            tmp.2 = a.0 == 3
            if tmp.2 jump case_3_switch_0
            jump break_switch_0
        
          case_0_switch_0:
            return 0
        
          case_2_switch_0:
            return 0
        
          case_3_switch_0:
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
            if tmp.0 jump case_1_switch_0
            jump break_switch_0
        
          case_1_switch_0:
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
            if tmp.0 jump case_0_switch_0
            tmp.1 = 4 == 4
            if tmp.1 jump case_4_switch_0
            jump break_switch_0
        
          case_0_switch_0:
            return 0
        
          case_4_switch_0:
            acc.0 = 0
            i.1 = 0
        
          start_loop_1:
            tmp.2 = i.1 < 10
            if !tmp.2 jump break_loop_1
            tmp.3 = i.1 % 2
            if !tmp.3 jump end_if_0
            jump continue_loop_1
        
          end_if_0:
            tmp.4 = acc.0 + 1
            acc.0 = tmp.4
        
          continue_loop_1:
            tmp.5 = i.1 + 1
            i.1 = tmp.5
            jump start_loop_1
        
          break_loop_1:
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
            if tmp.2 jump case_0_switch_1
            jump default_switch_1
            jump break_switch_1
        
          case_0_switch_1:
            jump continue_loop_0
        
          default_switch_1:
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
            if tmp.1 jump case_0_switch_0
            tmp.2 = x == 1
            if tmp.2 jump case_1_switch_0
            tmp.3 = x == 4
            if tmp.3 jump case_4_switch_0
            jump default_switch_0
            jump break_switch_0
        
          case_0_switch_0:
            return 1
        
          case_1_switch_0:
            return 2
        
          case_4_switch_0:
            return 0
        
          default_switch_0:
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
            if tmp.0 jump case_1_switch_0
            tmp.1 = a.0 == 2
            if tmp.1 jump case_2_switch_0
            tmp.2 = a.0 == 10
            if tmp.2 jump case_10_switch_0
            jump default_switch_0
            jump break_switch_0
        
          case_1_switch_0:
            return 1
        
          case_2_switch_0:
            return 2
        
          case_10_switch_0:
            tmp.3 = x * 2
            tmp.4 = tmp.3 == 30
            if !tmp.4 jump end_if_0
            return 0
        
          end_if_0:
        
          default_switch_0:
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
            if tmp.0 jump case_1_switch_0
            tmp.1 = a == 3
            if tmp.1 jump case_3_switch_0
            jump break_switch_0
        
          case_1_switch_0:
            x.0 = 0
        
          case_3_switch_0:
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
