use crate::pretty::{dedent, dump_tacky};

#[test]
fn test_valid_explicit_casts_chained_casts() {
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
fn test_valid_explicit_casts_extension() {
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
fn test_valid_explicit_casts_rewrite_movz_regression() {
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
fn test_valid_explicit_casts_round_trip_casts() {
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
fn test_valid_explicit_casts_same_size_conversion() {
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
fn test_valid_explicit_casts_truncate() {
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
fn test_valid_extra_credit_bitwise_unsigned_ops() {
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
fn test_valid_extra_credit_bitwise_unsigned_shift() {
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
fn test_valid_extra_credit_compound_assign_uint() {
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
            tmp.5 = x.0 == 3865470567U
            return tmp.5
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_compound_bitshift() {
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
fn test_valid_extra_credit_compound_bitwise() {
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
            if !ui.2 jump end_if_4
            return 3
        
          end_if_4:
            if !ui.2 jump end_if_6
            return 4
        
          end_if_6:
            tmp.12 = i.1 != 123456
            if !tmp.12 jump end_if_8
            return 5
        
          end_if_8:
            tmp.14 = - 252645136
            tmp.15 = sign_extend tmp.14
            tmp.13 = l.3 != tmp.15
            if !tmp.13 jump end_if_10
            return 6
        
          end_if_10:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_postfix_precedence() {
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
fn test_valid_extra_credit_switch_uint() {
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
            tmp.0 = 5U == ui.0
            if tmp.0 jump switch_0_case__1
            tmp.1 = 4294967286U == ui.0
            if tmp.1 jump switch_0_case__2
            tmp.2 = 10U == ui.0
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
fn test_valid_extra_credit_unsigned_incr_decr() {
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
fn test_valid_implicit_casts_common_type() {
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
fn test_valid_implicit_casts_convert_by_assignment() {
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
fn test_valid_implicit_casts_promote_constants() {
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
fn test_valid_implicit_casts_static_initializers() {
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
fn test_valid_libraries_unsigned_args() {
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
fn test_valid_libraries_unsigned_args_client() {
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
fn test_valid_libraries_unsigned_global_var() {
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
fn test_valid_libraries_unsigned_global_var_client() {
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
fn test_valid_type_specifiers_signed_type_specifiers() {
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
fn test_valid_type_specifiers_unsigned_type_specifiers() {
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
fn test_valid_unsigned_expressions_arithmetic_ops() {
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
        static global ui_a: Unsigned Int = zero[4]
        static global ui_b: Unsigned Int = zero[4]
        static global ul_a: Unsigned Long = zero[8]
        static global ul_b: Unsigned Long = zero[8]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_unsigned_expressions_arithmetic_wraparound() {
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
            ui_b = 3u;
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
            ui_b = 3U
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
        static global ui_a: Unsigned Int = zero[4]
        static global ui_b: Unsigned Int = zero[4]
        static global ul_a: Unsigned Long = zero[8]
        static global ul_b: Unsigned Long = zero[8]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_unsigned_expressions_comparisons() {
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
fn test_valid_unsigned_expressions_locals() {
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
fn test_valid_unsigned_expressions_logical() {
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
fn test_valid_unsigned_expressions_simple() {
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
fn test_valid_unsigned_expressions_static_variables() {
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
        static global zero_int: Unsigned Int = zero[4]
        static global zero_long: Unsigned Long = zero[8]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}
