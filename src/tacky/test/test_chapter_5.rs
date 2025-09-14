use crate::pretty::{dedent, dump_tacky};

#[test]
fn test_valid_add_variables() {
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
fn test_valid_allocate_temps_and_vars() {
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
fn test_valid_assign() {
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
fn test_valid_assign_val_in_initializer() {
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
fn test_valid_assignment_in_initializer() {
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
fn test_valid_assignment_lowest_precedence() {
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
fn test_valid_empty_function_body() {
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
fn test_valid_exp_then_declaration() {
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
fn test_valid_extra_credit_bitwise_in_initializer() {
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
fn test_valid_extra_credit_bitwise_ops_vars() {
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
fn test_valid_extra_credit_bitwise_shiftl_variable() {
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
fn test_valid_extra_credit_bitwise_shiftr_assign() {
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
fn test_valid_extra_credit_compound_assignment_chained() {
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
fn test_valid_extra_credit_compound_assignment_lowest_precedence() {
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
fn test_valid_extra_credit_compound_assignment_use_result() {
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
fn test_valid_extra_credit_compound_bitwise_and() {
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
fn test_valid_extra_credit_compound_bitwise_assignment_lowest_precedence() {
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
fn test_valid_extra_credit_compound_bitwise_chained() {
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
fn test_valid_extra_credit_compound_bitwise_or() {
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
fn test_valid_extra_credit_compound_bitwise_shiftl() {
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
fn test_valid_extra_credit_compound_bitwise_shiftr() {
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
fn test_valid_extra_credit_compound_bitwise_xor() {
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
fn test_valid_extra_credit_compound_divide() {
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
fn test_valid_extra_credit_compound_minus() {
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
fn test_valid_extra_credit_compound_mod() {
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
fn test_valid_extra_credit_compound_multiply() {
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
fn test_valid_extra_credit_compound_plus() {
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
fn test_valid_extra_credit_incr_expression_statement() {
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
fn test_valid_extra_credit_incr_in_binary_expr() {
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
fn test_valid_extra_credit_incr_parenthesized() {
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
fn test_valid_extra_credit_postfix_incr_and_decr() {
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
fn test_valid_extra_credit_postfix_precedence() {
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
fn test_valid_extra_credit_prefix_incr_and_decr() {
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
fn test_valid_kw_var_names() {
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
fn test_valid_local_var_missing_return() {
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
fn test_valid_mixed_precedence_assignment() {
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
fn test_valid_non_short_circuit_or() {
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
fn test_valid_null_statement() {
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
fn test_valid_null_then_return() {
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
fn test_valid_return_var() {
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
fn test_valid_short_circuit_and_fail() {
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
fn test_valid_short_circuit_or() {
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
fn test_valid_unused_exp() {
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
fn test_valid_use_assignment_result() {
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
fn test_valid_use_val_in_own_initializer() {
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
