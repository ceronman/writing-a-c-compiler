use crate::pretty::{dedent, dump_tacky};

#[test]
fn test_valid_explicit_casts_sign_extend() {
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
fn test_valid_explicit_casts_truncate() {
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
fn test_valid_extra_credit_bitshift() {
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
fn test_valid_extra_credit_bitwise_long_op() {
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
fn test_valid_extra_credit_compound_assign_to_int() {
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
            tmp.5 = i.0 != 2147483628
            if !tmp.5 jump end_if_0
            return 1
        
          end_if_0:
            tmp.6 = b.1 != 2147483647
            if !tmp.6 jump end_if_2
            return 2
        
          end_if_2:
            tmp.7 = sign_extend b.1
            tmp.9 = - 34359738367L
            tmp.8 = tmp.7 / tmp.9
            tmp.10 = truncate tmp.8
            b.1 = tmp.10
            if !b.1 jump end_if_4
            return 3
        
          end_if_4:
            tmp.11 = i.0 != 2147483628
            if !tmp.11 jump end_if_6
            return 4
        
          end_if_6:
            tmp.13 = - 5000000
            tmp.12 = c.2 != tmp.13
            if !tmp.12 jump end_if_8
            return 5
        
          end_if_8:
            tmp.14 = sign_extend c.2
            tmp.15 = tmp.14 * 10000L
            tmp.16 = truncate tmp.15
            c.2 = tmp.16
            tmp.17 = c.2 != 1539607552
            if !tmp.17 jump end_if_10
            return 6
        
          end_if_10:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_compound_assign_to_long() {
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
fn test_valid_extra_credit_compound_bitshift() {
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
fn test_valid_extra_credit_compound_bitwise() {
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
            tmp.20 = - 2130771713
            tmp.19 = i.2 != tmp.20
            if !tmp.19 jump end_if_8
            return 5
        
          end_if_8:
            tmp.22 = - 2130771713
            tmp.21 = i.2 != tmp.22
            if !tmp.21 jump end_if_10
            return 6
        
          end_if_10:
            return 0
            return 0
        }
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_extra_credit_increment_long() {
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
fn test_valid_extra_credit_switch_int() {
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
            tmp.0 = 5 == i.0
            if tmp.0 jump switch_0_case__1
            tmp.1 = 0 == i.0
            if tmp.1 jump switch_0_case__2
            tmp.2 = -1 == i.0
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
fn test_valid_extra_credit_switch_long() {
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
            tmp.0 = 0L == l.0
            if tmp.0 jump switch_0_case__1
            tmp.1 = 100L == l.0
            if tmp.1 jump switch_0_case__2
            tmp.2 = 8589934592L == l.0
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
fn test_valid_implicit_casts_common_type() {
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
            l = 2147483648;
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
            l = 2147483648L
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
        static global i: Int = zero[4]
        static global l: Long = zero[8]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_implicit_casts_convert_by_assignment() {
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
fn test_valid_implicit_casts_convert_function_arguments() {
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
fn test_valid_implicit_casts_convert_static_initializer() {
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
fn test_valid_implicit_casts_long_constants() {
    let src = r#"
        int main(void) {
            if (2147483647l + 2147483647l < 0l) {
                return 1;
            }
            if (19327352832 < 100l) {
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
            tmp.2 = 19327352832L < 100L
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
fn test_valid_libraries_long_args() {
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
fn test_valid_libraries_long_args_client() {
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
fn test_valid_libraries_long_global_var() {
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
fn test_valid_libraries_long_global_var_client() {
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
fn test_valid_libraries_maintain_stack_alignment() {
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
fn test_valid_libraries_maintain_stack_alignment_client() {
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
fn test_valid_libraries_return_long() {
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
fn test_valid_libraries_return_long_client() {
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
fn test_valid_long_expressions_arithmetic_ops() {
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
        static global a: Long = zero[8]
        static global b: Long = zero[8]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_long_expressions_assign() {
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
fn test_valid_long_expressions_comparisons() {
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
        static global l: Long = zero[8]
        static global l2: Long = zero[8]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}

#[test]
fn test_valid_long_expressions_large_constants() {
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
fn test_valid_long_expressions_logical() {
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
fn test_valid_long_expressions_long_and_int_locals() {
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
fn test_valid_long_expressions_long_args() {
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
fn test_valid_long_expressions_multi_op() {
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
fn test_valid_long_expressions_return_long() {
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
fn test_valid_long_expressions_rewrite_large_multiply_regression() {
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
fn test_valid_long_expressions_simple() {
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
fn test_valid_long_expressions_static_long() {
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
fn test_valid_long_expressions_type_specifiers() {
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
        static a: Long = zero[8]
    "#;
    assert_eq!(dump_tacky(src), dedent(expected));
}
