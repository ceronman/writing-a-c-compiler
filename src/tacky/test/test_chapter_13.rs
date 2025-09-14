use crate::pretty::{dedent, dump_tacky};

#[test]
fn test_valid_constants_constant_doubles() {
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
fn test_valid_constants_round_constants() {
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
fn test_valid_explicit_casts_cvttsd2si_rewrite() {
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
fn test_valid_explicit_casts_double_to_signed() {
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
fn test_valid_explicit_casts_double_to_unsigned() {
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
fn test_valid_explicit_casts_rewrite_cvttsd2si_regression() {
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
fn test_valid_explicit_casts_signed_to_double() {
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
fn test_valid_explicit_casts_unsigned_to_double() {
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
fn test_valid_extra_credit_compound_assign() {
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
fn test_valid_extra_credit_compound_assign_implicit_cast() {
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
            tmp.6 = ul.1 != 3446744073709551616UL
            if !tmp.6 jump end_if_2
            return 2
        
          end_if_2:
            i.2 = 10
            tmp.7 = int_to_double i.2
            tmp.8 = tmp.7 + 0.99999D
            tmp.9 = double_to_int tmp.8
            i.2 = tmp.9
            tmp.10 = i.2 != 10
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
fn test_valid_extra_credit_incr_and_decr() {
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
fn test_valid_extra_credit_nan() {
    let src = r#"
        int double_isnan(double d);
        int main(void) {
            static double zero = 0.0;
            double nan = 0.0 / zero;
            if (nan < 0.0 || nan == 0.0 || nan > 0.0 || nan <= 0.0 || nan >= 0.0)
                return 1;
            if (1 < nan || 1 == nan || 1 > nan || 1 <= nan || 1 >= nan)
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
            if (!nan) {
                return 9;
            }
            if (nan) {
            } else {
                return 10;
            }
            int nan_is_nonzero;
            for (nan_is_nonzero = 0; nan;) {
                nan_is_nonzero = 1;
                break;
            }
            if (!nan_is_nonzero) {
                return 11;
            }
            nan_is_nonzero = 0;
            while (nan) {
                nan_is_nonzero = 1;
                break;
            }
            if (!nan_is_nonzero) {
                return 12;
            }
            nan_is_nonzero = -1;
            do {
                nan_is_nonzero = nan_is_nonzero + 1;
                if (nan_is_nonzero) {
                    break;
                }
            } while (nan);
            if (!nan_is_nonzero) {
                return 13;
            }
            nan_is_nonzero = nan ? 1 : 0;
            if (!nan_is_nonzero) {
                return 14;
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
            tmp.15 = tmp.14 < nan.2
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
            tmp.47 = ! nan.2
            if !tmp.47 jump end_if_32
            return 9
        
          end_if_32:
            tmp.48 = nan.2 != 0D
            if !tmp.48 jump else_35
            jump end_if_34
        
          else_35:
            return 10
        
          end_if_34:
            nan_is_nonzero.3 = 0
        
          start_loop_0:
            tmp.49 = nan.2 != 0D
            if !tmp.49 jump break_loop_0
            nan_is_nonzero.3 = 1
            jump break_loop_0
        
          continue_loop_0:
            jump start_loop_0
        
          break_loop_0:
            tmp.50 = ! nan_is_nonzero.3
            if !tmp.50 jump end_if_36
            return 11
        
          end_if_36:
            nan_is_nonzero.3 = 0
        
          continue_loop_1:
            tmp.51 = nan.2 != 0D
            if !tmp.51 jump break_loop_1
            nan_is_nonzero.3 = 1
            jump break_loop_1
            jump continue_loop_1
        
          break_loop_1:
            tmp.52 = ! nan_is_nonzero.3
            if !tmp.52 jump end_if_38
            return 12
        
          end_if_38:
            tmp.53 = - 1
            nan_is_nonzero.3 = tmp.53
        
          start_loop_2:
            tmp.54 = nan_is_nonzero.3 + 1
            nan_is_nonzero.3 = tmp.54
            if !nan_is_nonzero.3 jump end_if_40
            jump break_loop_2
        
          end_if_40:
        
          continue_loop_2:
            tmp.55 = nan.2 != 0D
            if tmp.55 jump start_loop_2
        
          break_loop_2:
            tmp.56 = ! nan_is_nonzero.3
            if !tmp.56 jump end_if_42
            return 13
        
          end_if_42:
            tmp.57 = nan.2 != 0D
            if !tmp.57 jump else_45
            tmp.58 = 1
            jump end_if_44
        
          else_45:
            tmp.58 = 0
        
          end_if_44:
            nan_is_nonzero.3 = tmp.58
            tmp.59 = ! nan_is_nonzero.3
            if !tmp.59 jump end_if_46
            return 14
        
          end_if_46:
            return 0
            return 0
        }
        static zero.1: Double = 0D
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_nan_compound_assign() {
    let src = r#"
        int double_isnan(double d);
        int main(void) {
            static double zero = 0.0;
            double nan = 0.0 / zero;
            if (!double_isnan(nan += 99.2)) {
                return 1;
            }
            if (!double_isnan(nan -= nan)) {
                return 2;
            }
            if (!double_isnan(nan *= 4.0)) {
                return 3;
            }
            if (!double_isnan(nan /= 0.0)) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 0D / zero.1
            nan.2 = tmp.0
            tmp.1 = nan.2 + 99.2D
            nan.2 = tmp.1
            tmp.2 = double_isnan(nan.2)
            tmp.3 = ! tmp.2
            if !tmp.3 jump end_if_0
            return 1
        
          end_if_0:
            tmp.4 = nan.2 - nan.2
            nan.2 = tmp.4
            tmp.5 = double_isnan(nan.2)
            tmp.6 = ! tmp.5
            if !tmp.6 jump end_if_2
            return 2
        
          end_if_2:
            tmp.7 = nan.2 * 4D
            nan.2 = tmp.7
            tmp.8 = double_isnan(nan.2)
            tmp.9 = ! tmp.8
            if !tmp.9 jump end_if_4
            return 3
        
          end_if_4:
            tmp.10 = nan.2 / 0D
            nan.2 = tmp.10
            tmp.11 = double_isnan(nan.2)
            tmp.12 = ! tmp.11
            if !tmp.12 jump end_if_6
            return 4
        
          end_if_6:
            return 0
            return 0
        }
        static zero.1: Double = 0D
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_nan_incr_and_decr() {
    let src = r#"
        int double_isnan(double d);
        int main(void) {
            static double zero = 0.0;
            double nan = 0.0 / zero;
            if (!double_isnan(++nan)) {
                return 1;
            }
            if (!double_isnan(--nan)) {
                return 2;
            }
            if (!double_isnan(nan++)) {
                return 3;
            }
            if (!double_isnan(nan--)) {
                return 4;
            }
            return 0;
        }
    "#;
    let expected = r#"
        global function main() { 
            tmp.0 = 0D / zero.1
            nan.2 = tmp.0
            tmp.1 = inc nan.2
            nan.2 = tmp.1
            tmp.2 = double_isnan(tmp.1)
            tmp.3 = ! tmp.2
            if !tmp.3 jump end_if_0
            return 1
        
          end_if_0:
            tmp.4 = dec nan.2
            nan.2 = tmp.4
            tmp.5 = double_isnan(tmp.4)
            tmp.6 = ! tmp.5
            if !tmp.6 jump end_if_2
            return 2
        
          end_if_2:
            tmp.7 = nan.2
            tmp.8 = inc nan.2
            nan.2 = tmp.8
            tmp.9 = double_isnan(tmp.7)
            tmp.10 = ! tmp.9
            if !tmp.10 jump end_if_4
            return 3
        
          end_if_4:
            tmp.11 = nan.2
            tmp.12 = dec nan.2
            nan.2 = tmp.12
            tmp.13 = double_isnan(tmp.11)
            tmp.14 = ! tmp.13
            if !tmp.14 jump end_if_6
            return 4
        
          end_if_6:
            return 0
            return 0
        }
        static zero.1: Double = 0D
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_floating_expressions_arithmetic_ops() {
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
fn test_valid_floating_expressions_comparisons() {
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
fn test_valid_floating_expressions_logical() {
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
            tmp.0 = zero != 0D
            if !tmp.0 jump end_if_0
            return 1
        
          end_if_0:
            tmp.1 = rounded_to_zero != 0D
            if !tmp.1 jump end_if_2
            return 2
        
          end_if_2:
            tmp.2 = non_zero != 0D
            if !tmp.2 jump else_5
            jump end_if_4
        
          else_5:
            return 3
        
          end_if_4:
            tmp.3 = 0D != 0D
            if !tmp.3 jump end_if_6
            return 4
        
          end_if_6:
            tmp.4 = ! non_zero
            if !tmp.4 jump end_if_8
            return 4
        
          end_if_8:
            tmp.5 = ! zero
            tmp.6 = ! tmp.5
            if !tmp.6 jump end_if_10
            return 5
        
          end_if_10:
            tmp.7 = ! rounded_to_zero
            tmp.8 = ! tmp.7
            if !tmp.8 jump end_if_12
            return 6
        
          end_if_12:
            if !non_zero jump and_false_14
            if !1D jump and_false_14
            tmp.10 = 1
            jump and_end_15
        
          and_false_14:
            tmp.10 = 0
        
          and_end_15:
            tmp.11 = ! tmp.10
            if !tmp.11 jump end_if_16
            return 8
        
          end_if_16:
            if !3D jump and_false_18
            if !zero jump and_false_18
            tmp.13 = 1
            jump and_end_19
        
          and_false_18:
            tmp.13 = 0
        
          and_end_19:
            if !tmp.13 jump end_if_20
            return 8
        
          end_if_20:
            if !rounded_to_zero jump and_false_22
            if !10000000000000D jump and_false_22
            tmp.15 = 1
            jump and_end_23
        
          and_false_22:
            tmp.15 = 0
        
          and_end_23:
            if !tmp.15 jump end_if_24
            return 9
        
          end_if_24:
            if !18446744073709551615UL jump and_false_26
            if !zero jump and_false_26
            tmp.17 = 1
            jump and_end_27
        
          and_false_26:
            tmp.17 = 0
        
          and_end_27:
            if !tmp.17 jump end_if_28
            return 10
        
          end_if_28:
            if !non_zero jump and_false_30
            if !5L jump and_false_30
            tmp.19 = 1
            jump and_end_31
        
          and_false_30:
            tmp.19 = 0
        
          and_end_31:
            tmp.20 = ! tmp.19
            if !tmp.20 jump end_if_32
            return 11
        
          end_if_32:
            if 5D jump or_true_34
            if zero jump or_true_34
            tmp.22 = 0
            jump or_end_35
        
          or_true_34:
            tmp.22 = 1
        
          or_end_35:
            tmp.23 = ! tmp.22
            if !tmp.23 jump end_if_36
            return 12
        
          end_if_36:
            if zero jump or_true_38
            if rounded_to_zero jump or_true_38
            tmp.25 = 0
            jump or_end_39
        
          or_true_38:
            tmp.25 = 1
        
          or_end_39:
            if !tmp.25 jump end_if_40
            return 13
        
          end_if_40:
            if rounded_to_zero jump or_true_42
            if 0.0001D jump or_true_42
            tmp.27 = 0
            jump or_end_43
        
          or_true_42:
            tmp.27 = 1
        
          or_end_43:
            tmp.28 = ! tmp.27
            if !tmp.28 jump end_if_44
            return 14
        
          end_if_44:
            if non_zero jump or_true_46
            if 0U jump or_true_46
            tmp.30 = 0
            jump or_end_47
        
          or_true_46:
            tmp.30 = 1
        
          or_end_47:
            tmp.31 = ! tmp.30
            if !tmp.31 jump end_if_48
            return 15
        
          end_if_48:
            if 0 jump or_true_50
            if 0.0000005D jump or_true_50
            tmp.33 = 0
            jump or_end_51
        
          or_true_50:
            tmp.33 = 1
        
          or_end_51:
            tmp.34 = ! tmp.33
            if !tmp.34 jump end_if_52
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
fn test_valid_floating_expressions_loop_controlling_expression() {
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
fn test_valid_floating_expressions_simple() {
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
fn test_valid_floating_expressions_static_initialized_double() {
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
fn test_valid_function_calls_double_and_int_parameters() {
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
fn test_valid_function_calls_double_and_int_params_recursive() {
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
fn test_valid_function_calls_double_parameters() {
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
fn test_valid_function_calls_push_xmm() {
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
fn test_valid_function_calls_return_double() {
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
fn test_valid_function_calls_standard_library_call() {
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
fn test_valid_function_calls_use_arg_after_fun_call() {
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
fn test_valid_implicit_casts_common_type() {
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
            tmp.2 = flag.2 != 0D
            if !tmp.2 jump else_1
            tmp.4 = - 30
            tmp.5 = sign_extend tmp.4
            tmp.3 = tmp.5
            jump end_if_0
        
          else_1:
            tmp.3 = 10UL
        
          end_if_0:
            tmp.6 = uint_to_double tmp.3
            return tmp.6
            return 0
        }
        global function tern_double_result(flag.3) { 
            if !flag.3 jump else_3
            tmp.7 = 5D
            jump end_if_2
        
          else_3:
            tmp.8 = uint_to_double 9223372036854777850UL
            tmp.7 = tmp.8
        
          end_if_2:
            return tmp.7
            return 0
        }
        global function multiply() { 
            tmp.10 = int_to_double ten
            tmp.9 = 10.75D * tmp.10
            tmp.11 = double_to_int tmp.9
            i.4 = tmp.11
            tmp.12 = i.4 == 107
            return tmp.12
            return 0
        }
        global function main() { 
            tmp.13 = - 9007199254751228D
            tmp.14 = - 9007199254751227L
            tmp.15 = lt(tmp.13, tmp.14)
            if !tmp.15 jump end_if_4
            return 1
        
          end_if_4:
            tmp.16 = tern_double_flag(20D)
            tmp.17 = tmp.16 != 18446744073709552000D
            if !tmp.17 jump end_if_6
            return 2
        
          end_if_6:
            tmp.18 = tern_double_flag(0D)
            tmp.19 = tmp.18 != 10D
            if !tmp.19 jump end_if_8
            return 3
        
          end_if_8:
            tmp.20 = tern_double_result(1)
            tmp.21 = tmp.20 != 5D
            if !tmp.21 jump end_if_10
            return 4
        
          end_if_10:
            tmp.22 = tern_double_result(0)
            tmp.23 = tmp.22 != 9223372036854778000D
            if !tmp.23 jump end_if_12
            return 5
        
          end_if_12:
            tmp.24 = multiply()
            tmp.25 = ! tmp.24
            if !tmp.25 jump end_if_14
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
fn test_valid_implicit_casts_complex_arithmetic_common_type() {
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
fn test_valid_implicit_casts_convert_for_assignment() {
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
fn test_valid_implicit_casts_static_initializers() {
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
            tmp.7 = uninitialized != 0D
            if !tmp.7 jump end_if_14
            return 8
        
          end_if_14:
            tmp.8 = i != 4
            if !tmp.8 jump end_if_16
            return 9
        
          end_if_16:
            tmp.9 = u != 4294967292U
            if !tmp.9 jump end_if_18
            return 10
        
          end_if_18:
            tmp.10 = l != 4611686018427389952L
            if !tmp.10 jump end_if_20
            return 11
        
          end_if_20:
            tmp.11 = ul != 18446744073709549568UL
            if !tmp.11 jump end_if_22
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
        static global uninitialized: Double = zero[8]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_libraries_double_and_int_params_recursive() {
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
fn test_valid_libraries_double_and_int_params_recursive_client() {
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
fn test_valid_libraries_double_parameters() {
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
fn test_valid_libraries_double_parameters_client() {
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
fn test_valid_libraries_double_params_and_result() {
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
fn test_valid_libraries_double_params_and_result_client() {
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
fn test_valid_libraries_extern_double() {
    let src = r#"
        double d = 1e20;
    "#;
    let expected = r#"
        static global d: Double = 100000000000000000000D
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_libraries_extern_double_client() {
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
fn test_valid_libraries_use_arg_after_fun_call() {
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
fn test_valid_libraries_use_arg_after_fun_call_client() {
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
fn test_valid_special_values_infinity() {
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
fn test_valid_special_values_negative_zero() {
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
            tmp.13 = negative_zero.2 != 0D
            if !tmp.13 jump end_if_10
            return 5
        
          end_if_10:
            tmp.15 = - 0D
            tmp.14 = zero != tmp.15
            if !tmp.14 jump end_if_12
            return 6
        
          end_if_12:
            tmp.16 = - 0D
            tmp.17 = copysign(4D, tmp.16)
            negated.4 = tmp.17
            tmp.18 = - 5D
            tmp.19 = copysign(tmp.18, 0D)
            positive.5 = tmp.19
            tmp.21 = - 4D
            tmp.20 = negated.4 != tmp.21
            if !tmp.20 jump end_if_14
            return 7
        
          end_if_14:
            tmp.22 = positive.5 != 5D
            if !tmp.22 jump end_if_16
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
fn test_valid_special_values_subnormal_not_zero() {
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
